// Automatically generated rust module for 'pb_tables.proto' file

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
pub struct PB_Action {
    pub ActionId: i64,
    pub ActorUserId: i32,
    pub ActionType: i32,
    pub PeerUserId: i32,
    pub PostId: i64,
    pub CommentId: i64,
    pub Murmur64Hash: i64,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_Action {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ActionId = r.read_int64(bytes)?,
                Ok(16) => msg.ActorUserId = r.read_int32(bytes)?,
                Ok(24) => msg.ActionType = r.read_int32(bytes)?,
                Ok(32) => msg.PeerUserId = r.read_int32(bytes)?,
                Ok(40) => msg.PostId = r.read_int64(bytes)?,
                Ok(48) => msg.CommentId = r.read_int64(bytes)?,
                Ok(56) => msg.Murmur64Hash = r.read_int64(bytes)?,
                Ok(64) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Action {
    fn get_size(&self) -> usize {
        0
        + if self.ActionId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ActionId) as u64) }
        + if self.ActorUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ActorUserId) as u64) }
        + if self.ActionType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ActionType) as u64) }
        + if self.PeerUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PeerUserId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.CommentId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.CommentId) as u64) }
        + if self.Murmur64Hash == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Murmur64Hash) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ActionId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.ActionId))?; }
        if self.ActorUserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.ActorUserId))?; }
        if self.ActionType != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.ActionType))?; }
        if self.PeerUserId != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.PeerUserId))?; }
        if self.PostId != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.PostId))?; }
        if self.CommentId != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.CommentId))?; }
        if self.Murmur64Hash != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.Murmur64Hash))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Blocked {
    pub Id: i64,
    pub UserId: i32,
    pub BlockedUserId: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_Blocked {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(24) => msg.BlockedUserId = r.read_int32(bytes)?,
                Ok(32) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Blocked {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.BlockedUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.BlockedUserId) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        if self.BlockedUserId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.BlockedUserId))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Comment {
    pub CommentId: i64,
    pub UserId: i32,
    pub PostId: i64,
    pub Text: String,
    pub LikesCount: i32,
    pub IsEdited: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_Comment {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.CommentId = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(24) => msg.PostId = r.read_int64(bytes)?,
                Ok(34) => msg.Text = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.LikesCount = r.read_int32(bytes)?,
                Ok(48) => msg.IsEdited = r.read_int32(bytes)?,
                Ok(56) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Comment {
    fn get_size(&self) -> usize {
        0
        + if self.CommentId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.CommentId) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.Text == String::default() { 0 } else { 1 + sizeof_len((&self.Text).len()) }
        + if self.LikesCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.LikesCount) as u64) }
        + if self.IsEdited == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.IsEdited) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.CommentId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.CommentId))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        if self.PostId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.PostId))?; }
        if self.Text != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.Text))?; }
        if self.LikesCount != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.LikesCount))?; }
        if self.IsEdited != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.IsEdited))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_CommentDeleted {
    pub CommentId: i64,
    pub UserId: i32,
}

impl<'a> MessageRead<'a> for PB_CommentDeleted {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.CommentId = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_CommentDeleted {
    fn get_size(&self) -> usize {
        0
        + if self.CommentId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.CommentId) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.CommentId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.CommentId))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Event {
    pub EventId: i64,
    pub EventType: i32,
    pub ByUserId: i64,
    pub PeerUserId: i64,
    pub PostId: i64,
    pub CommentId: i64,
    pub HashTagId: i64,
    pub GroupId: i64,
    pub ActionId: i64,
    pub ChatId: i64,
    pub ChatKey: String,
    pub MessageId: i64,
    pub ReSharedId: i64,
    pub Murmur64Hash: i64,
}

impl<'a> MessageRead<'a> for PB_Event {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.EventId = r.read_int64(bytes)?,
                Ok(16) => msg.EventType = r.read_int32(bytes)?,
                Ok(24) => msg.ByUserId = r.read_int64(bytes)?,
                Ok(32) => msg.PeerUserId = r.read_int64(bytes)?,
                Ok(40) => msg.PostId = r.read_int64(bytes)?,
                Ok(48) => msg.CommentId = r.read_int64(bytes)?,
                Ok(56) => msg.HashTagId = r.read_int64(bytes)?,
                Ok(64) => msg.GroupId = r.read_int64(bytes)?,
                Ok(72) => msg.ActionId = r.read_int64(bytes)?,
                Ok(80) => msg.ChatId = r.read_int64(bytes)?,
                Ok(90) => msg.ChatKey = r.read_string(bytes)?.to_owned(),
                Ok(96) => msg.MessageId = r.read_int64(bytes)?,
                Ok(104) => msg.ReSharedId = r.read_int64(bytes)?,
                Ok(112) => msg.Murmur64Hash = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Event {
    fn get_size(&self) -> usize {
        0
        + if self.EventId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.EventId) as u64) }
        + if self.EventType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.EventType) as u64) }
        + if self.ByUserId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ByUserId) as u64) }
        + if self.PeerUserId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PeerUserId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.CommentId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.CommentId) as u64) }
        + if self.HashTagId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.HashTagId) as u64) }
        + if self.GroupId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.GroupId) as u64) }
        + if self.ActionId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ActionId) as u64) }
        + if self.ChatId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ChatId) as u64) }
        + if self.ChatKey == String::default() { 0 } else { 1 + sizeof_len((&self.ChatKey).len()) }
        + if self.MessageId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.MessageId) as u64) }
        + if self.ReSharedId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ReSharedId) as u64) }
        + if self.Murmur64Hash == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Murmur64Hash) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.EventId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.EventId))?; }
        if self.EventType != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.EventType))?; }
        if self.ByUserId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.ByUserId))?; }
        if self.PeerUserId != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.PeerUserId))?; }
        if self.PostId != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.PostId))?; }
        if self.CommentId != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.CommentId))?; }
        if self.HashTagId != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.HashTagId))?; }
        if self.GroupId != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.GroupId))?; }
        if self.ActionId != 0i64 { w.write_with_tag(72, |w| w.write_int64(*&self.ActionId))?; }
        if self.ChatId != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.ChatId))?; }
        if self.ChatKey != String::default() { w.write_with_tag(90, |w| w.write_string(&**&self.ChatKey))?; }
        if self.MessageId != 0i64 { w.write_with_tag(96, |w| w.write_int64(*&self.MessageId))?; }
        if self.ReSharedId != 0i64 { w.write_with_tag(104, |w| w.write_int64(*&self.ReSharedId))?; }
        if self.Murmur64Hash != 0i64 { w.write_with_tag(112, |w| w.write_int64(*&self.Murmur64Hash))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Followed {
    pub Id: i64,
    pub UserId: i32,
    pub FollowedUserId: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_Followed {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(24) => msg.FollowedUserId = r.read_int32(bytes)?,
                Ok(32) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Followed {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.FollowedUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.FollowedUserId) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        if self.FollowedUserId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.FollowedUserId))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Likes {
    pub Id: i64,
    pub PostId: i64,
    pub UserId: i32,
    pub PostType: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_Likes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.PostId = r.read_int64(bytes)?,
                Ok(24) => msg.UserId = r.read_int32(bytes)?,
                Ok(32) => msg.PostType = r.read_int32(bytes)?,
                Ok(40) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Likes {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.PostType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PostType) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.PostId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.PostId))?; }
        if self.UserId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.UserId))?; }
        if self.PostType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.PostType))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Notify {
    pub NotifyId: i64,
    pub ForUserId: i32,
    pub ActorUserId: i32,
    pub NotifyType: i32,
    pub PostId: i64,
    pub CommentId: i64,
    pub PeerUserId: i32,
    pub Murmur64Hash: i64,
    pub SeenStatus: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_Notify {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.NotifyId = r.read_int64(bytes)?,
                Ok(16) => msg.ForUserId = r.read_int32(bytes)?,
                Ok(24) => msg.ActorUserId = r.read_int32(bytes)?,
                Ok(32) => msg.NotifyType = r.read_int32(bytes)?,
                Ok(40) => msg.PostId = r.read_int64(bytes)?,
                Ok(48) => msg.CommentId = r.read_int64(bytes)?,
                Ok(56) => msg.PeerUserId = r.read_int32(bytes)?,
                Ok(64) => msg.Murmur64Hash = r.read_int64(bytes)?,
                Ok(72) => msg.SeenStatus = r.read_int32(bytes)?,
                Ok(80) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Notify {
    fn get_size(&self) -> usize {
        0
        + if self.NotifyId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.NotifyId) as u64) }
        + if self.ForUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ForUserId) as u64) }
        + if self.ActorUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ActorUserId) as u64) }
        + if self.NotifyType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.NotifyType) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.CommentId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.CommentId) as u64) }
        + if self.PeerUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PeerUserId) as u64) }
        + if self.Murmur64Hash == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Murmur64Hash) as u64) }
        + if self.SeenStatus == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.SeenStatus) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.NotifyId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.NotifyId))?; }
        if self.ForUserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.ForUserId))?; }
        if self.ActorUserId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.ActorUserId))?; }
        if self.NotifyType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.NotifyType))?; }
        if self.PostId != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.PostId))?; }
        if self.CommentId != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.CommentId))?; }
        if self.PeerUserId != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.PeerUserId))?; }
        if self.Murmur64Hash != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.Murmur64Hash))?; }
        if self.SeenStatus != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.SeenStatus))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_NotifyRemoved {
    pub Murmur64Hash: i64,
    pub ForUserId: i32,
    pub Id: i64,
}

impl<'a> MessageRead<'a> for PB_NotifyRemoved {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Murmur64Hash = r.read_int64(bytes)?,
                Ok(16) => msg.ForUserId = r.read_int32(bytes)?,
                Ok(24) => msg.Id = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_NotifyRemoved {
    fn get_size(&self) -> usize {
        0
        + if self.Murmur64Hash == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Murmur64Hash) as u64) }
        + if self.ForUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ForUserId) as u64) }
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Murmur64Hash != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Murmur64Hash))?; }
        if self.ForUserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.ForUserId))?; }
        if self.Id != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.Id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_PhoneContacts {
    pub Id: i64,
    pub UserId: i32,
    pub ClientId: i64,
    pub Phone: String,
    pub FirstName: String,
    pub LastName: String,
}

