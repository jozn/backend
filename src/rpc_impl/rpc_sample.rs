use crate::{com, com::*, pb, sms_sender, utils};
use std::cell::Cell;


pub async fn GetUsers1(up: &UserParam, param: pb::GetUsers1Param) -> Result<pb::GetUsers1Response, GenErr> {
    Ok(pb::GetUsers1Response::default())
    /*Ok(pb::GetUsers1Response{
        users
    })*/
}
const MAX_USER : u32= 1;
#[derive(Debug)]
pub struct MemDb {
    users: Vec<pb::User>,
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
            // ..Default::default() > will stackoverflow
        }
    }
}

impl MemDb {
    pub fn build(&mut self) {
        self.gen_users();
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
                contacts: vec![]
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
}

fn s1(){
    // let db = MemDb{},
    // let users = vec![];

    let mut cid = 0;

    for u in 1..100 {
        let ch = pb::Channel{
            cid: u,
            user_name: format!("profile{}",u),
            channel_name: format!("channel def u#{}",u),
            creator_profile_cid: u,
            is_profile_channel: true,
            about: format!("my about user def ch {}", u),
            ..Default::default()
        };

        let pro = pb::Profile{
            cid: u,
            user_cid: u,
            primary_channel: Some(ch),
            saved_channel: None,
            created_time: 1,
            setting: None,
            channels: vec![],
            directs: vec![],
            groups: vec![],
            contacts: vec![]
        };

        let msg = pb::Message{
            gid: 1,
            by_profile_cid: 5,
            message_type: pb::MessageType::Text as i32,
            text: format!("my message text {}", u),
            via_app_id: 1,
            seq: 1,
            created_time: 2,
            delivery_status: pb::MessageDeliveryStatues::Seen as i32,
            delivery_time: 1,
            counts: None,
            setting: None,
            product: None,
            files: vec![],
            ..Default::default()
        };
    }

}