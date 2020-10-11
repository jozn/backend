// Automatically generated rust module for 'pb_changes.proto' file

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

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_RoomsChanges {
    pub VersionTime: u64,
    pub Chats: Vec<mod_PB_RoomsChanges::Chat>,
    pub rooms: Vec<pb_views::PB_ChatView>,
}

impl<'a> MessageRead<'a> for PB_RoomsChanges {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.VersionTime = r.read_uint64(bytes)?,
                Ok(18) => msg.Chats.push(r.read_message::<mod_PB_RoomsChanges::Chat>(bytes)?),
                Ok(26) => msg.rooms.push(r.read_message::<pb_views::PB_ChatView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_RoomsChanges {
    fn get_size(&self) -> usize {
        0
        + if self.VersionTime == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.VersionTime) as u64) }
        + self.Chats.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.rooms.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.VersionTime != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.VersionTime))?; }
        for s in &self.Chats { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.rooms { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_PB_RoomsChanges {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Chat {
    pub ChatId: i64,
    pub RoomKey: String,
    pub RoomType: i32,
    pub PeerPush: i32,
    pub ReceivedMessages: Vec<u64>,
    pub SeenMessages: Vec<u64>,
    pub EditeMessages: Vec<mod_PB_RoomsChanges::mod_Chat::EditeMessage>,
    pub DeleteMessages: Vec<mod_PB_RoomsChanges::mod_Chat::DeleteMessage>,
    pub ClearHistroyFromMessageId: u64,
    pub DeleteChat: i32,
    pub ChatTitle: String,
    pub Muted: mod_PB_RoomsChanges::mod_Chat::Mute,
    pub MutedUntil: i32,
    pub Pined: mod_PB_RoomsChanges::mod_Chat::Pin,
    pub PinTime: i64,
}

impl<'a> MessageRead<'a> for Chat {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ChatId = r.read_int64(bytes)?,
                Ok(26) => msg.RoomKey = r.read_string(bytes)?.to_owned(),
                Ok(32) => msg.RoomType = r.read_int32(bytes)?,
                Ok(40) => msg.PeerPush = r.read_int32(bytes)?,
                Ok(80) => msg.ReceivedMessages.push(r.read_uint64(bytes)?),
                Ok(88) => msg.SeenMessages.push(r.read_uint64(bytes)?),
                Ok(98) => msg.EditeMessages.push(r.read_message::<mod_PB_RoomsChanges::mod_Chat::EditeMessage>(bytes)?),
                Ok(106) => msg.DeleteMessages.push(r.read_message::<mod_PB_RoomsChanges::mod_Chat::DeleteMessage>(bytes)?),
                Ok(112) => msg.ClearHistroyFromMessageId = r.read_uint64(bytes)?,
                Ok(120) => msg.DeleteChat = r.read_int32(bytes)?,
                Ok(130) => msg.ChatTitle = r.read_string(bytes)?.to_owned(),
                Ok(160) => msg.Muted = r.read_enum(bytes)?,
                Ok(168) => msg.MutedUntil = r.read_int32(bytes)?,
                Ok(240) => msg.Pined = r.read_enum(bytes)?,
                Ok(248) => msg.PinTime = r.read_int64(bytes)?,
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
        + if self.ChatId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ChatId) as u64) }
        + if self.RoomKey == String::default() { 0 } else { 1 + sizeof_len((&self.RoomKey).len()) }
        + if self.RoomType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.RoomType) as u64) }
        + if self.PeerPush == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PeerPush) as u64) }
        + self.ReceivedMessages.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.SeenMessages.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.EditeMessages.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.DeleteMessages.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.ClearHistroyFromMessageId == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.ClearHistroyFromMessageId) as u64) }
        + if self.DeleteChat == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.DeleteChat) as u64) }
        + if self.ChatTitle == String::default() { 0 } else { 2 + sizeof_len((&self.ChatTitle).len()) }
        + if self.Muted == pb_changes::mod_PB_RoomsChanges::mod_Chat::Mute::UNCHANGED_MUTE { 0 } else { 2 + sizeof_varint(*(&self.Muted) as u64) }
        + if self.MutedUntil == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.MutedUntil) as u64) }
        + if self.Pined == pb_changes::mod_PB_RoomsChanges::mod_Chat::Pin::UNCHANGED_PIN { 0 } else { 2 + sizeof_varint(*(&self.Pined) as u64) }
        + if self.PinTime == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.PinTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ChatId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.ChatId))?; }
        if self.RoomKey != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.RoomKey))?; }
        if self.RoomType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.RoomType))?; }
        if self.PeerPush != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.PeerPush))?; }
        for s in &self.ReceivedMessages { w.write_with_tag(80, |w| w.write_uint64(*s))?; }
        for s in &self.SeenMessages { w.write_with_tag(88, |w| w.write_uint64(*s))?; }
        for s in &self.EditeMessages { w.write_with_tag(98, |w| w.write_message(s))?; }
        for s in &self.DeleteMessages { w.write_with_tag(106, |w| w.write_message(s))?; }
        if self.ClearHistroyFromMessageId != 0u64 { w.write_with_tag(112, |w| w.write_uint64(*&self.ClearHistroyFromMessageId))?; }
        if self.DeleteChat != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.DeleteChat))?; }
        if self.ChatTitle != String::default() { w.write_with_tag(130, |w| w.write_string(&**&self.ChatTitle))?; }
        if self.Muted != pb_changes::mod_PB_RoomsChanges::mod_Chat::Mute::UNCHANGED_MUTE { w.write_with_tag(160, |w| w.write_enum(*&self.Muted as i32))?; }
        if self.MutedUntil != 0i32 { w.write_with_tag(168, |w| w.write_int32(*&self.MutedUntil))?; }
        if self.Pined != pb_changes::mod_PB_RoomsChanges::mod_Chat::Pin::UNCHANGED_PIN { w.write_with_tag(240, |w| w.write_enum(*&self.Pined as i32))?; }
        if self.PinTime != 0i64 { w.write_with_tag(248, |w| w.write_int64(*&self.PinTime))?; }
        Ok(())
    }
}