impl<'a> MessageRead<'a> for PB_PhoneContacts {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(24) => msg.ClientId = r.read_int64(bytes)?,
                Ok(34) => msg.Phone = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.FirstName = r.read_string(bytes)?.to_owned(),
                Ok(50) => msg.LastName = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_PhoneContacts {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.ClientId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ClientId) as u64) }
        + if self.Phone == String::default() { 0 } else { 1 + sizeof_len((&self.Phone).len()) }
        + if self.FirstName == String::default() { 0 } else { 1 + sizeof_len((&self.FirstName).len()) }
        + if self.LastName == String::default() { 0 } else { 1 + sizeof_len((&self.LastName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        if self.ClientId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.ClientId))?; }
        if self.Phone != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.Phone))?; }
        if self.FirstName != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.FirstName))?; }
        if self.LastName != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.LastName))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Post {
    pub PostId: i64,
    pub UserId: i32,
    pub PostType: i32,
    pub MediaId: i64,
    pub FileRefId: i64,
    pub PostKey: String,
    pub Text: String,
    pub RichText: String,
    pub MediaCount: i32,
    pub SharedTo: i32,
    pub DisableComment: i32,
    pub Via: i32,
    pub Seq: i32,
    pub CommentsCount: i32,
    pub LikesCount: i32,
    pub ViewsCount: i32,
    pub EditedTime: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_Post {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(24) => msg.PostType = r.read_int32(bytes)?,
                Ok(32) => msg.MediaId = r.read_int64(bytes)?,
                Ok(40) => msg.FileRefId = r.read_int64(bytes)?,
                Ok(50) => msg.PostKey = r.read_string(bytes)?.to_owned(),
                Ok(58) => msg.Text = r.read_string(bytes)?.to_owned(),
                Ok(66) => msg.RichText = r.read_string(bytes)?.to_owned(),
                Ok(72) => msg.MediaCount = r.read_int32(bytes)?,
                Ok(80) => msg.SharedTo = r.read_int32(bytes)?,
                Ok(88) => msg.DisableComment = r.read_int32(bytes)?,
                Ok(96) => msg.Via = r.read_int32(bytes)?,
                Ok(104) => msg.Seq = r.read_int32(bytes)?,
                Ok(112) => msg.CommentsCount = r.read_int32(bytes)?,
                Ok(120) => msg.LikesCount = r.read_int32(bytes)?,
                Ok(128) => msg.ViewsCount = r.read_int32(bytes)?,
                Ok(136) => msg.EditedTime = r.read_int32(bytes)?,
                Ok(144) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Post {
    fn get_size(&self) -> usize {
        0
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.PostType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PostType) as u64) }
        + if self.MediaId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.MediaId) as u64) }
        + if self.FileRefId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.FileRefId) as u64) }
        + if self.PostKey == String::default() { 0 } else { 1 + sizeof_len((&self.PostKey).len()) }
        + if self.Text == String::default() { 0 } else { 1 + sizeof_len((&self.Text).len()) }
        + if self.RichText == String::default() { 0 } else { 1 + sizeof_len((&self.RichText).len()) }
        + if self.MediaCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.MediaCount) as u64) }
        + if self.SharedTo == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.SharedTo) as u64) }
        + if self.DisableComment == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.DisableComment) as u64) }
        + if self.Via == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Via) as u64) }
        + if self.Seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Seq) as u64) }
        + if self.CommentsCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CommentsCount) as u64) }
        + if self.LikesCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.LikesCount) as u64) }
        + if self.ViewsCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.ViewsCount) as u64) }
        + if self.EditedTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.EditedTime) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        if self.PostType != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.PostType))?; }
        if self.MediaId != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.MediaId))?; }
        if self.FileRefId != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.FileRefId))?; }
        if self.PostKey != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.PostKey))?; }
        if self.Text != String::default() { w.write_with_tag(58, |w| w.write_string(&**&self.Text))?; }
        if self.RichText != String::default() { w.write_with_tag(66, |w| w.write_string(&**&self.RichText))?; }
        if self.MediaCount != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.MediaCount))?; }
        if self.SharedTo != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.SharedTo))?; }
        if self.DisableComment != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.DisableComment))?; }
        if self.Via != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.Via))?; }
        if self.Seq != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.Seq))?; }
        if self.CommentsCount != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.CommentsCount))?; }
        if self.LikesCount != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.LikesCount))?; }
        if self.ViewsCount != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.ViewsCount))?; }
        if self.EditedTime != 0i32 { w.write_with_tag(136, |w| w.write_int32(*&self.EditedTime))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(144, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_PostCount {
    pub PostId: i64,
    pub CommentsCount: i32,
    pub LikesCount: i32,
    pub ViewsCount: i64,
    pub ReSharedCount: i32,
    pub ChatSharedCount: i32,
}

impl<'a> MessageRead<'a> for PB_PostCount {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(16) => msg.CommentsCount = r.read_int32(bytes)?,
                Ok(24) => msg.LikesCount = r.read_int32(bytes)?,
                Ok(32) => msg.ViewsCount = r.read_int64(bytes)?,
                Ok(40) => msg.ReSharedCount = r.read_int32(bytes)?,
                Ok(48) => msg.ChatSharedCount = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_PostCount {
    fn get_size(&self) -> usize {
        0
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.CommentsCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CommentsCount) as u64) }
        + if self.LikesCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.LikesCount) as u64) }
        + if self.ViewsCount == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ViewsCount) as u64) }
        + if self.ReSharedCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ReSharedCount) as u64) }
        + if self.ChatSharedCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ChatSharedCount) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.CommentsCount != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.CommentsCount))?; }
        if self.LikesCount != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.LikesCount))?; }
        if self.ViewsCount != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.ViewsCount))?; }
        if self.ReSharedCount != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.ReSharedCount))?; }
        if self.ChatSharedCount != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.ChatSharedCount))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_PostDeleted {
    pub PostId: i64,
    pub UserId: i32,
}

impl<'a> MessageRead<'a> for PB_PostDeleted {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_PostDeleted {
    fn get_size(&self) -> usize {
        0
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_PostKeys {
    pub Id: i32,
    pub PostKeyStr: String,
    pub Used: i32,
    pub RandShard: i32,
}

impl<'a> MessageRead<'a> for PB_PostKeys {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int32(bytes)?,
                Ok(18) => msg.PostKeyStr = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.Used = r.read_int32(bytes)?,
                Ok(32) => msg.RandShard = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_PostKeys {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.PostKeyStr == String::default() { 0 } else { 1 + sizeof_len((&self.PostKeyStr).len()) }
        + if self.Used == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Used) as u64) }
        + if self.RandShard == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.RandShard) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.Id))?; }
        if self.PostKeyStr != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.PostKeyStr))?; }
        if self.Used != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.Used))?; }
        if self.RandShard != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.RandShard))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_PostLink {
    pub LinkId: i64,
    pub LinkUrl: String,
}

impl<'a> MessageRead<'a> for PB_PostLink {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.LinkId = r.read_int64(bytes)?,
                Ok(18) => msg.LinkUrl = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_PostLink {
    fn get_size(&self) -> usize {
        0
        + if self.LinkId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LinkId) as u64) }
        + if self.LinkUrl == String::default() { 0 } else { 1 + sizeof_len((&self.LinkUrl).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.LinkId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.LinkId))?; }
        if self.LinkUrl != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.LinkUrl))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_PostMedia {
    pub MediaId: i64,
    pub UserId: i32,
    pub PostId: i64,
    pub AlbumId: i32,
    pub MediaTypeEnum: i32,
    pub Width: i32,
    pub Height: i32,
    pub Size: i32,
    pub Duration: i32,
    pub Extension: String,
    pub Md5Hash: String,
    pub Color: String,
    pub CreatedTime: i32,
    pub ViewCount: i32,
    pub Extra: String,
}

impl<'a> MessageRead<'a> for PB_PostMedia {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.MediaId = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(24) => msg.PostId = r.read_int64(bytes)?,
                Ok(32) => msg.AlbumId = r.read_int32(bytes)?,
                Ok(40) => msg.MediaTypeEnum = r.read_int32(bytes)?,
                Ok(48) => msg.Width = r.read_int32(bytes)?,
                Ok(56) => msg.Height = r.read_int32(bytes)?,
                Ok(64) => msg.Size = r.read_int32(bytes)?,
                Ok(72) => msg.Duration = r.read_int32(bytes)?,
                Ok(82) => msg.Extension = r.read_string(bytes)?.to_owned(),
                Ok(90) => msg.Md5Hash = r.read_string(bytes)?.to_owned(),
                Ok(98) => msg.Color = r.read_string(bytes)?.to_owned(),
                Ok(104) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(112) => msg.ViewCount = r.read_int32(bytes)?,
                Ok(122) => msg.Extra = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_PostMedia {
    fn get_size(&self) -> usize {
        0
        + if self.MediaId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.MediaId) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.AlbumId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.AlbumId) as u64) }
        + if self.MediaTypeEnum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.MediaTypeEnum) as u64) }
        + if self.Width == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Width) as u64) }
        + if self.Height == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Height) as u64) }
        + if self.Size == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Size) as u64) }
        + if self.Duration == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Duration) as u64) }
        + if self.Extension == String::default() { 0 } else { 1 + sizeof_len((&self.Extension).len()) }
        + if self.Md5Hash == String::default() { 0 } else { 1 + sizeof_len((&self.Md5Hash).len()) }
        + if self.Color == String::default() { 0 } else { 1 + sizeof_len((&self.Color).len()) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
        + if self.ViewCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ViewCount) as u64) }
        + if self.Extra == String::default() { 0 } else { 1 + sizeof_len((&self.Extra).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.MediaId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.MediaId))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        if self.PostId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.PostId))?; }
        if self.AlbumId != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.AlbumId))?; }
        if self.MediaTypeEnum != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.MediaTypeEnum))?; }
        if self.Width != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.Width))?; }
        if self.Height != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.Height))?; }
        if self.Size != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.Size))?; }
        if self.Duration != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.Duration))?; }
        if self.Extension != String::default() { w.write_with_tag(82, |w| w.write_string(&**&self.Extension))?; }
        if self.Md5Hash != String::default() { w.write_with_tag(90, |w| w.write_string(&**&self.Md5Hash))?; }
        if self.Color != String::default() { w.write_with_tag(98, |w| w.write_string(&**&self.Color))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.CreatedTime))?; }
        if self.ViewCount != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.ViewCount))?; }
        if self.Extra != String::default() { w.write_with_tag(122, |w| w.write_string(&**&self.Extra))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_PostPromoted {
    pub PromoteId: i32,
    pub PostId: i64,
    pub ByUserId: i32,
    pub PostUserId: i32,
    pub BazzarUuid: String,
    pub Package: String,
    pub EndTime: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_PostPromoted {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PromoteId = r.read_int32(bytes)?,
                Ok(16) => msg.PostId = r.read_int64(bytes)?,
                Ok(24) => msg.ByUserId = r.read_int32(bytes)?,
                Ok(32) => msg.PostUserId = r.read_int32(bytes)?,
                Ok(42) => msg.BazzarUuid = r.read_string(bytes)?.to_owned(),
                Ok(50) => msg.Package = r.read_string(bytes)?.to_owned(),
                Ok(56) => msg.EndTime = r.read_int32(bytes)?,
                Ok(64) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_PostPromoted {
    fn get_size(&self) -> usize {
        0
        + if self.PromoteId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PromoteId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.ByUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ByUserId) as u64) }
        + if self.PostUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PostUserId) as u64) }
        + if self.BazzarUuid == String::default() { 0 } else { 1 + sizeof_len((&self.BazzarUuid).len()) }
        + if self.Package == String::default() { 0 } else { 1 + sizeof_len((&self.Package).len()) }
        + if self.EndTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.EndTime) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PromoteId != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.PromoteId))?; }
        if self.PostId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.PostId))?; }
        if self.ByUserId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.ByUserId))?; }
        if self.PostUserId != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.PostUserId))?; }
        if self.BazzarUuid != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.BazzarUuid))?; }
        if self.Package != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.Package))?; }
        if self.EndTime != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.EndTime))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_PostReshared {
    pub ResharedId: i64,
    pub PostId: i64,
    pub ByUserId: i32,
    pub PostUserId: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_PostReshared {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ResharedId = r.read_int64(bytes)?,
                Ok(16) => msg.PostId = r.read_int64(bytes)?,
                Ok(24) => msg.ByUserId = r.read_int32(bytes)?,
                Ok(32) => msg.PostUserId = r.read_int32(bytes)?,
                Ok(40) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_PostReshared {
    fn get_size(&self) -> usize {
        0
        + if self.ResharedId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ResharedId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.ByUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ByUserId) as u64) }
        + if self.PostUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PostUserId) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ResharedId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.ResharedId))?; }
        if self.PostId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.PostId))?; }
        if self.ByUserId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.ByUserId))?; }
        if self.PostUserId != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.PostUserId))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_ProfileAll {
    pub Id: i64,
    pub UserId: i32,
    pub PostId: i64,
    pub IsReShared: i32,
}

