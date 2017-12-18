// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Error {
    // message fields
    pub code: i32,
    pub message: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Error {}

impl Error {
    pub fn new() -> Error {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Error {
        static mut instance: ::protobuf::lazy::Lazy<Error> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Error,
        };
        unsafe {
            instance.get(Error::new)
        }
    }

    // int32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: i32) {
        self.code = v;
    }

    pub fn get_code(&self) -> i32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &i32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut i32 {
        &mut self.code
    }

    // string message = 2;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }
}

impl ::protobuf::Message for Error {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_int32(1, self.code)?;
        }
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Error {
    fn new() -> Error {
        Error::new()
    }

    fn descriptor_static(_: ::std::option::Option<Error>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "code",
                    Error::get_code_for_reflect,
                    Error::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    Error::get_message_for_reflect,
                    Error::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Error>(
                    "Error",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Error {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Error {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateTokenReq {
    // message fields
    pub issuer: ::std::string::String,
    pub subject: ::std::string::String,
    pub audience: ::std::string::String,
    pub issued_at: i64,
    pub expires_at: i64,
    pub not_before: i64,
    pub scopes: ::protobuf::RepeatedField<::std::string::String>,
    pub additionals: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateTokenReq {}

impl CreateTokenReq {
    pub fn new() -> CreateTokenReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateTokenReq {
        static mut instance: ::protobuf::lazy::Lazy<CreateTokenReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateTokenReq,
        };
        unsafe {
            instance.get(CreateTokenReq::new)
        }
    }

    // string issuer = 1;

    pub fn clear_issuer(&mut self) {
        self.issuer.clear();
    }

    // Param is passed by value, moved
    pub fn set_issuer(&mut self, v: ::std::string::String) {
        self.issuer = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_issuer(&mut self) -> &mut ::std::string::String {
        &mut self.issuer
    }

    // Take field
    pub fn take_issuer(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.issuer, ::std::string::String::new())
    }

    pub fn get_issuer(&self) -> &str {
        &self.issuer
    }

    fn get_issuer_for_reflect(&self) -> &::std::string::String {
        &self.issuer
    }

    fn mut_issuer_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.issuer
    }

    // string subject = 2;

    pub fn clear_subject(&mut self) {
        self.subject.clear();
    }

    // Param is passed by value, moved
    pub fn set_subject(&mut self, v: ::std::string::String) {
        self.subject = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subject(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // Take field
    pub fn take_subject(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subject, ::std::string::String::new())
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    fn get_subject_for_reflect(&self) -> &::std::string::String {
        &self.subject
    }

    fn mut_subject_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // string audience = 3;

    pub fn clear_audience(&mut self) {
        self.audience.clear();
    }

    // Param is passed by value, moved
    pub fn set_audience(&mut self, v: ::std::string::String) {
        self.audience = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_audience(&mut self) -> &mut ::std::string::String {
        &mut self.audience
    }

    // Take field
    pub fn take_audience(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.audience, ::std::string::String::new())
    }

    pub fn get_audience(&self) -> &str {
        &self.audience
    }

    fn get_audience_for_reflect(&self) -> &::std::string::String {
        &self.audience
    }

    fn mut_audience_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.audience
    }

    // int64 issued_at = 4;

    pub fn clear_issued_at(&mut self) {
        self.issued_at = 0;
    }

    // Param is passed by value, moved
    pub fn set_issued_at(&mut self, v: i64) {
        self.issued_at = v;
    }

    pub fn get_issued_at(&self) -> i64 {
        self.issued_at
    }

    fn get_issued_at_for_reflect(&self) -> &i64 {
        &self.issued_at
    }

    fn mut_issued_at_for_reflect(&mut self) -> &mut i64 {
        &mut self.issued_at
    }

    // int64 expires_at = 5;

    pub fn clear_expires_at(&mut self) {
        self.expires_at = 0;
    }

    // Param is passed by value, moved
    pub fn set_expires_at(&mut self, v: i64) {
        self.expires_at = v;
    }

    pub fn get_expires_at(&self) -> i64 {
        self.expires_at
    }

    fn get_expires_at_for_reflect(&self) -> &i64 {
        &self.expires_at
    }

    fn mut_expires_at_for_reflect(&mut self) -> &mut i64 {
        &mut self.expires_at
    }

    // int64 not_before = 6;

    pub fn clear_not_before(&mut self) {
        self.not_before = 0;
    }

    // Param is passed by value, moved
    pub fn set_not_before(&mut self, v: i64) {
        self.not_before = v;
    }

    pub fn get_not_before(&self) -> i64 {
        self.not_before
    }

    fn get_not_before_for_reflect(&self) -> &i64 {
        &self.not_before
    }

    fn mut_not_before_for_reflect(&mut self) -> &mut i64 {
        &mut self.not_before
    }

    // repeated string scopes = 7;

    pub fn clear_scopes(&mut self) {
        self.scopes.clear();
    }

    // Param is passed by value, moved
    pub fn set_scopes(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.scopes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_scopes(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.scopes
    }

    // Take field
    pub fn take_scopes(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.scopes, ::protobuf::RepeatedField::new())
    }

    pub fn get_scopes(&self) -> &[::std::string::String] {
        &self.scopes
    }

    fn get_scopes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.scopes
    }

    fn mut_scopes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.scopes
    }

    // repeated .jwt.CreateTokenReq.AdditionalsEntry additionals = 8;

    pub fn clear_additionals(&mut self) {
        self.additionals.clear();
    }

    // Param is passed by value, moved
    pub fn set_additionals(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.additionals = v;
    }

    // Mutable pointer to the field.
    pub fn mut_additionals(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.additionals
    }

    // Take field
    pub fn take_additionals(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.additionals, ::std::collections::HashMap::new())
    }

    pub fn get_additionals(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.additionals
    }

    fn get_additionals_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.additionals
    }

    fn mut_additionals_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.additionals
    }
}

