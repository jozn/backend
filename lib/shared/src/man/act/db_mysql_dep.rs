// use super::db_trait_dep::*;
use crate::gen::pb;
use crate::man::errors::GenErr;
use crate::{common, my_dep, utils::time};
use std::sync::Arc;

#[derive(Debug)]
pub struct DBMySql {
    mysql_pool: Arc<mysql_async::Pool>,
}

#[rustfmt::skip]
impl DBMySql {
    pub fn new() ->Self {
        let database_url = "mysql://flipper:12345678@192.168.92.115:3306/twitter";
        let my = DBMySql {
            mysql_pool: Arc::new(mysql_async::Pool::new(database_url)),
        };
        my
    }

    pub async fn get_next_cid(&self, intent: &str) -> Result<u64, GenErr> {
        let gen_cid = my_dep::GenCid {
            cid: 0,
            intent: intent.to_string(),
        };
        let next_cid = gen_cid.insert(&self.mysql_pool).await?.cid;
        Ok(next_cid as u64)
    }
}

#[rustfmt::skip]
impl DBMySql {
    // =================== Channel ====================
    pub async fn get_channel(&self, channel_id: u64) -> Result<pb::Channel, GenErr> {
        let channel_row = my_dep::ChannelSelector::new()
            .channel_cid_eq(channel_id)
            .get_row(&self.mysql_pool)
            .await?;

        let channel_pb: pb::Channel = common::prost_decode(&channel_row.pb_data)?;

        Ok(channel_pb)
    }

    pub async fn save_channel(&self, channel: &pb::Channel) -> Result<(), GenErr> {
        let buff = common::prost_encode(channel)?;
        let debug_data = format!("{:#?}", &channel);

        let channel_row = my_dep::Channel {
            channel_cid: channel.channel_cid as u64,
            pb_data: buff,
            debug_data: debug_data,
        };
        let r = channel_row.replace(&self.mysql_pool).await?;

        Ok(())
    }

    // =================== Channel Message ====================
    pub async fn get_channel_message(&self, channel_cid: u64, message_gid: u64) -> Result<pb::Message, GenErr> {
        let ch_msg_row = my_dep::ChannelMsgSelector::new()
            .channel_cid_eq(channel_cid)
            .and_msg_gid_eq(message_gid)
            .get_row(&self.mysql_pool)
            .await?;

        let msg_pb: pb::Message = common::prost_decode(&ch_msg_row.pb_data)?;

        Ok(msg_pb)
    }

    pub async fn save_channel_message(&self, message: &pb::Message) -> Result<(), GenErr> {
        let pb_data = common::prost_encode(message)?;
        let debug_data = format!("{:#?}", &message);

        // todo: check gid and channel cid
        let ch_mag_row = my_dep::ChannelMsg {
            channel_cid: message.channel_cid as u64,
            msg_gid: message.message_gid,
            pb_data,
            debug_data
        };
        let r = ch_mag_row.replace(&self.mysql_pool).await?;

        Ok(())
    }

    // =================== Channel Follower ====================
    pub async fn get_channel_followers(&self, channel_cid: u64) -> Result<Vec<u64>, GenErr> {

        unimplemented!("");
    }
    pub async fn save_channel_follower(&self, channel_cid: u32, profile_cid: u32) -> Result<(), GenErr> {
        let ch_fl_row = my_dep::ChannelFollower {
            profile_cid,
            channel_cid,
            created_time: time::get_time_now_sec(),
            follow_id: 0
        };
        let r = ch_fl_row.replace(&self.mysql_pool).await?;

        Ok(())
    }

    // =================== Channel Message Like ====================
    pub async fn get_message_likes(&self, channel_cid: u64, message_gid: u64) -> Result<Vec<u64>, GenErr> {
        unimplemented!("");
    }
    pub async fn save_message_like(&self, channel_cid: u32, message_gid: u64, profile_cid: u32) -> Result<(), GenErr> {
        let ch_lk_row = my_dep::ChannelMsgLike{
            message_gid,
            profile_cid,
            created_time: time::get_time_now_sec()
        };
        let r = ch_lk_row.replace(&self.mysql_pool).await?;

        Ok(())
    }

