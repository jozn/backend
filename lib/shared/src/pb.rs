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
    ///  bool is_response = 3; // imut // dep
    ///
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
    ///  bool is_response = 3; // imut
    ///
    /// imut
    #[prost(bytes, tag = "4")]
    pub rpc_data: std::vec::Vec<u8>,
}
///==================== User ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(fixed64, tag = "1")]
    pub gid: u64,
    #[prost(uint32, tag = "2")]
    pub profile_cid: u32,
    /// From session
    #[prost(uint64, tag = "3")]
    pub device_id: u64,
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
    #[prost(uint32, tag = "1")]
    pub cid: u32,
    #[prost(string, tag = "14")]
    pub phone: std::string::String,
    #[prost(string, tag = "15")]
    pub email: std::string::String,
    #[prost(string, tag = "17")]
    pub password_hash: std::string::String,
    #[prost(string, tag = "18")]
    pub password_salt: std::string::String,
    #[prost(uint32, tag = "36")]
    pub created_time: u32,
    #[prost(uint32, tag = "37")]
    pub version_time: u32,
    #[prost(bool, tag = "38")]
    pub is_deleted: bool,
    #[prost(bool, tag = "39")]
    pub is_banned: bool,
    #[prost(message, optional, tag = "1114")]
    pub def_profile: ::std::option::Option<Profile>,
    #[prost(message, repeated, tag = "110")]
    pub profiles: ::std::vec::Vec<Profile>,
    #[prost(message, repeated, tag = "113")]
    pub stores: ::std::vec::Vec<Store>,
    #[prost(message, repeated, tag = "102")]
    pub sessions: ::std::vec::Vec<Session>,
    #[prost(message, optional, tag = "111")]
    pub shopping_profile: ::std::option::Option<ShoppingProfile>,
    #[prost(string, tag = "4")]
    pub first_name: std::string::String,
    #[prost(string, tag = "5")]
    pub last_name: std::string::String,
    /// olds
    ///
    ///???
    #[prost(message, optional, tag = "80")]
    pub user_counts: ::std::option::Option<Channel>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCounts {
    /// Number of created channels
    #[prost(uint32, tag = "1")]
    pub created_channels: u32,
}
///==================== Profile ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Profile {
    /// imut - profile_cid
    #[prost(uint32, tag = "1")]
    pub cid: u32,
    /// imut
    #[prost(uint32, tag = "2")]
    pub user_cid: u32,
    /// imut
    #[prost(message, optional, tag = "100")]
    pub primary_channel: ::std::option::Option<Channel>,
    /// imut
    #[prost(message, optional, tag = "109")]
    pub saved_channel: ::std::option::Option<SavedChannel>,
    /// imut
    #[prost(uint32, tag = "103")]
    pub created_time: u32,
    /// imut
    #[prost(message, optional, tag = "107")]
    pub setting: ::std::option::Option<ProfileSettings>,
    /// Demonstration: collections > not really present > big > fetch with api
    ///
    /// mut
    #[prost(message, repeated, tag = "104")]
    pub channels: ::std::vec::Vec<Channel>,
    /// mut
    #[prost(message, repeated, tag = "105")]
    pub directs: ::std::vec::Vec<Direct>,
    /// mut
    #[prost(message, repeated, tag = "106")]
    pub groups: ::std::vec::Vec<Group>,
    /// mut
    #[prost(message, repeated, tag = "108")]
    pub contacts: ::std::vec::Vec<Contact>,
    /// Views
    ///
    /// When sending profile to others set this if they have this profile in their contacts
    #[prost(message, optional, tag = "111")]
    pub contact_info: ::std::option::Option<Contact>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileSettings {
    /// imut - ???
    #[prost(uint32, tag = "1")]
    pub profile_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Direct {
    #[prost(fixed64, tag = "1")]
    pub gid: u64,
    #[prost(uint32, tag = "5")]
    pub profile_cid: u32,
    #[prost(enumeration = "DirectTypeEnum", tag = "102")]
    pub direct_type: i32,
    #[prost(string, tag = "9")]
    pub custom_title: std::string::String,
    #[prost(fixed64, tag = "10")]
    pub pin_time_ms: u64,
    #[prost(uint32, tag = "12")]
    pub unseen_count: u32,
    #[prost(uint32, tag = "13")]
    pub seq: u32,
    #[prost(bool, tag = "22")]
    pub is_active: bool,
    #[prost(uint32, tag = "29")]
    pub mute_until: u32,
    #[prost(uint32, tag = "33")]
    pub created_time: u32,
    #[prost(fixed64, tag = "45")]
    pub sort_time_ms: u64,
    #[prost(fixed64, tag = "104")]
    pub sync_time_ms: u64,
    #[prost(fixed64, tag = "16")]
    pub my_last_seen_seq: u64,
    #[prost(fixed64, tag = "17")]
    pub my_last_seen_msg_id: u64,
    #[prost(uint32, tag = "108")]
    pub pined_msgs_count: u32,
    #[prost(fixed64, tag = "11")]
    pub visible_from_msg_gid: u64,
    #[prost(message, optional, tag = "48")]
    pub channel: ::std::option::Option<Channel>,
    /// ? must use profile
    #[prost(message, optional, tag = "49")]
    pub contact: ::std::option::Option<Contact>,
    ///  Profile profile = 149;
    #[prost(message, optional, tag = "50")]
    pub group: ::std::option::Option<Group>,
    #[prost(message, optional, tag = "25")]
    pub last_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag = "26")]
    pub pinned_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag = "43")]
    pub group_member: ::std::option::Option<GroupMember>,
    #[prost(message, optional, tag = "46")]
    pub draft: ::std::option::Option<DirectDraft>,
    #[prost(message, optional, tag = "47")]
    pub custom_notification: ::std::option::Option<DirectCustomNotification>,
    /// dep - del all below
    /// For chat
    ///
    /// ??
    #[prost(uint32, tag = "6")]
    pub peer_profile_cid: u32,
    /// ??
    #[prost(fixed64, tag = "18")]
    pub peer_last_seen_msg_id: u64,
    /// ??
    #[prost(fixed64, tag = "19")]
    pub my_last_delivered_seq: u64,
    /// ??
    #[prost(fixed64, tag = "20")]
    pub my_last_delivered_msg_id: u64,
    /// ??
    #[prost(fixed64, tag = "21")]
    pub peer_last_delivered_msg_id: u64,
    /// For channel
    ///
    /// ??
    #[prost(uint32, tag = "41")]
    pub peer_channel_cid: u32,
    /// For group
    ///
    /// ??
    #[prost(uint32, tag = "7")]
    pub group_cid: u32,
    /// ??
    #[prost(message, optional, tag = "44")]
    pub avatar: ::std::option::Option<FileMsg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDraft {
    #[prost(string, tag = "34")]
    pub draft_text: std::string::String,
    #[prost(int64, tag = "35")]
    pub drat_reply_to_msg_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectCustomNotification {
    #[prost(bool, tag = "13")]
    pub alert: bool,
    #[prost(bool, tag = "14")]
    pub preview: bool,
    #[prost(bool, tag = "15")]
    pub led_on: bool,
    #[prost(bool, tag = "16")]
    pub led_color: bool,
    #[prost(bool, tag = "17")]
    pub vibrate: bool,
    #[prost(bool, tag = "18")]
    pub popup: bool,
    #[prost(bool, tag = "19")]
    pub sound: bool,
    #[prost(bool, tag = "20")]
    pub priority: bool,
}
///==================== Messages ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
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
    #[prost(uint32, tag = "17")]
    pub edited_time: u32,
    /// imut
    #[prost(uint32, tag = "18")]
    pub created_time: u32,
    /// imut - verified system messages - 2 bytes tag
    #[prost(bool, tag = "111")]
    pub verified: bool,
    #[prost(enumeration = "MessageDeliveryStatues", tag = "105")]
    pub delivery_status: i32,
    /// mut-once
    #[prost(uint32, tag = "106")]
    pub delivery_time: u32,
    /// maybe temp with a purge period - 2 bytes
    #[prost(bool, tag = "150")]
    pub deleted: bool,
    #[prost(uint32, tag = "112")]
    pub flags: u32,
    /// forward is always live object from other channels, but not from other chats, groups
    #[prost(message, optional, boxed, tag = "16")]
    pub forward: ::std::option::Option<::std::boxed::Box<Message>>,
    #[prost(message, optional, boxed, tag = "50")]
    pub reply_to: ::std::option::Option<::std::boxed::Box<Message>>,
    /// For channels, stores
    ///
    /// For videos
    #[prost(string, tag = "109")]
    pub title: std::string::String,
    #[prost(message, optional, tag = "101")]
    pub counts: ::std::option::Option<MessageCount>,
    #[prost(message, optional, tag = "102")]
    pub setting: ::std::option::Option<MessageSetting>,
    #[prost(message, optional, tag = "110")]
    pub product: ::std::option::Option<Product>,
    #[prost(message, repeated, tag = "103")]
    pub files: ::std::vec::Vec<FileMsg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageCount {
    ///??
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
    #[prost(uint32, tag = "1")]
    pub cid: u32,
    #[prost(string, tag = "2")]
    pub user_name: std::string::String,
    /// For default channels take if from User first and lastname.
    #[prost(string, tag = "3")]
    pub channel_name: std::string::String,
    #[prost(uint32, tag = "7")]
    pub creator_profile_cid: u32,
    #[prost(bool, tag = "6")]
    pub is_verified: bool,
    #[prost(bool, tag = "101")]
    pub is_profile_channel: bool,
    #[prost(int64, tag = "40")]
    pub avatar_count: i64,
    #[prost(string, tag = "16")]
    pub about: std::string::String,
    #[prost(string, tag = "17")]
    pub invite_link_hash: std::string::String,
    #[prost(uint32, tag = "19")]
    pub message_seq: u32,
    /// version
    #[prost(fixed64, tag = "21")]
    pub sync_time_ms: u64,
    #[prost(uint32, tag = "36")]
    pub created_time: u32,
    /// conslidate in visivlity
    #[prost(uint32, tag = "38")]
    pub is_deleted: u32,
    #[prost(uint32, tag = "39")]
    pub is_banned: u32,
    /// for owner
    #[prost(message, optional, tag = "90")]
    pub notification_setting: ::std::option::Option<ChannelOwnerNotification>,
    #[prost(enumeration = "ChannelPrivacy", tag = "9")]
    pub privacy: i32,
    #[prost(message, optional, tag = "41")]
    pub counts: ::std::option::Option<ChannelCounts>,
    #[prost(message, optional, tag = "25")]
    pub last_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag = "26")]
    pub pinned_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag = "100")]
    pub avatar: ::std::option::Option<FileMsg>,
}
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOwnerNotification {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCounts {
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
///==================== Store ==================
///
///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShoppingProfile {}
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductPriceInfo {
    #[prost(uint32, tag = "1")]
    pub price: u32,
    #[prost(uint32, tag = "6")]
    pub discount_price: u32,
    /// from 1000
    #[prost(uint32, tag = "3")]
    pub rate: u32,
}
///==================== Saved ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedChannel {
    #[prost(uint32, tag = "1")]
    pub cid: u32,
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
    pub counts: ::std::option::Option<ChannelCounts>,
    #[prost(message, optional, tag = "25")]
    pub last_message: ::std::option::Option<Message>,
}
///==================== Group ==================
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    #[prost(uint32, tag = "1")]
    pub cid: u32,
    /// or _name
    #[prost(string, tag = "3")]
    pub group_title: std::string::String,
    /// with "group" suffix > not now > all is private now
    #[prost(string, tag = "4")]
    pub user_name: std::string::String,
    #[prost(uint32, tag = "7")]
    pub creator_profile_cid: u32,
    ///  GroupPrivacy group_privacy = 8;
    #[prost(bool, tag = "8")]
    pub history_viewable: bool,
    /// global searchable, view history without joining
    #[prost(bool, tag = "9")]
    pub is_open_group: bool,
    #[prost(uint32, tag = "10")]
    pub seq: u32,
    #[prost(uint32, tag = "14")]
    pub avatar_count: u32,
    #[prost(string, tag = "15")]
    pub about: std::string::String,
    #[prost(string, tag = "16")]
    pub invite_link_hash: std::string::String,
    ///Move to counts?
    #[prost(uint32, tag = "17")]
    pub members_count: u32,
    #[prost(uint32, tag = "18")]
    pub admins_count: u32,
    #[prost(uint32, tag = "19")]
    pub moderator_counts: u32,
    #[prost(fixed64, tag = "20")]
    pub sort_time: u64,
    #[prost(fixed64, tag = "40")]
    pub sync_time: u64,
    #[prost(uint32, tag = "21")]
    pub created_time: u32,
    #[prost(bool, tag = "23")]
    pub is_deleted: bool,
    #[prost(bool, tag = "24")]
    pub is_banned: bool,
    #[prost(message, optional, tag = "25")]
    pub last_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag = "26")]
    pub pinned_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag = "27")]
    pub avatar: ::std::option::Option<FileMsg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCounts {
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMember {
    #[prost(int64, tag = "1")]
    pub gid: i64,
    #[prost(int64, tag = "2")]
    pub group_cid: i64,
    #[prost(uint32, tag = "3")]
    pub profile_cid: u32,
    #[prost(uint32, tag = "4")]
    pub by_profile_cid: u32,
    #[prost(bool, tag = "5")]
    pub is_moderator: bool,
    #[prost(uint32, tag = "6")]
    pub created_time: u32,
}
//==================== Others ==================

///todo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileMsg {
    #[prost(fixed64, tag = "1")]
    pub gid: u64,
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
/////////////////////////// Docs ///////////////////////////

//
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
///==================== Direct ==================
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DirectTypeEnum {
    Deo = 0,
    Profile = 1,
    Channel = 2,
    Group = 3,
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
pub struct SendConfirmCodeParam {
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
pub struct SendConfirmCodeResponse {
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
pub struct ConfirmCodeParam {
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
pub struct ConfirmCodeResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    ///    SelfUserView SelfUserView = 3; //if it is login
    #[prost(string, tag = "2")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingUpParam {
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
pub struct SingUpResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    ///    SelfUserView SelfUserView = 3;
    #[prost(string, tag = "2")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingInParam {
    #[prost(string, tag = "4")]
    pub user_name_phone_email: std::string::String,
    /// 98... 989015132328
    #[prost(string, tag = "5")]
    pub password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingInResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    ///    SelfUserView SelfUserView = 3;
    #[prost(string, tag = "2")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogOutParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogOutResponse {
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarAddParam {
    #[prost(uint32, tag = "1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarAddResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarDeleteParam {
    #[prost(uint32, tag = "1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarDeleteResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarGetListParam {
    #[prost(uint32, tag = "1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarGetListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPinMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPinMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnPinMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnPinMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteMessageParam {
    #[prost(uint32, tag = "1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSetDraftParam {
    #[prost(uint32, tag = "1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSetDraftResponse {}
/// CrDU
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCreateChannelParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCreateChannelResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditChannelParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditChannelResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteChannelParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteChannelResponse {}
/// Author
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAddAuthorParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAddAuthorResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeAuthorPermissionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeAuthorPermissionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveAuthorParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveAuthorResponse {}
/// Follow
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelFollowChannelParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelFollowChannelResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnFollowChannelParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnFollowChannelResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveFollowersParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveFollowersResponse {}
// Subscribe

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscribeParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscribeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnSubscribeParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnSubscribeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveSubscribersParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveSubscribersResponse {}
// Privacy

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangePrivacyParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangePrivacyResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeDefaultPermissionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeDefaultPermissionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRevokeLinkParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRevokeLinkResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeUsernameParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeUsernameResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBlockChannelParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBlockChannelResponse {}
// Messages

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteMessagesParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteMessagesResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelClearHistoryParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelClearHistoryResponse {}
// Others

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarChangeParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarChangeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendDoingActionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendDoingActionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportChannelParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportChannelResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportMessageParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportMessageResponse {}
// Views

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
// crud

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
//========= One =========
// CRUD

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDeleteDirectParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDeleteDirectResponse {}
// Pin, Archives, Marks

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectChangeTitleParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectChangeTitleResponse {}
// Notifications

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSetCustomNotificationParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSetCustomNotificationResponse {}
// Others

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSendActionDoingParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSendActionDoingResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSetDraftParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSetDraftResponse {}
//========= One End =========

///========= Many =========
/// CRUD
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDeleteDirectsParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDeleteDirectsResponse {}
/// = Pin, Archives, Marks
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMarkAsReadParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMarkAsReadResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMarkAsUnReadParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMarkAsUnReadResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectPinDirectsParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectPinDirectsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUnPinDirectsParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUnPinDirectsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectArchiveDirectsParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectArchiveDirectsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUnArchiveDirectsParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUnArchiveDirectsResponse {}
// Messages

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectClearHistoriesParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectClearHistoriesResponse {}
// Notifications

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMuteDirectsParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMuteDirectsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUnMuteDirectsParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUnMuteDirectsResponse {}
//========= Many End =========

// Folders

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectCreateFolderParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectCreateFolderResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectChangeFolderParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectChangeFolderResponse {}
//
//message Param {
//}
//
//message Response {
//}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectRemoveFromFolderParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectRemoveFromFolderResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectReordersFolderParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectReordersFolderResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDeleteFolderParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDeleteFolderResponse {}
// Views

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetChatsListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetChatsListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetGroupsListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetGroupsListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetChannelsListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetChannelsListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetFoldersListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetFoldersListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetFoldersFullListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetFoldersFullListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarAddParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarAddResponse {}
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
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarGetListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendMessageParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditMessageParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
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
pub struct GroupSetDraftParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSetDraftResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetFullMessageParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetFullMessageResponse {}
/// CrDU
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateGroupParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateGroupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditGroupParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditGroupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteGroupParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteGroupResponse {}
// Members

//message Param {
//}
//
//message Response {
//}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddAdminParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddAdminResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddMemberParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddMemberResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRemoveMemberParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRemoveMemberResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberLevelParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberLevelResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberPermissionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberPermissionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinGroupParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinGroupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupLeaveGroupParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupLeaveGroupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupBanMemberParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupBanMemberResponse {}
// Privacy

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangePrivacyParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangePrivacyResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeDefaultPermissionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeDefaultPermissionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRevokeLinkParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRevokeLinkResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeUsernameParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeUsernameResponse {}
// Messages

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteMessagesParam {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteMessagesResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteHistoryParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteHistoryResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupClearHistoryParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupClearHistoryResponse {}
// Others

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarChangeParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarChangeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendDoingActionParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendDoingActionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupReportGroupParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupReportGroupResponse {}
// Views

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMessagesListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMessagesListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMediaListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMediaListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMembersListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMembersListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetAdminsListParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetAdminsListResponse {}
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectsResponse {
    #[prost(message, repeated, tag = "1")]
    pub directs: ::std::vec::Vec<Direct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessagesParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessagesResponse {
    #[prost(message, repeated, tag = "1")]
    pub directs: ::std::vec::Vec<Message>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoParam {
    #[prost(string, tag = "1")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    #[prost(string, tag = "2")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckUserNameParam {
    #[prost(string, tag = "1")]
    pub username: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckUserNameResponse {
    #[prost(bool, tag = "1")]
    pub is_available: bool,
    #[prost(string, tag = "2")]
    pub username: std::string::String,
    #[prost(string, tag = "3")]
    pub show_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    #[prost(string, tag = "2")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePhoneNumberParam {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePhoneNumberResponse {
    #[prost(bool, tag = "1")]
    pub done: bool,
    #[prost(string, tag = "2")]
    pub text: std::string::String,
}
