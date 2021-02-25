/////////////////////////// Docs ///////////////////////////

//
//add follower, subscriber
//+Categorize fields > favour maintablaity over tags 1 byte saving
//+Only when all things implemented go for correct tag ids
//+If need to use Box > just after full imple of everythings > or just test if Box works and do not change the wire binary > after launch optimize this
//==
//Some notes:
//+ Reduced: No bloated types > just those we need to now.
//+ Seperate things that are diffrent: channles, store, directs,
//
//
//No Rpc_Account > Rpc_User + Rpc_Profile ,...
//
//next: user relations, shop, product, message log, store, file
//next2: message integrations: product, payment, contacts,...
//next3: follwing and subcripation in: channels or profiles
//gid = unique nano second time;
//sid = scoped id; for future - bot platforms
//cid = common id - seqentaily increase id for user, channels, groups, shops,..
//
//imut = immutable fields
//mut  = mutable fields

// Words: reshared/username/

////////////////////////// Enums //////////////////////////

/// todo: chat,group,channels draft: embed in types or in shared spared with one api
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleMessage {
    #[prost(oneof = "sample_message::TestOneof", tags = "4, 9, 10")]
    pub test_oneof: ::std::option::Option<sample_message::TestOneof>,
}
pub mod sample_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TestOneof {
        #[prost(string, tag = "4")]
        Name(std::string::String),
        #[prost(message, tag = "9")]
        SubMessage(super::Invoke),
        #[prost(message, tag = "10")]
        AddContact(super::Contact),
    }
}
//////////////////////////// Common Types ////////////////////
///
/// imut all
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Invoke {
    /// imut
    #[prost(uint32, tag = "6")]
    pub namespace: u32,
    /// imut
    #[prost(uint32, tag = "1")]
    pub method: u32,
    /// imut
    #[prost(uint32, tag = "7")]
    pub user_id: u32,
    /// imut
    #[prost(uint64, tag = "2")]
    pub action_id: u64,
    /// imut
    #[prost(bytes, tag = "8")]
    pub session: std::vec::Vec<u8>,
    /// imut
    #[prost(bytes, tag = "4")]
    pub rpc_data: std::vec::Vec<u8>,
}
/// imut all
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvokeResponse {
    /// imut
    #[prost(uint32, tag = "6")]
    pub namespace: u32,
    /// imut
    #[prost(uint32, tag = "1")]
    pub method: u32,
    /// imut
    #[prost(uint32, tag = "7")]
    pub user_id: u32,
    /// imut
    #[prost(uint64, tag = "2")]
    pub action_id: u64,
    /// imut
    #[prost(bytes, tag = "4")]
    pub rpc_data: std::vec::Vec<u8>,
}
///==================== User ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(fixed64, tag = "1")]
    pub contact_gid: u64,
    #[prost(uint32, tag = "2")]
    pub profile_cid: u32,
    /// +98*
    #[prost(string, tag = "4")]
    pub phone: std::string::String,
    /// In device
    #[prost(string, tag = "5")]
    pub first_name: std::string::String,
    /// In device
    #[prost(string, tag = "6")]
    pub last_name: std::string::String,
    #[prost(uint32, tag = "12")]
    pub peer_profile_cid: u32,
    #[prost(uint32, tag = "13")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// Info 0-10
    #[prost(uint32, tag = "1")]
    pub user_cid: u32,
    /// for default profile
    #[prost(string, tag = "14")]
    pub phone: std::string::String,
    ///
    #[prost(string, tag = "15")]
    pub email: std::string::String,
    #[prost(uint32, tag = "36")]
    pub created_time: u32,
    #[prost(uint32, tag = "37")]
    pub version_time: u32,
    /// Names - for Default profile build from this > set def_profile title on change
    #[prost(string, tag = "40")]
    pub first_name: std::string::String,
    #[prost(string, tag = "41")]
    pub last_name: std::string::String,
    /// Access 20
    #[prost(bool, tag = "38")]
    pub is_deleted: bool,
    #[prost(bool, tag = "39")]
    pub is_banned: bool,
    /// Passwords 30
    #[prost(string, tag = "17")]
    pub password_hash: std::string::String,
    #[prost(string, tag = "18")]
    pub password_salt: std::string::String,
    /// Profile 40
    #[prost(message, optional, tag = "1114")]
    pub def_profile: ::std::option::Option<Profile>,
    /// Session 50
    #[prost(message, repeated, tag = "102")]
    pub sessions: ::std::vec::Vec<Session>,
    // Collections
    /// Shopping 60
    #[prost(message, optional, tag = "111")]
    pub shopping_profile: ::std::option::Option<ShoppingProfile>,
    #[prost(message, repeated, tag = "113")]
    pub stores: ::std::vec::Vec<Store>,
    /// Not now 800
    #[prost(message, repeated, tag = "110")]
    pub profiles: ::std::vec::Vec<Profile>,
}
///==================== Profile ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Profile {
    /// Info 0-10
    ///
    /// imut - profile_cid
    #[prost(uint32, tag = "1")]
    pub profile_cid: u32,
    /// imut
    #[prost(uint32, tag = "2")]
    pub user_cid: u32,
    /// imut
    #[prost(uint32, tag = "103")]
    pub created_time: u32,
    /// Channel
    ///
    /// imut
    #[prost(message, optional, tag = "100")]
    pub primary_channel: ::std::option::Option<Channel>,
    /// Save Channel
    ///
    /// imut
    #[prost(message, optional, tag = "109")]
    pub saved_channel: ::std::option::Option<SavedChannel>,
    /// Settings
    ///
    /// imut
    #[prost(message, optional, tag = "107")]
    pub setting: ::std::option::Option<ProfileSettings>,
    /// Views 100
    ///
    /// When sending profile to others set this if they have this profile in their contacts
    #[prost(message, optional, tag = "111")]
    pub contact_info: ::std::option::Option<Contact>,
    /// Demonstration: 800 - collections > not really present > big > fetch with api
    ///
    /// mut
    #[prost(message, repeated, tag = "104")]
    pub channels: ::std::vec::Vec<Channel>,
    ///  repeated Direct directs = 105;// mut
    ///
    /// mut
    #[prost(message, repeated, tag = "106")]
    pub groups: ::std::vec::Vec<Group>,
    /// mut
    #[prost(message, repeated, tag = "108")]
    pub contacts: ::std::vec::Vec<Contact>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileSettings {}
