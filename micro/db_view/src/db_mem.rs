#![feature(negative_impls)]

use crate::{db, db_trait, events, session};
use prost::alloc::sync::Arc;
use shared::pb::{Channel, Comment, Message, Session, User, Profile, Contact};
use shared::{common, common::prost_decode, common::prost_encode, errors::GenErr, pb, xc};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Mutex, MutexGuard};

#[derive(Default)]
pub struct DBMem {
    col: Arc<Mutex<DBMemInner>>,
    tables: Vec<String>,
}

#[derive(Default)]
struct DBMemInner {
    // User
    users: HashMap<i64, pb::User>,
    user_sessions: HashMap<i64, HashMap<String, pb::Session>>,

    tables: Vec<String>,
    channels: HashMap<i64, pb::Channel>,
    channel_msgs: HashMap<i64, HashMap<i64, pb::Message>>,
    channel_followers: HashMap<i64, Vec<i64>>,
    msgs_likes: HashMap<i64, Vec<i64>>,
    msg_comment: HashMap<i64, Vec<pb::Comment>>,

    profiles_old: HashMap<i64, pb::Profile>,
    profiles: HashMap<i64, Arc<Mutex<MemProfile>>>,

    chats: HashMap<i64, Arc<Mutex<MemChat>>>,

}

#[derive(Default, Debug)]
struct MemProfile {
    profiles: pb::Profile,
    following: Vec<i64>,
    contacts: Vec<Contact>,
}

#[derive(Default, Debug)]
struct MemChat {
   chat: pb::Chat,
   messages: Vec<pb::Message>,
}

// impl !Sync for DBMemInner{}

impl DBMem {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_inner(&self) -> MutexGuard<DBMemInner> {
        self.col.lock().unwrap()
    }

    fn get_profile_inner(&self, profile_cid: i64) -> Result<Arc<Mutex<MemProfile>>, GenErr> {
        let mut m = self.col.lock().unwrap();
        let profile = m.profiles.get(&profile_cid).ok_or(GenErr::NotFound)?;
        let o = profile.clone();
        Ok(o)
    }

    fn get_chat_inner(&self, profile_cid: i64) -> Result<Arc<Mutex<MemChat>>, GenErr> {
        let mut m = self.col.lock().unwrap();
        let arc = m.chats.get(&profile_cid).ok_or(GenErr::NotFound)?;
        let o = arc.clone();
        Ok(o)
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
        m.channels
            .insert(channel.channel_cid as i64, channel.clone());
        Ok(())
    }

    fn get_channel_message(&self, channel_id: i64, message_id: i64) -> Result<Message, GenErr> {
        let mut m = self.get_inner();
        // println!("#2.5 {:?}", m.channel_msgs.len());
        let mp = m.channel_msgs.get(&channel_id).ok_or(GenErr::NotFound)?;
        // let mp = m.channel_msgs.get(&101);
        // for v in m.channel_msgs.iter(){
        //     println!("#1 {}", v.0);
        // }
        // println!("#3 {}", mp.is_some());
        let msg = mp.get(&message_id).ok_or(GenErr::NotFound)?;
        Ok(msg.clone())
    }

    fn save_channel_message(&self, message: &Message) -> Result<(), GenErr> {
        let mut m = self.get_inner();
        let channel_cid = message.channel_cid as i64;
        let mut mp = m.channel_msgs.get_mut(&channel_cid);
        match mp {
            None => {
                let mut nm = HashMap::new();
                nm.insert(message.message_gid as i64, message.clone());
                m.channel_msgs.insert(channel_cid, nm);
                // println!("#2");
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
            None => Ok(vec![]),
            Some(a) => Ok(a.clone()),
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
            Some(a) => a.push(profile_cid),
        }
        Ok(())
    }

    fn get_message_likes(&self, message_gid: i64) -> Result<Vec<i64>, GenErr> {
        let mut m = self.get_inner();
        let arr = m.msgs_likes.get(&message_gid);
        match arr {
            None => Ok(vec![]),
            Some(a) => Ok(a.clone()),
        }
    }

    fn save_message_like(&self, message_gid: i64, profile_cid: i64) -> Result<(), GenErr> {
        let mut m = self.get_inner();
        let arr = m.msgs_likes.get_mut(&message_gid);
        match arr {
            None => {
                let mut a = vec![];
                a.push(profile_cid);
                m.msgs_likes.insert(message_gid, a);
            }
            Some(a) => a.push(profile_cid),
        }
        Ok(())
    }

    fn get_message_comments(&self, message_gid: i64) -> Result<Vec<Comment>, GenErr> {
        let mut m = self.get_inner();
        let arr = m.msg_comment.get(&message_gid);
        match arr {
            None => Ok(vec![]),
            Some(a) => Ok(a.clone()),
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
            Some(a) => a.push(comment.clone()),
        }
        Ok(())
    }
}

impl db_trait::DBUser for DBMem {
    fn get_user_by_cid(&self, user_cid: i64) -> Result<User, GenErr> {
        let mut m = self.get_inner();
        let r = m.users.get(&user_cid).ok_or(GenErr::NotFound)?;
        Ok(r.clone())
    }

