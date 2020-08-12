// Automatically generated rust module for 'store.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageType {
    TEXT = 0,
    IMAGE = 1,
    VIDEO = 3,
    AUDIO = 5,
    VOICE = 7,
    GIF = 8,
    FILE = 9,
    LOCATION = 11,
    LOG = 12,
    CONTACT = 13,
    WALLET = 15,
    PRODUCT = 16,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::TEXT
    }
}

impl From<i32> for MessageType {
    fn from(i: i32) -> Self {
        match i {
            0 => MessageType::TEXT,
            1 => MessageType::IMAGE,
            3 => MessageType::VIDEO,
            5 => MessageType::AUDIO,
            7 => MessageType::VOICE,
            8 => MessageType::GIF,
            9 => MessageType::FILE,
            11 => MessageType::LOCATION,
            12 => MessageType::LOG,
            13 => MessageType::CONTACT,
            15 => MessageType::WALLET,
            16 => MessageType::PRODUCT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for MessageType {
    fn from(s: &'a str) -> Self {
        match s {
            "TEXT" => MessageType::TEXT,
            "IMAGE" => MessageType::IMAGE,
            "VIDEO" => MessageType::VIDEO,
            "AUDIO" => MessageType::AUDIO,
            "VOICE" => MessageType::VOICE,
            "GIF" => MessageType::GIF,
            "FILE" => MessageType::FILE,
            "LOCATION" => MessageType::LOCATION,
            "LOG" => MessageType::LOG,
            "CONTACT" => MessageType::CONTACT,
            "WALLET" => MessageType::WALLET,
            "PRODUCT" => MessageType::PRODUCT,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageDeliveryStatues {
    FAILED = 0,
    SENDING = 1,
    SENT = 2,
    DELIVERED = 3,
    SEEN = 4,
    CONSUMED = 5,
}

impl Default for MessageDeliveryStatues {
    fn default() -> Self {
        MessageDeliveryStatues::FAILED
    }
}

impl From<i32> for MessageDeliveryStatues {
    fn from(i: i32) -> Self {
        match i {
            0 => MessageDeliveryStatues::FAILED,
            1 => MessageDeliveryStatues::SENDING,
            2 => MessageDeliveryStatues::SENT,
            3 => MessageDeliveryStatues::DELIVERED,
            4 => MessageDeliveryStatues::SEEN,
            5 => MessageDeliveryStatues::CONSUMED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for MessageDeliveryStatues {
    fn from(s: &'a str) -> Self {
        match s {
            "FAILED" => MessageDeliveryStatues::FAILED,
            "SENDING" => MessageDeliveryStatues::SENDING,
            "SENT" => MessageDeliveryStatues::SENT,
            "DELIVERED" => MessageDeliveryStatues::DELIVERED,
            "SEEN" => MessageDeliveryStatues::SEEN,
            "CONSUMED" => MessageDeliveryStatues::CONSUMED,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageLogType {
    USER_JOINED = 0,
    USER_DELETED = 1,
    ROOM_CREATED = 2,
    MEMBER_ADDED = 3,
    MEMBER_KICKED = 4,
    MEMBER_LEFT = 5,
    ROOM_CONVERTED_TO_PUBLIC = 6,
    ROOM_CONVERTED_TO_PRIVATE = 7,
    MEMBER_JOINED_BY_INVITE_LINK = 8,
    ROOM_DELETED = 9,
    MISSED_VOICE_CALL = 10,
    MISSED_VIDEO_CALL = 11,
    MISSED_SCREEN_SHARE = 12,
    MISSED_SECRET_CHAT = 13,
    PINNED_MESSAGE = 14,
}

impl Default for MessageLogType {
    fn default() -> Self {
        MessageLogType::USER_JOINED
    }
}

impl From<i32> for MessageLogType {
    fn from(i: i32) -> Self {
        match i {
            0 => MessageLogType::USER_JOINED,
            1 => MessageLogType::USER_DELETED,
            2 => MessageLogType::ROOM_CREATED,
            3 => MessageLogType::MEMBER_ADDED,
            4 => MessageLogType::MEMBER_KICKED,
            5 => MessageLogType::MEMBER_LEFT,
            6 => MessageLogType::ROOM_CONVERTED_TO_PUBLIC,
            7 => MessageLogType::ROOM_CONVERTED_TO_PRIVATE,
            8 => MessageLogType::MEMBER_JOINED_BY_INVITE_LINK,
            9 => MessageLogType::ROOM_DELETED,
            10 => MessageLogType::MISSED_VOICE_CALL,
            11 => MessageLogType::MISSED_VIDEO_CALL,
            12 => MessageLogType::MISSED_SCREEN_SHARE,
            13 => MessageLogType::MISSED_SECRET_CHAT,
            14 => MessageLogType::PINNED_MESSAGE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for MessageLogType {
    fn from(s: &'a str) -> Self {
        match s {
            "USER_JOINED" => MessageLogType::USER_JOINED,
            "USER_DELETED" => MessageLogType::USER_DELETED,
            "ROOM_CREATED" => MessageLogType::ROOM_CREATED,
            "MEMBER_ADDED" => MessageLogType::MEMBER_ADDED,
            "MEMBER_KICKED" => MessageLogType::MEMBER_KICKED,
            "MEMBER_LEFT" => MessageLogType::MEMBER_LEFT,
            "ROOM_CONVERTED_TO_PUBLIC" => MessageLogType::ROOM_CONVERTED_TO_PUBLIC,
            "ROOM_CONVERTED_TO_PRIVATE" => MessageLogType::ROOM_CONVERTED_TO_PRIVATE,
            "MEMBER_JOINED_BY_INVITE_LINK" => MessageLogType::MEMBER_JOINED_BY_INVITE_LINK,
            "ROOM_DELETED" => MessageLogType::ROOM_DELETED,
            "MISSED_VOICE_CALL" => MessageLogType::MISSED_VOICE_CALL,
            "MISSED_VIDEO_CALL" => MessageLogType::MISSED_VIDEO_CALL,
            "MISSED_SCREEN_SHARE" => MessageLogType::MISSED_SCREEN_SHARE,
            "MISSED_SECRET_CHAT" => MessageLogType::MISSED_SECRET_CHAT,
            "PINNED_MESSAGE" => MessageLogType::PINNED_MESSAGE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChannelType {
    CHANNEL_UNKNOWN = 0,
    CHANNEL_PRIMARY = 1,
    CHANNEL_NORMAL = 2,
    CHANNEL_SAVES = 3,
    CHANNEL_STORE = 4,
}

impl Default for ChannelType {
    fn default() -> Self {
        ChannelType::CHANNEL_UNKNOWN
    }
}

impl From<i32> for ChannelType {
    fn from(i: i32) -> Self {
        match i {
            0 => ChannelType::CHANNEL_UNKNOWN,
            1 => ChannelType::CHANNEL_PRIMARY,
            2 => ChannelType::CHANNEL_NORMAL,
            3 => ChannelType::CHANNEL_SAVES,
            4 => ChannelType::CHANNEL_STORE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ChannelType {
    fn from(s: &'a str) -> Self {
        match s {
            "CHANNEL_UNKNOWN" => ChannelType::CHANNEL_UNKNOWN,
            "CHANNEL_PRIMARY" => ChannelType::CHANNEL_PRIMARY,
            "CHANNEL_NORMAL" => ChannelType::CHANNEL_NORMAL,
            "CHANNEL_SAVES" => ChannelType::CHANNEL_SAVES,
            "CHANNEL_STORE" => ChannelType::CHANNEL_STORE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChannelPrivacy {
    CHANNEL_UNKNOWN_AB = 0,
    CHANNEL_OPEN = 1,
    CHANNEL_PRIVATE_LINK = 2,
    CHANNEL_CREATOR = 3,
}

impl Default for ChannelPrivacy {
    fn default() -> Self {
        ChannelPrivacy::CHANNEL_UNKNOWN_AB
    }
}

impl From<i32> for ChannelPrivacy {
    fn from(i: i32) -> Self {
        match i {
            0 => ChannelPrivacy::CHANNEL_UNKNOWN_AB,
            1 => ChannelPrivacy::CHANNEL_OPEN,
            2 => ChannelPrivacy::CHANNEL_PRIVATE_LINK,
            3 => ChannelPrivacy::CHANNEL_CREATOR,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ChannelPrivacy {
    fn from(s: &'a str) -> Self {
        match s {
            "CHANNEL_UNKNOWN_AB" => ChannelPrivacy::CHANNEL_UNKNOWN_AB,
            "CHANNEL_OPEN" => ChannelPrivacy::CHANNEL_OPEN,
            "CHANNEL_PRIVATE_LINK" => ChannelPrivacy::CHANNEL_PRIVATE_LINK,
            "CHANNEL_CREATOR" => ChannelPrivacy::CHANNEL_CREATOR,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cast {
    UNKNOWN = 0,
    USER = 1,
    CHANNEL = 2,
    GROUP = 3,
    BOT = 4,
    STORE = 6,
    TAG = 7,
}

impl Default for Cast {
    fn default() -> Self {
        Cast::UNKNOWN
    }
}

impl From<i32> for Cast {
    fn from(i: i32) -> Self {
        match i {
            0 => Cast::UNKNOWN,
            1 => Cast::USER,
            2 => Cast::CHANNEL,
            3 => Cast::GROUP,
            4 => Cast::BOT,
            6 => Cast::STORE,
            7 => Cast::TAG,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Cast {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Cast::UNKNOWN,
            "USER" => Cast::USER,
            "CHANNEL" => Cast::CHANNEL,
            "GROUP" => Cast::GROUP,
            "BOT" => Cast::BOT,
            "STORE" => Cast::STORE,
            "TAG" => Cast::TAG,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReactionType {
    NONE = 0,
    LIKE = 1,
}

impl Default for ReactionType {
    fn default() -> Self {
        ReactionType::NONE
    }
}

impl From<i32> for ReactionType {
    fn from(i: i32) -> Self {
        match i {
            0 => ReactionType::NONE,
            1 => ReactionType::LIKE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ReactionType {
    fn from(s: &'a str) -> Self {
        match s {
            "NONE" => ReactionType::NONE,
            "LIKE" => ReactionType::LIKE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SessionPlatform {
    UNKNOWN_PLATFORM = 0,
    ANDROID = 1,
    IOS = 2,
    MAC_OS = 3,
    WINDOWS = 4,
    LINUX = 5,
    BLACK_BERRY = 6,
    Web = 7,
}

impl Default for SessionPlatform {
    fn default() -> Self {
        SessionPlatform::UNKNOWN_PLATFORM
    }
}

impl From<i32> for SessionPlatform {
    fn from(i: i32) -> Self {
        match i {
            0 => SessionPlatform::UNKNOWN_PLATFORM,
            1 => SessionPlatform::ANDROID,
            2 => SessionPlatform::IOS,
            3 => SessionPlatform::MAC_OS,
            4 => SessionPlatform::WINDOWS,
            5 => SessionPlatform::LINUX,
            6 => SessionPlatform::BLACK_BERRY,
            7 => SessionPlatform::Web,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SessionPlatform {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN_PLATFORM" => SessionPlatform::UNKNOWN_PLATFORM,
            "ANDROID" => SessionPlatform::ANDROID,
            "IOS" => SessionPlatform::IOS,
            "MAC_OS" => SessionPlatform::MAC_OS,
            "WINDOWS" => SessionPlatform::WINDOWS,
            "LINUX" => SessionPlatform::LINUX,
            "BLACK_BERRY" => SessionPlatform::BLACK_BERRY,
            "Web" => SessionPlatform::Web,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GroupPrivacy {
    GROUP_UNKNOWN_GP = 0,
    GROUP_OPEN = 1,
    GROUP_PRIVATE_LINK = 2,
}

impl Default for GroupPrivacy {
    fn default() -> Self {
        GroupPrivacy::GROUP_UNKNOWN_GP
    }
}

impl From<i32> for GroupPrivacy {
    fn from(i: i32) -> Self {
        match i {
            0 => GroupPrivacy::GROUP_UNKNOWN_GP,
            1 => GroupPrivacy::GROUP_OPEN,
            2 => GroupPrivacy::GROUP_PRIVATE_LINK,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for GroupPrivacy {
    fn from(s: &'a str) -> Self {
        match s {
            "GROUP_UNKNOWN_GP" => GroupPrivacy::GROUP_UNKNOWN_GP,
            "GROUP_OPEN" => GroupPrivacy::GROUP_OPEN,
            "GROUP_PRIVATE_LINK" => GroupPrivacy::GROUP_PRIVATE_LINK,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GroupMemberRole {
    MEMBER_Unknown = 0,
    MEMBER_CREATOR = 1,
    MEMBER_MODERATOR = 3,
    MEMBER_NORMAL_USER = 2,
}

impl Default for GroupMemberRole {
    fn default() -> Self {
        GroupMemberRole::MEMBER_Unknown
    }
}

impl From<i32> for GroupMemberRole {
    fn from(i: i32) -> Self {
        match i {
            0 => GroupMemberRole::MEMBER_Unknown,
            1 => GroupMemberRole::MEMBER_CREATOR,
            3 => GroupMemberRole::MEMBER_MODERATOR,
            2 => GroupMemberRole::MEMBER_NORMAL_USER,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for GroupMemberRole {
    fn from(s: &'a str) -> Self {
        match s {
            "MEMBER_Unknown" => GroupMemberRole::MEMBER_Unknown,
            "MEMBER_CREATOR" => GroupMemberRole::MEMBER_CREATOR,
            "MEMBER_MODERATOR" => GroupMemberRole::MEMBER_MODERATOR,
            "MEMBER_NORMAL_USER" => GroupMemberRole::MEMBER_NORMAL_USER,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Message {
    pub gid: u64,
    pub by_user_cid: u32,
    pub by_channel_cid: u32,
    pub post_type: u32,
    pub media_id: i64,
    pub file_ref_id: i64,
    pub post_key: String,
    pub text: String,
    pub rich_text: String,
    pub shared_to: u32,
    pub via: u32,
    pub seq: u32,
    pub edited_time: u32,
    pub created_time: u32,
    pub delivery_status: MessageDeliveryStatues,
    pub delivery_time: u32,
    pub previous_message_id: u64,
    pub deleted: bool,
    pub forward_from: Option<Box<Message>>,
    pub reply_to: Option<Box<Message>>,
    pub counts: Option<MessageCount>,
    pub setting: Option<MessageSetting>,
    pub files: Vec<FileMsg>,
}

impl<'a> MessageRead<'a> for Message {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(16) => msg.by_user_cid = r.read_uint32(bytes)?,
                Ok(800) => msg.by_channel_cid = r.read_uint32(bytes)?,
                Ok(24) => msg.post_type = r.read_uint32(bytes)?,
                Ok(32) => msg.media_id = r.read_int64(bytes)?,
                Ok(40) => msg.file_ref_id = r.read_int64(bytes)?,
                Ok(50) => msg.post_key = r.read_string(bytes)?.to_owned(),
                Ok(58) => msg.text = r.read_string(bytes)?.to_owned(),
                Ok(66) => msg.rich_text = r.read_string(bytes)?.to_owned(),
                Ok(80) => msg.shared_to = r.read_uint32(bytes)?,
                Ok(96) => msg.via = r.read_uint32(bytes)?,
                Ok(104) => msg.seq = r.read_uint32(bytes)?,
                Ok(136) => msg.edited_time = r.read_uint32(bytes)?,
                Ok(144) => msg.created_time = r.read_uint32(bytes)?,
                Ok(840) => msg.delivery_status = r.read_enum(bytes)?,
                Ok(848) => msg.delivery_time = r.read_uint32(bytes)?,
                Ok(1440) => msg.previous_message_id = r.read_uint64(bytes)?,
                Ok(120) => msg.deleted = r.read_bool(bytes)?,
                Ok(130) => msg.forward_from = Some(Box::new(r.read_message::<Message>(bytes)?)),
                Ok(402) => msg.reply_to = Some(Box::new(r.read_message::<Message>(bytes)?)),
                Ok(810) => msg.counts = Some(r.read_message::<MessageCount>(bytes)?),
                Ok(818) => msg.setting = Some(r.read_message::<MessageSetting>(bytes)?),
                Ok(826) => msg.files.push(r.read_message::<FileMsg>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Message {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.by_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.by_user_cid) as u64) }
        + if self.by_channel_cid == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.by_channel_cid) as u64) }
        + if self.post_type == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.post_type) as u64) }
        + if self.media_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.media_id) as u64) }
        + if self.file_ref_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.file_ref_id) as u64) }
        + if self.post_key == String::default() { 0 } else { 1 + sizeof_len((&self.post_key).len()) }
        + if self.text == String::default() { 0 } else { 1 + sizeof_len((&self.text).len()) }
        + if self.rich_text == String::default() { 0 } else { 1 + sizeof_len((&self.rich_text).len()) }
        + if self.shared_to == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.shared_to) as u64) }
        + if self.via == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.via) as u64) }
        + if self.seq == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.edited_time == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.edited_time) as u64) }
        + if self.created_time == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.created_time) as u64) }
        + if self.delivery_status == store::MessageDeliveryStatues::FAILED { 0 } else { 2 + sizeof_varint(*(&self.delivery_status) as u64) }
        + if self.delivery_time == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.delivery_time) as u64) }
        + if self.previous_message_id == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.previous_message_id) as u64) }
        + if self.deleted == false { 0 } else { 1 + sizeof_varint(*(&self.deleted) as u64) }
        + self.forward_from.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.reply_to.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.counts.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.setting.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.files.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.by_user_cid != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.by_user_cid))?; }
        if self.by_channel_cid != 0u32 { w.write_with_tag(800, |w| w.write_uint32(*&self.by_channel_cid))?; }
        if self.post_type != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.post_type))?; }
        if self.media_id != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.media_id))?; }
        if self.file_ref_id != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.file_ref_id))?; }
        if self.post_key != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.post_key))?; }
        if self.text != String::default() { w.write_with_tag(58, |w| w.write_string(&**&self.text))?; }
        if self.rich_text != String::default() { w.write_with_tag(66, |w| w.write_string(&**&self.rich_text))?; }
        if self.shared_to != 0u32 { w.write_with_tag(80, |w| w.write_uint32(*&self.shared_to))?; }
        if self.via != 0u32 { w.write_with_tag(96, |w| w.write_uint32(*&self.via))?; }
        if self.seq != 0u32 { w.write_with_tag(104, |w| w.write_uint32(*&self.seq))?; }
        if self.edited_time != 0u32 { w.write_with_tag(136, |w| w.write_uint32(*&self.edited_time))?; }
        if self.created_time != 0u32 { w.write_with_tag(144, |w| w.write_uint32(*&self.created_time))?; }
        if self.delivery_status != store::MessageDeliveryStatues::FAILED { w.write_with_tag(840, |w| w.write_enum(*&self.delivery_status as i32))?; }
        if self.delivery_time != 0u32 { w.write_with_tag(848, |w| w.write_uint32(*&self.delivery_time))?; }
        if self.previous_message_id != 0u64 { w.write_with_tag(1440, |w| w.write_uint64(*&self.previous_message_id))?; }
        if self.deleted != false { w.write_with_tag(120, |w| w.write_bool(*&self.deleted))?; }
        if let Some(ref s) = self.forward_from { w.write_with_tag(130, |w| w.write_message(&**s))?; }
        if let Some(ref s) = self.reply_to { w.write_with_tag(402, |w| w.write_message(&**s))?; }
        if let Some(ref s) = self.counts { w.write_with_tag(810, |w| w.write_message(s))?; }
        if let Some(ref s) = self.setting { w.write_with_tag(818, |w| w.write_message(s))?; }
        for s in &self.files { w.write_with_tag(826, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageCount {
    pub message_gid: u64,
    pub comments_count: u32,
    pub likes_count: u32,
    pub views_count: i64,
    pub re_shared_count: u32,
    pub chat_shared_count: u32,
}

impl<'a> MessageRead<'a> for MessageCount {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.message_gid = r.read_fixed64(bytes)?,
                Ok(16) => msg.comments_count = r.read_uint32(bytes)?,
                Ok(24) => msg.likes_count = r.read_uint32(bytes)?,
                Ok(32) => msg.views_count = r.read_int64(bytes)?,
                Ok(40) => msg.re_shared_count = r.read_uint32(bytes)?,
                Ok(48) => msg.chat_shared_count = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MessageCount {
    fn get_size(&self) -> usize {
        0
        + if self.message_gid == 0u64 { 0 } else { 1 + 8 }
        + if self.comments_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.comments_count) as u64) }
        + if self.likes_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.likes_count) as u64) }
        + if self.views_count == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.views_count) as u64) }
        + if self.re_shared_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.re_shared_count) as u64) }
        + if self.chat_shared_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.chat_shared_count) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.message_gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.message_gid))?; }
        if self.comments_count != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.comments_count))?; }
        if self.likes_count != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.likes_count))?; }
        if self.views_count != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.views_count))?; }
        if self.re_shared_count != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.re_shared_count))?; }
        if self.chat_shared_count != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.chat_shared_count))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageSetting {
    pub disable_comment: u32,
}

