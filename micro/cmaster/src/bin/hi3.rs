// extern crate shared2;

use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};
use shared::pb::{ConfirmCodeParam, ConfirmCodeResponse, SendConfirmCodeParam, SendConfirmCodeResponse, EchoResponse, EchoParam};
use shared::{pb, rpc2};

#[derive(Clone)]
struct CommonRpcHandler {}

#[async_trait]
impl rpc2::RPC_Auth_Handler2 for CommonRpcHandler {
    async fn SendConfirmCode(
        &self,
        param: SendConfirmCodeParam,
    ) -> Result<SendConfirmCodeResponse, GenErr> {
        println!("yes here");
        Ok(SendConfirmCodeResponse {
            done: true,
            error_message: "dosflsdj sf sd".to_string(),
            just_email_register: false,
            sms_numbers: vec![],
            is_login: false,
        })
    }
}

#[async_trait]
impl rpc2::RPC_Shared_Handler2 for CommonRpcHandler {
    async fn Echo(&self, param: EchoParam) -> Result<EchoResponse, GenErr> {
        let res = EchoResponse{
            done: true,
            text: format!("Res::>> {}", param.text),
        };
        Ok(res)
    }
}
struct Cmaster {}

#[async_trait]
impl FIMicroService for Cmaster {
    fn port(&self) -> u16 {
        4020
    }

    async fn serve_request(req: FHttpRequest) -> Result<FHttpResponse, GenErr> {
        let rrr = CommonRpcHandler {};
        let reg = shared::rpc2::RPC_Registry {
            RPC_Auth: Some(Box::new(rrr.clone())),
            RPC_Channel: None,
            RPC_Chat: None,
            RPC_Direct: None,
            RPC_Group: None,
            RPC_Sample: None,
            RPC_Shared: Some(Box::new(rrr)),
            RPC_Upload: None,
            ..Default::default()
        };

        shared::common::rpc_handle_registry(&reg, req).await
    }
}

#[tokio::main]
async fn main() {
    println!("Hi there!");

    let c = Cmaster {};
    c.listen_http_requests().await;
}
