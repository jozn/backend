use crate::proto_gen;
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

    async fn upload_file(&self, request: Request<UploadFileRequest>) -> Result<Response<UploadFileResponse>, Status> {
        let r = request.into_inner();
        let s = bucket_act::StoragePathBuilder::new(r.bucket_id,0).bucket_dir();
        // let s = bucket_act::bucket_id_to_path_DEP(r.bucket_id);
        if !std::path::Path::new(&s).exists() {
            return Err(Status::new(Code::NotFound, "bucket does not exist."))
        }

        bucket_act::save_file_into_bucket(r.bucket_id, r.file_id, &r.blob_data).await;

        let res = UploadFileResponse{
            message: "sdfds".to_string(),
            ok: true
        };

        Ok(Response::new(res))
    }

    async fn remove_file(&self, request: Request<RemoveFileRequest>) -> Result<Response<RemoveFileResponse>, Status> {
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