    fn get_user_by_phone(&self, phone: &str) -> Result<User, GenErr> {
        let mut m = self.get_inner();
        for (i, u) in m.users.iter() {
            if u.phone == phone {
                return Ok(u.clone());
            }
        }
        Err(GenErr::NotFound)
    }

    fn save_user(&self, user: &User) -> Result<(), GenErr> {
        let mut m = self.get_inner();
        m.users.insert(user.user_cid as i64, user.clone());
        Ok(())
    }

/*    fn delete_user(&self, user: &User) -> Result<(), GenErr> {
        let mut u = user.clone();
        u.is_deleted = true;
        self.save_user(&u)
    }*/

    fn get_user_session(&self, user_cid: i64, session_id: String) -> Result<Session, GenErr> {
        let mut m = self.get_inner();
        let mp = m.user_sessions.get(&user_cid).ok_or(GenErr::NotFound)?;
        let sess = mp.get(&session_id).ok_or(GenErr::NotFound)?;
        Ok(sess.clone())
    }

    fn save_user_session(&self, session: &Session) -> Result<(), GenErr> {
        let mut m = self.get_inner();
        let user_cid = session.user_cid as i64;
        let channel_cid = session.session_uuid.clone();
        let mut mp = m.user_sessions.get_mut(&user_cid);
        match mp {
            None => {
                let mut nm = HashMap::new();
                nm.insert(channel_cid.clone(), session.clone());
                m.user_sessions.insert(user_cid, nm);
            }
            Some(r) => {
                r.insert(channel_cid.clone(), session.clone());
            }
        };
        Ok(())
    }

    fn delete_user_session(&self, session: &Session) -> Result<(), GenErr> {
        let mut m = self.get_inner();
        let channel_cid = session.session_uuid.clone();
        let mut mp = m.user_sessions.get_mut(&(session.user_cid as i64));
        match mp {
            None => {}
            Some(r) => {
                r.remove(&channel_cid);
            }
        };
        Ok(())
    }
}

impl db_trait::DBProfile for DBMem {
    fn get_profile(&self, profile_cid: i64) -> Result<Profile, GenErr> {
        let mut m = self.get_inner();
        let profile = m.profiles_old.get(&profile_cid).ok_or(GenErr::NotFound)?;
        Ok(profile.clone())
    }

    fn save_profile(&self, profile: &Profile) -> Result<(), GenErr> {
        let mut m = self.get_inner();
        m.profiles_old.insert(profile.profile_cid as i64, profile.clone());
        Ok(())
    }

    fn get_profile_followings(&self, profile_cid: i64) -> Result<Vec<i64>, GenErr> {
        let p = self.get_profile_inner(profile_cid)?;
        let p = p.lock().unwrap();
        Ok(p.following.clone())
    }

    fn save_profile_following(&self, channel_cid: i64, profile_cid: i64) -> Result<(), GenErr> {
        let p = self.get_profile_inner(profile_cid)?;
        let mut p = p.lock().unwrap();
        p.following.push(channel_cid);
        Ok(())
    }

    fn get_profile_contacts(&self, profile_cid: i64) -> Result<Vec<Contact>, GenErr> {
        let p = self.get_profile_inner(profile_cid)?;
        let p = p.lock().unwrap();
        Ok(p.contacts.clone())
    }

    fn save_profile_contacts(&self,profile_cid: i64, contacts: Vec<Contact>) -> Result<(), GenErr> {
        let p = self.get_profile_inner(profile_cid)?;
        let mut p = p.lock().unwrap();
        p.contacts.clear();
        p.contacts = contacts;
        Ok(())
    }

    fn remove_profile_contacts(&self, profile_cid: i64) -> Result<(), GenErr> {
        let p = self.get_profile_inner(profile_cid)?;
        let mut p = p.lock().unwrap();
        p.contacts.clear();
        Ok(())
    }
}

