#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

extern crate backbone;

use backbone::{pb, com::*, rpc::method_ids, rpc};
use quick_protobuf::{deserialize_from_slice, BytesReader, MessageRead, MessageWrite, Writer};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main()  {
    client_test().await;
}

async fn client_test()  {
    let crpc = rpc::RpcClient::new("http://127.0.0.1:3001/rpc");
    let res = crpc.ChangePhoneNumber(pb::ChangePhoneNumberParam{}).await;
    println!("{:#?}",res);

    let res = crpc.AddComment(pb::AddCommentParam::default()).await;
    println!("{:#?}",res);

    let res = crpc.SendConfirmCode(pb::SendConfirmCodeParam::default()).await;
    println!("{:#?}",res);

}