    // =================== Channel Message Comment ====================
    pub async fn get_message_comments(&self, channel_cid: u64,  message_gid: u64) -> Result<Vec<pb::Comment>, GenErr> {
        let db_rows = my_dep::MsgCommentSelector::new()
            .channel_cid_eq(channel_cid)
            .and_message_gid_eq(message_gid)
            .get_rows(&self.mysql_pool)
            .await?;

        let mut vec_res = Vec::with_capacity(db_rows.len());

        for row in db_rows {
            let comment: pb::Comment = common::prost_decode(&row.pb_data)?;
            vec_res.push(comment);
        }

        Ok(vec_res)
    }
    pub async fn save_message_comment(&self, comment: &pb::Comment) -> Result<(), GenErr> {
        let pb_data = common::prost_encode(comment)?;

        let row = my_dep::MsgComment {
            channel_cid: comment.channel_cid,
            message_gid: comment.message_gid,
            comment_gid: comment.comment_gid,
            pb_data,
            debug_data: format!("{:#?}", &comment)
        };
        let r = row.replace(&self.mysql_pool).await?;

        Ok(())
    }
}

#[rustfmt::skip]
impl DBMySql {
    // =================== User ====================
    pub async fn get_user_by_cid(&self, user_cid: u64) -> Result<pb::User, GenErr> {
        let user_row = my_dep::UserSelector::new()
            .user_cid_eq(user_cid)
            .get_row(&self.mysql_pool)
            .await?;

        let user_pb: pb::User = common::prost_decode(&user_row.pb_data)?;

        Ok(user_pb)
    }

    pub async fn get_user_by_phone(&self, phone: &str) -> Result<pb::User, GenErr>{
        let user_row = my_dep::UserSelector::new()
            .phone_number_eq(phone)
            .get_row(&self.mysql_pool)
            .await?;

        let user_pb: pb::User = common::prost_decode(&user_row.pb_data)?;

        Ok(user_pb)
    }

    pub async fn save_user(&self, user: &pb::User) -> Result<(), GenErr>{
        // user db
        let buff = common::prost_encode(user)?;
        let user_row = my_dep::User {
            user_cid: user.user_cid as u64,
            phone_number: user.phone.clone(),
            pb_data: buff,
            debug_data: format!("{:#?}", user),
        };
        user_row.replace(&self.mysql_pool).await?;

        Ok(())
    }
    // fn delete_user(&self, user: &pb::User) -> Result<(), GenErr>;

    // =================== User Session ====================
    pub async fn get_user_session(&self, user_cid: u64, session_hash: &str) -> Result<pb::Session, GenErr> {
        let session_row = my_dep::SessionSelector::new()
            .user_cid_eq(user_cid as u32)
            .and_session_hash_eq(session_hash)
            .get_row(&self.mysql_pool)
            .await?;

        let session_pb: pb::Session = common::prost_decode(&session_row.pb_data)?;

        Ok(session_pb)
    }

    pub async fn save_user_session(&self, session: &pb::Session) -> Result<(), GenErr> {
        let buff = common::prost_encode(session)?;
        let session_row = my_dep::Session {
            session_hash: session.session_hash.clone(),
            user_cid: session.user_cid, //todo db
            pb_data: buff,
            debug_data: format!("{:#?}", session),
        };
        session_row.replace(&self.mysql_pool).await?;

        Ok(())
    }

    pub async fn delete_user_session(&self, session: &pb::Session) -> Result<(), GenErr>{
        // todo db-walker deleter of rows
        unimplemented!("")
    }
}

