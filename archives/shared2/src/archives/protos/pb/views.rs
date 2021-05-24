// Automatically generated rust module for 'views.proto' file

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
pub enum ServerErrors {
    UNKNOWN_ERR = 0,
    ERR_CLIENT_IS_DEPRECATED = 1,
    ERR_SERVER_UPGRADING = 2,
}

impl Default for ServerErrors {
    fn default() -> Self {
        ServerErrors::UNKNOWN_ERR
    }
}

impl From<i32> for ServerErrors {
    fn from(i: i32) -> Self {
        match i {
            0 => ServerErrors::UNKNOWN_ERR,
            1 => ServerErrors::ERR_CLIENT_IS_DEPRECATED,
            2 => ServerErrors::ERR_SERVER_UPGRADING,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ServerErrors {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN_ERR" => ServerErrors::UNKNOWN_ERR,
            "ERR_CLIENT_IS_DEPRECATED" => ServerErrors::ERR_CLIENT_IS_DEPRECATED,
            "ERR_SERVER_UPGRADING" => ServerErrors::ERR_SERVER_UPGRADING,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MediaView { }

impl<'a> MessageRead<'a> for MediaView {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for MediaView { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionView {
    pub ActionId: i64,
    pub ActorUserId: i32,
    pub ActionTypeEnum: i32,
    pub PeerUserId: i32,
    pub PostId: i64,
    pub CommentId: i64,
    pub Murmur64Hash: i64,
    pub CreatedTime: i32,
    pub ActorUserView: Option<UserView>,
    pub PostView: Option<PostView>,
    pub CommentView: Option<CommentView>,
    pub FollowedUserView: Option<UserView>,
    pub ContentOwenerUserView: Option<UserView>,
}

impl<'a> MessageRead<'a> for ActionView {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ActionId = r.read_int64(bytes)?,
                Ok(16) => msg.ActorUserId = r.read_int32(bytes)?,
                Ok(24) => msg.ActionTypeEnum = r.read_int32(bytes)?,
                Ok(32) => msg.PeerUserId = r.read_int32(bytes)?,
                Ok(40) => msg.PostId = r.read_int64(bytes)?,
                Ok(48) => msg.CommentId = r.read_int64(bytes)?,
                Ok(56) => msg.Murmur64Hash = r.read_int64(bytes)?,
                Ok(64) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(802) => msg.ActorUserView = Some(r.read_message::<UserView>(bytes)?),
                Ok(810) => msg.PostView = Some(r.read_message::<PostView>(bytes)?),
                Ok(818) => msg.CommentView = Some(r.read_message::<CommentView>(bytes)?),
                Ok(826) => msg.FollowedUserView = Some(r.read_message::<UserView>(bytes)?),
                Ok(834) => msg.ContentOwenerUserView = Some(r.read_message::<UserView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionView {
    fn get_size(&self) -> usize {
        0
        + if self.ActionId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ActionId) as u64) }
        + if self.ActorUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ActorUserId) as u64) }
        + if self.ActionTypeEnum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ActionTypeEnum) as u64) }
        + if self.PeerUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PeerUserId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.CommentId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.CommentId) as u64) }
        + if self.Murmur64Hash == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Murmur64Hash) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
        + self.ActorUserView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.PostView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.CommentView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.FollowedUserView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.ContentOwenerUserView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ActionId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.ActionId))?; }
        if self.ActorUserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.ActorUserId))?; }
        if self.ActionTypeEnum != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.ActionTypeEnum))?; }
        if self.PeerUserId != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.PeerUserId))?; }
        if self.PostId != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.PostId))?; }
        if self.CommentId != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.CommentId))?; }
        if self.Murmur64Hash != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.Murmur64Hash))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.CreatedTime))?; }
        if let Some(ref s) = self.ActorUserView { w.write_with_tag(802, |w| w.write_message(s))?; }
        if let Some(ref s) = self.PostView { w.write_with_tag(810, |w| w.write_message(s))?; }
        if let Some(ref s) = self.CommentView { w.write_with_tag(818, |w| w.write_message(s))?; }
        if let Some(ref s) = self.FollowedUserView { w.write_with_tag(826, |w| w.write_message(s))?; }
        if let Some(ref s) = self.ContentOwenerUserView { w.write_with_tag(834, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NotifyView {
    pub NotifyId: i64,
    pub ForUserId: i32,
    pub ActorUserId: i32,
    pub NotiyTypeEnum: i32,
    pub PostId: i64,
    pub CommentId: i64,
    pub PeerUserId: i32,
    pub Murmur64Hash: i64,
    pub SeenStatus: i32,
    pub CreatedTime: i32,
    pub ActorUserView: Option<UserView>,
    pub PostView: Option<PostView>,
    pub CommentView: Option<CommentView>,
}

impl<'a> MessageRead<'a> for NotifyView {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.NotifyId = r.read_int64(bytes)?,
                Ok(16) => msg.ForUserId = r.read_int32(bytes)?,
                Ok(24) => msg.ActorUserId = r.read_int32(bytes)?,
                Ok(32) => msg.NotiyTypeEnum = r.read_int32(bytes)?,
                Ok(40) => msg.PostId = r.read_int64(bytes)?,
                Ok(48) => msg.CommentId = r.read_int64(bytes)?,
                Ok(56) => msg.PeerUserId = r.read_int32(bytes)?,
                Ok(64) => msg.Murmur64Hash = r.read_int64(bytes)?,
                Ok(72) => msg.SeenStatus = r.read_int32(bytes)?,
                Ok(80) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(802) => msg.ActorUserView = Some(r.read_message::<UserView>(bytes)?),
                Ok(810) => msg.PostView = Some(r.read_message::<PostView>(bytes)?),
                Ok(818) => msg.CommentView = Some(r.read_message::<CommentView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for NotifyView {
    fn get_size(&self) -> usize {
        0
        + if self.NotifyId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.NotifyId) as u64) }
        + if self.ForUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ForUserId) as u64) }
        + if self.ActorUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ActorUserId) as u64) }
        + if self.NotiyTypeEnum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.NotiyTypeEnum) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.CommentId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.CommentId) as u64) }
        + if self.PeerUserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.PeerUserId) as u64) }
        + if self.Murmur64Hash == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Murmur64Hash) as u64) }
        + if self.SeenStatus == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.SeenStatus) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
        + self.ActorUserView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.PostView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.CommentView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.NotifyId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.NotifyId))?; }
        if self.ForUserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.ForUserId))?; }
        if self.ActorUserId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.ActorUserId))?; }
        if self.NotiyTypeEnum != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.NotiyTypeEnum))?; }
        if self.PostId != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.PostId))?; }
        if self.CommentId != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.CommentId))?; }
        if self.PeerUserId != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.PeerUserId))?; }
        if self.Murmur64Hash != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.Murmur64Hash))?; }
        if self.SeenStatus != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.SeenStatus))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.CreatedTime))?; }
        if let Some(ref s) = self.ActorUserView { w.write_with_tag(802, |w| w.write_message(s))?; }
        if let Some(ref s) = self.PostView { w.write_with_tag(810, |w| w.write_message(s))?; }
        if let Some(ref s) = self.CommentView { w.write_with_tag(818, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CommentView {
    pub CommentId: i64,
    pub UserId: i32,
    pub PostId: i64,
    pub Text: String,
    pub LikesCount: i32,
    pub CreatedTime: i32,
    pub SenderUserView: Option<UserView>,
}

impl<'a> MessageRead<'a> for CommentView {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.CommentId = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(24) => msg.PostId = r.read_int64(bytes)?,
                Ok(34) => msg.Text = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.LikesCount = r.read_int32(bytes)?,
                Ok(48) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(122) => msg.SenderUserView = Some(r.read_message::<UserView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CommentView {
    fn get_size(&self) -> usize {
        0
        + if self.CommentId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.CommentId) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.Text == String::default() { 0 } else { 1 + sizeof_len((&self.Text).len()) }
        + if self.LikesCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.LikesCount) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
        + self.SenderUserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.CommentId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.CommentId))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        if self.PostId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.PostId))?; }
        if self.Text != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.Text))?; }
        if self.LikesCount != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.LikesCount))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.CreatedTime))?; }
        if let Some(ref s) = self.SenderUserView { w.write_with_tag(122, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PostView {
    pub PostId: i64,
    pub UserId: i32,
    pub Text: String,
    pub RichText: String,
    pub MediaCount: i32,
    pub SharedTo: i32,
    pub DisableComment: i32,
    pub HasTag: i32,
    pub CommentsCount: i32,
    pub LikesCount: i32,
    pub ViewsCount: i32,
    pub EditedTime: i32,
    pub CreatedTime: i32,
    pub ReSharedPostId: i64,
    pub DidILiked: bool,
    pub DidIReShared: bool,
    pub SenderUserView: Option<UserView>,
    pub ReSharedUserView: Option<UserView>,
    pub MediaView: Option<MediaView>,
    pub MediaViewList: Vec<MediaView>,
}

impl<'a> MessageRead<'a> for PostView {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int32(bytes)?,
                Ok(34) => msg.Text = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.RichText = r.read_string(bytes)?.to_owned(),
                Ok(48) => msg.MediaCount = r.read_int32(bytes)?,
                Ok(56) => msg.SharedTo = r.read_int32(bytes)?,
                Ok(64) => msg.DisableComment = r.read_int32(bytes)?,
                Ok(72) => msg.HasTag = r.read_int32(bytes)?,
                Ok(80) => msg.CommentsCount = r.read_int32(bytes)?,
                Ok(88) => msg.LikesCount = r.read_int32(bytes)?,
                Ok(96) => msg.ViewsCount = r.read_int32(bytes)?,
                Ok(104) => msg.EditedTime = r.read_int32(bytes)?,
                Ok(112) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(120) => msg.ReSharedPostId = r.read_int64(bytes)?,
                Ok(400) => msg.DidILiked = r.read_bool(bytes)?,
                Ok(408) => msg.DidIReShared = r.read_bool(bytes)?,
                Ok(802) => msg.SenderUserView = Some(r.read_message::<UserView>(bytes)?),
                Ok(810) => msg.ReSharedUserView = Some(r.read_message::<UserView>(bytes)?),
                Ok(818) => msg.MediaView = Some(r.read_message::<MediaView>(bytes)?),
                Ok(826) => msg.MediaViewList.push(r.read_message::<MediaView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PostView {
    fn get_size(&self) -> usize {
        0
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.Text == String::default() { 0 } else { 1 + sizeof_len((&self.Text).len()) }
        + if self.RichText == String::default() { 0 } else { 1 + sizeof_len((&self.RichText).len()) }
        + if self.MediaCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.MediaCount) as u64) }
        + if self.SharedTo == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.SharedTo) as u64) }
        + if self.DisableComment == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.DisableComment) as u64) }
        + if self.HasTag == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.HasTag) as u64) }
        + if self.CommentsCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CommentsCount) as u64) }
        + if self.LikesCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.LikesCount) as u64) }
        + if self.ViewsCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ViewsCount) as u64) }
        + if self.EditedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.EditedTime) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
        + if self.ReSharedPostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ReSharedPostId) as u64) }
        + if self.DidILiked == false { 0 } else { 2 + sizeof_varint(*(&self.DidILiked) as u64) }
        + if self.DidIReShared == false { 0 } else { 2 + sizeof_varint(*(&self.DidIReShared) as u64) }
        + self.SenderUserView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.ReSharedUserView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.MediaView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.MediaViewList.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.UserId != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.UserId))?; }
        if self.Text != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.Text))?; }
        if self.RichText != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.RichText))?; }
        if self.MediaCount != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.MediaCount))?; }
        if self.SharedTo != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.SharedTo))?; }
        if self.DisableComment != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.DisableComment))?; }
        if self.HasTag != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.HasTag))?; }
        if self.CommentsCount != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.CommentsCount))?; }
        if self.LikesCount != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.LikesCount))?; }
        if self.ViewsCount != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.ViewsCount))?; }
        if self.EditedTime != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.EditedTime))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.CreatedTime))?; }
        if self.ReSharedPostId != 0i64 { w.write_with_tag(120, |w| w.write_int64(*&self.ReSharedPostId))?; }
        if self.DidILiked != false { w.write_with_tag(400, |w| w.write_bool(*&self.DidILiked))?; }
        if self.DidIReShared != false { w.write_with_tag(408, |w| w.write_bool(*&self.DidIReShared))?; }
        if let Some(ref s) = self.SenderUserView { w.write_with_tag(802, |w| w.write_message(s))?; }
        if let Some(ref s) = self.ReSharedUserView { w.write_with_tag(810, |w| w.write_message(s))?; }
        if let Some(ref s) = self.MediaView { w.write_with_tag(818, |w| w.write_message(s))?; }
        for s in &self.MediaViewList { w.write_with_tag(826, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatView {
    pub ChatId: i64,
    pub ChatKey: String,
    pub RoomKey: String,
    pub RoomType: i32,
    pub UserId: i32,
    pub PeerUserId: i32,
    pub GroupId: i64,
    pub HashTagId: i64,
    pub StartedByMe: i32,
    pub Title: String,
    pub PinTime: i64,
    pub FromMsgId: i64,
    pub Seq: i32,
    pub LastMsgId: i64,
    pub LastMsgStatus: i32,
    pub SeenSeq: i32,
    pub SeenMsgId: i64,
    pub Left: i32,
    pub Creator: i32,
    pub Kicked: i32,
    pub Admin: i32,
    pub Deactivated: i32,
    pub VersionTime: i32,
    pub SortTime: i32,
    pub CreatedTime: i32,
    pub DraftText: String,
    pub DratReplyToMsgId: i64,
    pub IsMute: i32,
    pub UserView: Option<UserView>,
    pub GroupView: Option<GroupView>,
    pub FirstUnreadMessage: Option<MessageView>,
    pub LastMessage: Option<MessageView>,
}

impl<'a> MessageRead<'a> for ChatView {
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
                Ok(72) => msg.StartedByMe = r.read_int32(bytes)?,
                Ok(82) => msg.Title = r.read_string(bytes)?.to_owned(),
                Ok(88) => msg.PinTime = r.read_int64(bytes)?,
                Ok(96) => msg.FromMsgId = r.read_int64(bytes)?,
                Ok(104) => msg.Seq = r.read_int32(bytes)?,
                Ok(112) => msg.LastMsgId = r.read_int64(bytes)?,
                Ok(120) => msg.LastMsgStatus = r.read_int32(bytes)?,
                Ok(128) => msg.SeenSeq = r.read_int32(bytes)?,
                Ok(136) => msg.SeenMsgId = r.read_int64(bytes)?,
                Ok(144) => msg.Left = r.read_int32(bytes)?,
                Ok(152) => msg.Creator = r.read_int32(bytes)?,
                Ok(160) => msg.Kicked = r.read_int32(bytes)?,
                Ok(168) => msg.Admin = r.read_int32(bytes)?,
                Ok(176) => msg.Deactivated = r.read_int32(bytes)?,
                Ok(184) => msg.VersionTime = r.read_int32(bytes)?,
                Ok(192) => msg.SortTime = r.read_int32(bytes)?,
                Ok(200) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(210) => msg.DraftText = r.read_string(bytes)?.to_owned(),
                Ok(216) => msg.DratReplyToMsgId = r.read_int64(bytes)?,
                Ok(224) => msg.IsMute = r.read_int32(bytes)?,
                Ok(802) => msg.UserView = Some(r.read_message::<UserView>(bytes)?),
                Ok(810) => msg.GroupView = Some(r.read_message::<GroupView>(bytes)?),
                Ok(1602) => msg.FirstUnreadMessage = Some(r.read_message::<MessageView>(bytes)?),
                Ok(1610) => msg.LastMessage = Some(r.read_message::<MessageView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChatView {
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
        + if self.StartedByMe == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.StartedByMe) as u64) }
        + if self.Title == String::default() { 0 } else { 1 + sizeof_len((&self.Title).len()) }
        + if self.PinTime == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PinTime) as u64) }
        + if self.FromMsgId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.FromMsgId) as u64) }
        + if self.Seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Seq) as u64) }
        + if self.LastMsgId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastMsgId) as u64) }
        + if self.LastMsgStatus == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.LastMsgStatus) as u64) }
        + if self.SeenSeq == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.SeenSeq) as u64) }
        + if self.SeenMsgId == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.SeenMsgId) as u64) }
        + if self.Left == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.Left) as u64) }
        + if self.Creator == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.Creator) as u64) }
        + if self.Kicked == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.Kicked) as u64) }
        + if self.Admin == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.Admin) as u64) }
        + if self.Deactivated == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.Deactivated) as u64) }
        + if self.VersionTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.VersionTime) as u64) }
        + if self.SortTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.SortTime) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.CreatedTime) as u64) }
        + if self.DraftText == String::default() { 0 } else { 2 + sizeof_len((&self.DraftText).len()) }
        + if self.DratReplyToMsgId == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.DratReplyToMsgId) as u64) }
        + if self.IsMute == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.IsMute) as u64) }
        + self.UserView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.GroupView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.FirstUnreadMessage.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.LastMessage.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
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
        if self.StartedByMe != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.StartedByMe))?; }
        if self.Title != String::default() { w.write_with_tag(82, |w| w.write_string(&**&self.Title))?; }
        if self.PinTime != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.PinTime))?; }
        if self.FromMsgId != 0i64 { w.write_with_tag(96, |w| w.write_int64(*&self.FromMsgId))?; }
        if self.Seq != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.Seq))?; }
        if self.LastMsgId != 0i64 { w.write_with_tag(112, |w| w.write_int64(*&self.LastMsgId))?; }
        if self.LastMsgStatus != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.LastMsgStatus))?; }
        if self.SeenSeq != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.SeenSeq))?; }
        if self.SeenMsgId != 0i64 { w.write_with_tag(136, |w| w.write_int64(*&self.SeenMsgId))?; }
        if self.Left != 0i32 { w.write_with_tag(144, |w| w.write_int32(*&self.Left))?; }
        if self.Creator != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.Creator))?; }
        if self.Kicked != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.Kicked))?; }
        if self.Admin != 0i32 { w.write_with_tag(168, |w| w.write_int32(*&self.Admin))?; }
        if self.Deactivated != 0i32 { w.write_with_tag(176, |w| w.write_int32(*&self.Deactivated))?; }
        if self.VersionTime != 0i32 { w.write_with_tag(184, |w| w.write_int32(*&self.VersionTime))?; }
        if self.SortTime != 0i32 { w.write_with_tag(192, |w| w.write_int32(*&self.SortTime))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(200, |w| w.write_int32(*&self.CreatedTime))?; }
        if self.DraftText != String::default() { w.write_with_tag(210, |w| w.write_string(&**&self.DraftText))?; }
        if self.DratReplyToMsgId != 0i64 { w.write_with_tag(216, |w| w.write_int64(*&self.DratReplyToMsgId))?; }
        if self.IsMute != 0i32 { w.write_with_tag(224, |w| w.write_int32(*&self.IsMute))?; }
        if let Some(ref s) = self.UserView { w.write_with_tag(802, |w| w.write_message(s))?; }
        if let Some(ref s) = self.GroupView { w.write_with_tag(810, |w| w.write_message(s))?; }
        if let Some(ref s) = self.FirstUnreadMessage { w.write_with_tag(1602, |w| w.write_message(s))?; }
        if let Some(ref s) = self.LastMessage { w.write_with_tag(1610, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupView {
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
    pub InviteLink: String,
    pub MembersCount: i32,
    pub SortTime: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for GroupView {
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
                Ok(130) => msg.InviteLink = r.read_string(bytes)?.to_owned(),
                Ok(136) => msg.MembersCount = r.read_int32(bytes)?,
                Ok(144) => msg.SortTime = r.read_int32(bytes)?,
                Ok(152) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupView {
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
        + if self.InviteLink == String::default() { 0 } else { 2 + sizeof_len((&self.InviteLink).len()) }
        + if self.MembersCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.MembersCount) as u64) }
        + if self.SortTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.SortTime) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.CreatedTime) as u64) }
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
        if self.InviteLink != String::default() { w.write_with_tag(130, |w| w.write_string(&**&self.InviteLink))?; }
        if self.MembersCount != 0i32 { w.write_with_tag(136, |w| w.write_int32(*&self.MembersCount))?; }
        if self.SortTime != 0i32 { w.write_with_tag(144, |w| w.write_int32(*&self.SortTime))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupMemberView {
    pub OrderId: i64,
    pub GroupId: i64,
    pub UserId: i32,
    pub ByUserId: i32,
    pub GroupRole: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for GroupMemberView {
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

impl MessageWrite for GroupMemberView {
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
pub struct MessageView {
    pub RoomKey: String,
    pub MessageId: i64,
    pub UserId: i32,
    pub FileRefId: i64,
    pub MessageType: i32,
    pub Text: String,
    pub Hiden: i32,
    pub Seq: i32,
    pub ForwardedMsgId: i64,
    pub PostId: i64,
    pub StickerId: i64,
    pub CreatedTime: i32,
    pub DeliveredTime: i32,
    pub SeenTime: i32,
    pub DeliviryStatus: i32,
    pub ReplyToMessageId: i64,
    pub ViewsCount: i64,
    pub EditTime: i32,
    pub Ttl: i32,
    pub FileRedView: Option<FileRedView>,
}

impl<'a> MessageRead<'a> for MessageView {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.RoomKey = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.MessageId = r.read_int64(bytes)?,
                Ok(24) => msg.UserId = r.read_int32(bytes)?,
                Ok(32) => msg.FileRefId = r.read_int64(bytes)?,
                Ok(40) => msg.MessageType = r.read_int32(bytes)?,
                Ok(50) => msg.Text = r.read_string(bytes)?.to_owned(),
                Ok(56) => msg.Hiden = r.read_int32(bytes)?,
                Ok(64) => msg.Seq = r.read_int32(bytes)?,
                Ok(72) => msg.ForwardedMsgId = r.read_int64(bytes)?,
                Ok(80) => msg.PostId = r.read_int64(bytes)?,
                Ok(88) => msg.StickerId = r.read_int64(bytes)?,
                Ok(96) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(104) => msg.DeliveredTime = r.read_int32(bytes)?,
                Ok(112) => msg.SeenTime = r.read_int32(bytes)?,
                Ok(120) => msg.DeliviryStatus = r.read_int32(bytes)?,
                Ok(128) => msg.ReplyToMessageId = r.read_int64(bytes)?,
                Ok(136) => msg.ViewsCount = r.read_int64(bytes)?,
                Ok(144) => msg.EditTime = r.read_int32(bytes)?,
                Ok(152) => msg.Ttl = r.read_int32(bytes)?,
                Ok(402) => msg.FileRedView = Some(r.read_message::<FileRedView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MessageView {
    fn get_size(&self) -> usize {
        0
        + if self.RoomKey == String::default() { 0 } else { 1 + sizeof_len((&self.RoomKey).len()) }
        + if self.MessageId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.MessageId) as u64) }
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.FileRefId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.FileRefId) as u64) }
        + if self.MessageType == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.MessageType) as u64) }
        + if self.Text == String::default() { 0 } else { 1 + sizeof_len((&self.Text).len()) }
        + if self.Hiden == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Hiden) as u64) }
        + if self.Seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Seq) as u64) }
        + if self.ForwardedMsgId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ForwardedMsgId) as u64) }
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.StickerId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.StickerId) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
        + if self.DeliveredTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.DeliveredTime) as u64) }
        + if self.SeenTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.SeenTime) as u64) }
        + if self.DeliviryStatus == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.DeliviryStatus) as u64) }
        + if self.ReplyToMessageId == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.ReplyToMessageId) as u64) }
        + if self.ViewsCount == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.ViewsCount) as u64) }
        + if self.EditTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.EditTime) as u64) }
        + if self.Ttl == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.Ttl) as u64) }
        + self.FileRedView.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.RoomKey != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.RoomKey))?; }
        if self.MessageId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.MessageId))?; }
        if self.UserId != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.UserId))?; }
        if self.FileRefId != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.FileRefId))?; }
        if self.MessageType != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.MessageType))?; }
        if self.Text != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.Text))?; }
        if self.Hiden != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.Hiden))?; }
        if self.Seq != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.Seq))?; }
        if self.ForwardedMsgId != 0i64 { w.write_with_tag(72, |w| w.write_int64(*&self.ForwardedMsgId))?; }
        if self.PostId != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.PostId))?; }
        if self.StickerId != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.StickerId))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.CreatedTime))?; }
        if self.DeliveredTime != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.DeliveredTime))?; }
        if self.SeenTime != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.SeenTime))?; }
        if self.DeliviryStatus != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.DeliviryStatus))?; }
        if self.ReplyToMessageId != 0i64 { w.write_with_tag(128, |w| w.write_int64(*&self.ReplyToMessageId))?; }
        if self.ViewsCount != 0i64 { w.write_with_tag(136, |w| w.write_int64(*&self.ViewsCount))?; }
        if self.EditTime != 0i32 { w.write_with_tag(144, |w| w.write_int32(*&self.EditTime))?; }
        if self.Ttl != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.Ttl))?; }
        if let Some(ref s) = self.FileRedView { w.write_with_tag(402, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileRedView {
    pub FileRefId: i64,
    pub UserId: i64,
    pub Name: String,
    pub Width: i32,
    pub Height: i32,
    pub Duration: i32,
    pub Extension: String,
    pub UrlSource: String,
}

impl<'a> MessageRead<'a> for FileRedView {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.FileRefId = r.read_int64(bytes)?,
                Ok(16) => msg.UserId = r.read_int64(bytes)?,
                Ok(26) => msg.Name = r.read_string(bytes)?.to_owned(),
                Ok(32) => msg.Width = r.read_int32(bytes)?,
                Ok(40) => msg.Height = r.read_int32(bytes)?,
                Ok(48) => msg.Duration = r.read_int32(bytes)?,
                Ok(58) => msg.Extension = r.read_string(bytes)?.to_owned(),
                Ok(66) => msg.UrlSource = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for FileRedView {
    fn get_size(&self) -> usize {
        0
        + if self.FileRefId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.FileRefId) as u64) }
        + if self.UserId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.Name == String::default() { 0 } else { 1 + sizeof_len((&self.Name).len()) }
        + if self.Width == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Width) as u64) }
        + if self.Height == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Height) as u64) }
        + if self.Duration == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Duration) as u64) }
        + if self.Extension == String::default() { 0 } else { 1 + sizeof_len((&self.Extension).len()) }
        + if self.UrlSource == String::default() { 0 } else { 1 + sizeof_len((&self.UrlSource).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.FileRefId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.FileRefId))?; }
        if self.UserId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.UserId))?; }
        if self.Name != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.Name))?; }
        if self.Width != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.Width))?; }
        if self.Height != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.Height))?; }
        if self.Duration != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.Duration))?; }
        if self.Extension != String::default() { w.write_with_tag(58, |w| w.write_string(&**&self.Extension))?; }
        if self.UrlSource != String::default() { w.write_with_tag(66, |w| w.write_string(&**&self.UrlSource))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UserView {
    pub UserId: i32,
    pub UserName: String,
    pub FirstName: String,
    pub LastName: String,
    pub AvatarRefId: i64,
    pub ProfilePrivacy: i32,
    pub Phone: i64,
    pub About: String,
    pub FollowersCount: i32,
    pub FollowingCount: i32,
    pub PostsCount: i32,
    pub MediaCount: i32,
    pub UserOnlineStatusEnum: enums::UserOnlineStatusEnum,
    pub LastActiveTime: i32,
    pub LastActiveTimeShow: String,
    pub MyFollwing: enums::FollowingEnum,
}

impl<'a> MessageRead<'a> for UserView {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.UserId = r.read_int32(bytes)?,
                Ok(18) => msg.UserName = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.FirstName = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.LastName = r.read_string(bytes)?.to_owned(),
                Ok(64) => msg.AvatarRefId = r.read_int64(bytes)?,
                Ok(72) => msg.ProfilePrivacy = r.read_int32(bytes)?,
                Ok(80) => msg.Phone = r.read_int64(bytes)?,
                Ok(90) => msg.About = r.read_string(bytes)?.to_owned(),
                Ok(800) => msg.FollowersCount = r.read_int32(bytes)?,
                Ok(808) => msg.FollowingCount = r.read_int32(bytes)?,
                Ok(816) => msg.PostsCount = r.read_int32(bytes)?,
                Ok(824) => msg.MediaCount = r.read_int32(bytes)?,
                Ok(1600) => msg.UserOnlineStatusEnum = r.read_enum(bytes)?,
                Ok(1608) => msg.LastActiveTime = r.read_int32(bytes)?,
                Ok(1618) => msg.LastActiveTimeShow = r.read_string(bytes)?.to_owned(),
                Ok(2400) => msg.MyFollwing = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UserView {
    fn get_size(&self) -> usize {
        0
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.UserName == String::default() { 0 } else { 1 + sizeof_len((&self.UserName).len()) }
        + if self.FirstName == String::default() { 0 } else { 1 + sizeof_len((&self.FirstName).len()) }
        + if self.LastName == String::default() { 0 } else { 1 + sizeof_len((&self.LastName).len()) }
        + if self.AvatarRefId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.AvatarRefId) as u64) }
        + if self.ProfilePrivacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ProfilePrivacy) as u64) }
        + if self.Phone == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Phone) as u64) }
        + if self.About == String::default() { 0 } else { 1 + sizeof_len((&self.About).len()) }
        + if self.FollowersCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.FollowersCount) as u64) }
        + if self.FollowingCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.FollowingCount) as u64) }
        + if self.PostsCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.PostsCount) as u64) }
        + if self.MediaCount == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.MediaCount) as u64) }
        + if self.UserOnlineStatusEnum == enums::UserOnlineStatusEnum::EXACTLY { 0 } else { 2 + sizeof_varint(*(&self.UserOnlineStatusEnum) as u64) }
        + if self.LastActiveTime == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.LastActiveTime) as u64) }
        + if self.LastActiveTimeShow == String::default() { 0 } else { 2 + sizeof_len((&self.LastActiveTimeShow).len()) }
        + if self.MyFollwing == enums::FollowingEnum::FOLLOWING_NONE { 0 } else { 2 + sizeof_varint(*(&self.MyFollwing) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserId != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.UserId))?; }
        if self.UserName != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.UserName))?; }
        if self.FirstName != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.FirstName))?; }
        if self.LastName != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.LastName))?; }
        if self.AvatarRefId != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.AvatarRefId))?; }
        if self.ProfilePrivacy != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.ProfilePrivacy))?; }
        if self.Phone != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.Phone))?; }
        if self.About != String::default() { w.write_with_tag(90, |w| w.write_string(&**&self.About))?; }
        if self.FollowersCount != 0i32 { w.write_with_tag(800, |w| w.write_int32(*&self.FollowersCount))?; }
        if self.FollowingCount != 0i32 { w.write_with_tag(808, |w| w.write_int32(*&self.FollowingCount))?; }
        if self.PostsCount != 0i32 { w.write_with_tag(816, |w| w.write_int32(*&self.PostsCount))?; }
        if self.MediaCount != 0i32 { w.write_with_tag(824, |w| w.write_int32(*&self.MediaCount))?; }
        if self.UserOnlineStatusEnum != enums::UserOnlineStatusEnum::EXACTLY { w.write_with_tag(1600, |w| w.write_enum(*&self.UserOnlineStatusEnum as i32))?; }
        if self.LastActiveTime != 0i32 { w.write_with_tag(1608, |w| w.write_int32(*&self.LastActiveTime))?; }
        if self.LastActiveTimeShow != String::default() { w.write_with_tag(1618, |w| w.write_string(&**&self.LastActiveTimeShow))?; }
        if self.MyFollwing != enums::FollowingEnum::FOLLOWING_NONE { w.write_with_tag(2400, |w| w.write_enum(*&self.MyFollwing as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SettingNotificationView { }

impl<'a> MessageRead<'a> for SettingNotificationView {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for SettingNotificationView { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AppConfig {
    pub DeprecatedClient: bool,
    pub HasNewUpdate: bool,
}

impl<'a> MessageRead<'a> for AppConfig {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.DeprecatedClient = r.read_bool(bytes)?,
                Ok(16) => msg.HasNewUpdate = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for AppConfig {
    fn get_size(&self) -> usize {
        0
        + if self.DeprecatedClient == false { 0 } else { 1 + sizeof_varint(*(&self.DeprecatedClient) as u64) }
        + if self.HasNewUpdate == false { 0 } else { 1 + sizeof_varint(*(&self.HasNewUpdate) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.DeprecatedClient != false { w.write_with_tag(8, |w| w.write_bool(*&self.DeprecatedClient))?; }
        if self.HasNewUpdate != false { w.write_with_tag(16, |w| w.write_bool(*&self.HasNewUpdate))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UserProfileView { }

impl<'a> MessageRead<'a> for UserProfileView {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UserProfileView { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UserViewRowify {
    pub Id: i64,
    pub CreatedTime: i32,
    pub UserView: Option<UserView>,
}

impl<'a> MessageRead<'a> for UserViewRowify {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(16) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(82) => msg.UserView = Some(r.read_message::<UserView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UserViewRowify {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
        + self.UserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.CreatedTime))?; }
        if let Some(ref s) = self.UserView { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PostViewRowify {
    pub Id: i64,
    pub PostView: Option<PostView>,
}

impl<'a> MessageRead<'a> for PostViewRowify {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Id = r.read_int64(bytes)?,
                Ok(82) => msg.PostView = Some(r.read_message::<PostView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PostViewRowify {
    fn get_size(&self) -> usize {
        0
        + if self.Id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Id) as u64) }
        + self.PostView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.Id))?; }
        if let Some(ref s) = self.PostView { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TagView {
    pub TagId: i64,
    pub Name: String,
    pub Count: i32,
    pub TagStatusEnum: i32,
    pub CreatedTime: i32,
}

impl<'a> MessageRead<'a> for TagView {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.TagId = r.read_int64(bytes)?,
                Ok(18) => msg.Name = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.Count = r.read_int32(bytes)?,
                Ok(32) => msg.TagStatusEnum = r.read_int32(bytes)?,
                Ok(40) => msg.CreatedTime = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TagView {
    fn get_size(&self) -> usize {
        0
        + if self.TagId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.TagId) as u64) }
        + if self.Name == String::default() { 0 } else { 1 + sizeof_len((&self.Name).len()) }
        + if self.Count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Count) as u64) }
        + if self.TagStatusEnum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.TagStatusEnum) as u64) }
        + if self.CreatedTime == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.CreatedTime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.TagId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.TagId))?; }
        if self.Name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Name))?; }
        if self.Count != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.Count))?; }
        if self.TagStatusEnum != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.TagStatusEnum))?; }
        if self.CreatedTime != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.CreatedTime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TopTagWithSamplePosts {
    pub TagView: Option<TagView>,
    pub PostViewList: Vec<PostView>,
}

