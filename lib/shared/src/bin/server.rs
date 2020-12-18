#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

extern crate shared;

use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use quick_protobuf::{BytesReader, BytesWriter};
use quick_protobuf::{MessageRead, MessageWrite, Writer};

use shared::pb;
use shared::rpc;

fn to_bin(s: String) -> Vec<u8> {
    s.as_bytes().to_owned()
}

async fn server_http(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let res = match req.uri().path() {
        "/echo" => (200, to_bin(echo().await)),
        "/repeat" => (200, to_bin(repeat(req.uri()).await)),
        "/rpc" => (200, server_http_rpc(req).await),
        _ => (404, to_bin("Not found.".to_string())),
    };

    Ok(Response::builder()
        .status(res.0)
        .body(Body::from(res.1))
        .unwrap())
}

async fn server_http_rpc(req: Request<Body>) -> Vec<u8> {
    let bo = req.into_body();
    let bts = body::to_bytes(bo).await.unwrap();

    let mut buff: Vec<u8> = bts.to_vec();

    handle_invoke(buff).await
}

async fn handle_invoke(invoke_buff: Vec<u8>) -> Vec<u8> {
    let buff = invoke_buff;
    let invoke: Result<pb::Invoke, ::prost::DecodeError> = prost::Message::decode(buff.as_slice());

    if let Ok(pb_invoke) = invoke {
        println!("act {:?}", pb_invoke);
        let pb_bts = rpc::server_rpc(pb_invoke)
            .await
            .unwrap_or("".as_bytes().to_owned());
        return pb_bts;
    };

    println!(
        "Error2 in reading pb::Invoke - Err: {:?} bytes {:?}",
        invoke, buff
    );

    "".as_bytes().to_owned()
}

/*async fn sample_invoke_test() {
    let mut buff =vec![];
    let parm = pb::AddCommentParam{ text: "sdfdsf".to_string() };
    let m = prost::Message::encode(&parm, &mut buff).unwrap();

    let inoke = pb::Invoke{
        namespace: 1,
        method: 1222124115,
        action_id: 0,
        is_response: false,
        rpc_data: buff
    };

    let mut buff_invoke =vec![];
    let m = prost::Message::encode(&inoke, &mut buff_invoke);

    println!("buff: {:?}", buff_invoke);

    handle_invoke(buff_invoke).await;

    // let dec : Result<pb::pb2::Invoke, ::prost::DecodeError> = prost::Message::decode(buff_invoke.as_slice());
    //
    // println!("dec: {:?}", dec);
}*/

/////////// Routing Funcs /////////////
async fn echo() -> String {
    "echo me".to_string()
}
async fn repeat(u: &http::Uri) -> String {
    u.query().unwrap_or("[empty]").repeat(10)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        // println!("server xxxxxxx {:?}", _conn.clone());
        // Ok::<_, Infallible>(service_fn(hello_world))
        Ok::<_, Infallible>(service_fn(server_http))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server is running");

    // sample_invoke_test().await;

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
