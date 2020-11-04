//////////////////////////// Common Types ////////////////////
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Invoke {
    #[prost(uint32, tag="6")]
    pub namespace: u32,
    #[prost(uint32, tag="1")]
    pub method: u32,
    #[prost(uint64, tag="2")]
    pub action_id: u64,
    #[prost(bool, tag="3")]
    pub is_response: bool,
    #[prost(bytes, tag="4")]
    pub rpc_data: std::vec::Vec<u8>,
}
// next: user relations, shop, product, message log, store, file
// next2: message integrations: product, payment, contacts,...
// gid = unique nano second time;
// sid = scoped id; for future - bot platforms
// cid = common id - seqentaily increase id for user, channels, groups, shops,..

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(uint32, tag="2")]
    pub by_user_cid: u32,
    #[prost(uint32, tag="100")]
    pub by_channel_cid: u32,
    #[prost(uint32, tag="3")]
    pub post_type: u32,
    #[prost(int64, tag="4")]
    pub media_id: i64,
    #[prost(int64, tag="5")]
    pub file_ref_id: i64,
    #[prost(string, tag="6")]
    pub post_key: std::string::String,
    #[prost(string, tag="7")]
    pub text: std::string::String,
    #[prost(string, tag="8")]
    pub rich_text: std::string::String,
    #[prost(string, tag="109")]
    pub title: std::string::String,
    ///  uint32 media_count = 9;
    #[prost(uint32, tag="10")]
    pub shared_to: u32,
    #[prost(uint32, tag="12")]
    pub via: u32,
    #[prost(uint32, tag="13")]
    pub seq: u32,
    #[prost(uint64, tag="108")]
    pub version_time: u64,
    #[prost(uint32, tag="17")]
    pub edited_time: u32,
    #[prost(uint32, tag="18")]
    pub created_time: u32,
    #[prost(bool, tag="111")]
    pub verified: bool,
    #[prost(enumeration="MessageDeliveryStatues", tag="105")]
    pub delivery_status: i32,
    #[prost(uint32, tag="106")]
    pub delivery_time: u32,
    #[prost(uint64, tag="180")]
    pub previous_message_id: u64,
    ///??
    #[prost(bool, tag="15")]
    pub deleted: bool,
    #[prost(uint32, tag="112")]
    pub flags: u32,
    /// forward is always live object from other channels, but not from other chats, groups
    #[prost(message, optional, boxed, tag="16")]
    pub forward: ::std::option::Option<::std::boxed::Box<Message>>,
    #[prost(message, optional, boxed, tag="50")]
    pub reply_to: ::std::option::Option<::std::boxed::Box<Message>>,
    #[prost(message, optional, tag="101")]
    pub counts: ::std::option::Option<MessageCount>,
    #[prost(message, optional, tag="102")]
    pub setting: ::std::option::Option<MessageSetting>,
    #[prost(message, optional, tag="110")]
    pub product: ::std::option::Option<Product>,
    #[prost(message, repeated, tag="103")]
    pub files: ::std::vec::Vec<FileMsg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageCount {
    ///??
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
    pub by_user_cid: u32,
    #[prost(uint32, tag="50")]
    pub by_channel_cid: u32,
    #[prost(uint32, tag="3")]
    pub target_user_cid: u32,
    #[prost(uint32, tag="4")]
    pub target_channel_cid: u32,
    #[prost(message, optional, tag="11")]
    pub target_channel_view: ::std::option::Option<Channel>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    #[prost(uint32, tag="1")]
    pub cid: u32,
    #[prost(string, tag="2")]
    pub user_name: std::string::String,
    /// title
    #[prost(string, tag="4")]
    pub first_name: std::string::String,
    #[prost(string, tag="5")]
    pub last_name: std::string::String,
    ///  string channel_title = 3;
    #[prost(uint32, tag="7")]
    pub creator_user_cid: u32,
    #[prost(uint32, tag="6")]
    pub is_verified: u32,
    #[prost(int64, tag="40")]
    pub avatar_count: i64,
    ///  uint32 access_hash = 8;
    #[prost(string, tag="16")]
    pub about: std::string::String,
    #[prost(string, tag="17")]
    pub invite_link_hash: std::string::String,
    #[prost(uint32, tag="19")]
    pub post_seq: u32,
    ///  uint32 last_post_time = 35;
    #[prost(fixed64, tag="20")]
    pub sort_time_ms: u64,
    #[prost(fixed64, tag="21")]
    pub sync_time_ms: u64,
    #[prost(uint32, tag="36")]
    pub created_time: u32,
    /// conslidate in visivlity
    #[prost(uint32, tag="38")]
    pub is_deleted: u32,
    #[prost(uint32, tag="39")]
    pub is_banned: u32,
    /// for owner
    #[prost(message, optional, tag="90")]
    pub notification_setting: ::std::option::Option<ChannelNotificationSetting>,
    #[prost(enumeration="ChannelPrivacy", tag="9")]
    pub privacy: i32,
    #[prost(enumeration="ChannelType", tag="42")]
    pub channel_type: i32,
    #[prost(message, optional, tag="41")]
    pub counts: ::std::option::Option<ChannelCounts>,
    #[prost(message, optional, tag="25")]
    pub last_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag="26")]
    pub pinned_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag="100")]
    pub avatar: ::std::option::Option<Message>,
}
/// This channel or profile?
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPrivacySetting {
    #[prost(uint32, tag="10")]
    pub online_privacy: u32,
    #[prost(uint32, tag="11")]
    pub call_privacy: u32,
    #[prost(uint32, tag="12")]
    pub add_to_group_privacy: u32,
    #[prost(uint32, tag="13")]
    pub seen_message_privacy: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelNotificationSetting {
    #[prost(bool, tag="2")]
    pub social_led_on: bool,
    #[prost(string, tag="3")]
    pub social_led_color: std::string::String,
    #[prost(bool, tag="4")]
    pub request_to_follow_you: bool,
    #[prost(bool, tag="5")]
    pub followed_channel: bool,
    #[prost(bool, tag="6")]
    pub accepted_channel_follow_request: bool,
    #[prost(bool, tag="7")]
    pub channel_message_liked: bool,
    #[prost(bool, tag="8")]
    pub channel_message_commented: bool,
    #[prost(bool, tag="9")]
    pub mentioned_channel_in_message: bool,
    #[prost(bool, tag="10")]
    pub mentioned_channel_in_comment: bool,
    #[prost(bool, tag="11")]
    pub contacts_joined: bool,
    #[prost(bool, tag="12")]
    pub direct_message: bool,
    #[prost(bool, tag="13")]
    pub direct_alert: bool,
    #[prost(bool, tag="14")]
    pub direct_preview: bool,
    #[prost(bool, tag="15")]
    pub direct_led_on: bool,
    #[prost(bool, tag="16")]
    pub direct_led_color: bool,
    #[prost(bool, tag="17")]
    pub direct_vibrate: bool,
    #[prost(bool, tag="18")]
    pub direct_popup: bool,
    #[prost(bool, tag="19")]
    pub direct_sound: bool,
    #[prost(bool, tag="20")]
    pub direct_priority: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCounts {
    #[prost(uint32, tag="20")]
    pub followers_count: u32,
    #[prost(uint32, tag="21")]
    pub following_count: u32,
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Store {
    #[prost(string, tag="1")]
    pub address: std::string::String,
    #[prost(string, tag="2")]
    pub phone: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    /// seq
    #[prost(uint32, tag="1")]
    pub product_id: u32,
    #[prost(uint32, tag="53")]
    pub category_id: u32,
    #[prost(string, tag="50")]
    pub category: std::string::String,
    #[prost(string, tag="51")]
    pub brand: std::string::String,
    ///from 10_000 - 5% > 500
    #[prost(uint32, tag="3")]
    pub fee_rate: u32,
    #[prost(uint32, tag="5")]
    pub sales_count: u32,
    #[prost(uint32, tag="6")]
    pub price: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductPriceInfo {
    #[prost(uint32, tag="1")]
    pub price: u32,
    #[prost(uint32, tag="6")]
    pub discount_price: u32,
    /// from 1000
    #[prost(uint32, tag="3")]
    pub rate: u32,
}
/// Reconsider: we do not redistribute this info anymore; why we have it?
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(uint32, tag="2")]
    pub actor_user_cid: u32,
    #[prost(uint32, tag="50")]
    pub actor_channel_cid: u32,
    /// todo to enum
    #[prost(uint32, tag="3")]
    pub action_type: u32,
    #[prost(uint32, tag="4")]
    pub on_user_cid: u32,
    #[prost(fixed64, tag="9")]
    pub on_channel_cid: u64,
    ///post
    #[prost(fixed64, tag="5")]
    pub message_gid: u64,
    #[prost(fixed64, tag="6")]
    pub comment_gid: u64,
    ///Murmur
    #[prost(int64, tag="7")]
    pub hash_murm64: i64,
    #[prost(uint32, tag="8")]
    pub created_time: u32,
}
pub mod action {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ActionType {
        UnknownAt = 0,
        Liked = 1,
        Followed = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blocked {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(uint32, tag="2")]
    pub user_cid: u32,
    /// real active
    #[prost(uint32, tag="6")]
    pub blocked_user_cid: u32,
    /// for show to blocker
    #[prost(uint32, tag="7")]
    pub blocked_channel_cid: u32,
    #[prost(uint32, tag="5")]
    pub created_time: u32,
}
/// just by channels for now
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(uint32, tag="9")]
    pub channel_cid: u32,
    /// user channel store - now just channel
    #[prost(uint32, tag="2")]
    pub by_cast_cid: u32,
    ///  Cast cast_type = 8;
    ///
    /// post
    #[prost(fixed64, tag="3")]
    pub message_gid: u64,
    #[prost(string, tag="4")]
    pub text: std::string::String,
    #[prost(uint32, tag="5")]
    pub likes_count: u32,
    #[prost(uint32, tag="6")]
    pub edit_time: u32,
    #[prost(uint32, tag="7")]
    pub created_time: u32,
}
/// just by channels
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Followed {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(uint32, tag="2")]
    pub by_user_cid: u32,
    #[prost(uint32, tag="10")]
    pub by_channel_cid: u32,
    #[prost(uint32, tag="3")]
    pub target_cid: u32,
    #[prost(uint32, tag="11")]
    pub target_channel_id: u32,
    ///  Cast cast_type = 8;
    #[prost(uint32, tag="4")]
    pub created_time: u32,
}
/// like is reserved in RDMS(mysql)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reaction {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(int64, tag="2")]
    pub for_message_cid: i64,
    #[prost(int64, tag="11")]
    pub for_channel_cid: i64,
    #[prost(uint32, tag="3")]
    pub by_user_cid: u32,
    #[prost(uint32, tag="10")]
    pub by_channel_cid: u32,
    ///  Cast cast_type = 8;
    ///  uint32 message_type = 4;
    #[prost(uint32, tag="5")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(uint32, tag="2")]
    pub user_cid: u32,
    #[prost(uint32, tag="10")]
    pub channel_cid: u32,
    #[prost(int64, tag="3")]
    pub client_id: i64,
    #[prost(string, tag="4")]
    pub phone: std::string::String,
    #[prost(string, tag="5")]
    pub first_name: std::string::String,
    #[prost(string, tag="6")]
    pub last_name: std::string::String,
    #[prost(uint32, tag="12")]
    pub target_user_cid: u32,
    #[prost(uint32, tag="15")]
    pub target_channel_cid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(string, tag="2")]
    pub session_uuid: std::string::String,
    #[prost(uint32, tag="3")]
    pub user_cid: u32,
    #[prost(string, tag="4")]
    pub last_ip_address: std::string::String,
    #[prost(string, tag="8")]
    pub user_agent: std::string::String,
    #[prost(enumeration="SessionPlatform", tag="9")]
    pub platform: i32,
    #[prost(uint32, tag="5")]
    pub app_version: u32,
    #[prost(uint32, tag="6")]
    pub active_time: u32,
    #[prost(uint32, tag="7")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sms {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(string, tag="2")]
    pub hash: std::string::String,
    #[prost(string, tag="3")]
    pub app_uuid: std::string::String,
    #[prost(string, tag="4")]
    pub client_phone: std::string::String,
    #[prost(uint32, tag="5")]
    pub genrated_code: u32,
    #[prost(string, tag="6")]
    pub sms_sender_number: std::string::String,
    #[prost(string, tag="7")]
    pub sms_send_statues: std::string::String,
    #[prost(string, tag="8")]
    pub sms_http_body: std::string::String,
    #[prost(string, tag="9")]
    pub err: std::string::String,
    #[prost(string, tag="10")]
    pub carrier: std::string::String,
    #[prost(bytes, tag="11")]
    pub country: std::vec::Vec<u8>,
    #[prost(uint32, tag="12")]
    pub is_valid_phone: u32,
    #[prost(uint32, tag="13")]
    pub is_confirmed: u32,
    #[prost(uint32, tag="14")]
    pub is_login: u32,
    #[prost(uint32, tag="15")]
    pub is_register: u32,
    #[prost(uint32, tag="16")]
    pub retried_count: u32,
    #[prost(uint32, tag="17")]
    pub ttl: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(string, tag="2")]
    pub name: std::string::String,
    #[prost(uint32, tag="3")]
    pub count: u32,
    ///  uint32 tag_status_enum = 4;
    #[prost(bool, tag="5")]
    pub is_blocked: bool,
    #[prost(uint32, tag="6")]
    pub group_cid: u32,
    #[prost(uint32, tag="7")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(uint32, tag="1")]
    pub cid: u32,
    #[prost(string, tag="14")]
    pub phone: std::string::String,
    #[prost(string, tag="15")]
    pub email: std::string::String,
    #[prost(string, tag="17")]
    pub password_hash: std::string::String,
    #[prost(string, tag="18")]
    pub password_salt: std::string::String,
    #[prost(uint32, tag="36")]
    pub created_time: u32,
    #[prost(uint32, tag="37")]
    pub version_time: u32,
    #[prost(uint32, tag="38")]
    pub is_deleted: u32,
    #[prost(uint32, tag="39")]
    pub is_banned: u32,
    #[prost(uint32, tag="50")]
    pub primary_channel_changed_time: u32,
    #[prost(message, optional, tag="80")]
    pub user_counts: ::std::option::Option<Channel>,
    #[prost(message, optional, tag="100")]
    pub primary_channel: ::std::option::Option<Channel>,
    #[prost(message, repeated, tag="101")]
    pub channels: ::std::vec::Vec<Channel>,
    ///  repeated Chat chats = 103;
    ///  repeated Contact contacts= 103;
    #[prost(message, repeated, tag="102")]
    pub sessions: ::std::vec::Vec<Session>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCounts {
    /// Number of created channels
    #[prost(uint32, tag="1")]
    pub created_channels: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRelation {
    #[prost(int64, tag="1")]
    pub rel_nano_id: i64,
    #[prost(uint32, tag="2")]
    pub user_cid: u32,
    #[prost(uint32, tag="3")]
    pub peer_user_id: u32,
    #[prost(uint32, tag="4")]
    pub follwing: u32,
    #[prost(uint32, tag="5")]
    pub followed: u32,
    #[prost(uint32, tag="6")]
    pub in_contacts: u32,
    #[prost(uint32, tag="7")]
    pub mutual_contact: u32,
    #[prost(uint32, tag="8")]
    pub is_favorite: u32,
    #[prost(uint32, tag="9")]
    pub notify: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chat {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(uint32, tag="5")]
    pub user_cid: u32,
    #[prost(uint32, tag="40")]
    pub channel_cid: u32,
    #[prost(uint32, tag="6")]
    pub peer_user_cid: u32,
    #[prost(uint32, tag="41")]
    pub peer_channel_cid: u32,
    #[prost(uint32, tag="7")]
    pub group_cid: u32,
    #[prost(string, tag="9")]
    pub custom_title: std::string::String,
    #[prost(fixed64, tag="10")]
    pub pin_time_ms: u64,
    #[prost(int64, tag="11")]
    pub from_msg_gid: i64,
    #[prost(uint32, tag="12")]
    pub unseen_count: u32,
    #[prost(uint32, tag="13")]
    pub seq: u32,
    #[prost(uint32, tag="16")]
    pub my_last_seen_seq: u32,
    #[prost(int64, tag="17")]
    pub my_last_seen_msg_id: i64,
    #[prost(int64, tag="18")]
    pub peer_last_seen_msg_id: i64,
    #[prost(uint32, tag="19")]
    pub my_last_delivered_seq: u32,
    #[prost(int64, tag="20")]
    pub my_last_delivered_msg_id: i64,
    #[prost(int64, tag="21")]
    pub peer_last_delivered_msg_id: i64,
    #[prost(bool, tag="22")]
    pub is_active: bool,
    #[prost(uint32, tag="29")]
    pub mute_until: u32,
    #[prost(int64, tag="30")]
    pub sort_time_ms: i64,
    #[prost(uint32, tag="33")]
    pub created_time: u32,
    #[prost(fixed64, tag="45")]
    pub sort_time: u64,
    #[prost(message, optional, tag="48")]
    pub channel: ::std::option::Option<Channel>,
    #[prost(message, optional, tag="49")]
    pub contact: ::std::option::Option<Contact>,
    #[prost(message, optional, tag="50")]
    pub group: ::std::option::Option<Group>,
    #[prost(message, optional, tag="25")]
    pub last_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag="26")]
    pub pinned_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag="44")]
    pub avatar: ::std::option::Option<FileMsg>,
    #[prost(message, optional, tag="43")]
    pub group_member: ::std::option::Option<GroupMember>,
    #[prost(message, optional, tag="46")]
    pub draft: ::std::option::Option<MessageDraft>,
    #[prost(message, optional, tag="47")]
    pub custom_notification: ::std::option::Option<ChatCustomNotification>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageDraft {
    #[prost(string, tag="34")]
    pub draft_text: std::string::String,
    #[prost(int64, tag="35")]
    pub drat_reply_to_msg_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatCustomNotification {
    #[prost(bool, tag="13")]
    pub alert: bool,
    #[prost(bool, tag="14")]
    pub preview: bool,
    #[prost(bool, tag="15")]
    pub led_on: bool,
    #[prost(bool, tag="16")]
    pub led_color: bool,
    #[prost(bool, tag="17")]
    pub vibrate: bool,
    #[prost(bool, tag="18")]
    pub popup: bool,
    #[prost(bool, tag="19")]
    pub sound: bool,
    #[prost(bool, tag="20")]
    pub priority: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    #[prost(uint32, tag="1")]
    pub cid: u32,
    #[prost(string, tag="3")]
    pub group_title: std::string::String,
    #[prost(string, tag="4")]
    pub user_name: std::string::String,
    #[prost(uint32, tag="7")]
    pub creator_user_cid: u32,
    #[prost(uint32, tag="31")]
    pub creator_channel_cid: u32,
    #[prost(enumeration="GroupPrivacy", tag="8")]
    pub group_privacy: i32,
    #[prost(bool, tag="9")]
    pub history_viewable: bool,
    #[prost(uint32, tag="10")]
    pub seq: u32,
    #[prost(uint32, tag="14")]
    pub avatar_count: u32,
    #[prost(string, tag="15")]
    pub about: std::string::String,
    #[prost(string, tag="16")]
    pub invite_link_hash: std::string::String,
    #[prost(uint32, tag="17")]
    pub members_count: u32,
    #[prost(uint32, tag="18")]
    pub admins_count: u32,
    #[prost(uint32, tag="19")]
    pub moderator_counts: u32,
    #[prost(fixed64, tag="20")]
    pub sort_time: u64,
    #[prost(fixed64, tag="40")]
    pub sync_time: u64,
    #[prost(uint32, tag="21")]
    pub created_time: u32,
    #[prost(bool, tag="23")]
    pub is_deleted: bool,
    #[prost(bool, tag="24")]
    pub is_banned: bool,
    #[prost(message, optional, tag="25")]
    pub last_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag="26")]
    pub pinned_message: ::std::option::Option<Message>,
    #[prost(message, optional, tag="27")]
    pub avatar: ::std::option::Option<FileMsg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMember {
    #[prost(int64, tag="1")]
    pub gid: i64,
    #[prost(int64, tag="2")]
    pub group_cid: i64,
    #[prost(uint32, tag="3")]
    pub user_cid: u32,
    #[prost(uint32, tag="8")]
    pub channel_cid: u32,
    #[prost(uint32, tag="4")]
    pub by_user_cid: u32,
    #[prost(uint32, tag="7")]
    pub by_channel_cid: u32,
    #[prost(enumeration="GroupMemberRole", tag="5")]
    pub group_role: i32,
    #[prost(uint32, tag="6")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileMsg {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(uint32, tag="2")]
    pub access_hash: u32,
    #[prost(uint32, tag="3")]
    pub file_type: u32,
    #[prost(uint32, tag="4")]
    pub width: u32,
    #[prost(uint32, tag="5")]
    pub height: u32,
    #[prost(string, tag="6")]
    pub extension: std::string::String,
    #[prost(uint32, tag="7")]
    pub user_cid: u32,
    #[prost(bytes, tag="8")]
    pub data_thumb: std::vec::Vec<u8>,
    #[prost(bytes, tag="9")]
    pub data: std::vec::Vec<u8>,
}
/////////////////////////// Docs ///////////////////////////

