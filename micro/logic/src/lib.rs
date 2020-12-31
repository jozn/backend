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
use std::iter::Map;
use std::ops::{Deref, DerefMut};

// no locking
#[derive(Debug)]
pub struct UserSpace {
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

#[derive(Debug)]
pub enum Commands {
    Rpc,
    InternalRpc,
    Exit,
}

#[derive(Default,Debug)]
pub struct UserSpaceInner{
    maper: Arc<Mutex<HashMap<u32, UserSpace>>>,
}
impl UserSpaceInner {
    pub fn new() -> UserSpaceInner {
        UserSpaceInner::default()
    }

    pub fn dispatch_new_user_space3(&mut self, user_id: u32){
        let (sender, receiver) = channel(32);
        let user_space = UserSpace{
            contacts: Default::default(),
            liked_posts: Default::default(),
            // commands: receiver,
            sender: sender,
            last_rpc: 0,
            user_id: 0,
            user_info: Commands::Rpc,
            contact3: Default::default(),
            contacts2: vec![]
        };

        let mut lock = self.maper.lock().unwrap();
        let us = lock.deref_mut().insert(user_id, user_space);

    }

    pub fn dispatch_new_user_space2(&mut self, user_id: u32){
        let (sender,mut receiver) = channel(32);
        let mut user_space = UserSpace{
            contacts: Default::default(),
            liked_posts: Default::default(),
            // commands: receiver,
            sender: sender.clone(),
            last_rpc: 0,
            user_id: 0,
            user_info: Commands::Rpc,
            contact3: Default::default(),
            contacts2: vec![]
        };
        let sender2 = sender.clone();
        tokio::spawn(async move {
            let mut cnt = 0;
            loop {
                sender2.send(Commands::Rpc).await;
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
        us.dispatch_new_user_space3(32);
        println!("test user space");
    }

    #[tokio::test]
    async fn run_user_space_async() {
        let mut us = UserSpaceInner::new();
        us.dispatch_new_user_space2(32);
        println!("test user space async");
        tokio::time::sleep(std::time::Duration::from_secs(8)).await;
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