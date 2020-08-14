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
//! Generated file from `mlagents_envs/communicator_objects/unity_rl_initialization_output.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_17_0;

#[derive(PartialEq,Clone,Default)]
pub struct UnityRLInitializationOutputProto {
    // message fields
    pub name: ::std::string::String,
    pub communication_version: ::std::string::String,
    pub log_path: ::std::string::String,
    pub brain_parameters: ::protobuf::RepeatedField<super::brain_parameters::BrainParametersProto>,
    pub package_version: ::std::string::String,
    pub capabilities: ::protobuf::SingularPtrField<super::capabilities::UnityRLCapabilitiesProto>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UnityRLInitializationOutputProto {
    fn default() -> &'a UnityRLInitializationOutputProto {
        <UnityRLInitializationOutputProto as ::protobuf::Message>::default_instance()
    }
}

impl UnityRLInitializationOutputProto {
    pub fn new() -> UnityRLInitializationOutputProto {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // string communication_version = 2;


    pub fn get_communication_version(&self) -> &str {
        &self.communication_version
    }
    pub fn clear_communication_version(&mut self) {
        self.communication_version.clear();
    }

    // Param is passed by value, moved
    pub fn set_communication_version(&mut self, v: ::std::string::String) {
        self.communication_version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_communication_version(&mut self) -> &mut ::std::string::String {
        &mut self.communication_version
    }

    // Take field
    pub fn take_communication_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.communication_version, ::std::string::String::new())
    }

    // string log_path = 3;


    pub fn get_log_path(&self) -> &str {
        &self.log_path
    }
    pub fn clear_log_path(&mut self) {
        self.log_path.clear();
    }

