///dep
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MediaTypeEnum {
    MediaImage = 0,
    MediaVideo = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FollowingEnum {
    FollowingNone = 0,
    Following = 1,
    Requested = 2,
    Blocked = 3,
}
//enum UserTypeEnum {
//    USER = 0;
//    CHANNEL = 1;
//}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserLevelEnum {
    LevelNormal = 0,
    AppAdmin = 1,
    Susponded = 2,
    DeletedByOwener = 3,
    DeletedIran = 4,
    SuspondedIran = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GeneralPrivacyEnum {
    UnknownGeneralPrivacy = 0,
    AllPeoplePrivacy = 1,
    NobodyPrivacy = 2,
    ContactsOnlyPrivacy = 3,
    FollowedOnlyPrivacy = 4,
    ContactsAndFollowdPrivacy = 5,
}
///DEP
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserOnlinePrivacyEnum {
    E4 = 0,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserOnlineStatusEnum {
    Exactly = 0,
    Online = 1,
    Connected = 2,
    FewDaysAgo = 3,
    Recently = 4,
    LastWeek = 5,
    LastMonth = 6,
    LongTimeAgo = 7,
    Hide = 8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PostTypeEnum {
    PostTypeUnknown = 0,
    PostText = 1,
    PostPhoto = 2,
    PostVideo = 3,
    PostGif = 4,
    PostAudio = 5,
    PostFile = 7,
    PostPoll = 8,
    /// photo, video, gif
    PostMedia = 100,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CategoryEnum {
    ///POST_TEXT = 1;
    ///POST_PHOTO = 2;
    ///POST_VIDEO = 3;
    ///POST_GIFS = 4;
    ///POST_AUDIO = 5;
    ///POST_GIF = 6;
    ///POST_FILE = 7;
    ///POST_POLL = 8;
    CategoryReshared = 0,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PostCategoryEnum {
    PostCatText = 0,
    PostCatMedia = 1,
    PostCatFile = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotifyEnum {
    NotifyPostLiked = 0,
    NotifyPostCommented = 1,
    NotifyFollowedYou = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionEnum {
    ActionPostLiked = 0,
    ActionPostCommented = 1,
    ActionFollowedUser = 2,
}
// chat

//enum RoomTypeEnum {
//    UNKNOWN_ROOM_TYPE = 0;
//    DIRECT = 1;
//    GROUP = 2;
//    //    CHANNEL = 3;
//    BROADCAST = 3;
//    //    BULK = 3;
//}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomActionDoingEnum {
    UnknownRoomActionDoing = 0,
    Cancel = 1,
    Typing = 2,
    SendingImage = 3,
    CapturingImage = 4,
    SendingVideo = 5,
    CapturingVideo = 6,
    SendingAudio = 7,
    RecordingVoice = 8,
    SendingVoice = 9,
    SendingDocument = 11,
    SendingGif = 12,
    SendingFile = 13,
    SendingLocation = 14,
    ChoosingContact = 15,
    Painting = 16,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Param {
    #[prost(string, tag="1")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaView {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionView {
    #[prost(int64, tag="1")]
    pub action_id: i64,
    #[prost(int32, tag="2")]
    pub actor_user_id: i32,
    #[prost(int32, tag="3")]
    pub action_type_enum: i32,
    #[prost(int32, tag="4")]
    pub peer_user_id: i32,
    #[prost(int64, tag="5")]
    pub post_id: i64,
    #[prost(int64, tag="6")]
    pub comment_id: i64,
    #[prost(int64, tag="7")]
    pub murmur64_hash: i64,
    #[prost(int32, tag="8")]
    pub created_time: i32,
    #[prost(message, optional, tag="100")]
    pub actor_user_view: ::std::option::Option<UserView>,
    #[prost(message, optional, tag="101")]
    pub post_view: ::std::option::Option<PostView>,
    #[prost(message, optional, tag="102")]
    pub comment_view: ::std::option::Option<CommentView>,
    #[prost(message, optional, tag="103")]
    pub followed_user_view: ::std::option::Option<UserView>,
    #[prost(message, optional, tag="104")]
    pub content_owener_user_view: ::std::option::Option<UserView>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyView {
    #[prost(int64, tag="1")]
    pub notify_id: i64,
    #[prost(int32, tag="2")]
    pub for_user_id: i32,
    #[prost(int32, tag="3")]
    pub actor_user_id: i32,
    #[prost(int32, tag="4")]
    pub notiy_type_enum: i32,
    #[prost(int64, tag="5")]
    pub post_id: i64,
    #[prost(int64, tag="6")]
    pub comment_id: i64,
    #[prost(int32, tag="7")]
    pub peer_user_id: i32,
    #[prost(int64, tag="8")]
    pub murmur64_hash: i64,
    #[prost(int32, tag="9")]
    pub seen_status: i32,
    #[prost(int32, tag="10")]
    pub created_time: i32,
    #[prost(message, optional, tag="100")]
    pub actor_user_view: ::std::option::Option<UserView>,
    #[prost(message, optional, tag="101")]
    pub post_view: ::std::option::Option<PostView>,
    #[prost(message, optional, tag="102")]
    pub comment_view: ::std::option::Option<CommentView>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentView {
    #[prost(int64, tag="1")]
    pub comment_id: i64,
    #[prost(int32, tag="2")]
    pub user_id: i32,
    #[prost(int64, tag="3")]
    pub post_id: i64,
    #[prost(string, tag="4")]
    pub text: std::string::String,
    #[prost(int32, tag="5")]
    pub likes_count: i32,
    #[prost(int32, tag="6")]
    pub created_time: i32,
    #[prost(message, optional, tag="15")]
    pub sender_user_view: ::std::option::Option<UserView>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostView {
    #[prost(int64, tag="1")]
    pub post_id: i64,
    #[prost(int32, tag="2")]
    pub user_id: i32,
    #[prost(enumeration="PostTypeEnum", tag="3")]
    pub post_type_enum: i32,
    #[prost(string, tag="4")]
    pub text: std::string::String,
    #[prost(string, tag="5")]
    pub rich_text: std::string::String,
    #[prost(int32, tag="6")]
    pub media_count: i32,
    #[prost(int32, tag="7")]
    pub shared_to: i32,
    #[prost(int32, tag="8")]
    pub disable_comment: i32,
    #[prost(int32, tag="9")]
    pub has_tag: i32,
    #[prost(int32, tag="10")]
    pub comments_count: i32,
    #[prost(int32, tag="11")]
    pub likes_count: i32,
    #[prost(int32, tag="12")]
    pub views_count: i32,
    #[prost(int32, tag="13")]
    pub edited_time: i32,
    #[prost(int32, tag="14")]
    pub created_time: i32,
    #[prost(int64, tag="15")]
    pub re_shared_post_id: i64,
    ///With me
    #[prost(bool, tag="50")]
    pub did_i_liked: bool,
    #[prost(bool, tag="51")]
    pub did_i_re_shared: bool,
    #[prost(message, optional, tag="100")]
    pub sender_user_view: ::std::option::Option<UserView>,
    #[prost(message, optional, tag="101")]
    pub re_shared_user_view: ::std::option::Option<UserView>,
    #[prost(message, optional, tag="102")]
    pub media_view: ::std::option::Option<MediaView>,
    #[prost(message, repeated, tag="103")]
    pub media_view_list: ::std::vec::Vec<MediaView>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatView {
    #[prost(int64, tag="1")]
    pub chat_id: i64,
    #[prost(string, tag="2")]
    pub chat_key: std::string::String,
    #[prost(string, tag="3")]
    pub room_key: std::string::String,
    #[prost(int32, tag="4")]
    pub room_type: i32,
    #[prost(int32, tag="5")]
    pub user_id: i32,
    #[prost(int32, tag="6")]
    pub peer_user_id: i32,
    #[prost(int64, tag="7")]
    pub group_id: i64,
    #[prost(int64, tag="8")]
    pub hash_tag_id: i64,
    #[prost(int32, tag="9")]
    pub started_by_me: i32,
    #[prost(string, tag="10")]
    pub title: std::string::String,
    #[prost(int64, tag="11")]
    pub pin_time: i64,
    #[prost(int64, tag="12")]
    pub from_msg_id: i64,
    #[prost(int32, tag="13")]
    pub seq: i32,
    #[prost(int64, tag="14")]
    pub last_msg_id: i64,
    #[prost(int32, tag="15")]
    pub last_msg_status: i32,
    #[prost(int32, tag="16")]
    pub seen_seq: i32,
    #[prost(int64, tag="17")]
    pub seen_msg_id: i64,
    #[prost(int32, tag="18")]
    pub left: i32,
    #[prost(int32, tag="19")]
    pub creator: i32,
    #[prost(int32, tag="20")]
    pub kicked: i32,
    #[prost(int32, tag="21")]
    pub admin: i32,
    #[prost(int32, tag="22")]
    pub deactivated: i32,
    #[prost(int32, tag="23")]
    pub version_time: i32,
    #[prost(int32, tag="24")]
    pub sort_time: i32,
    #[prost(int32, tag="25")]
    pub created_time: i32,
    #[prost(string, tag="26")]
    pub draft_text: std::string::String,
    #[prost(int64, tag="27")]
    pub drat_reply_to_msg_id: i64,
    #[prost(int32, tag="28")]
    pub is_mute: i32,
    #[prost(message, optional, tag="100")]
    pub user_view: ::std::option::Option<UserView>,
    #[prost(message, optional, tag="101")]
    pub group_view: ::std::option::Option<GroupView>,
    ///seeting, notification, group, tag
    #[prost(message, optional, tag="200")]
    pub first_unread_message: ::std::option::Option<MessageView>,
    #[prost(message, optional, tag="201")]
    pub last_message: ::std::option::Option<MessageView>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupView {
    #[prost(int64, tag="1")]
    pub group_id: i64,
    #[prost(string, tag="2")]
    pub group_key: std::string::String,
    #[prost(string, tag="3")]
    pub group_name: std::string::String,
    #[prost(string, tag="4")]
    pub user_name: std::string::String,
    #[prost(int32, tag="5")]
    pub is_super_group: i32,
    #[prost(int64, tag="6")]
    pub hash_tag_id: i64,
    #[prost(int32, tag="7")]
    pub creator_user_id: i32,
    #[prost(int32, tag="8")]
    pub group_privacy: i32,
    #[prost(int32, tag="9")]
    pub history_view_able: i32,
    #[prost(int64, tag="10")]
    pub seq: i64,
    #[prost(int64, tag="11")]
    pub last_msg_id: i64,
    #[prost(int64, tag="12")]
    pub pined_msg_id: i64,
    #[prost(int64, tag="13")]
    pub avatar_ref_id: i64,
    #[prost(int32, tag="14")]
    pub avatar_count: i32,
    #[prost(string, tag="15")]
    pub about: std::string::String,
    #[prost(string, tag="16")]
    pub invite_link: std::string::String,
    #[prost(int32, tag="17")]
    pub members_count: i32,
    #[prost(int32, tag="18")]
    pub sort_time: i32,
    #[prost(int32, tag="19")]
    pub created_time: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMemberView {
    #[prost(int64, tag="1")]
    pub order_id: i64,
    #[prost(int64, tag="2")]
    pub group_id: i64,
    #[prost(int32, tag="3")]
    pub user_id: i32,
    #[prost(int32, tag="4")]
    pub by_user_id: i32,
    #[prost(int32, tag="5")]
    pub group_role: i32,
    #[prost(int32, tag="6")]
    pub created_time: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageView {
    #[prost(string, tag="1")]
    pub room_key: std::string::String,
    #[prost(int64, tag="2")]
    pub message_id: i64,
    #[prost(int32, tag="3")]
    pub user_id: i32,
    #[prost(int64, tag="4")]
    pub file_ref_id: i64,
    #[prost(int32, tag="5")]
    pub message_type: i32,
    #[prost(string, tag="6")]
    pub text: std::string::String,
    #[prost(int32, tag="7")]
    pub hiden: i32,
    #[prost(int32, tag="8")]
    pub seq: i32,
    #[prost(int64, tag="9")]
    pub forwarded_msg_id: i64,
    #[prost(int64, tag="10")]
    pub post_id: i64,
    #[prost(int64, tag="11")]
    pub sticker_id: i64,
    #[prost(int32, tag="12")]
    pub created_time: i32,
    #[prost(int32, tag="13")]
    pub delivered_time: i32,
    #[prost(int32, tag="14")]
    pub seen_time: i32,
    #[prost(int32, tag="15")]
    pub deliviry_status: i32,
    #[prost(int64, tag="16")]
    pub reply_to_message_id: i64,
    #[prost(int64, tag="17")]
    pub views_count: i64,
    #[prost(int32, tag="18")]
    pub edit_time: i32,
    #[prost(int32, tag="19")]
    pub ttl: i32,
    #[prost(message, optional, tag="50")]
    pub file_red_view: ::std::option::Option<FileRedView>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileRedView {
    #[prost(int64, tag="1")]
    pub file_ref_id: i64,
    #[prost(int64, tag="2")]
    pub user_id: i64,
    #[prost(string, tag="3")]
    pub name: std::string::String,
    #[prost(int32, tag="4")]
    pub width: i32,
    #[prost(int32, tag="5")]
    pub height: i32,
    #[prost(int32, tag="6")]
    pub duration: i32,
    #[prost(string, tag="7")]
    pub extension: std::string::String,
    #[prost(string, tag="8")]
    pub url_source: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserView {
    #[prost(int32, tag="1")]
    pub user_id: i32,
    #[prost(string, tag="2")]
    pub user_name: std::string::String,
    #[prost(string, tag="4")]
    pub first_name: std::string::String,
    #[prost(string, tag="5")]
    pub last_name: std::string::String,
    #[prost(int64, tag="8")]
    pub avatar_ref_id: i64,
    #[prost(int32, tag="9")]
    pub profile_privacy: i32,
    #[prost(int64, tag="10")]
    pub phone: i64,
    #[prost(string, tag="11")]
    pub about: std::string::String,
    ///counters 100 - 200
    #[prost(int32, tag="100")]
    pub followers_count: i32,
    #[prost(int32, tag="101")]
    pub following_count: i32,
    #[prost(int32, tag="102")]
    pub posts_count: i32,
    #[prost(int32, tag="103")]
    pub media_count: i32,
    ///last activities
    #[prost(enumeration="UserOnlineStatusEnum", tag="200")]
    pub user_online_status_enum: i32,
    #[prost(int32, tag="201")]
    pub last_active_time: i32,
    #[prost(string, tag="202")]
    pub last_active_time_show: std::string::String,
    ///with me
    #[prost(enumeration="FollowingEnum", tag="300")]
    pub my_follwing: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettingNotificationView {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConfig {
    #[prost(bool, tag="1")]
    pub deprecated_client: bool,
    #[prost(bool, tag="2")]
    pub has_new_update: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProfileView {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserViewRowify {
    #[prost(int64, tag="1")]
    pub id: i64,
    #[prost(int32, tag="2")]
    pub created_time: i32,
    #[prost(message, optional, tag="10")]
    pub user_view: ::std::option::Option<UserView>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostViewRowify {
    #[prost(int64, tag="1")]
    pub id: i64,
    #[prost(message, optional, tag="10")]
    pub post_view: ::std::option::Option<PostView>,
}
///////////// Tags /////////////
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagView {
    #[prost(int64, tag="1")]
    pub tag_id: i64,
    #[prost(string, tag="2")]
    pub name: std::string::String,
    #[prost(int32, tag="3")]
    pub count: i32,
    #[prost(int32, tag="4")]
    pub tag_status_enum: i32,
    #[prost(int32, tag="5")]
    pub created_time: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopTagWithSamplePosts {
    #[prost(message, optional, tag="1")]
    pub tag_view: ::std::option::Option<TagView>,
    #[prost(message, repeated, tag="2")]
    pub post_view_list: ::std::vec::Vec<PostView>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfUserView {
    ///1-30 basic user table views
    #[prost(message, optional, tag="1")]
    pub user_view: ::std::option::Option<UserView>,
    ///30-50 privacy
    #[prost(int32, tag="30")]
    pub profile_privacy: i32,
    #[prost(int32, tag="32")]
    pub online_privacy: i32,
    #[prost(int32, tag="33")]
    pub call_privacy: i32,
    #[prost(int32, tag="34")]
    pub add_to_group_privacy: i32,
    #[prost(int32, tag="35")]
    pub seen_message_privacy: i32,
    /// 100 other views
    ///
    /// settings , notifactions settings,... 
    ///privacy 
    #[prost(message, optional, tag="100")]
    pub setting_notification: ::std::option::Option<SettingNotificationView>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(enumeration="ServerErrors", tag="1")]
    pub error: i32,
    #[prost(bool, tag="2")]
    pub show_error: bool,
    #[prost(string, tag="3")]
    pub error_message: std::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServerErrors {
    UnknownErr = 0,
    ErrClientIsDeprecated = 1,
    ErrServerUpgrading = 2,
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
    #[prost(string, tag="2")]
    pub error_message: std::string::String,
    ///if it is login
    #[prost(message, optional, tag="3")]
    pub self_user_view: ::std::option::Option<SelfUserView>,
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
    #[prost(string, tag="2")]
    pub error_message: std::string::String,
    #[prost(message, optional, tag="3")]
    pub self_user_view: ::std::option::Option<SelfUserView>,
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
    #[prost(string, tag="2")]
    pub error_message: std::string::String,
    #[prost(message, optional, tag="3")]
    pub self_user_view: ::std::option::Option<SelfUserView>,
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
//import "pb_enum.proto";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoParam {
    #[prost(string, tag="1")]
    pub text: std::string::String,
}
///    bool done =1;
///    string text = 2;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoResponse {
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
// next: user relations, shop, product, message log, store, file
//
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
    ///  uint32 media_count = 9;
    #[prost(uint32, tag="10")]
    pub shared_to: u32,
    #[prost(uint32, tag="12")]
    pub via: u32,
    #[prost(uint32, tag="13")]
    pub seq: u32,
    #[prost(uint32, tag="17")]
    pub edited_time: u32,
    #[prost(uint32, tag="18")]
    pub created_time: u32,
    #[prost(enumeration="MessageDeliveryStatues", tag="105")]
    pub delivery_status: i32,
    #[prost(uint32, tag="106")]
    pub delivery_time: u32,
    #[prost(uint64, tag="180")]
    pub previous_message_id: u64,
    ///??
    #[prost(bool, tag="15")]
    pub deleted: bool,
    #[prost(message, optional, boxed, tag="16")]
    pub forward_from: ::std::option::Option<::std::boxed::Box<Message>>,
    #[prost(message, optional, boxed, tag="50")]
    pub reply_to: ::std::option::Option<::std::boxed::Box<Message>>,
    #[prost(message, optional, tag="101")]
    pub counts: ::std::option::Option<MessageCount>,
    #[prost(message, optional, tag="102")]
    pub setting: ::std::option::Option<MessageSetting>,
    #[prost(message, repeated, tag="103")]
    pub files: ::std::vec::Vec<FileMsg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageCount {
    #[prost(fixed64, tag="1")]
    pub message_gid: u64,
    #[prost(uint32, tag="2")]
    pub comments_count: u32,
    #[prost(uint32, tag="3")]
    pub likes_count: u32,
    #[prost(int64, tag="4")]
    pub views_count: i64,
    #[prost(uint32, tag="5")]
    pub re_shared_count: u32,
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
    pub sort_time: u64,
    #[prost(fixed64, tag="21")]
    pub sync_time: u64,
    #[prost(uint32, tag="36")]
    pub created_time: u32,
    #[prost(uint32, tag="38")]
    pub is_deleted: u32,
    #[prost(uint32, tag="39")]
    pub is_banned: u32,
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
//// archives

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Post {
    #[prost(fixed64, tag="1")]
    pub gid: u64,
    #[prost(uint32, tag="2")]
    pub user_cid: u32,
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
    #[prost(uint32, tag="9")]
    pub media_count: u32,
    #[prost(uint32, tag="10")]
    pub shared_to: u32,
    #[prost(uint32, tag="11")]
    pub disable_comment: u32,
    #[prost(uint32, tag="12")]
    pub via: u32,
    #[prost(uint32, tag="13")]
    pub seq: u32,
    #[prost(uint32, tag="14")]
    pub comments_count: u32,
    #[prost(uint32, tag="15")]
    pub likes_count: u32,
    #[prost(uint32, tag="16")]
    pub views_count: u32,
    #[prost(uint32, tag="17")]
    pub edited_time: u32,
    #[prost(uint32, tag="18")]
    pub created_time: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostCount {
    #[prost(fixed64, tag="1")]
    pub post_gid: u64,
    #[prost(uint32, tag="2")]
    pub comments_count: u32,
    #[prost(uint32, tag="3")]
    pub likes_count: u32,
    #[prost(int64, tag="4")]
    pub views_count: i64,
    #[prost(uint32, tag="5")]
    pub re_shared_count: u32,
    #[prost(uint32, tag="6")]
    pub chat_shared_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageDelivery {
    #[prost(enumeration="MessageDeliveryStatues", tag="11")]
    pub statues: i32,
    #[prost(uint32, tag="2")]
    pub seen_time: u32,
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
    Location = 11,
    Log = 12,
    Contact = 13,
    Wallet = 15,
    Product = 16,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageDeliveryStatues {
    Failed = 0,
    Sending = 1,
    Sent = 2,
    Delivered = 3,
    Seen = 4,
    /// listened, download, watched
    Consumed = 5,
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
pub struct Event {
    #[prost(int64, tag="1")]
    pub event_id: i64,
    #[prost(int32, tag="2")]
    pub event_type: i32,
    #[prost(int64, tag="3")]
    pub by_user_id: i64,
    #[prost(int64, tag="4")]
    pub peer_user_id: i64,
    #[prost(int64, tag="5")]
    pub post_id: i64,
    #[prost(int64, tag="6")]
    pub comment_id: i64,
    #[prost(int64, tag="7")]
    pub hash_tag_id: i64,
    #[prost(int64, tag="8")]
    pub group_id: i64,
    #[prost(int64, tag="9")]
    pub action_id: i64,
    #[prost(int64, tag="10")]
    pub chat_id: i64,
    #[prost(string, tag="11")]
    pub chat_key: std::string::String,
    #[prost(int64, tag="12")]
    pub message_id: i64,
    #[prost(int64, tag="13")]
    pub re_shared_id: i64,
    #[prost(int64, tag="14")]
    pub murmur64_hash: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbNotify {
    #[prost(int64, tag="1")]
    pub notify_id: i64,
    #[prost(int32, tag="2")]
    pub for_user_id: i32,
    #[prost(int32, tag="3")]
    pub actor_user_id: i32,
    #[prost(int32, tag="4")]
    pub notify_type: i32,
    #[prost(int64, tag="5")]
    pub post_id: i64,
    #[prost(int64, tag="6")]
    pub comment_id: i64,
    #[prost(int32, tag="7")]
    pub peer_user_id: i32,
    #[prost(int64, tag="8")]
    pub murmur64_hash: i64,
    #[prost(int32, tag="9")]
    pub seen_status: i32,
    #[prost(int32, tag="10")]
    pub created_time: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Invoke {
    #[prost(uint32, tag="1")]
    pub method: u32,
    #[prost(uint64, tag="2")]
    pub action_id: u64,
    #[prost(bool, tag="3")]
    pub is_response: bool,
    #[prost(bytes, tag="4")]
    pub rpc_data: std::vec::Vec<u8>,
}
