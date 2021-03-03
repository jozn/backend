use async_trait::async_trait;

use bytes::Bytes;
use once_cell::sync::OnceCell;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};
use shared::{pb, rpc2};
use std::sync::Arc;

static GATEWAY_INSTANCE: OnceCell<Gateway> = OnceCell::new();

// Note: blocking Api was added due to crates incompatablity, now it's
// fixed, just use async api.

#[derive(Debug)]
struct Gateway {
    pub endpoint: &'static str,
    pub reqwest_client: reqwest::Client,
}

impl Gateway {
    pub fn new(endpoint: &'static str) -> Self {
        Gateway {
            endpoint: endpoint,
            reqwest_client: reqwest::Client::new(),
        }
    }

    fn get_shared(&self) -> u64 {
        println!("get shared ");
        345
    }

    pub async fn send_http_request(&self, body_data: Vec<u8>) -> Result<Vec<u8>, GenErr> {
        let req = self
            .reqwest_client
            .post(self.endpoint)
            .body(body_data)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();
        Ok(res_bytes)
    }

}

#[derive(Debug)]
struct GatewayMicro {
    // gateway: Arc<Gateway>,
}

#[async_trait]
impl FIMicroService for GatewayMicro {
    fn port(&self) -> u16 {
        4000
    }

    async fn serve_request(req: FHttpRequest) -> Result<FHttpResponse, GenErr> {
        if req.method == http::Method::GET && req.path == "/" {
            return Ok((200, b"This is gateway.".to_vec()));
        }
        let body = req.body.clone();
        if body.len() == 0 {
            return Ok((500, b"body is empty.".to_vec()));
        }

        let invoke: pb::Invoke = prost::Message::decode(body)?;
        let gate = GATEWAY_INSTANCE.get().unwrap();

        match invoke.method {
            rpc2::method_ids::GetProfiles => {
                println!("rpc2::method_ids::Echo ");
            }
            _ => {
                println!("method {} ", invoke.method);

                // todo remve blokign cod once reqwest support tokio1
                // let res = gate.send_http_request(req.body.to_vec()).await?;
                let res = gate.send_http_request(req.body.to_vec()).await?;

                return Ok((200, res));
            }
        };
        Ok((200, b" manula".to_vec()))
    }
}

#[tokio::main]
async fn main() {
    println!("Starting gateway1");

    let gateway = Gateway {
        endpoint: "http://127.0.0.1:4001/rpc",
        reqwest_client: Default::default(),
    };

    GATEWAY_INSTANCE.set(gateway).unwrap();

    let gateway_micro = GatewayMicro{
        // gateway: gateway,
    };

    gateway_micro.listen_http_requests().await;
}
