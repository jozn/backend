use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    //tonic::include_proto!("helloworld");
    // pub use chunk::proto_gen::helloworld::greeter_server;
    pub use chunk::proto_gen::helloworld::*;
}
use chunk::proto_gen::storage;
use chunk::proto_gen::storage::{UploadFileRequest, CreateBucketResponse, PingRequest, CreateBucketRequest, PingResponse, UploadFileResponse};
use chunk::proto_gen::storage::client_to_chunk_server::ClientToChunkServer;
#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[derive(Default)]
pub struct ChunkServer {}

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
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:5051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(ClientToChunkServer::new(MyGreeter::default()))
        .serve(addr)
        .await?;

    Ok(())
}
