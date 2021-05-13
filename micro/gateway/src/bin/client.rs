extern crate shared;

use shared::{pb, rpc2};

#[tokio::main]
async fn main() {
    // client_sample_loop().await;
    auth_sample().await;
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

////// Auth codes
async fn auth_sample() {
    let crpc = shared::common::RpcClient::new("http://127.0.0.1:4000/rpc");
    let res = crpc
        .AuthSendConfirmCode(pb::AuthSendConfirmCodeParam {
            hash: "1234hash".to_string(),
            phone: "9015132328".to_string(),
            country_code: "".to_string(),
            resend: false,
        })
        .await;
    // let res = c
    println!("Sending auth code: {:?}", res);
}
