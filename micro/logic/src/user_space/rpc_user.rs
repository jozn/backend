use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::{pb, rpc2};

use crate::UserSpace;

#[async_trait]
impl rpc2::RPC_User_Handler2 for UserSpace {
    async fn UserRegisterUser(
        &self,
        param: pb::UserRegisterUserParam,
    ) -> Result<pb::UserRegisterUserResponse, GenErr> {
        use shared::utils::time::{get_time_now_milli, get_time_now_sec};

        // todo check hash_code parm first

        let user_cid_row = shared::my::GenCid {
            cid: 0,
            intent: "user".to_string(),
        };
        let user_cid = user_cid_row.insert(&self.mysql_pool).await?.cid;


        let profile_cid_row = shared::my::GenCid {
            cid: 0,
            intent: "channel".to_string(),
        };
        let profile_cid = profile_cid_row.insert(&self.mysql_pool).await?.cid;


        let channel_cid_row = shared::my::GenCid {
            cid: 0,
            intent: "channel".to_string(),
        };
        let channel_cid = channel_cid_row.insert(&self.mysql_pool).await?.cid;


        // channel
        let channel = pb::Channel {
            channel_cid: channel_cid,
            creator_profile_cid: profile_cid,
            is_profile_channel: true,
            created_time: get_time_now_sec(),
            user_name: "".to_string(),
            channel_title: format!("{} {}", param.first_name, param.last_name),
            about: "".to_string(),
            is_verified: false,
            sync_time_ms: get_time_now_milli(),
            is_deleted: 0,
            is_banned: 0,
            invite_link_hash: "".to_string(),
            notification_setting: None,
            privacy: 0,
            last_message: None,
            message_seq: 0,
            pinned_message: None,
            avatar: None,
            avatar_count: 0,
            inboxer: None,
            followers_count: 0,
            posts_count: 0,
            likes_count: 0,
            reshared_count: 0,
            counts: None,
        };

        // profile
        let profile = pb::Profile {
            profile_cid: profile_cid,
            user_cid: user_cid,
            created_time: get_time_now_sec(),
            primary_channel: Some(channel.clone()),
            saved_channel: None, // todo
            setting: None,
            contact_info: None,
            channels: vec![],
            groups: vec![],
            contacts: vec![],
        };

        let user = pb::User {
            user_cid: user_cid,
            phone: param.phone_number,
            email: "".to_string(),
            created_time: shared::utils::time::get_time_now_sec(),
            version_time: shared::utils::time::get_time_now_sec(),
            first_name: param.first_name,
            last_name: param.last_name,
            is_deleted: false,
            is_banned: false,
            password_hash: "".to_string(),
            password_salt: "".to_string(),
            def_profile: Some(profile.clone()),
            sessions: vec![],
            shopping_profile: None, // todo
            stores: vec![],
            profiles: vec![],
        };

        use shared::common;
        use shared::my;

        // channel db
        let buff = common::prost_encode(&channel)?;
        let channel_row = my::Channel {
            channel_cid: channel_cid as u64,
            pb_data: buff,
        };
        channel_row.replace(&self.mysql_pool).await?;

        // profile db
        let buff = common::prost_encode(&profile)?;
        let profile_row = my::Profile {
            profile_cid: profile_cid as u64,
            pb_data: buff,
        };
        profile_row.replace(&self.mysql_pool).await?;

        // user db
        let buff = common::prost_encode(&user)?;
        let user_row = my::User {
            user_cid: user_cid as u64,
            pb_data: buff,
        };
        user_row.replace(&self.mysql_pool).await?;

        Ok(pb::UserRegisterUserResponse { user: Some(user) })
    }

    async fn UserEditUser(
        &self,
        param: pb::UserEditUserParam,
    ) -> Result<pb::UserEditUserResponse, GenErr> {
        Ok(pb::UserEditUserResponse::default())
    }
    async fn UserRemoveSession(
        &self,
        param: pb::UserRemoveSessionParam,
    ) -> Result<pb::UserRemoveSessionResponse, GenErr> {
        Ok(pb::UserRemoveSessionResponse::default())
    }
    async fn UserRemoveOtherSessions(
        &self,
        param: pb::UserRemoveOtherParam,
    ) -> Result<pb::UserRemoveOtherResponse, GenErr> {
        Ok(pb::UserRemoveOtherResponse::default())
    }
    async fn UserGetMe(&self, param: pb::UserGetMeParam) -> Result<pb::UserGetMeResponse, GenErr> {
        Ok(pb::UserGetMeResponse::default())
    }
    async fn UserGetActiveSessions(
        &self,
        param: pb::UserGetActiveSessionsParam,
    ) -> Result<pb::UserGetActiveSessionsResponse, GenErr> {
        Ok(pb::UserGetActiveSessionsResponse::default())
    }
}
