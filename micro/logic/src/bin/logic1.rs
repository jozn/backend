extern crate logic;

use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};
use shared::pb::{
    ConfirmCodeParam, ConfirmCodeResponse, EchoParam, EchoResponse, SendConfirmCodeParam,
    SendConfirmCodeResponse,
};
use shared::{pb, rpc2};
use std::sync::{atomic, Arc, Mutex};
use once_cell::sync::OnceCell;
use std::sync::atomic::Ordering;
use rocksdb;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::collections::{HashMap, HashSet};
use tokio::sync::mpsc::{channel, Receiver};
use std::iter::Map;
use std::ops::{Deref, DerefMut};


use logic::UserSpace;



struct LogicMicro {

}
#[async_trait]
impl FIMicroService for LogicMicro {
    fn port(&self) -> u16 {
        4001
    }

    async fn serve_request(req: FHttpRequest) -> Result<FHttpResponse, GenErr> {
        println!("thread {:#?}", std::thread::current().id());
        tokio::spawn(async
            {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            println!(">>>>>thread {:#?}", std::thread::current());
        });
        Ok((200,b"sdfsd".to_vec()))
    }
}

#[tokio::main]
async fn main() {
    println!("Hi cmaster1 !");


    let c = LogicMicro {
        //handler: Arc::new(handler),
    };
    c.listen_http_requests().await;
}
