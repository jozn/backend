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
use tokio::sync::mpsc::{channel, Receiver,Sender};
use tokio::sync::oneshot;
use std::iter::Map;
use std::ops::{Deref, DerefMut};
use std::thread;

// no locking
#[derive(Debug)]
pub struct UserSpaceOld {
    contacts: HashSet<String>,
    liked_posts: HashSet<String>,
    // commands: Receiver<Commands>,
    sender: Sender<Commands>,
    last_rpc: u64,

    user_id: u64,
    user_info: Commands, // pb

    contact3: HashMap<String, bool>,
    contacts2: Vec<u64>,
}

// no locking
#[derive(Debug)]
pub struct UserSpace {
    contacts: HashSet<String>,
    liked_posts: HashSet<String>,
    // commands: Receiver<Commands>,
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

#[derive(Default,Debug)]
pub struct UserSpaceInner{
    maper: Arc<Mutex<HashMap<u32, UserSpaceOld>>>,
    maper2: Arc<Mutex<HashMap<u32, Sender<UserSpaceReq>>>>,
}
impl UserSpaceInner {
    pub fn new() -> UserSpaceInner {
        UserSpaceInner::default()
    }

    pub fn dispatch_new_user_space3(&mut self, user_id: u32){
        let (sender, receiver) = channel(32);
        let user_space = UserSpaceOld {
            contacts: Default::default(),
            liked_posts: Default::default(),
            // commands: receiver,
            sender: sender,
            last_rpc: 0,
            user_id: 0,
            user_info: Commands::RpcDead,
            contact3: Default::default(),
            contacts2: vec![]
        };

        let mut lock = self.maper.lock().unwrap();
        let us = lock.deref_mut().insert(user_id, user_space);

    }

    pub fn dispatch_new_user_space5(&mut self, user_id: u32){
        let (sender,mut receiver) = channel(32);
        let mut lock = self.maper2.lock().unwrap();
        let us = lock.deref_mut().insert(user_id, sender.clone());

        let mut user_space = UserSpace {
            contacts: Default::default(),
            liked_posts: Default::default(),
            // commands: receiver,
            sender: sender.clone(),
            last_rpc: 0,
            user_id: 0,
            user_info: Commands::RpcDead,
            contact3: Default::default(),
            contacts2: vec![]
        };
        let sender2 = sender.clone();
        tokio::spawn(async move {
            let mut cnt = 0;
            loop {
                let (req_sender,mut req_receiver) = oneshot::channel();
                let req_cmd = UserSpaceReq {
                    cmd: Commands::RpcDead,
                    sender: req_sender,
                };
                sender2.send(req_cmd).await;
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
                let res =  receiver.recv().await;
                println!("geting  {:?}", res);
                let req = res.unwrap();
                let res =  req.sender.send(b"sucess".to_vec());

            }
        });

    }

    pub fn dispatch_new_user_space2(&mut self, user_id: u32){
        let (sender,mut receiver) = channel(32);
        let mut user_space = UserSpaceOld {
            contacts: Default::default(),
            liked_posts: Default::default(),
            // commands: receiver,
            sender: sender.clone(),
            last_rpc: 0,
            user_id: 0,
            user_info: Commands::RpcDead,
            contact3: Default::default(),
            contacts2: vec![]
        };
        let sender2 = sender.clone();
        tokio::spawn(async move {
            let mut cnt = 0;
            loop {
                sender2.send(Commands::RpcDead).await;
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                println!("senidng {:?}", cnt);
                cnt +=1;
            };
        });

        tokio::spawn(async move {
            let mut cnt = 0;
            println!("geting  start ");
            loop {
                let res =  receiver.recv().await;
                println!("geting  {:?}", res);
            }
            /*            while let Some(cmd) = receiver.recv().await {
                            println!("geting  {:?}", cmd);
                            cnt +=1;
                        };*/
        });

        tokio::spawn(async move {
            let mut cnt = 0;
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                println!("{:?}", cnt);
                cnt +=1;
            };
        });
        let mut lock = self.maper.lock().unwrap();
        let us = lock.deref_mut().get(&user_id);

    }

    pub fn dispatch_new_user_space(&mut self, user_id: u32){
        let mut lock = self.maper.lock().unwrap();
        let us = lock.deref_mut().get(&user_id).unwrap();

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

    #[test]
    fn run_user_space() {
        let mut us = UserSpaceInner::new();
        us.dispatch_new_user_space2(32);
        println!("test user space");
    }

    #[tokio::test]
    async fn run_user_space_new() {
        let mut us = UserSpaceInner::new();
        us.dispatch_new_user_space5(32);
        println!("test user space");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

    #[tokio::test]
    async fn run_user_space_async() {
        let mut us = UserSpaceInner::new();
        us.dispatch_new_user_space2(32);
        println!("test user space async");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
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