// Automatically generated rust module for 'pb_rpc_social.proto' file

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
pub struct RPC_Social_Types { }

impl<'a> MessageRead<'a> for RPC_Social_Types {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RPC_Social_Types { }

pub mod mod_RPC_Social_Types {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddComment { }

impl<'a> MessageRead<'a> for AddComment {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for AddComment { }

pub mod mod_AddComment {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
    pub Text: String,
    pub RandHash: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(18) => msg.Text = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.RandHash = r.read_int64(bytes)?,
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
        + if self.Text == String::default() { 0 } else { 1 + sizeof_len((&self.Text).len()) }
        + if self.RandHash == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.RandHash) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.Text != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Text))?; }
        if self.RandHash != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.RandHash))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
    pub Comment: Option<pb_views::PB_CommentView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.Comment = Some(r.read_message::<pb_views::PB_CommentView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.Comment.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.Comment { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteComment { }

impl<'a> MessageRead<'a> for DeleteComment {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for DeleteComment { }

pub mod mod_DeleteComment {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
    pub CommentId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(16) => msg.CommentId = r.read_int64(bytes)?,
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
        + if self.CommentId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.CommentId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.CommentId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.CommentId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct EditComment { }

impl<'a> MessageRead<'a> for EditComment {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for EditComment { }

pub mod mod_EditComment {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
    pub CommentId: i64,
    pub Text: String,
    pub RandHash: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(16) => msg.CommentId = r.read_int64(bytes)?,
                Ok(26) => msg.Text = r.read_string(bytes)?.to_owned(),
                Ok(32) => msg.RandHash = r.read_int64(bytes)?,
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
        + if self.CommentId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.CommentId) as u64) }
        + if self.Text == String::default() { 0 } else { 1 + sizeof_len((&self.Text).len()) }
        + if self.RandHash == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.RandHash) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.CommentId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.CommentId))?; }
        if self.Text != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.Text))?; }
        if self.RandHash != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.RandHash))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
    pub Comment: Option<pb_views::PB_CommentView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.Comment = Some(r.read_message::<pb_views::PB_CommentView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.Comment.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.Comment { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LikeComment { }

impl<'a> MessageRead<'a> for LikeComment {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for LikeComment { }

pub mod mod_LikeComment {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
    pub CommentId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(16) => msg.CommentId = r.read_int64(bytes)?,
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
        + if self.CommentId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.CommentId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.CommentId != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.CommentId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddPost { }

impl<'a> MessageRead<'a> for AddPost {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for AddPost { }

pub mod mod_AddPost {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Text: String,
    pub ImageBlob: Vec<u8>,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Text = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.ImageBlob = r.read_bytes(bytes)?.to_owned(),
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
        + if self.Text == String::default() { 0 } else { 1 + sizeof_len((&self.Text).len()) }
        + if self.ImageBlob == vec![] { 0 } else { 1 + sizeof_len((&self.ImageBlob).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Text != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.Text))?; }
        if self.ImageBlob != vec![] { w.write_with_tag(18, |w| w.write_bytes(&**&self.ImageBlob))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
    pub PostView: Option<pb_views::PB_PostView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.PostView = Some(r.read_message::<pb_views::PB_PostView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.PostView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.PostView { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct EditPost { }

impl<'a> MessageRead<'a> for EditPost {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for EditPost { }

pub mod mod_EditPost {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
    pub Text: String,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
                Ok(18) => msg.Text = r.read_string(bytes)?.to_owned(),
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
        + if self.Text == String::default() { 0 } else { 1 + sizeof_len((&self.Text).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        if self.Text != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Text))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
    pub PostView: Option<pb_views::PB_PostView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.PostView = Some(r.read_message::<pb_views::PB_PostView>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.PostView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.PostView { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeletePost { }

impl<'a> MessageRead<'a> for DeletePost {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for DeletePost { }

pub mod mod_DeletePost {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
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
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ArchivePost { }

impl<'a> MessageRead<'a> for ArchivePost {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ArchivePost { }

pub mod mod_ArchivePost {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
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
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PromotePost { }

impl<'a> MessageRead<'a> for PromotePost {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for PromotePost { }

pub mod mod_PromotePost {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
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
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LikePost { }

impl<'a> MessageRead<'a> for LikePost {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for LikePost { }

pub mod mod_LikePost {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
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
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnLikePost { }

impl<'a> MessageRead<'a> for UnLikePost {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UnLikePost { }

pub mod mod_UnLikePost {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub PostId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.PostId = r.read_int64(bytes)?,
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
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.PostId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.PostId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FollowUser { }

impl<'a> MessageRead<'a> for FollowUser {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for FollowUser { }

pub mod mod_FollowUser {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub UserId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.UserId = r.read_int64(bytes)?,
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
        + if self.UserId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.UserId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnFollowUser { }

impl<'a> MessageRead<'a> for UnFollowUser {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UnFollowUser { }

pub mod mod_UnFollowUser {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub UserId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.UserId = r.read_int64(bytes)?,
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
        + if self.UserId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.UserId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockUser { }

impl<'a> MessageRead<'a> for BlockUser {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for BlockUser { }

pub mod mod_BlockUser {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub UserId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.UserId = r.read_int64(bytes)?,
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
        + if self.UserId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.UserId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnBlockUser { }

impl<'a> MessageRead<'a> for UnBlockUser {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UnBlockUser { }

pub mod mod_UnBlockUser {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub UserId: i64,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.UserId = r.read_int64(bytes)?,
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
        + if self.UserId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.UserId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.UserId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddSeenPosts { }

impl<'a> MessageRead<'a> for AddSeenPosts {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for AddSeenPosts { }

pub mod mod_AddSeenPosts {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub RandHash: i64,
    pub PostIds: Vec<i64>,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.RandHash = r.read_int64(bytes)?,
                Ok(16) => msg.PostIds.push(r.read_int64(bytes)?),
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
        + if self.RandHash == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.RandHash) as u64) }
        + self.PostIds.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.RandHash != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.RandHash))?; }
        for s in &self.PostIds { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
    pub RandHash: i64,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(24) => msg.RandHash = r.read_int64(bytes)?,
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
        + self.Error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.RandHash == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.RandHash) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.RandHash != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.RandHash))?; }
        Ok(())
    }
}

}

}


