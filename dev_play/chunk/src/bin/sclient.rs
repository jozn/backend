use chunk::spb::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    create_bucket().await;
    upload_sample().await;
    remove_sample().await;

    Ok(())
}

async fn upload_sample() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientToChunkClient::connect("http://0.0.0.0:11000").await?;

    let fo = std::fs::read("./img.jpg").unwrap();
    let request = tonic::Request::new(storage::InsertFileRequest {
        file_id: 253489,
        ref_id: 1235,
        bucket_id: 1753,
        secret: 12,
        file_type: 22,
        blob_data: fo
    });

    let response = client.insert_file(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

async fn remove_sample() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientToChunkClient::connect("http://0.0.0.0:11000").await?;
    let request = tonic::Request::new(storage::RemoveFileRequest {
        file_id: 253489,
        ref_id: 1235,
        bucket_id: 1753,
        secret: 12,
    });

    let response = client.remove_file(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}

async fn create_bucket() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientToChunkClient::connect("http://0.0.0.0:11000").await?;

    let request = tonic::Request::new(storage::CreateBucketRequest {
        bucket_id: 1753,
        intent: "video".to_string()
    });

    let response = client.create_bucket(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

async fn ping() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientToChunkClient::connect("http://0.0.0.0:11000").await?;

    let request = tonic::Request::new(storage::PingRequest {
        id: 234
    });

    let response = client.ping(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}