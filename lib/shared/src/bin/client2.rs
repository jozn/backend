#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

extern crate shared;

use serde::{Deserialize, Serialize};
use shared::{pb, rpc2};

#[tokio::main]
async fn main() {
    client_test().await;
}

async fn client_test() {
    let crpc = rpc2::RpcClient::new("http://127.0.0.1:4020/rpc");

    let param = pb::ChannelPinMessageParam { channel_id: 453 };
    let mut buff = vec![];
    ::prost::Message::encode_length_delimited(&param, &mut buff).unwrap();

    let out = crpc
        .Echo(pb::EchoParam {
            text: "sdfsd".to_string(),
        })
        .await;

    let res = crpc.ChangePhoneNumber(pb::ChangePhoneNumberParam {}).await;
    println!("{:#?}", res);

    let res = crpc
        .SendConfirmCode(pb::SendConfirmCodeParam::default())
        .await;
    println!("{:#?}", res);
}
