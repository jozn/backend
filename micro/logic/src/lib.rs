#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_features)]
#![allow(warnings)]
#![allow(soft_unstable)]
use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};
use shared::{pb, rpc2};
use std::sync::{atomic, Arc, Mutex};
use once_cell::sync::OnceCell;
use std::sync::atomic::Ordering;
use rocksdb;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::collections::{HashMap, HashSet};
use tokio::sync::mpsc::{channel, Receiver,Sender};
use tokio::sync::oneshot;
use std::iter::Map;
use std::ops::{Deref, DerefMut};
use std::thread;
use shared::rpc2::RPC_Registry;
use std::borrow::Borrow;
use bytes::Bytes;
use std::panic::RefUnwindSafe;

// no locking
#[derive(Debug)]
pub struct UserSpace {
    contacts: HashSet<String>,
    liked_posts: HashSet<String>,
    sender: Sender<UserSpaceReq>,
    last_rpc: u64,

    user_id: u64,
    user_info: Commands, // pb

    contact3: HashMap<String, bool>,
    contacts2: Vec<u64>,
}

#[derive(Debug)]
pub enum Commands {
    RpcDead,
    InternalRpc,
    Exit,
    Invoke(shared::rpc2::RpcInvoke),
}

#[derive(Debug)]
pub struct UserSpaceReq {
    cmd: Commands,
    sender: oneshot::Sender<Vec<u8>>
}

/*pub async fn rpc_handle_registry(
    reg: &RPC_Registry,
    req: FHttpRequest,
) -> Result<FHttpResponse, GenErr> {
    // println!("req{:?}", req);
    let invoke: pb::Invoke = prost::Message::decode(req.body)?;

    // println!("hi3 reg > method_id: {} ", invoke.method);

    let act = rpc2::invoke_to_parsed(&invoke)?;

    let res = rpc2::server_rpc(act, &reg).await?;
    let res = to_invoke_response(res, &invoke)?;

    Ok((200, res))
}*/

#[derive(Default,Debug)]
pub struct UserSpaceInner{
    mapper: Arc<Mutex<HashMap<u32, Sender<UserSpaceReq>>>>,
}
impl UserSpaceInner {
    pub fn new() -> UserSpaceInner {
        UserSpaceInner::default()
    }

    pub async fn server_rpc_rec_vec8(&mut self, rpc_http: Vec<u8>) -> Result<Vec<u8>,GenErr>{
        let invoke: pb::Invoke = prost::Message::decode(rpc_http.as_slice())?;
        let act = rpc2::invoke_to_parsed(&invoke)?;
        let user_id = invoke.user_id;

        let (req_sender,mut req_receiver) = oneshot::channel();
        let req_cmd = UserSpaceReq {
            cmd: Commands::RpcDead,
            sender: req_sender,
        };

        let req_sender_stream = self.get_user_stream(user_id);

        let (req_sender,mut req_receiver) = oneshot::channel();
        let req_cmd = UserSpaceReq {
            cmd: Commands::Invoke(act),
            sender: req_sender,
        };
        req_sender_stream.send(req_cmd).await;
        let res =  req_receiver.try_recv();
        println!("req recived  {:?}", res);

        Ok(b"".to_vec())
    }

    pub fn get_user_stream(&mut self, user_id: u32) -> Sender<UserSpaceReq> {
        let mut lock = self.mapper.lock().unwrap();
        let us_opt = lock.deref_mut().get(&user_id);

        let req_sender_stream = match us_opt {
            None => {
                let sender = self.dispatch_new_user_space8(user_id);
                lock.insert(user_id,sender.clone());
                sender
            }
            Some(sender_stream) => {sender_stream.clone()}
        };
       req_sender_stream
    }

    pub fn dispatch_new_user_space8(&self, user_id: u32) -> Sender<UserSpaceReq> {
        let (req_stream_sender,mut req_stream_receiver) = channel(32);

        let mut user_space = UserSpace {
            contacts: Default::default(),
            liked_posts: Default::default(),
            // commands: receiver,
            sender: req_stream_sender.clone(),
            last_rpc: 0,
            user_id: 0,
            user_info: Commands::RpcDead,
            contact3: Default::default(),
            contacts2: vec![]
        };

        // reciver
        tokio::spawn(async move {
            println!("geting  start ");
            loop {
                let res =  req_stream_receiver.recv().await;
                println!("geting  {:?}", res);
                let req = res.unwrap();
                let res =  req.sender.send(b"sucess".to_vec());

            }
        });

        req_stream_sender
    }

    pub fn dispatch_new_user_space5(&mut self, user_id: u32){
        let (req_stream_sender,mut req_stream_receiver) = channel(32);
        let mut lock = self.mapper.lock().unwrap();
        let us = lock.deref_mut().insert(user_id, req_stream_sender.clone());

        let mut user_space = UserSpace {
            contacts: Default::default(),
            liked_posts: Default::default(),
            // commands: receiver,
            sender: req_stream_sender.clone(),
            last_rpc: 0,
            user_id: 0,
            user_info: Commands::RpcDead,
            contact3: Default::default(),
            contacts2: vec![]
        };

        let _sender2 = req_stream_sender.clone();
        tokio::spawn(async move {
            let mut cnt = 0;
            loop {
                let (req_sender,mut req_receiver) = oneshot::channel();
                let req_cmd = UserSpaceReq {
                    cmd: Commands::RpcDead,
                    sender: req_sender,
                };
                req_stream_sender.send(req_cmd).await;
                println!("senidng req {:?}", cnt);
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                // let future = req_receiver.map(|i| {
                //     println!("got: {:?}", i);
                // });
                let res =  req_receiver.try_recv();
                println!("req recived  {:?}", res);
                cnt +=1;
            };
        });

        // reciver
        tokio::spawn(async move {
            println!("geting  start ");
            loop {
                let res =  req_stream_receiver.recv().await;
                println!("geting  {:?}", res);
                let req = res.unwrap();
                let res =  req.sender.send(b"sucess".to_vec());

            }
        });

    }
}

#[derive(Default,Debug)]
pub struct UserSpaceMapper{
    inner: Arc<Mutex<UserSpaceInner>>,
}

impl UserSpaceMapper {
    pub fn new() -> UserSpaceMapper {
        UserSpaceMapper::default()
    }

    pub fn dispatch_new_user_space(&mut self, user_id: u32){
        let mut lock = self.inner.lock().unwrap();

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn rpc_user_space_test1() {
        let invoke = pb::Invoke{
            namespace: 0,
            method: 1897027349,
            user_id: 3,
            action_id: 0,
            session: vec![],
            rpc_data: vec![]
        };
        let mut vec = vec![];
        prost::Message::encode(&invoke,&mut vec).unwrap();

        let mut us = UserSpaceInner::new();
        let out = us.server_rpc_rec_vec8(vec).await;


        println!("test user space");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    #[tokio::test]
    async fn run_user_space_new() {
        let mut us = UserSpaceInner::new();
        us.dispatch_new_user_space5(32);
        println!("test user space");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

    #[tokio::test]
    async fn play_tokio1() {
        let (tx, mut rx) = mpsc::channel(32);
        let tx2 = tx.clone();

        tokio::spawn(async move {
            tx.send("sending from first handle").await;
        });

        tokio::spawn(async move {
            tx2.send("sending from second handle").await;
        });

        while let Some(message) = rx.recv().await {
            println!("GOT = {}", message);
        }
    }

    #[test]
    fn it_works() {
        println!("hwerwe");
        assert_eq!(2 + 2, 4);
    }
}