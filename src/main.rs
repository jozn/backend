
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::body;
use serde::{Deserialize, Serialize};
use quick_protobuf::{BytesReader, BytesWriter};
use quick_protobuf::{MessageRead,MessageWrite,Writer};

mod pb;
// use pb::sys::Invoke;
// mod pb;
// mod ps;
// use pb::store::*;
// mod pbs;
use prost::Message;
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Inv {
    pub method: u32,
    pub action_id: u64,
    pub is_response: bool,
    pub rpc_data: Vec<u8>,
}

fn play(){
    let i = Inv{
        method: 0,
        action_id: 0,
        is_response: false,
        rpc_data: vec![]
    };

    let b= Inv{
        method: 0,
        action_id: 0,
        is_response: false,
        rpc_data: vec![1]
    };

    let r = pb::Message{
        gid: 0,
        by_user_cid: 0,
        by_channel_cid: 0,
        post_type: 0,
        media_id: 0,
        file_ref_id: 0,
        post_key: "".to_string(),
        text: "".to_string(),
        rich_text: "".to_string(),
        shared_to: 0,
        via: 0,
        seq: 0,
        edited_time: 0,
        created_time: 0,
        delivery_status: 0,
        delivery_time: 0,
        previous_message_id: 0,
        deleted: false,
        forward_from: None,
        reply_to: None,
        counts: None,
        setting: None,
        files: vec![]
    };

    let mut bts = vec![];
    let m = r.encode(&mut bts );

    // if b.rpc_data == Vec::new() {
    //     println!("sdfsd")
    // }
    // let _m = pb::sys::Invoke{
    //
    // };
}

fn to_bin(s: String) -> Vec<u8> {
    s.as_bytes().to_owned()
}

async fn server_http(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let res= match req.uri().path() {
        "/echo" => (200, to_bin(echo().await) ),
        "/repeat" => (200,to_bin(repeat(req.uri()).await) ),
        "/rpc" => {
            (200, server_http_rpc(req).await)
        },
        _ => (404, to_bin("Not found".to_string()))
    };

    Ok(Response::builder().status(res.0).body(Body::from(res.1)).unwrap())
}

async fn server_http_rpc(req: Request<Body>) -> Vec<u8> {
    // let bo = req.into_body();
    // let y = bo.poll_data().await.unwrap();
    // let bts = body::to_bytes(bo).await.unwrap();
    // let b = bts.as_bytes();
    // req.st

    let bo = req.into_body();
    let bts = body::to_bytes(bo).await.unwrap();
    let b = &bts;

    let mut bytes: Vec<u8>;
    let mut reader = BytesReader::from_bytes(b);
    let invoke = pb::sys::Invoke::from_reader(&mut reader, b);

    if let Ok(act) = invoke {
        println!("act {:?}", act);
        let pb_bts = server_rpc_old(act).unwrap_or("vec![]".as_bytes().to_owned());
        return  pb_bts
    }

    "error in rpc ".as_bytes().to_owned()
}

fn server_rpc_old(act : Invoke) -> Result<Vec<u8>,GenErr> {
    let up = UserParam{};
    match act.method {
        45 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::rpc_general::EchoParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::check_username(&up,param)?;

                let mut out_bytes = Vec::new();
                let mut writer = Writer::new(&mut out_bytes);
                let out = writer.write_message(&result);
                return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        _ => {
            Err(GenErr{})
        }
    }
}

mod rpc {
    use super::*;
    pub fn check_username(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse{
            // text: param.text.clone(),
            // done: true,
        })
    }
}

pub struct GenErr {}
pub struct UserParam {}

async fn echo() -> String {
    "echo me".to_string()
}
async fn repeat(u: &http::Uri) -> String {
    u.query().unwrap_or("[empty]").repeat(10)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        // println!("server xxxxxxx {:?}", _conn.clone());
        // Ok::<_, Infallible>(service_fn(hello_world))
        Ok::<_, Infallible>(service_fn(server_http))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

mod archive {
use super::*;
    async fn server_http_rpc(req: Request<Body>) -> Vec<u8> {
        let bo = req.into_body();
        let bts = body::to_bytes(bo).await.unwrap();

        // let act = serde_json::from_slice::<Act>(&bts);
        let act = bincode::deserialize::<Act>(&bts);

        if let Ok(act) = act {
            println!("act {:?}", act);
            let pb_bts = server_rpc_old(act).unwrap_or("vec![]".as_bytes().to_owned());
            return  pb_bts
            //return bincode::serialize(&pb_bts).unwrap()
        }

        "error in rpc ".as_bytes().to_owned()
    }

    fn server_rpc_old(act :Act) -> Result<Vec<u8>,GenErr> {
        let up = UserParam{};
        match act.method {
            45 => {
                let vec = "funk ".as_bytes().to_owned();

                // let req_param_pb = serde_json::from_slice::<CheckUsernameParam>(&act.data);
                let req_param_pb = bincode::deserialize::<CheckUsernameParam>(&act.data);
                if let Ok(act) = &req_param_pb {
                    println!("actingggggggggggggg {:?}", act);
                    let result = rpc::check_username(&up,act)?;
                    let bts = bincode::serialize(&result).unwrap();
                    return Ok(bts)

                    // Ok(Response::builder().status(404).body(Body::from("RPC Not found.")).unwrap())
                } else {
                    // Ok(Response::builder().status(404).body(Body::from("RPC Not found.")).unwrap())
                }

                Ok(vec)
            },
            _ => {
                Err(GenErr{})
            }
        }
    }

    mod rpc {
        use super::*;
        pub fn check_username(user_param: &UserParam, req: &CheckUsernameParam) -> Result<CheckUsernameRespose,GenErr> {
            Ok(CheckUsernameRespose{
                yes: "sdfsd".to_string()
            })
        }
    }

    trait AllRpc: RPC + RPC5 {

    }

    trait RPC {
        fn check_username(user_param: &UserParam, req: &CheckUsernameParam) -> Result<CheckUsernameRespose,GenErr>;
    }

    trait RPC5 {
        fn check_username5(user_param: &UserParam, req: &CheckUsernameParam) -> Result<CheckUsernameRespose,GenErr>;
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Act {
        method: u32,
        data: Vec<u8>,
        act_id: u64,
    }

    pub struct GenErr {}
    pub struct UserParam {}

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CheckUsernameParam {
        id: u64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CheckUsernameRespose {
        yes: String
    }

    async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let s = format!("{:#?}", req);
        let h =  req.headers();
        let host = h.get(http::header::HOST);
        // let host = h.get(http::header::HOST);
        let uri = req.uri();
        // println!("uri >>> {:#?}", uri);
        println!("uri >>> {:#?}", uri.path());
        println!("uri >>> {:#?}", uri.query());
        println!("method >>> {:#?}", req.method());
        let p = uri.path();
        match p {
            "/echo" => Ok(Response::new(Body::from(uri.query().unwrap_or("[none]").to_string().clone()))),
            "/repeat" => Ok(Response::new(Body::from(s.repeat(100)))),
            "/rpc" => {
                // let bo = req.body().clone();
                let bo = req.into_body();
                let bts = body::to_bytes(bo).await.unwrap();

                let act = serde_json::from_slice::<Act>(&bts);

                if let Ok(act) = act {
                    println!("act {:?}", act);

                    Ok(Response::new(Body::from(bts)))

                } else {
                    Ok(Response::builder().status(404).body(Body::from("RPC Not found.")).unwrap())
                }

            },
            _ => Ok(Response::builder().status(404).body(Body::from("Not found.")).unwrap())
        }
        // Ok(Response::new(Body::from(s.repeat(100))))
    }

}