impl<'a> MessageRead<'a> for MessageSetting {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(88) => msg.disable_comment = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MessageSetting {
    fn get_size(&self) -> usize {
        0
        + if self.disable_comment == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.disable_comment) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.disable_comment != 0u32 { w.write_with_tag(88, |w| w.write_uint32(*&self.disable_comment))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageLog {
    pub log_type: MessageLogType,
    pub by_user_cid: u32,
    pub by_channel_cid: u32,
    pub target_user_cid: u32,
    pub target_channel_cid: u32,
    pub target_channel_view: Option<Channel>,
}

impl<'a> MessageRead<'a> for MessageLog {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.log_type = r.read_enum(bytes)?,
                Ok(16) => msg.by_user_cid = r.read_uint32(bytes)?,
                Ok(400) => msg.by_channel_cid = r.read_uint32(bytes)?,
                Ok(24) => msg.target_user_cid = r.read_uint32(bytes)?,
                Ok(32) => msg.target_channel_cid = r.read_uint32(bytes)?,
                Ok(90) => msg.target_channel_view = Some(r.read_message::<Channel>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MessageLog {
    fn get_size(&self) -> usize {
        0
        + if self.log_type == store::MessageLogType::USER_JOINED { 0 } else { 1 + sizeof_varint(*(&self.log_type) as u64) }
        + if self.by_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.by_user_cid) as u64) }
        + if self.by_channel_cid == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.by_channel_cid) as u64) }
        + if self.target_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.target_user_cid) as u64) }
        + if self.target_channel_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.target_channel_cid) as u64) }
        + self.target_channel_view.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.log_type != store::MessageLogType::USER_JOINED { w.write_with_tag(80, |w| w.write_enum(*&self.log_type as i32))?; }
        if self.by_user_cid != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.by_user_cid))?; }
        if self.by_channel_cid != 0u32 { w.write_with_tag(400, |w| w.write_uint32(*&self.by_channel_cid))?; }
        if self.target_user_cid != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.target_user_cid))?; }
        if self.target_channel_cid != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.target_channel_cid))?; }
        if let Some(ref s) = self.target_channel_view { w.write_with_tag(90, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Channel {
    pub cid: u32,
    pub user_name: String,
    pub first_name: String,
    pub last_name: String,
    pub creator_user_cid: u32,
    pub is_verified: u32,
    pub avatar_count: i64,
    pub about: String,
    pub invite_link_hash: String,
    pub post_seq: u32,
    pub sort_time: u64,
    pub sync_time: u64,
    pub created_time: u32,
    pub is_deleted: u32,
    pub is_banned: u32,
    pub notification_setting: Option<ChannelNotificationSetting>,
    pub privacy: ChannelPrivacy,
    pub channel_type: ChannelType,
    pub counts: Option<ChannelCounts>,
    pub last_message: Option<Message>,
    pub pinned_message: Option<Message>,
    pub avatar: Option<Message>,
}

impl<'a> MessageRead<'a> for Channel {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.cid = r.read_uint32(bytes)?,
                Ok(18) => msg.user_name = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.first_name = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.last_name = r.read_string(bytes)?.to_owned(),
                Ok(56) => msg.creator_user_cid = r.read_uint32(bytes)?,
                Ok(48) => msg.is_verified = r.read_uint32(bytes)?,
                Ok(320) => msg.avatar_count = r.read_int64(bytes)?,
                Ok(130) => msg.about = r.read_string(bytes)?.to_owned(),
                Ok(138) => msg.invite_link_hash = r.read_string(bytes)?.to_owned(),
                Ok(152) => msg.post_seq = r.read_uint32(bytes)?,
                Ok(161) => msg.sort_time = r.read_fixed64(bytes)?,
                Ok(169) => msg.sync_time = r.read_fixed64(bytes)?,
                Ok(288) => msg.created_time = r.read_uint32(bytes)?,
                Ok(304) => msg.is_deleted = r.read_uint32(bytes)?,
                Ok(312) => msg.is_banned = r.read_uint32(bytes)?,
                Ok(722) => msg.notification_setting = Some(r.read_message::<ChannelNotificationSetting>(bytes)?),
                Ok(72) => msg.privacy = r.read_enum(bytes)?,
                Ok(336) => msg.channel_type = r.read_enum(bytes)?,
                Ok(330) => msg.counts = Some(r.read_message::<ChannelCounts>(bytes)?),
                Ok(202) => msg.last_message = Some(r.read_message::<Message>(bytes)?),
                Ok(210) => msg.pinned_message = Some(r.read_message::<Message>(bytes)?),
                Ok(802) => msg.avatar = Some(r.read_message::<Message>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Channel {
    fn get_size(&self) -> usize {
        0
        + if self.cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.cid) as u64) }
        + if self.user_name == String::default() { 0 } else { 1 + sizeof_len((&self.user_name).len()) }
        + if self.first_name == String::default() { 0 } else { 1 + sizeof_len((&self.first_name).len()) }
        + if self.last_name == String::default() { 0 } else { 1 + sizeof_len((&self.last_name).len()) }
        + if self.creator_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.creator_user_cid) as u64) }
        + if self.is_verified == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.is_verified) as u64) }
        + if self.avatar_count == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.avatar_count) as u64) }
        + if self.about == String::default() { 0 } else { 2 + sizeof_len((&self.about).len()) }
        + if self.invite_link_hash == String::default() { 0 } else { 2 + sizeof_len((&self.invite_link_hash).len()) }
        + if self.post_seq == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.post_seq) as u64) }
        + if self.sort_time == 0u64 { 0 } else { 2 + 8 }
        + if self.sync_time == 0u64 { 0 } else { 2 + 8 }
        + if self.created_time == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.created_time) as u64) }
        + if self.is_deleted == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.is_deleted) as u64) }
        + if self.is_banned == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.is_banned) as u64) }
        + self.notification_setting.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + if self.privacy == store::ChannelPrivacy::CHANNEL_UNKNOWN_AB { 0 } else { 1 + sizeof_varint(*(&self.privacy) as u64) }
        + if self.channel_type == store::ChannelType::CHANNEL_UNKNOWN { 0 } else { 2 + sizeof_varint(*(&self.channel_type) as u64) }
        + self.counts.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.last_message.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.pinned_message.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.avatar.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.cid != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.cid))?; }
        if self.user_name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.user_name))?; }
        if self.first_name != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.first_name))?; }
        if self.last_name != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.last_name))?; }
        if self.creator_user_cid != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.creator_user_cid))?; }
        if self.is_verified != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.is_verified))?; }
        if self.avatar_count != 0i64 { w.write_with_tag(320, |w| w.write_int64(*&self.avatar_count))?; }
        if self.about != String::default() { w.write_with_tag(130, |w| w.write_string(&**&self.about))?; }
        if self.invite_link_hash != String::default() { w.write_with_tag(138, |w| w.write_string(&**&self.invite_link_hash))?; }
        if self.post_seq != 0u32 { w.write_with_tag(152, |w| w.write_uint32(*&self.post_seq))?; }
        if self.sort_time != 0u64 { w.write_with_tag(161, |w| w.write_fixed64(*&self.sort_time))?; }
        if self.sync_time != 0u64 { w.write_with_tag(169, |w| w.write_fixed64(*&self.sync_time))?; }
        if self.created_time != 0u32 { w.write_with_tag(288, |w| w.write_uint32(*&self.created_time))?; }
        if self.is_deleted != 0u32 { w.write_with_tag(304, |w| w.write_uint32(*&self.is_deleted))?; }
        if self.is_banned != 0u32 { w.write_with_tag(312, |w| w.write_uint32(*&self.is_banned))?; }
        if let Some(ref s) = self.notification_setting { w.write_with_tag(722, |w| w.write_message(s))?; }
        if self.privacy != store::ChannelPrivacy::CHANNEL_UNKNOWN_AB { w.write_with_tag(72, |w| w.write_enum(*&self.privacy as i32))?; }
        if self.channel_type != store::ChannelType::CHANNEL_UNKNOWN { w.write_with_tag(336, |w| w.write_enum(*&self.channel_type as i32))?; }
        if let Some(ref s) = self.counts { w.write_with_tag(330, |w| w.write_message(s))?; }
        if let Some(ref s) = self.last_message { w.write_with_tag(202, |w| w.write_message(s))?; }
        if let Some(ref s) = self.pinned_message { w.write_with_tag(210, |w| w.write_message(s))?; }
        if let Some(ref s) = self.avatar { w.write_with_tag(802, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelPrivacySetting {
    pub online_privacy: u32,
    pub call_privacy: u32,
    pub add_to_group_privacy: u32,
    pub seen_message_privacy: u32,
}

impl<'a> MessageRead<'a> for ChannelPrivacySetting {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(80) => msg.online_privacy = r.read_uint32(bytes)?,
                Ok(88) => msg.call_privacy = r.read_uint32(bytes)?,
                Ok(96) => msg.add_to_group_privacy = r.read_uint32(bytes)?,
                Ok(104) => msg.seen_message_privacy = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelPrivacySetting {
    fn get_size(&self) -> usize {
        0
        + if self.online_privacy == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.online_privacy) as u64) }
        + if self.call_privacy == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.call_privacy) as u64) }
        + if self.add_to_group_privacy == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.add_to_group_privacy) as u64) }
        + if self.seen_message_privacy == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.seen_message_privacy) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.online_privacy != 0u32 { w.write_with_tag(80, |w| w.write_uint32(*&self.online_privacy))?; }
        if self.call_privacy != 0u32 { w.write_with_tag(88, |w| w.write_uint32(*&self.call_privacy))?; }
        if self.add_to_group_privacy != 0u32 { w.write_with_tag(96, |w| w.write_uint32(*&self.add_to_group_privacy))?; }
        if self.seen_message_privacy != 0u32 { w.write_with_tag(104, |w| w.write_uint32(*&self.seen_message_privacy))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelNotificationSetting {
    pub social_led_on: bool,
    pub social_led_color: String,
    pub request_to_follow_you: bool,
    pub followed_channel: bool,
    pub accepted_channel_follow_request: bool,
    pub channel_message_liked: bool,
    pub channel_message_commented: bool,
    pub mentioned_channel_in_message: bool,
    pub mentioned_channel_in_comment: bool,
    pub contacts_joined: bool,
    pub direct_message: bool,
    pub direct_alert: bool,
    pub direct_preview: bool,
    pub direct_led_on: bool,
    pub direct_led_color: bool,
    pub direct_vibrate: bool,
    pub direct_popup: bool,
    pub direct_sound: bool,
    pub direct_priority: bool,
}

impl<'a> MessageRead<'a> for ChannelNotificationSetting {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.social_led_on = r.read_bool(bytes)?,
                Ok(26) => msg.social_led_color = r.read_string(bytes)?.to_owned(),
                Ok(32) => msg.request_to_follow_you = r.read_bool(bytes)?,
                Ok(40) => msg.followed_channel = r.read_bool(bytes)?,
                Ok(48) => msg.accepted_channel_follow_request = r.read_bool(bytes)?,
                Ok(56) => msg.channel_message_liked = r.read_bool(bytes)?,
                Ok(64) => msg.channel_message_commented = r.read_bool(bytes)?,
                Ok(72) => msg.mentioned_channel_in_message = r.read_bool(bytes)?,
                Ok(80) => msg.mentioned_channel_in_comment = r.read_bool(bytes)?,
                Ok(88) => msg.contacts_joined = r.read_bool(bytes)?,
                Ok(96) => msg.direct_message = r.read_bool(bytes)?,
                Ok(104) => msg.direct_alert = r.read_bool(bytes)?,
                Ok(112) => msg.direct_preview = r.read_bool(bytes)?,
                Ok(120) => msg.direct_led_on = r.read_bool(bytes)?,
                Ok(128) => msg.direct_led_color = r.read_bool(bytes)?,
                Ok(136) => msg.direct_vibrate = r.read_bool(bytes)?,
                Ok(144) => msg.direct_popup = r.read_bool(bytes)?,
                Ok(152) => msg.direct_sound = r.read_bool(bytes)?,
                Ok(160) => msg.direct_priority = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelNotificationSetting {
    fn get_size(&self) -> usize {
        0
        + if self.social_led_on == false { 0 } else { 1 + sizeof_varint(*(&self.social_led_on) as u64) }
        + if self.social_led_color == String::default() { 0 } else { 1 + sizeof_len((&self.social_led_color).len()) }
        + if self.request_to_follow_you == false { 0 } else { 1 + sizeof_varint(*(&self.request_to_follow_you) as u64) }
        + if self.followed_channel == false { 0 } else { 1 + sizeof_varint(*(&self.followed_channel) as u64) }
        + if self.accepted_channel_follow_request == false { 0 } else { 1 + sizeof_varint(*(&self.accepted_channel_follow_request) as u64) }
        + if self.channel_message_liked == false { 0 } else { 1 + sizeof_varint(*(&self.channel_message_liked) as u64) }
        + if self.channel_message_commented == false { 0 } else { 1 + sizeof_varint(*(&self.channel_message_commented) as u64) }
        + if self.mentioned_channel_in_message == false { 0 } else { 1 + sizeof_varint(*(&self.mentioned_channel_in_message) as u64) }
        + if self.mentioned_channel_in_comment == false { 0 } else { 1 + sizeof_varint(*(&self.mentioned_channel_in_comment) as u64) }
        + if self.contacts_joined == false { 0 } else { 1 + sizeof_varint(*(&self.contacts_joined) as u64) }
        + if self.direct_message == false { 0 } else { 1 + sizeof_varint(*(&self.direct_message) as u64) }
        + if self.direct_alert == false { 0 } else { 1 + sizeof_varint(*(&self.direct_alert) as u64) }
        + if self.direct_preview == false { 0 } else { 1 + sizeof_varint(*(&self.direct_preview) as u64) }
        + if self.direct_led_on == false { 0 } else { 1 + sizeof_varint(*(&self.direct_led_on) as u64) }
        + if self.direct_led_color == false { 0 } else { 2 + sizeof_varint(*(&self.direct_led_color) as u64) }
        + if self.direct_vibrate == false { 0 } else { 2 + sizeof_varint(*(&self.direct_vibrate) as u64) }
        + if self.direct_popup == false { 0 } else { 2 + sizeof_varint(*(&self.direct_popup) as u64) }
        + if self.direct_sound == false { 0 } else { 2 + sizeof_varint(*(&self.direct_sound) as u64) }
        + if self.direct_priority == false { 0 } else { 2 + sizeof_varint(*(&self.direct_priority) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.social_led_on != false { w.write_with_tag(16, |w| w.write_bool(*&self.social_led_on))?; }
        if self.social_led_color != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.social_led_color))?; }
        if self.request_to_follow_you != false { w.write_with_tag(32, |w| w.write_bool(*&self.request_to_follow_you))?; }
        if self.followed_channel != false { w.write_with_tag(40, |w| w.write_bool(*&self.followed_channel))?; }
        if self.accepted_channel_follow_request != false { w.write_with_tag(48, |w| w.write_bool(*&self.accepted_channel_follow_request))?; }
        if self.channel_message_liked != false { w.write_with_tag(56, |w| w.write_bool(*&self.channel_message_liked))?; }
        if self.channel_message_commented != false { w.write_with_tag(64, |w| w.write_bool(*&self.channel_message_commented))?; }
        if self.mentioned_channel_in_message != false { w.write_with_tag(72, |w| w.write_bool(*&self.mentioned_channel_in_message))?; }
        if self.mentioned_channel_in_comment != false { w.write_with_tag(80, |w| w.write_bool(*&self.mentioned_channel_in_comment))?; }
        if self.contacts_joined != false { w.write_with_tag(88, |w| w.write_bool(*&self.contacts_joined))?; }
        if self.direct_message != false { w.write_with_tag(96, |w| w.write_bool(*&self.direct_message))?; }
        if self.direct_alert != false { w.write_with_tag(104, |w| w.write_bool(*&self.direct_alert))?; }
        if self.direct_preview != false { w.write_with_tag(112, |w| w.write_bool(*&self.direct_preview))?; }
        if self.direct_led_on != false { w.write_with_tag(120, |w| w.write_bool(*&self.direct_led_on))?; }
        if self.direct_led_color != false { w.write_with_tag(128, |w| w.write_bool(*&self.direct_led_color))?; }
        if self.direct_vibrate != false { w.write_with_tag(136, |w| w.write_bool(*&self.direct_vibrate))?; }
        if self.direct_popup != false { w.write_with_tag(144, |w| w.write_bool(*&self.direct_popup))?; }
        if self.direct_sound != false { w.write_with_tag(152, |w| w.write_bool(*&self.direct_sound))?; }
        if self.direct_priority != false { w.write_with_tag(160, |w| w.write_bool(*&self.direct_priority))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelCounts {
    pub followers_count: u32,
    pub following_count: u32,
    pub posts_count: u32,
    pub media_count: u32,
    pub photo_count: u32,
    pub video_count: u32,
    pub gif_count: u32,
    pub audio_count: u32,
    pub voice_count: u32,
    pub file_count: u32,
    pub link_count: u32,
    pub board_count: u32,
    pub pined_count: u32,
    pub likes_count: u32,
    pub reshared_count: u32,
}

impl<'a> MessageRead<'a> for ChannelCounts {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(160) => msg.followers_count = r.read_uint32(bytes)?,
                Ok(168) => msg.following_count = r.read_uint32(bytes)?,
                Ok(176) => msg.posts_count = r.read_uint32(bytes)?,
                Ok(184) => msg.media_count = r.read_uint32(bytes)?,
                Ok(192) => msg.photo_count = r.read_uint32(bytes)?,
                Ok(200) => msg.video_count = r.read_uint32(bytes)?,
                Ok(208) => msg.gif_count = r.read_uint32(bytes)?,
                Ok(216) => msg.audio_count = r.read_uint32(bytes)?,
                Ok(224) => msg.voice_count = r.read_uint32(bytes)?,
                Ok(232) => msg.file_count = r.read_uint32(bytes)?,
                Ok(240) => msg.link_count = r.read_uint32(bytes)?,
                Ok(248) => msg.board_count = r.read_uint32(bytes)?,
                Ok(256) => msg.pined_count = r.read_uint32(bytes)?,
                Ok(264) => msg.likes_count = r.read_uint32(bytes)?,
                Ok(272) => msg.reshared_count = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelCounts {
    fn get_size(&self) -> usize {
        0
        + if self.followers_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.followers_count) as u64) }
        + if self.following_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.following_count) as u64) }
        + if self.posts_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.posts_count) as u64) }
        + if self.media_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.media_count) as u64) }
        + if self.photo_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.photo_count) as u64) }
        + if self.video_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.video_count) as u64) }
        + if self.gif_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.gif_count) as u64) }
        + if self.audio_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.audio_count) as u64) }
        + if self.voice_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.voice_count) as u64) }
        + if self.file_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.file_count) as u64) }
        + if self.link_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.link_count) as u64) }
        + if self.board_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.board_count) as u64) }
        + if self.pined_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.pined_count) as u64) }
        + if self.likes_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.likes_count) as u64) }
        + if self.reshared_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.reshared_count) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.followers_count != 0u32 { w.write_with_tag(160, |w| w.write_uint32(*&self.followers_count))?; }
        if self.following_count != 0u32 { w.write_with_tag(168, |w| w.write_uint32(*&self.following_count))?; }
        if self.posts_count != 0u32 { w.write_with_tag(176, |w| w.write_uint32(*&self.posts_count))?; }
        if self.media_count != 0u32 { w.write_with_tag(184, |w| w.write_uint32(*&self.media_count))?; }
        if self.photo_count != 0u32 { w.write_with_tag(192, |w| w.write_uint32(*&self.photo_count))?; }
        if self.video_count != 0u32 { w.write_with_tag(200, |w| w.write_uint32(*&self.video_count))?; }
        if self.gif_count != 0u32 { w.write_with_tag(208, |w| w.write_uint32(*&self.gif_count))?; }
        if self.audio_count != 0u32 { w.write_with_tag(216, |w| w.write_uint32(*&self.audio_count))?; }
        if self.voice_count != 0u32 { w.write_with_tag(224, |w| w.write_uint32(*&self.voice_count))?; }
        if self.file_count != 0u32 { w.write_with_tag(232, |w| w.write_uint32(*&self.file_count))?; }
        if self.link_count != 0u32 { w.write_with_tag(240, |w| w.write_uint32(*&self.link_count))?; }
        if self.board_count != 0u32 { w.write_with_tag(248, |w| w.write_uint32(*&self.board_count))?; }
        if self.pined_count != 0u32 { w.write_with_tag(256, |w| w.write_uint32(*&self.pined_count))?; }
        if self.likes_count != 0u32 { w.write_with_tag(264, |w| w.write_uint32(*&self.likes_count))?; }
        if self.reshared_count != 0u32 { w.write_with_tag(272, |w| w.write_uint32(*&self.reshared_count))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Action {
    pub gid: u64,
    pub actor_user_cid: u32,
    pub actor_channel_cid: u32,
    pub action_type: u32,
    pub on_user_cid: u32,
    pub on_channel_cid: u64,
    pub message_gid: u64,
    pub comment_gid: u64,
    pub hash_murm64: i64,
    pub created_time: u32,
}

impl<'a> MessageRead<'a> for Action {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(16) => msg.actor_user_cid = r.read_uint32(bytes)?,
                Ok(400) => msg.actor_channel_cid = r.read_uint32(bytes)?,
                Ok(24) => msg.action_type = r.read_uint32(bytes)?,
                Ok(32) => msg.on_user_cid = r.read_uint32(bytes)?,
                Ok(73) => msg.on_channel_cid = r.read_fixed64(bytes)?,
                Ok(41) => msg.message_gid = r.read_fixed64(bytes)?,
                Ok(49) => msg.comment_gid = r.read_fixed64(bytes)?,
                Ok(56) => msg.hash_murm64 = r.read_int64(bytes)?,
                Ok(64) => msg.created_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Action {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.actor_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.actor_user_cid) as u64) }
        + if self.actor_channel_cid == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.actor_channel_cid) as u64) }
        + if self.action_type == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.action_type) as u64) }
        + if self.on_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.on_user_cid) as u64) }
        + if self.on_channel_cid == 0u64 { 0 } else { 1 + 8 }
        + if self.message_gid == 0u64 { 0 } else { 1 + 8 }
        + if self.comment_gid == 0u64 { 0 } else { 1 + 8 }
        + if self.hash_murm64 == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.hash_murm64) as u64) }
        + if self.created_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.actor_user_cid != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.actor_user_cid))?; }
        if self.actor_channel_cid != 0u32 { w.write_with_tag(400, |w| w.write_uint32(*&self.actor_channel_cid))?; }
        if self.action_type != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.action_type))?; }
        if self.on_user_cid != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.on_user_cid))?; }
        if self.on_channel_cid != 0u64 { w.write_with_tag(73, |w| w.write_fixed64(*&self.on_channel_cid))?; }
        if self.message_gid != 0u64 { w.write_with_tag(41, |w| w.write_fixed64(*&self.message_gid))?; }
        if self.comment_gid != 0u64 { w.write_with_tag(49, |w| w.write_fixed64(*&self.comment_gid))?; }
        if self.hash_murm64 != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.hash_murm64))?; }
        if self.created_time != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.created_time))?; }
        Ok(())
    }
}

