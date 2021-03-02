extern crate logic;

use std::collections::{HashMap, HashSet};
use std::iter::Map;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, atomic, Mutex};
use std::sync::atomic::Ordering;

use async_trait::async_trait;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use once_cell::sync::OnceCell;
use tokio::sync::mpsc::{channel, Receiver};

use gen::rpc2;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};
use shared::pb::{
    ConfirmCodeParam, ConfirmCodeResponse, EchoParam, EchoResponse, SendConfirmCodeParam,
    SendConfirmCodeResponse,
};
use shared::pb;

// use logic::UserSpaceOld;

struct LogicMicro {}

#[async_trait]
impl FIMicroService for LogicMicro {
    fn port(&self) -> u16 {
        4001
    }

    async fn serve_request(req: FHttpRequest) -> Result<FHttpResponse, GenErr> {
        println!("thread {:#?}", std::thread::current().id());
        tokio::spawn(async {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            println!(">>>>>thread {:#?}", std::thread::current());
        });
        Ok((200, b"sdfsd".to_vec()))
    }
}

#[tokio::main]
async fn main() {
    println!("Hi logic1 !");

    let c = LogicMicro {
        //handler: Arc::new(handler),
    };
    c.listen_http_requests().await;
}
