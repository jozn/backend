extern crate backbone;

use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use serde_json;
use backbone::{pb};
use quick_protobuf::{MessageRead, MessageWrite, Writer};

#[tokio::main]
async fn main() {
    let p = pb::Invoke{
        namespace: 0,
        method: 0,
        action_id: 0,
        is_response: false,
        rpc_data: vec![]
    };

    let mut buff = Vec::new();
    Writer::new(&mut buff).write_message(&p);

    println!("ewa: {:?}", buff );
}

