use crate::pb;
use crate::pb::{EchoParam, EchoResponse};
use crate::{errors::GenErr, UserParam};
use async_trait::async_trait;

use crate::rpc2;
use http::Version;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error as HyperError, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

enum Http2 {
    Not = 40,
    No = 2,
}

fn play() {
    let v = Http2::No;
    // Http2::from(2);
}

/*impl RPC_Registry {
    fn new() -> RPC_Registry {
        RPC_Registry { auth: None }
    }
}

impl RPC_Chat2_Trait for RPC_Registry {}*/

struct RpcInvoke {
    method_id: i64, // correct data type should be i32,
    rpc_service: RpcServiceData,
}

enum RpcServiceData {
    RPC_Chat(RPC_Social_MethodData),
    RPC_Social,
}

enum RPC_Social_MethodData {
    SendConfirmCode(pb::SendConfirmCodeParam),
    SingUp(pb::ChannelSetDraftParam),
}

#[async_trait]
trait RPC_Shared {
    async fn Echo(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
}

#[async_trait]
trait RPC_Chat {
    async fn ChatSendMsg(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
}

#[async_trait]
trait RPC_Chat2_Trait {
    async fn ChatSendMsg(
        &self,
        up: &UserParam,
        param: pb::EchoParam,
    ) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
}

struct Rpc {
    RPC_Chat2: Box<dyn RPC_Chat2_Trait>,
    RPC_Chat3: Option<Box<dyn RPC_Chat2_Trait>>,
}

impl Rpc {
    pub fn new(RPC_Chat2: Box<dyn RPC_Chat2_Trait>) -> Self {
        Rpc {
            RPC_Chat2: RPC_Chat2,
            RPC_Chat3: None,
        }
    }
}

struct Ut {
    s: String,
}

impl Ut {
    pub fn serve(data: Vec<u8>) {}
}

#[async_trait]
impl RPC_Chat2_Trait for Ut {
    async fn ChatSendMsg(&self, up: &UserParam, param: EchoParam) -> Result<EchoResponse, GenErr> {
        unimplemented!()
    }
}

/////////////////////Http

pub type FHttpResponse = (u16, Vec<u8>);

#[derive(Debug)]
pub struct FHttpRequest {
    pub method: http::Method,
    pub path: String,
    pub body: bytes::Bytes,
}

trait ServeHttpRequest {
    // fn serve_http_old(req: &http::Request<Vec<u8>>) -> FHttpResponse;
}

pub struct SampleService {
    pub port: u16,
}

#[async_trait]
impl FIMicroService for SampleService {
    fn port(&self) -> u16 {
        4040
    }

    async fn serve_request(req: FHttpRequest) -> (u16, Vec<u8>) {
        (200, b"hi there".to_vec())
    }
}

#[async_trait]
pub trait FIMicroService {
    fn port(&self) -> u16;
    async fn serve_request(req: FHttpRequest) -> FHttpResponse;

    async fn listen_http_requests(&self) {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port()));
        println!("There will some sort.");

        let make_svc = make_service_fn(move |_| async {
            Ok::<_, HyperError>(service_fn(move |_req| async {
                let path = _req.uri().path().to_string();
                let bo = _req.into_body();
                let bts = hyper::body::to_bytes(bo).await.unwrap();

                let req = FHttpRequest {
                    method: Default::default(),
                    path: path,
                    body: bts,
                };

                let respose = Self::serve_request(req).await;

                let res = Response::new(Body::from(respose.1));
                Ok::<_, HyperError>(res)
            }))
        });

        let server = Server::bind(&addr).serve(make_svc);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}

// dead code

/*
pub fn invoke_to_parsed(invoke: &pb::Invoke) -> Result<RpcInvoke, GenErr>{
    use RpcServiceData::*;
    let rpc = match invoke.method {
        method_ids::SendConfirmCode => {
            let rpc_param  : Result<pb::SendConfirmCodeParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice()).unwrap();
            RpcInvoke{
                method_id: 423423 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatDeleteHistory(rpc_param)),
            }
        },
        _ => { panic!("sdf")}
    };
    Ok(rpc)
}

pub fn parse_invoke(invoke: &pb::Invoke) -> Result<RpcInvoke>{
    let rpc = match invoke.method {
        method_ids::SendConfirmCode => {

        },
    }
}

pub async fn server_rpc_old(act: pb::Invoke) -> Result<Vec<u8>, GenErr> {
    let up = UserParam {};

    match act.method {
        // service: RPC_Auth
        method_ids::SendConfirmCode => {
            // 939965206
            let vec: Vec<u8> = vec![];
            let rpc_param: Result<pb::SendConfirmCodeParam, ::prost::DecodeError> =
                prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::SendConfirmCode(&up, param).await?;

                let mut buff = vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    }
}

pub async fn server_rpc(act: RpcInvoke, reg: RPC_Registery) -> Result<Vec<u8>, GenErr> {
    match act.rpc_service {
        RpcServiceData::RPC_Chat(method) => match method {
            RPC_Social_MethodData::SendConfirmCode(rr) => {
                reg.ChatSendMsg();
            }
            RPC_Social_MethodData::SingUp(rr) => RPC_Shared::Echo(),
        },
        RpcServiceData::RPC_Social => {}
    };
    Ok(vec![])
}

*/
