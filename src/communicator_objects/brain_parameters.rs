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
//! Generated file from `mlagents_envs/communicator_objects/brain_parameters.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_17_0;

#[derive(PartialEq,Clone,Default)]
pub struct BrainParametersProto {
    // message fields
    pub vector_action_size: ::std::vec::Vec<i32>,
    pub vector_action_descriptions: ::protobuf::RepeatedField<::std::string::String>,
    pub vector_action_space_type: super::space_type::SpaceTypeProto,
    pub brain_name: ::std::string::String,
    pub is_training: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a BrainParametersProto {
    fn default() -> &'a BrainParametersProto {
        <BrainParametersProto as ::protobuf::Message>::default_instance()
    }
}

impl BrainParametersProto {
    pub fn new() -> BrainParametersProto {
        ::std::default::Default::default()
    }

    // repeated int32 vector_action_size = 3;


    pub fn get_vector_action_size(&self) -> &[i32] {
        &self.vector_action_size
    }
    pub fn clear_vector_action_size(&mut self) {
        self.vector_action_size.clear();
    }

    // Param is passed by value, moved
    pub fn set_vector_action_size(&mut self, v: ::std::vec::Vec<i32>) {
        self.vector_action_size = v;
    }

    // Mutable pointer to the field.
    pub fn mut_vector_action_size(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.vector_action_size
    }

    // Take field
    pub fn take_vector_action_size(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.vector_action_size, ::std::vec::Vec::new())
    }

    // repeated string vector_action_descriptions = 5;


    pub fn get_vector_action_descriptions(&self) -> &[::std::string::String] {
        &self.vector_action_descriptions
    }
    pub fn clear_vector_action_descriptions(&mut self) {
        self.vector_action_descriptions.clear();
    }

    // Param is passed by value, moved
    pub fn set_vector_action_descriptions(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.vector_action_descriptions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_vector_action_descriptions(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.vector_action_descriptions
    }

    // Take field
    pub fn take_vector_action_descriptions(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.vector_action_descriptions, ::protobuf::RepeatedField::new())
    }

    // .communicator_objects.SpaceTypeProto vector_action_space_type = 6;


    pub fn get_vector_action_space_type(&self) -> super::space_type::SpaceTypeProto {
        self.vector_action_space_type
    }
    pub fn clear_vector_action_space_type(&mut self) {
        self.vector_action_space_type = super::space_type::SpaceTypeProto::discrete;
    }

    // Param is passed by value, moved
    pub fn set_vector_action_space_type(&mut self, v: super::space_type::SpaceTypeProto) {
        self.vector_action_space_type = v;
    }

    // string brain_name = 7;


    pub fn get_brain_name(&self) -> &str {
        &self.brain_name
    }
    pub fn clear_brain_name(&mut self) {
        self.brain_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_brain_name(&mut self, v: ::std::string::String) {
        self.brain_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_brain_name(&mut self) -> &mut ::std::string::String {
        &mut self.brain_name
    }

    // Take field
    pub fn take_brain_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.brain_name, ::std::string::String::new())
    }

    // bool is_training = 8;


    pub fn get_is_training(&self) -> bool {
        self.is_training
    }
    pub fn clear_is_training(&mut self) {
        self.is_training = false;
    }

    // Param is passed by value, moved
    pub fn set_is_training(&mut self, v: bool) {
        self.is_training = v;
    }
}

