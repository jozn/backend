use crate::{act, errors::GenErr, pb, utils::time,db_helper};

pub struct ChannelAct {
    db: db_helper::DBMySql,
}

#[rustfmt::skip]
impl ChannelAct{
    // CrUD
    pub async fn create_channel(&self, p: param::CreateChannel) -> Result<pb::Channel,GenErr> {
        let channel_cid = self.db.get_next_cid("channel").await?;

        let channel = pb::Channel{
            channel_cid: channel_cid as u32,
            creator_profile_cid: p.creator_profile_cid,
            is_profile_channel: p.is_def_profile,
            created_time: time::get_time_now_sec(),
            user_name: "".to_string(),
            channel_title: p.channel_title,
            about: p.about,
            is_verified: false,
            sync_time_ms: time::get_time_now_milli(),
            ..Default::default()
        };
        self.db.save_channel(&channel).await?;

        Ok(channel)
    }

    pub async fn edit_channel(&self, channel_cid: u32, p: param::EditChannel) -> Result<pb::Channel,GenErr> {
        let mut ch = self.db.get_channel(channel_cid as u64).await?;

        if p.set_new_title {
            ch.channel_title = p.new_title;
        }
        if p.set_new_about {
            ch.about = p.new_about;
        }

        self.db.save_channel(&ch).await?;

        Ok(ch)
    }

    pub async fn delete_channel(&self, channel_cid: u32, p: param::EditChannel) -> Result<pb::Channel,GenErr>{
        let mut ch = self.db.get_channel(channel_cid as u64).await?;

        ch.is_deleted = 1;//todo to bool

        // todo: delete messages, avatars, comments, ...
        // todo: log this event

        self.db.save_channel(&ch).await?;

        Ok(ch)
    }

    // Following
    pub async fn follow_channel(&self, channel_cid: u32, by_profile_cid: u32) -> Result<(),GenErr> {
        self.db.save_channel_follower(channel_cid,by_profile_cid).await?;
        // todo add 2021_jun

        Ok(())
    }

    pub async fn unfollow_channel(&self) -> Result<pb::Channel,GenErr> {
        let _ = pb::ChannelUnFollowChannelParam::default();
        unimplemented!("");
    }

    // Likes
    pub async fn like_message(&self, channel_cid: u32, message_gid: u64, by_profile_cid: u32) -> Result<(),GenErr> {
        self.db.save_message_like(channel_cid,message_gid,by_profile_cid).await?;
        // todo add 2021_jun

        Ok(())
    }

    pub async fn unlike_message (&self) -> Result<pb::Channel,GenErr> {
        let _ = pb::ChannelUnLikeMessageParam::default();
        unimplemented!("");
    }

    // Messages
    pub async fn add_message(&self, channel_cid: u32, by_profile_cid: u32, msg_in: pb::NewMessageInput) -> Result<pb::Message,GenErr> {
        let msg = pb::Message{
            message_gid: time::get_time_now().as_nanos() as u64, //todo
            by_profile_cid,
            message_type: pb::MessageType::Text as i32 ,
            text: msg_in.text,
            created_time: time::get_time_now_sec(),
            delivery_status: pb::MessageDeliveryStatues::Sent as i32,
            channel_cid,
            counts: None, // not here just view
            ..Default::default()
        };
        self.db.save_channel_message(&msg).await?;

        Ok(msg)
    }

    pub async fn edit_message(&self, p: param::EditMessage) -> Result<pb::Message,GenErr> {
        let mut  msg = self.db.get_channel_message( p.channel_cid as u64, p.message_gid).await?;
        msg.text = p.new_text;
        self.db.save_channel_message(&msg).await?;

        Ok(msg)
    }

    pub async fn delete_messages(&self, channel_cid: u32, message_gid: u64) -> Result<pb::Channel,GenErr> {
        // todo
        let _ = pb::ChannelDeleteMessagesParam::default();
        unimplemented!("");
    }

    // Comment
    pub async fn add_comment(&self, p: param::AddComment) -> Result<pb::Comment,GenErr> {
        let com = pb::Comment{
            comment_gid: 0, // todo shard it
            message_gid: p.message_gid,
            channel_cid: p.channel_cid as u64,
            profile_cid: p.by_profile_cid,
            created_time: time::get_time_now_sec(),
            text: p.comment_text
        };
        self.db.save_message_comment(&com).await?;

        Ok(com)
    }

    pub async fn delete_comment(&self, p: param::DeleteComment) -> Result<pb::Channel,GenErr> {
        // todo
        let _ = pb::ChannelDeleteCommentParam::default();
        unimplemented!("");
    }

}

pub mod param {

    #[derive(Clone, Default, Debug)]
    pub struct CreateChannel {
        pub is_def_profile: bool,
        pub creator_profile_cid: u32,
        pub channel_title: String,
        pub user_name: String,
        pub about: String,
    }

    #[derive(Clone, Default, Debug)]
    pub struct EditChannel {
        // pub channel_cid: u32,
        // pub by_profile_cid: u32,
        pub set_new_title: bool,
        pub new_title: String,
        pub set_new_about: bool,
        pub new_about: String,
    }

    #[derive(Clone, Default, Debug)]
    pub struct EditMessage {
        pub channel_cid: u32,
        pub message_gid: u64,
        pub by_profile_cid: u32,
        pub new_text: String,
    }

    #[derive(Clone, Default, Debug)]
    pub struct AddComment {
        pub channel_cid: u32,
        pub message_gid: u64,
        pub by_profile_cid: u32,
        pub comment_text: String,
    }

    #[derive(Clone, Default, Debug)]
    pub struct DeleteComment {
        pub channel_cid: u32,
        pub message_gid: u64,
        pub by_profile_cid: u32,
        pub comment_gid: u64,
    }

}

// #[cfg(test)]
pub mod tests {
    use super::*;

    // #[test]
    pub async fn play2() {
        let ca = ChannelAct { db: db_helper::DBMySql::new() };
        let p = param::CreateChannel {
            is_def_profile: false,
            creator_profile_cid: 5,
            channel_title: "for fun :)".to_string(),
            user_name: "".to_string(),
            about: "it's a test channel".to_string(),
        };

        ca.create_channel(p).await.unwrap();
    }

    pub async fn play1() {
        let ca = ChannelAct {  db: db_helper::DBMySql::new() };
        let p = pb::NewMessageInput {
            message_gid: 0,
            by_profile_cid: 7,
            message_type: 1,
            text: "heloo 😗 😙 😚".to_string(),
            via_app_id: 0,
            seq: 0,
            created_time: 0,
            verified: false
        };

        ca.add_message(8, 1888, p).await.unwrap();
    }
}
