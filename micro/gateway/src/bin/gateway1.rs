use async_trait::async_trait;

use bytes::Bytes;
use once_cell::sync::OnceCell;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};
use shared::{pb, rpc2};
use std::sync::Arc;

static FORWARD_INSTANCE: OnceCell<ForwardClient> = OnceCell::new();

// Note: blocking Api was added due to crates incompatablity, now it's
// fixed, just use async api.

#[derive(Debug)]
struct ForwardClient {
    endpoint: String,
    reqwest_client: reqwest::Client,
}

impl ForwardClient {
    pub fn new(endpoint: String) -> Self {
        ForwardClient {
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
            .post(&self.endpoint)
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

    // todo add self
    async fn serve_request(req: FHttpRequest) -> Result<FHttpResponse, GenErr> {
        // println!(">>serving gateway request {:?}", req);

        // Some path checking and validation
        if req.method == http::Method::GET && req.path == "/" {
            return Ok((200, b"This is gateway.".to_vec()));
        }
        if req.path != "rpc" {
            return Ok((404, b"Only /rpc route is recgonized".to_vec()));
        }

        let body = req.body.clone();
        // No serving of zero data (hence empty Invoke)
        if body.len() == 0 {
            return Ok((500, b"body is empty.".to_vec()));
        }

        let invoke: pb::Invoke = prost::Message::decode(body)?;
        let gate = FORWARD_INSTANCE.get().unwrap();

        // Handling rpc requests based on their method types
        match invoke.method {
            // Especial handling of some rpc methods: for example if all
            // groups calls must be directed to a particaluar shared
            rpc2::method_ids::GetProfiles => {
                println!("rpc2::method_ids::Echo ");
                Ok((200, b" manula".to_vec()))
            }

            // All other rpc calls handling
            _ => {
                println!("method {} ", invoke.method);
                let res = gate.send_http_request(req.body.to_vec()).await?;
                Ok((200, res))
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Starting gateway1");

    let forward = ForwardClient {
        endpoint: "http://127.0.0.1:4001/rpc".to_string(), // logic micro listener
        reqwest_client: Default::default(),
    };

    FORWARD_INSTANCE.set(forward).unwrap();

    let gateway_micro = GatewayMicro{
        // gateway: gateway,
    };

    gateway_micro.listen_http_requests().await;
}
