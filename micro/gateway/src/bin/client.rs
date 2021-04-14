extern crate shared;

use shared::{pb, rpc2};

#[tokio::main]
async fn main() {
    // client_sample_loop().await;
    client_sample_loop().await;
}
async fn client_sample() {
    let crpc = shared::common::RpcClient::new("http://127.0.0.1:4000/rpc");
    let res = crpc
        .SharedEcho(pb::SharedEchoParam {
            text: "hi there".to_string(),
        })
        .await;
    // let res = c
    println!("{:?}", res);
}

async fn client_sample_loop() {
    let crpc = shared::common::RpcClient::new("http://127.0.0.1:4000/rpc");
    loop {
        let res = crpc
            .SharedEcho(pb::SharedEchoParam {
                text: "hi there".to_string(),
            })
            .await;
        // let res = c
        println!("{:?}", res);
    }
}
