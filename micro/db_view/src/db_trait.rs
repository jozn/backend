use crate::session;
use shared::{common, common::prost_decode, common::prost_encode, errors::GenErr, pb, xc};

//todo: Change save fns to owned

// Channels Impl
pub trait DBChannels {
    // =================== Channel ====================
    fn get_channel(&self, channel_id: i64) -> Result<pb::Channel, GenErr>;

    fn save_channel(&self, channel: &pb::Channel) -> Result<(), GenErr>;

    //fn save_channel_verify(&self, channel: &pb::Channel) -> Result<(), GenErr>;
    // =================== Channel Message ====================
    fn get_channel_message(&self, channel_id: i64, message_id: i64) -> Result<pb::Message, GenErr>;

    fn save_channel_message(&self, message: &pb::Message) -> Result<(), GenErr>;

    // =================== Channel Follower ====================
    fn get_channel_followers(&self, channel_cid: i64) -> Result<Vec<i64>, GenErr>;

    fn save_channel_follower(&self, channel_cid: i64, profile_cid: i64) -> Result<(), GenErr>;

    // =================== Channel Message Like ====================
    fn get_message_likes(&self, message_gid: i64) -> Result<Vec<i64>, GenErr>;

    fn save_message_like(&self, message_gid: i64, profile_cid: i64) -> Result<(), GenErr>;

    // =================== Channel Message Comment ====================
    fn get_message_comments(&self, message_gid: i64) -> Result<Vec<pb::Comment>, GenErr>;
    fn save_message_comment(&self, comment: &pb::Comment) -> Result<(), GenErr>;
}

pub trait DB: DBChannels {}

/*
// Chat Impl
impl DBCassandra {
    // =================== Chat ====================
    fn get_chat(&self, profile_id: i64, chat_id: i64) -> Result<pb::Chat, GenErr> {
        let sess = &self.session;

        let r = xc::get_chat_by_profile_cid_and_chat_gid(sess, profile_id, chat_id)?;
        let chat = common::prost_decode(&r.pb_data)?;

        Ok(chat)
    }

    fn save_chat(&self, chat: &pb::Chat) -> Result<(), GenErr> {
        let sess = &self.session;

        //todo should be in batch

        let pb = prost_encode(chat)?;
        // For Profile 1
        let r1 = xc::Chat {
            chat_gid: chat.chat_gid as i64,
            profile_cid: chat.profile1_cid as i64,
            pb_data: pb.clone(),
        };
        r1.save(sess)?;

        // For Profile 2
        let r2 = xc::Chat {
            chat_gid: chat.chat_gid as i64,
            profile_cid: chat.profile2_cid as i64,
            pb_data: pb,
        };
        r2.save(sess)?;

        Ok(())
    }

    // =================== Chat Message ====================
    fn get_chat_message(
        //todo imple
        &self,
        channel_id: i64,
        message_id: i64,
    ) -> Result<pb::Message, GenErr> {
        /* let sess = &self.session;

        let r = xc::get_channel_msg_by_channel_id_and_msg_id(sess, channel_id, message_id)?;
        let ch = common::prost_decode(&r.pb_data)?;

        Ok(ch)*/
        unimplemented!("")
    }

    fn save_chat_message(&self, message: &pb::Message) -> Result<(), GenErr> {
        /*        let sess = &self.session;

        let pb = prost_encode(message)?;
        let r = xc::ChannelMsg {
            channel_id: message.cid as i64,
            msg_id: message.gid as i64,
            pb_data: pb,
        };
        r.save(sess)?;

        Ok(())*/
        unimplemented!("")
    }
}

// Profile Impl
impl DBCassandra {
    // =================== Profile ====================
    fn get_profile(&self, profile_cid: i64) -> Result<pb::Profile, GenErr> {
        let sess = &self.session;

        let r = xc::get_profile_by_profile_cid(sess, profile_cid)?;
        let chat = common::prost_decode(&r.pb_data)?;

        Ok(chat)
    }

    fn save_profile(&self, profile: &pb::Profile) -> Result<(), GenErr> {
        let sess = &self.session;

        let pb = prost_encode(profile)?;
        let r = xc::Profile {
            profile_cid: profile.profile_cid as i64,
            pb_data: pb,
        };
        r.save(sess)?;

        Ok(())
    }

    // =================== Profile Following ====================
    fn get_profile_followings(&self, profile_cid: i64) -> Result<Vec<i64>, GenErr> {
        let sess = &self.session;

        let rows = xc::ProfileFollower_Selector::new()
            .profile_cid_eq(profile_cid)
            .get_rows(sess)?;
        let mut out = vec![];
        for r in rows {
            out.push(r.channel_cid);
        }

        Ok(out)
    }

    // todo this with channel must be in one batch transaction
    fn save_profile_following(&self, channel_cid: i64, profile_cid: i64) -> Result<(), GenErr> {
        let sess = &self.session;

        let r = xc::ProfileFollower {
            follow_gid: 0, //todo
            channel_cid,
            profile_cid,
        };
        r.save(sess)?;

        Ok(())
    }
}
*/