//
//No Rpc_Account > Rpc_User + Rpc_Profile ,...
//
//

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
    Forward = 17,
    ///??
    ///
    /// photo, video, gif
    PostMedia = 100,
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
    NotAble = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageLogType {
    UserJoined = 0,
    UserDeleted = 1,
    RoomCreated = 2,
    MemberAdded = 3,
    MemberKicked = 4,
    MemberLeft = 5,
    RoomConvertedToPublic = 6,
    RoomConvertedToPrivate = 7,
    MemberJoinedByInviteLink = 8,
    RoomDeleted = 9,
    MissedVoiceCall = 10,
    MissedVideoCall = 11,
    MissedScreenShare = 12,
    MissedSecretChat = 13,
    PinnedMessage = 14,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelType {
    ChannelUnknown = 0,
    ChannelPrimary = 1,
    ChannelNormal = 2,
    ChannelSaves = 3,
    ChannelStore = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelPrivacy {
    ChannelUnknownAb = 0,
    ChannelOpen = 1,
    ChannelPrivateLink = 2,
    /// For save
    ChannelCreator = 3,
}
/// Shared common cid
/// todo to klass?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Cast {
    Unknown = 0,
    User = 1,
    Channel = 2,
    Group = 3,
    Bot = 4,
    Store = 6,
    ///should be ??
    Tag = 7,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReactionType {
    None = 0,
    Like = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SessionPlatform {
    UnknownPlatform = 0,
    Android = 1,
    Ios = 2,
    MacOs = 3,
    Windows = 4,
    Linux = 5,
    BlackBerry = 6,
    Web = 7,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GroupPrivacy {
    GroupUnknownGp = 0,
    /// via username
    GroupOpen = 1,
    GroupPrivateLink = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GroupMemberRole {
    MemberUnknown = 0,
    MemberCreator = 1,
    MemberModerator = 3,
    MemberNormalUser = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendConfirmCodeParam {
    #[prost(string, tag="1")]
    pub hash: std::string::String,
    /// 98... 989015132328
    #[prost(string, tag="2")]
    pub phone: std::string::String,
    #[prost(string, tag="3")]
    pub country_code: std::string::String,
    #[prost(bool, tag="4")]
    pub resend: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendConfirmCodeResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
    #[prost(string, tag="2")]
    pub error_message: std::string::String,
    #[prost(bool, tag="3")]
    pub just_email_register: bool,
    #[prost(string, repeated, tag="4")]
    pub sms_numbers: ::std::vec::Vec<std::string::String>,
    #[prost(bool, tag="5")]
    pub is_login: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmCodeParam {
    #[prost(string, tag="1")]
    pub hash: std::string::String,
    /// 98... 989015132328
    #[prost(string, tag="2")]
    pub phone: std::string::String,
    ///4 digit 4215
    #[prost(int32, tag="3")]
    pub code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmCodeResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
    ///    SelfUserView SelfUserView = 3; //if it is login
    #[prost(string, tag="2")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingUpParam {
    #[prost(string, tag="1")]
    pub hash: std::string::String,
    #[prost(string, tag="2")]
    pub first_name: std::string::String,
    #[prost(string, tag="3")]
    pub last_name: std::string::String,
    #[prost(string, tag="4")]
    pub user_name: std::string::String,
    #[prost(string, tag="5")]
    pub phone: std::string::String,
    #[prost(string, tag="6")]
    pub email: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingUpResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
    ///    SelfUserView SelfUserView = 3;
    #[prost(string, tag="2")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingInParam {
    #[prost(string, tag="4")]
    pub user_name_phone_email: std::string::String,
    /// 98... 989015132328
    #[prost(string, tag="5")]
    pub password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingInResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
    ///    SelfUserView SelfUserView = 3;
    #[prost(string, tag="2")]
    pub error_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogOutParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogOutResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
    #[prost(string, tag="2")]
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
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarAddResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarDeleteParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarDeleteResponse {
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
pub struct ChannelSendMessageParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditMessageParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditMessageResponse {
}
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteMessageParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSetDraftParam {
    #[prost(uint32, tag="1")]
    pub channel_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSetDraftResponse {
}
/// CrDU
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCreateChannelParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCreateChannelResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditChannelParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEditChannelResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteChannelParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteChannelResponse {
}
/// Author
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAddAuthorParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAddAuthorResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeAuthorPermissionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeAuthorPermissionResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveAuthorParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveAuthorResponse {
}
/// Follow
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelFollowChannelParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelFollowChannelResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnFollowChannelParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnFollowChannelResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveFollowersParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveFollowersResponse {
}
// Subscribe

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscribeParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscribeResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnSubscribeParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnSubscribeResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveSubscribersParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRemoveSubscribersResponse {
}
// Privacy

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangePrivacyParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangePrivacyResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeDefaultPermissionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeDefaultPermissionResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRevokeLinkParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRevokeLinkResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeUsernameParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelChangeUsernameResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBlockChannelParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBlockChannelResponse {
}
// Messages

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteMessagesParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteMessagesResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelClearHistoryParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelClearHistoryResponse {
}
// Others

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarChangeParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAvatarChangeResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendDoingActionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSendDoingActionResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportChannelParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportChannelResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportMessageParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReportMessageResponse {
}
// Views

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
// crud

// Members

// Privacy

// Messages

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSendMessageParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSendMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatEditMessageParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatEditMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteMessagesParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteMessagesResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteHistoryParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatDeleteHistoryResponse {
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
//========= One =========
// CRUD

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDeleteDirectParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDeleteDirectResponse {
}
// Pin, Archives, Marks

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectChangeTitleParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectChangeTitleResponse {
}
// Notifications

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSetCustomNotificationParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSetCustomNotificationResponse {
}
// Others

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSendActionDoingParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSendActionDoingResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSetDraftParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSetDraftResponse {
}
//========= One End =========

///========= Many =========
/// CRUD
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDeleteDirectsParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDeleteDirectsResponse {
}
/// = Pin, Archives, Marks
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMarkAsReadParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMarkAsReadResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMarkAsUnReadParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMarkAsUnReadResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectPinDirectsParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectPinDirectsResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUnPinDirectsParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUnPinDirectsResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectArchiveDirectsParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectArchiveDirectsResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUnArchiveDirectsParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUnArchiveDirectsResponse {
}
// Messages

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectClearHistoriesParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectClearHistoriesResponse {
}
// Notifications

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMuteDirectsParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMuteDirectsResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUnMuteDirectsParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUnMuteDirectsResponse {
}
//========= Many End =========

// Folders

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectCreateFolderParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectCreateFolderResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectChangeFolderParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectChangeFolderResponse {
}
//
//message Param {
//}
//
//message Response {
//}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectRemoveFromFolderParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectRemoveFromFolderResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectReordersFolderParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectReordersFolderResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDeleteFolderParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDeleteFolderResponse {
}
// Views

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetChatsListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetChatsListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetGroupsListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetGroupsListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetChannelsListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetChannelsListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetFoldersListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetFoldersListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetFoldersFullListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectGetFoldersFullListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarAddParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarAddResponse {
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendMessageParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendMessageResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditMessageParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
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
pub struct GroupSetDraftParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSetDraftResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetFullMessageParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetFullMessageResponse {
}
/// CrDU
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateGroupParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateGroupResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditGroupParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupEditGroupResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteGroupParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteGroupResponse {
}
// Members

//message Param {
//}
//
//message Response {
//}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddAdminParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddAdminResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddMemberParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAddMemberResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRemoveMemberParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRemoveMemberResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberLevelParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberLevelResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberPermissionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeMemberPermissionResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinGroupParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinGroupResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupLeaveGroupParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupLeaveGroupResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupBanMemberParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupBanMemberResponse {
}
// Privacy

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangePrivacyParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangePrivacyResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeDefaultPermissionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeDefaultPermissionResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRevokeLinkParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRevokeLinkResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeUsernameParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeUsernameResponse {
}
// Messages

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteMessagesParam {
    #[prost(uint32, tag="1")]
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteMessagesResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteHistoryParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteHistoryResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupClearHistoryParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupClearHistoryResponse {
}
// Others

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarChangeParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAvatarChangeResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendDoingActionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSendDoingActionResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupReportGroupParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupReportGroupResponse {
}
// Views

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMessagesListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMessagesListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMediaListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMediaListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMembersListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetMembersListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetAdminsListParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupGetAdminsListResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUsers1Param {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUsers1Response {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoParam {
    #[prost(string, tag="1")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
    #[prost(string, tag="2")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckUserNameParam {
    #[prost(string, tag="1")]
    pub username: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckUserNameResponse {
    #[prost(bool, tag="1")]
    pub is_available: bool,
    #[prost(string, tag="2")]
    pub username: std::string::String,
    #[prost(string, tag="3")]
    pub show_message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
    #[prost(string, tag="2")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePhoneNumberParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePhoneNumberResponse {
    #[prost(bool, tag="1")]
    pub done: bool,
    #[prost(string, tag="2")]
    pub text: std::string::String,
}