impl<'a> MessageRead<'a> for PB_ProfileAll {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(24) => msg.PostId = r.read_int64(bytes)?,
                Ok(32) => msg.IsReShared = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_ProfileAll {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.IsReShared == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.IsReShared) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        if self.PostId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.PostId))?; }
        if self.IsReShared != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.IsReShared))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_ProfileMedia {
    pub Id: i64,
    pub UserId: i32,
    pub PostId: i64,
    pub PostType: i32,
}

impl<'a> MessageRead<'a> for PB_ProfileMedia {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(24) => msg.PostId = r.read_int64(bytes)?,
                Ok(32) => msg.PostType = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_ProfileMedia {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.PostType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PostType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        if self.PostId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.PostId))?; }
        if self.PostType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.PostType))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_ProfileMentioned {
    pub Id: i64,
    pub ForUserId: i32,
    pub PostId: i64,
    pub PostUserId: i32,
    pub PostType: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_ProfileMentioned {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.ForUserId = r.read_int32(bytes)?,
                Ok(24) => msg.PostId = r.read_int64(bytes)?,
                Ok(32) => msg.PostUserId = r.read_int32(bytes)?,
                Ok(40) => msg.PostType = r.read_int32(bytes)?,
                Ok(48) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_ProfileMentioned {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.ForUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ForUserId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.PostUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PostUserId) as u64) }
        + if self.PostType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PostType) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.ForUserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.ForUserId))?; }
        if self.PostId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.PostId))?; }
        if self.PostUserId != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.PostUserId))?; }
        if self.PostType != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.PostType))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Session {
    pub Id: i64,
    pub SessionUuid: String,
    pub UserId: i32,
    pub LastIpAddress: String,
    pub AppVersion: i32,
    pub ActiveTime: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_Session {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(18) => msg.SessionUuid = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.UserId = r.read_int32(bytes)?,
                Ok(34) => msg.LastIpAddress = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.AppVersion = r.read_int32(bytes)?,
                Ok(48) => msg.ActiveTime = r.read_int32(bytes)?,
                Ok(56) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Session {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.SessionUuid == String::default() { 0 } else { 1 + sizeof_len((&self.SessionUuid).len()) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.LastIpAddress == String::default() { 0 } else { 1 + sizeof_len((&self.LastIpAddress).len()) }
        + if self.AppVersion == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.AppVersion) as u64) }
        + if self.ActiveTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ActiveTime) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.SessionUuid != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.SessionUuid))?; }
        if self.UserId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.UserId))?; }
        if self.LastIpAddress != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.LastIpAddress))?; }
        if self.AppVersion != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.AppVersion))?; }
        if self.ActiveTime != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.ActiveTime))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_SettingNotifications {
    pub UserId: i32,
    pub SocialLedOn: i32,
    pub SocialLedColor: String,
    pub ReqestToFollowYou: i32,
    pub FollowedYou: i32,
    pub AccptedYourFollowRequest: i32,
    pub YourPostLiked: i32,
    pub YourPostCommented: i32,
    pub MenthenedYouInPost: i32,
    pub MenthenedYouInComment: i32,
    pub YourContactsJoined: i32,
    pub DirectMessage: i32,
    pub DirectAlert: i32,
    pub DirectPerview: i32,
    pub DirectLedOn: i32,
    pub DirectLedColor: i32,
    pub DirectVibrate: i32,
    pub DirectPopup: i32,
    pub DirectSound: i32,
    pub DirectPriority: i32,
}

impl<'a> MessageRead<'a> for PB_SettingNotifications {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.UserId = r.read_int32(bytes)?,
                Ok(16) => msg.SocialLedOn = r.read_int32(bytes)?,
                Ok(26) => msg.SocialLedColor = r.read_string(bytes)?.to_owned(),
                Ok(32) => msg.ReqestToFollowYou = r.read_int32(bytes)?,
                Ok(40) => msg.FollowedYou = r.read_int32(bytes)?,
                Ok(48) => msg.AccptedYourFollowRequest = r.read_int32(bytes)?,
                Ok(56) => msg.YourPostLiked = r.read_int32(bytes)?,
                Ok(64) => msg.YourPostCommented = r.read_int32(bytes)?,
                Ok(72) => msg.MenthenedYouInPost = r.read_int32(bytes)?,
                Ok(80) => msg.MenthenedYouInComment = r.read_int32(bytes)?,
                Ok(88) => msg.YourContactsJoined = r.read_int32(bytes)?,
                Ok(96) => msg.DirectMessage = r.read_int32(bytes)?,
                Ok(104) => msg.DirectAlert = r.read_int32(bytes)?,
                Ok(112) => msg.DirectPerview = r.read_int32(bytes)?,
                Ok(120) => msg.DirectLedOn = r.read_int32(bytes)?,
                Ok(128) => msg.DirectLedColor = r.read_int32(bytes)?,
                Ok(136) => msg.DirectVibrate = r.read_int32(bytes)?,
                Ok(144) => msg.DirectPopup = r.read_int32(bytes)?,
                Ok(152) => msg.DirectSound = r.read_int32(bytes)?,
                Ok(160) => msg.DirectPriority = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_SettingNotifications {
    fn get_size(&self) -> usize {
        0
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.SocialLedOn == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.SocialLedOn) as u64) }
        + if self.SocialLedColor == String::default() { 0 } else { 1 + sizeof_len((&self.SocialLedColor).len()) }
        + if self.ReqestToFollowYou == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ReqestToFollowYou) as u64) }
        + if self.FollowedYou == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.FollowedYou) as u64) }
        + if self.AccptedYourFollowRequest == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.AccptedYourFollowRequest) as u64) }
        + if self.YourPostLiked == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.YourPostLiked) as u64) }
        + if self.YourPostCommented == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.YourPostCommented) as u64) }
        + if self.MenthenedYouInPost == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.MenthenedYouInPost) as u64) }
        + if self.MenthenedYouInComment == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.MenthenedYouInComment) as u64) }
        + if self.YourContactsJoined == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.YourContactsJoined) as u64) }
        + if self.DirectMessage == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.DirectMessage) as u64) }
        + if self.DirectAlert == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.DirectAlert) as u64) }
        + if self.DirectPerview == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.DirectPerview) as u64) }
        + if self.DirectLedOn == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.DirectLedOn) as u64) }
        + if self.DirectLedColor == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.DirectLedColor) as u64) }
        + if self.DirectVibrate == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.DirectVibrate) as u64) }
        + if self.DirectPopup == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.DirectPopup) as u64) }
        + if self.DirectSound == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.DirectSound) as u64) }
        + if self.DirectPriority == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.DirectPriority) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserId != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.UserId))?; }
        if self.SocialLedOn != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.SocialLedOn))?; }
        if self.SocialLedColor != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.SocialLedColor))?; }
        if self.ReqestToFollowYou != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.ReqestToFollowYou))?; }
        if self.FollowedYou != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.FollowedYou))?; }
        if self.AccptedYourFollowRequest != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.AccptedYourFollowRequest))?; }
        if self.YourPostLiked != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.YourPostLiked))?; }
        if self.YourPostCommented != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.YourPostCommented))?; }
        if self.MenthenedYouInPost != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.MenthenedYouInPost))?; }
        if self.MenthenedYouInComment != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.MenthenedYouInComment))?; }
        if self.YourContactsJoined != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.YourContactsJoined))?; }
        if self.DirectMessage != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.DirectMessage))?; }
        if self.DirectAlert != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.DirectAlert))?; }
        if self.DirectPerview != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.DirectPerview))?; }
        if self.DirectLedOn != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.DirectLedOn))?; }
        if self.DirectLedColor != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.DirectLedColor))?; }
        if self.DirectVibrate != 0i32 { w.write_with_tag(136, |w| w.write_int32(*&self.DirectVibrate))?; }
        if self.DirectPopup != 0i32 { w.write_with_tag(144, |w| w.write_int32(*&self.DirectPopup))?; }
        if self.DirectSound != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.DirectSound))?; }
        if self.DirectPriority != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.DirectPriority))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Sms {
    pub Id: i32,
    pub Hash: String,
    pub AppUuid: String,
    pub ClientPhone: String,
    pub GenratedCode: i32,
    pub SmsSenderNumber: String,
    pub SmsSendStatues: String,
    pub SmsHttpBody: String,
    pub Err_pb: String,
    pub Carrier: String,
    pub Country: Vec<u8>,
    pub IsValidPhone: i32,
    pub IsConfirmed: i32,
    pub IsLogin: i32,
    pub IsRegister: i32,
    pub RetriedCount: i32,
    pub TTL: i32,
}

