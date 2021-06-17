use chunk::proto_gen::storage;
use chunk::proto_gen::storage::client_to_chunk_client::ClientToChunkClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = ClientToChunkClient::connect("http://0.0.0.0:5051").await?;

    let request = tonic::Request::new(storage::PingRequest {
        id: 234
    });

    let response = client.ping(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