pub mod mod_Action {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ActionType {
    UNKNOWN_AT = 0,
    LIKED = 1,
    FOLLOWED = 2,
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::UNKNOWN_AT
    }
}

impl From<i32> for ActionType {
    fn from(i: i32) -> Self {
        match i {
            0 => ActionType::UNKNOWN_AT,
            1 => ActionType::LIKED,
            2 => ActionType::FOLLOWED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ActionType {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN_AT" => ActionType::UNKNOWN_AT,
            "LIKED" => ActionType::LIKED,
            "FOLLOWED" => ActionType::FOLLOWED,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Blocked {
    pub gid: u64,
    pub user_cid: u32,
    pub blocked_user_cid: u32,
    pub blocked_channel_cid: u32,
    pub created_time: u32,
}

impl<'a> MessageRead<'a> for Blocked {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(16) => msg.user_cid = r.read_uint32(bytes)?,
                Ok(48) => msg.blocked_user_cid = r.read_uint32(bytes)?,
                Ok(56) => msg.blocked_channel_cid = r.read_uint32(bytes)?,
                Ok(40) => msg.created_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Blocked {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.user_cid) as u64) }
        + if self.blocked_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.blocked_user_cid) as u64) }
        + if self.blocked_channel_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.blocked_channel_cid) as u64) }
        + if self.created_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.user_cid != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.user_cid))?; }
        if self.blocked_user_cid != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.blocked_user_cid))?; }
        if self.blocked_channel_cid != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.blocked_channel_cid))?; }
        if self.created_time != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Comment {
    pub gid: u64,
    pub channel_cid: u32,
    pub by_cast_cid: u32,
    pub message_gid: u64,
    pub text: String,
    pub likes_count: u32,
    pub edit_time: u32,
    pub created_time: u32,
}

