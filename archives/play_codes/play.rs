use crate::pb;
use crate::pb::{EchoParam, EchoResponse};
use crate::{errors::GenErr, UserParam};
use async_trait::async_trait;

use crate::rpc2::*;
use http::Version;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error as HyperError, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

pub async fn server_rpc(act: RpcInvoke, reg: &Reg) -> Result<Vec<u8>, GenErr> {
    let res_v8 = match act.rpc_service {
        RpcServiceData::RPC_Auth(method) => match method {
            RPC_Auth_MethodData::SendConfirmCode(param) => {
                // let reg = reg.clone();
                let handler = reg.auth?;
                // handler.SingUp();
                let response = handler.SingUp(param).await?;
                // let response = reg.SendConfirmCode(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            _ => panic!("sdf"),
        },

        _ => panic!("sdf"),
    };

    Ok(res_v8)
}

struct Reg {
    auth: Option<Box<RPC_Auth_Handler2>>,
    // RPC_Shared: RPC_Shared,
    // RPC_Chat: RPC_Chat,
}

fn to_vev8(msg: &impl prost::Message) -> Result<Vec<u8>, GenErr> {
    let mut buff = vec![];
    prost::Message::encode(msg, &mut buff)?;
    Ok(buff)
}
