#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

extern crate backbone;

use serde::{Deserialize, Serialize};
use backbone::{pb};
use quick_protobuf::{MessageWrite, Writer, BytesReader, MessageRead, deserialize_from_slice};
// use std::intrinsics::;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // pb_test();
    me1().await
}

/*fn main() {
    // pb_test();
    pb_test_compact();
}*/
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




/*/////////// Archives -- delete ///////////////////
fn pb_test()  /**/{
    let data =  pb::ChangePhoneNumberParam{
    };

    let mut out_bytes = Vec::new();
    println!("$$$$ aize {:} {}", out_bytes.len() , data.get_size());

    let mut writer = Writer::new(&mut out_bytes);
    let _result = writer.write_message(&data);

    println!("$$$$ method to send: {:?} {:}", out_bytes , out_bytes.len());

    let act = pb::Invoke{
        method: 706069694,
        action_id: 2,
        is_response: false,
        rpc_data: out_bytes,
        // rpc_data: vec![],
    };

    let mut out_invoke = Vec::new();
    let mut writer = Writer::new(&mut out_invoke);
    let _result = writer.write_message(&act);

    println!("$$$$ bytes to send: {:?}", out_invoke);


    // println!("{:#?}", invoke);


    let mut bytes_reader: Vec<u8> = vec![];
    // println!("$$$$ ves got: {:?}", bytes);

    let mut reader = BytesReader::from_bytes(&bytes_reader);
    let invoke = pb::Invoke::from_reader(&mut reader, &out_invoke);

    if let Ok(act) = &invoke {
        println!("act {:?}", act);
        // let pb_bts = server_rpc(act).unwrap_or("vec![]".as_bytes().to_owned());
        // let pb_bts = rpc::server_rpc(act).unwrap_or("vec![]".as_bytes().to_owned());
        // return pb_bts;
    };

    println!("reader: {:#?}", bytes_reader);
    println!("reader: {:#?}", invoke);


}

fn pb_test_compact()  {
    let pb_param =  pb::ChangePhoneNumberResponse{
        done: true,
        text: "a".to_string()
    };

    let mut out_bytes = Vec::new();
    // println!("$$$$ aize {:} {}", out_bytes.len(), pb_param.get_size());

    let _  = Writer::new(&mut out_bytes).write_message(&pb_param);
    // println!("$$$$ method to send: {:?} {:}", out_bytes , out_bytes.len());

    let invoke = pb::Invoke{
        method: 58,
        action_id: 0,
        is_response: false,
        rpc_data: out_bytes,
        // rpc_data: vec![],
    };

    let mut out_invoke = Vec::new();
    Writer::new(&mut out_invoke).write_message(&invoke);

    println!("$$$$ bytes to send: {:?}", out_invoke);

    // println!("{:#?}", invoke);

    let mut bytes_reader: Vec<u8> = vec![];
    // println!("$$$$ ves got: {:?}", bytes);

    let mut invoke = BytesReader::from_bytes(&bytes_reader).read_message::<pb::Invoke>(out_invoke.as_slice());
    // let invoke = pb::Invoke::from_reader(&mut reader, &out_invoke);

    if let Ok(act) = &invoke {
        println!("act {:?}", act);
        // let pb_bts = server_rpc(act).unwrap_or("vec![]".as_bytes().to_owned());
        // let pb_bts = rpc::server_rpc(act).unwrap_or("vec![]".as_bytes().to_owned());
        // return pb_bts;
    };

    println!("reader: {:#?}", bytes_reader);
    println!("out_invoke: {:#?}", out_invoke);
    println!("reader: {:#?}", invoke);


}
*/