impl ::protobuf::Message for CreateTokenReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.issuer)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subject)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.audience)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.issued_at = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.expires_at = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.not_before = tmp;
                },
                7 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.scopes)?;
                },
                8 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.additionals)?;
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
        if !self.issuer.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.issuer);
        }
        if !self.subject.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.subject);
        }
        if !self.audience.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.audience);
        }
        if self.issued_at != 0 {
            my_size += ::protobuf::rt::value_size(4, self.issued_at, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.expires_at != 0 {
            my_size += ::protobuf::rt::value_size(5, self.expires_at, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.not_before != 0 {
            my_size += ::protobuf::rt::value_size(6, self.not_before, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.scopes {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(8, &self.additionals);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.issuer.is_empty() {
            os.write_string(1, &self.issuer)?;
        }
        if !self.subject.is_empty() {
            os.write_string(2, &self.subject)?;
        }
        if !self.audience.is_empty() {
            os.write_string(3, &self.audience)?;
        }
        if self.issued_at != 0 {
            os.write_int64(4, self.issued_at)?;
        }
        if self.expires_at != 0 {
            os.write_int64(5, self.expires_at)?;
        }
        if self.not_before != 0 {
            os.write_int64(6, self.not_before)?;
        }
        for v in &self.scopes {
            os.write_string(7, &v)?;
        };
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(8, &self.additionals, os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateTokenReq {
    fn new() -> CreateTokenReq {
        CreateTokenReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateTokenReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "issuer",
                    CreateTokenReq::get_issuer_for_reflect,
                    CreateTokenReq::mut_issuer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "subject",
                    CreateTokenReq::get_subject_for_reflect,
                    CreateTokenReq::mut_subject_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "audience",
                    CreateTokenReq::get_audience_for_reflect,
                    CreateTokenReq::mut_audience_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "issued_at",
                    CreateTokenReq::get_issued_at_for_reflect,
                    CreateTokenReq::mut_issued_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "expires_at",
                    CreateTokenReq::get_expires_at_for_reflect,
                    CreateTokenReq::mut_expires_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "not_before",
                    CreateTokenReq::get_not_before_for_reflect,
                    CreateTokenReq::mut_not_before_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "scopes",
                    CreateTokenReq::get_scopes_for_reflect,
                    CreateTokenReq::mut_scopes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                    "additionals",
                    CreateTokenReq::get_additionals_for_reflect,
                    CreateTokenReq::mut_additionals_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateTokenReq>(
                    "CreateTokenReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateTokenReq {
    fn clear(&mut self) {
        self.clear_issuer();
        self.clear_subject();
        self.clear_audience();
        self.clear_issued_at();
        self.clear_expires_at();
        self.clear_not_before();
        self.clear_scopes();
        self.clear_additionals();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateTokenReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateTokenReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ValidationReq {
    // message fields
    pub token: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ValidationReq {}

impl ValidationReq {
    pub fn new() -> ValidationReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ValidationReq {
        static mut instance: ::protobuf::lazy::Lazy<ValidationReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ValidationReq,
        };
        unsafe {
            instance.get(ValidationReq::new)
        }
    }

    // string token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        &mut self.token
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.token, ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    fn get_token_for_reflect(&self) -> &::std::string::String {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.token
    }
}

impl ::protobuf::Message for ValidationReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.token)?;
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
        if !self.token.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.token);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.token.is_empty() {
            os.write_string(1, &self.token)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ValidationReq {
    fn new() -> ValidationReq {
        ValidationReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<ValidationReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    ValidationReq::get_token_for_reflect,
                    ValidationReq::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ValidationReq>(
                    "ValidationReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ValidationReq {
    fn clear(&mut self) {
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ValidationReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ValidationReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TokenRes {
    // message fields
    pub error: ::protobuf::SingularPtrField<Error>,
    pub token: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TokenRes {}

impl TokenRes {
    pub fn new() -> TokenRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TokenRes {
        static mut instance: ::protobuf::lazy::Lazy<TokenRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TokenRes,
        };
        unsafe {
            instance.get(TokenRes::new)
        }
    }

    // .jwt.Error error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: Error) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut Error {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(|| Error::new())
    }

    pub fn get_error(&self) -> &Error {
        self.error.as_ref().unwrap_or_else(|| Error::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<Error> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Error> {
        &mut self.error
    }

    // string token = 2;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        &mut self.token
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.token, ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    fn get_token_for_reflect(&self) -> &::std::string::String {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.token
    }
}

impl ::protobuf::Message for TokenRes {
    fn is_initialized(&self) -> bool {
        for v in &self.error {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.token)?;
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
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.token.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.token);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.token.is_empty() {
            os.write_string(2, &self.token)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TokenRes {
    fn new() -> TokenRes {
        TokenRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<TokenRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Error>>(
                    "error",
                    TokenRes::get_error_for_reflect,
                    TokenRes::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    TokenRes::get_token_for_reflect,
                    TokenRes::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TokenRes>(
                    "TokenRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TokenRes {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TokenRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TokenRes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tjwt.proto\x12\x03jwt\"5\n\x05Error\x12\x12\n\x04code\x18\x01\x20\x01\
    (\x05R\x04code\x12\x18\n\x07message\x18\x02\x20\x01(\tR\x07message\"\xd9\
    \x02\n\x0eCreateTokenReq\x12\x16\n\x06issuer\x18\x01\x20\x01(\tR\x06issu\
    er\x12\x18\n\x07subject\x18\x02\x20\x01(\tR\x07subject\x12\x1a\n\x08audi\
    ence\x18\x03\x20\x01(\tR\x08audience\x12\x1b\n\tissued_at\x18\x04\x20\
    \x01(\x03R\x08issuedAt\x12\x1d\n\nexpires_at\x18\x05\x20\x01(\x03R\texpi\
    resAt\x12\x1d\n\nnot_before\x18\x06\x20\x01(\x03R\tnotBefore\x12\x16\n\
    \x06scopes\x18\x07\x20\x03(\tR\x06scopes\x12F\n\x0badditionals\x18\x08\
    \x20\x03(\x0b2$.jwt.CreateTokenReq.AdditionalsEntryR\x0badditionals\x1a>\
    \n\x10AdditionalsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\
    \x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01\"%\n\rValidationR\
    eq\x12\x14\n\x05token\x18\x01\x20\x01(\tR\x05token\"B\n\x08TokenRes\x12\
    \x20\n\x05error\x18\x01\x20\x01(\x0b2\n.jwt.ErrorR\x05error\x12\x14\n\
    \x05token\x18\x02\x20\x01(\tR\x05token2c\n\x08Tokenizr\x12*\n\x04Sign\
    \x12\x13.jwt.CreateTokenReq\x1a\r.jwt.TokenRes\x12+\n\x06Verify\x12\x12.\
    jwt.ValidationReq\x1a\r.jwt.TokenResb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
