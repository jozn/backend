use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};

struct ImgServer {}

#[async_trait]
impl FIMicroService for ImgServer {
    fn port(&self) -> u16 {
        4040
    }

    async fn serve_request(req: FHttpRequest) -> Result<FHttpResponse, GenErr> {
        let m = b"sdflk sdflksdf sdfsdfl sdfsdjfs d".to_vec();
        let f = std::fs::read("./img.jpg").unwrap();
        Ok((200, f))
    }
}

#[tokio::main]
async fn main() {
    println!("Hi there!");

    let c = ImgServer {};
    c.listen_http_requests().await;
}
