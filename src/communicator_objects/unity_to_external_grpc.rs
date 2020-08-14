// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_UNITY_TO_EXTERNAL_PROTO_EXCHANGE: ::grpcio::Method<super::unity_message::UnityMessageProto, super::unity_message::UnityMessageProto> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/communicator_objects.UnityToExternalProto/Exchange",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct UnityToExternalProtoClient {
    client: ::grpcio::Client,
}

impl UnityToExternalProtoClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        UnityToExternalProtoClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn exchange_opt(&self, req: &super::unity_message::UnityMessageProto, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::unity_message::UnityMessageProto> {
        self.client.unary_call(&METHOD_UNITY_TO_EXTERNAL_PROTO_EXCHANGE, req, opt)
    }

    pub fn exchange(&self, req: &super::unity_message::UnityMessageProto) -> ::grpcio::Result<super::unity_message::UnityMessageProto> {
        self.exchange_opt(req, ::grpcio::CallOption::default())
    }

    pub fn exchange_async_opt(&self, req: &super::unity_message::UnityMessageProto, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::unity_message::UnityMessageProto>> {
        self.client.unary_call_async(&METHOD_UNITY_TO_EXTERNAL_PROTO_EXCHANGE, req, opt)
    }

    pub fn exchange_async(&self, req: &super::unity_message::UnityMessageProto) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::unity_message::UnityMessageProto>> {
        self.exchange_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait UnityToExternalProto {
    fn exchange(&mut self, ctx: ::grpcio::RpcContext, req: super::unity_message::UnityMessageProto, sink: ::grpcio::UnarySink<super::unity_message::UnityMessageProto>);
}

pub fn create_unity_to_external_proto<S: UnityToExternalProto + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_UNITY_TO_EXTERNAL_PROTO_EXCHANGE, move |ctx, req, resp| {
        instance.exchange(ctx, req, resp)
    });
    builder.build()
}
