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


// interface

pub trait Polzat {
    fn ScheduleTask(&self, p: super::polzat_pb::ScheduleTaskRequest) -> ::grpc::result::GrpcResult<super::polzat_pb::ScheduleTaskReply>;

    fn Stats(&self, p: super::polzat_pb::StatsRequest) -> ::grpc::result::GrpcResult<super::polzat_pb::StatsReply>;
}

pub trait PolzatAsync {
    fn ScheduleTask(&self, p: super::polzat_pb::ScheduleTaskRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::polzat_pb::ScheduleTaskReply>;

    fn Stats(&self, p: super::polzat_pb::StatsRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::polzat_pb::StatsReply>;
}

// sync client

pub struct PolzatClient {
    async_client: PolzatAsyncClient,
}

impl PolzatClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        PolzatAsyncClient::new(host, port, tls).map(|c| {
            PolzatClient {
                async_client: c,
            }
        })
    }
}

impl Polzat for PolzatClient {
    fn ScheduleTask(&self, p: super::polzat_pb::ScheduleTaskRequest) -> ::grpc::result::GrpcResult<super::polzat_pb::ScheduleTaskReply> {
        ::futures::Future::wait(self.async_client.ScheduleTask(p))
    }

    fn Stats(&self, p: super::polzat_pb::StatsRequest) -> ::grpc::result::GrpcResult<super::polzat_pb::StatsReply> {
        ::futures::Future::wait(self.async_client.Stats(p))
    }
}

// async client

pub struct PolzatAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_ScheduleTask: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::polzat_pb::ScheduleTaskRequest, super::polzat_pb::ScheduleTaskReply>>,
    method_Stats: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::polzat_pb::StatsRequest, super::polzat_pb::StatsReply>>,
}

impl PolzatAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            PolzatAsyncClient {
                grpc_client: c,
                method_ScheduleTask: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Polzat/ScheduleTask".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Stats: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Polzat/Stats".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl PolzatAsync for PolzatAsyncClient {
    fn ScheduleTask(&self, p: super::polzat_pb::ScheduleTaskRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::polzat_pb::ScheduleTaskReply> {
        self.grpc_client.call_unary(p, self.method_ScheduleTask.clone())
    }

    fn Stats(&self, p: super::polzat_pb::StatsRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::polzat_pb::StatsReply> {
        self.grpc_client.call_unary(p, self.method_Stats.clone())
    }
}

// sync server

pub struct PolzatServer {
    async_server: PolzatAsyncServer,
}

struct PolzatServerHandlerToAsync {
    handler: ::std::sync::Arc<Polzat + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl PolzatAsync for PolzatServerHandlerToAsync {
    fn ScheduleTask(&self, p: super::polzat_pb::ScheduleTaskRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::polzat_pb::ScheduleTaskReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.ScheduleTask(p)
        })
    }

    fn Stats(&self, p: super::polzat_pb::StatsRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::polzat_pb::StatsReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Stats(p)
        })
    }
}

impl PolzatServer {
    pub fn new<H : Polzat + Send + Sync + 'static>(port: u16, h: H) -> Self {
        let h = PolzatServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        PolzatServer {
            async_server: PolzatAsyncServer::new(port, h),
        }
    }
}

// async server

pub struct PolzatAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl PolzatAsyncServer {
    pub fn new<H : PolzatAsync + 'static + Sync + Send + 'static>(port: u16, h: H) -> Self {
        let handler_arc = ::std::sync::Arc::new(h);
        let service_definition = ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Polzat/ScheduleTask".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.ScheduleTask(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Polzat/Stats".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Stats(p))
                    },
                ),
            ],
        );
        PolzatAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(port, service_definition),
        }
    }
}
