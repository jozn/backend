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
use shared;
use shared::errors::GenErr;
use shared::gen::rpc2::RPC_Shared_Handler2;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};
use shared::pb::*;
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
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;

#[derive(Debug, Default)]
pub struct UserSpaceCache {
    contacts: HashSet<String>,
    liked_posts: HashSet<String>,
}

// No locking
#[derive(Debug)]
pub struct UserSpace {
    sender: Sender<UserSpaceCmdReq>,
    last_rpc: u64,
    user_id: u64,
    cache: UserSpaceCache,
}
impl UserSpace {}

#[async_trait]
impl RPC_Shared_Handler2 for UserSpace {
    async fn SharedEcho(&self, param: SharedEchoParam) -> Result<SharedEchoResponse, GenErr> {
        Ok(pb::SharedEchoResponse {
            done: true,
            text: format!("Echoing:: {}", param.text),
        })
    }
}

#[async_trait]
impl rpc2::RPC_Channel_Handler2 for UserSpace {}

#[derive(Debug)]
pub enum Commands {
    InternalRpc,
    Exit,
    Invoke(shared::rpc2::RpcInvoke),
}

#[derive(Debug)]
pub struct UserSpaceCmdReq {
    cmd: Commands,
    invoke_req: pb::Invoke, // needed?
    result_chan: oneshot::Sender<Vec<u8>>,
}

#[derive(Default, Debug)]
pub struct UserSpaceMapper {
    mapper: Arc<Mutex<HashMap<u32, Sender<UserSpaceCmdReq>>>>,
}
impl UserSpaceMapper {
    pub fn new() -> UserSpaceMapper {
        UserSpaceMapper::default()
    }

    pub async fn serve_rpc_request_vec8(&mut self, rpc_http: Vec<u8>) -> Result<Vec<u8>, GenErr> {
        let invoke: pb::Invoke = prost::Message::decode(rpc_http.as_slice())?;
        let rpc_invoke_enumed = rpc2::invoke_to_parsed(&invoke)?;
        let user_id = invoke.user_id;

        let req_sender_stream = self.get_or_init_userspcace_cmd(user_id);

        let (req_sender, mut req_receiver) = oneshot::channel();
        let us_cmd_req = UserSpaceCmdReq {
            cmd: Commands::Invoke(rpc_invoke_enumed),
            invoke_req: invoke,
            result_chan: req_sender,
        };
        let sending_res = req_sender_stream.send(us_cmd_req).await;
        match sending_res {
            Ok(r) => {}
            Err(er) => {
                println!("+++ send err {}", er);
            }
        }

        match req_receiver.await {
            Ok(val_bts) => Ok(val_bts),
            Err(e) => Err(GenErr::UserSpaceErr),
        }
    }

    fn get_or_init_userspcace_cmd(&mut self, user_id: u32) -> Sender<UserSpaceCmdReq> {
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

    fn dispatch_new_user_space(&self, user_id: u32) -> Sender<UserSpaceCmdReq> {
        let (req_stream_sender, mut req_stream_receiver) = channel(32);
        let req_stream_sender_cp = req_stream_sender.clone();

        // reciver
        tokio::spawn(async move {
            println!("user space start");

            let mut user_space = UserSpace {
                // commands: receiver,
                sender: req_stream_sender_cp,
                last_rpc: 0,
                user_id: user_id as u64,
                cache: Default::default(),
            };

            let arc_us = Arc::new(user_space);

            macro_rules! add {
                () => {
                    Some(Box::new(arc_us.clone()))
                };
            }

            let rpc_handler_registry = shared::rpc2::RPC_Registry {
                RPC_Shared: add!(),
                RPC_Channel: Some(Box::new(arc_us.clone())),
                ..Default::default()
            };

            while let Some(us_cmd_req) = req_stream_receiver.recv().await {
                match us_cmd_req.cmd {
                    Commands::InternalRpc => {}
                    Commands::Exit => break,
                    Commands::Invoke(invoke) => {
                        println!("userspace getting invoke: {:?}", invoke);
                        let out_res = rpc2::server_rpc(invoke, &rpc_handler_registry)
                            .await
                            .unwrap();
                        let res = shared::common::to_invoke_response(
                            out_res.clone(),
                            &us_cmd_req.invoke_req,
                        );
                        let res = us_cmd_req.result_chan.send(out_res); //fixme
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
        let echo = pb::SharedEchoParam {
            text: "Echo me flip".to_string(),
        };
        let mut echo_data = vec![];
        prost::Message::encode(&echo, &mut echo_data).unwrap();

        let invoke = pb::Invoke {
            namespace: 0,
            method: shared::rpc2::method_ids::SharedEcho,
            user_id: 3,
            action_id: 0,
            session: vec![],
            rpc_data: echo_data,
        };
        let mut vec = vec![];
        prost::Message::encode(&invoke, &mut vec).unwrap();

        let mut us = UserSpaceMapper::new();
        let out = us.serve_rpc_request_vec8(vec.clone()).await;

        let o: pb::SharedEchoResponse = shared::common::prost_decode(&out.unwrap()).unwrap();
        println!(">>>>>>>>>> las: {:?} ", o);

        // A sample loop
        for i in 1..100 {
            let out = us.serve_rpc_request_vec8(vec.clone()).await;
            println!("++++++ {}: {:?} ", i, out.unwrap().len());
        }
    }
}