pub mod mod_Chat {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct EditeMessage {
    pub MessageId: u64,
    pub NewRext: String,
}

impl<'a> MessageRead<'a> for EditeMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.MessageId = r.read_uint64(bytes)?,
                Ok(18) => msg.NewRext = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for EditeMessage {
    fn get_size(&self) -> usize {
        0
        + if self.MessageId == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.MessageId) as u64) }
        + if self.NewRext == String::default() { 0 } else { 1 + sizeof_len((&self.NewRext).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.MessageId != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.MessageId))?; }
        if self.NewRext != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.NewRext))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteMessage {
    pub MessageId: u64,
    pub Both: bool,
}

impl<'a> MessageRead<'a> for DeleteMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.MessageId = r.read_uint64(bytes)?,
                Ok(16) => msg.Both = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DeleteMessage {
    fn get_size(&self) -> usize {
        0
        + if self.MessageId == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.MessageId) as u64) }
        + if self.Both == false { 0 } else { 1 + sizeof_varint(*(&self.Both) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.MessageId != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.MessageId))?; }
        if self.Both != false { w.write_with_tag(16, |w| w.write_bool(*&self.Both))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mute {
    UNCHANGED_MUTE = 0,
    MUTED = 1,
    UNMUTED = 2,
}

impl Default for Mute {
    fn default() -> Self {
        Mute::UNCHANGED_MUTE
    }
}

impl From<i32> for Mute {
    fn from(i: i32) -> Self {
        match i {
            0 => Mute::UNCHANGED_MUTE,
            1 => Mute::MUTED,
            2 => Mute::UNMUTED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Mute {
    fn from(s: &'a str) -> Self {
        match s {
            "UNCHANGED_MUTE" => Mute::UNCHANGED_MUTE,
            "MUTED" => Mute::MUTED,
            "UNMUTED" => Mute::UNMUTED,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pin {
    UNCHANGED_PIN = 0,
    PINED = 1,
    UNPINED = 2,
}

impl Default for Pin {
    fn default() -> Self {
        Pin::UNCHANGED_PIN
    }
}

impl From<i32> for Pin {
    fn from(i: i32) -> Self {
        match i {
            0 => Pin::UNCHANGED_PIN,
            1 => Pin::PINED,
            2 => Pin::UNPINED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Pin {
    fn from(s: &'a str) -> Self {
        match s {
            "UNCHANGED_PIN" => Pin::UNCHANGED_PIN,
            "PINED" => Pin::PINED,
            "UNPINED" => Pin::UNPINED,
            _ => Self::default(),
        }
    }
}

}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_PushChanges {
    pub RoomsChanges: Vec<PB_RoomsChanges>,
    pub ChatView: Vec<pb_views::PB_ChatView>,
    pub InvalidateCacheForRoomKeys: Vec<String>,
    pub InvalidateAllChatCache: i32,
    pub InvalidateAllSocialCache: i32,
}

impl<'a> MessageRead<'a> for PB_PushChanges {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.RoomsChanges.push(r.read_message::<PB_RoomsChanges>(bytes)?),
                Ok(18) => msg.ChatView.push(r.read_message::<pb_views::PB_ChatView>(bytes)?),
                Ok(26) => msg.InvalidateCacheForRoomKeys.push(r.read_string(bytes)?.to_owned()),
                Ok(80) => msg.InvalidateAllChatCache = r.read_int32(bytes)?,
                Ok(88) => msg.InvalidateAllSocialCache = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_PushChanges {
    fn get_size(&self) -> usize {
        0
        + self.RoomsChanges.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.ChatView.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.InvalidateCacheForRoomKeys.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.InvalidateAllChatCache == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.InvalidateAllChatCache) as u64) }
        + if self.InvalidateAllSocialCache == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.InvalidateAllSocialCache) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.RoomsChanges { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.ChatView { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.InvalidateCacheForRoomKeys { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if self.InvalidateAllChatCache != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.InvalidateAllChatCache))?; }
        if self.InvalidateAllSocialCache != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.InvalidateAllSocialCache))?; }
        Ok(())
    }
}

