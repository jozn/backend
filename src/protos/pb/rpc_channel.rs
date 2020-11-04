// Automatically generated rust module for 'rpc_channel.proto' file

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
pub struct ChannelAvatarAddParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelAvatarAddParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.channel_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelAvatarAddParam {
    fn get_size(&self) -> usize {
        0
        + if self.channel_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.channel_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.channel_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelAvatarAddResponse { }

impl<'a> MessageRead<'a> for ChannelAvatarAddResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelAvatarAddResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelAvatarDeleteParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelAvatarDeleteParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.channel_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelAvatarDeleteParam {
    fn get_size(&self) -> usize {
        0
        + if self.channel_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.channel_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.channel_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelAvatarDeleteResponse { }

impl<'a> MessageRead<'a> for ChannelAvatarDeleteResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelAvatarDeleteResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelAvatarGetListParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelAvatarGetListParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.channel_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelAvatarGetListParam {
    fn get_size(&self) -> usize {
        0
        + if self.channel_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.channel_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.channel_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelAvatarGetListResponse { }

impl<'a> MessageRead<'a> for ChannelAvatarGetListResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelAvatarGetListResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelSendMessageParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelSendMessageParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.channel_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelSendMessageParam {
    fn get_size(&self) -> usize {
        0
        + if self.channel_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.channel_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.channel_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelSendMessageResponse { }

impl<'a> MessageRead<'a> for ChannelSendMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelSendMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelEditMessageParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelEditMessageParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.channel_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelEditMessageParam {
    fn get_size(&self) -> usize {
        0
        + if self.channel_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.channel_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.channel_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelEditMessageResponse { }

impl<'a> MessageRead<'a> for ChannelEditMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelEditMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelPinMessageParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelPinMessageParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.channel_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelPinMessageParam {
    fn get_size(&self) -> usize {
        0
        + if self.channel_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.channel_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.channel_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelPinMessageResponse { }

impl<'a> MessageRead<'a> for ChannelPinMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelPinMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelUnPinMessageParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelUnPinMessageParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.channel_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelUnPinMessageParam {
    fn get_size(&self) -> usize {
        0
        + if self.channel_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.channel_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.channel_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelUnPinMessageResponse { }

impl<'a> MessageRead<'a> for ChannelUnPinMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelUnPinMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelDeleteMessageParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelDeleteMessageParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.channel_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelDeleteMessageParam {
    fn get_size(&self) -> usize {
        0
        + if self.channel_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.channel_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.channel_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelDeleteMessageResponse { }

impl<'a> MessageRead<'a> for ChannelDeleteMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelDeleteMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelSetDraftParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelSetDraftParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.channel_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelSetDraftParam {
    fn get_size(&self) -> usize {
        0
        + if self.channel_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.channel_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.channel_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelSetDraftResponse { }

impl<'a> MessageRead<'a> for ChannelSetDraftResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelSetDraftResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelCreateChannelParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for ChannelCreateChannelParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelCreateChannelParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelCreateChannelResponse { }

impl<'a> MessageRead<'a> for ChannelCreateChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelCreateChannelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelEditChannelParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for ChannelEditChannelParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelEditChannelParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelEditChannelResponse { }

impl<'a> MessageRead<'a> for ChannelEditChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelEditChannelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelDeleteChannelParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for ChannelDeleteChannelParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelDeleteChannelParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelDeleteChannelResponse { }

impl<'a> MessageRead<'a> for ChannelDeleteChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelDeleteChannelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelAddAuthorParam { }

impl<'a> MessageRead<'a> for ChannelAddAuthorParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelAddAuthorParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelAddAuthorResponse { }

impl<'a> MessageRead<'a> for ChannelAddAuthorResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelAddAuthorResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelChangeAuthorPermissionParam { }

impl<'a> MessageRead<'a> for ChannelChangeAuthorPermissionParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelChangeAuthorPermissionParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelChangeAuthorPermissionResponse { }

impl<'a> MessageRead<'a> for ChannelChangeAuthorPermissionResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelChangeAuthorPermissionResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelRemoveAuthorParam { }

impl<'a> MessageRead<'a> for ChannelRemoveAuthorParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelRemoveAuthorParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelRemoveAuthorResponse { }

impl<'a> MessageRead<'a> for ChannelRemoveAuthorResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelRemoveAuthorResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelFollowChannelParam { }

impl<'a> MessageRead<'a> for ChannelFollowChannelParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelFollowChannelParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelFollowChannelResponse { }

impl<'a> MessageRead<'a> for ChannelFollowChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelFollowChannelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelUnFollowChannelParam { }

impl<'a> MessageRead<'a> for ChannelUnFollowChannelParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelUnFollowChannelParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelUnFollowChannelResponse { }

impl<'a> MessageRead<'a> for ChannelUnFollowChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelUnFollowChannelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelRemoveFollowersParam { }

impl<'a> MessageRead<'a> for ChannelRemoveFollowersParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelRemoveFollowersParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelRemoveFollowersResponse { }

impl<'a> MessageRead<'a> for ChannelRemoveFollowersResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelRemoveFollowersResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelSubscribeParam { }

impl<'a> MessageRead<'a> for ChannelSubscribeParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelSubscribeParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelSubscribeResponse { }

impl<'a> MessageRead<'a> for ChannelSubscribeResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelSubscribeResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelUnSubscribeParam { }

impl<'a> MessageRead<'a> for ChannelUnSubscribeParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelUnSubscribeParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelUnSubscribeResponse { }

impl<'a> MessageRead<'a> for ChannelUnSubscribeResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelUnSubscribeResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelRemoveSubscribersParam { }

impl<'a> MessageRead<'a> for ChannelRemoveSubscribersParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelRemoveSubscribersParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelRemoveSubscribersResponse { }

impl<'a> MessageRead<'a> for ChannelRemoveSubscribersResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelRemoveSubscribersResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelChangePrivacyParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for ChannelChangePrivacyParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelChangePrivacyParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelChangePrivacyResponse { }

impl<'a> MessageRead<'a> for ChannelChangePrivacyResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelChangePrivacyResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelChangeDefaultPermissionParam { }

impl<'a> MessageRead<'a> for ChannelChangeDefaultPermissionParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelChangeDefaultPermissionParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelChangeDefaultPermissionResponse { }

impl<'a> MessageRead<'a> for ChannelChangeDefaultPermissionResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelChangeDefaultPermissionResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelRevokeLinkParam { }

impl<'a> MessageRead<'a> for ChannelRevokeLinkParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelRevokeLinkParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelRevokeLinkResponse { }

impl<'a> MessageRead<'a> for ChannelRevokeLinkResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelRevokeLinkResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelChangeUsernameParam { }

impl<'a> MessageRead<'a> for ChannelChangeUsernameParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelChangeUsernameParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelChangeUsernameResponse { }

impl<'a> MessageRead<'a> for ChannelChangeUsernameResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelChangeUsernameResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelBlockChannelParam { }

impl<'a> MessageRead<'a> for ChannelBlockChannelParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelBlockChannelParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelBlockChannelResponse { }

impl<'a> MessageRead<'a> for ChannelBlockChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelBlockChannelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelDeleteMessagesParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for ChannelDeleteMessagesParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelDeleteMessagesParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelDeleteMessagesResponse { }

impl<'a> MessageRead<'a> for ChannelDeleteMessagesResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelDeleteMessagesResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelClearHistoryParam { }

impl<'a> MessageRead<'a> for ChannelClearHistoryParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelClearHistoryParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelClearHistoryResponse { }

impl<'a> MessageRead<'a> for ChannelClearHistoryResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelClearHistoryResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelAvatarChangeParam { }

impl<'a> MessageRead<'a> for ChannelAvatarChangeParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelAvatarChangeParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelAvatarChangeResponse { }

impl<'a> MessageRead<'a> for ChannelAvatarChangeResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelAvatarChangeResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelSendDoingActionParam { }

impl<'a> MessageRead<'a> for ChannelSendDoingActionParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelSendDoingActionParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelSendDoingActionResponse { }

impl<'a> MessageRead<'a> for ChannelSendDoingActionResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelSendDoingActionResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelReportChannelParam { }

impl<'a> MessageRead<'a> for ChannelReportChannelParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelReportChannelParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelReportChannelResponse { }

impl<'a> MessageRead<'a> for ChannelReportChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelReportChannelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelReportMessageParam { }

impl<'a> MessageRead<'a> for ChannelReportMessageParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelReportMessageParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelReportMessageResponse { }

impl<'a> MessageRead<'a> for ChannelReportMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelReportMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetFullParam { }

impl<'a> MessageRead<'a> for ChannelGetFullParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetFullParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetFullResponse { }

impl<'a> MessageRead<'a> for ChannelGetFullResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetFullResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetMessagesListParam { }

impl<'a> MessageRead<'a> for ChannelGetMessagesListParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetMessagesListParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetMessagesListResponse { }

impl<'a> MessageRead<'a> for ChannelGetMessagesListResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetMessagesListResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetMediaListParam { }

impl<'a> MessageRead<'a> for ChannelGetMediaListParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetMediaListParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetMediaListResponse { }

impl<'a> MessageRead<'a> for ChannelGetMediaListResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetMediaListResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetAuthorsParam { }

impl<'a> MessageRead<'a> for ChannelGetAuthorsParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetAuthorsParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetAuthorsResponse { }

impl<'a> MessageRead<'a> for ChannelGetAuthorsResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetAuthorsResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetFollowersParam { }

impl<'a> MessageRead<'a> for ChannelGetFollowersParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetFollowersParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetFollowersResponse { }

impl<'a> MessageRead<'a> for ChannelGetFollowersResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetFollowersResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetFollowingsParam { }

impl<'a> MessageRead<'a> for ChannelGetFollowingsParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetFollowingsParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetFollowingsResponse { }

impl<'a> MessageRead<'a> for ChannelGetFollowingsResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetFollowingsResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetSubscribersParam { }

impl<'a> MessageRead<'a> for ChannelGetSubscribersParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetSubscribersParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetSubscribersResponse { }

impl<'a> MessageRead<'a> for ChannelGetSubscribersResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetSubscribersResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelBlockedParam { }

impl<'a> MessageRead<'a> for ChannelBlockedParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelBlockedParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelBlockedResponse { }

impl<'a> MessageRead<'a> for ChannelBlockedResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelBlockedResponse { }


