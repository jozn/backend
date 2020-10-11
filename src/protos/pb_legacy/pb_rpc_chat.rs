// Automatically generated rust module for 'pb_rpc_chat.proto' file

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
pub struct PB_RPC_Chat_Types { }

impl<'a> MessageRead<'a> for PB_RPC_Chat_Types {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for PB_RPC_Chat_Types { }

pub mod mod_PB_RPC_Chat_Types {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddNewMessage { }

impl<'a> MessageRead<'a> for AddNewMessage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for AddNewMessage { }

pub mod mod_AddNewMessage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub MessageView: Option<pb_changes::PB_MessageView>,
    pub FileBlob: Vec<u8>,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.MessageView = Some(r.read_message::<pb_changes::PB_MessageView>(bytes)?),
                Ok(82) => msg.FileBlob = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Param {
    fn get_size(&self) -> usize {
        0
        + self.MessageView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.FileBlob == vec![] { 0 } else { 1 + sizeof_len((&self.FileBlob).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.MessageView { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.FileBlob != vec![] { w.write_with_tag(82, |w| w.write_bytes(&**&self.FileBlob))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub MessageView: Option<pb_changes::PB_MessageView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.MessageView = Some(r.read_message::<pb_changes::PB_MessageView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Response {
    fn get_size(&self) -> usize {
        0
        + self.MessageView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.MessageView { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SetRoomActionDoing { }

impl<'a> MessageRead<'a> for SetRoomActionDoing {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for SetRoomActionDoing { }

pub mod mod_SetRoomActionDoing {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub GroupId: i64,
    pub DirectRoomKey: String,
    pub ActionType: pb_changes::RoomActionDoingEnum,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.GroupId = r.read_int64(bytes)?,
                Ok(18) => msg.DirectRoomKey = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.ActionType = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Param {
    fn get_size(&self) -> usize {
        0
        + if self.GroupId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.GroupId) as u64) }
        + if self.DirectRoomKey == String::default() { 0 } else { 1 + sizeof_len((&self.DirectRoomKey).len()) }
        + if self.ActionType == pb_changes::RoomActionDoingEnum::UNKNOWN_ROOM_ACTION_DOING { 0 } else { 1 + sizeof_varint(*(&self.ActionType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.GroupId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.GroupId))?; }
        if self.DirectRoomKey != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.DirectRoomKey))?; }
        if self.ActionType != pb_changes::RoomActionDoingEnum::UNKNOWN_ROOM_ACTION_DOING { w.write_with_tag(24, |w| w.write_enum(*&self.ActionType as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response { }

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Response { }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetChatList { }

impl<'a> MessageRead<'a> for GetChatList {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetChatList { }

pub mod mod_GetChatList {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param { }

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Param { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Chats: Vec<pb_changes::PB_ChatView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.Chats.push(r.read_message::<pb_changes::PB_ChatView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Response {
    fn get_size(&self) -> usize {
        0
        + self.Chats.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.Chats { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetChatHistory { }

impl<'a> MessageRead<'a> for GetChatHistory {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetChatHistory { }

pub mod mod_GetChatHistory {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub ChatId: String,
    pub GroupId: String,
    pub Limit: i32,
    pub FromTopMessageId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ChatId = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.GroupId = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.Limit = r.read_int32(bytes)?,
                Ok(32) => msg.FromTopMessageId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Param {
    fn get_size(&self) -> usize {
        0
        + if self.ChatId == String::default() { 0 } else { 1 + sizeof_len((&self.ChatId).len()) }
        + if self.GroupId == String::default() { 0 } else { 1 + sizeof_len((&self.GroupId).len()) }
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.FromTopMessageId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.FromTopMessageId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ChatId != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.ChatId))?; }
        if self.GroupId != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.GroupId))?; }
        if self.Limit != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.Limit))?; }
        if self.FromTopMessageId != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.FromTopMessageId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub MessagesViews: Vec<pb_changes::PB_MessageView>,
    pub HasMore: bool,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.MessagesViews.push(r.read_message::<pb_changes::PB_MessageView>(bytes)?),
                Ok(24) => msg.HasMore = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Response {
    fn get_size(&self) -> usize {
        0
        + self.MessagesViews.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.HasMore == false { 0 } else { 1 + sizeof_varint(*(&self.HasMore) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.MessagesViews { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.HasMore != false { w.write_with_tag(24, |w| w.write_bool(*&self.HasMore))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddRoomsChange { }

impl<'a> MessageRead<'a> for AddRoomsChange {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for AddRoomsChange { }

pub mod mod_AddRoomsChange {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub RoomsChanges: Option<pb_changes::PB_RoomsChanges>,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.RoomsChanges = Some(r.read_message::<pb_changes::PB_RoomsChanges>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Param {
    fn get_size(&self) -> usize {
        0
        + self.RoomsChanges.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.RoomsChanges { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub UserChatVersionSeq: u64,
    pub UserChatVersionTime: u64,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(16) => msg.UserChatVersionSeq = r.read_uint64(bytes)?,
                Ok(24) => msg.UserChatVersionTime = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Response {
    fn get_size(&self) -> usize {
        0
        + if self.Done == false { 0 } else { 1 + sizeof_varint(*(&self.Done) as u64) }
        + if self.UserChatVersionSeq == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.UserChatVersionSeq) as u64) }
        + if self.UserChatVersionTime == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.UserChatVersionTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if self.UserChatVersionSeq != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.UserChatVersionSeq))?; }
        if self.UserChatVersionTime != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.UserChatVersionTime))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetRoomsChange { }

impl<'a> MessageRead<'a> for GetRoomsChange {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetRoomsChange { }

pub mod mod_GetRoomsChange {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub UserChatVersionSeq: u64,
    pub UserChatVersionTime: u64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.UserChatVersionSeq = r.read_uint64(bytes)?,
                Ok(24) => msg.UserChatVersionTime = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Param {
    fn get_size(&self) -> usize {
        0
        + if self.UserChatVersionSeq == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.UserChatVersionSeq) as u64) }
        + if self.UserChatVersionTime == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.UserChatVersionTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserChatVersionSeq != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.UserChatVersionSeq))?; }
        if self.UserChatVersionTime != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.UserChatVersionTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub PushChanges: Option<pb_changes::PB_PushChanges>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.PushChanges = Some(r.read_message::<pb_changes::PB_PushChanges>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Response {
    fn get_size(&self) -> usize {
        0
        + self.PushChanges.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.PushChanges { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

}


