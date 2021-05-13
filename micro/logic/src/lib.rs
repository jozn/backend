#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_features)]
#![allow(warnings)]
#![allow(soft_unstable)]

use bytes::Bytes;
use shared;
use shared::errors::GenErr;
use shared::{pb, rpc2};
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

mod user_space;

// This is holder of user own data cache in memory.
#[derive(Debug, Default)]
pub struct UserSpaceCache {
    contacts: HashSet<String>,
    liked_posts: HashSet<String>,
}

// This is holder of one user data cache and it's rpc processing functionality. For each user one
//  of this is instanced and in separate async loop waits for UserSpaceCmdReq who passed through
//  channel and then the result send back in oneshot channel.
// Notes: No locking.
#[derive(Debug)]
pub struct UserSpace {
    sender: tokio::sync::mpsc::Sender<UserSpaceCmdReq>, // one receiver, many senders(thread safe)
    last_rpc: u64,
    user_id: u64,
    cache: UserSpaceCache,
    mysql_pool: Arc<mysql_async::Pool>,
}

// This is Command that UserSpace async task process sequentially.
#[derive(Debug)]
pub enum Commands {
    InternalRpc,
    Exit,
    Invoke(shared::rpc2::RpcInvoke),
}

// This holds rpc command data and oneshot channel data.
// Notes: new intance for each rpc request.
#[derive(Debug)]
pub struct UserSpaceCmdReq {
    cmd: Commands,
    invoke_req: pb::Invoke, // needed?
    result_chan: tokio::sync::oneshot::Sender<Vec<u8>>,
}

// This holds a map of user_id to a stream sender channel of UserSpaceCmdReq. Each rpc request will
// be a UserSpaceCmdReq which then passed to this channel and then the result will be handled.
#[derive(Debug)]
pub struct UserSpaceMapper {
    us_cmd_req: Arc<Mutex<HashMap<u32, tokio::sync::mpsc::Sender<UserSpaceCmdReq>>>>,
    mysql_pool: Arc<mysql_async::Pool>,
}
impl UserSpaceMapper {
    pub fn new() -> UserSpaceMapper {
        let database_url = "mysql://flipper:12345678@192.168.193.115:3306/flip_my";
        let pool = mysql_async::Pool::new(database_url);

        UserSpaceMapper {
            us_cmd_req: Arc::new(Mutex::new(Default::default())),
            mysql_pool: Arc::new(pool),
        }
    }

    pub async fn serve_rpc_request(
        &self,
        invoke: pb::Invoke,
    ) -> Result<pb::InvokeResponse, GenErr> {
        let rpc_invoke_enumed = rpc2::invoke_to_parsed(&invoke)?;
        let user_id = invoke.user_id;

        if user_id == 0 {
            // err
        }

        let req_sender_stream = self.get_or_init_userspcace_cmd(user_id);

        let (req_sender, mut req_receiver) = tokio::sync::oneshot::channel();
        let us_cmd_req = UserSpaceCmdReq {
            cmd: Commands::Invoke(rpc_invoke_enumed),
            invoke_req: invoke.clone(),
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
            Ok(val_bts) => {
                let rres = pb::InvokeResponse {
                    namespace: invoke.namespace,
                    method: invoke.user_id,
                    user_id: invoke.user_id,
                    action_id: invoke.action_id,
                    rpc_data: val_bts,
                };
                Ok(rres)
            }
            Err(e) => Err(GenErr::UserSpaceErr),
        }
    }

    // deprecated
    pub async fn serve_rpc_request_vec8_dep(&self, rpc_http: Vec<u8>) -> Result<Vec<u8>, GenErr> {
        let invoke: pb::Invoke = prost::Message::decode(rpc_http.as_slice())?;
        let rpc_invoke_enumed = rpc2::invoke_to_parsed(&invoke)?;
        let user_id = invoke.user_id;

        if user_id == 0 {
            // err
        }

        let req_sender_stream = self.get_or_init_userspcace_cmd(user_id);

        let (req_sender, mut req_receiver) = tokio::sync::oneshot::channel();
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

    // This lockup mapper for the channel of stream of user_id, if not exist then dispatches new
    //  task for user id.
    fn get_or_init_userspcace_cmd(
        &self,
        user_id: u32,
    ) -> tokio::sync::mpsc::Sender<UserSpaceCmdReq> {
        let mut lock = self.us_cmd_req.lock().unwrap();
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

    // This spawn a new async task for UserSpace of user_id. Commands will handled sequentially in
    // user own async task.
    // Notes: Not blocking threads. Each user have one async task. UserSpace is used here. rpc registry
    //  is used in here.
    fn dispatch_new_user_space(&self, user_id: u32) -> tokio::sync::mpsc::Sender<UserSpaceCmdReq> {
        let (req_stream_sender, mut req_stream_receiver) = tokio::sync::mpsc::channel(32);
        let req_stream_sender_cp = req_stream_sender.clone();

        let mysql_arc = self.mysql_pool.clone();
        // receiver
        tokio::spawn(async move {
            println!("user space start");

            let mut user_space = UserSpace {
                sender: req_stream_sender_cp,
                last_rpc: 0,
                user_id: user_id as u64,
                cache: Default::default(),
                mysql_pool: mysql_arc,
            };

            let arc_us = Arc::new(user_space);

            macro_rules! add {
                () => {
                    Some(Box::new(arc_us.clone()))
                };
            }

            let rpc_handler_registry = shared::rpc2::RPC_Registry {
                RPC_Auth: add!(),
                RPC_Shared: add!(),
                RPC_User: add!(),
                RPC_Channel: Some(Box::new(arc_us.clone())), // Explict
                ..Default::default()
            };

            while let Some(us_cmd_req) = req_stream_receiver.recv().await {
                match us_cmd_req.cmd {
                    Commands::InternalRpc => {}
                    Commands::Exit => {
                        println!("Exiting user space of: {:}", user_id);
                        break;
                    }
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
        let out = us.serve_rpc_request_vec8_dep(vec.clone()).await;

        let o: pb::SharedEchoResponse = shared::common::prost_decode(&out.unwrap()).unwrap();
        println!(">>>>>>>>>> las: {:?} ", o);

        // A sample loop
        for i in 1..100 {
            let out = us.serve_rpc_request_vec8_dep(vec.clone()).await;
            println!("++++++ {}: {:?} ", i, out.unwrap().len());
        }
    }
}