impl ::protobuf::Message for BrainParametersProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                3 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.vector_action_size)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.vector_action_descriptions)?;
                },
                6 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.vector_action_space_type, 6, &mut self.unknown_fields)?
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.brain_name)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_training = tmp;
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
        for value in &self.vector_action_size {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.vector_action_descriptions {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        if self.vector_action_space_type != super::space_type::SpaceTypeProto::discrete {
            my_size += ::protobuf::rt::enum_size(6, self.vector_action_space_type);
        }
        if !self.brain_name.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.brain_name);
        }
        if self.is_training != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.vector_action_size {
            os.write_int32(3, *v)?;
        };
        for v in &self.vector_action_descriptions {
            os.write_string(5, &v)?;
        };
        if self.vector_action_space_type != super::space_type::SpaceTypeProto::discrete {
            os.write_enum(6, ::protobuf::ProtobufEnum::value(&self.vector_action_space_type))?;
        }
        if !self.brain_name.is_empty() {
            os.write_string(7, &self.brain_name)?;
        }
        if self.is_training != false {
            os.write_bool(8, self.is_training)?;
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

    fn new() -> BrainParametersProto {
        BrainParametersProto::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "vector_action_size",
                |m: &BrainParametersProto| { &m.vector_action_size },
                |m: &mut BrainParametersProto| { &mut m.vector_action_size },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "vector_action_descriptions",
                |m: &BrainParametersProto| { &m.vector_action_descriptions },
                |m: &mut BrainParametersProto| { &mut m.vector_action_descriptions },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::space_type::SpaceTypeProto>>(
                "vector_action_space_type",
                |m: &BrainParametersProto| { &m.vector_action_space_type },
                |m: &mut BrainParametersProto| { &mut m.vector_action_space_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "brain_name",
                |m: &BrainParametersProto| { &m.brain_name },
                |m: &mut BrainParametersProto| { &mut m.brain_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "is_training",
                |m: &BrainParametersProto| { &m.is_training },
                |m: &mut BrainParametersProto| { &mut m.is_training },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<BrainParametersProto>(
                "BrainParametersProto",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static BrainParametersProto {
        static instance: ::protobuf::rt::LazyV2<BrainParametersProto> = ::protobuf::rt::LazyV2::INIT;
        instance.get(BrainParametersProto::new)
    }
}

impl ::protobuf::Clear for BrainParametersProto {
    fn clear(&mut self) {
        self.vector_action_size.clear();
        self.vector_action_descriptions.clear();
        self.vector_action_space_type = super::space_type::SpaceTypeProto::discrete;
        self.brain_name.clear();
        self.is_training = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BrainParametersProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BrainParametersProto {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n9mlagents_envs/communicator_objects/brain_parameters.proto\x12\x14comm\
    unicator_objects\x1a3mlagents_envs/communicator_objects/space_type.proto\
    \"\xb3\x02\n\x14BrainParametersProto\x12,\n\x12vector_action_size\x18\
    \x03\x20\x03(\x05R\x10vectorActionSize\x12<\n\x1avector_action_descripti\
    ons\x18\x05\x20\x03(\tR\x18vectorActionDescriptions\x12]\n\x18vector_act\
    ion_space_type\x18\x06\x20\x01(\x0e2$.communicator_objects.SpaceTypeProt\
    oR\x15vectorActionSpaceType\x12\x1d\n\nbrain_name\x18\x07\x20\x01(\tR\tb\
    rainName\x12\x1f\n\x0bis_training\x18\x08\x20\x01(\x08R\nisTrainingJ\x04\
    \x08\x01\x10\x02J\x04\x08\x02\x10\x03J\x04\x08\x04\x10\x05B%\xaa\x02\"Un\
    ity.MLAgents.CommunicatorObjectsJ\xc6\x05\n\x06\x12\x04\0\0\x10\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\0=\n\x08\n\
    \x01\x08\x12\x03\x04\0?\n\t\n\x02\x08%\x12\x03\x04\0?\n\x08\n\x01\x02\
    \x12\x03\x05\0\x1d\n\n\n\x02\x04\0\x12\x04\x07\0\x10\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\x07\x08\x1c\n;\n\x03\x04\0\t\x12\x03\x08\x04\x0f\"/\x20depr\
    ecated\x20int32\x20vector_observation_size\x20=\x201;\n\n\x0b\n\x04\x04\
    \0\t\0\x12\x03\x08\r\x0e\n\x0c\n\x05\x04\0\t\0\x01\x12\x03\x08\r\x0e\n\
    \x0c\n\x05\x04\0\t\0\x02\x12\x03\x08\r\x0e\nC\n\x03\x04\0\t\x12\x03\t\
    \x04\x0f\"7\x20deprecated\x20int32\x20num_stacked_vector_observations\
    \x20=\x202;\n\n\x0b\n\x04\x04\0\t\x01\x12\x03\t\r\x0e\n\x0c\n\x05\x04\0\
    \t\x01\x01\x12\x03\t\r\x0e\n\x0c\n\x05\x04\0\t\x01\x02\x12\x03\t\r\x0e\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\n\x04*\n\x0c\n\x05\x04\0\x02\0\x04\x12\
    \x03\n\x04\x0c\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\n\r\x12\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03\n\x13%\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\n()\
    \nD\n\x03\x04\0\t\x12\x03\x0b\x04\x0f\"8\x20deprecated\x20repeated\x20Re\
    solutionProto\x20camera_resolutions\n\n\x0b\n\x04\x04\0\t\x02\x12\x03\
    \x0b\r\x0e\n\x0c\n\x05\x04\0\t\x02\x01\x12\x03\x0b\r\x0e\n\x0c\n\x05\x04\
    \0\t\x02\x02\x12\x03\x0b\r\x0e\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0c\x04\
    3\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x0c\x04\x0c\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03\x0c\r\x13\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0c\
    \x14.\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0c12\n\x0b\n\x04\x04\0\x02\
    \x02\x12\x03\r\x040\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\r\x04\x12\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x03\r\x13+\n\x0c\n\x05\x04\0\x02\x02\
    \x03\x12\x03\r./\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x0e\x04\x1a\n\x0c\n\
    \x05\x04\0\x02\x03\x05\x12\x03\x0e\x04\n\n\x0c\n\x05\x04\0\x02\x03\x01\
    \x12\x03\x0e\x0b\x15\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x0e\x18\x19\n\
    \x0b\n\x04\x04\0\x02\x04\x12\x03\x0f\x04\x19\n\x0c\n\x05\x04\0\x02\x04\
    \x05\x12\x03\x0f\x04\x08\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x0f\t\x14\
    \n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x0f\x17\x18b\x06proto3\
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