impl<'a> MessageRead<'a> for Comment {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(72) => msg.channel_cid = r.read_uint32(bytes)?,
                Ok(16) => msg.by_cast_cid = r.read_uint32(bytes)?,
                Ok(25) => msg.message_gid = r.read_fixed64(bytes)?,
                Ok(34) => msg.text = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.likes_count = r.read_uint32(bytes)?,
                Ok(48) => msg.edit_time = r.read_uint32(bytes)?,
                Ok(56) => msg.created_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Comment {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.channel_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel_cid) as u64) }
        + if self.by_cast_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.by_cast_cid) as u64) }
        + if self.message_gid == 0u64 { 0 } else { 1 + 8 }
        + if self.text == String::default() { 0 } else { 1 + sizeof_len((&self.text).len()) }
        + if self.likes_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.likes_count) as u64) }
        + if self.edit_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.edit_time) as u64) }
        + if self.created_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.channel_cid != 0u32 { w.write_with_tag(72, |w| w.write_uint32(*&self.channel_cid))?; }
        if self.by_cast_cid != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.by_cast_cid))?; }
        if self.message_gid != 0u64 { w.write_with_tag(25, |w| w.write_fixed64(*&self.message_gid))?; }
        if self.text != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.text))?; }
        if self.likes_count != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.likes_count))?; }
        if self.edit_time != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.edit_time))?; }
        if self.created_time != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Followed {
    pub gid: u64,
    pub by_user_cid: u32,
    pub by_channel_cid: u32,
    pub target_cid: u32,
    pub target_channel_id: u32,
    pub created_time: u32,
}

