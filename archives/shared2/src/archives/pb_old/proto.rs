// Automatically generated rust module for 'igap.proto' file

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
pub enum Gender {
    UNKNOWN = 0,
    MALE = 1,
    FEMALE = 2,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::UNKNOWN
    }
}

impl From<i32> for Gender {
    fn from(i: i32) -> Self {
        match i {
            0 => Gender::UNKNOWN,
            1 => Gender::MALE,
            2 => Gender::FEMALE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Gender {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => Gender::UNKNOWN,
            "MALE" => Gender::MALE,
            "FEMALE" => Gender::FEMALE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Device {
    UNKNOWN_DEVICE = 0,
    PC = 1,
    TABLET = 2,
    MOBILE = 3,
}

impl Default for Device {
    fn default() -> Self {
        Device::UNKNOWN_DEVICE
    }
}

impl From<i32> for Device {
    fn from(i: i32) -> Self {
        match i {
            0 => Device::UNKNOWN_DEVICE,
            1 => Device::PC,
            2 => Device::TABLET,
            3 => Device::MOBILE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Device {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN_DEVICE" => Device::UNKNOWN_DEVICE,
            "PC" => Device::PC,
            "TABLET" => Device::TABLET,
            "MOBILE" => Device::MOBILE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Platform {
    UNKNOWN_PLATFORM = 0,
    ANDROID = 1,
    IOS = 2,
    MAC_OS = 3,
    WINDOWS = 4,
    LINUX = 5,
    BLACK_BERRY = 6,
}

impl Default for Platform {
    fn default() -> Self {
        Platform::UNKNOWN_PLATFORM
    }
}

impl From<i32> for Platform {
    fn from(i: i32) -> Self {
        match i {
            0 => Platform::UNKNOWN_PLATFORM,
            1 => Platform::ANDROID,
            2 => Platform::IOS,
            3 => Platform::MAC_OS,
            4 => Platform::WINDOWS,
            5 => Platform::LINUX,
            6 => Platform::BLACK_BERRY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Platform {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN_PLATFORM" => Platform::UNKNOWN_PLATFORM,
            "ANDROID" => Platform::ANDROID,
            "IOS" => Platform::IOS,
            "MAC_OS" => Platform::MAC_OS,
            "WINDOWS" => Platform::WINDOWS,
            "LINUX" => Platform::LINUX,
            "BLACK_BERRY" => Platform::BLACK_BERRY,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Language {
    EN_US = 0,
    FA_IR = 1,
}

impl Default for Language {
    fn default() -> Self {
        Language::EN_US
    }
}

impl From<i32> for Language {
    fn from(i: i32) -> Self {
        match i {
            0 => Language::EN_US,
            1 => Language::FA_IR,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Language {
    fn from(s: &'a str) -> Self {
        match s {
            "EN_US" => Language::EN_US,
            "FA_IR" => Language::FA_IR,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RoomMessageType {
    TEXT = 0,
    IMAGE = 1,
    IMAGE_TEXT = 2,
    VIDEO = 3,
    VIDEO_TEXT = 4,
    AUDIO = 5,
    AUDIO_TEXT = 6,
    VOICE = 7,
    GIF = 8,
    GIF_TEXT = 14,
    FILE = 9,
    FILE_TEXT = 10,
    LOCATION = 11,
    LOG = 12,
    CONTACT = 13,
    WALLET = 15,
}

impl Default for RoomMessageType {
    fn default() -> Self {
        RoomMessageType::TEXT
    }
}

impl From<i32> for RoomMessageType {
    fn from(i: i32) -> Self {
        match i {
            0 => RoomMessageType::TEXT,
            1 => RoomMessageType::IMAGE,
            2 => RoomMessageType::IMAGE_TEXT,
            3 => RoomMessageType::VIDEO,
            4 => RoomMessageType::VIDEO_TEXT,
            5 => RoomMessageType::AUDIO,
            6 => RoomMessageType::AUDIO_TEXT,
            7 => RoomMessageType::VOICE,
            8 => RoomMessageType::GIF,
            14 => RoomMessageType::GIF_TEXT,
            9 => RoomMessageType::FILE,
            10 => RoomMessageType::FILE_TEXT,
            11 => RoomMessageType::LOCATION,
            12 => RoomMessageType::LOG,
            13 => RoomMessageType::CONTACT,
            15 => RoomMessageType::WALLET,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for RoomMessageType {
    fn from(s: &'a str) -> Self {
        match s {
            "TEXT" => RoomMessageType::TEXT,
            "IMAGE" => RoomMessageType::IMAGE,
            "IMAGE_TEXT" => RoomMessageType::IMAGE_TEXT,
            "VIDEO" => RoomMessageType::VIDEO,
            "VIDEO_TEXT" => RoomMessageType::VIDEO_TEXT,
            "AUDIO" => RoomMessageType::AUDIO,
            "AUDIO_TEXT" => RoomMessageType::AUDIO_TEXT,
            "VOICE" => RoomMessageType::VOICE,
            "GIF" => RoomMessageType::GIF,
            "GIF_TEXT" => RoomMessageType::GIF_TEXT,
            "FILE" => RoomMessageType::FILE,
            "FILE_TEXT" => RoomMessageType::FILE_TEXT,
            "LOCATION" => RoomMessageType::LOCATION,
            "LOG" => RoomMessageType::LOG,
            "CONTACT" => RoomMessageType::CONTACT,
            "WALLET" => RoomMessageType::WALLET,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RoomMessageStatus {
    FAILED = 0,
    SENDING = 1,
    SENT = 2,
    DELIVERED = 3,
    SEEN = 4,
    LISTENED = 5,
}

impl Default for RoomMessageStatus {
    fn default() -> Self {
        RoomMessageStatus::FAILED
    }
}

impl From<i32> for RoomMessageStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => RoomMessageStatus::FAILED,
            1 => RoomMessageStatus::SENDING,
            2 => RoomMessageStatus::SENT,
            3 => RoomMessageStatus::DELIVERED,
            4 => RoomMessageStatus::SEEN,
            5 => RoomMessageStatus::LISTENED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for RoomMessageStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "FAILED" => RoomMessageStatus::FAILED,
            "SENDING" => RoomMessageStatus::SENDING,
            "SENT" => RoomMessageStatus::SENT,
            "DELIVERED" => RoomMessageStatus::DELIVERED,
            "SEEN" => RoomMessageStatus::SEEN,
            "LISTENED" => RoomMessageStatus::LISTENED,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RoomMessageReaction {
    THUMBS_UP = 0,
    THUMBS_DOWN = 1,
}

impl Default for RoomMessageReaction {
    fn default() -> Self {
        RoomMessageReaction::THUMBS_UP
    }
}

impl From<i32> for RoomMessageReaction {
    fn from(i: i32) -> Self {
        match i {
            0 => RoomMessageReaction::THUMBS_UP,
            1 => RoomMessageReaction::THUMBS_DOWN,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for RoomMessageReaction {
    fn from(s: &'a str) -> Self {
        match s {
            "THUMBS_UP" => RoomMessageReaction::THUMBS_UP,
            "THUMBS_DOWN" => RoomMessageReaction::THUMBS_DOWN,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClientAction {
    CANCEL = 0,
    TYPING = 1,
    SENDING_IMAGE = 2,
    CAPTURING_IMAGE = 3,
    SENDING_VIDEO = 4,
    CAPTURING_VIDEO = 5,
    SENDING_AUDIO = 6,
    RECORDING_VOICE = 7,
    SENDING_VOICE = 8,
    SENDING_DOCUMENT = 9,
    SENDING_GIF = 10,
    SENDING_FILE = 11,
    SENDING_LOCATION = 12,
    CHOOSING_CONTACT = 13,
    PAINTING = 14,
}

impl Default for ClientAction {
    fn default() -> Self {
        ClientAction::CANCEL
    }
}

impl From<i32> for ClientAction {
    fn from(i: i32) -> Self {
        match i {
            0 => ClientAction::CANCEL,
            1 => ClientAction::TYPING,
            2 => ClientAction::SENDING_IMAGE,
            3 => ClientAction::CAPTURING_IMAGE,
            4 => ClientAction::SENDING_VIDEO,
            5 => ClientAction::CAPTURING_VIDEO,
            6 => ClientAction::SENDING_AUDIO,
            7 => ClientAction::RECORDING_VOICE,
            8 => ClientAction::SENDING_VOICE,
            9 => ClientAction::SENDING_DOCUMENT,
            10 => ClientAction::SENDING_GIF,
            11 => ClientAction::SENDING_FILE,
            12 => ClientAction::SENDING_LOCATION,
            13 => ClientAction::CHOOSING_CONTACT,
            14 => ClientAction::PAINTING,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ClientAction {
    fn from(s: &'a str) -> Self {
        match s {
            "CANCEL" => ClientAction::CANCEL,
            "TYPING" => ClientAction::TYPING,
            "SENDING_IMAGE" => ClientAction::SENDING_IMAGE,
            "CAPTURING_IMAGE" => ClientAction::CAPTURING_IMAGE,
            "SENDING_VIDEO" => ClientAction::SENDING_VIDEO,
            "CAPTURING_VIDEO" => ClientAction::CAPTURING_VIDEO,
            "SENDING_AUDIO" => ClientAction::SENDING_AUDIO,
            "RECORDING_VOICE" => ClientAction::RECORDING_VOICE,
            "SENDING_VOICE" => ClientAction::SENDING_VOICE,
            "SENDING_DOCUMENT" => ClientAction::SENDING_DOCUMENT,
            "SENDING_GIF" => ClientAction::SENDING_GIF,
            "SENDING_FILE" => ClientAction::SENDING_FILE,
            "SENDING_LOCATION" => ClientAction::SENDING_LOCATION,
            "CHOOSING_CONTACT" => ClientAction::CHOOSING_CONTACT,
            "PAINTING" => ClientAction::PAINTING,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RoomMute {
    UNMUTE = 0,
    MUTE = 1,
}

impl Default for RoomMute {
    fn default() -> Self {
        RoomMute::UNMUTE
    }
}

impl From<i32> for RoomMute {
    fn from(i: i32) -> Self {
        match i {
            0 => RoomMute::UNMUTE,
            1 => RoomMute::MUTE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for RoomMute {
    fn from(s: &'a str) -> Self {
        match s {
            "UNMUTE" => RoomMute::UNMUTE,
            "MUTE" => RoomMute::MUTE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrivacyType {
    USER_STATUS = 0,
    AVATAR = 1,
    GROUP_INVITE = 2,
    CHANNEL_INVITE = 3,
    VOICE_CALLING = 4,
    VIDEO_CALLING = 5,
    SCREEN_SHARING = 6,
    SECRET_CHAT = 7,
}

impl Default for PrivacyType {
    fn default() -> Self {
        PrivacyType::USER_STATUS
    }
}

impl From<i32> for PrivacyType {
    fn from(i: i32) -> Self {
        match i {
            0 => PrivacyType::USER_STATUS,
            1 => PrivacyType::AVATAR,
            2 => PrivacyType::GROUP_INVITE,
            3 => PrivacyType::CHANNEL_INVITE,
            4 => PrivacyType::VOICE_CALLING,
            5 => PrivacyType::VIDEO_CALLING,
            6 => PrivacyType::SCREEN_SHARING,
            7 => PrivacyType::SECRET_CHAT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PrivacyType {
    fn from(s: &'a str) -> Self {
        match s {
            "USER_STATUS" => PrivacyType::USER_STATUS,
            "AVATAR" => PrivacyType::AVATAR,
            "GROUP_INVITE" => PrivacyType::GROUP_INVITE,
            "CHANNEL_INVITE" => PrivacyType::CHANNEL_INVITE,
            "VOICE_CALLING" => PrivacyType::VOICE_CALLING,
            "VIDEO_CALLING" => PrivacyType::VIDEO_CALLING,
            "SCREEN_SHARING" => PrivacyType::SCREEN_SHARING,
            "SECRET_CHAT" => PrivacyType::SECRET_CHAT,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrivacyLevel {
    ALLOW_ALL = 0,
    DENY_ALL = 1,
    ALLOW_CONTACTS = 2,
}

impl Default for PrivacyLevel {
    fn default() -> Self {
        PrivacyLevel::ALLOW_ALL
    }
}

impl From<i32> for PrivacyLevel {
    fn from(i: i32) -> Self {
        match i {
            0 => PrivacyLevel::ALLOW_ALL,
            1 => PrivacyLevel::DENY_ALL,
            2 => PrivacyLevel::ALLOW_CONTACTS,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PrivacyLevel {
    fn from(s: &'a str) -> Self {
        match s {
            "ALLOW_ALL" => PrivacyLevel::ALLOW_ALL,
            "DENY_ALL" => PrivacyLevel::DENY_ALL,
            "ALLOW_CONTACTS" => PrivacyLevel::ALLOW_CONTACTS,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RoomMessageLocation {
    pub lat: f64,
    pub lon: f64,
}

impl<'a> MessageRead<'a> for RoomMessageLocation {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.lat = r.read_double(bytes)?,
                Ok(17) => msg.lon = r.read_double(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RoomMessageLocation {
    fn get_size(&self) -> usize {
        0
        + if self.lat == 0f64 { 0 } else { 1 + 8 }
        + if self.lon == 0f64 { 0 } else { 1 + 8 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.lat != 0f64 { w.write_with_tag(9, |w| w.write_double(*&self.lat))?; }
        if self.lon != 0f64 { w.write_with_tag(17, |w| w.write_double(*&self.lon))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RoomMessageLog {
    pub type_pb: proto::mod_RoomMessageLog::Type,
    pub extra_type: proto::mod_RoomMessageLog::ExtraType,
    pub target_user: Option<proto::mod_RoomMessageLog::TargetUser>,
}

impl<'a> MessageRead<'a> for RoomMessageLog {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(16) => msg.extra_type = r.read_enum(bytes)?,
                Ok(26) => msg.target_user = Some(r.read_message::<proto::mod_RoomMessageLog::TargetUser>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RoomMessageLog {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == proto::mod_RoomMessageLog::Type::USER_JOINED { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.extra_type == proto::mod_RoomMessageLog::ExtraType::NO_EXTRA { 0 } else { 1 + sizeof_varint(*(&self.extra_type) as u64) }
        + self.target_user.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != proto::mod_RoomMessageLog::Type::USER_JOINED { w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.extra_type != proto::mod_RoomMessageLog::ExtraType::NO_EXTRA { w.write_with_tag(16, |w| w.write_enum(*&self.extra_type as i32))?; }
        if let Some(ref s) = self.target_user { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_RoomMessageLog {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TargetUser {
    pub id: u64,
}

impl<'a> MessageRead<'a> for TargetUser {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TargetUser {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.id))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
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

impl Default for Type {
    fn default() -> Self {
        Type::USER_JOINED
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::USER_JOINED,
            1 => Type::USER_DELETED,
            2 => Type::ROOM_CREATED,
            3 => Type::MEMBER_ADDED,
            4 => Type::MEMBER_KICKED,
            5 => Type::MEMBER_LEFT,
            6 => Type::ROOM_CONVERTED_TO_PUBLIC,
            7 => Type::ROOM_CONVERTED_TO_PRIVATE,
            8 => Type::MEMBER_JOINED_BY_INVITE_LINK,
            9 => Type::ROOM_DELETED,
            10 => Type::MISSED_VOICE_CALL,
            11 => Type::MISSED_VIDEO_CALL,
            12 => Type::MISSED_SCREEN_SHARE,
            13 => Type::MISSED_SECRET_CHAT,
            14 => Type::PINNED_MESSAGE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "USER_JOINED" => Type::USER_JOINED,
            "USER_DELETED" => Type::USER_DELETED,
            "ROOM_CREATED" => Type::ROOM_CREATED,
            "MEMBER_ADDED" => Type::MEMBER_ADDED,
            "MEMBER_KICKED" => Type::MEMBER_KICKED,
            "MEMBER_LEFT" => Type::MEMBER_LEFT,
            "ROOM_CONVERTED_TO_PUBLIC" => Type::ROOM_CONVERTED_TO_PUBLIC,
            "ROOM_CONVERTED_TO_PRIVATE" => Type::ROOM_CONVERTED_TO_PRIVATE,
            "MEMBER_JOINED_BY_INVITE_LINK" => Type::MEMBER_JOINED_BY_INVITE_LINK,
            "ROOM_DELETED" => Type::ROOM_DELETED,
            "MISSED_VOICE_CALL" => Type::MISSED_VOICE_CALL,
            "MISSED_VIDEO_CALL" => Type::MISSED_VIDEO_CALL,
            "MISSED_SCREEN_SHARE" => Type::MISSED_SCREEN_SHARE,
            "MISSED_SECRET_CHAT" => Type::MISSED_SECRET_CHAT,
            "PINNED_MESSAGE" => Type::PINNED_MESSAGE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExtraType {
    NO_EXTRA = 0,
    TARGET_USER = 1,
}

impl Default for ExtraType {
    fn default() -> Self {
        ExtraType::NO_EXTRA
    }
}

impl From<i32> for ExtraType {
    fn from(i: i32) -> Self {
        match i {
            0 => ExtraType::NO_EXTRA,
            1 => ExtraType::TARGET_USER,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ExtraType {
    fn from(s: &'a str) -> Self {
        match s {
            "NO_EXTRA" => ExtraType::NO_EXTRA,
            "TARGET_USER" => ExtraType::TARGET_USER,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RoomMessageContact {
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub phone: Vec<String>,
    pub email: Vec<String>,
}

impl<'a> MessageRead<'a> for RoomMessageContact {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.first_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.last_name = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.nickname = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.phone.push(r.read_string(bytes)?.to_owned()),
                Ok(42) => msg.email.push(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RoomMessageContact {
    fn get_size(&self) -> usize {
        0
        + if self.first_name == String::default() { 0 } else { 1 + sizeof_len((&self.first_name).len()) }
        + if self.last_name == String::default() { 0 } else { 1 + sizeof_len((&self.last_name).len()) }
        + if self.nickname == String::default() { 0 } else { 1 + sizeof_len((&self.nickname).len()) }
        + self.phone.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.email.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.first_name != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.first_name))?; }
        if self.last_name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.last_name))?; }
        if self.nickname != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.nickname))?; }
        for s in &self.phone { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        for s in &self.email { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RoomMessageWallet {
    pub type_pb: proto::mod_RoomMessageWallet::Type,
    pub money_transfer: Option<proto::mod_RoomMessageWallet::MoneyTransfer>,
}

impl<'a> MessageRead<'a> for RoomMessageWallet {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(18) => msg.money_transfer = Some(r.read_message::<proto::mod_RoomMessageWallet::MoneyTransfer>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RoomMessageWallet {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == proto::mod_RoomMessageWallet::Type::MONEY_TRANSFER { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + self.money_transfer.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != proto::mod_RoomMessageWallet::Type::MONEY_TRANSFER { w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?; }
        if let Some(ref s) = self.money_transfer { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_RoomMessageWallet {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MoneyTransfer {
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub trace_number: i64,
    pub invoice_number: i64,
    pub pay_time: u32,
    pub description: String,
}

impl<'a> MessageRead<'a> for MoneyTransfer {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.from_user_id = r.read_uint64(bytes)?,
                Ok(16) => msg.to_user_id = r.read_uint64(bytes)?,
                Ok(24) => msg.amount = r.read_uint64(bytes)?,
                Ok(32) => msg.trace_number = r.read_int64(bytes)?,
                Ok(40) => msg.invoice_number = r.read_int64(bytes)?,
                Ok(48) => msg.pay_time = r.read_uint32(bytes)?,
                Ok(58) => msg.description = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MoneyTransfer {
    fn get_size(&self) -> usize {
        0
        + if self.from_user_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.from_user_id) as u64) }
        + if self.to_user_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.to_user_id) as u64) }
        + if self.amount == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.amount) as u64) }
        + if self.trace_number == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.trace_number) as u64) }
        + if self.invoice_number == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.invoice_number) as u64) }
        + if self.pay_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.pay_time) as u64) }
        + if self.description == String::default() { 0 } else { 1 + sizeof_len((&self.description).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.from_user_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.from_user_id))?; }
        if self.to_user_id != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.to_user_id))?; }
        if self.amount != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.amount))?; }
        if self.trace_number != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.trace_number))?; }
        if self.invoice_number != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.invoice_number))?; }
        if self.pay_time != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.pay_time))?; }
        if self.description != String::default() { w.write_with_tag(58, |w| w.write_string(&**&self.description))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    MONEY_TRANSFER = 0,
}

impl Default for Type {
    fn default() -> Self {
        Type::MONEY_TRANSFER
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::MONEY_TRANSFER,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "MONEY_TRANSFER" => Type::MONEY_TRANSFER,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RoomMessageForwardFrom {
    pub room_id: u64,
    pub message_id: u64,
}

impl<'a> MessageRead<'a> for RoomMessageForwardFrom {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.room_id = r.read_uint64(bytes)?,
                Ok(16) => msg.message_id = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RoomMessageForwardFrom {
    fn get_size(&self) -> usize {
        0
        + if self.room_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.room_id) as u64) }
        + if self.message_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.message_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.room_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.room_id))?; }
        if self.message_id != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.message_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RegisteredUser {
    pub id: u64,
    pub username: String,
    pub phone: u64,
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
    pub initials: String,
    pub color: String,
    pub status: proto::mod_RegisteredUser::Status,
    pub last_seen: u32,
    pub avatar_count: u32,
    pub avatar: Option<proto::Avatar>,
    pub mutual: bool,
    pub deleted: bool,
    pub cache_id: String,
    pub bio: String,
    pub verified: bool,
    pub bot: bool,
}

impl<'a> MessageRead<'a> for RegisteredUser {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint64(bytes)?,
                Ok(18) => msg.username = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.phone = r.read_uint64(bytes)?,
                Ok(34) => msg.first_name = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.last_name = r.read_string(bytes)?.to_owned(),
                Ok(50) => msg.display_name = r.read_string(bytes)?.to_owned(),
                Ok(58) => msg.initials = r.read_string(bytes)?.to_owned(),
                Ok(66) => msg.color = r.read_string(bytes)?.to_owned(),
                Ok(72) => msg.status = r.read_enum(bytes)?,
                Ok(80) => msg.last_seen = r.read_uint32(bytes)?,
                Ok(88) => msg.avatar_count = r.read_uint32(bytes)?,
                Ok(98) => msg.avatar = Some(r.read_message::<proto::Avatar>(bytes)?),
                Ok(104) => msg.mutual = r.read_bool(bytes)?,
                Ok(112) => msg.deleted = r.read_bool(bytes)?,
                Ok(122) => msg.cache_id = r.read_string(bytes)?.to_owned(),
                Ok(130) => msg.bio = r.read_string(bytes)?.to_owned(),
                Ok(136) => msg.verified = r.read_bool(bytes)?,
                Ok(144) => msg.bot = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RegisteredUser {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.username == String::default() { 0 } else { 1 + sizeof_len((&self.username).len()) }
        + if self.phone == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.phone) as u64) }
        + if self.first_name == String::default() { 0 } else { 1 + sizeof_len((&self.first_name).len()) }
        + if self.last_name == String::default() { 0 } else { 1 + sizeof_len((&self.last_name).len()) }
        + if self.display_name == String::default() { 0 } else { 1 + sizeof_len((&self.display_name).len()) }
        + if self.initials == String::default() { 0 } else { 1 + sizeof_len((&self.initials).len()) }
        + if self.color == String::default() { 0 } else { 1 + sizeof_len((&self.color).len()) }
        + if self.status == proto::mod_RegisteredUser::Status::LONG_TIME_AGO { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.last_seen == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.last_seen) as u64) }
        + if self.avatar_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.avatar_count) as u64) }
        + self.avatar.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.mutual == false { 0 } else { 1 + sizeof_varint(*(&self.mutual) as u64) }
        + if self.deleted == false { 0 } else { 1 + sizeof_varint(*(&self.deleted) as u64) }
        + if self.cache_id == String::default() { 0 } else { 1 + sizeof_len((&self.cache_id).len()) }
        + if self.bio == String::default() { 0 } else { 2 + sizeof_len((&self.bio).len()) }
        + if self.verified == false { 0 } else { 2 + sizeof_varint(*(&self.verified) as u64) }
        + if self.bot == false { 0 } else { 2 + sizeof_varint(*(&self.bot) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.id))?; }
        if self.username != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.username))?; }
        if self.phone != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.phone))?; }
        if self.first_name != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.first_name))?; }
        if self.last_name != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.last_name))?; }
        if self.display_name != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.display_name))?; }
        if self.initials != String::default() { w.write_with_tag(58, |w| w.write_string(&**&self.initials))?; }
        if self.color != String::default() { w.write_with_tag(66, |w| w.write_string(&**&self.color))?; }
        if self.status != proto::mod_RegisteredUser::Status::LONG_TIME_AGO { w.write_with_tag(72, |w| w.write_enum(*&self.status as i32))?; }
        if self.last_seen != 0u32 { w.write_with_tag(80, |w| w.write_uint32(*&self.last_seen))?; }
        if self.avatar_count != 0u32 { w.write_with_tag(88, |w| w.write_uint32(*&self.avatar_count))?; }
        if let Some(ref s) = self.avatar { w.write_with_tag(98, |w| w.write_message(s))?; }
        if self.mutual != false { w.write_with_tag(104, |w| w.write_bool(*&self.mutual))?; }
        if self.deleted != false { w.write_with_tag(112, |w| w.write_bool(*&self.deleted))?; }
        if self.cache_id != String::default() { w.write_with_tag(122, |w| w.write_string(&**&self.cache_id))?; }
        if self.bio != String::default() { w.write_with_tag(130, |w| w.write_string(&**&self.bio))?; }
        if self.verified != false { w.write_with_tag(136, |w| w.write_bool(*&self.verified))?; }
        if self.bot != false { w.write_with_tag(144, |w| w.write_bool(*&self.bot))?; }
        Ok(())
    }
}

pub mod mod_RegisteredUser {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    LONG_TIME_AGO = 0,
    LAST_MONTH = 1,
    LAST_WEEK = 2,
    ONLINE = 3,
    EXACTLY = 4,
    RECENTLY = 5,
    SUPPORT = 6,
    SERVICE_NOTIFICATIONS = 7,
}

impl Default for Status {
    fn default() -> Self {
        Status::LONG_TIME_AGO
    }
}

impl From<i32> for Status {
    fn from(i: i32) -> Self {
        match i {
            0 => Status::LONG_TIME_AGO,
            1 => Status::LAST_MONTH,
            2 => Status::LAST_WEEK,
            3 => Status::ONLINE,
            4 => Status::EXACTLY,
            5 => Status::RECENTLY,
            6 => Status::SUPPORT,
            7 => Status::SERVICE_NOTIFICATIONS,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Status {
    fn from(s: &'a str) -> Self {
        match s {
            "LONG_TIME_AGO" => Status::LONG_TIME_AGO,
            "LAST_MONTH" => Status::LAST_MONTH,
            "LAST_WEEK" => Status::LAST_WEEK,
            "ONLINE" => Status::ONLINE,
            "EXACTLY" => Status::EXACTLY,
            "RECENTLY" => Status::RECENTLY,
            "SUPPORT" => Status::SUPPORT,
            "SERVICE_NOTIFICATIONS" => Status::SERVICE_NOTIFICATIONS,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Avatar {
    pub id: u64,
    pub file: Option<proto::File>,
}

impl<'a> MessageRead<'a> for Avatar {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint64(bytes)?,
                Ok(18) => msg.file = Some(r.read_message::<proto::File>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Avatar {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + self.file.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.id))?; }
        if let Some(ref s) = self.file { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RoomMessage {
    pub message_id: u64,
    pub message_version: u64,
    pub status: proto::RoomMessageStatus,
    pub status_version: u64,
    pub message_type: proto::RoomMessageType,
    pub message: String,
    pub attachment: Option<proto::File>,
    pub author: Option<Box<proto::mod_RoomMessage::Author>>,
    pub location: Option<proto::RoomMessageLocation>,
    pub log: Option<proto::RoomMessageLog>,
    pub contact: Option<proto::RoomMessageContact>,
    pub wallet: Option<proto::RoomMessageWallet>,
    pub edited: bool,
    pub create_time: u32,
    pub update_time: u32,
    pub deleted: bool,
    pub forward_from: Option<Box<proto::RoomMessage>>,
    pub reply_to: Option<Box<proto::RoomMessage>>,
    pub previous_message_id: u64,
    pub random_id: u64,
    pub additional_type: u32,
    pub additional_data: String,
    pub extra_type: proto::mod_RoomMessage::ExtraType,
    pub channel_extra: Option<proto::mod_RoomMessage::ChannelExtra>,
}

impl<'a> MessageRead<'a> for RoomMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.message_id = r.read_uint64(bytes)?,
                Ok(16) => msg.message_version = r.read_uint64(bytes)?,
                Ok(24) => msg.status = r.read_enum(bytes)?,
                Ok(32) => msg.status_version = r.read_uint64(bytes)?,
                Ok(40) => msg.message_type = r.read_enum(bytes)?,
                Ok(50) => msg.message = r.read_string(bytes)?.to_owned(),
                Ok(58) => msg.attachment = Some(r.read_message::<proto::File>(bytes)?),
                Ok(66) => msg.author = Some(Box::new(r.read_message::<proto::mod_RoomMessage::Author>(bytes)?)),
                Ok(74) => msg.location = Some(r.read_message::<proto::RoomMessageLocation>(bytes)?),
                Ok(82) => msg.log = Some(r.read_message::<proto::RoomMessageLog>(bytes)?),
                Ok(90) => msg.contact = Some(r.read_message::<proto::RoomMessageContact>(bytes)?),
                Ok(178) => msg.wallet = Some(r.read_message::<proto::RoomMessageWallet>(bytes)?),
                Ok(96) => msg.edited = r.read_bool(bytes)?,
                Ok(104) => msg.create_time = r.read_uint32(bytes)?,
                Ok(112) => msg.update_time = r.read_uint32(bytes)?,
                Ok(120) => msg.deleted = r.read_bool(bytes)?,
                Ok(130) => msg.forward_from = Some(Box::new(r.read_message::<proto::RoomMessage>(bytes)?)),
                Ok(138) => msg.reply_to = Some(Box::new(r.read_message::<proto::RoomMessage>(bytes)?)),
                Ok(144) => msg.previous_message_id = r.read_uint64(bytes)?,
                Ok(168) => msg.random_id = r.read_uint64(bytes)?,
                Ok(184) => msg.additional_type = r.read_uint32(bytes)?,
                Ok(194) => msg.additional_data = r.read_string(bytes)?.to_owned(),
                Ok(152) => msg.extra_type = r.read_enum(bytes)?,
                Ok(162) => msg.channel_extra = Some(r.read_message::<proto::mod_RoomMessage::ChannelExtra>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RoomMessage {
    fn get_size(&self) -> usize {
        0
        + if self.message_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.message_id) as u64) }
        + if self.message_version == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.message_version) as u64) }
        + if self.status == proto::RoomMessageStatus::FAILED { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.status_version == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.status_version) as u64) }
        + if self.message_type == proto::RoomMessageType::TEXT { 0 } else { 1 + sizeof_varint(*(&self.message_type) as u64) }
        + if self.message == String::default() { 0 } else { 1 + sizeof_len((&self.message).len()) }
        + self.attachment.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.author.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.location.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.log.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.contact.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.wallet.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + if self.edited == false { 0 } else { 1 + sizeof_varint(*(&self.edited) as u64) }
        + if self.create_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.create_time) as u64) }
        + if self.update_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.update_time) as u64) }
        + if self.deleted == false { 0 } else { 1 + sizeof_varint(*(&self.deleted) as u64) }
        + self.forward_from.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.reply_to.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + if self.previous_message_id == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.previous_message_id) as u64) }
        + if self.random_id == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.random_id) as u64) }
        + if self.additional_type == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.additional_type) as u64) }
        + if self.additional_data == String::default() { 0 } else { 2 + sizeof_len((&self.additional_data).len()) }
        + if self.extra_type == proto::mod_RoomMessage::ExtraType::NO_EXTRA { 0 } else { 2 + sizeof_varint(*(&self.extra_type) as u64) }
        + self.channel_extra.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.message_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.message_id))?; }
        if self.message_version != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.message_version))?; }
        if self.status != proto::RoomMessageStatus::FAILED { w.write_with_tag(24, |w| w.write_enum(*&self.status as i32))?; }
        if self.status_version != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.status_version))?; }
        if self.message_type != proto::RoomMessageType::TEXT { w.write_with_tag(40, |w| w.write_enum(*&self.message_type as i32))?; }
        if self.message != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.message))?; }
        if let Some(ref s) = self.attachment { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.author { w.write_with_tag(66, |w| w.write_message(&**s))?; }
        if let Some(ref s) = self.location { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.log { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.contact { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.wallet { w.write_with_tag(178, |w| w.write_message(s))?; }
        if self.edited != false { w.write_with_tag(96, |w| w.write_bool(*&self.edited))?; }
        if self.create_time != 0u32 { w.write_with_tag(104, |w| w.write_uint32(*&self.create_time))?; }
        if self.update_time != 0u32 { w.write_with_tag(112, |w| w.write_uint32(*&self.update_time))?; }
        if self.deleted != false { w.write_with_tag(120, |w| w.write_bool(*&self.deleted))?; }
        if let Some(ref s) = self.forward_from { w.write_with_tag(130, |w| w.write_message(&**s))?; }
        if let Some(ref s) = self.reply_to { w.write_with_tag(138, |w| w.write_message(&**s))?; }
        if self.previous_message_id != 0u64 { w.write_with_tag(144, |w| w.write_uint64(*&self.previous_message_id))?; }
        if self.random_id != 0u64 { w.write_with_tag(168, |w| w.write_uint64(*&self.random_id))?; }
        if self.additional_type != 0u32 { w.write_with_tag(184, |w| w.write_uint32(*&self.additional_type))?; }
        if self.additional_data != String::default() { w.write_with_tag(194, |w| w.write_string(&**&self.additional_data))?; }
        if self.extra_type != proto::mod_RoomMessage::ExtraType::NO_EXTRA { w.write_with_tag(152, |w| w.write_enum(*&self.extra_type as i32))?; }
        if let Some(ref s) = self.channel_extra { w.write_with_tag(162, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_RoomMessage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Author {
    pub hash: String,
    pub user: Option<proto::mod_RoomMessage::mod_Author::User>,
    pub room: Option<Box<proto::Room>>,
}

impl<'a> MessageRead<'a> for Author {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hash = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.user = Some(r.read_message::<proto::mod_RoomMessage::mod_Author::User>(bytes)?),
                Ok(26) => msg.room = Some(Box::new(r.read_message::<proto::Room>(bytes)?)),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Author {
    fn get_size(&self) -> usize {
        0
        + if self.hash == String::default() { 0 } else { 1 + sizeof_len((&self.hash).len()) }
        + self.user.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.room.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hash != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.hash))?; }
        if let Some(ref s) = self.user { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.room { w.write_with_tag(26, |w| w.write_message(&**s))?; }
        Ok(())
    }
}

pub mod mod_Author {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct User {
    pub user_id: u64,
    pub cache_id: String,
}

impl<'a> MessageRead<'a> for User {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.user_id = r.read_uint64(bytes)?,
                Ok(18) => msg.cache_id = r.read_string(bytes)?.to_owned(),
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
        + if self.user_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.cache_id == String::default() { 0 } else { 1 + sizeof_len((&self.cache_id).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.user_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.user_id))?; }
        if self.cache_id != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.cache_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Room {
    pub room_id: u64,
}

impl<'a> MessageRead<'a> for Room {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.room_id = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Room {
    fn get_size(&self) -> usize {
        0
        + if self.room_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.room_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.room_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.room_id))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelExtra {
    pub signature: String,
    pub views_label: String,
    pub thumbs_up_label: String,
    pub thumbs_down_label: String,
}

impl<'a> MessageRead<'a> for ChannelExtra {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.signature = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.views_label = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.thumbs_up_label = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.thumbs_down_label = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelExtra {
    fn get_size(&self) -> usize {
        0
        + if self.signature == String::default() { 0 } else { 1 + sizeof_len((&self.signature).len()) }
        + if self.views_label == String::default() { 0 } else { 1 + sizeof_len((&self.views_label).len()) }
        + if self.thumbs_up_label == String::default() { 0 } else { 1 + sizeof_len((&self.thumbs_up_label).len()) }
        + if self.thumbs_down_label == String::default() { 0 } else { 1 + sizeof_len((&self.thumbs_down_label).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.signature != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.signature))?; }
        if self.views_label != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.views_label))?; }
        if self.thumbs_up_label != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.thumbs_up_label))?; }
        if self.thumbs_down_label != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.thumbs_down_label))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExtraType {
    NO_EXTRA = 0,
    CHANNEL_EXTRA = 1,
}

impl Default for ExtraType {
    fn default() -> Self {
        ExtraType::NO_EXTRA
    }
}

impl From<i32> for ExtraType {
    fn from(i: i32) -> Self {
        match i {
            0 => ExtraType::NO_EXTRA,
            1 => ExtraType::CHANNEL_EXTRA,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ExtraType {
    fn from(s: &'a str) -> Self {
        match s {
            "NO_EXTRA" => ExtraType::NO_EXTRA,
            "CHANNEL_EXTRA" => ExtraType::CHANNEL_EXTRA,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RoomDraft {
    pub message: String,
    pub reply_to: u64,
}

impl<'a> MessageRead<'a> for RoomDraft {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.message = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.reply_to = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RoomDraft {
    fn get_size(&self) -> usize {
        0
        + if self.message == String::default() { 0 } else { 1 + sizeof_len((&self.message).len()) }
        + if self.reply_to == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.reply_to) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.message != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.message))?; }
        if self.reply_to != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.reply_to))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Room {
    pub id: u64,
    pub type_pb: proto::mod_Room::Type,
    pub title: String,
    pub initials: String,
    pub color: String,
    pub unread_count: u32,
    pub last_message: Option<Box<proto::RoomMessage>>,
    pub read_only: bool,
    pub is_participant: bool,
    pub draft: Option<proto::RoomDraft>,
    pub first_unread_message: Option<Box<proto::RoomMessage>>,
    pub room_mute: proto::RoomMute,
    pub pin_id: u64,
    pub pinned_message: Option<Box<proto::RoomMessage>>,
    pub priority: u32,
    pub chat_room_extra: Option<proto::ChatRoom>,
    pub group_room_extra: Option<proto::GroupRoom>,
    pub channel_room_extra: Option<proto::ChannelRoom>,
}

impl<'a> MessageRead<'a> for Room {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint64(bytes)?,
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(26) => msg.title = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.initials = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.color = r.read_string(bytes)?.to_owned(),
                Ok(48) => msg.unread_count = r.read_uint32(bytes)?,
                Ok(58) => msg.last_message = Some(Box::new(r.read_message::<proto::RoomMessage>(bytes)?)),
                Ok(64) => msg.read_only = r.read_bool(bytes)?,
                Ok(72) => msg.is_participant = r.read_bool(bytes)?,
                Ok(82) => msg.draft = Some(r.read_message::<proto::RoomDraft>(bytes)?),
                Ok(114) => msg.first_unread_message = Some(Box::new(r.read_message::<proto::RoomMessage>(bytes)?)),
                Ok(120) => msg.room_mute = r.read_enum(bytes)?,
                Ok(128) => msg.pin_id = r.read_uint64(bytes)?,
                Ok(138) => msg.pinned_message = Some(Box::new(r.read_message::<proto::RoomMessage>(bytes)?)),
                Ok(144) => msg.priority = r.read_uint32(bytes)?,
                Ok(90) => msg.chat_room_extra = Some(r.read_message::<proto::ChatRoom>(bytes)?),
                Ok(98) => msg.group_room_extra = Some(r.read_message::<proto::GroupRoom>(bytes)?),
                Ok(106) => msg.channel_room_extra = Some(r.read_message::<proto::ChannelRoom>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Room {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.type_pb == proto::mod_Room::Type::CHAT { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.title == String::default() { 0 } else { 1 + sizeof_len((&self.title).len()) }
        + if self.initials == String::default() { 0 } else { 1 + sizeof_len((&self.initials).len()) }
        + if self.color == String::default() { 0 } else { 1 + sizeof_len((&self.color).len()) }
        + if self.unread_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.unread_count) as u64) }
        + self.last_message.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.read_only == false { 0 } else { 1 + sizeof_varint(*(&self.read_only) as u64) }
        + if self.is_participant == false { 0 } else { 1 + sizeof_varint(*(&self.is_participant) as u64) }
        + self.draft.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.first_unread_message.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.room_mute == proto::RoomMute::UNMUTE { 0 } else { 1 + sizeof_varint(*(&self.room_mute) as u64) }
        + if self.pin_id == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.pin_id) as u64) }
        + self.pinned_message.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + if self.priority == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.priority) as u64) }
        + self.chat_room_extra.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.group_room_extra.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.channel_room_extra.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.id))?; }
        if self.type_pb != proto::mod_Room::Type::CHAT { w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.title != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.title))?; }
        if self.initials != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.initials))?; }
        if self.color != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.color))?; }
        if self.unread_count != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.unread_count))?; }
        if let Some(ref s) = self.last_message { w.write_with_tag(58, |w| w.write_message(&**s))?; }
        if self.read_only != false { w.write_with_tag(64, |w| w.write_bool(*&self.read_only))?; }
        if self.is_participant != false { w.write_with_tag(72, |w| w.write_bool(*&self.is_participant))?; }
        if let Some(ref s) = self.draft { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.first_unread_message { w.write_with_tag(114, |w| w.write_message(&**s))?; }
        if self.room_mute != proto::RoomMute::UNMUTE { w.write_with_tag(120, |w| w.write_enum(*&self.room_mute as i32))?; }
        if self.pin_id != 0u64 { w.write_with_tag(128, |w| w.write_uint64(*&self.pin_id))?; }
        if let Some(ref s) = self.pinned_message { w.write_with_tag(138, |w| w.write_message(&**s))?; }
        if self.priority != 0u32 { w.write_with_tag(144, |w| w.write_uint32(*&self.priority))?; }
        if let Some(ref s) = self.chat_room_extra { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.group_room_extra { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.channel_room_extra { w.write_with_tag(106, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Room {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    CHAT = 0,
    GROUP = 1,
    CHANNEL = 2,
}

impl Default for Type {
    fn default() -> Self {
        Type::CHAT
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::CHAT,
            1 => Type::GROUP,
            2 => Type::CHANNEL,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "CHAT" => Type::CHAT,
            "GROUP" => Type::GROUP,
            "CHANNEL" => Type::CHANNEL,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatRoom {
    pub peer: Option<proto::RegisteredUser>,
}

impl<'a> MessageRead<'a> for ChatRoom {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.peer = Some(r.read_message::<proto::RegisteredUser>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChatRoom {
    fn get_size(&self) -> usize {
        0
        + self.peer.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.peer { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupRoom {
    pub type_pb: proto::mod_GroupRoom::Type,
    pub role: proto::mod_GroupRoom::Role,
    pub participants_count: u32,
    pub participants_count_label: String,
    pub participants_count_limit: u32,
    pub participants_count_limit_label: String,
    pub description: String,
    pub avatar_count: u32,
    pub avatar: Option<proto::Avatar>,
    pub private_extra: Option<proto::mod_GroupRoom::PrivateExtra>,
    pub public_extra: Option<proto::mod_GroupRoom::PublicExtra>,
}

impl<'a> MessageRead<'a> for GroupRoom {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(16) => msg.role = r.read_enum(bytes)?,
                Ok(24) => msg.participants_count = r.read_uint32(bytes)?,
                Ok(34) => msg.participants_count_label = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.participants_count_limit = r.read_uint32(bytes)?,
                Ok(50) => msg.participants_count_limit_label = r.read_string(bytes)?.to_owned(),
                Ok(58) => msg.description = r.read_string(bytes)?.to_owned(),
                Ok(64) => msg.avatar_count = r.read_uint32(bytes)?,
                Ok(74) => msg.avatar = Some(r.read_message::<proto::Avatar>(bytes)?),
                Ok(82) => msg.private_extra = Some(r.read_message::<proto::mod_GroupRoom::PrivateExtra>(bytes)?),
                Ok(90) => msg.public_extra = Some(r.read_message::<proto::mod_GroupRoom::PublicExtra>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupRoom {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == proto::mod_GroupRoom::Type::PRIVATE_ROOM { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.role == proto::mod_GroupRoom::Role::MEMBER { 0 } else { 1 + sizeof_varint(*(&self.role) as u64) }
        + if self.participants_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.participants_count) as u64) }
        + if self.participants_count_label == String::default() { 0 } else { 1 + sizeof_len((&self.participants_count_label).len()) }
        + if self.participants_count_limit == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.participants_count_limit) as u64) }
        + if self.participants_count_limit_label == String::default() { 0 } else { 1 + sizeof_len((&self.participants_count_limit_label).len()) }
        + if self.description == String::default() { 0 } else { 1 + sizeof_len((&self.description).len()) }
        + if self.avatar_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.avatar_count) as u64) }
        + self.avatar.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.private_extra.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.public_extra.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != proto::mod_GroupRoom::Type::PRIVATE_ROOM { w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.role != proto::mod_GroupRoom::Role::MEMBER { w.write_with_tag(16, |w| w.write_enum(*&self.role as i32))?; }
        if self.participants_count != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.participants_count))?; }
        if self.participants_count_label != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.participants_count_label))?; }
        if self.participants_count_limit != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.participants_count_limit))?; }
        if self.participants_count_limit_label != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.participants_count_limit_label))?; }
        if self.description != String::default() { w.write_with_tag(58, |w| w.write_string(&**&self.description))?; }
        if self.avatar_count != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.avatar_count))?; }
        if let Some(ref s) = self.avatar { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.private_extra { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.public_extra { w.write_with_tag(90, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_GroupRoom {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PrivateExtra {
    pub invite_link: String,
    pub invite_token: String,
}

impl<'a> MessageRead<'a> for PrivateExtra {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.invite_link = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.invite_token = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PrivateExtra {
    fn get_size(&self) -> usize {
        0
        + if self.invite_link == String::default() { 0 } else { 1 + sizeof_len((&self.invite_link).len()) }
        + if self.invite_token == String::default() { 0 } else { 1 + sizeof_len((&self.invite_token).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.invite_link != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.invite_link))?; }
        if self.invite_token != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.invite_token))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PublicExtra {
    pub username: String,
}

impl<'a> MessageRead<'a> for PublicExtra {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.username = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PublicExtra {
    fn get_size(&self) -> usize {
        0
        + if self.username == String::default() { 0 } else { 1 + sizeof_len((&self.username).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.username != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.username))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    PRIVATE_ROOM = 0,
    PUBLIC_ROOM = 1,
}

impl Default for Type {
    fn default() -> Self {
        Type::PRIVATE_ROOM
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::PRIVATE_ROOM,
            1 => Type::PUBLIC_ROOM,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "PRIVATE_ROOM" => Type::PRIVATE_ROOM,
            "PUBLIC_ROOM" => Type::PUBLIC_ROOM,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Role {
    MEMBER = 0,
    MODERATOR = 1,
    ADMIN = 2,
    OWNER = 3,
}

impl Default for Role {
    fn default() -> Self {
        Role::MEMBER
    }
}

impl From<i32> for Role {
    fn from(i: i32) -> Self {
        match i {
            0 => Role::MEMBER,
            1 => Role::MODERATOR,
            2 => Role::ADMIN,
            3 => Role::OWNER,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Role {
    fn from(s: &'a str) -> Self {
        match s {
            "MEMBER" => Role::MEMBER,
            "MODERATOR" => Role::MODERATOR,
            "ADMIN" => Role::ADMIN,
            "OWNER" => Role::OWNER,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelRoom {
    pub type_pb: proto::mod_ChannelRoom::Type,
    pub role: proto::mod_ChannelRoom::Role,
    pub participants_count: u32,
    pub participants_count_label: String,
    pub description: String,
    pub avatar_count: u32,
    pub avatar: Option<proto::Avatar>,
    pub private_extra: Option<proto::mod_ChannelRoom::PrivateExtra>,
    pub public_extra: Option<proto::mod_ChannelRoom::PublicExtra>,
    pub signature: bool,
    pub seen_id: u64,
    pub verified: bool,
    pub reaction_status: bool,
}

impl<'a> MessageRead<'a> for ChannelRoom {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(16) => msg.role = r.read_enum(bytes)?,
                Ok(24) => msg.participants_count = r.read_uint32(bytes)?,
                Ok(34) => msg.participants_count_label = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.description = r.read_string(bytes)?.to_owned(),
                Ok(48) => msg.avatar_count = r.read_uint32(bytes)?,
                Ok(58) => msg.avatar = Some(r.read_message::<proto::Avatar>(bytes)?),
                Ok(66) => msg.private_extra = Some(r.read_message::<proto::mod_ChannelRoom::PrivateExtra>(bytes)?),
                Ok(74) => msg.public_extra = Some(r.read_message::<proto::mod_ChannelRoom::PublicExtra>(bytes)?),
                Ok(80) => msg.signature = r.read_bool(bytes)?,
                Ok(88) => msg.seen_id = r.read_uint64(bytes)?,
                Ok(96) => msg.verified = r.read_bool(bytes)?,
                Ok(104) => msg.reaction_status = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelRoom {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == proto::mod_ChannelRoom::Type::PRIVATE_ROOM { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.role == proto::mod_ChannelRoom::Role::MEMBER { 0 } else { 1 + sizeof_varint(*(&self.role) as u64) }
        + if self.participants_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.participants_count) as u64) }
        + if self.participants_count_label == String::default() { 0 } else { 1 + sizeof_len((&self.participants_count_label).len()) }
        + if self.description == String::default() { 0 } else { 1 + sizeof_len((&self.description).len()) }
        + if self.avatar_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.avatar_count) as u64) }
        + self.avatar.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.private_extra.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.public_extra.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.signature == false { 0 } else { 1 + sizeof_varint(*(&self.signature) as u64) }
        + if self.seen_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.seen_id) as u64) }
        + if self.verified == false { 0 } else { 1 + sizeof_varint(*(&self.verified) as u64) }
        + if self.reaction_status == false { 0 } else { 1 + sizeof_varint(*(&self.reaction_status) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != proto::mod_ChannelRoom::Type::PRIVATE_ROOM { w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.role != proto::mod_ChannelRoom::Role::MEMBER { w.write_with_tag(16, |w| w.write_enum(*&self.role as i32))?; }
        if self.participants_count != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.participants_count))?; }
        if self.participants_count_label != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.participants_count_label))?; }
        if self.description != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.description))?; }
        if self.avatar_count != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.avatar_count))?; }
        if let Some(ref s) = self.avatar { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.private_extra { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.public_extra { w.write_with_tag(74, |w| w.write_message(s))?; }
        if self.signature != false { w.write_with_tag(80, |w| w.write_bool(*&self.signature))?; }
        if self.seen_id != 0u64 { w.write_with_tag(88, |w| w.write_uint64(*&self.seen_id))?; }
        if self.verified != false { w.write_with_tag(96, |w| w.write_bool(*&self.verified))?; }
        if self.reaction_status != false { w.write_with_tag(104, |w| w.write_bool(*&self.reaction_status))?; }
        Ok(())
    }
}

