extern crate protobuf;
extern crate grpc;
extern crate rs_jwt_micro;

use std::thread;
use rs_jwt_micro::jwt_grpc::*;
use rs_jwt_micro::jwt::*;

struct TokenizrImpl;

impl Tokenizr for TokenizrImpl {
    fn sign(&self, 
            _o: ::grpc::RequestOptions, 
            _p: CreateTokenReq
            ) -> ::grpc::SingleResponse<TokenRes> {
        
        let r = TokenRes::new();

        grpc::SingleResponse::completed(r)
    }

    fn verify(&self, 
              _o: ::grpc::RequestOptions, 
              _p: ValidationReq
              ) -> ::grpc::SingleResponse<TokenRes> {
        let r = TokenRes::new();

        grpc::SingleResponse::completed(r)
    }

}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(50051);
    server.add_service(TokenizrServer::new_service_def(TokenizrImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}