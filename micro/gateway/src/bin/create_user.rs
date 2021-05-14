extern crate shared;

use shared::{pb, rpc2};

#[tokio::main]
async fn main() {
    // client_sample_loop().await;
    auth_sample().await;
}

////// Auth codes
async fn auth_sample() {
    let crpc = shared::common::RpcClient::new("http://127.0.0.1:4000/rpc");
    let res = crpc
        .UserRegisterUser(pb::UserRegisterUserParam {
            user_cid: 0,
            first_name: "Nima".to_string(),
            last_name: "Sabet".to_string(),
            created_time: 0,
            hash_code: "zzzzzzzzzzzzz".to_string(),
            phone_number: "989015131234".to_string(),
            confirm_code: "2345".to_string(),
        })
        .await;
    // let res = c
    println!("Registering user: {:?}", res);
}
