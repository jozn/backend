
pub struct ChannelAct {
    db: DBMySql,
}

use crate::{act,pb,errors::GenErr,utils::time};
use crate::man::act::DBMySql;

#[rustfmt::skip]
impl ChannelAct{
    pub async fn channel_create_channel(&self, p: param::CreateChannel) -> Result<pb::Channel,GenErr> {
        let channel_cid = self.db.get_next_cid("channel").await?;

        let channel = pb::Channel {
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

    pub async fn channel_edit_channel(&self, channel_cid: u32, p: param::EditChannel) -> Result<pb::Channel,GenErr> {
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

    pub async fn channel_delete_channel(&self, channel_cid: u32, p: param::EditChannel) -> Result<pb::Channel,GenErr>{
        let mut ch = self.db.get_channel(channel_cid as u64).await?;

        ch.is_deleted = 1;//todo to bool

        // todo: delete messages, avatars, comments, ...
        // todo: log this event

        self.db.save_channel(&ch).await?;

        Ok(ch)
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
    pub struct EditChannel{
        // pub channel_cid: u32,
        // pub by_profile_cid: u32,
        pub set_new_title: bool,
        pub new_title: String,
        pub set_new_about: bool,
        pub new_about: String,
    }


}


// Deprecated
#[derive(Clone, Default, Debug)]
pub struct CreateChannelParam_old {
    pub channel_cid: u32,
    pub is_def_profile: bool,
    pub creator_profile_cid: u32,
    pub channel_title: String,
    pub user_name: String,
    pub about: String,
}


// #[cfg(test)]
pub mod tests {
    use super::*;

    // #[test]
    pub async fn play1() {
        let ca = ChannelAct{
            db: DBMySql::new()
        };
        let p = param::CreateChannel{
            is_def_profile: false,
            creator_profile_cid: 5,
            channel_title: "for fun :)".to_string(),
            user_name: "".to_string(),
            about: "it's a test channel".to_string()
        };

        ca.channel_create_channel(p).await.unwrap();
    }
}
