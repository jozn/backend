use crate::man::act::DBMySql;
use crate::{
    act,
    errors::GenErr,
    pb,
    utils::{rand, time},
};

pub struct UserAct {
    db: DBMySql,
}

#[rustfmt::skip]
impl UserAct {

    pub async fn register_user(&self, p: param::RegisterUser) -> Result<pb::User,GenErr> {
        let user_cid = self.db.get_next_cid("user").await?;
        let profile_cid = self.db.get_next_cid("profile").await?;
        let channel_cid = self.db.get_next_cid("channel").await?;

        // channel
        let channel = pb::Channel {
            channel_cid: channel_cid as u32,
            creator_profile_cid: profile_cid as u32,
            is_profile_channel: true,
            created_time: time::get_time_now_sec(),
            channel_title: format!("{} {}", p.first_name, p.last_name),
            sync_time_ms: time::get_time_now_milli(),
            ..Default::default()
        };

        // profile
        let profile = pb::Profile {
            profile_cid: profile_cid as u32,
            user_cid: user_cid as u32,
            created_time: time::get_time_now_sec(),
            primary_channel: Some(channel.clone()),
            saved_channel: None, // todo
            setting: None,
            ..Default::default()
        };

        // user
        let user = pb::User {
            user_cid: user_cid as u32,
            phone: p.phone_number.clone(),
            created_time: time::get_time_now_sec(),
            version_time: time::get_time_now_sec(),
            first_name: p.first_name,
            last_name: p.last_name,
            def_profile: Some(profile.clone()),
            shopping_profile: None, // todo
            ..Default::default()
        };

        let r = self.db.save_channel(&channel).await?;
        let r = self.db.save_profile(&profile).await?;
        let r = self.db.save_user(&user).await?;

        Ok(user)
    }

    pub async fn edit_user(&self,user_cid: u64, p: param::EditUser) -> Result<pb::User,GenErr> {
        let mut user = self.db.get_user_by_cid(user_cid).await?;

        if p.set_new_name {
            user.first_name = p.new_first_name;
            user.last_name = p.new_last_name;
        }

        self.db.save_user(&user).await?;

        Ok(user)
    }

    pub async fn delete_user(&self, user_cid: u64) -> Result<pb::User,GenErr> {
        let mut user = self.db.get_user_by_cid(user_cid).await?;

        user.is_deleted = true;
        // todo remove things
        self.db.save_user(&user).await?;

        Ok(user)
    }

    pub async fn create_session(&self, user_cid: u64, p: param::CreateSession) -> Result<pb::Session,GenErr> {
        let mut user = self.db.get_user_by_cid(user_cid).await?;

        // Session handling
        let session_pb = pb::Session {
            session_hash: rand::get_rand_session(18),
            user_cid: user_cid as u32,
            last_ip: "".to_string(),                //todo
            user_agent: "Android v0.4".to_string(), // todo
            api_version: 0,                         // todo
            app_name: "".to_string(),
            app_version: "".to_string(),
            device_name: "".to_string(),
            active_time: time::get_time_now_sec(),
            created_time: time::get_time_now_sec(),
        };

        self.db.save_user_session(&session_pb).await?;

        Ok(session_pb)
    }

}

pub mod param {

    #[derive(Clone, Default, Debug)]
    pub struct RegisterUser {
        pub first_name: String,
        pub last_name: String,
        pub phone_number: String,
    }

    #[derive(Clone, Default, Debug)]
    pub struct EditUser {
        pub set_new_name: bool,
        pub new_first_name: String,
        pub new_last_name: String,
    }

    #[derive(Clone, Default, Debug)]
    pub struct CreateSession {}
}

// #[cfg(test)]
pub mod tests {
    use super::*;

    // #[test]
    pub async fn play1() {
        let ca = UserAct { db: DBMySql::new() };
    }
}
