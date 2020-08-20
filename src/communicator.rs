use super::communicator_objects as co;
use co::header::HeaderProto;
use co::unity_input::UnityInputProto;
use co::unity_message::UnityMessageProto;
use co::unity_output::UnityOutputProto;
use co::unity_to_external_grpc::{create_unity_to_external_proto, UnityToExternalProto};
use grpcio::*;
use protobuf::SingularPtrField;
use std::sync::mpsc::*;
use std::sync::Arc;

use futures::executor::block_on;
use std::convert::From;
use std::result::Result;
use std::sync::mpsc::SendError;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommunicatorError {
    #[error("Communicator has exited.")]
    CommunicatorExited,
}

impl From<RecvError> for CommunicatorError {
    fn from(_err: RecvError) -> CommunicatorError {
        CommunicatorError::CommunicatorExited
    }
}

impl<T> From<SendError<T>> for CommunicatorError {
    fn from(_err: SendError<T>) -> CommunicatorError {
        CommunicatorError::CommunicatorExited
    }
}

pub trait Communicator {
    fn initialize(&self, inputs: UnityInputProto) -> Result<UnityOutputProto, CommunicatorError>;
    fn exchange(&self, inputs: UnityInputProto) -> Result<UnityOutputProto, CommunicatorError>;
    fn close(&mut self);
}

struct UnityToExternalService {
    to_comm: Arc<Mutex<Sender<UnityMessageProto>>>,
    from_comm: Arc<Mutex<Receiver<UnityMessageProto>>>,
}

impl UnityToExternalService {
    fn create(
        to_comm: Arc<Mutex<Sender<UnityMessageProto>>>,
        from_comm: Arc<Mutex<Receiver<UnityMessageProto>>>,
    ) -> UnityToExternalService {
        UnityToExternalService { to_comm, from_comm }
    }
}

impl Clone for UnityToExternalService {
    fn clone(&self) -> UnityToExternalService {
        UnityToExternalService::create(Arc::clone(&self.to_comm), Arc::clone(&self.from_comm))
    }
}

impl UnityToExternalProto for UnityToExternalService {
    fn exchange(
        &mut self,
        _ctx: RpcContext,
        req: UnityMessageProto,
        sink: UnarySink<UnityMessageProto>,
    ) {
        {
            let request_result = self.to_comm.lock().unwrap().send(req);
            if let Err(e) = request_result {
                panic!("failed to exchange with GRPC: {:?}", e);
            }
        }
        let response_result = self
            .from_comm
            .lock()
            .unwrap()
            .recv()
            .map(|input| sink.success(input));
        if let Err(e) = response_result {
            panic!("failed to exchange with GRPC: {:?}", e);
        }
    }
}

pub struct GrpcCommunicator {
    server: Server,
    to_service: Sender<UnityMessageProto>,
    from_service: Receiver<UnityMessageProto>,
}

fn build_server(env: Arc<Environment>, port: u16, service: Service) -> Server {
    let quota = grpcio::ResourceQuota::new(Some("RustyAgentsQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", port)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server.start();
    for (host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    server
}

impl GrpcCommunicator {
    pub fn create(port: u16) -> GrpcCommunicator {
        let (to_service, from_comm) = channel(); // GRPC service => communicator channel
        let (to_comm, from_service) = channel(); // communicator => GRPC service channel
        let to_comm = Arc::from(Mutex::from(to_comm));
        let from_comm = Arc::from(Mutex::from(from_comm));
        let service = UnityToExternalService::create(to_comm, from_comm);
        let generic_service = create_unity_to_external_proto(service);
        let env = Arc::new(Environment::new(1));
        let server = build_server(env, port, generic_service);
        GrpcCommunicator {
            server,
            to_service,
            from_service,
        }
    }
}

impl Communicator for GrpcCommunicator {
    fn initialize(&self, inputs: UnityInputProto) -> Result<UnityOutputProto, CommunicatorError> {
        let aca_param = self
            .from_service
            .recv()
            .map(|m| m.unity_output.unwrap())
            .map_err(|_| CommunicatorError::CommunicatorExited);
        let header = HeaderProto {
            status: 200,
            ..HeaderProto::default()
        };
        let message = UnityMessageProto {
            header: SingularPtrField::some(header),
            unity_input: SingularPtrField::some(inputs.clone()),
            ..UnityMessageProto::default()
        };
        let aca_param = aca_param.map(|mut output| {
            output.rl_initialization_output = output.rl_initialization_output.map(|init_params| {
                println!("Aca params received: {}", init_params.name);
                init_params
            });
            output
        });
        println!("Sending initialization response.");
        self.to_service.send(message)?;
        self.from_service.recv()?;
        aca_param
    }

    fn exchange(&self, inputs: UnityInputProto) -> Result<UnityOutputProto, CommunicatorError> {
        let header = HeaderProto {
            status: 200,
            ..HeaderProto::default()
        };
        let message = UnityMessageProto {
            header: SingularPtrField::some(header),
            unity_input: SingularPtrField::some(inputs.clone()),
            ..UnityMessageProto::default()
        };
        self.to_service.send(message)?;
        self.from_service
            .recv()
            .map(|output| output.unity_output.unwrap())
            .map_err(|_| CommunicatorError::CommunicatorExited)
    }

    fn close(&mut self) {
        let header = HeaderProto {
            status: 400,
            ..HeaderProto::default()
        };
        let message = UnityMessageProto {
            header: SingularPtrField::some(header),
            ..UnityMessageProto::default()
        };
        self.to_service
            .send(message)
            .map_err(|_e| println!("Error sending final message to environment."))
            .ok();
        block_on::<ShutdownFuture>(self.server.shutdown())
            .map_err(|_e| println!("Error shutting down server."))
            .ok();
    }
}