impl<'a> MessageRead<'a> for PB_Sms {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int32(bytes)?,
                Ok(18) => msg.Hash = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.AppUuid = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.ClientPhone = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.GenratedCode = r.read_int32(bytes)?,
                Ok(50) => msg.SmsSenderNumber = r.read_string(bytes)?.to_owned(),
                Ok(58) => msg.SmsSendStatues = r.read_string(bytes)?.to_owned(),
                Ok(66) => msg.SmsHttpBody = r.read_string(bytes)?.to_owned(),
                Ok(74) => msg.Err_pb = r.read_string(bytes)?.to_owned(),
                Ok(82) => msg.Carrier = r.read_string(bytes)?.to_owned(),
                Ok(90) => msg.Country = r.read_bytes(bytes)?.to_owned(),
                Ok(96) => msg.IsValidPhone = r.read_int32(bytes)?,
                Ok(104) => msg.IsConfirmed = r.read_int32(bytes)?,
                Ok(112) => msg.IsLogin = r.read_int32(bytes)?,
                Ok(120) => msg.IsRegister = r.read_int32(bytes)?,
                Ok(128) => msg.RetriedCount = r.read_int32(bytes)?,
                Ok(136) => msg.TTL = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Sms {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.Hash == String::default() { 0 } else { 1 + sizeof_len((&self.Hash).len()) }
        + if self.AppUuid == String::default() { 0 } else { 1 + sizeof_len((&self.AppUuid).len()) }
        + if self.ClientPhone == String::default() { 0 } else { 1 + sizeof_len((&self.ClientPhone).len()) }
        + if self.GenratedCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.GenratedCode) as u64) }
        + if self.SmsSenderNumber == String::default() { 0 } else { 1 + sizeof_len((&self.SmsSenderNumber).len()) }
        + if self.SmsSendStatues == String::default() { 0 } else { 1 + sizeof_len((&self.SmsSendStatues).len()) }
        + if self.SmsHttpBody == String::default() { 0 } else { 1 + sizeof_len((&self.SmsHttpBody).len()) }
        + if self.Err_pb == String::default() { 0 } else { 1 + sizeof_len((&self.Err_pb).len()) }
        + if self.Carrier == String::default() { 0 } else { 1 + sizeof_len((&self.Carrier).len()) }
        + if self.Country == vec![] { 0 } else { 1 + sizeof_len((&self.Country).len()) }
        + if self.IsValidPhone == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.IsValidPhone) as u64) }
        + if self.IsConfirmed == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.IsConfirmed) as u64) }
        + if self.IsLogin == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.IsLogin) as u64) }
        + if self.IsRegister == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.IsRegister) as u64) }
        + if self.RetriedCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.RetriedCount) as u64) }
        + if self.TTL == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.TTL) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.Id))?; }
        if self.Hash != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Hash))?; }
        if self.AppUuid != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.AppUuid))?; }
        if self.ClientPhone != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.ClientPhone))?; }
        if self.GenratedCode != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.GenratedCode))?; }
        if self.SmsSenderNumber != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.SmsSenderNumber))?; }
        if self.SmsSendStatues != String::default() { w.write_with_tag(58, |w| w.write_string(&**&self.SmsSendStatues))?; }
        if self.SmsHttpBody != String::default() { w.write_with_tag(66, |w| w.write_string(&**&self.SmsHttpBody))?; }
        if self.Err_pb != String::default() { w.write_with_tag(74, |w| w.write_string(&**&self.Err_pb))?; }
        if self.Carrier != String::default() { w.write_with_tag(82, |w| w.write_string(&**&self.Carrier))?; }
        if self.Country != vec![] { w.write_with_tag(90, |w| w.write_bytes(&**&self.Country))?; }
        if self.IsValidPhone != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.IsValidPhone))?; }
        if self.IsConfirmed != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.IsConfirmed))?; }
        if self.IsLogin != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.IsLogin))?; }
        if self.IsRegister != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.IsRegister))?; }
        if self.RetriedCount != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.RetriedCount))?; }
        if self.TTL != 0i32 { w.write_with_tag(136, |w| w.write_int32(*&self.TTL))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Tag {
    pub TagId: i64,
    pub Name: String,
    pub Count: i32,
    pub TagStatusEnum: i32,
    pub IsBlocked: i32,
    pub GroupId: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_Tag {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.TagId = r.read_int64(bytes)?,
                Ok(18) => msg.Name = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.Count = r.read_int32(bytes)?,
                Ok(32) => msg.TagStatusEnum = r.read_int32(bytes)?,
                Ok(40) => msg.IsBlocked = r.read_int32(bytes)?,
                Ok(48) => msg.GroupId = r.read_int32(bytes)?,
                Ok(56) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Tag {
    fn get_size(&self) -> usize {
        0
        + if self.TagId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.TagId) as u64) }
        + if self.Name == String::default() { 0 } else { 1 + sizeof_len((&self.Name).len()) }
        + if self.Count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Count) as u64) }
        + if self.TagStatusEnum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.TagStatusEnum) as u64) }
        + if self.IsBlocked == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.IsBlocked) as u64) }
        + if self.GroupId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.GroupId) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.TagId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.TagId))?; }
        if self.Name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Name))?; }
        if self.Count != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.Count))?; }
        if self.TagStatusEnum != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.TagStatusEnum))?; }
        if self.IsBlocked != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.IsBlocked))?; }
        if self.GroupId != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.GroupId))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_TagPost {
    pub Id: i64,
    pub TagId: i32,
    pub PostId: i32,
    pub PostType: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_TagPost {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.TagId = r.read_int32(bytes)?,
                Ok(24) => msg.PostId = r.read_int32(bytes)?,
                Ok(32) => msg.PostType = r.read_int32(bytes)?,
                Ok(40) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_TagPost {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.TagId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.TagId) as u64) }
        + if self.PostId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.PostType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PostType) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.TagId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.TagId))?; }
        if self.PostId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.PostId))?; }
        if self.PostType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.PostType))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_TriggerLog {
    pub Id: i64,
    pub ModelName: String,
    pub ChangeType: String,
    pub TargetId: i64,
    pub TargetStr: String,
    pub CreatedSe: i32,
}

impl<'a> MessageRead<'a> for PB_TriggerLog {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(18) => msg.ModelName = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.ChangeType = r.read_string(bytes)?.to_owned(),
                Ok(32) => msg.TargetId = r.read_int64(bytes)?,
                Ok(42) => msg.TargetStr = r.read_string(bytes)?.to_owned(),
                Ok(48) => msg.CreatedSe = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_TriggerLog {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.ModelName == String::default() { 0 } else { 1 + sizeof_len((&self.ModelName).len()) }
        + if self.ChangeType == String::default() { 0 } else { 1 + sizeof_len((&self.ChangeType).len()) }
        + if self.TargetId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.TargetId) as u64) }
        + if self.TargetStr == String::default() { 0 } else { 1 + sizeof_len((&self.TargetStr).len()) }
        + if self.CreatedSe == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedSe) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.ModelName != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.ModelName))?; }
        if self.ChangeType != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.ChangeType))?; }
        if self.TargetId != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.TargetId))?; }
        if self.TargetStr != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.TargetStr))?; }
        if self.CreatedSe != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.CreatedSe))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_User {
    pub UserId: i32,
    pub UserName: String,
    pub UserNameLower: String,
    pub FirstName: String,
    pub LastName: String,
    pub IsVerified: i32,
    pub AvatarId: i64,
    pub AccessHash: i32,
    pub ProfilePrivacy: i32,
    pub OnlinePrivacy: i32,
    pub CallPrivacy: i32,
    pub AddToGroupPrivacy: i32,
    pub SeenMessagePrivacy: i32,
    pub Phone: String,
    pub Email: String,
    pub About: String,
    pub PasswordHash: String,
    pub PasswordSalt: String,
    pub PostSeq: i32,
    pub FollowersCount: i32,
    pub FollowingCount: i32,
    pub PostsCount: i32,
    pub MediaCount: i32,
    pub PhotoCount: i32,
    pub VideoCount: i32,
    pub GifCount: i32,
    pub AudioCount: i32,
    pub VoiceCount: i32,
    pub FileCount: i32,
    pub LinkCount: i32,
    pub BoardCount: i32,
    pub PinedCount: i32,
    pub LikesCount: i32,
    pub ResharedCount: i32,
    pub LastPostTime: i32,
    pub CreatedTime: i32,
    pub VersionTime: i32,
    pub IsDeleted: i32,
    pub IsBanned: i32,
}

