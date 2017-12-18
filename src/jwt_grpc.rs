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


// interface

pub trait Tokenizr {
    fn sign(&self, o: ::grpc::RequestOptions, p: super::jwt::CreateTokenReq) -> ::grpc::SingleResponse<super::jwt::TokenRes>;

    fn verify(&self, o: ::grpc::RequestOptions, p: super::jwt::ValidationReq) -> ::grpc::SingleResponse<super::jwt::TokenRes>;
}

// client

pub struct TokenizrClient {
    grpc_client: ::grpc::Client,
    method_Sign: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::jwt::CreateTokenReq, super::jwt::TokenRes>>,
    method_Verify: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::jwt::ValidationReq, super::jwt::TokenRes>>,
}

impl TokenizrClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        TokenizrClient {
            grpc_client: grpc_client,
            method_Sign: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/jwt.Tokenizr/Sign".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Verify: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/jwt.Tokenizr/Verify".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            TokenizrClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            TokenizrClient::with_client(c)
        })
    }
}

impl Tokenizr for TokenizrClient {
    fn sign(&self, o: ::grpc::RequestOptions, p: super::jwt::CreateTokenReq) -> ::grpc::SingleResponse<super::jwt::TokenRes> {
        self.grpc_client.call_unary(o, p, self.method_Sign.clone())
    }

    fn verify(&self, o: ::grpc::RequestOptions, p: super::jwt::ValidationReq) -> ::grpc::SingleResponse<super::jwt::TokenRes> {
        self.grpc_client.call_unary(o, p, self.method_Verify.clone())
    }
}

// server

pub struct TokenizrServer;


impl TokenizrServer {
    pub fn new_service_def<H : Tokenizr + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/jwt.Tokenizr",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/jwt.Tokenizr/Sign".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.sign(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/jwt.Tokenizr/Verify".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.verify(o, p))
                    },
                ),
            ],
        )
    }
}
