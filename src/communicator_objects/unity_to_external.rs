// This file is generated by rust-protobuf 2.17.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `mlagents_envs/communicator_objects/unity_to_external.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_17_0;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n:mlagents_envs/communicator_objects/unity_to_external.proto\x12\x14com\
    municator_objects\x1a6mlagents_envs/communicator_objects/unity_message.p\
    roto2v\n\x14UnityToExternalProto\x12^\n\x08Exchange\x12'.communicator_ob\
    jects.UnityMessageProto\x1a'.communicator_objects.UnityMessageProto\"\0B\
    %\xaa\x02\"Unity.MLAgents.CommunicatorObjectsJ\xab\x01\n\x06\x12\x04\0\0\
    \n\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\0@\n\
    \x08\n\x01\x08\x12\x03\x04\0?\n\t\n\x02\x08%\x12\x03\x04\0?\n\x08\n\x01\
    \x02\x12\x03\x05\0\x1d\n\n\n\x02\x06\0\x12\x04\x07\0\n\x01\n\n\n\x03\x06\
    \0\x01\x12\x03\x07\x08\x1c\n+\n\x04\x06\0\x02\0\x12\x03\t\x04B\x1a\x1e\
    \x20Sends\x20the\x20academy\x20parameters\n\n\x0c\n\x05\x06\0\x02\0\x01\
    \x12\x03\t\x08\x10\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\t\x11\"\n\x0c\n\
    \x05\x06\0\x02\0\x03\x12\x03\t->b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