impl<'a> MessageRead<'a> for Followed {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(16) => msg.by_user_cid = r.read_uint32(bytes)?,
                Ok(80) => msg.by_channel_cid = r.read_uint32(bytes)?,
                Ok(24) => msg.target_cid = r.read_uint32(bytes)?,
                Ok(88) => msg.target_channel_id = r.read_uint32(bytes)?,
                Ok(32) => msg.created_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Followed {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.by_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.by_user_cid) as u64) }
        + if self.by_channel_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.by_channel_cid) as u64) }
        + if self.target_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.target_cid) as u64) }
        + if self.target_channel_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.target_channel_id) as u64) }
        + if self.created_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.by_user_cid != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.by_user_cid))?; }
        if self.by_channel_cid != 0u32 { w.write_with_tag(80, |w| w.write_uint32(*&self.by_channel_cid))?; }
        if self.target_cid != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.target_cid))?; }
        if self.target_channel_id != 0u32 { w.write_with_tag(88, |w| w.write_uint32(*&self.target_channel_id))?; }
        if self.created_time != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Reaction {
    pub gid: u64,
    pub for_message_cid: i64,
    pub for_channel_cid: i64,
    pub by_user_cid: u32,
    pub by_channel_cid: u32,
    pub created_time: u32,
}

impl<'a> MessageRead<'a> for Reaction {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(16) => msg.for_message_cid = r.read_int64(bytes)?,
                Ok(88) => msg.for_channel_cid = r.read_int64(bytes)?,
                Ok(24) => msg.by_user_cid = r.read_uint32(bytes)?,
                Ok(80) => msg.by_channel_cid = r.read_uint32(bytes)?,
                Ok(40) => msg.created_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Reaction {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.for_message_cid == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.for_message_cid) as u64) }
        + if self.for_channel_cid == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.for_channel_cid) as u64) }
        + if self.by_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.by_user_cid) as u64) }
        + if self.by_channel_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.by_channel_cid) as u64) }
        + if self.created_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.for_message_cid != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.for_message_cid))?; }
        if self.for_channel_cid != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.for_channel_cid))?; }
        if self.by_user_cid != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.by_user_cid))?; }
        if self.by_channel_cid != 0u32 { w.write_with_tag(80, |w| w.write_uint32(*&self.by_channel_cid))?; }
        if self.created_time != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Contact {
    pub gid: u64,
    pub user_cid: u32,
    pub channel_cid: u32,
    pub client_id: i64,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
    pub target_user_cid: u32,
    pub target_channel_cid: u32,
}

impl<'a> MessageRead<'a> for Contact {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(16) => msg.user_cid = r.read_uint32(bytes)?,
                Ok(80) => msg.channel_cid = r.read_uint32(bytes)?,
                Ok(24) => msg.client_id = r.read_int64(bytes)?,
                Ok(34) => msg.phone = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.first_name = r.read_string(bytes)?.to_owned(),
                Ok(50) => msg.last_name = r.read_string(bytes)?.to_owned(),
                Ok(96) => msg.target_user_cid = r.read_uint32(bytes)?,
                Ok(120) => msg.target_channel_cid = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Contact {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.user_cid) as u64) }
        + if self.channel_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel_cid) as u64) }
        + if self.client_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.client_id) as u64) }
        + if self.phone == String::default() { 0 } else { 1 + sizeof_len((&self.phone).len()) }
        + if self.first_name == String::default() { 0 } else { 1 + sizeof_len((&self.first_name).len()) }
        + if self.last_name == String::default() { 0 } else { 1 + sizeof_len((&self.last_name).len()) }
        + if self.target_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.target_user_cid) as u64) }
        + if self.target_channel_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.target_channel_cid) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.user_cid != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.user_cid))?; }
        if self.channel_cid != 0u32 { w.write_with_tag(80, |w| w.write_uint32(*&self.channel_cid))?; }
        if self.client_id != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.client_id))?; }
        if self.phone != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.phone))?; }
        if self.first_name != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.first_name))?; }
        if self.last_name != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.last_name))?; }
        if self.target_user_cid != 0u32 { w.write_with_tag(96, |w| w.write_uint32(*&self.target_user_cid))?; }
        if self.target_channel_cid != 0u32 { w.write_with_tag(120, |w| w.write_uint32(*&self.target_channel_cid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Session {
    pub gid: u64,
    pub session_uuid: String,
    pub user_cid: u32,
    pub last_ip_address: String,
    pub user_agent: String,
    pub platform: SessionPlatform,
    pub app_version: u32,
    pub active_time: u32,
    pub created_time: u32,
}

impl<'a> MessageRead<'a> for Session {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(18) => msg.session_uuid = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.user_cid = r.read_uint32(bytes)?,
                Ok(34) => msg.last_ip_address = r.read_string(bytes)?.to_owned(),
                Ok(66) => msg.user_agent = r.read_string(bytes)?.to_owned(),
                Ok(72) => msg.platform = r.read_enum(bytes)?,
                Ok(40) => msg.app_version = r.read_uint32(bytes)?,
                Ok(48) => msg.active_time = r.read_uint32(bytes)?,
                Ok(56) => msg.created_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Session {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.session_uuid == String::default() { 0 } else { 1 + sizeof_len((&self.session_uuid).len()) }
        + if self.user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.user_cid) as u64) }
        + if self.last_ip_address == String::default() { 0 } else { 1 + sizeof_len((&self.last_ip_address).len()) }
        + if self.user_agent == String::default() { 0 } else { 1 + sizeof_len((&self.user_agent).len()) }
        + if self.platform == store::SessionPlatform::UNKNOWN_PLATFORM { 0 } else { 1 + sizeof_varint(*(&self.platform) as u64) }
        + if self.app_version == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.app_version) as u64) }
        + if self.active_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.active_time) as u64) }
        + if self.created_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.session_uuid != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.session_uuid))?; }
        if self.user_cid != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.user_cid))?; }
        if self.last_ip_address != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.last_ip_address))?; }
        if self.user_agent != String::default() { w.write_with_tag(66, |w| w.write_string(&**&self.user_agent))?; }
        if self.platform != store::SessionPlatform::UNKNOWN_PLATFORM { w.write_with_tag(72, |w| w.write_enum(*&self.platform as i32))?; }
        if self.app_version != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.app_version))?; }
        if self.active_time != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.active_time))?; }
        if self.created_time != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sms {
    pub gid: u64,
    pub hash: String,
    pub app_uuid: String,
    pub client_phone: String,
    pub genrated_code: u32,
    pub sms_sender_number: String,
    pub sms_send_statues: String,
    pub sms_http_body: String,
    pub err: String,
    pub carrier: String,
    pub country: Vec<u8>,
    pub is_valid_phone: u32,
    pub is_confirmed: u32,
    pub is_login: u32,
    pub is_register: u32,
    pub retried_count: u32,
    pub ttl: u32,
}

impl<'a> MessageRead<'a> for Sms {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(18) => msg.hash = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.app_uuid = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.client_phone = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.genrated_code = r.read_uint32(bytes)?,
                Ok(50) => msg.sms_sender_number = r.read_string(bytes)?.to_owned(),
                Ok(58) => msg.sms_send_statues = r.read_string(bytes)?.to_owned(),
                Ok(66) => msg.sms_http_body = r.read_string(bytes)?.to_owned(),
                Ok(74) => msg.err = r.read_string(bytes)?.to_owned(),
                Ok(82) => msg.carrier = r.read_string(bytes)?.to_owned(),
                Ok(90) => msg.country = r.read_bytes(bytes)?.to_owned(),
                Ok(96) => msg.is_valid_phone = r.read_uint32(bytes)?,
                Ok(104) => msg.is_confirmed = r.read_uint32(bytes)?,
                Ok(112) => msg.is_login = r.read_uint32(bytes)?,
                Ok(120) => msg.is_register = r.read_uint32(bytes)?,
                Ok(128) => msg.retried_count = r.read_uint32(bytes)?,
                Ok(136) => msg.ttl = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Sms {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.hash == String::default() { 0 } else { 1 + sizeof_len((&self.hash).len()) }
        + if self.app_uuid == String::default() { 0 } else { 1 + sizeof_len((&self.app_uuid).len()) }
        + if self.client_phone == String::default() { 0 } else { 1 + sizeof_len((&self.client_phone).len()) }
        + if self.genrated_code == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.genrated_code) as u64) }
        + if self.sms_sender_number == String::default() { 0 } else { 1 + sizeof_len((&self.sms_sender_number).len()) }
        + if self.sms_send_statues == String::default() { 0 } else { 1 + sizeof_len((&self.sms_send_statues).len()) }
        + if self.sms_http_body == String::default() { 0 } else { 1 + sizeof_len((&self.sms_http_body).len()) }
        + if self.err == String::default() { 0 } else { 1 + sizeof_len((&self.err).len()) }
        + if self.carrier == String::default() { 0 } else { 1 + sizeof_len((&self.carrier).len()) }
        + if self.country.is_empty() { 0 } else { 1 + sizeof_len((&self.country).len()) }
        + if self.is_valid_phone == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.is_valid_phone) as u64) }
        + if self.is_confirmed == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.is_confirmed) as u64) }
        + if self.is_login == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.is_login) as u64) }
        + if self.is_register == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.is_register) as u64) }
        + if self.retried_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.retried_count) as u64) }
        + if self.ttl == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.ttl) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.hash != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.hash))?; }
        if self.app_uuid != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.app_uuid))?; }
        if self.client_phone != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.client_phone))?; }
        if self.genrated_code != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.genrated_code))?; }
        if self.sms_sender_number != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.sms_sender_number))?; }
        if self.sms_send_statues != String::default() { w.write_with_tag(58, |w| w.write_string(&**&self.sms_send_statues))?; }
        if self.sms_http_body != String::default() { w.write_with_tag(66, |w| w.write_string(&**&self.sms_http_body))?; }
        if self.err != String::default() { w.write_with_tag(74, |w| w.write_string(&**&self.err))?; }
        if self.carrier != String::default() { w.write_with_tag(82, |w| w.write_string(&**&self.carrier))?; }
        if !self.country.is_empty() { w.write_with_tag(90, |w| w.write_bytes(&**&self.country))?; }
        if self.is_valid_phone != 0u32 { w.write_with_tag(96, |w| w.write_uint32(*&self.is_valid_phone))?; }
        if self.is_confirmed != 0u32 { w.write_with_tag(104, |w| w.write_uint32(*&self.is_confirmed))?; }
        if self.is_login != 0u32 { w.write_with_tag(112, |w| w.write_uint32(*&self.is_login))?; }
        if self.is_register != 0u32 { w.write_with_tag(120, |w| w.write_uint32(*&self.is_register))?; }
        if self.retried_count != 0u32 { w.write_with_tag(128, |w| w.write_uint32(*&self.retried_count))?; }
        if self.ttl != 0u32 { w.write_with_tag(136, |w| w.write_uint32(*&self.ttl))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Tag {
    pub gid: u64,
    pub name: String,
    pub count: u32,
    pub is_blocked: bool,
    pub group_cid: u32,
    pub created_time: u32,
}

impl<'a> MessageRead<'a> for Tag {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.count = r.read_uint32(bytes)?,
                Ok(40) => msg.is_blocked = r.read_bool(bytes)?,
                Ok(48) => msg.group_cid = r.read_uint32(bytes)?,
                Ok(56) => msg.created_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Tag {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.count) as u64) }
        + if self.is_blocked == false { 0 } else { 1 + sizeof_varint(*(&self.is_blocked) as u64) }
        + if self.group_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_cid) as u64) }
        + if self.created_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.count != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.count))?; }
        if self.is_blocked != false { w.write_with_tag(40, |w| w.write_bool(*&self.is_blocked))?; }
        if self.group_cid != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.group_cid))?; }
        if self.created_time != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct User {
    pub cid: u32,
    pub phone: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub created_time: u32,
    pub version_time: u32,
    pub is_deleted: u32,
    pub is_banned: u32,
    pub primary_channel_changed_time: u32,
    pub UserCounts: Option<Channel>,
    pub primary_channel: Option<Channel>,
    pub channels: Vec<Channel>,
    pub sessions: Vec<Session>,
}

