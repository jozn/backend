#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

extern crate shared2;

use prost;
use shared2::pb;

fn main() {
    let p = pb::Invoke {
        namespace: 1,
        method: 0,
        action_id: 0,
        is_response: false,
        rpc_data: vec![],
    };

    let mut buff = vec![];
    let m = prost::Message::encode(&p, &mut buff);
    // let m2 = p.encode(&mut buff2); // this does not works

    println!("buff: {:?}", buff);

    let dec: Result<pb::pb2::Invoke, ::prost::DecodeError> =
        prost::Message::decode(buff.as_slice());

    println!("dec: {:?}", dec);
}
