// extern crate shared2;

use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FIMicroService};
use shared::pb::{
    ConfirmCodeParam, ConfirmCodeResponse, SendConfirmCodeParam, SendConfirmCodeResponse,
};
use shared::{pb, rpc2};

struct rpc_auth {}

#[async_trait]
impl rpc2::RPC_Auth_Handler2 for rpc_auth {
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
struct Cmaster {}

#[async_trait]
impl FIMicroService for Cmaster {
    fn port(&self) -> u16 {
        4020
    }

    async fn serve_request(req: FHttpRequest) -> (u16, Vec<u8>) {
        println!("req{:?}", req);
        let invoke: Result<pb::Invoke, ::prost::DecodeError> = prost::Message::decode(req.body);

        let rrr = rpc_auth {};
        match invoke {
            Ok(invoker) => {
                println!("#1");
                let reg = shared::rpc2::RPC_Registry {
                    RPC_Auth: Some(Box::new(rrr)),
                    RPC_Channel: None,
                    RPC_Chat: None,
                    RPC_Direct: None,
                    RPC_Group: None,
                    RPC_Sample: None,
                    RPC_Shared: None,
                    RPC_Upload: None,
                    RPC_User: None,
                };

                let act = rpc2::invoke_to_parsed(&invoker).unwrap();
                println!("#2");

                let res = shared::rpc2::server_rpc(act, &reg).await;
                println!("#end");
                return (200, res.unwrap());
            }

            Err(err) => {}
        }

        let m = b"sdflk sdflksdf sdfsdfl sdfsdjfs d".to_vec();
        let f = std::fs::read("./img.jpg").unwrap();
        // (200, f)
        (200, m)
    }
}

#[tokio::main]
async fn main() {
    let reg = shared::rpc2::RPC_Registry {
        RPC_Auth: None,
        RPC_Channel: None,
        RPC_Chat: None,
        RPC_Direct: None,
        RPC_Group: None,
        RPC_Sample: None,
        RPC_Shared: None,
        RPC_Upload: None,
        RPC_User: None,
    };

    println!("Hi there!");

    let c = Cmaster {};
    c.listen_http_requests().await;
}