impl<'a> MessageRead<'a> for User {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.cid = r.read_uint32(bytes)?,
                Ok(114) => msg.phone = r.read_string(bytes)?.to_owned(),
                Ok(122) => msg.email = r.read_string(bytes)?.to_owned(),
                Ok(138) => msg.password_hash = r.read_string(bytes)?.to_owned(),
                Ok(146) => msg.password_salt = r.read_string(bytes)?.to_owned(),
                Ok(288) => msg.created_time = r.read_uint32(bytes)?,
                Ok(296) => msg.version_time = r.read_uint32(bytes)?,
                Ok(304) => msg.is_deleted = r.read_uint32(bytes)?,
                Ok(312) => msg.is_banned = r.read_uint32(bytes)?,
                Ok(400) => msg.primary_channel_changed_time = r.read_uint32(bytes)?,
                Ok(642) => msg.UserCounts = Some(r.read_message::<Channel>(bytes)?),
                Ok(802) => msg.primary_channel = Some(r.read_message::<Channel>(bytes)?),
                Ok(810) => msg.channels.push(r.read_message::<Channel>(bytes)?),
                Ok(818) => msg.sessions.push(r.read_message::<Session>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for User {
    fn get_size(&self) -> usize {
        0
        + if self.cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.cid) as u64) }
        + if self.phone == String::default() { 0 } else { 1 + sizeof_len((&self.phone).len()) }
        + if self.email == String::default() { 0 } else { 1 + sizeof_len((&self.email).len()) }
        + if self.password_hash == String::default() { 0 } else { 2 + sizeof_len((&self.password_hash).len()) }
        + if self.password_salt == String::default() { 0 } else { 2 + sizeof_len((&self.password_salt).len()) }
        + if self.created_time == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.created_time) as u64) }
        + if self.version_time == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.version_time) as u64) }
        + if self.is_deleted == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.is_deleted) as u64) }
        + if self.is_banned == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.is_banned) as u64) }
        + if self.primary_channel_changed_time == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.primary_channel_changed_time) as u64) }
        + self.UserCounts.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.primary_channel.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.channels.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.sessions.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.cid != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.cid))?; }
        if self.phone != String::default() { w.write_with_tag(114, |w| w.write_string(&**&self.phone))?; }
        if self.email != String::default() { w.write_with_tag(122, |w| w.write_string(&**&self.email))?; }
        if self.password_hash != String::default() { w.write_with_tag(138, |w| w.write_string(&**&self.password_hash))?; }
        if self.password_salt != String::default() { w.write_with_tag(146, |w| w.write_string(&**&self.password_salt))?; }
        if self.created_time != 0u32 { w.write_with_tag(288, |w| w.write_uint32(*&self.created_time))?; }
        if self.version_time != 0u32 { w.write_with_tag(296, |w| w.write_uint32(*&self.version_time))?; }
        if self.is_deleted != 0u32 { w.write_with_tag(304, |w| w.write_uint32(*&self.is_deleted))?; }
        if self.is_banned != 0u32 { w.write_with_tag(312, |w| w.write_uint32(*&self.is_banned))?; }
        if self.primary_channel_changed_time != 0u32 { w.write_with_tag(400, |w| w.write_uint32(*&self.primary_channel_changed_time))?; }
        if let Some(ref s) = self.UserCounts { w.write_with_tag(642, |w| w.write_message(s))?; }
        if let Some(ref s) = self.primary_channel { w.write_with_tag(802, |w| w.write_message(s))?; }
        for s in &self.channels { w.write_with_tag(810, |w| w.write_message(s))?; }
        for s in &self.sessions { w.write_with_tag(818, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UserCounts {
    pub created_channels: u32,
}

impl<'a> MessageRead<'a> for UserCounts {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.created_channels = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UserCounts {
    fn get_size(&self) -> usize {
        0
        + if self.created_channels == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.created_channels) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.created_channels != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.created_channels))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UserRelation {
    pub rel_nano_id: i64,
    pub user_cid: u32,
    pub peer_user_id: u32,
    pub follwing: u32,
    pub followed: u32,
    pub in_contacts: u32,
    pub mutual_contact: u32,
    pub is_favorite: u32,
    pub notify: u32,
}

impl<'a> MessageRead<'a> for UserRelation {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.rel_nano_id = r.read_int64(bytes)?,
                Ok(16) => msg.user_cid = r.read_uint32(bytes)?,
                Ok(24) => msg.peer_user_id = r.read_uint32(bytes)?,
                Ok(32) => msg.follwing = r.read_uint32(bytes)?,
                Ok(40) => msg.followed = r.read_uint32(bytes)?,
                Ok(48) => msg.in_contacts = r.read_uint32(bytes)?,
                Ok(56) => msg.mutual_contact = r.read_uint32(bytes)?,
                Ok(64) => msg.is_favorite = r.read_uint32(bytes)?,
                Ok(72) => msg.notify = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UserRelation {
    fn get_size(&self) -> usize {
        0
        + if self.rel_nano_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.rel_nano_id) as u64) }
        + if self.user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.user_cid) as u64) }
        + if self.peer_user_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.peer_user_id) as u64) }
        + if self.follwing == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.follwing) as u64) }
        + if self.followed == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.followed) as u64) }
        + if self.in_contacts == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.in_contacts) as u64) }
        + if self.mutual_contact == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.mutual_contact) as u64) }
        + if self.is_favorite == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.is_favorite) as u64) }
        + if self.notify == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.notify) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.rel_nano_id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.rel_nano_id))?; }
        if self.user_cid != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.user_cid))?; }
        if self.peer_user_id != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.peer_user_id))?; }
        if self.follwing != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.follwing))?; }
        if self.followed != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.followed))?; }
        if self.in_contacts != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.in_contacts))?; }
        if self.mutual_contact != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.mutual_contact))?; }
        if self.is_favorite != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.is_favorite))?; }
        if self.notify != 0u32 { w.write_with_tag(72, |w| w.write_uint32(*&self.notify))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Chat {
    pub gid: u64,
    pub user_cid: u32,
    pub channel_cid: u32,
    pub peer_user_cid: u32,
    pub peer_channel_cid: u32,
    pub group_cid: u32,
    pub custom_title: String,
    pub pin_time_ms: u64,
    pub from_msg_gid: i64,
    pub unseen_count: u32,
    pub seq: u32,
    pub my_last_seen_seq: u32,
    pub my_last_seen_msg_id: i64,
    pub peer_last_seen_msg_id: i64,
    pub my_last_delivered_seq: u32,
    pub my_last_delivered_msg_id: i64,
    pub peer_last_delivered_msg_id: i64,
    pub is_active: bool,
    pub mute_until: u32,
    pub sort_time_ms: i64,
    pub created_time: u32,
    pub sort_time: u64,
    pub channel: Option<Channel>,
    pub contact: Option<Contact>,
    pub group: Option<Group>,
    pub last_message: Option<Message>,
    pub pinned_message: Option<Message>,
    pub avatar: Option<FileMsg>,
    pub group_member: Option<GroupMember>,
    pub draft: Option<MessageDraft>,
    pub custom_notification: Option<ChatCustomNotification>,
}

impl<'a> MessageRead<'a> for Chat {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(40) => msg.user_cid = r.read_uint32(bytes)?,
                Ok(320) => msg.channel_cid = r.read_uint32(bytes)?,
                Ok(48) => msg.peer_user_cid = r.read_uint32(bytes)?,
                Ok(328) => msg.peer_channel_cid = r.read_uint32(bytes)?,
                Ok(56) => msg.group_cid = r.read_uint32(bytes)?,
                Ok(74) => msg.custom_title = r.read_string(bytes)?.to_owned(),
                Ok(81) => msg.pin_time_ms = r.read_fixed64(bytes)?,
                Ok(88) => msg.from_msg_gid = r.read_int64(bytes)?,
                Ok(96) => msg.unseen_count = r.read_uint32(bytes)?,
                Ok(104) => msg.seq = r.read_uint32(bytes)?,
                Ok(128) => msg.my_last_seen_seq = r.read_uint32(bytes)?,
                Ok(136) => msg.my_last_seen_msg_id = r.read_int64(bytes)?,
                Ok(144) => msg.peer_last_seen_msg_id = r.read_int64(bytes)?,
                Ok(152) => msg.my_last_delivered_seq = r.read_uint32(bytes)?,
                Ok(160) => msg.my_last_delivered_msg_id = r.read_int64(bytes)?,
                Ok(168) => msg.peer_last_delivered_msg_id = r.read_int64(bytes)?,
                Ok(176) => msg.is_active = r.read_bool(bytes)?,
                Ok(232) => msg.mute_until = r.read_uint32(bytes)?,
                Ok(240) => msg.sort_time_ms = r.read_int64(bytes)?,
                Ok(264) => msg.created_time = r.read_uint32(bytes)?,
                Ok(361) => msg.sort_time = r.read_fixed64(bytes)?,
                Ok(386) => msg.channel = Some(r.read_message::<Channel>(bytes)?),
                Ok(394) => msg.contact = Some(r.read_message::<Contact>(bytes)?),
                Ok(402) => msg.group = Some(r.read_message::<Group>(bytes)?),
                Ok(202) => msg.last_message = Some(r.read_message::<Message>(bytes)?),
                Ok(210) => msg.pinned_message = Some(r.read_message::<Message>(bytes)?),
                Ok(354) => msg.avatar = Some(r.read_message::<FileMsg>(bytes)?),
                Ok(346) => msg.group_member = Some(r.read_message::<GroupMember>(bytes)?),
                Ok(370) => msg.draft = Some(r.read_message::<MessageDraft>(bytes)?),
                Ok(378) => msg.custom_notification = Some(r.read_message::<ChatCustomNotification>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Chat {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.user_cid) as u64) }
        + if self.channel_cid == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.channel_cid) as u64) }
        + if self.peer_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.peer_user_cid) as u64) }
        + if self.peer_channel_cid == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.peer_channel_cid) as u64) }
        + if self.group_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_cid) as u64) }
        + if self.custom_title == String::default() { 0 } else { 1 + sizeof_len((&self.custom_title).len()) }
        + if self.pin_time_ms == 0u64 { 0 } else { 1 + 8 }
        + if self.from_msg_gid == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.from_msg_gid) as u64) }
        + if self.unseen_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.unseen_count) as u64) }
        + if self.seq == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.my_last_seen_seq == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.my_last_seen_seq) as u64) }
        + if self.my_last_seen_msg_id == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.my_last_seen_msg_id) as u64) }
        + if self.peer_last_seen_msg_id == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.peer_last_seen_msg_id) as u64) }
        + if self.my_last_delivered_seq == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.my_last_delivered_seq) as u64) }
        + if self.my_last_delivered_msg_id == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.my_last_delivered_msg_id) as u64) }
        + if self.peer_last_delivered_msg_id == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.peer_last_delivered_msg_id) as u64) }
        + if self.is_active == false { 0 } else { 2 + sizeof_varint(*(&self.is_active) as u64) }
        + if self.mute_until == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.mute_until) as u64) }
        + if self.sort_time_ms == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.sort_time_ms) as u64) }
        + if self.created_time == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.created_time) as u64) }
        + if self.sort_time == 0u64 { 0 } else { 2 + 8 }
        + self.channel.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.contact.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.group.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.last_message.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.pinned_message.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.avatar.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.group_member.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.draft.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.custom_notification.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.user_cid != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.user_cid))?; }
        if self.channel_cid != 0u32 { w.write_with_tag(320, |w| w.write_uint32(*&self.channel_cid))?; }
        if self.peer_user_cid != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.peer_user_cid))?; }
        if self.peer_channel_cid != 0u32 { w.write_with_tag(328, |w| w.write_uint32(*&self.peer_channel_cid))?; }
        if self.group_cid != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.group_cid))?; }
        if self.custom_title != String::default() { w.write_with_tag(74, |w| w.write_string(&**&self.custom_title))?; }
        if self.pin_time_ms != 0u64 { w.write_with_tag(81, |w| w.write_fixed64(*&self.pin_time_ms))?; }
        if self.from_msg_gid != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.from_msg_gid))?; }
        if self.unseen_count != 0u32 { w.write_with_tag(96, |w| w.write_uint32(*&self.unseen_count))?; }
        if self.seq != 0u32 { w.write_with_tag(104, |w| w.write_uint32(*&self.seq))?; }
        if self.my_last_seen_seq != 0u32 { w.write_with_tag(128, |w| w.write_uint32(*&self.my_last_seen_seq))?; }
        if self.my_last_seen_msg_id != 0i64 { w.write_with_tag(136, |w| w.write_int64(*&self.my_last_seen_msg_id))?; }
        if self.peer_last_seen_msg_id != 0i64 { w.write_with_tag(144, |w| w.write_int64(*&self.peer_last_seen_msg_id))?; }
        if self.my_last_delivered_seq != 0u32 { w.write_with_tag(152, |w| w.write_uint32(*&self.my_last_delivered_seq))?; }
        if self.my_last_delivered_msg_id != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.my_last_delivered_msg_id))?; }
        if self.peer_last_delivered_msg_id != 0i64 { w.write_with_tag(168, |w| w.write_int64(*&self.peer_last_delivered_msg_id))?; }
        if self.is_active != false { w.write_with_tag(176, |w| w.write_bool(*&self.is_active))?; }
        if self.mute_until != 0u32 { w.write_with_tag(232, |w| w.write_uint32(*&self.mute_until))?; }
        if self.sort_time_ms != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.sort_time_ms))?; }
        if self.created_time != 0u32 { w.write_with_tag(264, |w| w.write_uint32(*&self.created_time))?; }
        if self.sort_time != 0u64 { w.write_with_tag(361, |w| w.write_fixed64(*&self.sort_time))?; }
        if let Some(ref s) = self.channel { w.write_with_tag(386, |w| w.write_message(s))?; }
        if let Some(ref s) = self.contact { w.write_with_tag(394, |w| w.write_message(s))?; }
        if let Some(ref s) = self.group { w.write_with_tag(402, |w| w.write_message(s))?; }
        if let Some(ref s) = self.last_message { w.write_with_tag(202, |w| w.write_message(s))?; }
        if let Some(ref s) = self.pinned_message { w.write_with_tag(210, |w| w.write_message(s))?; }
        if let Some(ref s) = self.avatar { w.write_with_tag(354, |w| w.write_message(s))?; }
        if let Some(ref s) = self.group_member { w.write_with_tag(346, |w| w.write_message(s))?; }
        if let Some(ref s) = self.draft { w.write_with_tag(370, |w| w.write_message(s))?; }
        if let Some(ref s) = self.custom_notification { w.write_with_tag(378, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageDraft {
    pub draft_text: String,
    pub drat_reply_to_msg_id: i64,
}

impl<'a> MessageRead<'a> for MessageDraft {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(274) => msg.draft_text = r.read_string(bytes)?.to_owned(),
                Ok(280) => msg.drat_reply_to_msg_id = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MessageDraft {
    fn get_size(&self) -> usize {
        0
        + if self.draft_text == String::default() { 0 } else { 2 + sizeof_len((&self.draft_text).len()) }
        + if self.drat_reply_to_msg_id == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.drat_reply_to_msg_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.draft_text != String::default() { w.write_with_tag(274, |w| w.write_string(&**&self.draft_text))?; }
        if self.drat_reply_to_msg_id != 0i64 { w.write_with_tag(280, |w| w.write_int64(*&self.drat_reply_to_msg_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatCustomNotification {
    pub alert: bool,
    pub preview: bool,
    pub led_on: bool,
    pub led_color: bool,
    pub vibrate: bool,
    pub popup: bool,
    pub sound: bool,
    pub priority: bool,
}

impl<'a> MessageRead<'a> for ChatCustomNotification {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(104) => msg.alert = r.read_bool(bytes)?,
                Ok(112) => msg.preview = r.read_bool(bytes)?,
                Ok(120) => msg.led_on = r.read_bool(bytes)?,
                Ok(128) => msg.led_color = r.read_bool(bytes)?,
                Ok(136) => msg.vibrate = r.read_bool(bytes)?,
                Ok(144) => msg.popup = r.read_bool(bytes)?,
                Ok(152) => msg.sound = r.read_bool(bytes)?,
                Ok(160) => msg.priority = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChatCustomNotification {
    fn get_size(&self) -> usize {
        0
        + if self.alert == false { 0 } else { 1 + sizeof_varint(*(&self.alert) as u64) }
        + if self.preview == false { 0 } else { 1 + sizeof_varint(*(&self.preview) as u64) }
        + if self.led_on == false { 0 } else { 1 + sizeof_varint(*(&self.led_on) as u64) }
        + if self.led_color == false { 0 } else { 2 + sizeof_varint(*(&self.led_color) as u64) }
        + if self.vibrate == false { 0 } else { 2 + sizeof_varint(*(&self.vibrate) as u64) }
        + if self.popup == false { 0 } else { 2 + sizeof_varint(*(&self.popup) as u64) }
        + if self.sound == false { 0 } else { 2 + sizeof_varint(*(&self.sound) as u64) }
        + if self.priority == false { 0 } else { 2 + sizeof_varint(*(&self.priority) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.alert != false { w.write_with_tag(104, |w| w.write_bool(*&self.alert))?; }
        if self.preview != false { w.write_with_tag(112, |w| w.write_bool(*&self.preview))?; }
        if self.led_on != false { w.write_with_tag(120, |w| w.write_bool(*&self.led_on))?; }
        if self.led_color != false { w.write_with_tag(128, |w| w.write_bool(*&self.led_color))?; }
        if self.vibrate != false { w.write_with_tag(136, |w| w.write_bool(*&self.vibrate))?; }
        if self.popup != false { w.write_with_tag(144, |w| w.write_bool(*&self.popup))?; }
        if self.sound != false { w.write_with_tag(152, |w| w.write_bool(*&self.sound))?; }
        if self.priority != false { w.write_with_tag(160, |w| w.write_bool(*&self.priority))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Group {
    pub cid: u32,
    pub group_title: String,
    pub user_name: String,
    pub creator_user_cid: u32,
    pub creator_channel_cid: u32,
    pub group_privacy: GroupPrivacy,
    pub history_viewable: bool,
    pub seq: u32,
    pub avatar_count: u32,
    pub about: String,
    pub invite_link_hash: String,
    pub members_count: u32,
    pub admins_count: u32,
    pub moderator_counts: u32,
    pub sort_time: u64,
    pub sync_time: u64,
    pub created_time: u32,
    pub is_deleted: bool,
    pub is_banned: bool,
    pub last_message: Option<Message>,
    pub pinned_message: Option<Message>,
    pub avatar: Option<FileMsg>,
}

impl<'a> MessageRead<'a> for Group {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.cid = r.read_uint32(bytes)?,
                Ok(26) => msg.group_title = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.user_name = r.read_string(bytes)?.to_owned(),
                Ok(56) => msg.creator_user_cid = r.read_uint32(bytes)?,
                Ok(248) => msg.creator_channel_cid = r.read_uint32(bytes)?,
                Ok(64) => msg.group_privacy = r.read_enum(bytes)?,
                Ok(72) => msg.history_viewable = r.read_bool(bytes)?,
                Ok(80) => msg.seq = r.read_uint32(bytes)?,
                Ok(112) => msg.avatar_count = r.read_uint32(bytes)?,
                Ok(122) => msg.about = r.read_string(bytes)?.to_owned(),
                Ok(130) => msg.invite_link_hash = r.read_string(bytes)?.to_owned(),
                Ok(136) => msg.members_count = r.read_uint32(bytes)?,
                Ok(144) => msg.admins_count = r.read_uint32(bytes)?,
                Ok(152) => msg.moderator_counts = r.read_uint32(bytes)?,
                Ok(161) => msg.sort_time = r.read_fixed64(bytes)?,
                Ok(321) => msg.sync_time = r.read_fixed64(bytes)?,
                Ok(168) => msg.created_time = r.read_uint32(bytes)?,
                Ok(184) => msg.is_deleted = r.read_bool(bytes)?,
                Ok(192) => msg.is_banned = r.read_bool(bytes)?,
                Ok(202) => msg.last_message = Some(r.read_message::<Message>(bytes)?),
                Ok(210) => msg.pinned_message = Some(r.read_message::<Message>(bytes)?),
                Ok(218) => msg.avatar = Some(r.read_message::<FileMsg>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Group {
    fn get_size(&self) -> usize {
        0
        + if self.cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.cid) as u64) }
        + if self.group_title == String::default() { 0 } else { 1 + sizeof_len((&self.group_title).len()) }
        + if self.user_name == String::default() { 0 } else { 1 + sizeof_len((&self.user_name).len()) }
        + if self.creator_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.creator_user_cid) as u64) }
        + if self.creator_channel_cid == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.creator_channel_cid) as u64) }
        + if self.group_privacy == store::GroupPrivacy::GROUP_UNKNOWN_GP { 0 } else { 1 + sizeof_varint(*(&self.group_privacy) as u64) }
        + if self.history_viewable == false { 0 } else { 1 + sizeof_varint(*(&self.history_viewable) as u64) }
        + if self.seq == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.avatar_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.avatar_count) as u64) }
        + if self.about == String::default() { 0 } else { 1 + sizeof_len((&self.about).len()) }
        + if self.invite_link_hash == String::default() { 0 } else { 2 + sizeof_len((&self.invite_link_hash).len()) }
        + if self.members_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.members_count) as u64) }
        + if self.admins_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.admins_count) as u64) }
        + if self.moderator_counts == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.moderator_counts) as u64) }
        + if self.sort_time == 0u64 { 0 } else { 2 + 8 }
        + if self.sync_time == 0u64 { 0 } else { 2 + 8 }
        + if self.created_time == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.created_time) as u64) }
        + if self.is_deleted == false { 0 } else { 2 + sizeof_varint(*(&self.is_deleted) as u64) }
        + if self.is_banned == false { 0 } else { 2 + sizeof_varint(*(&self.is_banned) as u64) }
        + self.last_message.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.pinned_message.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.avatar.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.cid != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.cid))?; }
        if self.group_title != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.group_title))?; }
        if self.user_name != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.user_name))?; }
        if self.creator_user_cid != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.creator_user_cid))?; }
        if self.creator_channel_cid != 0u32 { w.write_with_tag(248, |w| w.write_uint32(*&self.creator_channel_cid))?; }
        if self.group_privacy != store::GroupPrivacy::GROUP_UNKNOWN_GP { w.write_with_tag(64, |w| w.write_enum(*&self.group_privacy as i32))?; }
        if self.history_viewable != false { w.write_with_tag(72, |w| w.write_bool(*&self.history_viewable))?; }
        if self.seq != 0u32 { w.write_with_tag(80, |w| w.write_uint32(*&self.seq))?; }
        if self.avatar_count != 0u32 { w.write_with_tag(112, |w| w.write_uint32(*&self.avatar_count))?; }
        if self.about != String::default() { w.write_with_tag(122, |w| w.write_string(&**&self.about))?; }
        if self.invite_link_hash != String::default() { w.write_with_tag(130, |w| w.write_string(&**&self.invite_link_hash))?; }
        if self.members_count != 0u32 { w.write_with_tag(136, |w| w.write_uint32(*&self.members_count))?; }
        if self.admins_count != 0u32 { w.write_with_tag(144, |w| w.write_uint32(*&self.admins_count))?; }
        if self.moderator_counts != 0u32 { w.write_with_tag(152, |w| w.write_uint32(*&self.moderator_counts))?; }
        if self.sort_time != 0u64 { w.write_with_tag(161, |w| w.write_fixed64(*&self.sort_time))?; }
        if self.sync_time != 0u64 { w.write_with_tag(321, |w| w.write_fixed64(*&self.sync_time))?; }
        if self.created_time != 0u32 { w.write_with_tag(168, |w| w.write_uint32(*&self.created_time))?; }
        if self.is_deleted != false { w.write_with_tag(184, |w| w.write_bool(*&self.is_deleted))?; }
        if self.is_banned != false { w.write_with_tag(192, |w| w.write_bool(*&self.is_banned))?; }
        if let Some(ref s) = self.last_message { w.write_with_tag(202, |w| w.write_message(s))?; }
        if let Some(ref s) = self.pinned_message { w.write_with_tag(210, |w| w.write_message(s))?; }
        if let Some(ref s) = self.avatar { w.write_with_tag(218, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupMember {
    pub gid: i64,
    pub group_cid: i64,
    pub user_cid: u32,
    pub channel_cid: u32,
    pub by_user_cid: u32,
    pub by_channel_cid: u32,
    pub group_role: GroupMemberRole,
    pub created_time: u32,
}

impl<'a> MessageRead<'a> for GroupMember {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.gid = r.read_int64(bytes)?,
                Ok(16) => msg.group_cid = r.read_int64(bytes)?,
                Ok(24) => msg.user_cid = r.read_uint32(bytes)?,
                Ok(64) => msg.channel_cid = r.read_uint32(bytes)?,
                Ok(32) => msg.by_user_cid = r.read_uint32(bytes)?,
                Ok(56) => msg.by_channel_cid = r.read_uint32(bytes)?,
                Ok(40) => msg.group_role = r.read_enum(bytes)?,
                Ok(48) => msg.created_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupMember {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.gid) as u64) }
        + if self.group_cid == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.group_cid) as u64) }
        + if self.user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.user_cid) as u64) }
        + if self.channel_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel_cid) as u64) }
        + if self.by_user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.by_user_cid) as u64) }
        + if self.by_channel_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.by_channel_cid) as u64) }
        + if self.group_role == store::GroupMemberRole::MEMBER_Unknown { 0 } else { 1 + sizeof_varint(*(&self.group_role) as u64) }
        + if self.created_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.gid))?; }
        if self.group_cid != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.group_cid))?; }
        if self.user_cid != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.user_cid))?; }
        if self.channel_cid != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.channel_cid))?; }
        if self.by_user_cid != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.by_user_cid))?; }
        if self.by_channel_cid != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.by_channel_cid))?; }
        if self.group_role != store::GroupMemberRole::MEMBER_Unknown { w.write_with_tag(40, |w| w.write_enum(*&self.group_role as i32))?; }
        if self.created_time != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileMsg {
    pub gid: u64,
    pub access_hash: u32,
    pub file_type: u32,
    pub width: u32,
    pub height: u32,
    pub extension: String,
    pub user_cid: u32,
    pub data_thumb: Vec<u8>,
    pub data: Vec<u8>,
}

