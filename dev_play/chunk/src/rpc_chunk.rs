
use crate::spb::*;
use crate::{sutil, bucket_act};

use tonic::{transport::Server, Request, Response, Status, Code};

#[derive(Default)]
pub struct ChunkRpcHanlder {}

#[tonic::async_trait]
impl ClientToChunk for ChunkRpcHanlder {
    async fn create_bucket(&self, request: Request<CreateBucketRequest>) -> Result<Response<CreateBucketResponse>, Status> {
        println!(">>> {:?}",request.remote_addr());

        let cb = request.into_inner();

        bucket_act::create_bucket(cb.bucket_id, &cb.intent).await;

        let res = CreateBucketResponse{
            bucket_id: cb.bucket_id,
        };

        Ok(Response::new(res))
    }

    async fn insert_file(&self, request: Request<InsertFileRequest>) -> Result<Response<InsertFileResponse>, Status> {
        let r = request.into_inner();

        let res = bucket_act::save_file_into_bucket(r.bucket_id, r.file_id, &r.blob_data).await;
        if !res {
            return Err(Status::new(Code::NotFound, "bucket does not exist."))
        }

        let res = InsertFileResponse{
            message: "sdfds".to_string(),
            ok: true
        };

        Ok(Response::new(res))
    }

    async fn remove_file(&self, request: Request<RemoveFileRequest>) -> Result<Response<RemoveFileResponse>, Status> {
        let r = request.into_inner();

        bucket_act::remove_file_from_bucket(r.bucket_id, r.file_id).await;

        let res = RemoveFileResponse{
            message: "sdfds".to_string(),
            ok: true
        };

        Ok(Response::new(res))
    }

    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        println!(">>> {:?}",request.remote_addr());

        let res = PingResponse{
            id: request.into_inner().id,
        };

        Ok(Response::new(res))
    }
}

pub async fn server_chunk(grpc_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), grpc_port);

    let chunk_grpc_handler = ChunkRpcHanlder::default();

    println!("gRPC listening on http://{}", socket_addr);

    Server::builder()
        .add_service(ClientToChunkServer::new(chunk_grpc_handler))
        .serve(socket_addr)
        .await?;

    Ok(())
}
