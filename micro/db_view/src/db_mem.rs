#![feature(negative_impls)]

use shared::{common, common::prost_decode, common::prost_encode, errors::GenErr, pb, xc};
use crate::{db, events, session,db_trait};
use shared::pb::{Comment, Message, Channel};
use prost::alloc::sync::Arc;
use std::sync::{Mutex, MutexGuard};
use std::ops::DerefMut;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

#[derive(Default)]
pub struct DBMem {
    col: Arc<Mutex<DBMemInner>>,
    tables: Vec<String>,
}

#[derive(Default)]
struct DBMemInner {
    tables: Vec<String>,
    channels: HashMap<i64,pb::Channel>,
    channel_msgs: HashMap<i64,HashMap<i64, pb::Message>>,
    channel_followers: HashMap<i64,Vec<i64>>,
    msgs_likes: HashMap<i64,Vec<i64>>,
    msg_comment: HashMap<i64,Vec<pb::Comment>>,
}

// impl !Sync for DBMemInner{}

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
        let mut m = self.get_inner();
        let ch = m.channels.get(&channel_id).ok_or(GenErr::NotFound)?;
        Ok(ch.clone())
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
        let mut m = self.get_inner();
        let mp = m.channel_msgs.get(&channel_id).ok_or(GenErr::NotFound)?;
        let msg = mp.get(&message_id).ok_or(GenErr::NotFound)?;
        Ok(msg.clone())
    }

    fn save_channel_message(&self, message: &Message) -> Result<(), GenErr> {
        let mut m = self.get_inner();
        let mut mp = m.channel_msgs.get_mut(&(message.channel_cid as i64));
        if mp.is_none() {
            // m.channel_msgs.insert(message.channel_cid as i64, HashMap::new());
        }
        let mut mp = m.channel_msgs.get_mut(&(message.channel_cid as i64));
        // mp.insert(message.message_gid as i64, message.clone());
       match mp {
            None => {
                let mut nm  = HashMap::new();
                nm.insert(message.message_gid as i64, message.clone());
                m.channel_msgs.insert(message.channel_cid as i64, nm);

            }
            Some(r) => {
                r.insert(message.message_gid as i64, message.clone());
            }
        };
        Ok(())
    }

    fn get_channel_followers(&self, channel_id: i64) -> Result<Vec<i64>, GenErr> {
        let mut m = self.get_inner();
        let arr = m.channel_followers.get(&channel_id);
        match arr {
            None => {
                Ok(vec![])
            }
            Some(a) => {
                Ok(a.clone())
            }
        }
    }

    fn save_channel_follower(&self, channel_cid: i64, profile_cid: i64) -> Result<(), GenErr> {
        let mut m = self.get_inner();
        let arr = m.channel_followers.get_mut(&channel_cid);
        match arr {
            None => {
                let mut a = vec![];
                a.push(profile_cid);
                m.channel_followers.insert(channel_cid, a);

            }
            Some(a) => {
                a.push(profile_cid)
            }
        }
        Ok(())
    }

    fn get_message_likes(&self, message_gid: i64) -> Result<Vec<i64>, GenErr> {
        let mut m = self.get_inner();
        let arr = m.msgs_likes.get(&message_gid);
        match arr {
            None => {
                Ok(vec![])
            }
            Some(a) => {
                Ok(a.clone())
            }
        }
    }

    fn save_message_like(&self, message_gid: i64, profile_cid: i64) -> Result<(), GenErr> {
        let mut m = self.get_inner();
        let arr = m.msgs_likes.get_mut(&message_gid);
        match arr {
            None => {
                let mut a = vec![];
                a.push(profile_cid);
                m.channel_followers.insert(message_gid, a);

            }
            Some(a) => {
                a.push(profile_cid)
            }
        }
        Ok(())
    }

    fn get_message_comments(&self, message_gid: i64) -> Result<Vec<Comment>, GenErr> {
        let mut m = self.get_inner();
        let arr = m.msg_comment.get(&message_gid);
        match arr {
            None => {
                Ok(vec![])
            }
            Some(a) => {
                Ok(a.clone())
            }
        }
    }

    fn save_message_comment(&self, comment: &Comment) -> Result<(), GenErr> {
        let mut m = self.get_inner();
        let arr = m.msg_comment.get_mut(&(comment.message_gid as i64));
        match arr {
            None => {
                let mut a = vec![];
                a.push(comment.clone());
                m.msg_comment.insert(comment.message_gid as i64, a);

            }
            Some(a) => {
                a.push(comment.clone())
            }
        }
        Ok(())
    }
}