impl<'a> MessageRead<'a> for FileMsg {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(16) => msg.access_hash = r.read_uint32(bytes)?,
                Ok(24) => msg.file_type = r.read_uint32(bytes)?,
                Ok(32) => msg.width = r.read_uint32(bytes)?,
                Ok(40) => msg.height = r.read_uint32(bytes)?,
                Ok(50) => msg.extension = r.read_string(bytes)?.to_owned(),
                Ok(56) => msg.user_cid = r.read_uint32(bytes)?,
                Ok(66) => msg.data_thumb = r.read_bytes(bytes)?.to_owned(),
                Ok(74) => msg.data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for FileMsg {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.access_hash == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.access_hash) as u64) }
        + if self.file_type == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.file_type) as u64) }
        + if self.width == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.width) as u64) }
        + if self.height == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.height) as u64) }
        + if self.extension == String::default() { 0 } else { 1 + sizeof_len((&self.extension).len()) }
        + if self.user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.user_cid) as u64) }
        + if self.data_thumb.is_empty() { 0 } else { 1 + sizeof_len((&self.data_thumb).len()) }
        + if self.data.is_empty() { 0 } else { 1 + sizeof_len((&self.data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.access_hash != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.access_hash))?; }
        if self.file_type != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.file_type))?; }
        if self.width != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.width))?; }
        if self.height != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.height))?; }
        if self.extension != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.extension))?; }
        if self.user_cid != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.user_cid))?; }
        if !self.data_thumb.is_empty() { w.write_with_tag(66, |w| w.write_bytes(&**&self.data_thumb))?; }
        if !self.data.is_empty() { w.write_with_tag(74, |w| w.write_bytes(&**&self.data))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Post {
    pub gid: u64,
    pub user_cid: u32,
    pub post_type: u32,
    pub media_id: i64,
    pub file_ref_id: i64,
    pub post_key: String,
    pub text: String,
    pub rich_text: String,
    pub media_count: u32,
    pub shared_to: u32,
    pub disable_comment: u32,
    pub via: u32,
    pub seq: u32,
    pub comments_count: u32,
    pub likes_count: u32,
    pub views_count: u32,
    pub edited_time: u32,
    pub created_time: u32,
}

impl<'a> MessageRead<'a> for Post {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.gid = r.read_fixed64(bytes)?,
                Ok(16) => msg.user_cid = r.read_uint32(bytes)?,
                Ok(24) => msg.post_type = r.read_uint32(bytes)?,
                Ok(32) => msg.media_id = r.read_int64(bytes)?,
                Ok(40) => msg.file_ref_id = r.read_int64(bytes)?,
                Ok(50) => msg.post_key = r.read_string(bytes)?.to_owned(),
                Ok(58) => msg.text = r.read_string(bytes)?.to_owned(),
                Ok(66) => msg.rich_text = r.read_string(bytes)?.to_owned(),
                Ok(72) => msg.media_count = r.read_uint32(bytes)?,
                Ok(80) => msg.shared_to = r.read_uint32(bytes)?,
                Ok(88) => msg.disable_comment = r.read_uint32(bytes)?,
                Ok(96) => msg.via = r.read_uint32(bytes)?,
                Ok(104) => msg.seq = r.read_uint32(bytes)?,
                Ok(112) => msg.comments_count = r.read_uint32(bytes)?,
                Ok(120) => msg.likes_count = r.read_uint32(bytes)?,
                Ok(128) => msg.views_count = r.read_uint32(bytes)?,
                Ok(136) => msg.edited_time = r.read_uint32(bytes)?,
                Ok(144) => msg.created_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Post {
    fn get_size(&self) -> usize {
        0
        + if self.gid == 0u64 { 0 } else { 1 + 8 }
        + if self.user_cid == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.user_cid) as u64) }
        + if self.post_type == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.post_type) as u64) }
        + if self.media_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.media_id) as u64) }
        + if self.file_ref_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.file_ref_id) as u64) }
        + if self.post_key == String::default() { 0 } else { 1 + sizeof_len((&self.post_key).len()) }
        + if self.text == String::default() { 0 } else { 1 + sizeof_len((&self.text).len()) }
        + if self.rich_text == String::default() { 0 } else { 1 + sizeof_len((&self.rich_text).len()) }
        + if self.media_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.media_count) as u64) }
        + if self.shared_to == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.shared_to) as u64) }
        + if self.disable_comment == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.disable_comment) as u64) }
        + if self.via == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.via) as u64) }
        + if self.seq == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.comments_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.comments_count) as u64) }
        + if self.likes_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.likes_count) as u64) }
        + if self.views_count == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.views_count) as u64) }
        + if self.edited_time == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.edited_time) as u64) }
        + if self.created_time == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.gid))?; }
        if self.user_cid != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.user_cid))?; }
        if self.post_type != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.post_type))?; }
        if self.media_id != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.media_id))?; }
        if self.file_ref_id != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.file_ref_id))?; }
        if self.post_key != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.post_key))?; }
        if self.text != String::default() { w.write_with_tag(58, |w| w.write_string(&**&self.text))?; }
        if self.rich_text != String::default() { w.write_with_tag(66, |w| w.write_string(&**&self.rich_text))?; }
        if self.media_count != 0u32 { w.write_with_tag(72, |w| w.write_uint32(*&self.media_count))?; }
        if self.shared_to != 0u32 { w.write_with_tag(80, |w| w.write_uint32(*&self.shared_to))?; }
        if self.disable_comment != 0u32 { w.write_with_tag(88, |w| w.write_uint32(*&self.disable_comment))?; }
        if self.via != 0u32 { w.write_with_tag(96, |w| w.write_uint32(*&self.via))?; }
        if self.seq != 0u32 { w.write_with_tag(104, |w| w.write_uint32(*&self.seq))?; }
        if self.comments_count != 0u32 { w.write_with_tag(112, |w| w.write_uint32(*&self.comments_count))?; }
        if self.likes_count != 0u32 { w.write_with_tag(120, |w| w.write_uint32(*&self.likes_count))?; }
        if self.views_count != 0u32 { w.write_with_tag(128, |w| w.write_uint32(*&self.views_count))?; }
        if self.edited_time != 0u32 { w.write_with_tag(136, |w| w.write_uint32(*&self.edited_time))?; }
        if self.created_time != 0u32 { w.write_with_tag(144, |w| w.write_uint32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PostCount {
    pub post_gid: u64,
    pub comments_count: u32,
    pub likes_count: u32,
    pub views_count: i64,
    pub re_shared_count: u32,
    pub chat_shared_count: u32,
}

impl<'a> MessageRead<'a> for PostCount {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.post_gid = r.read_fixed64(bytes)?,
                Ok(16) => msg.comments_count = r.read_uint32(bytes)?,
                Ok(24) => msg.likes_count = r.read_uint32(bytes)?,
                Ok(32) => msg.views_count = r.read_int64(bytes)?,
                Ok(40) => msg.re_shared_count = r.read_uint32(bytes)?,
                Ok(48) => msg.chat_shared_count = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PostCount {
    fn get_size(&self) -> usize {
        0
        + if self.post_gid == 0u64 { 0 } else { 1 + 8 }
        + if self.comments_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.comments_count) as u64) }
        + if self.likes_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.likes_count) as u64) }
        + if self.views_count == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.views_count) as u64) }
        + if self.re_shared_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.re_shared_count) as u64) }
        + if self.chat_shared_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.chat_shared_count) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.post_gid != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.post_gid))?; }
        if self.comments_count != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.comments_count))?; }
        if self.likes_count != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.likes_count))?; }
        if self.views_count != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.views_count))?; }
        if self.re_shared_count != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.re_shared_count))?; }
        if self.chat_shared_count != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.chat_shared_count))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageDelivery {
    pub statues: MessageDeliveryStatues,
    pub seen_time: u32,
}

impl<'a> MessageRead<'a> for MessageDelivery {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(88) => msg.statues = r.read_enum(bytes)?,
                Ok(16) => msg.seen_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MessageDelivery {
    fn get_size(&self) -> usize {
        0
        + if self.statues == store::MessageDeliveryStatues::FAILED { 0 } else { 1 + sizeof_varint(*(&self.statues) as u64) }
        + if self.seen_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.seen_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.statues != store::MessageDeliveryStatues::FAILED { w.write_with_tag(88, |w| w.write_enum(*&self.statues as i32))?; }
        if self.seen_time != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.seen_time))?; }
        Ok(())
    }
}