pub mod mod_ChannelRoom {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PrivateExtra {
    pub invite_link: String,
    pub invite_token: String,
}

impl<'a> MessageRead<'a> for PrivateExtra {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.invite_link = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.invite_token = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PrivateExtra {
    fn get_size(&self) -> usize {
        0
        + if self.invite_link == String::default() { 0 } else { 1 + sizeof_len((&self.invite_link).len()) }
        + if self.invite_token == String::default() { 0 } else { 1 + sizeof_len((&self.invite_token).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.invite_link != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.invite_link))?; }
        if self.invite_token != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.invite_token))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PublicExtra {
    pub username: String,
}

impl<'a> MessageRead<'a> for PublicExtra {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.username = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PublicExtra {
    fn get_size(&self) -> usize {
        0
        + if self.username == String::default() { 0 } else { 1 + sizeof_len((&self.username).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.username != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.username))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    PRIVATE_ROOM = 0,
    PUBLIC_ROOM = 1,
}

impl Default for Type {
    fn default() -> Self {
        Type::PRIVATE_ROOM
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::PRIVATE_ROOM,
            1 => Type::PUBLIC_ROOM,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "PRIVATE_ROOM" => Type::PRIVATE_ROOM,
            "PUBLIC_ROOM" => Type::PUBLIC_ROOM,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Role {
    MEMBER = 0,
    MODERATOR = 1,
    ADMIN = 2,
    OWNER = 3,
}

impl Default for Role {
    fn default() -> Self {
        Role::MEMBER
    }
}

impl From<i32> for Role {
    fn from(i: i32) -> Self {
        match i {
            0 => Role::MEMBER,
            1 => Role::MODERATOR,
            2 => Role::ADMIN,
            3 => Role::OWNER,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Role {
    fn from(s: &'a str) -> Self {
        match s {
            "MEMBER" => Role::MEMBER,
            "MODERATOR" => Role::MODERATOR,
            "ADMIN" => Role::ADMIN,
            "OWNER" => Role::OWNER,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Thumbnail {
    pub size: i64,
    pub width: i32,
    pub height: i32,
    pub cache_id: String,
    pub name: String,
    pub mime: String,
}

impl<'a> MessageRead<'a> for Thumbnail {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.size = r.read_int64(bytes)?,
                Ok(16) => msg.width = r.read_int32(bytes)?,
                Ok(24) => msg.height = r.read_int32(bytes)?,
                Ok(34) => msg.cache_id = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(50) => msg.mime = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Thumbnail {
    fn get_size(&self) -> usize {
        0
        + if self.size == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.size) as u64) }
        + if self.width == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.width) as u64) }
        + if self.height == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.height) as u64) }
        + if self.cache_id == String::default() { 0 } else { 1 + sizeof_len((&self.cache_id).len()) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.mime == String::default() { 0 } else { 1 + sizeof_len((&self.mime).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.size != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.size))?; }
        if self.width != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.width))?; }
        if self.height != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.height))?; }
        if self.cache_id != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.cache_id))?; }
        if self.name != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.name))?; }
        if self.mime != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.mime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct File {
    pub token: String,
    pub name: String,
    pub size: i64,
    pub large_thumbnail: Option<proto::Thumbnail>,
    pub small_thumbnail: Option<proto::Thumbnail>,
    pub waveform_thumbnail: Option<proto::Thumbnail>,
    pub width: i32,
    pub height: i32,
    pub duration: f64,
    pub cache_id: String,
    pub mime: String,
    pub public_url: String,
}

