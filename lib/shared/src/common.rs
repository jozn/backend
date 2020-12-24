use crate::rpc2::RPC_Registry;
use crate::new_rpc::{FHttpRequest, FHttpResponse};
use crate::{pb, rpc2,};
use crate::errors::GenErr;

pub async fn rpc_handle_registry(reg: &RPC_Registry, req: FHttpRequest) -> Result<FHttpResponse, GenErr> {
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