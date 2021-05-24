// Automatically generated rust module for 'pb_rpc_user.proto' file

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
pub struct RPC_User_Types { }

impl<'a> MessageRead<'a> for RPC_User_Types {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RPC_User_Types { }

pub mod mod_RPC_User_Types {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateAbout { }

impl<'a> MessageRead<'a> for UpdateAbout {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UpdateAbout { }

pub mod mod_UpdateAbout {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub NewAbout: String,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.NewAbout = r.read_string(bytes)?.to_owned(),
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
        + if self.NewAbout == String::default() { 0 } else { 1 + sizeof_len((&self.NewAbout).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.NewAbout != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.NewAbout))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
    pub SelfUserView: Option<pb_views::PB_SelfUserView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.SelfUserView = Some(r.read_message::<pb_views::PB_SelfUserView>(bytes)?),
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
        + self.SelfUserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.SelfUserView { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateUserName { }

impl<'a> MessageRead<'a> for UpdateUserName {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UpdateUserName { }

pub mod mod_UpdateUserName {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub NewUserName: String,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.NewUserName = r.read_string(bytes)?.to_owned(),
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
        + if self.NewUserName == String::default() { 0 } else { 1 + sizeof_len((&self.NewUserName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.NewUserName != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.NewUserName))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
    pub SelfUserView: Option<pb_views::PB_SelfUserView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.SelfUserView = Some(r.read_message::<pb_views::PB_SelfUserView>(bytes)?),
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
        + self.SelfUserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.SelfUserView { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChangeAvatar { }

impl<'a> MessageRead<'a> for ChangeAvatar {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChangeAvatar { }

pub mod mod_ChangeAvatar {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub None_pb: bool,
    pub ImageData2: Vec<u8>,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.None_pb = r.read_bool(bytes)?,
                Ok(18) => msg.ImageData2 = r.read_bytes(bytes)?.to_owned(),
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
        + if self.None_pb == false { 0 } else { 1 + sizeof_varint(*(&self.None_pb) as u64) }
        + if self.ImageData2 == vec![] { 0 } else { 1 + sizeof_len((&self.ImageData2).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.None_pb != false { w.write_with_tag(8, |w| w.write_bool(*&self.None_pb))?; }
        if self.ImageData2 != vec![] { w.write_with_tag(18, |w| w.write_bytes(&**&self.ImageData2))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
    pub SelfUserView: Option<pb_views::PB_SelfUserView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.SelfUserView = Some(r.read_message::<pb_views::PB_SelfUserView>(bytes)?),
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
        + self.SelfUserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.SelfUserView { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChangePrivacy { }

impl<'a> MessageRead<'a> for ChangePrivacy {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChangePrivacy { }

pub mod mod_ChangePrivacy {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Level: pb_views::ProfilePrivacyLevelEnum,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Level = r.read_enum(bytes)?,
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
        + if self.Level == pb_views::ProfilePrivacyLevelEnum::NONE { 0 } else { 1 + sizeof_varint(*(&self.Level) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Level != pb_views::ProfilePrivacyLevelEnum::NONE { w.write_with_tag(8, |w| w.write_enum(*&self.Level as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Error: Option<pb_views::PB_Error>,
    pub SelfUserView: Option<pb_views::PB_SelfUserView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Error = Some(r.read_message::<pb_views::PB_Error>(bytes)?),
                Ok(26) => msg.SelfUserView = Some(r.read_message::<pb_views::PB_SelfUserView>(bytes)?),
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
        + self.SelfUserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if let Some(ref s) = self.Error { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.SelfUserView { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

}


