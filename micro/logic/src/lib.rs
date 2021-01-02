#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_features)]
#![allow(warnings)]
#![allow(soft_unstable)]
use async_trait::async_trait;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use bytes::Bytes;
use once_cell::sync::OnceCell;
use rocksdb;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};
use shared::rpc2::{IPC_CMaster_Handler2, RPC_Auth_Handler2, RPC_Registry};
use shared::{pb, rpc2};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::iter::Map;
use std::ops::{Deref, DerefMut};
use std::panic::RefUnwindSafe;
use std::sync::atomic::Ordering;
use std::sync::{atomic, Arc, Mutex};
use std::thread;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot;
use shared::pb::{ConfirmCodeParam, ConfirmCodeResponse};

#[derive(Debug, Default)]
pub struct UserSpaceCache {
    contacts: HashSet<String>,
    liked_posts: HashSet<String>,
    //del?
    contact3: HashMap<String, bool>,
    contacts2: Vec<u64>,
}

// no locking
#[derive(Debug)]
pub struct UserSpace {
    sender: Sender<UserSpaceCmd>,
    last_rpc: u64,
    user_id: u64,
    user_info: Commands, // pb
    cache: UserSpaceCache,
}
impl UserSpace {

}

#[async_trait]
impl RPC_Auth_Handler2 for UserSpace {
    async fn ConfirmCode(&self, param: ConfirmCodeParam) -> Result<ConfirmCodeResponse, GenErr> {
        println!("here is the way to go");
        Ok(pb::ConfirmCodeResponse{ done: true, error_message: "ssdf sdfsdfsd fsd flsa".to_string() })
    }
}

#[async_trait]
impl rpc2::RPC_Channel_Handler2 for UserSpace {}

#[derive(Debug)]
pub enum Commands {
    RpcDead,
    InternalRpc,
    Exit,
    Invoke(shared::rpc2::RpcInvoke),
}

#[derive(Debug)]
pub struct UserSpaceCmd {
    cmd: Commands,
    invoke_req: pb::Invoke,
    result: oneshot::Sender<Vec<u8>>,
}

#[derive(Default, Debug)]
pub struct UserSpacMapper {
    mapper: Arc<Mutex<HashMap<u32, Sender<UserSpaceCmd>>>>,
}
impl UserSpacMapper {
    pub fn new() -> UserSpacMapper {
        UserSpacMapper::default()
    }

    pub async fn server_rpc_rec_vec8(&mut self, rpc_http: Vec<u8>) -> Result<Vec<u8>, GenErr> {
        let invoke: pb::Invoke = prost::Message::decode(rpc_http.as_slice())?;
        let act = rpc2::invoke_to_parsed(&invoke)?;
        let user_id = invoke.user_id;

        let req_sender_stream = self.get_user_stream(user_id);

        let (req_sender, mut req_receiver) = oneshot::channel();
        let req_cmd = UserSpaceCmd {
            cmd: Commands::Invoke(act),
            invoke_req: invoke,
            result: req_sender,
        };
        req_sender_stream.send(req_cmd).await;
        let res = req_receiver.try_recv();
        println!("req recived  {:?}", res);

        Ok(b"".to_vec())
    }

    pub fn get_user_stream(&mut self, user_id: u32) -> Sender<UserSpaceCmd> {
        let mut lock = self.mapper.lock().unwrap();
        let us_opt = lock.deref_mut().get(&user_id);

        let req_sender_stream = match us_opt {
            None => {
                let sender = self.dispatch_new_user_space(user_id);
                lock.insert(user_id, sender.clone());
                sender
            }
            Some(sender_stream) => sender_stream.clone(),
        };
        req_sender_stream
    }

    pub fn dispatch_new_user_space(&self, user_id: u32) -> Sender<UserSpaceCmd> {
        let (req_stream_sender, mut req_stream_receiver) = channel(32);

        let mut user_space = UserSpace {
            // commands: receiver,
            sender: req_stream_sender.clone(),
            last_rpc: 0,
            user_id: 0,
            user_info: Commands::RpcDead,
            cache: Default::default(),
        };

        // reciver
        tokio::spawn(async move {
            println!("geting  start ");
            let arc_us = Arc::new(user_space);

            macro_rules! add{
                () => {
                    Some(Box::new(arc_us.clone()))
                }
            }

            let reg = shared::rpc2::RPC_Registry {
                RPC_Auth: add!(),
                RPC_Channel: Some(Box::new(arc_us.clone())),
                ..Default::default()
            };

            while let Some(us_cmd) = req_stream_receiver.recv().await {
                println!(">>>> Registry ");
                match us_cmd.cmd {
                    Commands::RpcDead => {}
                    Commands::InternalRpc => {}
                    Commands::Exit => break,
                    Commands::Invoke(invoke) => {
                        println!("getting  {:?}", invoke);
                        let out_res = rpc2::server_rpc(invoke, &reg).await.unwrap();
                        let res = shared::common::to_invoke_response(out_res, &us_cmd.invoke_req);
                        let res = us_cmd.result.send(b"sucess".to_vec());
                    }
                }
            }
        });

        req_stream_sender
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn rpc_user_space_test1() {
        let invoke = pb::Invoke {
            namespace: 0,
            method: shared::rpc2::method_ids::ConfirmCode,
            user_id: 3,
            action_id: 0,
            session: vec![],
            rpc_data: vec![],
        };
        let mut vec = vec![];
        prost::Message::encode(&invoke, &mut vec).unwrap();

        let mut us = UserSpacMapper::new();
        let out = us.server_rpc_rec_vec8(vec.clone()).await;
        let out = us.server_rpc_rec_vec8(vec).await;

        println!("test user space");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

    #[test]
    fn it_works() {
        println!("hwerwe");
        assert_eq!(2 + 2, 4);
    }
}
