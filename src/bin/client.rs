#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

extern crate backbone;

use serde::{Deserialize, Serialize};
use backbone::{pb};
use quick_protobuf::{MessageWrite, Writer, BytesReader, MessageRead, deserialize_from_slice};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    me1().await
}

async fn me1() -> Result<(), reqwest::Error> {
    let data =  pb::ChangePhoneNumberParam{
    };

    let mut out_bytes = Vec::new();
    println!("$$$$ aize {:} {}", out_bytes.len() , data.get_size());

    let mut writer = Writer::new(&mut out_bytes);
    let _result = writer.write_message(&data);

    println!("$$$$ method to send: {:?} {:}", out_bytes , out_bytes.len());

    let act = pb::Invoke{
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
