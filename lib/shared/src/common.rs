use crate::errors::GenErr;
use crate::new_rpc::{FHttpRequest, FHttpResponse};
use crate::rpc2::RPC_Registry;
use crate::{pb, rpc2};
use std::panic::RefUnwindSafe;

pub async fn rpc_handle_registry(
    reg: &RPC_Registry,
    req: FHttpRequest,
) -> Result<FHttpResponse, GenErr> {
    // println!("req{:?}", req);
    let invoke: pb::Invoke = prost::Message::decode(req.body)?;

    let act = rpc2::invoke_to_parsed(&invoke)?;

    let res = rpc2::server_rpc(act, &reg).await?;
    let res = to_invoke_response(res, &invoke)?;

    Ok((200, res))
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

pub fn prost_encode(pb: &impl prost::Message) -> Result<Vec<u8>, prost::EncodeError> {
    let mut buff = vec![];
    ::prost::Message::encode(pb, &mut buff)?;
    Ok(buff)
}

pub fn prost_decode<T: ::prost::Message + Default>(pb_vec: &[u8]) -> Result<T, prost::DecodeError> {
    ::prost::Message::decode(pb_vec)
}

pub fn param_to_invoke(param: &impl prost::Message, method_id: u32) -> Result<Vec<u8>, GenErr> {
    let param_buff = prost_encode(param)?;
    let invoke = pb::Invoke {
        namespace: 0,
        method: method_id,
        action_id: 0,
        is_response: false,
        rpc_data: param_buff,
    };

    let param_buff = prost_encode(&invoke)?;
    Ok(param_buff)
}

#[derive(Debug)]
pub struct RpcClient {
    pub endpoint: &'static str,
    pub reqwest_client: reqwest::Client,
}

impl RpcClient {
    pub fn new(endpoint: &'static str) -> Self {
        RpcClient {
            endpoint: endpoint,
            reqwest_client: reqwest::Client::new(),
        }
    }

    fn get_next_action_id(&self) -> u64 {
        8
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

    pub async fn rpc_invoke<T: ::prost::Message + Default>(
        &self,
        param: &impl prost::Message,
        method_id: u32,
    ) -> Result<T, GenErr> {
        let invoke_vec = param_to_invoke(param, method_id)?;
        let res_body_vec = self.send_http_request(invoke_vec).await?;
        let pb_res_invoke: pb::Invoke = prost_decode(res_body_vec.as_slice())?;
        let pb_response: T = prost_decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_response)
    }
}

impl RpcClient {
    fn get_next_action_id3(&self) -> u64 {
        8
    }
}