impl<'a> MessageRead<'a> for PB_User {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.UserId = r.read_int32(bytes)?,
                Ok(18) => msg.UserName = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.UserNameLower = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.FirstName = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.LastName = r.read_string(bytes)?.to_owned(),
                Ok(48) => msg.IsVerified = r.read_int32(bytes)?,
                Ok(56) => msg.AvatarId = r.read_int64(bytes)?,
                Ok(64) => msg.AccessHash = r.read_int32(bytes)?,
                Ok(72) => msg.ProfilePrivacy = r.read_int32(bytes)?,
                Ok(80) => msg.OnlinePrivacy = r.read_int32(bytes)?,
                Ok(88) => msg.CallPrivacy = r.read_int32(bytes)?,
                Ok(96) => msg.AddToGroupPrivacy = r.read_int32(bytes)?,
                Ok(104) => msg.SeenMessagePrivacy = r.read_int32(bytes)?,
                Ok(114) => msg.Phone = r.read_string(bytes)?.to_owned(),
                Ok(122) => msg.Email = r.read_string(bytes)?.to_owned(),
                Ok(130) => msg.About = r.read_string(bytes)?.to_owned(),
                Ok(138) => msg.PasswordHash = r.read_string(bytes)?.to_owned(),
                Ok(146) => msg.PasswordSalt = r.read_string(bytes)?.to_owned(),
                Ok(152) => msg.PostSeq = r.read_int32(bytes)?,
                Ok(160) => msg.FollowersCount = r.read_int32(bytes)?,
                Ok(168) => msg.FollowingCount = r.read_int32(bytes)?,
                Ok(176) => msg.PostsCount = r.read_int32(bytes)?,
                Ok(184) => msg.MediaCount = r.read_int32(bytes)?,
                Ok(192) => msg.PhotoCount = r.read_int32(bytes)?,
                Ok(200) => msg.VideoCount = r.read_int32(bytes)?,
                Ok(208) => msg.GifCount = r.read_int32(bytes)?,
                Ok(216) => msg.AudioCount = r.read_int32(bytes)?,
                Ok(224) => msg.VoiceCount = r.read_int32(bytes)?,
                Ok(232) => msg.FileCount = r.read_int32(bytes)?,
                Ok(240) => msg.LinkCount = r.read_int32(bytes)?,
                Ok(248) => msg.BoardCount = r.read_int32(bytes)?,
                Ok(256) => msg.PinedCount = r.read_int32(bytes)?,
                Ok(264) => msg.LikesCount = r.read_int32(bytes)?,
                Ok(272) => msg.ResharedCount = r.read_int32(bytes)?,
                Ok(280) => msg.LastPostTime = r.read_int32(bytes)?,
                Ok(288) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(296) => msg.VersionTime = r.read_int32(bytes)?,
                Ok(304) => msg.IsDeleted = r.read_int32(bytes)?,
                Ok(312) => msg.IsBanned = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_User {
    fn get_size(&self) -> usize {
        0
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.UserName == String::default() { 0 } else { 1 + sizeof_len((&self.UserName).len()) }
        + if self.UserNameLower == String::default() { 0 } else { 1 + sizeof_len((&self.UserNameLower).len()) }
        + if self.FirstName == String::default() { 0 } else { 1 + sizeof_len((&self.FirstName).len()) }
        + if self.LastName == String::default() { 0 } else { 1 + sizeof_len((&self.LastName).len()) }
        + if self.IsVerified == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.IsVerified) as u64) }
        + if self.AvatarId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.AvatarId) as u64) }
        + if self.AccessHash == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.AccessHash) as u64) }
        + if self.ProfilePrivacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ProfilePrivacy) as u64) }
        + if self.OnlinePrivacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.OnlinePrivacy) as u64) }
        + if self.CallPrivacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CallPrivacy) as u64) }
        + if self.AddToGroupPrivacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.AddToGroupPrivacy) as u64) }
        + if self.SeenMessagePrivacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.SeenMessagePrivacy) as u64) }
        + if self.Phone == String::default() { 0 } else { 1 + sizeof_len((&self.Phone).len()) }
        + if self.Email == String::default() { 0 } else { 1 + sizeof_len((&self.Email).len()) }
        + if self.About == String::default() { 0 } else { 2 + sizeof_len((&self.About).len()) }
        + if self.PasswordHash == String::default() { 0 } else { 2 + sizeof_len((&self.PasswordHash).len()) }
        + if self.PasswordSalt == String::default() { 0 } else { 2 + sizeof_len((&self.PasswordSalt).len()) }
        + if self.PostSeq == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.PostSeq) as u64) }
        + if self.FollowersCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.FollowersCount) as u64) }
        + if self.FollowingCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.FollowingCount) as u64) }
        + if self.PostsCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.PostsCount) as u64) }
        + if self.MediaCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.MediaCount) as u64) }
        + if self.PhotoCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.PhotoCount) as u64) }
        + if self.VideoCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.VideoCount) as u64) }
        + if self.GifCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.GifCount) as u64) }
        + if self.AudioCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.AudioCount) as u64) }
        + if self.VoiceCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.VoiceCount) as u64) }
        + if self.FileCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.FileCount) as u64) }
        + if self.LinkCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.LinkCount) as u64) }
        + if self.BoardCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.BoardCount) as u64) }
        + if self.PinedCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.PinedCount) as u64) }
        + if self.LikesCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.LikesCount) as u64) }
        + if self.ResharedCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.ResharedCount) as u64) }
        + if self.LastPostTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.LastPostTime) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.CreatedTime) as u64) }
        + if self.VersionTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.VersionTime) as u64) }
        + if self.IsDeleted == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsDeleted) as u64) }
        + if self.IsBanned == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsBanned) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserId != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.UserId))?; }
        if self.UserName != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.UserName))?; }
        if self.UserNameLower != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.UserNameLower))?; }
        if self.FirstName != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.FirstName))?; }
        if self.LastName != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.LastName))?; }
        if self.IsVerified != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.IsVerified))?; }
        if self.AvatarId != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.AvatarId))?; }
        if self.AccessHash != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.AccessHash))?; }
        if self.ProfilePrivacy != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.ProfilePrivacy))?; }
        if self.OnlinePrivacy != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.OnlinePrivacy))?; }
        if self.CallPrivacy != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.CallPrivacy))?; }
        if self.AddToGroupPrivacy != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.AddToGroupPrivacy))?; }
        if self.SeenMessagePrivacy != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.SeenMessagePrivacy))?; }
        if self.Phone != String::default() { w.write_with_tag(114, |w| w.write_string(&**&self.Phone))?; }
        if self.Email != String::default() { w.write_with_tag(122, |w| w.write_string(&**&self.Email))?; }
        if self.About != String::default() { w.write_with_tag(130, |w| w.write_string(&**&self.About))?; }
        if self.PasswordHash != String::default() { w.write_with_tag(138, |w| w.write_string(&**&self.PasswordHash))?; }
        if self.PasswordSalt != String::default() { w.write_with_tag(146, |w| w.write_string(&**&self.PasswordSalt))?; }
        if self.PostSeq != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.PostSeq))?; }
        if self.FollowersCount != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.FollowersCount))?; }
        if self.FollowingCount != 0i32 { w.write_with_tag(168, |w| w.write_int32(*&self.FollowingCount))?; }
        if self.PostsCount != 0i32 { w.write_with_tag(176, |w| w.write_int32(*&self.PostsCount))?; }
        if self.MediaCount != 0i32 { w.write_with_tag(184, |w| w.write_int32(*&self.MediaCount))?; }
        if self.PhotoCount != 0i32 { w.write_with_tag(192, |w| w.write_int32(*&self.PhotoCount))?; }
        if self.VideoCount != 0i32 { w.write_with_tag(200, |w| w.write_int32(*&self.VideoCount))?; }
        if self.GifCount != 0i32 { w.write_with_tag(208, |w| w.write_int32(*&self.GifCount))?; }
        if self.AudioCount != 0i32 { w.write_with_tag(216, |w| w.write_int32(*&self.AudioCount))?; }
        if self.VoiceCount != 0i32 { w.write_with_tag(224, |w| w.write_int32(*&self.VoiceCount))?; }
        if self.FileCount != 0i32 { w.write_with_tag(232, |w| w.write_int32(*&self.FileCount))?; }
        if self.LinkCount != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.LinkCount))?; }
        if self.BoardCount != 0i32 { w.write_with_tag(248, |w| w.write_int32(*&self.BoardCount))?; }
        if self.PinedCount != 0i32 { w.write_with_tag(256, |w| w.write_int32(*&self.PinedCount))?; }
        if self.LikesCount != 0i32 { w.write_with_tag(264, |w| w.write_int32(*&self.LikesCount))?; }
        if self.ResharedCount != 0i32 { w.write_with_tag(272, |w| w.write_int32(*&self.ResharedCount))?; }
        if self.LastPostTime != 0i32 { w.write_with_tag(280, |w| w.write_int32(*&self.LastPostTime))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(288, |w| w.write_int32(*&self.CreatedTime))?; }
        if self.VersionTime != 0i32 { w.write_with_tag(296, |w| w.write_int32(*&self.VersionTime))?; }
        if self.IsDeleted != 0i32 { w.write_with_tag(304, |w| w.write_int32(*&self.IsDeleted))?; }
        if self.IsBanned != 0i32 { w.write_with_tag(312, |w| w.write_int32(*&self.IsBanned))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_UserRelation {
    pub RelNanoId: i64,
    pub UserId: i32,
    pub PeerUserId: i32,
    pub Follwing: i32,
    pub Followed: i32,
    pub InContacts: i32,
    pub MutualContact: i32,
    pub IsFavorite: i32,
    pub Notify: i32,
}

impl<'a> MessageRead<'a> for PB_UserRelation {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.RelNanoId = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(24) => msg.PeerUserId = r.read_int32(bytes)?,
                Ok(32) => msg.Follwing = r.read_int32(bytes)?,
                Ok(40) => msg.Followed = r.read_int32(bytes)?,
                Ok(48) => msg.InContacts = r.read_int32(bytes)?,
                Ok(56) => msg.MutualContact = r.read_int32(bytes)?,
                Ok(64) => msg.IsFavorite = r.read_int32(bytes)?,
                Ok(72) => msg.Notify = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_UserRelation {
    fn get_size(&self) -> usize {
        0
        + if self.RelNanoId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.RelNanoId) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.PeerUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PeerUserId) as u64) }
        + if self.Follwing == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Follwing) as u64) }
        + if self.Followed == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Followed) as u64) }
        + if self.InContacts == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.InContacts) as u64) }
        + if self.MutualContact == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.MutualContact) as u64) }
        + if self.IsFavorite == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.IsFavorite) as u64) }
        + if self.Notify == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Notify) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.RelNanoId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.RelNanoId))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        if self.PeerUserId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.PeerUserId))?; }
        if self.Follwing != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.Follwing))?; }
        if self.Followed != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.Followed))?; }
        if self.InContacts != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.InContacts))?; }
        if self.MutualContact != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.MutualContact))?; }
        if self.IsFavorite != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.IsFavorite))?; }
        if self.Notify != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.Notify))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Chat {
    pub ChatId: i64,
    pub ChatKey: String,
    pub RoomKey: String,
    pub RoomType: i32,
    pub UserId: i32,
    pub PeerUserId: i32,
    pub GroupId: i64,
    pub HashTagId: i64,
    pub Title: String,
    pub PinTimeMs: i64,
    pub FromMsgId: i64,
    pub UnseenCount: i32,
    pub Seq: i32,
    pub LastMsgId: i64,
    pub LastMyMsgStatus: i32,
    pub MyLastSeenSeq: i32,
    pub MyLastSeenMsgId: i64,
    pub PeerLastSeenMsgId: i64,
    pub MyLastDeliveredSeq: i32,
    pub MyLastDeliveredMsgId: i64,
    pub PeerLastDeliveredMsgId: i64,
    pub IsActive: i32,
    pub IsLeft: i32,
    pub IsCreator: i32,
    pub IsKicked: i32,
    pub IsAdmin: i32,
    pub IsDeactivated: i32,
    pub IsMuted: i32,
    pub MuteUntil: i32,
    pub VersionTimeMs: i64,
    pub VersionSeq: i32,
    pub OrderTime: i32,
    pub CreatedTime: i32,
    pub DraftText: String,
    pub DratReplyToMsgId: i64,
}

