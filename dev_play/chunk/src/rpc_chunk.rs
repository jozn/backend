use crate::proto_gen;
use crate::spb;

use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    //tonic::include_proto!("helloworld");
    // pub use proto_gen::helloworld::greeter_server;
    pub use crate::proto_gen::helloworld::*;
}
use proto_gen::storage;
use proto_gen::storage::*;
use proto_gen::storage::{
    client_to_chunk_client::ClientToChunkClient,
    client_to_chunk_server::{
        ClientToChunk,
        ClientToChunkServer
    }
};
// use proto_gen::storage::{UploadFileRequest, CreateBucketResponse, PingRequest, CreateBucketRequest, PingResponse, UploadFileResponse};
// use proto_gen::storage::client_to_chunk_server::ClientToChunkServer;

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl storage::client_to_chunk_server::ClientToChunk for MyGreeter {
    async fn create_bucket(&self, request: Request<CreateBucketRequest>) -> Result<Response<CreateBucketResponse>, Status> {
        println!(">>> {:?}",request.remote_addr());

        let res = CreateBucketResponse{
            bucket_id: request.into_inner().bucket_id,
        };


        Ok(Response::new(res))
    }

    async fn upload_file(&self, request: Request<UploadFileRequest>) -> Result<Response<UploadFileResponse>, Status> {
        todo!()
    }

    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        println!(">>> {:?}",request.remote_addr());

        let res = PingResponse{
            id: request.into_inner().id,
        };

        Ok(Response::new(res))
    }
}

pub async fn server_chunk() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:5051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(ClientToChunkServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
