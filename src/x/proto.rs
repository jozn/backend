#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomMessageLocation {
    #[prost(double, tag="1")]
    pub lat: f64,
    #[prost(double, tag="2")]
    pub lon: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomMessageLog {
    #[prost(enumeration="room_message_log::Type", tag="1")]
    pub r#type: i32,
    #[prost(enumeration="room_message_log::ExtraType", tag="2")]
    pub extra_type: i32,
    #[prost(message, optional, tag="3")]
    pub target_user: ::std::option::Option<room_message_log::TargetUser>,
}
pub mod room_message_log {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetUser {
        #[prost(uint64, tag="1")]
        pub id: u64,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
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
    pub enum ExtraType {
        NoExtra = 0,
        TargetUser = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomMessageContact {
    #[prost(string, tag="1")]
    pub first_name: std::string::String,
    #[prost(string, tag="2")]
    pub last_name: std::string::String,
    #[prost(string, tag="3")]
    pub nickname: std::string::String,
    #[prost(string, repeated, tag="4")]
    pub phone: ::std::vec::Vec<std::string::String>,
    #[prost(string, repeated, tag="5")]
    pub email: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomMessageWallet {
    #[prost(enumeration="room_message_wallet::Type", tag="1")]
    pub r#type: i32,
    #[prost(message, optional, tag="2")]
    pub money_transfer: ::std::option::Option<room_message_wallet::MoneyTransfer>,
}
pub mod room_message_wallet {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MoneyTransfer {
        #[prost(uint64, tag="1")]
        pub from_user_id: u64,
        #[prost(uint64, tag="2")]
        pub to_user_id: u64,
        #[prost(uint64, tag="3")]
        pub amount: u64,
        #[prost(int64, tag="4")]
        pub trace_number: i64,
        #[prost(int64, tag="5")]
        pub invoice_number: i64,
        #[prost(uint32, tag="6")]
        pub pay_time: u32,
        #[prost(string, tag="7")]
        pub description: std::string::String,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        MoneyTransfer = 0,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomMessageForwardFrom {
    #[prost(uint64, tag="1")]
    pub room_id: u64,
    #[prost(uint64, tag="2")]
    pub message_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredUser {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub username: std::string::String,
    #[prost(uint64, tag="3")]
    pub phone: u64,
    #[prost(string, tag="4")]
    pub first_name: std::string::String,
    #[prost(string, tag="5")]
    pub last_name: std::string::String,
    #[prost(string, tag="6")]
    pub display_name: std::string::String,
    #[prost(string, tag="7")]
    pub initials: std::string::String,
    #[prost(string, tag="8")]
    pub color: std::string::String,
    #[prost(enumeration="registered_user::Status", tag="9")]
    pub status: i32,
    #[prost(uint32, tag="10")]
    pub last_seen: u32,
    #[prost(uint32, tag="11")]
    pub avatar_count: u32,
    #[prost(message, optional, tag="12")]
    pub avatar: ::std::option::Option<Avatar>,
    #[prost(bool, tag="13")]
    pub mutual: bool,
    #[prost(bool, tag="14")]
    pub deleted: bool,
    #[prost(string, tag="15")]
    pub cache_id: std::string::String,
    #[prost(string, tag="16")]
    pub bio: std::string::String,
    #[prost(bool, tag="17")]
    pub verified: bool,
    #[prost(bool, tag="18")]
    pub bot: bool,
}
pub mod registered_user {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        LongTimeAgo = 0,
        LastMonth = 1,
        LastWeek = 2,
        Online = 3,
        Exactly = 4,
        Recently = 5,
        Support = 6,
        ServiceNotifications = 7,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Avatar {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(message, optional, tag="2")]
    pub file: ::std::option::Option<File>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomMessage {
    #[prost(uint64, tag="1")]
    pub message_id: u64,
    #[prost(uint64, tag="2")]
    pub message_version: u64,
    #[prost(enumeration="RoomMessageStatus", tag="3")]
    pub status: i32,
    #[prost(uint64, tag="4")]
    pub status_version: u64,
    #[prost(enumeration="RoomMessageType", tag="5")]
    pub message_type: i32,
    #[prost(string, tag="6")]
    pub message: std::string::String,
    #[prost(message, optional, tag="7")]
    pub attachment: ::std::option::Option<File>,
    #[prost(message, optional, tag="8")]
    pub author: ::std::option::Option<room_message::Author>,
    #[prost(message, optional, tag="9")]
    pub location: ::std::option::Option<RoomMessageLocation>,
    #[prost(message, optional, tag="10")]
    pub log: ::std::option::Option<RoomMessageLog>,
    #[prost(message, optional, tag="11")]
    pub contact: ::std::option::Option<RoomMessageContact>,
    #[prost(message, optional, tag="22")]
    pub wallet: ::std::option::Option<RoomMessageWallet>,
    #[prost(bool, tag="12")]
    pub edited: bool,
    #[prost(uint32, tag="13")]
    pub create_time: u32,
    #[prost(uint32, tag="14")]
    pub update_time: u32,
    #[prost(bool, tag="15")]
    pub deleted: bool,
    #[prost(message, optional, boxed, tag="16")]
    pub forward_from: ::std::option::Option<::std::boxed::Box<RoomMessage>>,
    #[prost(message, optional, boxed, tag="17")]
    pub reply_to: ::std::option::Option<::std::boxed::Box<RoomMessage>>,
    #[prost(uint64, tag="18")]
    pub previous_message_id: u64,
    #[prost(uint64, tag="21")]
    pub random_id: u64,
    #[prost(uint32, tag="23")]
    pub additional_type: u32,
    #[prost(string, tag="24")]
    pub additional_data: std::string::String,
    #[prost(enumeration="room_message::ExtraType", tag="19")]
    pub extra_type: i32,
    #[prost(message, optional, tag="20")]
    pub channel_extra: ::std::option::Option<room_message::ChannelExtra>,
}
pub mod room_message {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Author {
        #[prost(string, tag="1")]
        pub hash: std::string::String,
        #[prost(message, optional, tag="2")]
        pub user: ::std::option::Option<author::User>,
        #[prost(message, optional, tag="3")]
        pub room: ::std::option::Option<author::Room>,
    }
    pub mod author {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct User {
            #[prost(uint64, tag="1")]
            pub user_id: u64,
            #[prost(string, tag="2")]
            pub cache_id: std::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Room {
            #[prost(uint64, tag="1")]
            pub room_id: u64,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChannelExtra {
        #[prost(string, tag="1")]
        pub signature: std::string::String,
        #[prost(string, tag="2")]
        pub views_label: std::string::String,
        #[prost(string, tag="3")]
        pub thumbs_up_label: std::string::String,
        #[prost(string, tag="4")]
        pub thumbs_down_label: std::string::String,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExtraType {
        NoExtra = 0,
        ChannelExtra = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomDraft {
    #[prost(string, tag="1")]
    pub message: std::string::String,
    #[prost(uint64, tag="2")]
    pub reply_to: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Room {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(enumeration="room::Type", tag="2")]
    pub r#type: i32,
    #[prost(string, tag="3")]
    pub title: std::string::String,
    #[prost(string, tag="4")]
    pub initials: std::string::String,
    #[prost(string, tag="5")]
    pub color: std::string::String,
    #[prost(uint32, tag="6")]
    pub unread_count: u32,
    #[prost(message, optional, tag="7")]
    pub last_message: ::std::option::Option<RoomMessage>,
    #[prost(bool, tag="8")]
    pub read_only: bool,
    #[prost(bool, tag="9")]
    pub is_participant: bool,
    #[prost(message, optional, tag="10")]
    pub draft: ::std::option::Option<RoomDraft>,
    #[prost(message, optional, tag="14")]
    pub first_unread_message: ::std::option::Option<RoomMessage>,
    #[prost(enumeration="RoomMute", tag="15")]
    pub room_mute: i32,
    #[prost(uint64, tag="16")]
    pub pin_id: u64,
    #[prost(message, optional, tag="17")]
    pub pinned_message: ::std::option::Option<RoomMessage>,
    #[prost(uint32, tag="18")]
    pub priority: u32,
    #[prost(message, optional, tag="11")]
    pub chat_room_extra: ::std::option::Option<ChatRoom>,
    #[prost(message, optional, tag="12")]
    pub group_room_extra: ::std::option::Option<GroupRoom>,
    #[prost(message, optional, tag="13")]
    pub channel_room_extra: ::std::option::Option<ChannelRoom>,
}
pub mod room {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Chat = 0,
        Group = 1,
        Channel = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatRoom {
    #[prost(message, optional, tag="1")]
    pub peer: ::std::option::Option<RegisteredUser>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRoom {
    #[prost(enumeration="group_room::Type", tag="1")]
    pub r#type: i32,
    #[prost(enumeration="group_room::Role", tag="2")]
    pub role: i32,
    #[prost(uint32, tag="3")]
    pub participants_count: u32,
    #[prost(string, tag="4")]
    pub participants_count_label: std::string::String,
    #[prost(uint32, tag="5")]
    pub participants_count_limit: u32,
    #[prost(string, tag="6")]
    pub participants_count_limit_label: std::string::String,
    #[prost(string, tag="7")]
    pub description: std::string::String,
    #[prost(uint32, tag="8")]
    pub avatar_count: u32,
    #[prost(message, optional, tag="9")]
    pub avatar: ::std::option::Option<Avatar>,
    #[prost(message, optional, tag="10")]
    pub private_extra: ::std::option::Option<group_room::PrivateExtra>,
    #[prost(message, optional, tag="11")]
    pub public_extra: ::std::option::Option<group_room::PublicExtra>,
}
pub mod group_room {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrivateExtra {
        #[prost(string, tag="1")]
        pub invite_link: std::string::String,
        #[prost(string, tag="2")]
        pub invite_token: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PublicExtra {
        #[prost(string, tag="1")]
        pub username: std::string::String,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        PrivateRoom = 0,
        PublicRoom = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Role {
        Member = 0,
        Moderator = 1,
        Admin = 2,
        Owner = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRoom {
    #[prost(enumeration="channel_room::Type", tag="1")]
    pub r#type: i32,
    #[prost(enumeration="channel_room::Role", tag="2")]
    pub role: i32,
    #[prost(uint32, tag="3")]
    pub participants_count: u32,
    #[prost(string, tag="4")]
    pub participants_count_label: std::string::String,
    #[prost(string, tag="5")]
    pub description: std::string::String,
    #[prost(uint32, tag="6")]
    pub avatar_count: u32,
    #[prost(message, optional, tag="7")]
    pub avatar: ::std::option::Option<Avatar>,
    #[prost(message, optional, tag="8")]
    pub private_extra: ::std::option::Option<channel_room::PrivateExtra>,
    #[prost(message, optional, tag="9")]
    pub public_extra: ::std::option::Option<channel_room::PublicExtra>,
    #[prost(bool, tag="10")]
    pub signature: bool,
    #[prost(uint64, tag="11")]
    pub seen_id: u64,
    #[prost(bool, tag="12")]
    pub verified: bool,
    #[prost(bool, tag="13")]
    pub reaction_status: bool,
}
pub mod channel_room {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrivateExtra {
        #[prost(string, tag="1")]
        pub invite_link: std::string::String,
        #[prost(string, tag="2")]
        pub invite_token: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PublicExtra {
        #[prost(string, tag="1")]
        pub username: std::string::String,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        PrivateRoom = 0,
        PublicRoom = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Role {
        Member = 0,
        Moderator = 1,
        Admin = 2,
        Owner = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Thumbnail {
    #[prost(int64, tag="1")]
    pub size: i64,
    #[prost(int32, tag="2")]
    pub width: i32,
    #[prost(int32, tag="3")]
    pub height: i32,
    #[prost(string, tag="4")]
    pub cache_id: std::string::String,
    #[prost(string, tag="5")]
    pub name: std::string::String,
    #[prost(string, tag="6")]
    pub mime: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    #[prost(string, tag="1")]
    pub token: std::string::String,
    #[prost(string, tag="2")]
    pub name: std::string::String,
    #[prost(int64, tag="3")]
    pub size: i64,
    #[prost(message, optional, tag="4")]
    pub large_thumbnail: ::std::option::Option<Thumbnail>,
    #[prost(message, optional, tag="5")]
    pub small_thumbnail: ::std::option::Option<Thumbnail>,
    #[prost(message, optional, tag="6")]
    pub waveform_thumbnail: ::std::option::Option<Thumbnail>,
    #[prost(int32, tag="7")]
    pub width: i32,
    #[prost(int32, tag="8")]
    pub height: i32,
    #[prost(double, tag="9")]
    pub duration: f64,
    #[prost(string, tag="10")]
    pub cache_id: std::string::String,
    #[prost(string, tag="11")]
    pub mime: std::string::String,
    #[prost(string, tag="12")]
    pub public_url: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wallpaper {
    #[prost(message, optional, tag="1")]
    pub file: ::std::option::Option<File>,
    #[prost(string, tag="2")]
    pub color: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pagination {
    #[prost(uint32, tag="1")]
    pub offset: u32,
    #[prost(uint32, tag="2")]
    pub limit: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Gender {
    Unknown = 0,
    Male = 1,
    Female = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Device {
    UnknownDevice = 0,
    Pc = 1,
    Tablet = 2,
    Mobile = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Platform {
    UnknownPlatform = 0,
    Android = 1,
    Ios = 2,
    MacOs = 3,
    Windows = 4,
    Linux = 5,
    BlackBerry = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Language {
    EnUs = 0,
    FaIr = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomMessageType {
    Text = 0,
    Image = 1,
    ImageText = 2,
    Video = 3,
    VideoText = 4,
    Audio = 5,
    AudioText = 6,
    Voice = 7,
    Gif = 8,
    GifText = 14,
    File = 9,
    FileText = 10,
    Location = 11,
    Log = 12,
    Contact = 13,
    Wallet = 15,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomMessageStatus {
    Failed = 0,
    Sending = 1,
    Sent = 2,
    Delivered = 3,
    Seen = 4,
    Listened = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomMessageReaction {
    ThumbsUp = 0,
    ThumbsDown = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientAction {
    Cancel = 0,
    Typing = 1,
    SendingImage = 2,
    CapturingImage = 3,
    SendingVideo = 4,
    CapturingVideo = 5,
    SendingAudio = 6,
    RecordingVoice = 7,
    SendingVoice = 8,
    SendingDocument = 9,
    SendingGif = 10,
    SendingFile = 11,
    SendingLocation = 12,
    ChoosingContact = 13,
    Painting = 14,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomMute {
    Unmute = 0,
    Mute = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PrivacyType {
    UserStatus = 0,
    Avatar = 1,
    GroupInvite = 2,
    ChannelInvite = 3,
    VoiceCalling = 4,
    VideoCalling = 5,
    ScreenSharing = 6,
    SecretChat = 7,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PrivacyLevel {
    AllowAll = 0,
    DenyAll = 1,
    AllowContacts = 2,
}
