extern crate protobuf;
extern crate grpc;
extern crate rs_jwt_micro;
extern crate frank_jwt;
extern crate chrono;

use std::thread;
use rs_jwt_micro::jwt_grpc::*;
use rs_jwt_micro::jwt::*;
use frank_jwt::{Header, Payload, Algorithm, encode};

struct TokenizrImpl;

impl Tokenizr for TokenizrImpl {
    fn sign(&self, 
            _o: ::grpc::RequestOptions, 
            req: CreateTokenReq
            ) -> ::grpc::SingleResponse<TokenRes> {
        
        // Prepare the response object
        let mut r = TokenRes::new();

        // Now timestamp in UTC TZ
        let now = chrono::Utc::now().timestamp();

        // Prepare the payload
        let mut payload = Payload::new();

        // The issuer is the email / url of the token issuer
        payload.insert("iss".to_string(), "rs-jwt-micro".to_string());

        // The subject is the email of the identity 
        payload.insert("sub".to_string(), req.subject.to_string());

        // The expiration time of the assertion, specified as seconds since
        // 00:00:00 UTC, January 1, 1970.
        // This value has a maximum of 1 hour after the issued time.
        payload.insert("exp".to_string(), format!("{}", now + 60 * 60));

        // The time the assertion was issued, specified as seconds since
        // 00:00:00 UTC, January 1, 1970.
        payload.insert("iat".to_string(), format!("{}", now));

        // The time the assertion could be used after, specified as seconds since
        // 00:00:00 UTC, January 1, 1970.
        payload.insert("nbf".to_string(), format!("{}", now - 1));

        // Initialize a JWT header with SHA512 signature
        let header = Header::new(Algorithm::HS512);
        
        // Encode the JWT token
        r.token = encode(header, "secret".to_string(), payload.clone());

        // Send gRPC response
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