impl<'a> MessageRead<'a> for PB_Chat {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ChatId = r.read_int64(bytes)?,
                Ok(18) => msg.ChatKey = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.RoomKey = r.read_string(bytes)?.to_owned(),
                Ok(32) => msg.RoomType = r.read_int32(bytes)?,
                Ok(40) => msg.UserId = r.read_int32(bytes)?,
                Ok(48) => msg.PeerUserId = r.read_int32(bytes)?,
                Ok(56) => msg.GroupId = r.read_int64(bytes)?,
                Ok(64) => msg.HashTagId = r.read_int64(bytes)?,
                Ok(74) => msg.Title = r.read_string(bytes)?.to_owned(),
                Ok(80) => msg.PinTimeMs = r.read_int64(bytes)?,
                Ok(88) => msg.FromMsgId = r.read_int64(bytes)?,
                Ok(96) => msg.UnseenCount = r.read_int32(bytes)?,
                Ok(104) => msg.Seq = r.read_int32(bytes)?,
                Ok(112) => msg.LastMsgId = r.read_int64(bytes)?,
                Ok(120) => msg.LastMyMsgStatus = r.read_int32(bytes)?,
                Ok(128) => msg.MyLastSeenSeq = r.read_int32(bytes)?,
                Ok(136) => msg.MyLastSeenMsgId = r.read_int64(bytes)?,
                Ok(144) => msg.PeerLastSeenMsgId = r.read_int64(bytes)?,
                Ok(152) => msg.MyLastDeliveredSeq = r.read_int32(bytes)?,
                Ok(160) => msg.MyLastDeliveredMsgId = r.read_int64(bytes)?,
                Ok(168) => msg.PeerLastDeliveredMsgId = r.read_int64(bytes)?,
                Ok(176) => msg.IsActive = r.read_int32(bytes)?,
                Ok(184) => msg.IsLeft = r.read_int32(bytes)?,
                Ok(192) => msg.IsCreator = r.read_int32(bytes)?,
                Ok(200) => msg.IsKicked = r.read_int32(bytes)?,
                Ok(208) => msg.IsAdmin = r.read_int32(bytes)?,
                Ok(216) => msg.IsDeactivated = r.read_int32(bytes)?,
                Ok(224) => msg.IsMuted = r.read_int32(bytes)?,
                Ok(232) => msg.MuteUntil = r.read_int32(bytes)?,
                Ok(240) => msg.VersionTimeMs = r.read_int64(bytes)?,
                Ok(248) => msg.VersionSeq = r.read_int32(bytes)?,
                Ok(256) => msg.OrderTime = r.read_int32(bytes)?,
                Ok(264) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(274) => msg.DraftText = r.read_string(bytes)?.to_owned(),
                Ok(280) => msg.DratReplyToMsgId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Chat {
    fn get_size(&self) -> usize {
        0
        + if self.ChatId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ChatId) as u64) }
        + if self.ChatKey == String::default() { 0 } else { 1 + sizeof_len((&self.ChatKey).len()) }
        + if self.RoomKey == String::default() { 0 } else { 1 + sizeof_len((&self.RoomKey).len()) }
        + if self.RoomType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.RoomType) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.PeerUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PeerUserId) as u64) }
        + if self.GroupId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.GroupId) as u64) }
        + if self.HashTagId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.HashTagId) as u64) }
        + if self.Title == String::default() { 0 } else { 1 + sizeof_len((&self.Title).len()) }
        + if self.PinTimeMs == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PinTimeMs) as u64) }
        + if self.FromMsgId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.FromMsgId) as u64) }
        + if self.UnseenCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UnseenCount) as u64) }
        + if self.Seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Seq) as u64) }
        + if self.LastMsgId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastMsgId) as u64) }
        + if self.LastMyMsgStatus == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.LastMyMsgStatus) as u64) }
        + if self.MyLastSeenSeq == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.MyLastSeenSeq) as u64) }
        + if self.MyLastSeenMsgId == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.MyLastSeenMsgId) as u64) }
        + if self.PeerLastSeenMsgId == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.PeerLastSeenMsgId) as u64) }
        + if self.MyLastDeliveredSeq == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.MyLastDeliveredSeq) as u64) }
        + if self.MyLastDeliveredMsgId == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.MyLastDeliveredMsgId) as u64) }
        + if self.PeerLastDeliveredMsgId == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.PeerLastDeliveredMsgId) as u64) }
        + if self.IsActive == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsActive) as u64) }
        + if self.IsLeft == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsLeft) as u64) }
        + if self.IsCreator == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsCreator) as u64) }
        + if self.IsKicked == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsKicked) as u64) }
        + if self.IsAdmin == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsAdmin) as u64) }
        + if self.IsDeactivated == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsDeactivated) as u64) }
        + if self.IsMuted == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsMuted) as u64) }
        + if self.MuteUntil == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.MuteUntil) as u64) }
        + if self.VersionTimeMs == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.VersionTimeMs) as u64) }
        + if self.VersionSeq == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.VersionSeq) as u64) }
        + if self.OrderTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.OrderTime) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.CreatedTime) as u64) }
        + if self.DraftText == String::default() { 0 } else { 2 + sizeof_len((&self.DraftText).len()) }
        + if self.DratReplyToMsgId == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.DratReplyToMsgId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ChatId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.ChatId))?; }
        if self.ChatKey != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.ChatKey))?; }
        if self.RoomKey != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.RoomKey))?; }
        if self.RoomType != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.RoomType))?; }
        if self.UserId != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.UserId))?; }
        if self.PeerUserId != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.PeerUserId))?; }
        if self.GroupId != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.GroupId))?; }
        if self.HashTagId != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.HashTagId))?; }
        if self.Title != String::default() { w.write_with_tag(74, |w| w.write_string(&**&self.Title))?; }
        if self.PinTimeMs != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.PinTimeMs))?; }
        if self.FromMsgId != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.FromMsgId))?; }
        if self.UnseenCount != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.UnseenCount))?; }
        if self.Seq != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.Seq))?; }
        if self.LastMsgId != 0i64 { w.write_with_tag(112, |w| w.write_int64(*&self.LastMsgId))?; }
        if self.LastMyMsgStatus != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.LastMyMsgStatus))?; }
        if self.MyLastSeenSeq != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.MyLastSeenSeq))?; }
        if self.MyLastSeenMsgId != 0i64 { w.write_with_tag(136, |w| w.write_int64(*&self.MyLastSeenMsgId))?; }
        if self.PeerLastSeenMsgId != 0i64 { w.write_with_tag(144, |w| w.write_int64(*&self.PeerLastSeenMsgId))?; }
        if self.MyLastDeliveredSeq != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.MyLastDeliveredSeq))?; }
        if self.MyLastDeliveredMsgId != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.MyLastDeliveredMsgId))?; }
        if self.PeerLastDeliveredMsgId != 0i64 { w.write_with_tag(168, |w| w.write_int64(*&self.PeerLastDeliveredMsgId))?; }
        if self.IsActive != 0i32 { w.write_with_tag(176, |w| w.write_int32(*&self.IsActive))?; }
        if self.IsLeft != 0i32 { w.write_with_tag(184, |w| w.write_int32(*&self.IsLeft))?; }
        if self.IsCreator != 0i32 { w.write_with_tag(192, |w| w.write_int32(*&self.IsCreator))?; }
        if self.IsKicked != 0i32 { w.write_with_tag(200, |w| w.write_int32(*&self.IsKicked))?; }
        if self.IsAdmin != 0i32 { w.write_with_tag(208, |w| w.write_int32(*&self.IsAdmin))?; }
        if self.IsDeactivated != 0i32 { w.write_with_tag(216, |w| w.write_int32(*&self.IsDeactivated))?; }
        if self.IsMuted != 0i32 { w.write_with_tag(224, |w| w.write_int32(*&self.IsMuted))?; }
        if self.MuteUntil != 0i32 { w.write_with_tag(232, |w| w.write_int32(*&self.MuteUntil))?; }
        if self.VersionTimeMs != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.VersionTimeMs))?; }
        if self.VersionSeq != 0i32 { w.write_with_tag(248, |w| w.write_int32(*&self.VersionSeq))?; }
        if self.OrderTime != 0i32 { w.write_with_tag(256, |w| w.write_int32(*&self.OrderTime))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(264, |w| w.write_int32(*&self.CreatedTime))?; }
        if self.DraftText != String::default() { w.write_with_tag(274, |w| w.write_string(&**&self.DraftText))?; }
        if self.DratReplyToMsgId != 0i64 { w.write_with_tag(280, |w| w.write_int64(*&self.DratReplyToMsgId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_ChatDeleted {
    pub ChatId: i64,
    pub RoomKey: String,
}

impl<'a> MessageRead<'a> for PB_ChatDeleted {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ChatId = r.read_int64(bytes)?,
                Ok(18) => msg.RoomKey = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_ChatDeleted {
    fn get_size(&self) -> usize {
        0
        + if self.ChatId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ChatId) as u64) }
        + if self.RoomKey == String::default() { 0 } else { 1 + sizeof_len((&self.RoomKey).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ChatId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.ChatId))?; }
        if self.RoomKey != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.RoomKey))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_ChatLastMessage {
    pub ChatIdGroupId: String,
    pub LastMsgPb: Vec<u8>,
}

impl<'a> MessageRead<'a> for PB_ChatLastMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ChatIdGroupId = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.LastMsgPb = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_ChatLastMessage {
    fn get_size(&self) -> usize {
        0
        + if self.ChatIdGroupId == String::default() { 0 } else { 1 + sizeof_len((&self.ChatIdGroupId).len()) }
        + if self.LastMsgPb == vec![] { 0 } else { 1 + sizeof_len((&self.LastMsgPb).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ChatIdGroupId != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.ChatIdGroupId))?; }
        if self.LastMsgPb != vec![] { w.write_with_tag(18, |w| w.write_bytes(&**&self.LastMsgPb))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_ChatUserVersion {
    pub UserId: i32,
    pub ChatId: i64,
    pub VersionTimeNano: i32,
}

impl<'a> MessageRead<'a> for PB_ChatUserVersion {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.UserId = r.read_int32(bytes)?,
                Ok(16) => msg.ChatId = r.read_int64(bytes)?,
                Ok(24) => msg.VersionTimeNano = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_ChatUserVersion {
    fn get_size(&self) -> usize {
        0
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.ChatId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ChatId) as u64) }
        + if self.VersionTimeNano == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.VersionTimeNano) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserId != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.UserId))?; }
        if self.ChatId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.ChatId))?; }
        if self.VersionTimeNano != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.VersionTimeNano))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Group {
    pub GroupId: i64,
    pub GroupKey: String,
    pub GroupName: String,
    pub UserName: String,
    pub IsSuperGroup: i32,
    pub HashTagId: i64,
    pub CreatorUserId: i32,
    pub GroupPrivacy: i32,
    pub HistoryViewAble: i32,
    pub Seq: i64,
    pub LastMsgId: i64,
    pub PinedMsgId: i64,
    pub AvatarRefId: i64,
    pub AvatarCount: i32,
    pub About: String,
    pub InviteLinkHash: String,
    pub MembersCount: i32,
    pub AdminsCount: i32,
    pub ModeratorCounts: i32,
    pub SortTime: i32,
    pub CreatedTime: i32,
    pub IsMute: i32,
    pub IsDeleted: i32,
    pub IsBanned: i32,
}

impl<'a> MessageRead<'a> for PB_Group {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.GroupId = r.read_int64(bytes)?,
                Ok(18) => msg.GroupKey = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.GroupName = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.UserName = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.IsSuperGroup = r.read_int32(bytes)?,
                Ok(48) => msg.HashTagId = r.read_int64(bytes)?,
                Ok(56) => msg.CreatorUserId = r.read_int32(bytes)?,
                Ok(64) => msg.GroupPrivacy = r.read_int32(bytes)?,
                Ok(72) => msg.HistoryViewAble = r.read_int32(bytes)?,
                Ok(80) => msg.Seq = r.read_int64(bytes)?,
                Ok(88) => msg.LastMsgId = r.read_int64(bytes)?,
                Ok(96) => msg.PinedMsgId = r.read_int64(bytes)?,
                Ok(104) => msg.AvatarRefId = r.read_int64(bytes)?,
                Ok(112) => msg.AvatarCount = r.read_int32(bytes)?,
                Ok(122) => msg.About = r.read_string(bytes)?.to_owned(),
                Ok(130) => msg.InviteLinkHash = r.read_string(bytes)?.to_owned(),
                Ok(136) => msg.MembersCount = r.read_int32(bytes)?,
                Ok(144) => msg.AdminsCount = r.read_int32(bytes)?,
                Ok(152) => msg.ModeratorCounts = r.read_int32(bytes)?,
                Ok(160) => msg.SortTime = r.read_int32(bytes)?,
                Ok(168) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(176) => msg.IsMute = r.read_int32(bytes)?,
                Ok(184) => msg.IsDeleted = r.read_int32(bytes)?,
                Ok(192) => msg.IsBanned = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_Group {
    fn get_size(&self) -> usize {
        0
        + if self.GroupId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.GroupId) as u64) }
        + if self.GroupKey == String::default() { 0 } else { 1 + sizeof_len((&self.GroupKey).len()) }
        + if self.GroupName == String::default() { 0 } else { 1 + sizeof_len((&self.GroupName).len()) }
        + if self.UserName == String::default() { 0 } else { 1 + sizeof_len((&self.UserName).len()) }
        + if self.IsSuperGroup == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.IsSuperGroup) as u64) }
        + if self.HashTagId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.HashTagId) as u64) }
        + if self.CreatorUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatorUserId) as u64) }
        + if self.GroupPrivacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.GroupPrivacy) as u64) }
        + if self.HistoryViewAble == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.HistoryViewAble) as u64) }
        + if self.Seq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Seq) as u64) }
        + if self.LastMsgId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastMsgId) as u64) }
        + if self.PinedMsgId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PinedMsgId) as u64) }
        + if self.AvatarRefId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.AvatarRefId) as u64) }
        + if self.AvatarCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.AvatarCount) as u64) }
        + if self.About == String::default() { 0 } else { 1 + sizeof_len((&self.About).len()) }
        + if self.InviteLinkHash == String::default() { 0 } else { 2 + sizeof_len((&self.InviteLinkHash).len()) }
        + if self.MembersCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.MembersCount) as u64) }
        + if self.AdminsCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.AdminsCount) as u64) }
        + if self.ModeratorCounts == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.ModeratorCounts) as u64) }
        + if self.SortTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.SortTime) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.CreatedTime) as u64) }
        + if self.IsMute == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsMute) as u64) }
        + if self.IsDeleted == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsDeleted) as u64) }
        + if self.IsBanned == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsBanned) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.GroupId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.GroupId))?; }
        if self.GroupKey != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.GroupKey))?; }
        if self.GroupName != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.GroupName))?; }
        if self.UserName != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.UserName))?; }
        if self.IsSuperGroup != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.IsSuperGroup))?; }
        if self.HashTagId != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.HashTagId))?; }
        if self.CreatorUserId != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.CreatorUserId))?; }
        if self.GroupPrivacy != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.GroupPrivacy))?; }
        if self.HistoryViewAble != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.HistoryViewAble))?; }
        if self.Seq != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.Seq))?; }
        if self.LastMsgId != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.LastMsgId))?; }
        if self.PinedMsgId != 0i64 { w.write_with_tag(96, |w| w.write_int64(*&self.PinedMsgId))?; }
        if self.AvatarRefId != 0i64 { w.write_with_tag(104, |w| w.write_int64(*&self.AvatarRefId))?; }
        if self.AvatarCount != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.AvatarCount))?; }
        if self.About != String::default() { w.write_with_tag(122, |w| w.write_string(&**&self.About))?; }
        if self.InviteLinkHash != String::default() { w.write_with_tag(130, |w| w.write_string(&**&self.InviteLinkHash))?; }
        if self.MembersCount != 0i32 { w.write_with_tag(136, |w| w.write_int32(*&self.MembersCount))?; }
        if self.AdminsCount != 0i32 { w.write_with_tag(144, |w| w.write_int32(*&self.AdminsCount))?; }
        if self.ModeratorCounts != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.ModeratorCounts))?; }
        if self.SortTime != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.SortTime))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(168, |w| w.write_int32(*&self.CreatedTime))?; }
        if self.IsMute != 0i32 { w.write_with_tag(176, |w| w.write_int32(*&self.IsMute))?; }
        if self.IsDeleted != 0i32 { w.write_with_tag(184, |w| w.write_int32(*&self.IsDeleted))?; }
        if self.IsBanned != 0i32 { w.write_with_tag(192, |w| w.write_int32(*&self.IsBanned))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_GroupMember {
    pub OrderId: i64,
    pub GroupId: i64,
    pub UserId: i32,
    pub ByUserId: i32,
    pub GroupRole: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_GroupMember {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.OrderId = r.read_int64(bytes)?,
                Ok(16) => msg.GroupId = r.read_int64(bytes)?,
                Ok(24) => msg.UserId = r.read_int32(bytes)?,
                Ok(32) => msg.ByUserId = r.read_int32(bytes)?,
                Ok(40) => msg.GroupRole = r.read_int32(bytes)?,
                Ok(48) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_GroupMember {
    fn get_size(&self) -> usize {
        0
        + if self.OrderId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.OrderId) as u64) }
        + if self.GroupId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.GroupId) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.ByUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ByUserId) as u64) }
        + if self.GroupRole == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.GroupRole) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.OrderId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.OrderId))?; }
        if self.GroupId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.GroupId))?; }
        if self.UserId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.UserId))?; }
        if self.ByUserId != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.ByUserId))?; }
        if self.GroupRole != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.GroupRole))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_GroupOrderdUser {
    pub OrderId: i64,
    pub GroupId: i64,
    pub UserId: i32,
}

