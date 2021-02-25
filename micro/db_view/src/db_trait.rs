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

pub trait DBUser {
    // =================== User ====================
    fn get_user_by_cid(&self, user_cid: i64) -> Result<pb::User, GenErr>;
    fn get_user_by_phone(&self, phone: &str) -> Result<pb::User, GenErr>;
    fn save_user(&self, user: &pb::User) -> Result<(), GenErr>;
    // fn delete_user(&self, user: &pb::User) -> Result<(), GenErr>;

    // =================== User Session ====================
    fn get_user_session(&self, user_cid: i64, session_id: String) -> Result<pb::Session, GenErr>;
    fn save_user_session(&self, session: &pb::Session) -> Result<(), GenErr>;
    fn delete_user_session(&self, session: &pb::Session) -> Result<(), GenErr>;
}

pub trait DBProfile {
    // =================== Profile ====================
    fn get_profile(&self, profile_cid: i64) -> Result<pb::Profile, GenErr>;
    fn save_profile(&self, profile: &pb::Profile) -> Result<(), GenErr>;

    // =================== Profile Following ====================
    fn get_profile_followings(&self, profile_cid: i64) -> Result<Vec<i64>, GenErr>;
    // todo this with channel must be in one batch transaction
    fn save_profile_following(&self, channel_cid: i64, profile_cid: i64) -> Result<(), GenErr>;

    // =================== Profile Contacts ====================
    fn get_profile_contacts(&self, profile_cid: i64) -> Result<Vec<pb::Contact>, GenErr>;
    fn save_profile_contacts(&self, profile_cid: i64, contacts: Vec<pb::Contact>) -> Result<(), GenErr>;
    fn remove_profile_contacts(&self, profile_cid: i64) -> Result<(), GenErr>;

/*    // =================== Profile Directs ====================
    fn get_profile_chat_directs(&self, profile_cid: i64) -> Result<Vec<pb::Direct>, GenErr>;
    fn get_profile_channel_directs(&self, profile_cid: i64) -> Result<Vec<pb::Direct>, GenErr>;
    fn save_profile_direct(&self, contacts: &pb::Direct) -> Result<(), GenErr>;
    fn delete_profile_directs(
        &self,
        profile_cid: i64,
        directs: Vec<pb::Direct>,
    ) -> Result<(), GenErr>;*/
}

pub trait DBChat {
    // =================== Chat ====================
    fn get_chat(&self, profile_id: i64, chat_id: i64) -> Result<pb::Chat, GenErr>;
    fn save_chat(&self, chat: &pb::Chat) -> Result<(), GenErr>;

    // =================== Chat Message ====================
    fn get_chat_message(&self, channel_id: i64, message_id: i64) -> Result<pb::Message, GenErr>;
    fn save_chat_message(&self, message: &pb::Message) -> Result<(), GenErr>;
}

pub trait DB: DBChannels + DBUser {}
// pub trait DB: DBChannels + DBProfile + DBUser + DBChat {}

/*
// Chat Impl



*/
