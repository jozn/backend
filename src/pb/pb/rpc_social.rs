// Automatically generated rust module for 'rpc_social.proto' file

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
pub struct AddCommentParam {
    pub text: String,
}

impl<'a> MessageRead<'a> for AddCommentParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for AddCommentParam {
    fn get_size(&self) -> usize {
        0
        + if self.text == String::default() { 0 } else { 1 + sizeof_len((&self.text).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.text != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.text))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddCommentResponse {
    pub done: bool,
    pub text: String,
}

impl<'a> MessageRead<'a> for AddCommentResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.done = r.read_bool(bytes)?,
                Ok(18) => msg.text = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for AddCommentResponse {
    fn get_size(&self) -> usize {
        0
        + if self.done == false { 0 } else { 1 + sizeof_varint(*(&self.done) as u64) }
        + if self.text == String::default() { 0 } else { 1 + sizeof_len((&self.text).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.done != false { w.write_with_tag(8, |w| w.write_bool(*&self.done))?; }
        if self.text != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.text))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteCommentParam { }

impl<'a> MessageRead<'a> for DeleteCommentParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for DeleteCommentParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteCommentResponse { }

impl<'a> MessageRead<'a> for DeleteCommentResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for DeleteCommentResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct EditCommentParam { }

impl<'a> MessageRead<'a> for EditCommentParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for EditCommentParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct EditCommentResponse { }

impl<'a> MessageRead<'a> for EditCommentResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for EditCommentResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LikeCommentParam { }

impl<'a> MessageRead<'a> for LikeCommentParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for LikeCommentParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LikeCommentResponse { }

impl<'a> MessageRead<'a> for LikeCommentResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for LikeCommentResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddSeenPostsParam { }

impl<'a> MessageRead<'a> for AddSeenPostsParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for AddSeenPostsParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddSeenPostsResponse { }

impl<'a> MessageRead<'a> for AddSeenPostsResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for AddSeenPostsResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LikePostParam { }

impl<'a> MessageRead<'a> for LikePostParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for LikePostParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LikePostResponse { }

impl<'a> MessageRead<'a> for LikePostResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for LikePostResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnLikePostParam { }

impl<'a> MessageRead<'a> for UnLikePostParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UnLikePostParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnLikePostResponse { }

impl<'a> MessageRead<'a> for UnLikePostResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UnLikePostResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FollowChannelParam { }

impl<'a> MessageRead<'a> for FollowChannelParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for FollowChannelParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FollowChannelResponse { }

impl<'a> MessageRead<'a> for FollowChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for FollowChannelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnFollowChannelParam { }

impl<'a> MessageRead<'a> for UnFollowChannelParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UnFollowChannelParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnFollowChannelResponse { }

impl<'a> MessageRead<'a> for UnFollowChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UnFollowChannelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PinChannelParam { }

impl<'a> MessageRead<'a> for PinChannelParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for PinChannelParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PinChannelResponse { }

impl<'a> MessageRead<'a> for PinChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for PinChannelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnPinChannelParam { }

impl<'a> MessageRead<'a> for UnPinChannelParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UnPinChannelParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnPinChannelResponse { }

impl<'a> MessageRead<'a> for UnPinChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UnPinChannelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockChannelParam { }

impl<'a> MessageRead<'a> for BlockChannelParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for BlockChannelParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockChannelResponse { }

impl<'a> MessageRead<'a> for BlockChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for BlockChannelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnBlockChannelParam { }

impl<'a> MessageRead<'a> for UnBlockChannelParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UnBlockChannelParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnBlockChannelResponse { }

impl<'a> MessageRead<'a> for UnBlockChannelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UnBlockChannelResponse { }


