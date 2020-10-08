#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

extern crate backbone;

use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use quick_protobuf::{BytesReader, BytesWriter};
use quick_protobuf::{MessageRead, MessageWrite, Writer};

use backbone::pb;
use backbone::rpc;

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
    let b = &bts;

    let mut bytes: Vec<u8>;
    let mut reader = BytesReader::from_bytes(b);
    let invoke = pb::sys::Invoke::from_reader(&mut reader, b);

    if let Ok(act) = invoke {
        println!("act {:?}", act);
        // let pb_bts = server_rpc(act).unwrap_or("vec![]".as_bytes().to_owned());
        let pb_bts = rpc::server_rpc(act).unwrap_or("vec![]".as_bytes().to_owned());
        return pb_bts;
    };

    "error in rpc ".as_bytes().to_owned()
}

/////////// Routing Funcs /////////////
async fn echo() -> String {
    "echo me".to_string()
}
async fn repeat(u: &http::Uri) -> String {
    u.query().unwrap_or("[empty]").repeat(10)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

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

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
