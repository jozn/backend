
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

mod pb;
// mod pbs;

fn play(){
    let _m = pb::store::User{
        cid: 0,
        phone: Default::default(),
        email: Default::default(),
        password_hash: Default::default(),
        password_salt: Default::default(),
        created_time: 0,
        version_time: 0,
        is_deleted: 0,
        is_banned: 0,
        primary_channel_changed_time: 0,
        UserCounts: None,
        primary_channel: None,
        channels: vec![],
        sessions: vec![]
    };
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

async fn echo() -> String {
    "echo me".to_string()
}
async fn repeat(u: &http::Uri) -> String {
    u.query().unwrap_or("[empty]").repeat(10)
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
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

/*
fn play1() {
    use pb::mod_RoomMessage as mrm;
    use pb::mod_RoomMessage::mod_Author as mrma;

    std::mem::size_of();

    mrm::ChannelExtra{
        signature: "".to_string(),
        views_label: "".to_string(),
        thumbs_up_label: "".to_string(),
        thumbs_down_label: "".to_string()
    };

    let param = pb::RoomMessage{
        message_id: 0,
        message_version: 0,
        status: Default::default(),
        status_version: 0,
        message_type: Default::default(),
        message: "".to_string(),
        attachment: None,
        author: None,
        location: None,
        log: None,
        contact: None,
        wallet: None,
        edited: false,
        create_time: 0,
        update_time: 0,
        deleted: false,
        forward_from: None,
        reply_to: None,
        previous_message_id: 0,
        random_id: 0,
        additional_type: 0,
        additional_data: "".to_string(),
        extra_type: Default::default(),
        channel_extra: None
    };

    let m  = pbs::mod_GetLikesPage::Param{
        PostId: 0,
        Limit: 0,
        LastId: 0
    };

    use pbs::mod_PB_RoomsChanges as rc;
    rc::Chat{
        ChatId: 0,
        RoomKey: "".to_string(),
        RoomType: 0,
        PeerPush: 0,
        ReceivedMessages: vec![],
        SeenMessages: vec![],
        EditeMessages: vec![],
        DeleteMessages: vec![],
        ClearHistroyFromMessageId: 0,
        DeleteChat: 0,
        ChatTitle: "".to_string(),
        Muted: Default::default(),
        MutedUntil: 0,
        Pined: Default::default(),
        PinTime: 0
    };

}
*/

// println!("uri >>> {:#?}", uri.port());
// println!("uri >>> {:#?}", uri.host());
// println!("uri >>> {:#?}", host);
// println!("> {:#?}",  h.get(http::header::ACCEPT_ENCODING));
// println!("> {:#?}",  h.get(http::header::ACCEPT_CHARSET));
// println!("> {:#?}",  h.get(http::header::FORWARDED));
// Ok(Response::new("Hello, World".into()))
// Body::empty();
// match uri.
