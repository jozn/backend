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
    #[prost(oneof="sample_message::TestOneof", tags="4, 9, 10")]
    pub test_oneof: ::core::option::Option<sample_message::TestOneof>,
}
/// Nested message and enum types in `SampleMessage`.
pub mod sample_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TestOneof {
        #[prost(string, tag="4")]
        Name(::prost::alloc::string::String),
        #[prost(message, tag="9")]
        SubMessage(super::Invoke),
        #[prost(message, tag="10")]
        AddContact(super::Contact),
    }
}
//////////////////////////// Common Types ////////////////////
/// Invoke is send from user mobile client app.
///
/// imut all
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Invoke {
    /// imut
    #[prost(uint32, tag="6")]
    pub namespace: u32,
    /// imut
    #[prost(uint32, tag="1")]
    pub method: u32,
    /// imut
    #[prost(uint32, tag="7")]
    pub user_id: u32,
    /// imut
    #[prost(uint64, tag="2")]
    pub invoke_id: u64,
    /// imut
    #[prost(string, tag="8")]
    pub session_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="10")]
    pub api_version: u32,
    /// ex: "Android"
    #[prost(string, tag="11")]
    pub app_name: ::prost::alloc::string::String,
    /// "v3.2"
    #[prost(string, tag="12")]
    pub app_version: ::prost::alloc::string::String,
    /// "HMD GlobalNokia 3.2, Android 10 Q (29)"
    #[prost(string, tag="13")]
    pub device_name: ::prost::alloc::string::String,
    /// imut
    #[prost(bytes="vec", tag="4")]
    pub rpc_data: ::prost::alloc::vec::Vec<u8>,
}
/// InvokeResponse is returned from server in response of Invoke.
///
/// imut all
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvokeResponse {
    /// imut
    #[prost(uint32, tag="6")]
    pub namespace: u32,
    /// imut
    #[prost(uint32, tag="1")]
    pub method: u32,
    /// imut
    #[prost(uint32, tag="7")]
    pub user_id: u32,
    /// imut
    #[prost(uint64, tag="2")]
    pub invoke_id: u64,
    /// imut
    #[prost(bytes="vec", tag="4")]
    pub rpc_data: ::prost::alloc::vec::Vec<u8>,
}
///==================== User ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(fixed64, tag="1")]
    pub contact_gid: u64,
    #[prost(uint32, tag="2")]
    pub profile_id: u32,
    /// +98*
    #[prost(string, tag="4")]
    pub phone: ::prost::alloc::string::String,
    /// In device
    #[prost(string, tag="5")]
    pub first_name: ::prost::alloc::string::String,
    /// In device
    #[prost(string, tag="6")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="12")]
    pub peer_profile_id: u32,
    #[prost(uint32, tag="13")]
    pub created_time: u32,
}
/// Notes: User should only be returned for user own api calls, it's contain user's private data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// Info 0-10
    #[prost(uint32, tag="1")]
    pub user_id: u32,
    /// for default profile
    #[prost(string, tag="14")]
    pub phone: ::prost::alloc::string::String,
    ///
    #[prost(string, tag="15")]
    pub email: ::prost::alloc::string::String,
    #[prost(uint32, tag="36")]
    pub created_time: u32,
    #[prost(uint32, tag="37")]
    pub version_time: u32,
    /// Names - for Default profile build from this > set def_profile title on change
    #[prost(string, tag="40")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(string, tag="41")]
    pub last_name: ::prost::alloc::string::String,
    /// Access 20
    #[prost(bool, tag="38")]
    pub is_deleted: bool,
    #[prost(bool, tag="39")]
    pub is_banned: bool,
    /// Passwords 30
    #[prost(string, tag="17")]
    pub password_hash: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub password_salt: ::prost::alloc::string::String,
    /// Profile 40
    #[prost(message, optional, tag="1114")]
    pub def_profile: ::core::option::Option<Profile>,
    /// Session 50
    #[prost(message, repeated, tag="102")]
    pub sessions: ::prost::alloc::vec::Vec<Session>,
    // Collections

    /// Shopping 60
    #[prost(message, optional, tag="111")]
    pub shopping_profile: ::core::option::Option<ShoppingProfile>,
    #[prost(message, repeated, tag="113")]
    pub stores: ::prost::alloc::vec::Vec<Store>,
    /// Not now 800
    #[prost(message, repeated, tag="110")]
    pub profiles: ::prost::alloc::vec::Vec<Profile>,
}
///==================== Profile ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Profile {
    /// Info 0-10
    ///
    /// imut - profile_id
    #[prost(uint32, tag="1")]
    pub profile_id: u32,
    /// imut
    #[prost(uint32, tag="2")]
    pub user_id: u32,
    /// imut
    #[prost(uint32, tag="103")]
    pub created_time: u32,
    /// Channel
    ///
    /// imut
    #[prost(message, optional, tag="100")]
    pub primary_channel: ::core::option::Option<Channel>,
    /// Save Channel
    ///
    /// imut
    #[prost(message, optional, tag="109")]
    pub saved_channel: ::core::option::Option<SavedChannel>,
    /// Settings
    ///
    /// imut
    #[prost(message, optional, tag="107")]
    pub setting: ::core::option::Option<ProfileSettings>,
    /// Views 100
    ///
    /// When sending profile to others set this if they have this profile in their contacts
    #[prost(message, optional, tag="111")]
    pub contact_info: ::core::option::Option<Contact>,
    /// Demonstration: 800 - collections > not really present > big > fetch with api
    ///
    /// mut
    #[prost(message, repeated, tag="104")]
    pub channels: ::prost::alloc::vec::Vec<Channel>,
    ///  repeated Direct directs = 105;// mut
    ///
    /// mut
    #[prost(message, repeated, tag="106")]
    pub groups: ::prost::alloc::vec::Vec<Group>,
    /// mut
    #[prost(message, repeated, tag="108")]
    pub contacts: ::prost::alloc::vec::Vec<Contact>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileSettings {
}
///==================== Peer Chat ==================
///
///?? or embed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chat {
    /// imut
    #[prost(fixed64, tag="7")]
    pub chat_gid: u64,
    /// imut
    #[prost(uint32, tag="1")]
    pub profile1_id: u32,
    /// imut
    #[prost(uint32, tag="2")]
    pub profile2_id: u32,
    /// imut
    #[prost(fixed64, tag="3")]
    pub direct1_gid: u64,
    /// imut
    #[prost(fixed64, tag="4")]
    pub direct2_gid: u64,
    /// Views
    ///
    /// ? must use profile
    #[prost(message, optional, tag="49")]
    pub contact: ::core::option::Option<Contact>,
    /// Profile > or Peer Chat ?
    #[prost(message, optional, tag="149")]
    pub profile: ::core::option::Option<Profile>,
    /// Messages
    ///
    /// mut
    #[prost(message, optional, tag="25")]
    pub last_message: ::core::option::Option<Message>,
    /// mut
    #[prost(message, optional, tag="26")]
    pub pinned_message: ::core::option::Option<Message>,
    #[prost(message, optional, tag="700")]
    pub inboxer: ::core::option::Option<Inboxer>,
}
///?? or embed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDep {
    /// imut
    #[prost(fixed64, tag="7")]
    pub chat_gid: u64,
    /// imut
    #[prost(uint32, tag="1")]
    pub profile1_cid: u32,
    /// imut
    #[prost(uint32, tag="2")]
    pub profile2_cid: u32,
    /// imut
    #[prost(fixed64, tag="3")]
    pub direct1_gid: u64,
    /// imut
    #[prost(fixed64, tag="4")]
    pub direct2_gid: u64,
}
///==================== Messages ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// 1-10 Info
    ///
    /// imut
    #[prost(fixed64, tag="1")]
    pub message_gid: u64,
    /// imut
    #[prost(uint32, tag="2")]
    pub by_profile_id: u32,
    /// imut - mostly
    #[prost(enumeration="MessageType", tag="107")]
    pub message_type: i32,
    #[prost(string, tag="7")]
    pub text: ::prost::alloc::string::String,
    /// imut > Including bots 1: Android app, 2... to 1000 reserved - bots >1000
    #[prost(uint32, tag="12")]
    pub via_app_id: u32,
    /// imut
    #[prost(uint32, tag="13")]
    pub seq: u32,
    /// imut
    #[prost(uint32, tag="18")]
    pub created_time: u32,
    /// Edit
    ///
    /// mut
    #[prost(uint32, tag="17")]
    pub edited_time: u32,
    /// Sync info
    #[prost(enumeration="MessageDeliveryStatues", tag="105")]
    pub delivery_status: i32,
    /// mut-once
    #[prost(uint32, tag="106")]
    pub delivery_time: u32,
    /// Visibility
    ///
    /// imut - verified system messages - 2 bytes tag
    #[prost(bool, tag="111")]
    pub verified: bool,
    /// maybe temp with a purge period - 2 bytes
    #[prost(bool, tag="150")]
    pub deleted: bool,
    /// Header: Forward/Reply
    ///
    /// forward is always live object from other channels, but not from other chats, groups
    #[prost(message, optional, boxed, tag="16")]
    pub forward: ::core::option::Option<::prost::alloc::boxed::Box<Message>>,
    #[prost(message, optional, boxed, tag="50")]
    pub reply_to: ::core::option::Option<::prost::alloc::boxed::Box<Message>>,
    /// Channels Settings
    #[prost(uint32, tag="1001")]
    pub channel_id: u32,
    #[prost(message, optional, tag="102")]
    pub setting: ::core::option::Option<MessageSetting>,
    /// Channels Extra
    #[prost(message, optional, tag="101")]
    pub counts: ::core::option::Option<MessageCount>,
    /// Group
    #[prost(uint32, tag="1002")]
    pub group_id: u32,
    /// Media
    #[prost(message, repeated, tag="103")]
    pub files: ::prost::alloc::vec::Vec<FileMsg>,
    /// Stores
    #[prost(message, optional, tag="110")]
    pub product: ::core::option::Option<Product>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageCount {
    ///???
    #[prost(fixed64, tag="1")]
    pub message_gid: u64,
    #[prost(uint32, tag="2")]
    pub comments_count: u32,
    #[prost(uint32, tag="3")]
    pub likes_count: u32,
    #[prost(int64, tag="4")]
    pub views_count: i64,
    #[prost(uint32, tag="5")]
    pub reshared_count: u32,
    #[prost(uint32, tag="6")]
    pub chat_shared_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageSetting {
    #[prost(uint32, tag="11")]
    pub disable_comment: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageLog {
    #[prost(enumeration="MessageLogType", tag="10")]
    pub log_type: i32,
    #[prost(uint32, tag="2")]
    pub by_profile_id: u32,
    #[prost(uint32, tag="3")]
    pub target_profile_id: u32,
    #[prost(message, optional, tag="11")]
    pub target_profile_view: ::core::option::Option<Profile>,
}
///==================== Channel ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    /// Info
    ///
    /// imut
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    /// imut
    #[prost(uint32, tag="7")]
    pub creator_profile_id: u32,
    /// imut
    #[prost(bool, tag="101")]
    pub is_profile_channel: bool,
    /// imut
    #[prost(uint32, tag="36")]
    pub created_time: u32,
    /// Visibility
    ///
    /// mut
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
    /// mut - For default channels take if from User first and lastname.
    #[prost(string, tag="3")]
    pub channel_title: ::prost::alloc::string::String,
    /// mut
    #[prost(string, tag="16")]
    pub about: ::prost::alloc::string::String,
    /// mut
    #[prost(bool, tag="6")]
    pub is_verified: bool,
    /// Sync
    ///
    /// mut - version
    #[prost(fixed64, tag="21")]
    pub sync_time_ms: u64,
    /// Access
    ///
    /// mut
    #[prost(uint32, tag="38")]
    pub is_deleted: u32,
    /// mut
    #[prost(uint32, tag="39")]
    pub is_banned: u32,
    /// Settings
    ///
    /// mut
    #[prost(string, tag="17")]
    pub invite_link_hash: ::prost::alloc::string::String,
    /// for owner
    #[prost(message, optional, tag="90")]
    pub notification_setting: ::core::option::Option<ChannelOwnerNotification>,
    /// mut
    #[prost(enumeration="ChannelPrivacy", tag="9")]
    pub privacy: i32,
    /// Messages
    ///
    /// mut
    #[prost(message, optional, tag="25")]
    pub last_message: ::core::option::Option<Message>,
    /// mut
    #[prost(uint32, tag="19")]
    pub message_seq: u32,
    /// Pin
    ///
    /// mut
    #[prost(message, optional, tag="26")]
    pub pinned_message: ::core::option::Option<Message>,
    /// Avatar
    ///
    /// mut
    #[prost(message, optional, tag="100")]
    pub avatar: ::core::option::Option<FileMsg>,
    /// mut
    #[prost(int64, tag="40")]
    pub avatar_count: i64,
    #[prost(message, optional, tag="700")]
    pub inboxer: ::core::option::Option<Inboxer>,
    /// Counts -> followers_count in profile
    #[prost(uint32, tag="20")]
    pub followers_count: u32,
    #[prost(uint32, tag="22")]
    pub posts_count: u32,
    #[prost(uint32, tag="33")]
    pub likes_count: u32,
    #[prost(uint32, tag="34")]
    pub reshared_count: u32,
    /// mut
    #[prost(message, optional, tag="44")]
    pub counts: ::core::option::Option<MediaCounts>,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOwnerNotification {
}
///==================== Store ==================
///
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShoppingProfile {
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Store {
    #[prost(uint32, tag="1")]
    pub cid: u32,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
    /// For default channels take if from User.
    #[prost(string, tag="3")]
    pub store_name: ::prost::alloc::string::String,
    #[prost(string, tag="102")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub creator_user_id: u32,
    #[prost(string, tag="16")]
    pub about: ::prost::alloc::string::String,
    #[prost(uint32, tag="19")]
    pub message_seq: u32,
    /// version
    #[prost(fixed64, tag="21")]
    pub sync_time_ms: u64,
    #[prost(uint32, tag="36")]
    pub created_time: u32,
    #[prost(uint32, tag="39")]
    pub is_banned: u32,
    #[prost(bool, tag="6")]
    pub is_verified: bool,
    #[prost(message, optional, tag="100")]
    pub avatar: ::core::option::Option<FileMsg>,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    /// seq
    #[prost(uint32, tag="1")]
    pub product_id: u32,
    #[prost(uint32, tag="53")]
    pub category_id: u32,
    #[prost(string, tag="50")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, tag="51")]
    pub brand: ::prost::alloc::string::String,
    ///from 10_000 - 5% > 500
    #[prost(uint32, tag="3")]
    pub fee_rate: u32,
    #[prost(uint32, tag="5")]
    pub sales_count: u32,
    #[prost(uint32, tag="6")]
    pub price: u32,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductPriceInfo {
    #[prost(uint32, tag="1")]
    pub price: u32,
    #[prost(uint32, tag="6")]
    pub discount_price: u32,
    /// from 1000
    #[prost(uint32, tag="3")]
    pub commission_rate: u32,
}
///==================== Inboxer ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inboxer {
    /// Info 1-10
    ///
    /// imut  > del
    #[prost(fixed64, tag="1")]
    pub inboxer_gid: u64,
    /// imutt > del
    #[prost(uint32, tag="5")]
    pub profile_id: u32,
    /// Meta info (sync) - mut
    ///
    /// mut
    #[prost(uint32, tag="12")]
    pub unseen_count: u32,
    /// mut
    #[prost(fixed64, tag="45")]
    pub sort_time_ms: u64,
    /// mut
    #[prost(fixed64, tag="104")]
    pub sync_time_ms: u64,
    /// mut
    #[prost(fixed64, tag="16")]
    pub my_last_seen_seq: u64,
    /// mut
    #[prost(fixed64, tag="17")]
    pub my_last_seen_msg_id: u64,
    /// Pin
    ///
    /// mut
    #[prost(fixed64, tag="10")]
    pub pin_time_ms: u64,
}
///==================== Saved ==================
///
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedChannel {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    #[prost(uint32, tag="7")]
    pub creator_profile_id: u32,
    #[prost(uint32, tag="19")]
    pub message_seq: u32,
    /// version
    #[prost(fixed64, tag="21")]
    pub sync_time_ms: u64,
    #[prost(uint32, tag="36")]
    pub created_time: u32,
    #[prost(message, optional, tag="41")]
    pub counts: ::core::option::Option<ChannelCountsDep>,
    #[prost(message, optional, tag="25")]
    pub last_message: ::core::option::Option<Message>,
}
///==================== Group ==================
///
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// Info
    #[prost(uint32, tag="1")]
    pub group_id: u32,
    #[prost(uint32, tag="7")]
    pub creator_profile_id: u32,
    #[prost(uint32, tag="21")]
    pub created_time: u32,
    /// Visibility
    ///
    /// or _name
    #[prost(string, tag="3")]
    pub group_title: ::prost::alloc::string::String,
    /// with "group" suffix > not now > all is private now
    #[prost(string, tag="4")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub about: ::prost::alloc::string::String,
    /// Sync
    #[prost(uint32, tag="10")]
    pub seq: u32,
    #[prost(fixed64, tag="20")]
    pub sort_time: u64,
    #[prost(fixed64, tag="40")]
    pub sync_time: u64,
    /// Access
    #[prost(bool, tag="23")]
    pub is_deleted: bool,
    #[prost(bool, tag="24")]
    pub is_banned: bool,
    /// Setting
    #[prost(bool, tag="8")]
    pub history_viewable: bool,
    /// global searchable, view history without joining
    #[prost(bool, tag="9")]
    pub is_open_group: bool,
    #[prost(string, tag="16")]
    pub invite_link_hash: ::prost::alloc::string::String,
    /// Messages
    #[prost(message, optional, tag="25")]
    pub last_message: ::core::option::Option<Message>,
    #[prost(message, optional, tag="26")]
    pub pinned_message: ::core::option::Option<Message>,
    // Pin

    /// Avatar
    #[prost(uint32, tag="14")]
    pub avatar_count: u32,
    #[prost(message, optional, tag="27")]
    pub avatar: ::core::option::Option<FileMsg>,
    /// Counts
    #[prost(uint32, tag="17")]
    pub members_count: u32,
    #[prost(uint32, tag="18")]
    pub admins_count: u32,
    #[prost(uint32, tag="19")]
    pub moderator_counts: u32,
    #[prost(message, optional, tag="200")]
    pub media_counts: ::core::option::Option<MediaCounts>,
    /// Member
    ///
    /// s_imut
    #[prost(message, optional, tag="43")]
    pub group_member: ::core::option::Option<GroupMember>,
    /// imut
    #[prost(fixed64, tag="11")]
    pub visible_from_msg_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMember {
    /// Info
    #[prost(int64, tag="1")]
    pub member_gid: i64,
    #[prost(uint32, tag="2")]
    pub group_id: u32,
    #[prost(uint32, tag="3")]
    pub profile_id: u32,
    #[prost(uint32, tag="4")]
    pub by_profile_id: u32,
    #[prost(uint32, tag="6")]
    pub created_time: u32,
}
///==================== Others ==================
///
/// Shared amoung chat, groups, channel
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaCounts {
    #[prost(uint32, tag="23")]
    pub media_count: u32,
    #[prost(uint32, tag="24")]
    pub photo_count: u32,
    #[prost(uint32, tag="25")]
    pub video_count: u32,
    #[prost(uint32, tag="26")]
    pub gif_count: u32,
    #[prost(uint32, tag="27")]
    pub audio_count: u32,
    #[prost(uint32, tag="28")]
    pub voice_count: u32,
    #[prost(uint32, tag="29")]
    pub file_count: u32,
    #[prost(uint32, tag="30")]
    pub link_count: u32,
    #[prost(uint32, tag="32")]
    pub pined_count: u32,
}
///todo file, image, video, audio seperatin
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Media {
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileMsg {
    #[prost(fixed64, tag="1")]
    pub file_gid: u64,
    #[prost(uint32, tag="2")]
    pub access_hash: u32,
    #[prost(uint32, tag="3")]
    pub file_type: u32,
    #[prost(uint32, tag="4")]
    pub width: u32,
    #[prost(uint32, tag="5")]
    pub height: u32,
    #[prost(string, tag="6")]
    pub extension: ::prost::alloc::string::String,
    #[prost(string, tag="61")]
    pub full_path: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub user_id: u32,
    #[prost(bytes="vec", tag="8")]
    pub data_thumb: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
    #[prost(string, tag="2")]
    pub session_hash: ::prost::alloc::string::String,
    ///  uint64 device_id = 100;
    #[prost(uint32, tag="3")]
    pub user_id: u32,
    #[prost(string, tag="4")]
    pub last_ip: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub user_agent: ::prost::alloc::string::String,
    ///  DevicePlatform platform = 9;
    ///  uint32 api_version = 5;
    #[prost(uint32, tag="10")]
    pub api_version: u32,
    /// ex: "Android"
    #[prost(string, tag="11")]
    pub app_name: ::prost::alloc::string::String,
    /// "v3.2"
    #[prost(string, tag="12")]
    pub app_version: ::prost::alloc::string::String,
    /// "HMD GlobalNokia 3.2, Android 10 Q (29)"
    #[prost(string, tag="13")]
    pub device_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub active_time: u32,
    #[prost(uint32, tag="7")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sms {
    /// we maybe need this
    #[prost(string, tag="3")]
    pub install_uuid: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub phone_number: ::prost::alloc::string::String,
    #[prost(string, tag="32")]
    pub country_code: ::prost::alloc::string::String,
    #[prost(bool, tag="103")]
    pub for_login: bool,
    #[prost(string, tag="107")]
    pub hash_code: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub confirm_code: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub gateway_number: ::prost::alloc::string::String,
    #[prost(string, tag="101")]
    pub text_body: ::prost::alloc::string::String,
    #[prost(uint32, tag="100")]
    pub created_time: u32,
    /// Below are for easier debugging purpose > stringy
    ///
    /// some custom debug info: http header, code, body, ...
    #[prost(string, tag="9")]
    pub gateway_error: ::prost::alloc::string::String,
    /// like "register" "login" "delete" "marketing" ,...
    #[prost(string, tag="14")]
    pub intent: ::prost::alloc::string::String,
    /// like "confirmed" "unconfirmed" "sending" "gateway_error"
    #[prost(string, tag="102")]
    pub result: ::prost::alloc::string::String,
}
///==================== New > move ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    /// imut
    #[prost(uint64, tag="6")]
    pub comment_gid: u64,
    /// imut
    #[prost(uint64, tag="1")]
    pub message_gid: u64,
    /// imut
    #[prost(uint32, tag="4")]
    pub channel_id: u32,
    /// imut
    #[prost(uint32, tag="2")]
    pub profile_id: u32,
    /// imut
    #[prost(uint32, tag="3")]
    pub created_time: u32,
    /// mut
    #[prost(string, tag="7")]
    pub text: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Follower {
    /// imut
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    /// imut
    #[prost(uint32, tag="2")]
    pub profile_id: u32,
    /// imut
    #[prost(uint32, tag="3")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscribe {
    /// imut
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    /// imut
    #[prost(uint32, tag="2")]
    pub profile_id: u32,
    /// imut
    #[prost(uint32, tag="3")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Like {
    /// imut
    #[prost(uint64, tag="1")]
    pub message_gid: u64,
    /// imut
    #[prost(uint32, tag="2")]
    pub profile_id: u32,
    /// imut
    #[prost(uint32, tag="3")]
    pub created_time: u32,
}
//==================== Views ==================

/// C: Common
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewMessageInput {
    /// imut
    #[prost(fixed64, tag="1")]
    pub message_gid: u64,
    /// imut
    #[prost(uint32, tag="2")]
    pub by_profile_id: u32,
    /// imut - mostly
    #[prost(enumeration="MessageType", tag="107")]
    pub message_type: i32,
    #[prost(string, tag="7")]
    pub text: ::prost::alloc::string::String,
    /// imut
    #[prost(uint32, tag="12")]
    pub via_app_id: u32,
    /// imut
    #[prost(uint32, tag="13")]
    pub seq: u32,
    /// imut
    #[prost(uint32, tag="18")]
    pub created_time: u32,
    /// imut - verified system messages - 2 bytes tag
    #[prost(bool, tag="111")]
    pub verified: bool,
}
///==================== Deprecated =============
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCountsDep {
    #[prost(uint32, tag="20")]
    pub followers_count: u32,
    ///uint32 following_count = 21; // ?? just profile
    #[prost(uint32, tag="22")]
    pub posts_count: u32,
    #[prost(uint32, tag="23")]
    pub media_count: u32,
    #[prost(uint32, tag="24")]
    pub photo_count: u32,
    #[prost(uint32, tag="25")]
    pub video_count: u32,
    #[prost(uint32, tag="26")]
    pub gif_count: u32,
    #[prost(uint32, tag="27")]
    pub audio_count: u32,
    #[prost(uint32, tag="28")]
    pub voice_count: u32,
    #[prost(uint32, tag="29")]
    pub file_count: u32,
    #[prost(uint32, tag="30")]
    pub link_count: u32,
    #[prost(uint32, tag="31")]
    pub board_count: u32,
    #[prost(uint32, tag="32")]
    pub pined_count: u32,
    #[prost(uint32, tag="33")]
    pub likes_count: u32,
    #[prost(uint32, tag="34")]
    pub reshared_count: u32,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCountsDep {
    #[prost(uint32, tag="23")]
    pub media_count: u32,
    #[prost(uint32, tag="24")]
    pub photo_count: u32,
    #[prost(uint32, tag="25")]
    pub video_count: u32,
    #[prost(uint32, tag="26")]
    pub gif_count: u32,
    #[prost(uint32, tag="27")]
    pub audio_count: u32,
    #[prost(uint32, tag="28")]
    pub voice_count: u32,
    #[prost(uint32, tag="29")]
    pub file_count: u32,
    #[prost(uint32, tag="30")]
    pub link_count: u32,
    #[prost(uint32, tag="32")]
    pub pined_count: u32,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmsBk {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(string, tag="3")]
    pub install_uuid: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub phone_number: ::prost::alloc::string::String,
    #[prost(string, tag="32")]
    pub country_code: ::prost::alloc::string::String,
    #[prost(bool, tag="103")]
    pub for_login: bool,
    #[prost(string, tag="107")]
    pub hash_code: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub confirm_code: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub gateway_number: ::prost::alloc::string::String,
    #[prost(string, tag="101")]
    pub text_body: ::prost::alloc::string::String,
    #[prost(uint32, tag="100")]
    pub created_time: u32,
    /// Below are for easier debugging purpose > stringy
    ///
    /// some custom debug info: http header, code, body, ...
    #[prost(string, tag="9")]
    pub gateway_error: ::prost::alloc::string::String,
    /// like "register" "login" "delete" "marketing" ,...
    #[prost(string, tag="14")]
    pub intent: ::prost::alloc::string::String,
    /// like "confirmed" "unconfirmed" "sending" "gateway_error"
    #[prost(string, tag="102")]
    pub result: ::prost::alloc::string::String,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionBk {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(string, tag="2")]
    pub session_uuid: ::prost::alloc::string::String,
    #[prost(uint64, tag="100")]
    pub device_id: u64,
    #[prost(uint32, tag="3")]
    pub user_id: u32,
    #[prost(string, tag="4")]
    pub last_ip_address: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub user_agent: ::prost::alloc::string::String,
    #[prost(enumeration="DevicePlatform", tag="9")]
    pub platform: i32,
    #[prost(uint32, tag="5")]
    pub api_version: u32,
    #[prost(uint32, tag="6")]
    pub active_time: u32,
    #[prost(uint32, tag="7")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shop {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Upload {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shared {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Auth {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sample {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMaster {
}
//================ File Media ==============

/// flat represenation of file
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileView {
    #[prost(fixed64, tag="1")]
    pub file_id: u64,
    #[prost(fixed64, tag="11")]
    pub ref_id: u64,
    #[prost(uint32, tag="12")]
    pub bucket_id: u32,
    #[prost(uint32, tag="2")]
    pub secret: u32,
    /// from client > is needed??? it should be present in Message Document or other higher layer not in here
    #[prost(string, tag="13")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(enumeration="FileType", tag="3")]
    pub file_type: i32,
    #[prost(string, tag="6")]
    pub file_mime: ::prost::alloc::string::String,
    /// Image
    #[prost(uint32, tag="100")]
    pub image_width: u32,
    #[prost(uint32, tag="101")]
    pub image_height: u32,
    /// gifs
    #[prost(message, optional, boxed, tag="102")]
    pub image_cover: ::core::option::Option<::prost::alloc::boxed::Box<FileView>>,
    /// Video
    #[prost(uint32, tag="200")]
    pub video_width: u32,
    #[prost(uint32, tag="201")]
    pub video_height: u32,
    #[prost(uint32, tag="203")]
    pub video_duration: u32,
    #[prost(message, optional, boxed, tag="204")]
    pub video_cover: ::core::option::Option<::prost::alloc::boxed::Box<FileView>>,
    /// Audio
    #[prost(bool, tag="300")]
    pub audio_is_voice: bool,
    #[prost(uint32, tag="301")]
    pub audio_duration: u32,
    #[prost(string, tag="302")]
    pub audio_title: ::prost::alloc::string::String,
    #[prost(string, tag="304")]
    pub audio_performer: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="305")]
    pub audio_waveform: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, boxed, tag="306")]
    pub audio_cover: ::core::option::Option<::prost::alloc::boxed::Box<FileView>>,
}
///============ Internal to server =============
///
/// internal to server
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File1 {
    #[prost(fixed64, tag="1")]
    pub file_id: u64,
    #[prost(fixed64, tag="11")]
    pub ref_id: u64,
    #[prost(uint32, tag="12")]
    pub bucket_id: u32,
    #[prost(uint32, tag="2")]
    pub secret: u32,
    /// from client
    #[prost(string, tag="13")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub user_id: u32,
    #[prost(uint32, tag="14")]
    pub file_size: u32,
    #[prost(enumeration="FileType", tag="3")]
    pub file_type: i32,
    #[prost(string, tag="6")]
    pub file_mime: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileImage {
    #[prost(uint32, tag="4")]
    pub width: u32,
    #[prost(uint32, tag="5")]
    pub height: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileVideo {
    #[prost(uint32, tag="4")]
    pub width: u32,
    #[prost(uint32, tag="5")]
    pub height: u32,
    #[prost(uint32, tag="6")]
    pub duration: u32,
    #[prost(message, optional, tag="7")]
    pub cover: ::core::option::Option<File1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioFile {
    #[prost(bool, tag="1")]
    pub is_voice: bool,
    #[prost(uint32, tag="6")]
    pub duration: u32,
    #[prost(string, tag="8")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub performer: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="10")]
    pub waveform: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="7")]
    pub cover: ::core::option::Option<File1>,
}
/// internal to server
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileRef {
    #[prost(fixed64, tag="1")]
    pub file_id: u64,
    #[prost(fixed64, tag="11")]
    pub ref_id: u64,
    ///?
    #[prost(uint32, tag="12")]
    pub bucket_id: u32,
    #[prost(uint32, tag="2")]
    pub secret: u32,
    /// from client
    #[prost(string, tag="13")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub user_id: u32,
    #[prost(uint32, tag="14")]
    pub created_time: u32,
}
/// internal to server
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bucket {
    #[prost(uint32, tag="12")]
    pub bucket_id: u32,
    /// image video avatar music voice image_thumb document
    #[prost(string, tag="3")]
    pub intent: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub is_open: bool,
    #[prost(uint32, tag="14")]
    pub created_time: u32,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FileType {
    FileUnknown = 0,
    FileJpeg = 1,
    FileGif = 2,
    FilePng = 3,
    FileWebp = 4,
    FileMp3 = 5,
    FileMp4 = 6,
    FilePdf = 7,
    FileOther = 8,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNextIdParam {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub start_from: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNextIdResponse {
    #[prost(uint64, tag="1")]
    pub next_id: u64,
    #[prost(bool, tag="2")]
    pub error: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthSendCodeParam {
    /// 989015132134
    #[prost(string, tag="2")]
    pub phone_number: ::prost::alloc::string::String,
    /// 98
    #[prost(string, tag="3")]
    pub country_code: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub resend: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthSendCodeResponse {
    /// a unique code like a session for duration of login or register
    #[prost(string, tag="1")]
    pub hash_code: ::prost::alloc::string::String,
    /// if ture then LogIn otherwise use:create rpc
    #[prost(bool, tag="2")]
    pub phone_registered: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthLogInParam {
    #[prost(string, tag="1")]
    pub hash_code: ::prost::alloc::string::String,
    /// 9015132134 , not needed just for rechecking
    #[prost(string, tag="2")]
    pub phone_number: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub confirm_code: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthLogInResponse {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// A new session for this log in
    #[prost(message, optional, tag="2")]
    pub session: ::core::option::Option<Session>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthLogOutParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthLogOutResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
}
/// CrDU
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCreateChannelParam {
    /// or _name
    #[prost(string, tag="3")]
    pub channel_title: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub about: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCreateChannelResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditChannelParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    #[prost(bool, tag="3")]
    pub set_new_title: bool,
    #[prost(string, tag="4")]
    pub new_title: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub set_new_about: bool,
    #[prost(string, tag="6")]
    pub new_about: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditChannelResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteChannelParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteChannelResponse {
}
/// Author
///
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAddAuthorParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAddAuthorResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeAuthorPermissionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeAuthorPermissionResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveAuthorParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveAuthorResponse {
}
/// Follow
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelFollowChannelParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelFollowChannelResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnFollowChannelParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnFollowChannelResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveFollowersParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveFollowersResponse {
}
/// Subscribe
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscribeParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscribeResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnSubscribeParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnSubscribeResponse {
}
/// NTO NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveSubscribersParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveSubscribersResponse {
}
/// Privacy
///
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangePrivacyParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangePrivacyResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeDefaultPermissionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeDefaultPermissionResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRevokeLinkParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRevokeLinkResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeUsernameParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeUsernameResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBlockProfileParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBlockProfileResponse {
}
/// Messages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendMessageParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    #[prost(message, optional, tag="2")]
    pub message_input: ::core::option::Option<NewMessageInput>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditMessageParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    #[prost(fixed64, tag="4")]
    pub message_gid: u64,
    #[prost(string, tag="2")]
    pub new_text: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteMessagesParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    #[prost(uint64, repeated, tag="3")]
    pub message_gids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteMessagesResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelClearHistoryParam {
    /// NOT NOW
    #[prost(uint64, tag="3")]
    pub from_message_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelClearHistoryResponse {
}
/// Like
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelLikeMessageParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    #[prost(fixed64, tag="2")]
    pub message_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelLikeMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnLikeMessageParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    #[prost(fixed64, tag="2")]
    pub message_gid: u64,
    ///?
    #[prost(fixed64, tag="4")]
    pub like_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnLikeMessageResponse {
}
/// ReShare
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReShareMessageParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    #[prost(fixed64, tag="2")]
    pub message_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReShareMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnReShareMessageParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    #[prost(fixed64, tag="2")]
    pub message_gid: u64,
    #[prost(fixed64, tag="4")]
    pub re_share_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnReShareMessageResponse {
}
/// Comment
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAddCommentParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    #[prost(fixed64, tag="2")]
    pub message_gid: u64,
    #[prost(string, tag="4")]
    pub comment_text: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAddCommentResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteCommentParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    #[prost(fixed64, tag="2")]
    pub message_gid: u64,
    #[prost(fixed64, tag="4")]
    pub comment_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteCommentResponse {
}
/// Pin
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPinMessageParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPinMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnPinMessageParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnPinMessageResponse {
}
/// Avatar
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarAddParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
    #[prost(uint64, tag="2")]
    pub file_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarAddResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarChangeParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarChangeResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarDeleteParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarDeleteResponse {
}
/// Others
///
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendDoingActionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendDoingActionResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportChannelParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportChannelResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportMessageParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportMessageResponse {
}
/// Views
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetFullParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetFullResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetMessagesListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetMessagesListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetMediaListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetMediaListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetAuthorsParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetAuthorsResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetFollowersParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetFollowersResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetFollowingsParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetFollowingsResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetSubscribersParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetSubscribersResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBlockedParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBlockedResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarGetListParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarGetListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetInboxParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGetInboxResponse {
}
/// todo : clean profile_cid
/// crud
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteChatParam {
    #[prost(uint32, tag="1")]
    pub profile_id: u32,
    /// both - many
    #[prost(uint64, tag="2")]
    pub chat_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteChatResponse {
}
// Members

// Privacy

// Messages

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSendMessageParam {
    ///    uint64 profile_cid = 1;
    #[prost(uint64, tag="2")]
    pub chat_gid: u64,
    #[prost(message, optional, tag="3")]
    pub message_input: ::core::option::Option<NewMessageInput>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSendMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatEditMessageParam {
    #[prost(uint64, tag="2")]
    pub chat_gid: u64,
    #[prost(fixed64, tag="4")]
    pub message_gid: u64,
    ///    uint32 by_profile_cid = 3;
    #[prost(string, tag="6")]
    pub new_text: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatEditMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteMessagesParam {
    ///    uint64 profile_cid = 1;
    #[prost(uint64, tag="2")]
    pub chat_gid: u64,
    #[prost(uint64, repeated, tag="3")]
    pub message_gids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteMessagesResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteHistoryParam {
    ///    uint64 profile_cid = 1;
    #[prost(uint64, tag="2")]
    pub chat_gid: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteHistoryResponse {
}
/// Notifications
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSetNotificationParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSetNotificationResponse {
}
// Others

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatAvatarChangeParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatAvatarChangeResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSendDoingActionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSendDoingActionResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatReportChatParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatReportChatResponse {
}
// Views

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetFullMessageParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetFullMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetMessagesListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetMessagesListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetMediaListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetMediaListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetInboxParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatGetInboxResponse {
}
/// CrDU
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateGroupParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
    #[prost(uint32, tag="2")]
    pub creator_profile_id: u32,
    /// or _name
    #[prost(string, tag="3")]
    pub group_title: ::prost::alloc::string::String,
    /// if older message is is viewable for new members > now true for all new groups
    #[prost(bool, tag="8")]
    pub history_viewable: bool,
    /// view history without joining
    #[prost(bool, tag="9")]
    pub force_join: bool,
    /// global searchable in bar > force_join must be false + history_viewable = ture
    #[prost(bool, tag="17")]
    pub global_search: bool,
    #[prost(string, tag="15")]
    pub about: ::prost::alloc::string::String,
    /// Not Now
    ///
    /// with "group" suffix > not now > all is private now
    #[prost(string, tag="4")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateGroupResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditGroupParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
    #[prost(uint32, tag="2")]
    pub by_profile_id: u32,
    #[prost(bool, tag="3")]
    pub set_new_title: bool,
    #[prost(string, tag="4")]
    pub new_title: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub set_new_about: bool,
    /// Add others editable options
    #[prost(string, tag="6")]
    pub new_about: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditGroupResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteGroupParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
    #[prost(uint32, tag="2")]
    pub by_profile_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteGroupResponse {
}
/// Admins Members Actions
///
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddAdminParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddAdminResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRemoveMemberParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
    #[prost(uint32, tag="2")]
    pub by_profile_id: u32,
    #[prost(uint32, tag="3")]
    pub member_profile_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRemoveMemberResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberLevelParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberLevelResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberPermissionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberPermissionResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupBanMemberParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
    #[prost(uint32, tag="2")]
    pub by_profile_id: u32,
    #[prost(uint32, tag="3")]
    pub banned_profile_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupBanMemberResponse {
}
/// Members Actions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinGroupParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
    #[prost(uint32, tag="3")]
    pub new_member_profile_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinGroupResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupLeaveGroupParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
    #[prost(uint32, tag="3")]
    pub member_profile_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupLeaveGroupResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddMemberParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
    #[prost(uint32, tag="2")]
    pub by_profile_id: u32,
    #[prost(uint32, tag="3")]
    pub new_member_profile_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddMemberResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSetNotificationParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSetNotificationResponse {
}
/// Privacy
///
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangePrivacyParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangePrivacyResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeDefaultPermissionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeDefaultPermissionResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRevokeLinkParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRevokeLinkResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeUsernameParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeUsernameResponse {
}
/// Messages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendMessageParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
    #[prost(message, optional, tag="2")]
    pub message_input: ::core::option::Option<NewMessageInput>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditMessageParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
    #[prost(uint32, tag="3")]
    pub by_profile_id: u32,
    #[prost(string, tag="2")]
    pub new_text: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupPinMessageParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupPinMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupUnPinMessageParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupUnPinMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteMessageParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteMessagesParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
    #[prost(uint32, tag="2")]
    pub by_profile_id: u32,
    #[prost(uint64, repeated, tag="3")]
    pub message_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteMessagesResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteHistoryParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteHistoryResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupClearHistoryParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupClearHistoryResponse {
}
/// Others
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarAddParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarAddResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarChangeParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarChangeResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarDeleteParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarDeleteResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarGetListParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarGetListResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendDoingActionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendDoingActionResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupReportGroupParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupReportGroupResponse {
}
/// Views
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetFullParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetFullResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMessagesListParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMessagesListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMediaListParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMediaListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMembersListParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMembersListResponse {
}
/// NOT NOW
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetAdminsListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetAdminsListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetInboxParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetInboxResponse {
}
/// Dep
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSetDraftParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSetDraftResponse {
}
// Add Profile

/// Settings
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileSetSettingsParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileSetSettingsResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUsers1Param {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUsers1Response {
    #[prost(message, repeated, tag="1")]
    pub users: ::prost::alloc::vec::Vec<Profile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProfilesParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProfilesResponse {
    #[prost(message, repeated, tag="1")]
    pub profiles: ::prost::alloc::vec::Vec<Profile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelsParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelsResponse {
    #[prost(message, repeated, tag="1")]
    pub channels: ::prost::alloc::vec::Vec<Channel>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectsParam {
}
///repeated Direct directs = 1;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectsResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessagesParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessagesResponse {
    #[prost(message, repeated, tag="1")]
    pub directs: ::prost::alloc::vec::Vec<Message>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedEchoParam {
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedEchoResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
    #[prost(string, tag="2")]
    pub text: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedCheckUserNameParam {
    #[prost(string, tag="1")]
    pub username: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedCheckUserNameResponse {
    #[prost(bool, tag="1")]
    pub is_available: bool,
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub show_message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Param {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
    #[prost(string, tag="2")]
    pub text: ::prost::alloc::string::String,
}
/// CrUd
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRegisterUserParam {
    #[prost(string, tag="2")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub phone_number: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub hash_code: ::prost::alloc::string::String,
    #[prost(string, tag="30")]
    pub confirm_code: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRegisterUserResponse {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// A new session for this log in
    #[prost(message, optional, tag="2")]
    pub session: ::core::option::Option<Session>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEditUserParam {
    #[prost(bool, tag="3")]
    pub set_new_name: bool,
    #[prost(string, tag="4")]
    pub new_first_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub new_last_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEditUserResponse {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
/// Delete User
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteSendCodeParam {
    #[prost(bool, tag="1")]
    pub resend: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteSendCodeResponse {
    #[prost(string, repeated, tag="4")]
    pub sms_numbers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteConfirmCodeParam {
    ///4 digit 4215
    #[prost(string, tag="3")]
    pub code: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteConfirmCodeResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteUserParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeleteUserResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
}
/// Phone
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserChangePhoneNumberParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserChangePhoneNumberResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
    #[prost(string, tag="2")]
    pub text: ::prost::alloc::string::String,
}
// Email

// Password

/// Session
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRemoveSessionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRemoveSessionResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRemoveOtherParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRemoveOtherResponse {
}
/// Views
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGetMeParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGetMeResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGetActiveSessionsParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGetActiveSessionsResponse {
}