impl<'a> MessageRead<'a> for PB_GroupOrderdUser {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.OrderId = r.read_int64(bytes)?,
                Ok(16) => msg.GroupId = r.read_int64(bytes)?,
                Ok(24) => msg.UserId = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_GroupOrderdUser {
    fn get_size(&self) -> usize {
        0
        + if self.OrderId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.OrderId) as u64) }
        + if self.GroupId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.GroupId) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.OrderId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.OrderId))?; }
        if self.GroupId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.GroupId))?; }
        if self.UserId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.UserId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_GroupPinedMsg {
    pub MsgId: i64,
    pub MsgPb: Vec<u8>,
}

impl<'a> MessageRead<'a> for PB_GroupPinedMsg {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.MsgId = r.read_int64(bytes)?,
                Ok(18) => msg.MsgPb = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_GroupPinedMsg {
    fn get_size(&self) -> usize {
        0
        + if self.MsgId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.MsgId) as u64) }
        + if self.MsgPb == vec![] { 0 } else { 1 + sizeof_len((&self.MsgPb).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.MsgId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.MsgId))?; }
        if self.MsgPb != vec![] { w.write_with_tag(18, |w| w.write_bytes(&**&self.MsgPb))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_FileMsg {
    pub Id: i64,
    pub AccessHash: i32,
    pub FileType: i32,
    pub Width: i32,
    pub Height: i32,
    pub Extension: String,
    pub UserId: i32,
    pub DataThumb: Vec<u8>,
    pub Data: Vec<u8>,
}

impl<'a> MessageRead<'a> for PB_FileMsg {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.AccessHash = r.read_int32(bytes)?,
                Ok(24) => msg.FileType = r.read_int32(bytes)?,
                Ok(32) => msg.Width = r.read_int32(bytes)?,
                Ok(40) => msg.Height = r.read_int32(bytes)?,
                Ok(50) => msg.Extension = r.read_string(bytes)?.to_owned(),
                Ok(56) => msg.UserId = r.read_int32(bytes)?,
                Ok(66) => msg.DataThumb = r.read_bytes(bytes)?.to_owned(),
                Ok(74) => msg.Data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_FileMsg {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.AccessHash == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.AccessHash) as u64) }
        + if self.FileType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.FileType) as u64) }
        + if self.Width == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Width) as u64) }
        + if self.Height == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Height) as u64) }
        + if self.Extension == String::default() { 0 } else { 1 + sizeof_len((&self.Extension).len()) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.DataThumb == vec![] { 0 } else { 1 + sizeof_len((&self.DataThumb).len()) }
        + if self.Data == vec![] { 0 } else { 1 + sizeof_len((&self.Data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.AccessHash != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.AccessHash))?; }
        if self.FileType != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.FileType))?; }
        if self.Width != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.Width))?; }
        if self.Height != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.Height))?; }
        if self.Extension != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.Extension))?; }
        if self.UserId != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.UserId))?; }
        if self.DataThumb != vec![] { w.write_with_tag(66, |w| w.write_bytes(&**&self.DataThumb))?; }
        if self.Data != vec![] { w.write_with_tag(74, |w| w.write_bytes(&**&self.Data))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_FilePost {
    pub Id: i64,
    pub AccessHash: i32,
    pub FileType: i32,
    pub Width: i32,
    pub Height: i32,
    pub Extension: String,
    pub UserId: i32,
    pub DataThumb: Vec<u8>,
    pub Data: Vec<u8>,
}

impl<'a> MessageRead<'a> for PB_FilePost {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.AccessHash = r.read_int32(bytes)?,
                Ok(24) => msg.FileType = r.read_int32(bytes)?,
                Ok(32) => msg.Width = r.read_int32(bytes)?,
                Ok(40) => msg.Height = r.read_int32(bytes)?,
                Ok(50) => msg.Extension = r.read_string(bytes)?.to_owned(),
                Ok(56) => msg.UserId = r.read_int32(bytes)?,
                Ok(66) => msg.DataThumb = r.read_bytes(bytes)?.to_owned(),
                Ok(74) => msg.Data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_FilePost {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.AccessHash == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.AccessHash) as u64) }
        + if self.FileType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.FileType) as u64) }
        + if self.Width == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Width) as u64) }
        + if self.Height == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Height) as u64) }
        + if self.Extension == String::default() { 0 } else { 1 + sizeof_len((&self.Extension).len()) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.DataThumb == vec![] { 0 } else { 1 + sizeof_len((&self.DataThumb).len()) }
        + if self.Data == vec![] { 0 } else { 1 + sizeof_len((&self.Data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.AccessHash != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.AccessHash))?; }
        if self.FileType != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.FileType))?; }
        if self.Width != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.Width))?; }
        if self.Height != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.Height))?; }
        if self.Extension != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.Extension))?; }
        if self.UserId != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.UserId))?; }
        if self.DataThumb != vec![] { w.write_with_tag(66, |w| w.write_bytes(&**&self.DataThumb))?; }
        if self.Data != vec![] { w.write_with_tag(74, |w| w.write_bytes(&**&self.Data))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_ActionFanout {
    pub OrderId: i64,
    pub ForUserId: i32,
    pub ActionId: i64,
    pub ActorUserId: i32,
}

impl<'a> MessageRead<'a> for PB_ActionFanout {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.OrderId = r.read_int64(bytes)?,
                Ok(16) => msg.ForUserId = r.read_int32(bytes)?,
                Ok(24) => msg.ActionId = r.read_int64(bytes)?,
                Ok(32) => msg.ActorUserId = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_ActionFanout {
    fn get_size(&self) -> usize {
        0
        + if self.OrderId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.OrderId) as u64) }
        + if self.ForUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ForUserId) as u64) }
        + if self.ActionId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ActionId) as u64) }
        + if self.ActorUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ActorUserId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.OrderId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.OrderId))?; }
        if self.ForUserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.ForUserId))?; }
        if self.ActionId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.ActionId))?; }
        if self.ActorUserId != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.ActorUserId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_HomeFanout {
    pub OrderId: i64,
    pub ForUserId: i64,
    pub PostId: i64,
    pub PostUserId: i64,
    pub ResharedId: i64,
}

impl<'a> MessageRead<'a> for PB_HomeFanout {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.OrderId = r.read_int64(bytes)?,
                Ok(16) => msg.ForUserId = r.read_int64(bytes)?,
                Ok(24) => msg.PostId = r.read_int64(bytes)?,
                Ok(32) => msg.PostUserId = r.read_int64(bytes)?,
                Ok(40) => msg.ResharedId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_HomeFanout {
    fn get_size(&self) -> usize {
        0
        + if self.OrderId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.OrderId) as u64) }
        + if self.ForUserId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ForUserId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.PostUserId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostUserId) as u64) }
        + if self.ResharedId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ResharedId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.OrderId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.OrderId))?; }
        if self.ForUserId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.ForUserId))?; }
        if self.PostId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.PostId))?; }
        if self.PostUserId != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.PostUserId))?; }
        if self.ResharedId != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.ResharedId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_SuggestedTopPosts {
    pub Id: i64,
    pub PostId: i64,
}