///==================== Peer Chat ==================
///
///?? or embed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chat {
    /// imut
    #[prost(fixed64, tag = "7")]
    pub chat_gid: u64,
    /// imut
    #[prost(uint32, tag = "1")]
    pub profile1_cid: u32,
    /// imut
    #[prost(uint32, tag = "2")]
    pub profile2_cid: u32,
    /// imut
    #[prost(fixed64, tag = "3")]
    pub direct1_gid: u64,
    /// imut
    #[prost(fixed64, tag = "4")]
    pub direct2_gid: u64,
    /// Views
    ///
    /// ? must use profile
    #[prost(message, optional, tag = "49")]
    pub contact: ::std::option::Option<Contact>,
    /// Profile > or Peer Chat ?
    #[prost(message, optional, tag = "149")]
    pub profile: ::std::option::Option<Profile>,
    /// Messages
    ///
    /// mut
    #[prost(message, optional, tag = "25")]
    pub last_message: ::std::option::Option<Message>,
    /// mut
    #[prost(message, optional, tag = "26")]
    pub pinned_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag = "700")]
    pub inboxer: ::std::option::Option<Inboxer>,
}
///?? or embed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDep {
    /// imut
    #[prost(fixed64, tag = "7")]
    pub chat_gid: u64,
    /// imut
    #[prost(uint32, tag = "1")]
    pub profile1_cid: u32,
    /// imut
    #[prost(uint32, tag = "2")]
    pub profile2_cid: u32,
    /// imut
    #[prost(fixed64, tag = "3")]
    pub direct1_gid: u64,
    /// imut
    #[prost(fixed64, tag = "4")]
    pub direct2_gid: u64,
}
///==================== Messages ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// 1-10 Info
    ///
    /// imut
    #[prost(fixed64, tag = "1")]
    pub message_gid: u64,
    /// imut
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
    /// imut - mostly
    #[prost(enumeration = "MessageType", tag = "107")]
    pub message_type: i32,
    #[prost(string, tag = "7")]
    pub text: std::string::String,
    /// imut > Including bots 1: Android app, 2... to 1000 reserved - bots >1000
    #[prost(uint32, tag = "12")]
    pub via_app_id: u32,
    /// imut
    #[prost(uint32, tag = "13")]
    pub seq: u32,
    /// imut
    #[prost(uint32, tag = "18")]
    pub created_time: u32,
    /// Edit
    ///
    /// mut
    #[prost(uint32, tag = "17")]
    pub edited_time: u32,
    /// Sync info
    #[prost(enumeration = "MessageDeliveryStatues", tag = "105")]
    pub delivery_status: i32,
    /// mut-once
    #[prost(uint32, tag = "106")]
    pub delivery_time: u32,
    /// Visibility
    ///
    /// imut - verified system messages - 2 bytes tag
    #[prost(bool, tag = "111")]
    pub verified: bool,
    /// maybe temp with a purge period - 2 bytes
    #[prost(bool, tag = "150")]
    pub deleted: bool,
    /// Header: Forward/Reply
    ///
    /// forward is always live object from other channels, but not from other chats, groups
    #[prost(message, optional, boxed, tag = "16")]
    pub forward: ::std::option::Option<::std::boxed::Box<Message>>,
    #[prost(message, optional, boxed, tag = "50")]
    pub reply_to: ::std::option::Option<::std::boxed::Box<Message>>,
    /// Channels Settings
    #[prost(uint32, tag = "1001")]
    pub channel_cid: u32,
    #[prost(message, optional, tag = "102")]
    pub setting: ::std::option::Option<MessageSetting>,
    /// Channels Extra
    #[prost(message, optional, tag = "101")]
    pub counts: ::std::option::Option<MessageCount>,
    /// Group
    #[prost(uint32, tag = "1002")]
    pub group_cid: u32,
    /// Media
    #[prost(message, repeated, tag = "103")]
    pub files: ::std::vec::Vec<FileMsg>,
    /// Stores
    #[prost(message, optional, tag = "110")]
    pub product: ::std::option::Option<Product>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageCount {
    ///???
    #[prost(fixed64, tag = "1")]
    pub message_gid: u64,
    #[prost(uint32, tag = "2")]
    pub comments_count: u32,
    #[prost(uint32, tag = "3")]
    pub likes_count: u32,
    #[prost(int64, tag = "4")]
    pub views_count: i64,
    #[prost(uint32, tag = "5")]
    pub reshared_count: u32,
    #[prost(uint32, tag = "6")]
    pub chat_shared_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageSetting {
    #[prost(uint32, tag = "11")]
    pub disable_comment: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageLog {
    #[prost(enumeration = "MessageLogType", tag = "10")]
    pub log_type: i32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
    #[prost(uint32, tag = "3")]
    pub target_profile_cid: u32,
    #[prost(message, optional, tag = "11")]
    pub target_profile_view: ::std::option::Option<Profile>,
}
///==================== Channel ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    /// Info
    ///
    /// imut
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    /// imut
    #[prost(uint32, tag = "7")]
    pub creator_profile_cid: u32,
    /// imut
    #[prost(bool, tag = "101")]
    pub is_profile_channel: bool,
    /// imut
    #[prost(uint32, tag = "36")]
    pub created_time: u32,
    /// Visibility
    ///
    /// mut
    #[prost(string, tag = "2")]
    pub user_name: std::string::String,
    /// mut - For default channels take if from User first and lastname.
    #[prost(string, tag = "3")]
    pub channel_title: std::string::String,
    /// mut
    #[prost(string, tag = "16")]
    pub about: std::string::String,
    /// mut
    #[prost(bool, tag = "6")]
    pub is_verified: bool,
    /// Sync
    ///
    /// mut - version
    #[prost(fixed64, tag = "21")]
    pub sync_time_ms: u64,
    /// Access
    ///
    /// mut
    #[prost(uint32, tag = "38")]
    pub is_deleted: u32,
    /// mut
    #[prost(uint32, tag = "39")]
    pub is_banned: u32,
    /// Settings
    ///
    /// mut
    #[prost(string, tag = "17")]
    pub invite_link_hash: std::string::String,
    /// for owner
    #[prost(message, optional, tag = "90")]
    pub notification_setting: ::std::option::Option<ChannelOwnerNotification>,
    /// mut
    #[prost(enumeration = "ChannelPrivacy", tag = "9")]
    pub privacy: i32,
    /// Messages
    ///
    /// mut
    #[prost(message, optional, tag = "25")]
    pub last_message: ::std::option::Option<Message>,
    /// mut
    #[prost(uint32, tag = "19")]
    pub message_seq: u32,
    /// Pin
    ///
    /// mut
    #[prost(message, optional, tag = "26")]
    pub pinned_message: ::std::option::Option<Message>,
    /// Avatar
    ///
    /// mut
    #[prost(message, optional, tag = "100")]
    pub avatar: ::std::option::Option<FileMsg>,
    /// mut
    #[prost(int64, tag = "40")]
    pub avatar_count: i64,
    #[prost(message, optional, tag = "700")]
    pub inboxer: ::std::option::Option<Inboxer>,
    /// Counts -> followers_count in profile
    #[prost(uint32, tag = "20")]
    pub followers_count: u32,
    #[prost(uint32, tag = "22")]
    pub posts_count: u32,
    #[prost(uint32, tag = "33")]
    pub likes_count: u32,
    #[prost(uint32, tag = "34")]
    pub reshared_count: u32,
    /// mut
    #[prost(message, optional, tag = "44")]
    pub counts: ::std::option::Option<MediaCounts>,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOwnerNotification {}
///==================== Store ==================
///
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShoppingProfile {}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Store {
    #[prost(uint32, tag = "1")]
    pub cid: u32,
    #[prost(string, tag = "2")]
    pub user_name: std::string::String,
    /// For default channels take if from User.
    #[prost(string, tag = "3")]
    pub store_name: std::string::String,
    #[prost(string, tag = "102")]
    pub address: std::string::String,
    #[prost(uint32, tag = "7")]
    pub creator_user_cid: u32,
    #[prost(string, tag = "16")]
    pub about: std::string::String,
    #[prost(uint32, tag = "19")]
    pub message_seq: u32,
    /// version
    #[prost(fixed64, tag = "21")]
    pub sync_time_ms: u64,
    #[prost(uint32, tag = "36")]
    pub created_time: u32,
    #[prost(uint32, tag = "39")]
    pub is_banned: u32,
    #[prost(bool, tag = "6")]
    pub is_verified: bool,
    #[prost(message, optional, tag = "100")]
    pub avatar: ::std::option::Option<FileMsg>,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    /// seq
    #[prost(uint32, tag = "1")]
    pub product_id: u32,
    #[prost(uint32, tag = "53")]
    pub category_id: u32,
    #[prost(string, tag = "50")]
    pub category: std::string::String,
    #[prost(string, tag = "51")]
    pub brand: std::string::String,
    ///from 10_000 - 5% > 500
    #[prost(uint32, tag = "3")]
    pub fee_rate: u32,
    #[prost(uint32, tag = "5")]
    pub sales_count: u32,
    #[prost(uint32, tag = "6")]
    pub price: u32,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductPriceInfo {
    #[prost(uint32, tag = "1")]
    pub price: u32,
    #[prost(uint32, tag = "6")]
    pub discount_price: u32,
    /// from 1000
    #[prost(uint32, tag = "3")]
    pub commission_rate: u32,
}
///==================== Inboxer ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inboxer {
    /// Info 1-10
    ///
    /// imut  > del
    #[prost(fixed64, tag = "1")]
    pub inboxer_gid: u64,
    /// imutt > del
    #[prost(uint32, tag = "5")]
    pub profile_cid: u32,
    /// Meta info (sync) - mut
    ///
    /// mut
    #[prost(uint32, tag = "12")]
    pub unseen_count: u32,
    /// mut
    #[prost(fixed64, tag = "45")]
    pub sort_time_ms: u64,
    /// mut
    #[prost(fixed64, tag = "104")]
    pub sync_time_ms: u64,
    /// mut
    #[prost(fixed64, tag = "16")]
    pub my_last_seen_seq: u64,
    /// mut
    #[prost(fixed64, tag = "17")]
    pub my_last_seen_msg_id: u64,
    /// Pin
    ///
    /// mut
    #[prost(fixed64, tag = "10")]
    pub pin_time_ms: u64,
}
///==================== Saved ==================
///
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedChannel {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(uint32, tag = "7")]
    pub creator_profile_cid: u32,
    #[prost(uint32, tag = "19")]
    pub message_seq: u32,
    /// version
    #[prost(fixed64, tag = "21")]
    pub sync_time_ms: u64,
    #[prost(uint32, tag = "36")]
    pub created_time: u32,
    #[prost(message, optional, tag = "41")]
    pub counts: ::std::option::Option<ChannelCountsDep>,
    #[prost(message, optional, tag = "25")]
    pub last_message: ::std::option::Option<Message>,
}
///==================== Group ==================
///
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// Info
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
    #[prost(uint32, tag = "7")]
    pub creator_profile_cid: u32,
    #[prost(uint32, tag = "21")]
    pub created_time: u32,
    /// Visibility
    ///
    /// or _name
    #[prost(string, tag = "3")]
    pub group_title: std::string::String,
    /// with "group" suffix > not now > all is private now
    #[prost(string, tag = "4")]
    pub user_name: std::string::String,
    #[prost(string, tag = "15")]
    pub about: std::string::String,
    /// Sync
    #[prost(uint32, tag = "10")]
    pub seq: u32,
    #[prost(fixed64, tag = "20")]
    pub sort_time: u64,
    #[prost(fixed64, tag = "40")]
    pub sync_time: u64,
    /// Access
    #[prost(bool, tag = "23")]
    pub is_deleted: bool,
    #[prost(bool, tag = "24")]
    pub is_banned: bool,
    /// Setting
    #[prost(bool, tag = "8")]
    pub history_viewable: bool,
    /// global searchable, view history without joining
    #[prost(bool, tag = "9")]
    pub is_open_group: bool,
    #[prost(string, tag = "16")]
    pub invite_link_hash: std::string::String,
    /// Messages
    #[prost(message, optional, tag = "25")]
    pub last_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag = "26")]
    pub pinned_message: ::std::option::Option<Message>,
    // Pin
    /// Avatar
    #[prost(uint32, tag = "14")]
    pub avatar_count: u32,
    #[prost(message, optional, tag = "27")]
    pub avatar: ::std::option::Option<FileMsg>,
    /// Counts
    #[prost(uint32, tag = "17")]
    pub members_count: u32,
    #[prost(uint32, tag = "18")]
    pub admins_count: u32,
    #[prost(uint32, tag = "19")]
    pub moderator_counts: u32,
    #[prost(message, optional, tag = "200")]
    pub media_counts: ::std::option::Option<MediaCounts>,
    /// Member
    ///
    /// s_imut
    #[prost(message, optional, tag = "43")]
    pub group_member: ::std::option::Option<GroupMember>,
    /// imut
    #[prost(fixed64, tag = "11")]
    pub visible_from_msg_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMember {
    /// Info
    #[prost(int64, tag = "1")]
    pub member_gid: i64,
    #[prost(int64, tag = "2")]
    pub group_cid: i64,
    #[prost(uint32, tag = "3")]
    pub profile_cid: u32,
    #[prost(uint32, tag = "4")]
    pub by_profile_cid: u32,
    #[prost(uint32, tag = "6")]
    pub created_time: u32,
}
///==================== Others ==================
///
/// Shared amoung chat, groups, channel
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaCounts {
    #[prost(uint32, tag = "23")]
    pub media_count: u32,
    #[prost(uint32, tag = "24")]
    pub photo_count: u32,
    #[prost(uint32, tag = "25")]
    pub video_count: u32,
    #[prost(uint32, tag = "26")]
    pub gif_count: u32,
    #[prost(uint32, tag = "27")]
    pub audio_count: u32,
    #[prost(uint32, tag = "28")]
    pub voice_count: u32,
    #[prost(uint32, tag = "29")]
    pub file_count: u32,
    #[prost(uint32, tag = "30")]
    pub link_count: u32,
    #[prost(uint32, tag = "32")]
    pub pined_count: u32,
}
///todo file, image, video, audio seperatin
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Media {}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileMsg {
    #[prost(fixed64, tag = "1")]
    pub file_gid: u64,
    #[prost(uint32, tag = "2")]
    pub access_hash: u32,
    #[prost(uint32, tag = "3")]
    pub file_type: u32,
    #[prost(uint32, tag = "4")]
    pub width: u32,
    #[prost(uint32, tag = "5")]
    pub height: u32,
    #[prost(string, tag = "6")]
    pub extension: std::string::String,
    #[prost(string, tag = "61")]
    pub full_path: std::string::String,
    #[prost(uint32, tag = "7")]
    pub user_cid: u32,
    #[prost(bytes, tag = "8")]
    pub data_thumb: std::vec::Vec<u8>,
    #[prost(bytes, tag = "9")]
    pub data: std::vec::Vec<u8>,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
    #[prost(fixed64, tag = "1")]
    pub gid: u64,
    #[prost(string, tag = "2")]
    pub session_uuid: std::string::String,
    #[prost(uint64, tag = "100")]
    pub device_id: u64,
    #[prost(uint32, tag = "3")]
    pub user_cid: u32,
    #[prost(string, tag = "4")]
    pub last_ip_address: std::string::String,
    #[prost(string, tag = "8")]
    pub user_agent: std::string::String,
    #[prost(enumeration = "DevicePlatform", tag = "9")]
    pub platform: i32,
    #[prost(uint32, tag = "5")]
    pub api_version: u32,
    #[prost(uint32, tag = "6")]
    pub active_time: u32,
    #[prost(uint32, tag = "7")]
    pub created_time: u32,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sms {
    #[prost(fixed64, tag = "1")]
    pub gid: u64,
    #[prost(string, tag = "3")]
    pub install_uuid: std::string::String,
    #[prost(string, tag = "4")]
    pub to_phone: std::string::String,
    #[prost(bool, tag = "103")]
    pub for_confirm: bool,
    #[prost(string, tag = "5")]
    pub confirm_code: std::string::String,
    #[prost(string, tag = "6")]
    pub gateway_number: std::string::String,
    #[prost(string, tag = "101")]
    pub text_body: std::string::String,
    #[prost(uint32, tag = "100")]
    pub created_time: u32,
    /// Below are for easier debugging purpose > stringy
    ///
    /// some custom debug info: http header, code, body, ...
    #[prost(string, tag = "9")]
    pub gateway_error: std::string::String,
    /// like "register" "login" "delete" "marketing" ,...
    #[prost(string, tag = "14")]
    pub intent: std::string::String,
    /// like "confirmed" "unconfirmed" "sending" "gateway_error"
    #[prost(string, tag = "102")]
    pub result: std::string::String,
}
///==================== New > move ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    /// imut
    #[prost(uint64, tag = "6")]
    pub comment_gid: u64,
    /// imut
    #[prost(uint64, tag = "1")]
    pub message_gid: u64,
    /// imut
    #[prost(uint32, tag = "2")]
    pub profile_cid: u32,
    /// imut
    #[prost(uint32, tag = "3")]
    pub created_time: u32,
    /// mut
    #[prost(string, tag = "4")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Follower {
    /// imut
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    /// imut
    #[prost(uint32, tag = "2")]
    pub profile_cid: u32,
    /// imut
    #[prost(uint32, tag = "3")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscribe {
    /// imut
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    /// imut
    #[prost(uint32, tag = "2")]
    pub profile_cid: u32,
    /// imut
    #[prost(uint32, tag = "3")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Like {
    /// imut
    #[prost(uint64, tag = "1")]
    pub message_gid: u64,
    /// imut
    #[prost(uint32, tag = "2")]
    pub profile_cid: u32,
    /// imut
    #[prost(uint32, tag = "3")]
    pub created_time: u32,
}
//==================== Views ==================

