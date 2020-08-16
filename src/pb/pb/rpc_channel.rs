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
pub struct ChannelCreateParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelCreateParam {
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

impl MessageWrite for ChannelCreateParam {
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
pub struct ChannelCreateResponse { }

impl<'a> MessageRead<'a> for ChannelCreateResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelCreateResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelEditParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelEditParam {
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

impl MessageWrite for ChannelEditParam {
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
pub struct ChannelEditResponse { }

impl<'a> MessageRead<'a> for ChannelEditResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelEditResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelDeleteParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelDeleteParam {
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

impl MessageWrite for ChannelDeleteParam {
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
pub struct ChannelDeleteResponse { }

impl<'a> MessageRead<'a> for ChannelDeleteResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelDeleteResponse { }

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
pub struct ChannelChangePrivacyParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelChangePrivacyParam {
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

impl MessageWrite for ChannelChangePrivacyParam {
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
pub struct ChannelChangePrivacyResponse { }

impl<'a> MessageRead<'a> for ChannelChangePrivacyResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelChangePrivacyResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelGetFullMessageParam {
    pub channel_id: u32,
}

impl<'a> MessageRead<'a> for ChannelGetFullMessageParam {
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

impl MessageWrite for ChannelGetFullMessageParam {
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
pub struct ChannelGetFullMessageResponse { }

impl<'a> MessageRead<'a> for ChannelGetFullMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelGetFullMessageResponse { }


