use shared::{common, common::prost_decode, common::prost_encode, errors::GenErr, pb, xc};
use crate::{db, events, session,db_trait};
use shared::pb::{Comment, Message, Channel};
use prost::alloc::sync::Arc;
use std::sync::{Mutex, MutexGuard};
use std::ops::DerefMut;
use std::collections::HashMap;

#[derive(Default)]
pub struct DBMem {
    col: Arc<Mutex<DBMemInner>>,
    tables: Vec<String>,
}

#[derive(Default)]
struct DBMemInner {
    tables: Vec<String>,
    channels: HashMap<i64,pb::Channel>,
}

impl DBMem {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_inner(&self) -> MutexGuard<DBMemInner> {
        self.col.lock().unwrap()
    }
}
impl db_trait::DB for DBMem {}

impl db_trait::DBChannels for DBMem {
    fn get_channel(&self, channel_id: i64) -> Result<Channel, GenErr> {
       /* let mut m = self.col.lock().unwrap();
        m.tables.push("sdf".to_string());*/


        let mut m = self.get_inner();
        let ch = m.channels.get(&channel_id).ok_or(GenErr::NotFound)?;
        let ch = ch.clone();
        m.tables.push("sdf".to_string());
        println!(" +++++++++++++++++++++++++++++ {:?}", m.tables);

        Ok(ch)
    }

    fn save_channel(&self, channel: &Channel) -> Result<(), GenErr> {
        let mut m = self.get_inner();
        m.channels.insert(channel.channel_cid as i64, channel.clone());
        Ok(())
    }

    fn save_channel_verify(&self, channel: &Channel) -> Result<(), GenErr> {
        unimplemented!()
    }

    fn get_channel_message(&self, channel_id: i64, message_id: i64) -> Result<Message, GenErr> {
        unimplemented!()
    }

    fn save_channel_message(&self, message: &Message) -> Result<(), GenErr> {
        unimplemented!()
    }

    fn get_channel_followers(&self, channel_id: i64) -> Result<Vec<i64>, GenErr> {
        unimplemented!()
    }

    fn save_channel_follower(&self, channel_cid: i64, profile_cid: i64) -> Result<(), GenErr> {
        unimplemented!()
    }

    fn get_message_likes(&self, message_gid: i64) -> Result<Vec<i64>, GenErr> {
        unimplemented!()
    }

    fn save_message_like(&self, message_gid: i64, profile_cid: i64) -> Result<(), GenErr> {
        unimplemented!()
    }

    fn get_message_comments(&self, message_gid: i64) -> Result<Vec<Comment>, GenErr> {
        unimplemented!()
    }

    fn save_message_comment(&self, comment: &Comment) -> Result<(), GenErr> {
        unimplemented!()
    }
}