/// C: Common
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewMessageInput {
    /// imut
    #[prost(fixed64, tag = "1")]
    pub message_gid: u64,
    /// imut
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
    /// imut - mostly
    #[prost(enumeration = "MessageType", tag = "107")]
    pub message_type: i32,
    #[prost(string, tag = "7")]
    pub text: std::string::String,
    /// imut
    #[prost(uint32, tag = "12")]
    pub via_app_id: u32,
    /// imut
    #[prost(uint32, tag = "13")]
    pub seq: u32,
    /// imut
    #[prost(uint32, tag = "18")]
    pub created_time: u32,
    /// imut - verified system messages - 2 bytes tag
    #[prost(bool, tag = "111")]
    pub verified: bool,
}
///==================== Deprecated =============
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCountsDep {
    #[prost(uint32, tag = "20")]
    pub followers_count: u32,
    ///uint32 following_count = 21; // ?? just profile
    #[prost(uint32, tag = "22")]
    pub posts_count: u32,
    #[prost(uint32, tag = "23")]
    pub media_count: u32,
    #[prost(uint32, tag = "24")]
    pub photo_count: u32,
    #[prost(uint32, tag = "25")]
    pub video_count: u32,
    #[prost(uint32, tag = "26")]
    pub gif_count: u32,
    #[prost(uint32, tag = "27")]
    pub audio_count: u32,
    #[prost(uint32, tag = "28")]
    pub voice_count: u32,
    #[prost(uint32, tag = "29")]
    pub file_count: u32,
    #[prost(uint32, tag = "30")]
    pub link_count: u32,
    #[prost(uint32, tag = "31")]
    pub board_count: u32,
    #[prost(uint32, tag = "32")]
    pub pined_count: u32,
    #[prost(uint32, tag = "33")]
    pub likes_count: u32,
    #[prost(uint32, tag = "34")]
    pub reshared_count: u32,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCountsDep {
    #[prost(uint32, tag = "23")]
    pub media_count: u32,
    #[prost(uint32, tag = "24")]
    pub photo_count: u32,
    #[prost(uint32, tag = "25")]
    pub video_count: u32,
    #[prost(uint32, tag = "26")]
    pub gif_count: u32,
    #[prost(uint32, tag = "27")]
    pub audio_count: u32,
    #[prost(uint32, tag = "28")]
    pub voice_count: u32,
    #[prost(uint32, tag = "29")]
    pub file_count: u32,
    #[prost(uint32, tag = "30")]
    pub link_count: u32,
    #[prost(uint32, tag = "32")]
    pub pined_count: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProfileLevelEnum {
    LevelNormal = 0,
    AppAdmin = 1,
    Suspended = 2,
    DeletedByOwner = 3,
    DeletedIran = 4,
    SuspendedIran = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageType {
    Text = 0,
    Image = 1,
    Video = 3,
    Audio = 5,
    Voice = 7,
    Gif = 8,
    File = 9,
    Poll = 10,
    Location = 11,
    Log = 12,
    Contact = 13,
    Wallet = 15,
    /// ??
    Product = 16,
    /// Add multi
    Forward = 17,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageDeliveryStatues {
    UnknownMd = 0,
    Sending = 1,
    /// Sent to server
    Sent = 2,
    Delivered = 3,
    Seen = 4,
    /// listened, download, watched
    Consumed = 5,
    /// blocked, restricted,
    Failed = 6,
}
/// Just for: groups and peer chats
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageLogType {
    MltUnknown = 0,
    /// Just in chat
    UserSingedUp = 1,
    GroupCreated = 2,
    MemberAdded = 3,
    MemberKicked = 4,
    MemberLeft = 5,
    MemberJoinedByLink = 8,
    RoomDeleted = 9,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelPrivacy {
    ChannelUnknownAb = 0,
    ChannelOpen = 1,
    /// Just for none-default profile channels
    ChannelPrivateLink = 2,
    ChannelAccept = 3,
    /// For save
    ChannelCreator = 4,
}
//==================== Utils ==================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DevicePlatform {
    UnknownPlatform = 0,
    Android = 1,
    Ios = 2,
    Windows = 3,
    MacOs = 4,
    Linux = 5,
    Web = 7,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCommand {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(
        oneof = "channel_command::SubCommand",
        tags = "50, 51, 52, 30, 31, 40, 41, 10, 11, 12, 200, 201, 300, 301, 400, 401, 80, 81"
    )]
    pub sub_command: ::std::option::Option<channel_command::SubCommand>,
}
pub mod channel_command {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QCreateChannel {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(uint32, tag = "2")]
        pub creator_profile_cid: u32,
        #[prost(string, tag = "3")]
        pub channel_title: std::string::String,
        #[prost(string, tag = "4")]
        pub user_name: std::string::String,
        #[prost(string, tag = "15")]
        pub about: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QEditChannel {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(uint32, tag = "2")]
        pub by_profile_cid: u32,
        #[prost(bool, tag = "3")]
        pub set_new_title: bool,
        #[prost(string, tag = "4")]
        pub new_title: std::string::String,
        #[prost(bool, tag = "5")]
        pub set_new_about: bool,
        #[prost(string, tag = "6")]
        pub new_about: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QDeleteChannel {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(uint32, tag = "2")]
        pub by_profile_cid: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QFollowChannel {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(uint32, tag = "2")]
        pub by_profile_cid: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QUnFollowChannel {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(uint32, tag = "2")]
        pub by_profile_cid: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QSubscribe {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(uint32, tag = "2")]
        pub by_profile_cid: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QUnSubscribe {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(uint32, tag = "2")]
        pub by_profile_cid: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QSendMessage {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(message, optional, tag = "2")]
        pub message_input: ::std::option::Option<super::NewMessageInput>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QEditMessage {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(fixed64, tag = "4")]
        pub message_gid: u64,
        #[prost(uint32, tag = "3")]
        pub by_profile_cid: u32,
        #[prost(string, tag = "2")]
        pub new_text: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QDeleteMessages {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(uint32, tag = "2")]
        pub by_profile_cid: u32,
        #[prost(uint64, repeated, tag = "3")]
        pub message_gids: ::std::vec::Vec<u64>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QLikeMessage {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(fixed64, tag = "2")]
        pub message_gid: u64,
        #[prost(uint32, tag = "3")]
        pub by_profile_cid: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QUnLikeMessage {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(fixed64, tag = "2")]
        pub message_gid: u64,
        #[prost(uint32, tag = "3")]
        pub by_profile_cid: u32,
        #[prost(fixed64, tag = "4")]
        pub like_gid: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QReShareMessage {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(fixed64, tag = "2")]
        pub message_gid: u64,
        #[prost(uint32, tag = "3")]
        pub by_profile_cid: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QUnReShareMessage {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(fixed64, tag = "2")]
        pub message_gid: u64,
        #[prost(uint32, tag = "3")]
        pub by_profile_cid: u32,
        #[prost(fixed64, tag = "4")]
        pub re_share_gid: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QAddComment {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(fixed64, tag = "2")]
        pub message_gid: u64,
        #[prost(uint32, tag = "3")]
        pub by_profile_cid: u32,
        #[prost(string, tag = "4")]
        pub comment_text: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QDeleteComment {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(fixed64, tag = "2")]
        pub message_gid: u64,
        #[prost(uint32, tag = "3")]
        pub by_profile_cid: u32,
        #[prost(fixed64, tag = "4")]
        pub comment_gid: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QAvatarAdd {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
        #[prost(uint64, tag = "2")]
        pub file_id: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QAvatarDelete {
        #[prost(uint32, tag = "1")]
        pub channel_cid: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SubCommand {
        #[prost(message, tag = "50")]
        CreateChannel(QCreateChannel),
        #[prost(message, tag = "51")]
        EditChannel(QEditChannel),
        #[prost(message, tag = "52")]
        DeleteChannel(QDeleteChannel),
        #[prost(message, tag = "30")]
        FollowChannel(QFollowChannel),
        #[prost(message, tag = "31")]
        UnFollowChannel(QUnFollowChannel),
        #[prost(message, tag = "40")]
        Subscribe(QSubscribe),
        #[prost(message, tag = "41")]
        UnSubscribe(QUnSubscribe),
        #[prost(message, tag = "10")]
        SendMessage(QSendMessage),
        #[prost(message, tag = "11")]
        EditMessage(QEditMessage),
        #[prost(message, tag = "12")]
        DeleteMessages(QDeleteMessages),
        #[prost(message, tag = "200")]
        LikeMessage(QLikeMessage),
        #[prost(message, tag = "201")]
        UnLikeMessage(QUnLikeMessage),
        #[prost(message, tag = "300")]
        ReShareMessage(QReShareMessage),
        #[prost(message, tag = "301")]
        UnReShareMessage(QUnReShareMessage),
        #[prost(message, tag = "400")]
        AddComment(QAddComment),
        #[prost(message, tag = "401")]
        DeleteComment(QDeleteComment),
        #[prost(message, tag = "80")]
        AvatarAdd(QAvatarAdd),
        #[prost(message, tag = "81")]
        AvatarDelete(QAvatarDelete),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatCommand {
    #[prost(oneof = "chat_command::SubCommand", tags = "50, 10, 11, 12, 13")]
    pub sub_command: ::std::option::Option<chat_command::SubCommand>,
}
pub mod chat_command {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QDeleteChat {}
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QSendMessage {
        #[prost(uint32, tag = "1")]
        pub group_id: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QEditMessage {
        #[prost(uint32, tag = "1")]
        pub group_id: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QDeleteMessages {
        #[prost(uint32, tag = "1")]
        pub group_id: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QDeleteHistory {}
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SubCommand {
        #[prost(message, tag = "50")]
        DeleteChat(QDeleteChat),
        #[prost(message, tag = "10")]
        SendMessage(QSendMessage),
        #[prost(message, tag = "11")]
        EditMessage(QEditMessage),
        #[prost(message, tag = "12")]
        DeleteMessages(QDeleteMessages),
        #[prost(message, tag = "13")]
        DeleteHistory(QDeleteHistory),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCommand {
    #[prost(
        oneof = "group_command::SubCommand",
        tags = "40, 41, 42, 50, 51, 52, 10, 11, 12, 13, 600, 601, 80, 81, 82"
    )]
    pub sub_command: ::std::option::Option<group_command::SubCommand>,
}
pub mod group_command {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QCreateGroup {
        #[prost(uint32, tag = "1")]
        pub group_cid: u32,
        #[prost(uint32, tag = "2")]
        pub creator_profile_cid: u32,
        #[prost(string, tag = "3")]
        pub group_title: std::string::String,
        #[prost(bool, tag = "8")]
        pub history_viewable: bool,
        #[prost(bool, tag = "9")]
        pub force_join: bool,
        #[prost(bool, tag = "17")]
        pub global_search: bool,
        #[prost(string, tag = "15")]
        pub about: std::string::String,
        #[prost(string, tag = "4")]
        pub user_name: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QEditGroup {
        #[prost(uint32, tag = "1")]
        pub group_cid: u32,
        #[prost(uint32, tag = "2")]
        pub by_profile_cid: u32,
        #[prost(bool, tag = "3")]
        pub set_new_title: bool,
        #[prost(string, tag = "4")]
        pub new_title: std::string::String,
        #[prost(bool, tag = "5")]
        pub set_new_about: bool,
        #[prost(string, tag = "6")]
        pub new_about: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QDeleteGroup {
        #[prost(uint32, tag = "1")]
        pub group_cid: u32,
        #[prost(uint32, tag = "2")]
        pub by_profile_cid: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QJoinGroup {
        #[prost(uint32, tag = "1")]
        pub group_cid: u32,
        #[prost(uint32, tag = "3")]
        pub new_member_profile_cid: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QLeaveGroup {
        #[prost(uint32, tag = "1")]
        pub group_cid: u32,
        #[prost(uint32, tag = "3")]
        pub member_profile_cid: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QAddMember {
        #[prost(uint32, tag = "1")]
        pub group_cid: u32,
        #[prost(uint32, tag = "2")]
        pub by_profile_cid: u32,
        #[prost(uint32, tag = "3")]
        pub new_member_profile_cid: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QSendMessage {
        #[prost(uint32, tag = "1")]
        pub group_id: u32,
        #[prost(message, optional, tag = "2")]
        pub message_input: ::std::option::Option<super::NewMessageInput>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QEditMessage {
        #[prost(uint32, tag = "1")]
        pub group_id: u32,
        #[prost(uint32, tag = "3")]
        pub by_profile_cid: u32,
        #[prost(string, tag = "2")]
        pub new_text: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QDeleteMessages {
        #[prost(uint32, tag = "1")]
        pub group_cid: u32,
        #[prost(uint32, tag = "2")]
        pub by_profile_cid: u32,
        #[prost(uint64, repeated, tag = "3")]
        pub message_ids: ::std::vec::Vec<u64>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QDeleteHistory {}
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QPinMessage {
        #[prost(uint32, tag = "1")]
        pub group_id: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QUnPinMessage {
        #[prost(uint32, tag = "1")]
        pub group_id: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QAvatarAdd {
        #[prost(uint32, tag = "1")]
        pub group_id: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QAvatarDelete {
        #[prost(uint32, tag = "1")]
        pub group_id: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QReportGroup {}
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SubCommand {
        #[prost(message, tag = "40")]
        CreateGroup(QCreateGroup),
        #[prost(message, tag = "41")]
        EditGroup(QEditGroup),
        #[prost(message, tag = "42")]
        DeleteGroup(QDeleteGroup),
        #[prost(message, tag = "50")]
        JoinGroup(QJoinGroup),
        #[prost(message, tag = "51")]
        LeaveGroup(QLeaveGroup),
        #[prost(message, tag = "52")]
        AddMember(QAddMember),
        #[prost(message, tag = "10")]
        SendMessage(QSendMessage),
        #[prost(message, tag = "11")]
        EditMessage(QEditMessage),
        #[prost(message, tag = "12")]
        DeleteMessages(QDeleteMessages),
        #[prost(message, tag = "13")]
        DeleteHistory(QDeleteHistory),
        #[prost(message, tag = "600")]
        PinMessage(QPinMessage),
        #[prost(message, tag = "601")]
        UnPinMessage(QUnPinMessage),
        #[prost(message, tag = "80")]
        AvatarAdd(QAvatarAdd),
        #[prost(message, tag = "81")]
        AvatarDelete(QAvatarDelete),
        #[prost(message, tag = "82")]
        ReportGroup(QReportGroup),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileCommand {
    #[prost(oneof = "profile_command::SubCommand", tags = "10")]
    pub sub_command: ::std::option::Option<profile_command::SubCommand>,
}
pub mod profile_command {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QSetSettings {}
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SubCommand {
        #[prost(message, tag = "10")]
        SetSettings(QSetSettings),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCommand {
    #[prost(
        oneof = "user_command::SubCommand",
        tags = "100, 101, 800, 801, 802, 300, 400, 401"
    )]
    pub sub_command: ::std::option::Option<user_command::SubCommand>,
}
pub mod user_command {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QRegisterUser {
        #[prost(uint32, tag = "1")]
        pub user_cid: u32,
        #[prost(string, tag = "2")]
        pub first_name: std::string::String,
        #[prost(string, tag = "3")]
        pub last_name: std::string::String,
        #[prost(string, tag = "15")]
        pub phone: std::string::String,
        #[prost(uint32, tag = "5")]
        pub created_time: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QEditUser {
        #[prost(uint32, tag = "1")]
        pub user_cid: u32,
        #[prost(bool, tag = "3")]
        pub set_new_name: bool,
        #[prost(string, tag = "4")]
        pub new_first_name: std::string::String,
        #[prost(string, tag = "6")]
        pub new_last_name: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QDeleteSendCode {
        #[prost(bool, tag = "1")]
        pub resend: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QDeleteConfirmCode {
        #[prost(string, tag = "3")]
        pub code: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QDeleteUser {}
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QChangePhoneNumber {}
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QRemoveSession {}
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QRemoveOtherSessions {}
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SubCommand {
        #[prost(message, tag = "100")]
        RegisterUser(QRegisterUser),
        #[prost(message, tag = "101")]
        EditUser(QEditUser),
        #[prost(message, tag = "800")]
        DeleteSendCode(QDeleteSendCode),
        #[prost(message, tag = "801")]
        DeleteConfirmCode(QDeleteConfirmCode),
        #[prost(message, tag = "802")]
        DeleteUser(QDeleteUser),
        #[prost(message, tag = "300")]
        ChangePhoneNumber(QChangePhoneNumber),
        #[prost(message, tag = "400")]
        RemoveSession(QRemoveSession),
        #[prost(message, tag = "401")]
        RemoveOtherSessions(QRemoveOtherSessions),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCommand {
    #[prost(fixed64, tag = "1")]
    pub event_id: u64,
    #[prost(uint32, tag = "2")]
    pub user_id: u32,
    #[prost(oneof = "event_command::Command", tags = "17, 7, 9, 5, 6")]
    pub command: ::std::option::Option<event_command::Command>,
}
pub mod event_command {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        #[prost(message, tag = "17")]
        User(super::UserCommand),
        #[prost(message, tag = "7")]
        Profile(super::ProfileCommand),
        ///DirectCommand direct = 8;
        #[prost(message, tag = "9")]
        Chat(super::ChatCommand),
        #[prost(message, tag = "5")]
        Channel(super::ChannelCommand),
        #[prost(message, tag = "6")]
        Group(super::GroupCommand),
    }
}
/// C: Common
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CNewMessageInput {
    /// imut
    #[prost(fixed64, tag = "1")]
    pub gid: u64,
    /// imut
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
    /// imut - mostly
    #[prost(enumeration = "MessageType", tag = "107")]
    pub message_type: i32,
    #[prost(string, tag = "7")]
    pub text: std::string::String,
    /// imut
    #[prost(uint32, tag = "12")]
    pub via_app_id: u32,
    /// imut
    #[prost(uint32, tag = "13")]
    pub seq: u32,
    ///  uint32 edited_time = 17;
    ///
    /// imut
    #[prost(uint32, tag = "18")]
    pub created_time: u32,
    /// imut - verified system messages - 2 bytes tag
    #[prost(bool, tag = "111")]
    pub verified: bool,
    ///  Message forward = 16; // forward is always live object from other channels, but not from other chats, groups
    ///  Message reply_to = 50;
    #[prost(uint32, tag = "112")]
    pub flags: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNextIdParam {
    #[prost(string, tag = "1")]
    pub key: std::string::String,
    #[prost(int32, tag = "2")]
    pub start_from: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNextIdResponse {
    #[prost(uint64, tag = "1")]
    pub next_id: u64,
    #[prost(bool, tag = "2")]
    pub error: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthSendConfirmCodeParam {
    #[prost(string, tag = "1")]
    pub hash: std::string::String,
    /// 98... 989015132328
    #[prost(string, tag = "2")]
    pub phone: std::string::String,
    #[prost(string, tag = "3")]
    pub country_code: std::string::String,
    #[prost(bool, tag = "4")]
    pub resend: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthSendConfirmCodeResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    #[prost(string, tag = "2")]
    pub error_message: std::string::String,
    #[prost(bool, tag = "3")]
    pub just_email_register: bool,
    #[prost(string, repeated, tag = "4")]
    pub sms_numbers: ::std::vec::Vec<std::string::String>,
    #[prost(bool, tag = "5")]
    pub is_login: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthConfirmCodeParam {
    #[prost(string, tag = "1")]
    pub hash: std::string::String,
    /// 98... 989015132328
    #[prost(string, tag = "2")]
    pub phone: std::string::String,
    ///4 digit 4215
    #[prost(int32, tag = "3")]
    pub code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthConfirmCodeResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    ///    SelfUserView SelfUserView = 3; //if it is login
    #[prost(string, tag = "2")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthSingUpParam {
    #[prost(string, tag = "1")]
    pub hash: std::string::String,
    #[prost(string, tag = "2")]
    pub first_name: std::string::String,
    #[prost(string, tag = "3")]
    pub last_name: std::string::String,
    #[prost(string, tag = "4")]
    pub user_name: std::string::String,
    #[prost(string, tag = "5")]
    pub phone: std::string::String,
    #[prost(string, tag = "6")]
    pub email: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthSingUpResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    ///    SelfUserView SelfUserView = 3;
    #[prost(string, tag = "2")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthSingInParam {
    #[prost(string, tag = "4")]
    pub user_name_phone_email: std::string::String,
    /// 98... 989015132328
    #[prost(string, tag = "5")]
    pub password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthSingInResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    ///    SelfUserView SelfUserView = 3;
    #[prost(string, tag = "2")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthLogOutParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthLogOutResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    #[prost(string, tag = "2")]
    pub error_message: std::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SendConfirmCodeTypeEnum {
    SendCodeOk = 0,
    SendCodeEmail = 1,
}
/// CrDU
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCreateChannelParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(uint32, tag = "2")]
    pub creator_profile_cid: u32,
    /// or _name
    #[prost(string, tag = "3")]
    pub channel_title: std::string::String,
    #[prost(string, tag = "4")]
    pub user_name: std::string::String,
    #[prost(string, tag = "15")]
    pub about: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCreateChannelResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditChannelParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
    #[prost(bool, tag = "3")]
    pub set_new_title: bool,
    #[prost(string, tag = "4")]
    pub new_title: std::string::String,
    #[prost(bool, tag = "5")]
    pub set_new_about: bool,
    #[prost(string, tag = "6")]
    pub new_about: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditChannelResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteChannelParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteChannelResponse {}
/// Author
///
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAddAuthorParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAddAuthorResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeAuthorPermissionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeAuthorPermissionResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveAuthorParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveAuthorResponse {}
/// Follow
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelFollowChannelParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelFollowChannelResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnFollowChannelParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnFollowChannelResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveFollowersParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveFollowersResponse {}
/// Subscribe
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscribeParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscribeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnSubscribeParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnSubscribeResponse {}
/// NTO NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveSubscribersParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveSubscribersResponse {}
/// Privacy
///
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangePrivacyParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangePrivacyResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeDefaultPermissionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeDefaultPermissionResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRevokeLinkParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRevokeLinkResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeUsernameParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeUsernameResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBlockProfileParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBlockProfileResponse {}
/// Messages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(message, optional, tag = "2")]
    pub message_input: ::std::option::Option<NewMessageInput>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(fixed64, tag = "4")]
    pub message_gid: u64,
    #[prost(uint32, tag = "3")]
    pub by_profile_cid: u32,
    #[prost(string, tag = "2")]
    pub new_text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteMessagesParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
    #[prost(uint64, repeated, tag = "3")]
    pub message_gids: ::std::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteMessagesResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelClearHistoryParam {
    /// NOT NOW
    #[prost(uint64, tag = "3")]
    pub from_message_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelClearHistoryResponse {}
/// Like
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelLikeMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(fixed64, tag = "2")]
    pub message_gid: u64,
    #[prost(uint32, tag = "3")]
    pub by_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelLikeMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnLikeMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(fixed64, tag = "2")]
    pub message_gid: u64,
    #[prost(uint32, tag = "3")]
    pub by_profile_cid: u32,
    #[prost(fixed64, tag = "4")]
    pub like_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnLikeMessageResponse {}
/// ReShare
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReShareMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(fixed64, tag = "2")]
    pub message_gid: u64,
    #[prost(uint32, tag = "3")]
    pub by_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReShareMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnReShareMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(fixed64, tag = "2")]
    pub message_gid: u64,
    #[prost(uint32, tag = "3")]
    pub by_profile_cid: u32,
    #[prost(fixed64, tag = "4")]
    pub re_share_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnReShareMessageResponse {}
/// Comment
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAddCommentParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(fixed64, tag = "2")]
    pub message_gid: u64,
    #[prost(uint32, tag = "3")]
    pub by_profile_cid: u32,
    #[prost(string, tag = "4")]
    pub comment_text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAddCommentResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteCommentParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(fixed64, tag = "2")]
    pub message_gid: u64,
    #[prost(uint32, tag = "3")]
    pub by_profile_cid: u32,
    #[prost(fixed64, tag = "4")]
    pub comment_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteCommentResponse {}
/// Pin
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPinMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPinMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnPinMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnPinMessageResponse {}
/// Avatar
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarAddParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
    #[prost(uint64, tag = "2")]
    pub file_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarAddResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarChangeParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarChangeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarDeleteParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarDeleteResponse {}
/// Others
///
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendDoingActionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendDoingActionResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportChannelParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportChannelResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportMessageParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportMessageResponse {}
/// Views
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetFullParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetFullResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetMessagesListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetMessagesListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetMediaListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetMediaListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetAuthorsParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetAuthorsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetFollowersParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetFollowersResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetFollowingsParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetFollowingsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetSubscribersParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetSubscribersResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBlockedParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBlockedResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarGetListParam {
    #[prost(uint32, tag = "1")]
    pub channel_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarGetListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetInboxParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetInboxResponse {}
/// crud
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteChatParam {
    /// both - many
    #[prost(uint64, tag = "1")]
    pub chat_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteChatResponse {}
// Members

// Privacy

// Messages

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSendMessageParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSendMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatEditMessageParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatEditMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteMessagesParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteMessagesResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteHistoryParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteHistoryResponse {}
/// Notifications
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSetNotificationParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSetNotificationResponse {}
// Others

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatAvatarChangeParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatAvatarChangeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSendDoingActionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSendDoingActionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatReportChatParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatReportChatResponse {}
// Views

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetFullMessageParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetFullMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetMessagesListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetMessagesListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetMediaListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetMediaListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetInboxParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetInboxResponse {}
/// CrDU
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateGroupParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
    #[prost(uint32, tag = "2")]
    pub creator_profile_cid: u32,
    /// or _name
    #[prost(string, tag = "3")]
    pub group_title: std::string::String,
    /// if older message is is viewable for new members > now true for all new groups
    #[prost(bool, tag = "8")]
    pub history_viewable: bool,
    /// view history without joining
    #[prost(bool, tag = "9")]
    pub force_join: bool,
    /// global searchable in bar > force_join must be false + history_viewable = ture
    #[prost(bool, tag = "17")]
    pub global_search: bool,
    #[prost(string, tag = "15")]
    pub about: std::string::String,
    /// Not Now
    ///
    /// with "group" suffix > not now > all is private now
    #[prost(string, tag = "4")]
    pub user_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateGroupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditGroupParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
    #[prost(bool, tag = "3")]
    pub set_new_title: bool,
    #[prost(string, tag = "4")]
    pub new_title: std::string::String,
    #[prost(bool, tag = "5")]
    pub set_new_about: bool,
    /// Add others editable options
    #[prost(string, tag = "6")]
    pub new_about: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditGroupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteGroupParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteGroupResponse {}
/// Admins Members Actions
///
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddAdminParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddAdminResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRemoveMemberParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
    #[prost(uint32, tag = "3")]
    pub member_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRemoveMemberResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberLevelParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberLevelResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberPermissionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberPermissionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupBanMemberParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
    #[prost(uint32, tag = "3")]
    pub banned_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupBanMemberResponse {}
/// Members Actions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinGroupParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
    #[prost(uint32, tag = "3")]
    pub new_member_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinGroupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupLeaveGroupParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
    #[prost(uint32, tag = "3")]
    pub member_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupLeaveGroupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddMemberParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
    #[prost(uint32, tag = "3")]
    pub new_member_profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddMemberResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSetNotificationParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSetNotificationResponse {}
/// Privacy
///
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangePrivacyParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangePrivacyResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeDefaultPermissionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeDefaultPermissionResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRevokeLinkParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRevokeLinkResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeUsernameParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeUsernameResponse {}
/// Messages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendMessageParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
    #[prost(message, optional, tag = "2")]
    pub message_input: ::std::option::Option<NewMessageInput>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditMessageParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
    #[prost(uint32, tag = "3")]
    pub by_profile_cid: u32,
    #[prost(string, tag = "2")]
    pub new_text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupPinMessageParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupPinMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupUnPinMessageParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupUnPinMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteMessageParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteMessagesParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
    #[prost(uint32, tag = "2")]
    pub by_profile_cid: u32,
    #[prost(uint64, repeated, tag = "3")]
    pub message_ids: ::std::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteMessagesResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteHistoryParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteHistoryResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupClearHistoryParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupClearHistoryResponse {}
/// Others
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarAddParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarAddResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarChangeParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarChangeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarDeleteParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarDeleteResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarGetListParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarGetListResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendDoingActionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendDoingActionResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupReportGroupParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupReportGroupResponse {}
/// Views
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetFullParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetFullResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMessagesListParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMessagesListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMediaListParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMediaListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMembersListParam {
    #[prost(uint32, tag = "1")]
    pub group_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMembersListResponse {}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetAdminsListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetAdminsListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetInboxParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetInboxResponse {}
/// Dep
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSetDraftParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSetDraftResponse {}
// Add Profile

/// Settings
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileSetSettingsParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileSetSettingsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUsers1Param {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUsers1Response {
    #[prost(message, repeated, tag = "1")]
    pub users: ::std::vec::Vec<Profile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProfilesParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProfilesResponse {
    #[prost(message, repeated, tag = "1")]
    pub profiles: ::std::vec::Vec<Profile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelsParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelsResponse {
    #[prost(message, repeated, tag = "1")]
    pub channels: ::std::vec::Vec<Channel>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectsParam {}
///repeated Direct directs = 1;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessagesParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessagesResponse {
    #[prost(message, repeated, tag = "1")]
    pub directs: ::std::vec::Vec<Message>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedEchoParam {
    #[prost(string, tag = "1")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedEchoResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    #[prost(string, tag = "2")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedCheckUserNameParam {
    #[prost(string, tag = "1")]
    pub username: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedCheckUserNameResponse {
    #[prost(bool, tag = "1")]
    pub is_available: bool,
    #[prost(string, tag = "2")]
    pub username: std::string::String,
    #[prost(string, tag = "3")]
    pub show_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Param {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    #[prost(string, tag = "2")]
    pub text: std::string::String,
}
/// CrUd
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRegisterUserParam {
    #[prost(uint32, tag = "1")]
    pub user_cid: u32,
    #[prost(string, tag = "2")]
    pub first_name: std::string::String,
    #[prost(string, tag = "3")]
    pub last_name: std::string::String,
    #[prost(string, tag = "15")]
    pub phone: std::string::String,
    #[prost(uint32, tag = "5")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRegisterUserResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::std::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEditUserParam {
    #[prost(uint32, tag = "1")]
    pub user_cid: u32,
    #[prost(bool, tag = "3")]
    pub set_new_name: bool,
    #[prost(string, tag = "4")]
    pub new_first_name: std::string::String,
    #[prost(string, tag = "6")]
    pub new_last_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEditUserResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::std::option::Option<User>,
}
/// Delete User
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteSendCodeParam {
    #[prost(bool, tag = "1")]
    pub resend: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteSendCodeResponse {
    #[prost(string, repeated, tag = "4")]
    pub sms_numbers: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteConfirmCodeParam {
    ///4 digit 4215
    #[prost(string, tag = "3")]
    pub code: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteConfirmCodeResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteUserParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteUserResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
}
/// Phone
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserChangePhoneNumberParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserChangePhoneNumberResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    #[prost(string, tag = "2")]
    pub text: std::string::String,
}
// Email

// Password

/// Session
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRemoveSessionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRemoveSessionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRemoveOtherParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRemoveOtherResponse {}
/// Views
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGetMeParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGetMeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGetActiveSessionsParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGetActiveSessionsResponse {}
