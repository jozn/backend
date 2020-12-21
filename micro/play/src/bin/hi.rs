// extern crate shared2;

use shared;
use shared::new_rpc::FIMicroService;

#[tokio::main]
async fn main() {
    println!("Hi there!");
    let m = shared::new_rpc::SampleService{
        port: 4050,
    };

    let m = m.server_http_requests().await;

}
