// extern crate shared2;

use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};
use shared::pb::{ConfirmCodeParam, ConfirmCodeResponse, SendConfirmCodeParam, SendConfirmCodeResponse, EchoResponse, EchoParam};
use shared::{pb, rpc2};

#[derive(Clone)]
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

#[async_trait]
impl rpc2::RPC_Shared_Handler2 for rpc_auth {
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
        // println!("req{:?}", req);
        let invoke: Result<pb::Invoke, ::prost::DecodeError> = prost::Message::decode(req.body);

        let rrr = rpc_auth {};
        let rrr2 = rpc_auth {};
        match &invoke {
            Ok(invoker) => {
                // println!("#1");
                let reg = shared::rpc2::RPC_Registry {
                    RPC_Auth: Some(Box::new(rrr.clone())),
                    RPC_Channel: None,
                    RPC_Chat: None,
                    RPC_Direct: None,
                    RPC_Group: None,
                    RPC_Sample: None,
                    RPC_Shared: Some(Box::new(rrr2)),
                    RPC_Upload: None,
                    ..Default::default()
                };

                let act = rpc2::invoke_to_parsed(invoker).unwrap();
                // println!("#2");

                let res = shared::rpc2::server_rpc(act, &reg).await?;
                let res = to_invoke_response(res, invoker)?;
                // println!("#end");
                return Ok((200, res));
            }

            Err(err) => {}
        }

        let m = b"sdflk sdflksdf sdfsdfl sdfsdjfs d".to_vec();
        let f = std::fs::read("./img.jpg").unwrap();
        // (200, f)
        Ok((200, m))
    }
}

fn to_invoke_response(data: Vec<u8>, req_invoke: &pb::Invoke) -> Result<Vec<u8>, GenErr> {
    let invoke = pb::Invoke {
        namespace: req_invoke.namespace,
        method: req_invoke.method,
        action_id: req_invoke.action_id,
        is_response: true,
        rpc_data: data,
    };
    let mut buff = vec![];
    let out = prost::Message::encode(&invoke, &mut buff)?;
    Ok(buff)
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
