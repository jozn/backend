#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

extern crate shared2;

use quick_protobuf::{deserialize_from_slice, BytesReader, MessageRead, MessageWrite, Writer};
use serde::{Deserialize, Serialize};
use shared2::{com::*, pb, rpc::method_ids};

#[tokio::main]
async fn main() {
    client_test().await;
}

async fn client_test() {
    let crpc = RpcClient { endpoint: "" };
    let res = crpc.ChangePhoneNumber(pb::ChangePhoneNumberParam {}).await;
    println!("{:#?}", res);
}

async fn me1() -> Result<(), reqwest::Error> {
    let data = pb::ChangePhoneNumberParam {};

    let mut out_bytes = Vec::new();
    println!("$$$$ aize {:} {}", out_bytes.len(), data.get_size());

    let mut writer = Writer::new(&mut out_bytes);
    let _result = writer.write_message(&data);

    println!("$$$$ method to send: {:?} {:}", out_bytes, out_bytes.len());

    let act = pb::Invoke {
        namespace: 0,
        method: 706069694,
        action_id: 2,
        is_response: true,
        rpc_data: out_bytes,
        // rpc_data: vec![],
    };

    let mut out_invoke = Vec::new();
    let mut writer = Writer::new(&mut out_invoke);
    let _result = writer.write_message(&act);

    println!("$$$$ bytes to send: {:?}", out_invoke);

    let new_post = reqwest::Client::new()
        .post("http://127.0.0.1:3000/rpc")
        // .json(&act)
        .body(out_invoke)
        .send()
        .await?;

    println!("{:#?}", new_post);
    let vs = new_post.bytes().await?;
    let vv = vs.to_vec();
    println!("body {:#?}", vs);
    println!("body bytes {:?}", vv);

    let res = deserialize_from_slice::<pb::ChangePhoneNumberResponse>(&vv);
    println!("response {:?}", res);

    Ok(())
}

async fn send_rpc() {}

struct RpcClient {
    endpoint: &'static str,
}

impl RpcClient {
    fn get_action_id(&self) -> u64 {
        8
    }
}

impl RpcClient {
    pub async fn ChangePhoneNumber(
        &self,
        param: pb::ChangePhoneNumberParam,
    ) -> Result<pb::ChangePhoneNumberResponse, GenErr> {
        let mut buff = Vec::new();
        Writer::new(&mut buff).write_message(&param).unwrap();

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChangePhoneNumber,
            action_id: self.get_action_id(),
            is_response: false,
            rpc_data: buff,
        };

        let mut buff = Vec::new();
        Writer::new(&mut buff).write_message(&invoke).unwrap();

        let req = reqwest::Client::new()
            .post("http://127.0.0.1:3000/rpc")
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = deserialize_from_slice::<pb::ChangePhoneNumberResponse>(&res_bytes)?;
        Ok(pb_res)
    }
}