impl<'a> MessageRead<'a> for TopTagWithSamplePosts {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.TagView = Some(r.read_message::<TagView>(bytes)?),
                Ok(18) => msg.PostViewList.push(r.read_message::<PostView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TopTagWithSamplePosts {
    fn get_size(&self) -> usize {
        0
        + self.TagView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.PostViewList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.TagView { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.PostViewList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SelfUserView {
    pub UserView: Option<UserView>,
    pub ProfilePrivacy: i32,
    pub OnlinePrivacy: i32,
    pub CallPrivacy: i32,
    pub AddToGroupPrivacy: i32,
    pub SeenMessagePrivacy: i32,
    pub SettingNotification: Option<SettingNotificationView>,
}

impl<'a> MessageRead<'a> for SelfUserView {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.UserView = Some(r.read_message::<UserView>(bytes)?),
                Ok(240) => msg.ProfilePrivacy = r.read_int32(bytes)?,
                Ok(256) => msg.OnlinePrivacy = r.read_int32(bytes)?,
                Ok(264) => msg.CallPrivacy = r.read_int32(bytes)?,
                Ok(272) => msg.AddToGroupPrivacy = r.read_int32(bytes)?,
                Ok(280) => msg.SeenMessagePrivacy = r.read_int32(bytes)?,
                Ok(802) => msg.SettingNotification = Some(r.read_message::<SettingNotificationView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SelfUserView {
    fn get_size(&self) -> usize {
        0
        + self.UserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.ProfilePrivacy == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.ProfilePrivacy) as u64) }
        + if self.OnlinePrivacy == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.OnlinePrivacy) as u64) }
        + if self.CallPrivacy == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.CallPrivacy) as u64) }
        + if self.AddToGroupPrivacy == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.AddToGroupPrivacy) as u64) }
        + if self.SeenMessagePrivacy == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.SeenMessagePrivacy) as u64) }
        + self.SettingNotification.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.UserView { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.ProfilePrivacy != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.ProfilePrivacy))?; }
        if self.OnlinePrivacy != 0i32 { w.write_with_tag(256, |w| w.write_int32(*&self.OnlinePrivacy))?; }
        if self.CallPrivacy != 0i32 { w.write_with_tag(264, |w| w.write_int32(*&self.CallPrivacy))?; }
        if self.AddToGroupPrivacy != 0i32 { w.write_with_tag(272, |w| w.write_int32(*&self.AddToGroupPrivacy))?; }
        if self.SeenMessagePrivacy != 0i32 { w.write_with_tag(280, |w| w.write_int32(*&self.SeenMessagePrivacy))?; }
        if let Some(ref s) = self.SettingNotification { w.write_with_tag(802, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Error {
    pub Error: ServerErrors,
    pub ShowError: bool,
    pub ErrorMessage: String,
}

impl<'a> MessageRead<'a> for Error {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Error = r.read_enum(bytes)?,
                Ok(16) => msg.ShowError = r.read_bool(bytes)?,
                Ok(26) => msg.ErrorMessage = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Error {
    fn get_size(&self) -> usize {
        0
        + if self.Error == views::ServerErrors::UNKNOWN_ERR { 0 } else { 1 + sizeof_varint(*(&self.Error) as u64) }
        + if self.ShowError == false { 0 } else { 1 + sizeof_varint(*(&self.ShowError) as u64) }
        + if self.ErrorMessage == String::default() { 0 } else { 1 + sizeof_len((&self.ErrorMessage).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Error != views::ServerErrors::UNKNOWN_ERR { w.write_with_tag(8, |w| w.write_enum(*&self.Error as i32))?; }
        if self.ShowError != false { w.write_with_tag(16, |w| w.write_bool(*&self.ShowError))?; }
        if self.ErrorMessage != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.ErrorMessage))?; }
        Ok(())
    }
}