    // Param is passed by value, moved
    pub fn set_log_path(&mut self, v: ::std::string::String) {
        self.log_path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log_path(&mut self) -> &mut ::std::string::String {
        &mut self.log_path
    }

    // Take field
    pub fn take_log_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.log_path, ::std::string::String::new())
    }

    // repeated .communicator_objects.BrainParametersProto brain_parameters = 5;


    pub fn get_brain_parameters(&self) -> &[super::brain_parameters::BrainParametersProto] {
        &self.brain_parameters
    }
    pub fn clear_brain_parameters(&mut self) {
        self.brain_parameters.clear();
    }

    // Param is passed by value, moved
    pub fn set_brain_parameters(&mut self, v: ::protobuf::RepeatedField<super::brain_parameters::BrainParametersProto>) {
        self.brain_parameters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_brain_parameters(&mut self) -> &mut ::protobuf::RepeatedField<super::brain_parameters::BrainParametersProto> {
        &mut self.brain_parameters
    }

    // Take field
    pub fn take_brain_parameters(&mut self) -> ::protobuf::RepeatedField<super::brain_parameters::BrainParametersProto> {
        ::std::mem::replace(&mut self.brain_parameters, ::protobuf::RepeatedField::new())
    }

    // string package_version = 7;


    pub fn get_package_version(&self) -> &str {
        &self.package_version
    }
    pub fn clear_package_version(&mut self) {
        self.package_version.clear();
    }

    // Param is passed by value, moved
    pub fn set_package_version(&mut self, v: ::std::string::String) {
        self.package_version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_package_version(&mut self) -> &mut ::std::string::String {
        &mut self.package_version
    }

    // Take field
    pub fn take_package_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.package_version, ::std::string::String::new())
    }

    // .communicator_objects.UnityRLCapabilitiesProto capabilities = 8;


    pub fn get_capabilities(&self) -> &super::capabilities::UnityRLCapabilitiesProto {
        self.capabilities.as_ref().unwrap_or_else(|| <super::capabilities::UnityRLCapabilitiesProto as ::protobuf::Message>::default_instance())
    }
    pub fn clear_capabilities(&mut self) {
        self.capabilities.clear();
    }

    pub fn has_capabilities(&self) -> bool {
        self.capabilities.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capabilities(&mut self, v: super::capabilities::UnityRLCapabilitiesProto) {
        self.capabilities = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_capabilities(&mut self) -> &mut super::capabilities::UnityRLCapabilitiesProto {
        if self.capabilities.is_none() {
            self.capabilities.set_default();
        }
        self.capabilities.as_mut().unwrap()
    }

    // Take field
    pub fn take_capabilities(&mut self) -> super::capabilities::UnityRLCapabilitiesProto {
        self.capabilities.take().unwrap_or_else(|| super::capabilities::UnityRLCapabilitiesProto::new())
    }
}

impl ::protobuf::Message for UnityRLInitializationOutputProto {
    fn is_initialized(&self) -> bool {
        for v in &self.brain_parameters {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.capabilities {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.communication_version)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.log_path)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.brain_parameters)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.package_version)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.capabilities)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.communication_version.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.communication_version);
        }
        if !self.log_path.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.log_path);
        }
        for value in &self.brain_parameters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.package_version.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.package_version);
        }
        if let Some(ref v) = self.capabilities.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.communication_version.is_empty() {
            os.write_string(2, &self.communication_version)?;
        }
        if !self.log_path.is_empty() {
            os.write_string(3, &self.log_path)?;
        }
        for v in &self.brain_parameters {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.package_version.is_empty() {
            os.write_string(7, &self.package_version)?;
        }
        if let Some(ref v) = self.capabilities.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> UnityRLInitializationOutputProto {
        UnityRLInitializationOutputProto::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &UnityRLInitializationOutputProto| { &m.name },
                |m: &mut UnityRLInitializationOutputProto| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "communication_version",
                |m: &UnityRLInitializationOutputProto| { &m.communication_version },
                |m: &mut UnityRLInitializationOutputProto| { &mut m.communication_version },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "log_path",
                |m: &UnityRLInitializationOutputProto| { &m.log_path },
                |m: &mut UnityRLInitializationOutputProto| { &mut m.log_path },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::brain_parameters::BrainParametersProto>>(
                "brain_parameters",
                |m: &UnityRLInitializationOutputProto| { &m.brain_parameters },
                |m: &mut UnityRLInitializationOutputProto| { &mut m.brain_parameters },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "package_version",
                |m: &UnityRLInitializationOutputProto| { &m.package_version },
                |m: &mut UnityRLInitializationOutputProto| { &mut m.package_version },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::capabilities::UnityRLCapabilitiesProto>>(
                "capabilities",
                |m: &UnityRLInitializationOutputProto| { &m.capabilities },
                |m: &mut UnityRLInitializationOutputProto| { &mut m.capabilities },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UnityRLInitializationOutputProto>(
                "UnityRLInitializationOutputProto",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UnityRLInitializationOutputProto {
        static instance: ::protobuf::rt::LazyV2<UnityRLInitializationOutputProto> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UnityRLInitializationOutputProto::new)
    }
}

impl ::protobuf::Clear for UnityRLInitializationOutputProto {
    fn clear(&mut self) {
        self.name.clear();
        self.communication_version.clear();
        self.log_path.clear();
        self.brain_parameters.clear();
        self.package_version.clear();
        self.capabilities.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnityRLInitializationOutputProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnityRLInitializationOutputProto {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \nGmlagents_envs/communicator_objects/unity_rl_initialization_output.pro\
    to\x12\x14communicator_objects\x1a5mlagents_envs/communicator_objects/ca\
    pabilities.proto\x1a9mlagents_envs/communicator_objects/brain_parameters\
    .proto\"\xe0\x02\n\x20UnityRLInitializationOutputProto\x12\x12\n\x04name\
    \x18\x01\x20\x01(\tR\x04name\x123\n\x15communication_version\x18\x02\x20\
    \x01(\tR\x14communicationVersion\x12\x19\n\x08log_path\x18\x03\x20\x01(\
    \tR\x07logPath\x12U\n\x10brain_parameters\x18\x05\x20\x03(\x0b2*.communi\
    cator_objects.BrainParametersProtoR\x0fbrainParameters\x12'\n\x0fpackage\
    _version\x18\x07\x20\x01(\tR\x0epackageVersion\x12R\n\x0ccapabilities\
    \x18\x08\x20\x01(\x0b2..communicator_objects.UnityRLCapabilitiesProtoR\
    \x0ccapabilitiesJ\x04\x08\x06\x10\x07B%\xaa\x02\"Unity.MLAgents.Communic\
    atorObjectsJ\xac\x06\n\x06\x12\x04\0\0\x1a\x01\n\x08\n\x01\x0c\x12\x03\0\
    \0\x12\n\t\n\x02\x03\0\x12\x03\x02\0?\n\t\n\x02\x03\x01\x12\x03\x03\0C\n\
    \x08\n\x01\x08\x12\x03\x05\0?\n\t\n\x02\x08%\x12\x03\x05\0?\n\x08\n\x01\
    \x02\x12\x03\x06\0\x1d\nF\n\x02\x04\0\x12\x04\t\0\x1a\x01\x1a:\x20The\
    \x20request\x20message\x20containing\x20the\x20academy's\x20parameters.\
    \n\n\n\n\x03\x04\0\x01\x12\x03\t\x08(\n\x0b\n\x04\x04\0\x02\0\x12\x03\n\
    \x04\x14\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\n\x04\n\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\n\x0b\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\n\x12\
    \x13\no\n\x04\x04\0\x02\x01\x12\x03\r\x04%\x1ab\x20Communication\x20prot\
    ocol\x20version\x20that\x20the\x20responding\x20side\x20(typically\x20th\
    e\x20C#\x20environment)\x20is\x20using.\n\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03\r\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\r\x0b\x20\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\r#$\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0f\
    \x04\x18\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x0f\x04\n\n\x0c\n\x05\x04\
    \0\x02\x02\x01\x12\x03\x0f\x0b\x13\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\
    \x0f\x16\x17\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x11\x047\n\x0c\n\x05\x04\
    \0\x02\x03\x04\x12\x03\x11\x04\x0c\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\
    \x11\r!\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x11\"2\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03\x1156\n#\n\x03\x04\0\t\x12\x03\x13\x04\x0f\"\x17env\
    ironment\x20parameters\n\n\x0b\n\x04\x04\0\t\0\x12\x03\x13\r\x0e\n\x0c\n\
    \x05\x04\0\t\0\x01\x12\x03\x13\r\x0e\n\x0c\n\x05\x04\0\t\0\x02\x12\x03\
    \x13\r\x0e\nh\n\x04\x04\0\x02\x04\x12\x03\x16\x04\x1f\x1a[\x20Package/li\
    brary\x20version\x20that\x20the\x20responding\x20side\x20(typically\x20t\
    he\x20C#\x20environment)\x20is\x20using.\n\n\x0c\n\x05\x04\0\x02\x04\x05\
    \x12\x03\x16\x04\n\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x16\x0b\x1a\n\
    \x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x16\x1d\x1e\n5\n\x04\x04\0\x02\x05\
    \x12\x03\x19\x04.\x1a(\x20The\x20RL\x20Capabilities\x20of\x20the\x20C#\
    \x20package.\n\n\x0c\n\x05\x04\0\x02\x05\x06\x12\x03\x19\x04\x1c\n\x0c\n\
    \x05\x04\0\x02\x05\x01\x12\x03\x19\x1d)\n\x0c\n\x05\x04\0\x02\x05\x03\
    \x12\x03\x19,-b\x06proto3\
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