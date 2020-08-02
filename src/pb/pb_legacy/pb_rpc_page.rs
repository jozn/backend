// Automatically generated rust module for 'pb_rpc_page.proto' file

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
pub struct RPC_Page_Types { }

impl<'a> MessageRead<'a> for RPC_Page_Types {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RPC_Page_Types { }

pub mod mod_RPC_Page_Types {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetCommentsPage { }

impl<'a> MessageRead<'a> for GetCommentsPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetCommentsPage { }

pub mod mod_GetCommentsPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
    pub Limit: i32,
    pub Last: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(16) => msg.Limit = r.read_int32(bytes)?,
                Ok(24) => msg.Last = r.read_int64(bytes)?,
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
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.Last == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.Last) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.Limit != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.Limit))?; }
        if self.Last != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.Last))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub CommentViewList: Vec<pb_views::PB_CommentView>,
    pub HasMore: bool,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.CommentViewList.push(r.read_message::<pb_views::PB_CommentView>(bytes)?),
                Ok(32) => msg.HasMore = r.read_bool(bytes)?,
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.CommentViewList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.HasMore == false { 0 } else { 1 + sizeof_varint(*(&self.HasMore) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.CommentViewList { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.HasMore != false { w.write_with_tag(32, |w| w.write_bool(*&self.HasMore))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetHomePage { }

impl<'a> MessageRead<'a> for GetHomePage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetHomePage { }

pub mod mod_GetHomePage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Limit: i32,
    pub LastId: i64,
    pub Category: pb_enum::CategoryEnum,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Limit = r.read_int32(bytes)?,
                Ok(16) => msg.LastId = r.read_int64(bytes)?,
                Ok(24) => msg.Category = r.read_enum(bytes)?,
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
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
        + if self.Category == pb_enum::CategoryEnum::Category_RESHARED { 0 } else { 1 + sizeof_varint(*(&self.Category) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Limit != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.Limit))?; }
        if self.LastId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.LastId))?; }
        if self.Category != pb_enum::CategoryEnum::Category_RESHARED { w.write_with_tag(24, |w| w.write_enum(*&self.Category as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub PostViewList: Vec<pb_views::PB_PostView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.PostViewList.push(r.read_message::<pb_views::PB_PostView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.PostViewList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.PostViewList { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetProfileAbout { }

impl<'a> MessageRead<'a> for GetProfileAbout {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetProfileAbout { }

pub mod mod_GetProfileAbout {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub UserId: i32,
    pub Username: String,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.UserId = r.read_int32(bytes)?,
                Ok(18) => msg.Username = r.read_string(bytes)?.to_owned(),
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
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.Username == String::default() { 0 } else { 1 + sizeof_len((&self.Username).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserId != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.UserId))?; }
        if self.Username != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Username))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub UserProfileView: Option<pb_views::PB_UserProfileView>,
    pub GroupView: Option<pb_views::PB_GroupView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.UserProfileView = Some(r.read_message::<pb_views::PB_UserProfileView>(bytes)?),
                Ok(34) => msg.GroupView = Some(r.read_message::<pb_views::PB_GroupView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.UserProfileView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.GroupView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.UserProfileView { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.GroupView { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetProfileAllShared { }

impl<'a> MessageRead<'a> for GetProfileAllShared {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetProfileAllShared { }

pub mod mod_GetProfileAllShared {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub UserId: i32,
    pub Username: String,
    pub LastId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.UserId = r.read_int32(bytes)?,
                Ok(18) => msg.Username = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.LastId = r.read_int64(bytes)?,
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
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.Username == String::default() { 0 } else { 1 + sizeof_len((&self.Username).len()) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserId != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.UserId))?; }
        if self.Username != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Username))?; }
        if self.LastId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.LastId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub PostViewList: Vec<pb_views::PB_PostView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.PostViewList.push(r.read_message::<pb_views::PB_PostView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.PostViewList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.PostViewList { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetProfileByCategoryPage { }

impl<'a> MessageRead<'a> for GetProfileByCategoryPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetProfileByCategoryPage { }

pub mod mod_GetProfileByCategoryPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub UserId: i32,
    pub LastId: i64,
    pub Category: pb_enum::CategoryEnum,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.UserId = r.read_int32(bytes)?,
                Ok(24) => msg.LastId = r.read_int64(bytes)?,
                Ok(16) => msg.Category = r.read_enum(bytes)?,
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
        + if self.UserId == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
        + if self.Category == pb_enum::CategoryEnum::Category_RESHARED { 0 } else { 1 + sizeof_varint(*(&self.Category) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserId != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.UserId))?; }
        if self.LastId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.LastId))?; }
        if self.Category != pb_enum::CategoryEnum::Category_RESHARED { w.write_with_tag(16, |w| w.write_enum(*&self.Category as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub PostViewList: Vec<pb_views::PB_PostView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.PostViewList.push(r.read_message::<pb_views::PB_PostView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.PostViewList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.PostViewList { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetLikesPage { }

impl<'a> MessageRead<'a> for GetLikesPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetLikesPage { }

pub mod mod_GetLikesPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
    pub Limit: i32,
    pub LastId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(16) => msg.Limit = r.read_int32(bytes)?,
                Ok(24) => msg.LastId = r.read_int64(bytes)?,
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
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.Limit != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.Limit))?; }
        if self.LastId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.LastId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub UserViewRowifyList: Vec<pb_views::PB_UserViewRowify>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(18) => msg.UserViewRowifyList.push(r.read_message::<pb_views::PB_UserViewRowify>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.UserViewRowifyList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.UserViewRowifyList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetFollowersPage { }

impl<'a> MessageRead<'a> for GetFollowersPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetFollowersPage { }

pub mod mod_GetFollowersPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
    pub Limit: i32,
    pub LastId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(16) => msg.Limit = r.read_int32(bytes)?,
                Ok(24) => msg.LastId = r.read_int64(bytes)?,
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
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.Limit != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.Limit))?; }
        if self.LastId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.LastId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub UserViewRowifyList: Vec<pb_views::PB_UserViewRowify>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(18) => msg.UserViewRowifyList.push(r.read_message::<pb_views::PB_UserViewRowify>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.UserViewRowifyList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.UserViewRowifyList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetFollowingsPage { }

impl<'a> MessageRead<'a> for GetFollowingsPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetFollowingsPage { }

pub mod mod_GetFollowingsPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
    pub Limit: i32,
    pub LastId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(16) => msg.Limit = r.read_int32(bytes)?,
                Ok(24) => msg.LastId = r.read_int64(bytes)?,
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
        + if self.PostId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.PostId) as u64) }
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.Limit != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.Limit))?; }
        if self.LastId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.LastId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub UserViewRowifyList: Vec<pb_views::PB_UserViewRowify>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(18) => msg.UserViewRowifyList.push(r.read_message::<pb_views::PB_UserViewRowify>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.UserViewRowifyList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.UserViewRowifyList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetNotifiesPage { }

impl<'a> MessageRead<'a> for GetNotifiesPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetNotifiesPage { }

pub mod mod_GetNotifiesPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Limit: i32,
    pub LastId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.Limit = r.read_int32(bytes)?,
                Ok(24) => msg.LastId = r.read_int64(bytes)?,
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
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Limit != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.Limit))?; }
        if self.LastId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.LastId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub NotifyViewList: Vec<pb_views::PB_NotifyView>,
    pub RemoveIdsList: Vec<i64>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(18) => msg.NotifyViewList.push(r.read_message::<pb_views::PB_NotifyView>(bytes)?),
                Ok(24) => msg.RemoveIdsList.push(r.read_int64(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.NotifyViewList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.RemoveIdsList.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.NotifyViewList { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.RemoveIdsList { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetUserActionsPage { }

impl<'a> MessageRead<'a> for GetUserActionsPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetUserActionsPage { }

pub mod mod_GetUserActionsPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Limit: i32,
    pub LastId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.Limit = r.read_int32(bytes)?,
                Ok(24) => msg.LastId = r.read_int64(bytes)?,
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
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Limit != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.Limit))?; }
        if self.LastId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.LastId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub ActionViewList: Vec<pb_views::PB_ActionView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(18) => msg.ActionViewList.push(r.read_message::<pb_views::PB_ActionView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.ActionViewList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.ActionViewList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetPromotedPostsPage { }

impl<'a> MessageRead<'a> for GetPromotedPostsPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetPromotedPostsPage { }

pub mod mod_GetPromotedPostsPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Limit: i32,
    pub LastId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.Limit = r.read_int32(bytes)?,
                Ok(24) => msg.LastId = r.read_int64(bytes)?,
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
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Limit != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.Limit))?; }
        if self.LastId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.LastId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub PostViewRowifyList: Vec<pb_views::PB_PostViewRowify>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(18) => msg.PostViewRowifyList.push(r.read_message::<pb_views::PB_PostViewRowify>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.PostViewRowifyList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.PostViewRowifyList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetSuggestedUsersPage { }

impl<'a> MessageRead<'a> for GetSuggestedUsersPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetSuggestedUsersPage { }

pub mod mod_GetSuggestedUsersPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Limit: i32,
    pub LastId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.Limit = r.read_int32(bytes)?,
                Ok(24) => msg.LastId = r.read_int64(bytes)?,
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
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Limit != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.Limit))?; }
        if self.LastId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.LastId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub UserViewRowifyList: Vec<pb_views::PB_UserViewRowify>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(18) => msg.UserViewRowifyList.push(r.read_message::<pb_views::PB_UserViewRowify>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.UserViewRowifyList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.UserViewRowifyList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetSuggestedTagsPage { }

impl<'a> MessageRead<'a> for GetSuggestedTagsPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetSuggestedTagsPage { }

pub mod mod_GetSuggestedTagsPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Limit: i32,
    pub LastId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.Limit = r.read_int32(bytes)?,
                Ok(24) => msg.LastId = r.read_int64(bytes)?,
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
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Limit != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.Limit))?; }
        if self.LastId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.LastId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub TopTagWithSamplePostsList: Vec<pb_views::PB_TopTagWithSamplePosts>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(18) => msg.TopTagWithSamplePostsList.push(r.read_message::<pb_views::PB_TopTagWithSamplePosts>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.TopTagWithSamplePostsList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.TopTagWithSamplePostsList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetLastPostsPage { }

impl<'a> MessageRead<'a> for GetLastPostsPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetLastPostsPage { }

pub mod mod_GetLastPostsPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Limit: i32,
    pub LastId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.Limit = r.read_int32(bytes)?,
                Ok(24) => msg.LastId = r.read_int64(bytes)?,
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
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Limit != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.Limit))?; }
        if self.LastId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.LastId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub PostViewList: Vec<pb_views::PB_PostView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(18) => msg.PostViewList.push(r.read_message::<pb_views::PB_PostView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.PostViewList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.PostViewList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetLastTagPage { }

impl<'a> MessageRead<'a> for GetLastTagPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GetLastTagPage { }

pub mod mod_GetLastTagPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Tag: String,
    pub Limit: i32,
    pub LastId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Tag = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.Limit = r.read_int32(bytes)?,
                Ok(24) => msg.LastId = r.read_int64(bytes)?,
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
        + if self.Tag == String::default() { 0 } else { 1 + sizeof_len((&self.Tag).len()) }
        + if self.Limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Limit) as u64) }
        + if self.LastId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.LastId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Tag != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.Tag))?; }
        if self.Limit != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.Limit))?; }
        if self.LastId != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.LastId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub PostViewList: Vec<pb_views::PB_PostView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(18) => msg.PostViewList.push(r.read_message::<pb_views::PB_PostView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.PostViewList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.PostViewList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SearchTagsPage { }

impl<'a> MessageRead<'a> for SearchTagsPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for SearchTagsPage { }

pub mod mod_SearchTagsPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Query: String,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Query = r.read_string(bytes)?.to_owned(),
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
        + if self.Query == String::default() { 0 } else { 1 + sizeof_len((&self.Query).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Query != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.Query))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub TagViewList: Vec<pb_views::PB_TagView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(18) => msg.TagViewList.push(r.read_message::<pb_views::PB_TagView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.TagViewList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.TagViewList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SearchUsersPage { }

impl<'a> MessageRead<'a> for SearchUsersPage {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for SearchUsersPage { }

pub mod mod_SearchUsersPage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Query: String,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Query = r.read_string(bytes)?.to_owned(),
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
        + if self.Query == String::default() { 0 } else { 1 + sizeof_len((&self.Query).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Query != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.Query))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Error: Option<pb_views::PB_Error>,
    pub UserViewList: Vec<pb_views::PB_UserView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(18) => msg.UserViewList.push(r.read_message::<pb_views::PB_UserView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.UserViewList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.Error { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.UserViewList { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

}