impl DBMySql {
    // =================== Profile ====================
    pub async fn get_profile(&self, profile_cid: u64) -> Result<pb::Profile, GenErr> {
        let profile_row = my_dep::ProfileSelector::new()
            .profile_cid_eq(profile_cid)
            .get_row(&self.mysql_pool)
            .await?;

        let profile_pb: pb::Profile = common::prost_decode(&profile_row.pb_data)?;

        Ok(profile_pb)
    }
    pub async fn save_profile(&self, profile: &pb::Profile) -> Result<(), GenErr> {
        let buff = common::prost_encode(profile)?;
        let profile_row = my_dep::Profile {
            profile_cid: profile.profile_cid as u64,
            pb_data: buff,
            debug_data: format!("{:#?}", profile),
        };
        profile_row.replace(&self.mysql_pool).await?;

        Ok(())
    }

    //todo below
    /*    // =================== Profile Following ====================
    pub async fn get_profile_followings(&self, profile_cid: u64) -> Result<Vec<u64>, GenErr>;
    // todo this with channel must be in one batch transaction
    pub async fn save_profile_following(&self, channel_cid: u64, profile_cid: u64) -> Result<(), GenErr>;

    // =================== Profile Contacts ====================
    pub async fn get_profile_contacts(&self, profile_cid: u64) -> Result<Vec<pb::Contact>, GenErr>;
    pub async fn save_profile_contacts(
        &self,
        profile_cid: u64,
        contacts: Vec<pb::Contact>,
    ) -> Result<(), GenErr>;
    pub async fn remove_profile_contacts(&self, profile_cid: u64) -> Result<(), GenErr>;*/
}

// Other db
impl DBMySql {
    // =================== Sms ====================
    pub async fn get_sms(&self, phone: &str, hash_code: &str) -> Result<pb::Sms, GenErr> {
        let sms_row = my_dep::SmsSelector::new()
            .hash_code_eq(hash_code)
            .and_phone_number_eq(phone)
            .get_row(&self.mysql_pool)
            .await?;

        let sms_pb: pb::Sms = common::prost_decode(&sms_row.pb_data)?;

        Ok(sms_pb)
    }
    pub async fn save_sms(&self, sms: &pb::Sms) -> Result<(), GenErr> {
        let buff = common::prost_encode(sms)?;
        let sms_row = my_dep::Sms {
            sms_id: 0,
            phone_number: sms.phone_number.clone(),
            hash_code: sms.hash_code.clone(),
            result_code: 200,
            pb_data: buff,
            debug_data: format!("{:#?}", sms),
        };
        sms_row.insert(&self.mysql_pool).await;

        Ok(())
    }
}

// #[cfg(test)]
pub mod tests {
    use super::*;

    impl DBMySql {
        fn playasd() {}
    }

    // #[test]
    pub async fn play_channel1() {
        let database_url = "mysql://flipper:12345678@192.168.1.115:3306/flip_my";
        let my = DBMySql {
            mysql_pool: Arc::new(mysql_async::Pool::new(database_url)),
        };

        // Save Channel
        let channel = pb::Channel {
            channel_cid: 61,
            creator_profile_cid: 11,
            is_profile_channel: true,
            created_time: 0,
            user_name: "hamid_ria".to_string(),
            channel_title: "My blogs".to_string(),
            about: "anything in my_dep mind".to_string(),
            is_verified: false,
            sync_time_ms: 234,
            ..Default::default()
        };
        my.save_channel(&channel).await;

        // Load Channel
        let ch_load = my.get_channel(61).await.unwrap();
        assert_eq!(ch_load.about, channel.about);

        // Channel Message
        let msg = pb::Message {
            message_gid: 1234,
            by_profile_cid: 11,
            message_type: 4,
            text: "first message to the world 😆".to_string(),
            via_app_id: 2,
            seq: 234,
            created_time: 0,
            channel_cid: 61,
            ..Default::default()
        };
        my.save_channel_message(&msg).await.unwrap();

        // Load Channel Message
        let ch_msg_load = my.get_channel_message(61, 1234).await.unwrap();
        assert_eq!(ch_msg_load.text, msg.text);

        // Save Comment
        let com = pb::Comment {
            comment_gid: 2345,
            message_gid: 1234,
            channel_cid: 61,
            profile_cid: 11,
            created_time: 123234,
            text: "our First comment 💪".to_string(),
        };
        my.save_message_comment(&com).await.unwrap();
    }
}
