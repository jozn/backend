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
    echo_test().await;
}
async fn echo_test() {
    let crpc = shared::common::RpcClient::new("http://127.0.0.1:4020/rpc");
    let out = crpc
        .Echo(pb::EchoParam {
            text: "sdfsd".to_string(),
        })
        .await;
    println!("{:#?}", out);
}

async fn echo_loop_test() {
    let crpc = shared::common::RpcClient::new("http://127.0.0.1:4020/rpc");

    for i in 0..100 {
        let out = crpc
            .Echo(pb::EchoParam {
                text: format!("Hi there {}", i),
            })
            .await;
        if i % 10 == 0 {
            println!("{:#?}", i);
        }
    }
}

async fn send_code_test() {
    let crpc = shared::common::RpcClient::new("http://127.0.0.1:4020/rpc");

    let res = crpc
        .SendConfirmCode(pb::SendConfirmCodeParam::default())
        .await;
    println!("{:#?}", res);
}
