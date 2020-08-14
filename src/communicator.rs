use std::sync::Arc;
use super::communicator_objects as co;
use co::unity_input::UnityInputProto;
use co::unity_output::UnityOutputProto;
use co::unity_to_external_grpc::{UnityToExternalProto, create_unity_to_external_proto};
use co::unity_message::UnityMessageProto;
use co::header::HeaderProto;
use std::sync::mpsc::{channel, Sender, Receiver};
use grpcio::*;
use protobuf::SingularPtrField;

use thiserror::Error;
use std::result::Result;
use std::sync::Mutex;
use futures::executor::block_on;

#[derive(Error, Debug)]
pub enum CommunicatorError {
    #[error("Communicator has exited.")]
    CommunicatorExited
}

pub trait Communicator {
    fn initialize(&self, inputs: UnityInputProto) -> Result<UnityOutputProto, CommunicatorError>;
    fn exchange(&self, inputs: UnityInputProto) -> Result<UnityOutputProto, CommunicatorError>;
    fn close(&mut self);
}

struct UnityToExternalService {
    to_comm: Arc<Mutex<Sender<UnityMessageProto>>>,
    from_comm: Arc<Mutex<Receiver<UnityMessageProto>>>
}

impl UnityToExternalService {
    fn create(to_comm: Arc<Mutex<Sender<UnityMessageProto>>>, from_comm: Arc<Mutex<Receiver<UnityMessageProto>>>) -> UnityToExternalService {
        UnityToExternalService {
            to_comm,
            from_comm
        }
    }
}

impl Clone for UnityToExternalService {
    fn clone(&self) -> UnityToExternalService {
        UnityToExternalService::create(Arc::clone(&self.to_comm), Arc::clone(&self.from_comm))
    }
}

impl UnityToExternalProto for UnityToExternalService {
    fn exchange(&mut self, _ctx: RpcContext, req: UnityMessageProto, sink: UnarySink<UnityMessageProto>) {
        {
            self.to_comm.lock().unwrap().send(req);
        }
        let send_result = self.from_comm.lock().unwrap().recv()
            .map(|input| sink.success(input));
        if let Err(e) = send_result {
            panic!("failed to exchange with GRPC: {:?}", e);
        }
    }
}

pub struct GrpcCommunicator {
    server: Server,
    to_service: Sender<UnityMessageProto>,
    from_service: Receiver<UnityMessageProto>
}

fn build_server(env: Arc<Environment>, port: u16, service: Service) -> Server {
    let quota = grpcio::ResourceQuota::new(Some("RustyAgentsQuota"))
        .resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone())
        .set_resource_quota(quota);

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
    pub fn create() -> GrpcCommunicator {
        let (to_service, from_comm): (Sender<UnityMessageProto>, Receiver<UnityMessageProto>) = channel();
        let (to_comm, from_service): (Sender<UnityMessageProto>, Receiver<UnityMessageProto>) = channel();
        let to_comm = Arc::from(Mutex::from(to_comm));
        let from_comm = Arc::from(Mutex::from(from_comm));
        let service = UnityToExternalService::create(to_comm, from_comm);
        let generic_service = create_unity_to_external_proto(service);
        let env = Arc::new(Environment::new(1));
        let server = build_server(env, 5004, generic_service);
        GrpcCommunicator {
            server,
            to_service,
            from_service
        }
    }
}

impl Communicator for GrpcCommunicator {
    fn initialize(&self, inputs: UnityInputProto) -> Result<UnityOutputProto, CommunicatorError> {
        let mut aca_param = self.from_service
            .recv()
            .map(|m| m.unity_output.unwrap())
            .map_err(|_| CommunicatorError::CommunicatorExited);
        let mut message = UnityMessageProto::default();
        let mut header = HeaderProto::default();
        header.status = 200;
        message.header = SingularPtrField::from(Some(header));
        message.unity_input = SingularPtrField::from(Some(inputs.clone()));
        let aca_param = aca_param.map(|mut output| {
            output.rl_initialization_output = output.rl_initialization_output.map(|init_params| {
                println!("Aca params received: {}", init_params.name);
                init_params
            });
            output
        });
        println!("Sending initialization response.");
        self.to_service.send(message);
        self.from_service.recv();
        aca_param
    }

    fn exchange(&self, inputs: UnityInputProto) -> Result<UnityOutputProto, CommunicatorError> {
        let mut message = UnityMessageProto::default();
        let mut header = HeaderProto::default();
        header.status = 200;
        message.header = SingularPtrField::from(Some(header));
        message.unity_input = SingularPtrField::from(Some(inputs.clone()));
        self.to_service.send(message);
        self.from_service.recv()
            .map(|output| output.unity_output.unwrap())
            .map_err(|_| CommunicatorError::CommunicatorExited)
    }

    fn close(&mut self) {
        let mut message = UnityMessageProto::default();
        let mut header = HeaderProto::default();
        header.status = 400;
        message.header = SingularPtrField::from(Some(header));
        self.to_service.send(message);
        block_on::<ShutdownFuture>(self.server.shutdown());
    }
}
