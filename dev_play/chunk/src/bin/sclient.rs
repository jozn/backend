use chunk::proto_gen::storage;
use chunk::proto_gen::storage::client_to_chunk_client::ClientToChunkClient;
use shared::common::get_random_u64;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    upload_sample().await;

    Ok(())
}

async fn upload_sample() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = ClientToChunkClient::connect("http://0.0.0.0:5051").await?;

    let fo = std::fs::read("./img.jpg").unwrap();
    let request = tonic::Request::new(storage::UploadFileRequest {
        file_id: 123,
        ref_id: 1235,
        bucket_id: 2535,
        secret: 12,
        file_type: 22,
        blob_data: fo
    });

    let response = client.upload_file(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

async fn create_bucket() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = ClientToChunkClient::connect("http://0.0.0.0:5051").await?;

    let request = tonic::Request::new(storage::CreateBucketRequest {
        bucket_id: 2535,
        intent: "video".to_string()
    });

    let response = client.create_bucket(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

async fn ping() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = ClientToChunkClient::connect("http://0.0.0.0:5051").await?;

    let request = tonic::Request::new(storage::PingRequest {
        id: 234
    });

    let response = client.ping(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}