impl<'a> MessageRead<'a> for File {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.token = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.size = r.read_int64(bytes)?,
                Ok(34) => msg.large_thumbnail = Some(r.read_message::<proto::Thumbnail>(bytes)?),
                Ok(42) => msg.small_thumbnail = Some(r.read_message::<proto::Thumbnail>(bytes)?),
                Ok(50) => msg.waveform_thumbnail = Some(r.read_message::<proto::Thumbnail>(bytes)?),
                Ok(56) => msg.width = r.read_int32(bytes)?,
                Ok(64) => msg.height = r.read_int32(bytes)?,
                Ok(73) => msg.duration = r.read_double(bytes)?,
                Ok(82) => msg.cache_id = r.read_string(bytes)?.to_owned(),
                Ok(90) => msg.mime = r.read_string(bytes)?.to_owned(),
                Ok(98) => msg.public_url = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for File {
    fn get_size(&self) -> usize {
        0
        + if self.token == String::default() { 0 } else { 1 + sizeof_len((&self.token).len()) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.size == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.size) as u64) }
        + self.large_thumbnail.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.small_thumbnail.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.waveform_thumbnail.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.width == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.width) as u64) }
        + if self.height == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.height) as u64) }
        + if self.duration == 0f64 { 0 } else { 1 + 8 }
        + if self.cache_id == String::default() { 0 } else { 1 + sizeof_len((&self.cache_id).len()) }
        + if self.mime == String::default() { 0 } else { 1 + sizeof_len((&self.mime).len()) }
        + if self.public_url == String::default() { 0 } else { 1 + sizeof_len((&self.public_url).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.token != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.token))?; }
        if self.name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.size != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.size))?; }
        if let Some(ref s) = self.large_thumbnail { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.small_thumbnail { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.waveform_thumbnail { w.write_with_tag(50, |w| w.write_message(s))?; }
        if self.width != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.width))?; }
        if self.height != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.height))?; }
        if self.duration != 0f64 { w.write_with_tag(73, |w| w.write_double(*&self.duration))?; }
        if self.cache_id != String::default() { w.write_with_tag(82, |w| w.write_string(&**&self.cache_id))?; }
        if self.mime != String::default() { w.write_with_tag(90, |w| w.write_string(&**&self.mime))?; }
        if self.public_url != String::default() { w.write_with_tag(98, |w| w.write_string(&**&self.public_url))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Wallpaper {
    pub file: Option<proto::File>,
    pub color: String,
}

impl<'a> MessageRead<'a> for Wallpaper {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.file = Some(r.read_message::<proto::File>(bytes)?),
                Ok(18) => msg.color = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Wallpaper {
    fn get_size(&self) -> usize {
        0
        + self.file.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.color == String::default() { 0 } else { 1 + sizeof_len((&self.color).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.file { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.color != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.color))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Pagination {
    pub offset: u32,
    pub limit: u32,
}

impl<'a> MessageRead<'a> for Pagination {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.offset = r.read_uint32(bytes)?,
                Ok(16) => msg.limit = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Pagination {
    fn get_size(&self) -> usize {
        0
        + if self.offset == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.offset) as u64) }
        + if self.limit == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.limit) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.offset != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.offset))?; }
        if self.limit != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.limit))?; }
        Ok(())
    }
}

