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
//! Generated file from `mlagents_envs/communicator_objects/unity_message.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_17_0;

#[derive(PartialEq,Clone,Default)]
pub struct UnityMessageProto {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::header::HeaderProto>,
    pub unity_output: ::protobuf::SingularPtrField<super::unity_output::UnityOutputProto>,
    pub unity_input: ::protobuf::SingularPtrField<super::unity_input::UnityInputProto>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UnityMessageProto {
    fn default() -> &'a UnityMessageProto {
        <UnityMessageProto as ::protobuf::Message>::default_instance()
    }
}

impl UnityMessageProto {
    pub fn new() -> UnityMessageProto {
        ::std::default::Default::default()
    }

    // .communicator_objects.HeaderProto header = 1;


    pub fn get_header(&self) -> &super::header::HeaderProto {
        self.header.as_ref().unwrap_or_else(|| <super::header::HeaderProto as ::protobuf::Message>::default_instance())
    }
    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::header::HeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::header::HeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::header::HeaderProto {
        self.header.take().unwrap_or_else(|| super::header::HeaderProto::new())
    }

    // .communicator_objects.UnityOutputProto unity_output = 2;


    pub fn get_unity_output(&self) -> &super::unity_output::UnityOutputProto {
        self.unity_output.as_ref().unwrap_or_else(|| <super::unity_output::UnityOutputProto as ::protobuf::Message>::default_instance())
    }
    pub fn clear_unity_output(&mut self) {
        self.unity_output.clear();
    }

    pub fn has_unity_output(&self) -> bool {
        self.unity_output.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unity_output(&mut self, v: super::unity_output::UnityOutputProto) {
        self.unity_output = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unity_output(&mut self) -> &mut super::unity_output::UnityOutputProto {
        if self.unity_output.is_none() {
            self.unity_output.set_default();
        }
        self.unity_output.as_mut().unwrap()
    }

    // Take field
    pub fn take_unity_output(&mut self) -> super::unity_output::UnityOutputProto {
        self.unity_output.take().unwrap_or_else(|| super::unity_output::UnityOutputProto::new())
    }

    // .communicator_objects.UnityInputProto unity_input = 3;


    pub fn get_unity_input(&self) -> &super::unity_input::UnityInputProto {
        self.unity_input.as_ref().unwrap_or_else(|| <super::unity_input::UnityInputProto as ::protobuf::Message>::default_instance())
    }
    pub fn clear_unity_input(&mut self) {
        self.unity_input.clear();
    }

    pub fn has_unity_input(&self) -> bool {
        self.unity_input.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unity_input(&mut self, v: super::unity_input::UnityInputProto) {
        self.unity_input = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unity_input(&mut self) -> &mut super::unity_input::UnityInputProto {
        if self.unity_input.is_none() {
            self.unity_input.set_default();
        }
        self.unity_input.as_mut().unwrap()
    }

    // Take field
    pub fn take_unity_input(&mut self) -> super::unity_input::UnityInputProto {
        self.unity_input.take().unwrap_or_else(|| super::unity_input::UnityInputProto::new())
    }
}

impl ::protobuf::Message for UnityMessageProto {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.unity_output {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.unity_input {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unity_output)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unity_input)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.unity_output.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.unity_input.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.unity_output.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.unity_input.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> UnityMessageProto {
        UnityMessageProto::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::header::HeaderProto>>(
                "header",
                |m: &UnityMessageProto| { &m.header },
                |m: &mut UnityMessageProto| { &mut m.header },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::unity_output::UnityOutputProto>>(
                "unity_output",
                |m: &UnityMessageProto| { &m.unity_output },
                |m: &mut UnityMessageProto| { &mut m.unity_output },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::unity_input::UnityInputProto>>(
                "unity_input",
                |m: &UnityMessageProto| { &m.unity_input },
                |m: &mut UnityMessageProto| { &mut m.unity_input },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UnityMessageProto>(
                "UnityMessageProto",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UnityMessageProto {
        static instance: ::protobuf::rt::LazyV2<UnityMessageProto> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UnityMessageProto::new)
    }
}

impl ::protobuf::Clear for UnityMessageProto {
    fn clear(&mut self) {
        self.header.clear();
        self.unity_output.clear();
        self.unity_input.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnityMessageProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnityMessageProto {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n6mlagents_envs/communicator_objects/unity_message.proto\x12\x14communi\
    cator_objects\x1a5mlagents_envs/communicator_objects/unity_output.proto\
    \x1a4mlagents_envs/communicator_objects/unity_input.proto\x1a/mlagents_e\
    nvs/communicator_objects/header.proto\"\xe1\x01\n\x11UnityMessageProto\
    \x129\n\x06header\x18\x01\x20\x01(\x0b2!.communicator_objects.HeaderProt\
    oR\x06header\x12I\n\x0cunity_output\x18\x02\x20\x01(\x0b2&.communicator_\
    objects.UnityOutputProtoR\x0bunityOutput\x12F\n\x0bunity_input\x18\x03\
    \x20\x01(\x0b2%.communicator_objects.UnityInputProtoR\nunityInputB%\xaa\
    \x02\"Unity.MLAgents.CommunicatorObjectsJ\x8f\x02\n\x06\x12\x04\0\0\r\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\0?\n\t\
    \n\x02\x03\x01\x12\x03\x03\0>\n\t\n\x02\x03\x02\x12\x03\x04\09\n\x08\n\
    \x01\x08\x12\x03\x06\0?\n\t\n\x02\x08%\x12\x03\x06\0?\n\x08\n\x01\x02\
    \x12\x03\x07\0\x1d\n\n\n\x02\x04\0\x12\x04\t\0\r\x01\n\n\n\x03\x04\0\x01\
    \x12\x03\t\x08\x19\n\x0b\n\x04\x04\0\x02\0\x12\x03\n\x04\x1b\n\x0c\n\x05\
    \x04\0\x02\0\x06\x12\x03\n\x04\x0f\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\
    \x10\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\n\x19\x1a\n\x0b\n\x04\x04\0\
    \x02\x01\x12\x03\x0b\x04&\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x0b\x04\
    \x14\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0b\x15!\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\x0b$%\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0c\x04$\n\
    \x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x0c\x04\x13\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\x0c\x14\x1f\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0c\"\
    #b\x06proto3\
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
