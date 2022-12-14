use crate::errors::GenErr;
use crate::new_rpc::{FHttpRequest, FHttpResponse};
use crate::rpc2::RPC_Registry;
use crate::{pb, rpc2};
use rand::RngCore;
use std::panic::RefUnwindSafe;

//////////// Ranodm ////////////
pub fn get_random_u64() -> u64 {
    let mut rng = rand::thread_rng();
    rng.next_u64()
}

//////////// Rpc ///////////////
pub async fn rpc_handle_registry(
    reg: &RPC_Registry,
    req: FHttpRequest,
) -> Result<FHttpResponse, GenErr> {
    // println!("req{:?}", req);
    let invoke: pb::Invoke = prost::Message::decode(req.body)?;

    // println!("hi3 reg > method_id: {} ", invoke.method);

    let act = rpc2::invoke_to_parsed(&invoke)?;

    let res = rpc2::server_rpc(act, &reg).await?;
    let res = to_invoke_response(res, &invoke)?;

    Ok((200, res))
}

pub fn to_invoke_response(data: Vec<u8>, req_invoke: &pb::Invoke) -> Result<Vec<u8>, GenErr> {
    let invoke = pb::InvokeResponse {
        namespace: req_invoke.namespace,
        method: req_invoke.method,
        user_id: 0,
        invoke_id: req_invoke.invoke_id,
        rpc_data: data,
    };
    let mut buff = vec![];
    let out = prost::Message::encode(&invoke, &mut buff)?;
    Ok(buff)
}

// Do not keep this vec in memory for long as it has 512bytes buffer for fast encoding
pub fn prost_encode(pb: &impl prost::Message) -> Result<Vec<u8>, prost::EncodeError> {
    // let mut buff = vec![];
    // let mut buff = Vec::with_capacity(128);
    let mut buff = Vec::with_capacity(256); // good buffer makes a things faster
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
        user_id: 0,
        invoke_id: 0,
        session_hash: "".to_string(),
        api_version: 0,
        app_name: "".to_string(),
        app_version: "".to_string(),
        device_name: "".to_string(),
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

    async fn _send_http_request(&self, body_data: Vec<u8>) -> Result<Vec<u8>, GenErr> {
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

        let res_body_vec = self._send_http_request(invoke_vec).await?;

        //todo zero len respose should be considerd error
        let pb_res_invoke: pb::InvokeResponse = prost_decode(res_body_vec.as_slice())?;
        let pb_response: T = prost_decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_response)
    }
}
