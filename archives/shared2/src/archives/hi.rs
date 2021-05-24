// extern crate shared2;

use shared;
use shared::new_rpc::{FHttpRequest, FIMicroService};

struct Cmaster {}

impl FIMicroService for Cmaster {
    fn port(&self) -> u16 {
        4070
    }

    fn serve_request(req: FHttpRequest) -> (u16, Vec<u8>) {
        let m = b"sdflk sdflksdf sdfsdfl sdfsdjfs d".to_vec();
        let f = std::fs::read("./img.jpg").unwrap();
        (200, f);
        (200, m)
    }
}

#[tokio::main]
async fn main() {
    println!("Hi there!");
    let m = shared::new_rpc::SampleService { port: 4050 };

    let c = Cmaster {};
    c.listen_http_requests().await;

    let m = m.listen_http_requests().await;
}
