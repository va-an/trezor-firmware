// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc 3.19.6
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `messages-thp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:hw.trezor.messages.thp.ThpCredentialMetadata)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ThpCredentialMetadata {
    // message fields
    // @@protoc_insertion_point(field:hw.trezor.messages.thp.ThpCredentialMetadata.host_name)
    pub host_name: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:hw.trezor.messages.thp.ThpCredentialMetadata.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ThpCredentialMetadata {
    fn default() -> &'a ThpCredentialMetadata {
        <ThpCredentialMetadata as ::protobuf::Message>::default_instance()
    }
}

impl ThpCredentialMetadata {
    pub fn new() -> ThpCredentialMetadata {
        ::std::default::Default::default()
    }

    // optional string host_name = 1;

    pub fn host_name(&self) -> &str {
        match self.host_name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_host_name(&mut self) {
        self.host_name = ::std::option::Option::None;
    }

    pub fn has_host_name(&self) -> bool {
        self.host_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_name(&mut self, v: ::std::string::String) {
        self.host_name = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host_name(&mut self) -> &mut ::std::string::String {
        if self.host_name.is_none() {
            self.host_name = ::std::option::Option::Some(::std::string::String::new());
        }
        self.host_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_host_name(&mut self) -> ::std::string::String {
        self.host_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "host_name",
            |m: &ThpCredentialMetadata| { &m.host_name },
            |m: &mut ThpCredentialMetadata| { &mut m.host_name },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ThpCredentialMetadata>(
            "ThpCredentialMetadata",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ThpCredentialMetadata {
    const NAME: &'static str = "ThpCredentialMetadata";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.host_name = ::std::option::Option::Some(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.host_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.host_name.as_ref() {
            os.write_string(1, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ThpCredentialMetadata {
        ThpCredentialMetadata::new()
    }

    fn clear(&mut self) {
        self.host_name = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ThpCredentialMetadata {
        static instance: ThpCredentialMetadata = ThpCredentialMetadata {
            host_name: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ThpCredentialMetadata {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ThpCredentialMetadata").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ThpCredentialMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ThpCredentialMetadata {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:hw.trezor.messages.thp.ThpPairingCredential)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ThpPairingCredential {
    // message fields
    // @@protoc_insertion_point(field:hw.trezor.messages.thp.ThpPairingCredential.cred_metadata)
    pub cred_metadata: ::protobuf::MessageField<ThpCredentialMetadata>,
    // @@protoc_insertion_point(field:hw.trezor.messages.thp.ThpPairingCredential.mac)
    pub mac: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:hw.trezor.messages.thp.ThpPairingCredential.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ThpPairingCredential {
    fn default() -> &'a ThpPairingCredential {
        <ThpPairingCredential as ::protobuf::Message>::default_instance()
    }
}

impl ThpPairingCredential {
    pub fn new() -> ThpPairingCredential {
        ::std::default::Default::default()
    }

    // optional bytes mac = 2;

    pub fn mac(&self) -> &[u8] {
        match self.mac.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_mac(&mut self) {
        self.mac = ::std::option::Option::None;
    }

    pub fn has_mac(&self) -> bool {
        self.mac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mac(&mut self, v: ::std::vec::Vec<u8>) {
        self.mac = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mac(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.mac.is_none() {
            self.mac = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.mac.as_mut().unwrap()
    }

    // Take field
    pub fn take_mac(&mut self) -> ::std::vec::Vec<u8> {
        self.mac.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ThpCredentialMetadata>(
            "cred_metadata",
            |m: &ThpPairingCredential| { &m.cred_metadata },
            |m: &mut ThpPairingCredential| { &mut m.cred_metadata },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "mac",
            |m: &ThpPairingCredential| { &m.mac },
            |m: &mut ThpPairingCredential| { &mut m.mac },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ThpPairingCredential>(
            "ThpPairingCredential",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ThpPairingCredential {
    const NAME: &'static str = "ThpPairingCredential";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.cred_metadata)?;
                },
                18 => {
                    self.mac = ::std::option::Option::Some(is.read_bytes()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.cred_metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.mac.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.cred_metadata.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.mac.as_ref() {
            os.write_bytes(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ThpPairingCredential {
        ThpPairingCredential::new()
    }

    fn clear(&mut self) {
        self.cred_metadata.clear();
        self.mac = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ThpPairingCredential {
        static instance: ThpPairingCredential = ThpPairingCredential {
            cred_metadata: ::protobuf::MessageField::none(),
            mac: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ThpPairingCredential {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ThpPairingCredential").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ThpPairingCredential {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ThpPairingCredential {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:hw.trezor.messages.thp.ThpAuthenticatedCredentialData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ThpAuthenticatedCredentialData {
    // message fields
    // @@protoc_insertion_point(field:hw.trezor.messages.thp.ThpAuthenticatedCredentialData.host_static_pubkey)
    pub host_static_pubkey: ::std::option::Option<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:hw.trezor.messages.thp.ThpAuthenticatedCredentialData.cred_metadata)
    pub cred_metadata: ::protobuf::MessageField<ThpCredentialMetadata>,
    // special fields
    // @@protoc_insertion_point(special_field:hw.trezor.messages.thp.ThpAuthenticatedCredentialData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ThpAuthenticatedCredentialData {
    fn default() -> &'a ThpAuthenticatedCredentialData {
        <ThpAuthenticatedCredentialData as ::protobuf::Message>::default_instance()
    }
}

impl ThpAuthenticatedCredentialData {
    pub fn new() -> ThpAuthenticatedCredentialData {
        ::std::default::Default::default()
    }

    // optional bytes host_static_pubkey = 1;

    pub fn host_static_pubkey(&self) -> &[u8] {
        match self.host_static_pubkey.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_host_static_pubkey(&mut self) {
        self.host_static_pubkey = ::std::option::Option::None;
    }

    pub fn has_host_static_pubkey(&self) -> bool {
        self.host_static_pubkey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_static_pubkey(&mut self, v: ::std::vec::Vec<u8>) {
        self.host_static_pubkey = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host_static_pubkey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.host_static_pubkey.is_none() {
            self.host_static_pubkey = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.host_static_pubkey.as_mut().unwrap()
    }

    // Take field
    pub fn take_host_static_pubkey(&mut self) -> ::std::vec::Vec<u8> {
        self.host_static_pubkey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "host_static_pubkey",
            |m: &ThpAuthenticatedCredentialData| { &m.host_static_pubkey },
            |m: &mut ThpAuthenticatedCredentialData| { &mut m.host_static_pubkey },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ThpCredentialMetadata>(
            "cred_metadata",
            |m: &ThpAuthenticatedCredentialData| { &m.cred_metadata },
            |m: &mut ThpAuthenticatedCredentialData| { &mut m.cred_metadata },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ThpAuthenticatedCredentialData>(
            "ThpAuthenticatedCredentialData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ThpAuthenticatedCredentialData {
    const NAME: &'static str = "ThpAuthenticatedCredentialData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.host_static_pubkey = ::std::option::Option::Some(is.read_bytes()?);
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.cred_metadata)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.host_static_pubkey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.cred_metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.host_static_pubkey.as_ref() {
            os.write_bytes(1, v)?;
        }
        if let Some(v) = self.cred_metadata.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ThpAuthenticatedCredentialData {
        ThpAuthenticatedCredentialData::new()
    }

    fn clear(&mut self) {
        self.host_static_pubkey = ::std::option::Option::None;
        self.cred_metadata.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ThpAuthenticatedCredentialData {
        static instance: ThpAuthenticatedCredentialData = ThpAuthenticatedCredentialData {
            host_static_pubkey: ::std::option::Option::None,
            cred_metadata: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ThpAuthenticatedCredentialData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ThpAuthenticatedCredentialData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ThpAuthenticatedCredentialData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ThpAuthenticatedCredentialData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12messages-thp.proto\x12\x16hw.trezor.messages.thp\x1a\roptions.prot\
    o\":\n\x15ThpCredentialMetadata\x12\x1b\n\thost_name\x18\x01\x20\x01(\tR\
    \x08hostName:\x04\x98\xb2\x19\x01\"\x82\x01\n\x14ThpPairingCredential\
    \x12R\n\rcred_metadata\x18\x01\x20\x01(\x0b2-.hw.trezor.messages.thp.Thp\
    CredentialMetadataR\x0ccredMetadata\x12\x10\n\x03mac\x18\x02\x20\x01(\
    \x0cR\x03mac:\x04\x98\xb2\x19\x01\"\xa8\x01\n\x1eThpAuthenticatedCredent\
    ialData\x12,\n\x12host_static_pubkey\x18\x01\x20\x01(\x0cR\x10hostStatic\
    Pubkey\x12R\n\rcred_metadata\x18\x02\x20\x01(\x0b2-.hw.trezor.messages.t\
    hp.ThpCredentialMetadataR\x0ccredMetadata:\x04\x98\xb2\x19\x01B;\n#com.s\
    atoshilabs.trezor.lib.protobufB\x10TrezorMessageThp\x80\xa6\x1d\x01\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::options::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(ThpCredentialMetadata::generated_message_descriptor_data());
            messages.push(ThpPairingCredential::generated_message_descriptor_data());
            messages.push(ThpAuthenticatedCredentialData::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
