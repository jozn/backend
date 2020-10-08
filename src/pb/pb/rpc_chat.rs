// Automatically generated rust module for 'rpc_chat.proto' file

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
pub struct ChatSendMessageParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for ChatSendMessageParam {
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

impl MessageWrite for ChatSendMessageParam {
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
pub struct ChatSendMessageResponse { }

impl<'a> MessageRead<'a> for ChatSendMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatSendMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatEditMessageParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for ChatEditMessageParam {
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

impl MessageWrite for ChatEditMessageParam {
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
pub struct ChatEditMessageResponse { }

impl<'a> MessageRead<'a> for ChatEditMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatEditMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatDeleteMessagesParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for ChatDeleteMessagesParam {
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

impl MessageWrite for ChatDeleteMessagesParam {
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
pub struct ChatDeleteMessagesResponse { }

impl<'a> MessageRead<'a> for ChatDeleteMessagesResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatDeleteMessagesResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatDeleteHistoryParam { }

impl<'a> MessageRead<'a> for ChatDeleteHistoryParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatDeleteHistoryParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatDeleteHistoryResponse { }

impl<'a> MessageRead<'a> for ChatDeleteHistoryResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatDeleteHistoryResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatAvatarChangeParam { }

impl<'a> MessageRead<'a> for ChatAvatarChangeParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatAvatarChangeParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatAvatarChangeResponse { }

impl<'a> MessageRead<'a> for ChatAvatarChangeResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatAvatarChangeResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatSendDoingActionParam { }

impl<'a> MessageRead<'a> for ChatSendDoingActionParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatSendDoingActionParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatSendDoingActionResponse { }

impl<'a> MessageRead<'a> for ChatSendDoingActionResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatSendDoingActionResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatReportChatParam { }

impl<'a> MessageRead<'a> for ChatReportChatParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatReportChatParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatReportChatResponse { }

impl<'a> MessageRead<'a> for ChatReportChatResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatReportChatResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatGetFullMessageParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for ChatGetFullMessageParam {
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

impl MessageWrite for ChatGetFullMessageParam {
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
pub struct ChatGetFullMessageResponse { }

impl<'a> MessageRead<'a> for ChatGetFullMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatGetFullMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatGetMessagesListParam { }

impl<'a> MessageRead<'a> for ChatGetMessagesListParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatGetMessagesListParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatGetMessagesListResponse { }

impl<'a> MessageRead<'a> for ChatGetMessagesListResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatGetMessagesListResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatGetMediaListParam { }

impl<'a> MessageRead<'a> for ChatGetMediaListParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatGetMediaListParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatGetMediaListResponse { }

impl<'a> MessageRead<'a> for ChatGetMediaListResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChatGetMediaListResponse { }


