// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct ScheduleTaskRequest {
    // message fields
    execution_id: ::std::option::Option<u32>,
    priority: ::std::option::Option<u32>,
    url: ::protobuf::SingularField<::std::string::String>,
    url_type: ::std::option::Option<ScheduleTaskRequest_UrlType>,
    operation: ::std::option::Option<ScheduleTaskRequest_Operation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScheduleTaskRequest {}

impl ScheduleTaskRequest {
    pub fn new() -> ScheduleTaskRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScheduleTaskRequest {
        static mut instance: ::protobuf::lazy::Lazy<ScheduleTaskRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScheduleTaskRequest,
        };
        unsafe {
            instance.get(|| {
                ScheduleTaskRequest {
                    execution_id: ::std::option::Option::None,
                    priority: ::std::option::Option::None,
                    url: ::protobuf::SingularField::none(),
                    url_type: ::std::option::Option::None,
                    operation: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 execution_id = 1;

    pub fn clear_execution_id(&mut self) {
        self.execution_id = ::std::option::Option::None;
    }

    pub fn has_execution_id(&self) -> bool {
        self.execution_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_execution_id(&mut self, v: u32) {
        self.execution_id = ::std::option::Option::Some(v);
    }

    pub fn get_execution_id(&self) -> u32 {
        self.execution_id.unwrap_or(0)
    }

    // optional uint32 priority = 2;

    pub fn clear_priority(&mut self) {
        self.priority = ::std::option::Option::None;
    }

    pub fn has_priority(&self) -> bool {
        self.priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_priority(&mut self, v: u32) {
        self.priority = ::std::option::Option::Some(v);
    }

    pub fn get_priority(&self) -> u32 {
        self.priority.unwrap_or(0)
    }

    // optional string url = 3;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        };
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .ScheduleTaskRequest.UrlType url_type = 4;

    pub fn clear_url_type(&mut self) {
        self.url_type = ::std::option::Option::None;
    }

    pub fn has_url_type(&self) -> bool {
        self.url_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url_type(&mut self, v: ScheduleTaskRequest_UrlType) {
        self.url_type = ::std::option::Option::Some(v);
    }

    pub fn get_url_type(&self) -> ScheduleTaskRequest_UrlType {
        self.url_type.unwrap_or(ScheduleTaskRequest_UrlType::Web)
    }

    // optional .ScheduleTaskRequest.Operation operation = 5;

    pub fn clear_operation(&mut self) {
        self.operation = ::std::option::Option::None;
    }

    pub fn has_operation(&self) -> bool {
        self.operation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_operation(&mut self, v: ScheduleTaskRequest_Operation) {
        self.operation = ::std::option::Option::Some(v);
    }

    pub fn get_operation(&self) -> ScheduleTaskRequest_Operation {
        self.operation.unwrap_or(ScheduleTaskRequest_Operation::Crawl)
    }
}

impl ::protobuf::Message for ScheduleTaskRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.execution_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.priority = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.url_type = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.operation = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.execution_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.priority {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.url {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.url_type {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in &self.operation {
            my_size += ::protobuf::rt::enum_size(5, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.execution_id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.priority {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.url.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.url_type {
            try!(os.write_enum(4, v.value()));
        };
        if let Some(v) = self.operation {
            try!(os.write_enum(5, v.value()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ScheduleTaskRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScheduleTaskRequest {
    fn new() -> ScheduleTaskRequest {
        ScheduleTaskRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScheduleTaskRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "execution_id",
                    ScheduleTaskRequest::has_execution_id,
                    ScheduleTaskRequest::get_execution_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "priority",
                    ScheduleTaskRequest::has_priority,
                    ScheduleTaskRequest::get_priority,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "url",
                    ScheduleTaskRequest::has_url,
                    ScheduleTaskRequest::get_url,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "url_type",
                    ScheduleTaskRequest::has_url_type,
                    ScheduleTaskRequest::get_url_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "operation",
                    ScheduleTaskRequest::has_operation,
                    ScheduleTaskRequest::get_operation,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScheduleTaskRequest>(
                    "ScheduleTaskRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScheduleTaskRequest {
    fn clear(&mut self) {
        self.clear_execution_id();
        self.clear_priority();
        self.clear_url();
        self.clear_url_type();
        self.clear_operation();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ScheduleTaskRequest {
    fn eq(&self, other: &ScheduleTaskRequest) -> bool {
        self.execution_id == other.execution_id &&
        self.priority == other.priority &&
        self.url == other.url &&
        self.url_type == other.url_type &&
        self.operation == other.operation &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ScheduleTaskRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ScheduleTaskRequest_UrlType {
    Web = 0,
    TorHiddenService = 1,
}

impl ::protobuf::ProtobufEnum for ScheduleTaskRequest_UrlType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ScheduleTaskRequest_UrlType> {
        match value {
            0 => ::std::option::Option::Some(ScheduleTaskRequest_UrlType::Web),
            1 => ::std::option::Option::Some(ScheduleTaskRequest_UrlType::TorHiddenService),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ScheduleTaskRequest_UrlType] = &[
            ScheduleTaskRequest_UrlType::Web,
            ScheduleTaskRequest_UrlType::TorHiddenService,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ScheduleTaskRequest_UrlType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ScheduleTaskRequest_UrlType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ScheduleTaskRequest_UrlType {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ScheduleTaskRequest_Operation {
    Crawl = 0,
    Scrape = 1,
}

impl ::protobuf::ProtobufEnum for ScheduleTaskRequest_Operation {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ScheduleTaskRequest_Operation> {
        match value {
            0 => ::std::option::Option::Some(ScheduleTaskRequest_Operation::Crawl),
            1 => ::std::option::Option::Some(ScheduleTaskRequest_Operation::Scrape),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ScheduleTaskRequest_Operation] = &[
            ScheduleTaskRequest_Operation::Crawl,
            ScheduleTaskRequest_Operation::Scrape,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ScheduleTaskRequest_Operation>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ScheduleTaskRequest_Operation", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ScheduleTaskRequest_Operation {
}

#[derive(Clone,Default)]
pub struct ScheduleTaskReply {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScheduleTaskReply {}

impl ScheduleTaskReply {
    pub fn new() -> ScheduleTaskReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScheduleTaskReply {
        static mut instance: ::protobuf::lazy::Lazy<ScheduleTaskReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScheduleTaskReply,
        };
        unsafe {
            instance.get(|| {
                ScheduleTaskReply {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for ScheduleTaskReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ScheduleTaskReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScheduleTaskReply {
    fn new() -> ScheduleTaskReply {
        ScheduleTaskReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScheduleTaskReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ScheduleTaskReply>(
                    "ScheduleTaskReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScheduleTaskReply {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ScheduleTaskReply {
    fn eq(&self, other: &ScheduleTaskReply) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ScheduleTaskReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StatsRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatsRequest {}

impl StatsRequest {
    pub fn new() -> StatsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatsRequest {
        static mut instance: ::protobuf::lazy::Lazy<StatsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatsRequest,
        };
        unsafe {
            instance.get(|| {
                StatsRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for StatsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StatsRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StatsRequest {
    fn new() -> StatsRequest {
        StatsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StatsRequest>(
                    "StatsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatsRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StatsRequest {
    fn eq(&self, other: &StatsRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StatsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StatsReply {
    // message fields
    frontier_length: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatsReply {}

impl StatsReply {
    pub fn new() -> StatsReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatsReply {
        static mut instance: ::protobuf::lazy::Lazy<StatsReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatsReply,
        };
        unsafe {
            instance.get(|| {
                StatsReply {
                    frontier_length: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 frontier_length = 1;

    pub fn clear_frontier_length(&mut self) {
        self.frontier_length = ::std::option::Option::None;
    }

    pub fn has_frontier_length(&self) -> bool {
        self.frontier_length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frontier_length(&mut self, v: u64) {
        self.frontier_length = ::std::option::Option::Some(v);
    }

    pub fn get_frontier_length(&self) -> u64 {
        self.frontier_length.unwrap_or(0)
    }
}

impl ::protobuf::Message for StatsReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.frontier_length = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.frontier_length {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.frontier_length {
            try!(os.write_uint64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StatsReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StatsReply {
    fn new() -> StatsReply {
        StatsReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatsReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "frontier_length",
                    StatsReply::has_frontier_length,
                    StatsReply::get_frontier_length,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatsReply>(
                    "StatsReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatsReply {
    fn clear(&mut self) {
        self.clear_frontier_length();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StatsReply {
    fn eq(&self, other: &StatsReply) -> bool {
        self.frontier_length == other.frontier_length &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StatsReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x70, 0x6f, 0x6c, 0x7a, 0x61,
    0x74, 0x5f, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xab, 0x02, 0x0a, 0x13, 0x53,
    0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x54, 0x61, 0x73, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74,
    0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74,
    0x79, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03,
    0x75, 0x72, 0x6c, 0x12, 0x37, 0x0a, 0x08, 0x75, 0x72, 0x6c, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1c, 0x2e, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65,
    0x54, 0x61, 0x73, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x55, 0x72, 0x6c, 0x54,
    0x79, 0x70, 0x65, 0x52, 0x07, 0x75, 0x72, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x12, 0x3c, 0x0a, 0x09,
    0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x1e, 0x2e, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x54, 0x61, 0x73, 0x6b, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x09, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x28, 0x0a, 0x07, 0x55, 0x72,
    0x6c, 0x54, 0x79, 0x70, 0x65, 0x12, 0x07, 0x0a, 0x03, 0x57, 0x65, 0x62, 0x10, 0x00, 0x12, 0x14,
    0x0a, 0x10, 0x54, 0x6f, 0x72, 0x48, 0x69, 0x64, 0x64, 0x65, 0x6e, 0x53, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x10, 0x01, 0x22, 0x22, 0x0a, 0x09, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x09, 0x0a, 0x05, 0x43, 0x72, 0x61, 0x77, 0x6c, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06,
    0x53, 0x63, 0x72, 0x61, 0x70, 0x65, 0x10, 0x01, 0x22, 0x13, 0x0a, 0x11, 0x53, 0x63, 0x68, 0x65,
    0x64, 0x75, 0x6c, 0x65, 0x54, 0x61, 0x73, 0x6b, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x0e, 0x0a,
    0x0c, 0x53, 0x74, 0x61, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x35, 0x0a,
    0x0a, 0x53, 0x74, 0x61, 0x74, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x27, 0x0a, 0x0f, 0x66,
    0x72, 0x6f, 0x6e, 0x74, 0x69, 0x65, 0x72, 0x5f, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x0e, 0x66, 0x72, 0x6f, 0x6e, 0x74, 0x69, 0x65, 0x72, 0x4c, 0x65,
    0x6e, 0x67, 0x74, 0x68, 0x32, 0x6b, 0x0a, 0x06, 0x50, 0x6f, 0x6c, 0x7a, 0x61, 0x74, 0x12, 0x3a,
    0x0a, 0x0c, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x54, 0x61, 0x73, 0x6b, 0x12, 0x14,
    0x2e, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x54, 0x61, 0x73, 0x6b, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x1a, 0x12, 0x2e, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x54,
    0x61, 0x73, 0x6b, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x12, 0x25, 0x0a, 0x05, 0x53, 0x74,
    0x61, 0x74, 0x73, 0x12, 0x0d, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x1a, 0x0b, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22,
    0x00, 0x4a, 0xb0, 0x07, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2b, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x10, 0x0a, 0x17, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00,
    0x18, 0x01, 0x32, 0x0b, 0x0a, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x07, 0x04, 0x06, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x07, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x07, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x08, 0x04, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x08, 0x04, 0x07, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x08, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x08, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x09, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x09,
    0x04, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x09, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x09, 0x0b, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x09, 0x11, 0x12, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x04, 0x0e, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x09, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x0c, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x0d, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x10, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x10,
    0x04, 0x0e, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x10, 0x04,
    0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x0c, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x10, 0x17, 0x18, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x04, 0x01, 0x12, 0x04, 0x12, 0x04, 0x15, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x04, 0x01, 0x01, 0x12, 0x03, 0x12, 0x09, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x13, 0x08, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x01,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x13, 0x10, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x14, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x01, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x14, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12,
    0x03, 0x17, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x17,
    0x04, 0x15, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x17, 0x04,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x17, 0x0e, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x17, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x1a, 0x00, 0x1b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x1a, 0x08, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1d, 0x00, 0x1f,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x14, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x21, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x21, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x22,
    0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x22, 0x04, 0x21,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x1d, 0x1e, 0x0a, 0x1d, 0x0a, 0x02, 0x06,
    0x00, 0x12, 0x04, 0x28, 0x00, 0x2b, 0x01, 0x1a, 0x11, 0x0a, 0x20, 0x44, 0x65, 0x66, 0x69, 0x6e,
    0x65, 0x20, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00,
    0x01, 0x12, 0x03, 0x28, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x29, 0x04, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x08,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x29, 0x15, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x32, 0x43, 0x0a, 0x0b, 0x0a,
    0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x04, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x2a, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x2a, 0x24, 0x2e, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

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
