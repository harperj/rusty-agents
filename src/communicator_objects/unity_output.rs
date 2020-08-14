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
//! Generated file from `mlagents_envs/communicator_objects/unity_output.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_17_0;

#[derive(PartialEq,Clone,Default)]
pub struct UnityOutputProto {
    // message fields
    pub rl_output: ::protobuf::SingularPtrField<super::unity_rl_output::UnityRLOutputProto>,
    pub rl_initialization_output: ::protobuf::SingularPtrField<super::unity_rl_initialization_output::UnityRLInitializationOutputProto>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UnityOutputProto {
    fn default() -> &'a UnityOutputProto {
        <UnityOutputProto as ::protobuf::Message>::default_instance()
    }
}

impl UnityOutputProto {
    pub fn new() -> UnityOutputProto {
        ::std::default::Default::default()
    }

    // .communicator_objects.UnityRLOutputProto rl_output = 1;


    pub fn get_rl_output(&self) -> &super::unity_rl_output::UnityRLOutputProto {
        self.rl_output.as_ref().unwrap_or_else(|| <super::unity_rl_output::UnityRLOutputProto as ::protobuf::Message>::default_instance())
    }
    pub fn clear_rl_output(&mut self) {
        self.rl_output.clear();
    }

    pub fn has_rl_output(&self) -> bool {
        self.rl_output.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rl_output(&mut self, v: super::unity_rl_output::UnityRLOutputProto) {
        self.rl_output = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rl_output(&mut self) -> &mut super::unity_rl_output::UnityRLOutputProto {
        if self.rl_output.is_none() {
            self.rl_output.set_default();
        }
        self.rl_output.as_mut().unwrap()
    }

    // Take field
    pub fn take_rl_output(&mut self) -> super::unity_rl_output::UnityRLOutputProto {
        self.rl_output.take().unwrap_or_else(|| super::unity_rl_output::UnityRLOutputProto::new())
    }

    // .communicator_objects.UnityRLInitializationOutputProto rl_initialization_output = 2;


    pub fn get_rl_initialization_output(&self) -> &super::unity_rl_initialization_output::UnityRLInitializationOutputProto {
        self.rl_initialization_output.as_ref().unwrap_or_else(|| <super::unity_rl_initialization_output::UnityRLInitializationOutputProto as ::protobuf::Message>::default_instance())
    }
    pub fn clear_rl_initialization_output(&mut self) {
        self.rl_initialization_output.clear();
    }

    pub fn has_rl_initialization_output(&self) -> bool {
        self.rl_initialization_output.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rl_initialization_output(&mut self, v: super::unity_rl_initialization_output::UnityRLInitializationOutputProto) {
        self.rl_initialization_output = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rl_initialization_output(&mut self) -> &mut super::unity_rl_initialization_output::UnityRLInitializationOutputProto {
        if self.rl_initialization_output.is_none() {
            self.rl_initialization_output.set_default();
        }
        self.rl_initialization_output.as_mut().unwrap()
    }

    // Take field
    pub fn take_rl_initialization_output(&mut self) -> super::unity_rl_initialization_output::UnityRLInitializationOutputProto {
        self.rl_initialization_output.take().unwrap_or_else(|| super::unity_rl_initialization_output::UnityRLInitializationOutputProto::new())
    }
}

impl ::protobuf::Message for UnityOutputProto {
    fn is_initialized(&self) -> bool {
        for v in &self.rl_output {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rl_initialization_output {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rl_output)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rl_initialization_output)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.rl_output.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.rl_initialization_output.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.rl_output.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.rl_initialization_output.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> UnityOutputProto {
        UnityOutputProto::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::unity_rl_output::UnityRLOutputProto>>(
                "rl_output",
                |m: &UnityOutputProto| { &m.rl_output },
                |m: &mut UnityOutputProto| { &mut m.rl_output },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::unity_rl_initialization_output::UnityRLInitializationOutputProto>>(
                "rl_initialization_output",
                |m: &UnityOutputProto| { &m.rl_initialization_output },
                |m: &mut UnityOutputProto| { &mut m.rl_initialization_output },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UnityOutputProto>(
                "UnityOutputProto",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UnityOutputProto {
        static instance: ::protobuf::rt::LazyV2<UnityOutputProto> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UnityOutputProto::new)
    }
}

impl ::protobuf::Clear for UnityOutputProto {
    fn clear(&mut self) {
        self.rl_output.clear();
        self.rl_initialization_output.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnityOutputProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnityOutputProto {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n5mlagents_envs/communicator_objects/unity_output.proto\x12\x14communic\
    ator_objects\x1a8mlagents_envs/communicator_objects/unity_rl_output.prot\
    o\x1aGmlagents_envs/communicator_objects/unity_rl_initialization_output.\
    proto\"\xcb\x01\n\x10UnityOutputProto\x12E\n\trl_output\x18\x01\x20\x01(\
    \x0b2(.communicator_objects.UnityRLOutputProtoR\x08rlOutput\x12p\n\x18rl\
    _initialization_output\x18\x02\x20\x01(\x0b26.communicator_objects.Unity\
    RLInitializationOutputProtoR\x16rlInitializationOutputB%\xaa\x02\"Unity.\
    MLAgents.CommunicatorObjectsJ\xcd\x01\n\x06\x12\x04\0\0\r\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\0B\n\t\n\x02\x03\
    \x01\x12\x03\x03\0Q\n\x08\n\x01\x08\x12\x03\x05\0?\n\t\n\x02\x08%\x12\
    \x03\x05\0?\n\x08\n\x01\x02\x12\x03\x06\0\x1d\n\n\n\x02\x04\0\x12\x04\
    \x08\0\r\x01\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x18\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\t\x04%\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\t\x04\x16\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\t\x17\x20\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\t#$\n\x0b\n\x04\x04\0\x02\x01\x12\x03\n\x04B\n\x0c\n\x05\x04\0\
    \x02\x01\x06\x12\x03\n\x04$\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\n%=\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03\n@Ab\x06proto3\
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