impl<'a> MessageRead<'a> for PB_SuggestedTopPosts {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.PostId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_SuggestedTopPosts {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.PostId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.PostId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_SuggestedUser {
    pub Id: i32,
    pub UserId: i32,
    pub TargetId: i32,
    pub Weight: f32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_SuggestedUser {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int32(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(24) => msg.TargetId = r.read_int32(bytes)?,
                Ok(37) => msg.Weight = r.read_float(bytes)?,
                Ok(40) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_SuggestedUser {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.TargetId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.TargetId) as u64) }
        + if self.Weight == 0f32 { 0 } else { 1 + 4 }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.Id))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        if self.TargetId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.TargetId))?; }
        if self.Weight != 0f32 { w.write_with_tag(37, |w| w.write_float(*&self.Weight))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_PushChat {
    pub PushId: i64,
    pub ToUserId: i32,
    pub PushTypeId: i32,
    pub RoomKey: String,
    pub ChatKey: String,
    pub Seq: i32,
    pub UnseenCount: i32,
    pub FromHighMessageId: i64,
    pub ToLowMessageId: i64,
    pub MessageId: i64,
    pub MessageFileId: i64,
    pub MessagePb: Vec<u8>,
    pub MessageJson: String,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for PB_PushChat {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PushId = r.read_int64(bytes)?,
                Ok(16) => msg.ToUserId = r.read_int32(bytes)?,
                Ok(24) => msg.PushTypeId = r.read_int32(bytes)?,
                Ok(34) => msg.RoomKey = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.ChatKey = r.read_string(bytes)?.to_owned(),
                Ok(48) => msg.Seq = r.read_int32(bytes)?,
                Ok(56) => msg.UnseenCount = r.read_int32(bytes)?,
                Ok(64) => msg.FromHighMessageId = r.read_int64(bytes)?,
                Ok(72) => msg.ToLowMessageId = r.read_int64(bytes)?,
                Ok(80) => msg.MessageId = r.read_int64(bytes)?,
                Ok(88) => msg.MessageFileId = r.read_int64(bytes)?,
                Ok(98) => msg.MessagePb = r.read_bytes(bytes)?.to_owned(),
                Ok(106) => msg.MessageJson = r.read_string(bytes)?.to_owned(),
                Ok(112) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_PushChat {
    fn get_size(&self) -> usize {
        0
        + if self.PushId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PushId) as u64) }
        + if self.ToUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ToUserId) as u64) }
        + if self.PushTypeId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PushTypeId) as u64) }
        + if self.RoomKey == String::default() { 0 } else { 1 + sizeof_len((&self.RoomKey).len()) }
        + if self.ChatKey == String::default() { 0 } else { 1 + sizeof_len((&self.ChatKey).len()) }
        + if self.Seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Seq) as u64) }
        + if self.UnseenCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UnseenCount) as u64) }
        + if self.FromHighMessageId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.FromHighMessageId) as u64) }
        + if self.ToLowMessageId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ToLowMessageId) as u64) }
        + if self.MessageId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.MessageId) as u64) }
        + if self.MessageFileId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.MessageFileId) as u64) }
        + if self.MessagePb == vec![] { 0 } else { 1 + sizeof_len((&self.MessagePb).len()) }
        + if self.MessageJson == String::default() { 0 } else { 1 + sizeof_len((&self.MessageJson).len()) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PushId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PushId))?; }
        if self.ToUserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.ToUserId))?; }
        if self.PushTypeId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.PushTypeId))?; }
        if self.RoomKey != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.RoomKey))?; }
        if self.ChatKey != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.ChatKey))?; }
        if self.Seq != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.Seq))?; }
        if self.UnseenCount != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.UnseenCount))?; }
        if self.FromHighMessageId != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.FromHighMessageId))?; }
        if self.ToLowMessageId != 0i64 { w.write_with_tag(72, |w| w.write_int64(*&self.ToLowMessageId))?; }
        if self.MessageId != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.MessageId))?; }
        if self.MessageFileId != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.MessageFileId))?; }
        if self.MessagePb != vec![] { w.write_with_tag(98, |w| w.write_bytes(&**&self.MessagePb))?; }
        if self.MessageJson != String::default() { w.write_with_tag(106, |w| w.write_string(&**&self.MessageJson))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_HTTPRPCLog {
    pub Id: i32,
    pub Time: String,
    pub MethodFull: String,
    pub MethodParent: String,
    pub UserId: i32,
    pub SessionId: String,
    pub StatusCode: i32,
    pub InputSize: i32,
    pub OutputSize: i32,
    pub ReqestJson: String,
    pub ResponseJson: String,
    pub ReqestParamJson: String,
    pub ResponseMsgJson: String,
}

impl<'a> MessageRead<'a> for PB_HTTPRPCLog {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int32(bytes)?,
                Ok(18) => msg.Time = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.MethodFull = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.MethodParent = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.UserId = r.read_int32(bytes)?,
                Ok(50) => msg.SessionId = r.read_string(bytes)?.to_owned(),
                Ok(56) => msg.StatusCode = r.read_int32(bytes)?,
                Ok(64) => msg.InputSize = r.read_int32(bytes)?,
                Ok(72) => msg.OutputSize = r.read_int32(bytes)?,
                Ok(82) => msg.ReqestJson = r.read_string(bytes)?.to_owned(),
                Ok(90) => msg.ResponseJson = r.read_string(bytes)?.to_owned(),
                Ok(98) => msg.ReqestParamJson = r.read_string(bytes)?.to_owned(),
                Ok(106) => msg.ResponseMsgJson = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_HTTPRPCLog {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.Time == String::default() { 0 } else { 1 + sizeof_len((&self.Time).len()) }
        + if self.MethodFull == String::default() { 0 } else { 1 + sizeof_len((&self.MethodFull).len()) }
        + if self.MethodParent == String::default() { 0 } else { 1 + sizeof_len((&self.MethodParent).len()) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.SessionId == String::default() { 0 } else { 1 + sizeof_len((&self.SessionId).len()) }
        + if self.StatusCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.StatusCode) as u64) }
        + if self.InputSize == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.InputSize) as u64) }
        + if self.OutputSize == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.OutputSize) as u64) }
        + if self.ReqestJson == String::default() { 0 } else { 1 + sizeof_len((&self.ReqestJson).len()) }
        + if self.ResponseJson == String::default() { 0 } else { 1 + sizeof_len((&self.ResponseJson).len()) }
        + if self.ReqestParamJson == String::default() { 0 } else { 1 + sizeof_len((&self.ReqestParamJson).len()) }
        + if self.ResponseMsgJson == String::default() { 0 } else { 1 + sizeof_len((&self.ResponseMsgJson).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.Id))?; }
        if self.Time != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Time))?; }
        if self.MethodFull != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.MethodFull))?; }
        if self.MethodParent != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.MethodParent))?; }
        if self.UserId != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.UserId))?; }
        if self.SessionId != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.SessionId))?; }
        if self.StatusCode != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.StatusCode))?; }
        if self.InputSize != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.InputSize))?; }
        if self.OutputSize != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.OutputSize))?; }
        if self.ReqestJson != String::default() { w.write_with_tag(82, |w| w.write_string(&**&self.ReqestJson))?; }
        if self.ResponseJson != String::default() { w.write_with_tag(90, |w| w.write_string(&**&self.ResponseJson))?; }
        if self.ReqestParamJson != String::default() { w.write_with_tag(98, |w| w.write_string(&**&self.ReqestParamJson))?; }
        if self.ResponseMsgJson != String::default() { w.write_with_tag(106, |w| w.write_string(&**&self.ResponseMsgJson))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_MetricLog {
    pub Id: i32,
    pub InstanceId: i32,
    pub StartFrom: String,
    pub EndTo: String,
    pub StartTime: i32,
    pub Duration: String,
    pub MetericsJson: String,
}

impl<'a> MessageRead<'a> for PB_MetricLog {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int32(bytes)?,
                Ok(16) => msg.InstanceId = r.read_int32(bytes)?,
                Ok(26) => msg.StartFrom = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.EndTo = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.StartTime = r.read_int32(bytes)?,
                Ok(50) => msg.Duration = r.read_string(bytes)?.to_owned(),
                Ok(58) => msg.MetericsJson = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_MetricLog {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.InstanceId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.InstanceId) as u64) }
        + if self.StartFrom == String::default() { 0 } else { 1 + sizeof_len((&self.StartFrom).len()) }
        + if self.EndTo == String::default() { 0 } else { 1 + sizeof_len((&self.EndTo).len()) }
        + if self.StartTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.StartTime) as u64) }
        + if self.Duration == String::default() { 0 } else { 1 + sizeof_len((&self.Duration).len()) }
        + if self.MetericsJson == String::default() { 0 } else { 1 + sizeof_len((&self.MetericsJson).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.Id))?; }
        if self.InstanceId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.InstanceId))?; }
        if self.StartFrom != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.StartFrom))?; }
        if self.EndTo != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.EndTo))?; }
        if self.StartTime != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.StartTime))?; }
        if self.Duration != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.Duration))?; }
        if self.MetericsJson != String::default() { w.write_with_tag(58, |w| w.write_string(&**&self.MetericsJson))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_XfileServiceInfoLog {
    pub Id: i64,
    pub InstanceId: i32,
    pub Url: String,
    pub CreatedTime: String,
}

impl<'a> MessageRead<'a> for PB_XfileServiceInfoLog {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.InstanceId = r.read_int32(bytes)?,
                Ok(26) => msg.Url = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.CreatedTime = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_XfileServiceInfoLog {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.InstanceId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.InstanceId) as u64) }
        + if self.Url == String::default() { 0 } else { 1 + sizeof_len((&self.Url).len()) }
        + if self.CreatedTime == String::default() { 0 } else { 1 + sizeof_len((&self.CreatedTime).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.InstanceId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.InstanceId))?; }
        if self.Url != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.Url))?; }
        if self.CreatedTime != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_XfileServiceMetricLog {
    pub Id: i64,
    pub InstanceId: i32,
    pub MetricJson: String,
}

impl<'a> MessageRead<'a> for PB_XfileServiceMetricLog {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.InstanceId = r.read_int32(bytes)?,
                Ok(26) => msg.MetricJson = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_XfileServiceMetricLog {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.InstanceId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.InstanceId) as u64) }
        + if self.MetricJson == String::default() { 0 } else { 1 + sizeof_len((&self.MetricJson).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.InstanceId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.InstanceId))?; }
        if self.MetricJson != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.MetricJson))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_XfileServiceRequestLog {
    pub Id: i64,
    pub LocalSeq: i32,
    pub InstanceId: i32,
    pub Url: String,
    pub HttpCode: i32,
    pub CreatedTime: String,
}

impl<'a> MessageRead<'a> for PB_XfileServiceRequestLog {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.LocalSeq = r.read_int32(bytes)?,
                Ok(24) => msg.InstanceId = r.read_int32(bytes)?,
                Ok(34) => msg.Url = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.HttpCode = r.read_int32(bytes)?,
                Ok(50) => msg.CreatedTime = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_XfileServiceRequestLog {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.LocalSeq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.LocalSeq) as u64) }
        + if self.InstanceId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.InstanceId) as u64) }
        + if self.Url == String::default() { 0 } else { 1 + sizeof_len((&self.Url).len()) }
        + if self.HttpCode == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.HttpCode) as u64) }
        + if self.CreatedTime == String::default() { 0 } else { 1 + sizeof_len((&self.CreatedTime).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.LocalSeq != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.LocalSeq))?; }
        if self.InstanceId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.InstanceId))?; }
        if self.Url != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.Url))?; }
        if self.HttpCode != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.HttpCode))?; }
        if self.CreatedTime != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_InvalidateCache {
    pub OrderId: i64,
    pub CacheKey: String,
}

impl<'a> MessageRead<'a> for PB_InvalidateCache {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.OrderId = r.read_int64(bytes)?,
                Ok(18) => msg.CacheKey = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_InvalidateCache {
    fn get_size(&self) -> usize {
        0
        + if self.OrderId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.OrderId) as u64) }
        + if self.CacheKey == String::default() { 0 } else { 1 + sizeof_len((&self.CacheKey).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.OrderId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.OrderId))?; }
        if self.CacheKey != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.CacheKey))?; }
        Ok(())
    }
}

