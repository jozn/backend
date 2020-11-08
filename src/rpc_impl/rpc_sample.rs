use crate::{com, com::*, pb, sms_sender, utils};
use std::cell::Cell;
use std::borrow::Borrow;
use std::collections::HashMap;


pub async fn GetUsers1(up: &UserParam, param: pb::GetUsers1Param) -> Result<pb::GetUsers1Response, GenErr> {
    Ok(pb::GetUsers1Response::default())
    /*Ok(pb::GetUsers1Response{
        users
    })*/
}
const MAX_USER : u32= 5;
#[derive(Debug)]
pub struct MemDb {
    users: Vec<pb::User>,
    messages: HashMap<u32, Vec<pb::Message>>,
    contacts: HashMap<u32, Vec<pb::Contact>>,
    gid_gen: utils::id_gen::SeqTimeIdGen,
   //  cid3: Cell<u32>,
    cid: u32,
}

impl Default for MemDb {
    fn default() -> Self {
        MemDb {
            users: vec![],
            gid_gen: Default::default(),
            cid: 1,
            messages: HashMap::new(),
            contacts: HashMap::new(),
            // ..Default::default() > will stackoverflow
        }
    }
}

impl MemDb {
    pub fn build(&mut self) {
        self.gen_users();
        self.gen_extra_channels();
        // self.gen_messages();
        self.gen_contacts();
    }
    pub fn get_next_cid(&mut self) -> u32{
        let res = self.cid;
        self.cid += 1;
        res
    }

    fn gen_users(&mut self) {
        for u in 0..MAX_USER {
            let user_cid = self.get_next_cid();
            let pro_cid = self.get_next_cid();
            let chanel_cid = self.get_next_cid();

            let ch = pb::Channel{
                cid: chanel_cid,
                user_name: format!("profile{}",u),
                channel_name: format!("channel def u#{}",u),
                creator_profile_cid: pro_cid,
                is_profile_channel: true,
                about: format!("my about user def ch {}", u),
                ..Default::default()
            };

            let pro = pb::Profile{
                cid: pro_cid,
                user_cid: user_cid,
                primary_channel: Some(ch),
                saved_channel: None,
                created_time: 1,
                setting: None,
                channels: vec![],
                directs: vec![],
                groups: vec![],
                // contacts: //make_contacts(pro_cid, &self),
                ..Default::default()
            };

            let user = pb::User{
                cid: user_cid,
                phone: format!("+9890151323{}", u),
                email: format!("exampl{}@gmail.com", u),
                password_hash: "".to_string(),
                password_salt: "".to_string(),
                created_time: 1,
                version_time: 2,
                is_deleted: false,
                is_banned: false,
                def_profile: Some(pro),
                profiles: vec![],
                stores: vec![],
                sessions: vec![],
                shopping_profile: None,
                first_name: format!("first"),
                last_name: format!("last #{}", u),
                user_counts: None
            };

            self.users.push(user);
        }
    }

    fn gen_extra_channels(&mut self) {
        let user_cid = self.get_next_cid();
        let pro_cid = self.get_next_cid();
        let chanel_cid = self.get_next_cid();

        let id = 0;
        for u in self.users.clone() {
            let mut profile = u.def_profile.borrow().as_ref().unwrap();
            let ch = pb::Channel{
                cid: chanel_cid,
                user_name: format!("Chan#{}",chanel_cid),
                channel_name: format!("channel #{} u{}",chanel_cid, u.cid),
                creator_profile_cid: profile.cid,
                is_profile_channel: false,
                about: format!("my about extra chan {}", u.first_name),
                ..Default::default()
            };

            self.users.get_mut(id).unwrap().def_profile.as_mut().unwrap().channels.push(ch);
           // profile.channels.push(ch);
        }
    }

    fn gen_messages(&mut self) {
        for u in &self.users.clone() {
            // profile msgs
            let profile = u.def_profile.as_ref().unwrap();
            let def_ch = (profile.primary_channel.as_ref()).unwrap();
            self.messages.insert(def_ch.cid, make_channel_msgs(def_ch.clone(), profile.cid));

            // channels msgs
            for ch in &profile.channels {
                self.messages.insert(ch.cid, make_channel_msgs(ch.clone(), profile.cid));
            }
        }
    }

    fn gen_contacts(&mut self) {
        for u in &self.users.clone() {
            let profile = u.def_profile.as_ref().unwrap();
            self.contacts.insert(profile.cid,make_contacts(profile.cid,&self));
        }
    }
}

fn make_channel_msgs(ch: pb::Channel, pid :u32) -> Vec<pb::Message>{
    let mut v =  vec![];
    let mut genid = utils::id_gen::SeqTimeIdGen::new(15);
    for i in 0..40 {
        let m = pb::Message {
            gid: genid.get_next_id().to_u64(),
            by_profile_cid: pid,
            message_type: pb::MessageType::Text as i32,
            text: format!("some random text {}", i),
            via_app_id: 1,
            seq: i+1,
            edited_time: 0,
            created_time: 123,
            verified: false,
            delivery_status: pb::MessageDeliveryStatues::Seen as i32,
            delivery_time: 345,
            deleted: false,
            flags: 0,
            forward: None,
            reply_to: None,
            title: "".to_string(),
            counts: None,
            setting: None,
            product: None,
            files: vec![]
        };
        v.push(m);
    }
    v
}

fn make_contacts(pid :u32, db: &MemDb) -> Vec<pb::Contact>{
    let mut v =  vec![];
    let mut genid = utils::id_gen::SeqTimeIdGen::new(15);
    for u in &db.users {
        let peer_cid = u.def_profile.as_ref().unwrap().cid;
        if peer_cid == pid {
            continue
        }
        let m = pb::Contact {
            gid: genid.get_next_id().to_u64(),
            profile_cid: pid,
            device_id: 0,
            phone: "+989015132328".to_string(),
            first_name: "dev name".to_string(),
            last_name: "last".to_string(),
            peer_profile_cid: peer_cid,
            created_time: 123412341
        };

        v.push(m);
    }
    v
}
