// Automatically generated rust module for 'store_v0.proto' file

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
pub struct Action {
    pub action_id: u64,
    pub actor_user_id: u32,
    pub action_type: i32,
    pub peer_user_id: u32,
    pub message_id: u64,
    pub comment_id: u64,
    pub hash_murm64: i64,
    pub created_time: u32,
}

impl<'a> MessageRead<'a> for Action {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.action_id = r.read_fixed64(bytes)?,
                Ok(16) => msg.actor_user_id = r.read_uint32(bytes)?,
                Ok(24) => msg.action_type = r.read_int32(bytes)?,
                Ok(32) => msg.peer_user_id = r.read_uint32(bytes)?,
                Ok(41) => msg.message_id = r.read_fixed64(bytes)?,
                Ok(49) => msg.comment_id = r.read_fixed64(bytes)?,
                Ok(56) => msg.hash_murm64 = r.read_int64(bytes)?,
                Ok(64) => msg.created_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Action {
    fn get_size(&self) -> usize {
        0
        + if self.action_id == 0u64 { 0 } else { 1 + 8 }
        + if self.actor_user_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.actor_user_id) as u64) }
        + if self.action_type == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.action_type) as u64) }
        + if self.peer_user_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.peer_user_id) as u64) }
        + if self.message_id == 0u64 { 0 } else { 1 + 8 }
        + if self.comment_id == 0u64 { 0 } else { 1 + 8 }
        + if self.hash_murm64 == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.hash_murm64) as u64) }
        + if self.created_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.action_id != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.action_id))?; }
        if self.actor_user_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.actor_user_id))?; }
        if self.action_type != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.action_type))?; }
        if self.peer_user_id != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.peer_user_id))?; }
        if self.message_id != 0u64 { w.write_with_tag(41, |w| w.write_fixed64(*&self.message_id))?; }
        if self.comment_id != 0u64 { w.write_with_tag(49, |w| w.write_fixed64(*&self.comment_id))?; }
        if self.hash_murm64 != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.hash_murm64))?; }
        if self.created_time != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Blocked {
    pub id: u64,
    pub user_id: u32,
    pub blocked_user_id: u32,
    pub created_time: u32,
}

impl<'a> MessageRead<'a> for Blocked {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.id = r.read_fixed64(bytes)?,
                Ok(16) => msg.user_id = r.read_uint32(bytes)?,
                Ok(24) => msg.blocked_user_id = r.read_uint32(bytes)?,
                Ok(32) => msg.created_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Blocked {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u64 { 0 } else { 1 + 8 }
        + if self.user_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.blocked_user_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.blocked_user_id) as u64) }
        + if self.created_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u64 { w.write_with_tag(9, |w| w.write_fixed64(*&self.id))?; }
        if self.user_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.user_id))?; }
        if self.blocked_user_id != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.blocked_user_id))?; }
        if self.created_time != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Comment {
    pub comment_id: u64,
    pub user_id: u32,
    pub message_id: u64,
    pub text: String,
    pub likes_count: i32,
    pub edited: bool,
    pub created_time: u32,
}

impl<'a> MessageRead<'a> for Comment {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.comment_id = r.read_uint64(bytes)?,
                Ok(16) => msg.user_id = r.read_uint32(bytes)?,
                Ok(24) => msg.message_id = r.read_uint64(bytes)?,
                Ok(34) => msg.text = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.likes_count = r.read_int32(bytes)?,
                Ok(48) => msg.edited = r.read_bool(bytes)?,
                Ok(56) => msg.created_time = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Comment {
    fn get_size(&self) -> usize {
        0
        + if self.comment_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.comment_id) as u64) }
        + if self.user_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.message_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.message_id) as u64) }
        + if self.text == String::default() { 0 } else { 1 + sizeof_len((&self.text).len()) }
        + if self.likes_count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.likes_count) as u64) }
        + if self.edited == false { 0 } else { 1 + sizeof_varint(*(&self.edited) as u64) }
        + if self.created_time == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.comment_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.comment_id))?; }
        if self.user_id != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.user_id))?; }
        if self.message_id != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.message_id))?; }
        if self.text != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.text))?; }
        if self.likes_count != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.likes_count))?; }
        if self.edited != false { w.write_with_tag(48, |w| w.write_bool(*&self.edited))?; }
        if self.created_time != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Followed {
    pub id: i64,
    pub user_id: i32,
    pub followed_user_id: i32,
    pub created_time: i32,
}

impl<'a> MessageRead<'a> for Followed {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int64(bytes)?,
                Ok(16) => msg.user_id = r.read_int32(bytes)?,
                Ok(24) => msg.followed_user_id = r.read_int32(bytes)?,
                Ok(32) => msg.created_time = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Followed {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.followed_user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.followed_user_id) as u64) }
        + if self.created_time == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.id))?; }
        if self.user_id != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.user_id))?; }
        if self.followed_user_id != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.followed_user_id))?; }
        if self.created_time != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Like {
    pub id: i64,
    pub post_id: i64,
    pub user_id: i32,
    pub post_type: i32,
    pub created_time: i32,
}

impl<'a> MessageRead<'a> for Like {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int64(bytes)?,
                Ok(16) => msg.post_id = r.read_int64(bytes)?,
                Ok(24) => msg.user_id = r.read_int32(bytes)?,
                Ok(32) => msg.post_type = r.read_int32(bytes)?,
                Ok(40) => msg.created_time = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Like {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.post_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.post_id) as u64) }
        + if self.user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.post_type == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.post_type) as u64) }
        + if self.created_time == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.id))?; }
        if self.post_id != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.post_id))?; }
        if self.user_id != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.user_id))?; }
        if self.post_type != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.post_type))?; }
        if self.created_time != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Contact {
    pub id: i64,
    pub user_id: i32,
    pub client_id: i64,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
}

impl<'a> MessageRead<'a> for Contact {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int64(bytes)?,
                Ok(16) => msg.user_id = r.read_int32(bytes)?,
                Ok(24) => msg.client_id = r.read_int64(bytes)?,
                Ok(34) => msg.phone = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.first_name = r.read_string(bytes)?.to_owned(),
                Ok(50) => msg.last_name = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Contact {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.client_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.client_id) as u64) }
        + if self.phone == String::default() { 0 } else { 1 + sizeof_len((&self.phone).len()) }
        + if self.first_name == String::default() { 0 } else { 1 + sizeof_len((&self.first_name).len()) }
        + if self.last_name == String::default() { 0 } else { 1 + sizeof_len((&self.last_name).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.id))?; }
        if self.user_id != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.user_id))?; }
        if self.client_id != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.client_id))?; }
        if self.phone != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.phone))?; }
        if self.first_name != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.first_name))?; }
        if self.last_name != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.last_name))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Post {
    pub post_id: i64,
    pub user_id: i32,
    pub post_type: i32,
    pub media_id: i64,
    pub file_ref_id: i64,
    pub post_key: String,
    pub text: String,
    pub rich_text: String,
    pub media_count: i32,
    pub shared_to: i32,
    pub disable_comment: i32,
    pub via: i32,
    pub seq: i32,
    pub comments_count: i32,
    pub likes_count: i32,
    pub views_count: i32,
    pub edited_time: i32,
    pub created_time: i32,
}

impl<'a> MessageRead<'a> for Post {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.post_id = r.read_int64(bytes)?,
                Ok(16) => msg.user_id = r.read_int32(bytes)?,
                Ok(24) => msg.post_type = r.read_int32(bytes)?,
                Ok(32) => msg.media_id = r.read_int64(bytes)?,
                Ok(40) => msg.file_ref_id = r.read_int64(bytes)?,
                Ok(50) => msg.post_key = r.read_string(bytes)?.to_owned(),
                Ok(58) => msg.text = r.read_string(bytes)?.to_owned(),
                Ok(66) => msg.rich_text = r.read_string(bytes)?.to_owned(),
                Ok(72) => msg.media_count = r.read_int32(bytes)?,
                Ok(80) => msg.shared_to = r.read_int32(bytes)?,
                Ok(88) => msg.disable_comment = r.read_int32(bytes)?,
                Ok(96) => msg.via = r.read_int32(bytes)?,
                Ok(104) => msg.seq = r.read_int32(bytes)?,
                Ok(112) => msg.comments_count = r.read_int32(bytes)?,
                Ok(120) => msg.likes_count = r.read_int32(bytes)?,
                Ok(128) => msg.views_count = r.read_int32(bytes)?,
                Ok(136) => msg.edited_time = r.read_int32(bytes)?,
                Ok(144) => msg.created_time = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Post {
    fn get_size(&self) -> usize {
        0
        + if self.post_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.post_id) as u64) }
        + if self.user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.post_type == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.post_type) as u64) }
        + if self.media_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.media_id) as u64) }
        + if self.file_ref_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.file_ref_id) as u64) }
        + if self.post_key == String::default() { 0 } else { 1 + sizeof_len((&self.post_key).len()) }
        + if self.text == String::default() { 0 } else { 1 + sizeof_len((&self.text).len()) }
        + if self.rich_text == String::default() { 0 } else { 1 + sizeof_len((&self.rich_text).len()) }
        + if self.media_count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.media_count) as u64) }
        + if self.shared_to == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.shared_to) as u64) }
        + if self.disable_comment == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.disable_comment) as u64) }
        + if self.via == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.via) as u64) }
        + if self.seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.comments_count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.comments_count) as u64) }
        + if self.likes_count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.likes_count) as u64) }
        + if self.views_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.views_count) as u64) }
        + if self.edited_time == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.edited_time) as u64) }
        + if self.created_time == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.post_id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.post_id))?; }
        if self.user_id != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.user_id))?; }
        if self.post_type != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.post_type))?; }
        if self.media_id != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.media_id))?; }
        if self.file_ref_id != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.file_ref_id))?; }
        if self.post_key != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.post_key))?; }
        if self.text != String::default() { w.write_with_tag(58, |w| w.write_string(&**&self.text))?; }
        if self.rich_text != String::default() { w.write_with_tag(66, |w| w.write_string(&**&self.rich_text))?; }
        if self.media_count != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.media_count))?; }
        if self.shared_to != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.shared_to))?; }
        if self.disable_comment != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.disable_comment))?; }
        if self.via != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.via))?; }
        if self.seq != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.seq))?; }
        if self.comments_count != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.comments_count))?; }
        if self.likes_count != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.likes_count))?; }
        if self.views_count != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.views_count))?; }
        if self.edited_time != 0i32 { w.write_with_tag(136, |w| w.write_int32(*&self.edited_time))?; }
        if self.created_time != 0i32 { w.write_with_tag(144, |w| w.write_int32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PostCount {
    pub post_id: i64,
    pub comments_count: i32,
    pub likes_count: i32,
    pub views_count: i64,
    pub re_shared_count: i32,
    pub chat_shared_count: i32,
}

impl<'a> MessageRead<'a> for PostCount {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.post_id = r.read_int64(bytes)?,
                Ok(16) => msg.comments_count = r.read_int32(bytes)?,
                Ok(24) => msg.likes_count = r.read_int32(bytes)?,
                Ok(32) => msg.views_count = r.read_int64(bytes)?,
                Ok(40) => msg.re_shared_count = r.read_int32(bytes)?,
                Ok(48) => msg.chat_shared_count = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PostCount {
    fn get_size(&self) -> usize {
        0
        + if self.post_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.post_id) as u64) }
        + if self.comments_count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.comments_count) as u64) }
        + if self.likes_count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.likes_count) as u64) }
        + if self.views_count == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.views_count) as u64) }
        + if self.re_shared_count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.re_shared_count) as u64) }
        + if self.chat_shared_count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.chat_shared_count) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.post_id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.post_id))?; }
        if self.comments_count != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.comments_count))?; }
        if self.likes_count != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.likes_count))?; }
        if self.views_count != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.views_count))?; }
        if self.re_shared_count != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.re_shared_count))?; }
        if self.chat_shared_count != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.chat_shared_count))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PostPromoted {
    pub promote_id: i32,
    pub post_id: i64,
    pub by_user_id: i32,
    pub post_user_id: i32,
    pub bazzar_uuid: String,
    pub package: String,
    pub end_time: i32,
    pub created_time: i32,
}

impl<'a> MessageRead<'a> for PostPromoted {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.promote_id = r.read_int32(bytes)?,
                Ok(16) => msg.post_id = r.read_int64(bytes)?,
                Ok(24) => msg.by_user_id = r.read_int32(bytes)?,
                Ok(32) => msg.post_user_id = r.read_int32(bytes)?,
                Ok(42) => msg.bazzar_uuid = r.read_string(bytes)?.to_owned(),
                Ok(50) => msg.package = r.read_string(bytes)?.to_owned(),
                Ok(56) => msg.end_time = r.read_int32(bytes)?,
                Ok(64) => msg.created_time = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PostPromoted {
    fn get_size(&self) -> usize {
        0
        + if self.promote_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.promote_id) as u64) }
        + if self.post_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.post_id) as u64) }
        + if self.by_user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.by_user_id) as u64) }
        + if self.post_user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.post_user_id) as u64) }
        + if self.bazzar_uuid == String::default() { 0 } else { 1 + sizeof_len((&self.bazzar_uuid).len()) }
        + if self.package == String::default() { 0 } else { 1 + sizeof_len((&self.package).len()) }
        + if self.end_time == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.end_time) as u64) }
        + if self.created_time == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.promote_id != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.promote_id))?; }
        if self.post_id != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.post_id))?; }
        if self.by_user_id != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.by_user_id))?; }
        if self.post_user_id != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.post_user_id))?; }
        if self.bazzar_uuid != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.bazzar_uuid))?; }
        if self.package != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.package))?; }
        if self.end_time != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.end_time))?; }
        if self.created_time != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Session {
    pub id: i64,
    pub session_uuid: String,
    pub user_id: i32,
    pub last_ip_address: String,
    pub app_version: i32,
    pub active_time: i32,
    pub created_time: i32,
}

impl<'a> MessageRead<'a> for Session {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int64(bytes)?,
                Ok(18) => msg.session_uuid = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.user_id = r.read_int32(bytes)?,
                Ok(34) => msg.last_ip_address = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.app_version = r.read_int32(bytes)?,
                Ok(48) => msg.active_time = r.read_int32(bytes)?,
                Ok(56) => msg.created_time = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Session {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.session_uuid == String::default() { 0 } else { 1 + sizeof_len((&self.session_uuid).len()) }
        + if self.user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.last_ip_address == String::default() { 0 } else { 1 + sizeof_len((&self.last_ip_address).len()) }
        + if self.app_version == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.app_version) as u64) }
        + if self.active_time == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.active_time) as u64) }
        + if self.created_time == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.id))?; }
        if self.session_uuid != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.session_uuid))?; }
        if self.user_id != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.user_id))?; }
        if self.last_ip_address != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.last_ip_address))?; }
        if self.app_version != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.app_version))?; }
        if self.active_time != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.active_time))?; }
        if self.created_time != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NotificationSetting {
    pub user_id: i32,
    pub social_led_on: i32,
    pub social_led_color: String,
    pub reqest_to_follow_you: i32,
    pub followed_you: i32,
    pub accpted_your_follow_request: i32,
    pub your_post_liked: i32,
    pub your_post_commented: i32,
    pub menthened_you_in_post: i32,
    pub menthened_you_in_comment: i32,
    pub your_contacts_joined: i32,
    pub direct_message: i32,
    pub direct_alert: i32,
    pub direct_perview: i32,
    pub direct_led_on: i32,
    pub direct_led_color: i32,
    pub direct_vibrate: i32,
    pub direct_popup: i32,
    pub direct_sound: i32,
    pub direct_priority: i32,
}

impl<'a> MessageRead<'a> for NotificationSetting {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.user_id = r.read_int32(bytes)?,
                Ok(16) => msg.social_led_on = r.read_int32(bytes)?,
                Ok(26) => msg.social_led_color = r.read_string(bytes)?.to_owned(),
                Ok(32) => msg.reqest_to_follow_you = r.read_int32(bytes)?,
                Ok(40) => msg.followed_you = r.read_int32(bytes)?,
                Ok(48) => msg.accpted_your_follow_request = r.read_int32(bytes)?,
                Ok(56) => msg.your_post_liked = r.read_int32(bytes)?,
                Ok(64) => msg.your_post_commented = r.read_int32(bytes)?,
                Ok(72) => msg.menthened_you_in_post = r.read_int32(bytes)?,
                Ok(80) => msg.menthened_you_in_comment = r.read_int32(bytes)?,
                Ok(88) => msg.your_contacts_joined = r.read_int32(bytes)?,
                Ok(96) => msg.direct_message = r.read_int32(bytes)?,
                Ok(104) => msg.direct_alert = r.read_int32(bytes)?,
                Ok(112) => msg.direct_perview = r.read_int32(bytes)?,
                Ok(120) => msg.direct_led_on = r.read_int32(bytes)?,
                Ok(128) => msg.direct_led_color = r.read_int32(bytes)?,
                Ok(136) => msg.direct_vibrate = r.read_int32(bytes)?,
                Ok(144) => msg.direct_popup = r.read_int32(bytes)?,
                Ok(152) => msg.direct_sound = r.read_int32(bytes)?,
                Ok(160) => msg.direct_priority = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for NotificationSetting {
    fn get_size(&self) -> usize {
        0
        + if self.user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.social_led_on == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.social_led_on) as u64) }
        + if self.social_led_color == String::default() { 0 } else { 1 + sizeof_len((&self.social_led_color).len()) }
        + if self.reqest_to_follow_you == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.reqest_to_follow_you) as u64) }
        + if self.followed_you == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.followed_you) as u64) }
        + if self.accpted_your_follow_request == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.accpted_your_follow_request) as u64) }
        + if self.your_post_liked == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.your_post_liked) as u64) }
        + if self.your_post_commented == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.your_post_commented) as u64) }
        + if self.menthened_you_in_post == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.menthened_you_in_post) as u64) }
        + if self.menthened_you_in_comment == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.menthened_you_in_comment) as u64) }
        + if self.your_contacts_joined == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.your_contacts_joined) as u64) }
        + if self.direct_message == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.direct_message) as u64) }
        + if self.direct_alert == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.direct_alert) as u64) }
        + if self.direct_perview == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.direct_perview) as u64) }
        + if self.direct_led_on == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.direct_led_on) as u64) }
        + if self.direct_led_color == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.direct_led_color) as u64) }
        + if self.direct_vibrate == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.direct_vibrate) as u64) }
        + if self.direct_popup == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.direct_popup) as u64) }
        + if self.direct_sound == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.direct_sound) as u64) }
        + if self.direct_priority == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.direct_priority) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.user_id != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.user_id))?; }
        if self.social_led_on != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.social_led_on))?; }
        if self.social_led_color != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.social_led_color))?; }
        if self.reqest_to_follow_you != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.reqest_to_follow_you))?; }
        if self.followed_you != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.followed_you))?; }
        if self.accpted_your_follow_request != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.accpted_your_follow_request))?; }
        if self.your_post_liked != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.your_post_liked))?; }
        if self.your_post_commented != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.your_post_commented))?; }
        if self.menthened_you_in_post != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.menthened_you_in_post))?; }
        if self.menthened_you_in_comment != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.menthened_you_in_comment))?; }
        if self.your_contacts_joined != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.your_contacts_joined))?; }
        if self.direct_message != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.direct_message))?; }
        if self.direct_alert != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.direct_alert))?; }
        if self.direct_perview != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.direct_perview))?; }
        if self.direct_led_on != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.direct_led_on))?; }
        if self.direct_led_color != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.direct_led_color))?; }
        if self.direct_vibrate != 0i32 { w.write_with_tag(136, |w| w.write_int32(*&self.direct_vibrate))?; }
        if self.direct_popup != 0i32 { w.write_with_tag(144, |w| w.write_int32(*&self.direct_popup))?; }
        if self.direct_sound != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.direct_sound))?; }
        if self.direct_priority != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.direct_priority))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sms {
    pub id: i32,
    pub hash: String,
    pub app_uuid: String,
    pub client_phone: String,
    pub genrated_code: i32,
    pub sms_sender_number: String,
    pub sms_send_statues: String,
    pub sms_http_body: String,
    pub err: String,
    pub carrier: String,
    pub country: Vec<u8>,
    pub is_valid_phone: i32,
    pub is_confirmed: i32,
    pub is_login: i32,
    pub is_register: i32,
    pub retried_count: i32,
    pub ttl: i32,
}

impl<'a> MessageRead<'a> for Sms {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int32(bytes)?,
                Ok(18) => msg.hash = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.app_uuid = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.client_phone = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.genrated_code = r.read_int32(bytes)?,
                Ok(50) => msg.sms_sender_number = r.read_string(bytes)?.to_owned(),
                Ok(58) => msg.sms_send_statues = r.read_string(bytes)?.to_owned(),
                Ok(66) => msg.sms_http_body = r.read_string(bytes)?.to_owned(),
                Ok(74) => msg.err = r.read_string(bytes)?.to_owned(),
                Ok(82) => msg.carrier = r.read_string(bytes)?.to_owned(),
                Ok(90) => msg.country = r.read_bytes(bytes)?.to_owned(),
                Ok(96) => msg.is_valid_phone = r.read_int32(bytes)?,
                Ok(104) => msg.is_confirmed = r.read_int32(bytes)?,
                Ok(112) => msg.is_login = r.read_int32(bytes)?,
                Ok(120) => msg.is_register = r.read_int32(bytes)?,
                Ok(128) => msg.retried_count = r.read_int32(bytes)?,
                Ok(136) => msg.ttl = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Sms {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.hash == String::default() { 0 } else { 1 + sizeof_len((&self.hash).len()) }
        + if self.app_uuid == String::default() { 0 } else { 1 + sizeof_len((&self.app_uuid).len()) }
        + if self.client_phone == String::default() { 0 } else { 1 + sizeof_len((&self.client_phone).len()) }
        + if self.genrated_code == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.genrated_code) as u64) }
        + if self.sms_sender_number == String::default() { 0 } else { 1 + sizeof_len((&self.sms_sender_number).len()) }
        + if self.sms_send_statues == String::default() { 0 } else { 1 + sizeof_len((&self.sms_send_statues).len()) }
        + if self.sms_http_body == String::default() { 0 } else { 1 + sizeof_len((&self.sms_http_body).len()) }
        + if self.err == String::default() { 0 } else { 1 + sizeof_len((&self.err).len()) }
        + if self.carrier == String::default() { 0 } else { 1 + sizeof_len((&self.carrier).len()) }
        + if self.country == vec![] { 0 } else { 1 + sizeof_len((&self.country).len()) }
        + if self.is_valid_phone == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.is_valid_phone) as u64) }
        + if self.is_confirmed == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.is_confirmed) as u64) }
        + if self.is_login == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.is_login) as u64) }
        + if self.is_register == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.is_register) as u64) }
        + if self.retried_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.retried_count) as u64) }
        + if self.ttl == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.ttl) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.id))?; }
        if self.hash != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.hash))?; }
        if self.app_uuid != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.app_uuid))?; }
        if self.client_phone != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.client_phone))?; }
        if self.genrated_code != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.genrated_code))?; }
        if self.sms_sender_number != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.sms_sender_number))?; }
        if self.sms_send_statues != String::default() { w.write_with_tag(58, |w| w.write_string(&**&self.sms_send_statues))?; }
        if self.sms_http_body != String::default() { w.write_with_tag(66, |w| w.write_string(&**&self.sms_http_body))?; }
        if self.err != String::default() { w.write_with_tag(74, |w| w.write_string(&**&self.err))?; }
        if self.carrier != String::default() { w.write_with_tag(82, |w| w.write_string(&**&self.carrier))?; }
        if self.country != vec![] { w.write_with_tag(90, |w| w.write_bytes(&**&self.country))?; }
        if self.is_valid_phone != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.is_valid_phone))?; }
        if self.is_confirmed != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.is_confirmed))?; }
        if self.is_login != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.is_login))?; }
        if self.is_register != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.is_register))?; }
        if self.retried_count != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.retried_count))?; }
        if self.ttl != 0i32 { w.write_with_tag(136, |w| w.write_int32(*&self.ttl))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Tag {
    pub tag_id: i64,
    pub name: String,
    pub count: i32,
    pub tag_status_enum: i32,
    pub is_blocked: i32,
    pub group_id: i32,
    pub created_time: i32,
}

impl<'a> MessageRead<'a> for Tag {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.tag_id = r.read_int64(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.count = r.read_int32(bytes)?,
                Ok(32) => msg.tag_status_enum = r.read_int32(bytes)?,
                Ok(40) => msg.is_blocked = r.read_int32(bytes)?,
                Ok(48) => msg.group_id = r.read_int32(bytes)?,
                Ok(56) => msg.created_time = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Tag {
    fn get_size(&self) -> usize {
        0
        + if self.tag_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.tag_id) as u64) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.count) as u64) }
        + if self.tag_status_enum == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.tag_status_enum) as u64) }
        + if self.is_blocked == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.is_blocked) as u64) }
        + if self.group_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
        + if self.created_time == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.tag_id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.tag_id))?; }
        if self.name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.count != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.count))?; }
        if self.tag_status_enum != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.tag_status_enum))?; }
        if self.is_blocked != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.is_blocked))?; }
        if self.group_id != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.group_id))?; }
        if self.created_time != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub user_name_lower: String,
    pub first_name: String,
    pub last_name: String,
    pub is_verified: i32,
    pub avatar_id: i64,
    pub access_hash: i32,
    pub profile_privacy: i32,
    pub online_privacy: i32,
    pub call_privacy: i32,
    pub add_to_group_privacy: i32,
    pub seen_message_privacy: i32,
    pub phone: String,
    pub email: String,
    pub about: String,
    pub password_hash: String,
    pub password_salt: String,
    pub post_seq: i32,
    pub followers_count: i32,
    pub following_count: i32,
    pub posts_count: i32,
    pub media_count: i32,
    pub photo_count: i32,
    pub video_count: i32,
    pub gif_count: i32,
    pub audio_count: i32,
    pub voice_count: i32,
    pub file_count: i32,
    pub link_count: i32,
    pub board_count: i32,
    pub pined_count: i32,
    pub likes_count: i32,
    pub reshared_count: i32,
    pub last_post_time: i32,
    pub created_time: i32,
    pub version_time: i32,
    pub is_deleted: i32,
    pub is_banned: i32,
}

impl<'a> MessageRead<'a> for User {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.user_id = r.read_int32(bytes)?,
                Ok(18) => msg.user_name = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.user_name_lower = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.first_name = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.last_name = r.read_string(bytes)?.to_owned(),
                Ok(48) => msg.is_verified = r.read_int32(bytes)?,
                Ok(56) => msg.avatar_id = r.read_int64(bytes)?,
                Ok(64) => msg.access_hash = r.read_int32(bytes)?,
                Ok(72) => msg.profile_privacy = r.read_int32(bytes)?,
                Ok(80) => msg.online_privacy = r.read_int32(bytes)?,
                Ok(88) => msg.call_privacy = r.read_int32(bytes)?,
                Ok(96) => msg.add_to_group_privacy = r.read_int32(bytes)?,
                Ok(104) => msg.seen_message_privacy = r.read_int32(bytes)?,
                Ok(114) => msg.phone = r.read_string(bytes)?.to_owned(),
                Ok(122) => msg.email = r.read_string(bytes)?.to_owned(),
                Ok(130) => msg.about = r.read_string(bytes)?.to_owned(),
                Ok(138) => msg.password_hash = r.read_string(bytes)?.to_owned(),
                Ok(146) => msg.password_salt = r.read_string(bytes)?.to_owned(),
                Ok(152) => msg.post_seq = r.read_int32(bytes)?,
                Ok(160) => msg.followers_count = r.read_int32(bytes)?,
                Ok(168) => msg.following_count = r.read_int32(bytes)?,
                Ok(176) => msg.posts_count = r.read_int32(bytes)?,
                Ok(184) => msg.media_count = r.read_int32(bytes)?,
                Ok(192) => msg.photo_count = r.read_int32(bytes)?,
                Ok(200) => msg.video_count = r.read_int32(bytes)?,
                Ok(208) => msg.gif_count = r.read_int32(bytes)?,
                Ok(216) => msg.audio_count = r.read_int32(bytes)?,
                Ok(224) => msg.voice_count = r.read_int32(bytes)?,
                Ok(232) => msg.file_count = r.read_int32(bytes)?,
                Ok(240) => msg.link_count = r.read_int32(bytes)?,
                Ok(248) => msg.board_count = r.read_int32(bytes)?,
                Ok(256) => msg.pined_count = r.read_int32(bytes)?,
                Ok(264) => msg.likes_count = r.read_int32(bytes)?,
                Ok(272) => msg.reshared_count = r.read_int32(bytes)?,
                Ok(280) => msg.last_post_time = r.read_int32(bytes)?,
                Ok(288) => msg.created_time = r.read_int32(bytes)?,
                Ok(296) => msg.version_time = r.read_int32(bytes)?,
                Ok(304) => msg.is_deleted = r.read_int32(bytes)?,
                Ok(312) => msg.is_banned = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for User {
    fn get_size(&self) -> usize {
        0
        + if self.user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.user_name == String::default() { 0 } else { 1 + sizeof_len((&self.user_name).len()) }
        + if self.user_name_lower == String::default() { 0 } else { 1 + sizeof_len((&self.user_name_lower).len()) }
        + if self.first_name == String::default() { 0 } else { 1 + sizeof_len((&self.first_name).len()) }
        + if self.last_name == String::default() { 0 } else { 1 + sizeof_len((&self.last_name).len()) }
        + if self.is_verified == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.is_verified) as u64) }
        + if self.avatar_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.avatar_id) as u64) }
        + if self.access_hash == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.access_hash) as u64) }
        + if self.profile_privacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.profile_privacy) as u64) }
        + if self.online_privacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.online_privacy) as u64) }
        + if self.call_privacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.call_privacy) as u64) }
        + if self.add_to_group_privacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.add_to_group_privacy) as u64) }
        + if self.seen_message_privacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.seen_message_privacy) as u64) }
        + if self.phone == String::default() { 0 } else { 1 + sizeof_len((&self.phone).len()) }
        + if self.email == String::default() { 0 } else { 1 + sizeof_len((&self.email).len()) }
        + if self.about == String::default() { 0 } else { 2 + sizeof_len((&self.about).len()) }
        + if self.password_hash == String::default() { 0 } else { 2 + sizeof_len((&self.password_hash).len()) }
        + if self.password_salt == String::default() { 0 } else { 2 + sizeof_len((&self.password_salt).len()) }
        + if self.post_seq == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.post_seq) as u64) }
        + if self.followers_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.followers_count) as u64) }
        + if self.following_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.following_count) as u64) }
        + if self.posts_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.posts_count) as u64) }
        + if self.media_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.media_count) as u64) }
        + if self.photo_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.photo_count) as u64) }
        + if self.video_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.video_count) as u64) }
        + if self.gif_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.gif_count) as u64) }
        + if self.audio_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.audio_count) as u64) }
        + if self.voice_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.voice_count) as u64) }
        + if self.file_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.file_count) as u64) }
        + if self.link_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.link_count) as u64) }
        + if self.board_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.board_count) as u64) }
        + if self.pined_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.pined_count) as u64) }
        + if self.likes_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.likes_count) as u64) }
        + if self.reshared_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.reshared_count) as u64) }
        + if self.last_post_time == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.last_post_time) as u64) }
        + if self.created_time == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.created_time) as u64) }
        + if self.version_time == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.version_time) as u64) }
        + if self.is_deleted == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.is_deleted) as u64) }
        + if self.is_banned == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.is_banned) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.user_id != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.user_id))?; }
        if self.user_name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.user_name))?; }
        if self.user_name_lower != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.user_name_lower))?; }
        if self.first_name != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.first_name))?; }
        if self.last_name != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.last_name))?; }
        if self.is_verified != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.is_verified))?; }
        if self.avatar_id != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.avatar_id))?; }
        if self.access_hash != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.access_hash))?; }
        if self.profile_privacy != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.profile_privacy))?; }
        if self.online_privacy != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.online_privacy))?; }
        if self.call_privacy != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.call_privacy))?; }
        if self.add_to_group_privacy != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.add_to_group_privacy))?; }
        if self.seen_message_privacy != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.seen_message_privacy))?; }
        if self.phone != String::default() { w.write_with_tag(114, |w| w.write_string(&**&self.phone))?; }
        if self.email != String::default() { w.write_with_tag(122, |w| w.write_string(&**&self.email))?; }
        if self.about != String::default() { w.write_with_tag(130, |w| w.write_string(&**&self.about))?; }
        if self.password_hash != String::default() { w.write_with_tag(138, |w| w.write_string(&**&self.password_hash))?; }
        if self.password_salt != String::default() { w.write_with_tag(146, |w| w.write_string(&**&self.password_salt))?; }
        if self.post_seq != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.post_seq))?; }
        if self.followers_count != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.followers_count))?; }
        if self.following_count != 0i32 { w.write_with_tag(168, |w| w.write_int32(*&self.following_count))?; }
        if self.posts_count != 0i32 { w.write_with_tag(176, |w| w.write_int32(*&self.posts_count))?; }
        if self.media_count != 0i32 { w.write_with_tag(184, |w| w.write_int32(*&self.media_count))?; }
        if self.photo_count != 0i32 { w.write_with_tag(192, |w| w.write_int32(*&self.photo_count))?; }
        if self.video_count != 0i32 { w.write_with_tag(200, |w| w.write_int32(*&self.video_count))?; }
        if self.gif_count != 0i32 { w.write_with_tag(208, |w| w.write_int32(*&self.gif_count))?; }
        if self.audio_count != 0i32 { w.write_with_tag(216, |w| w.write_int32(*&self.audio_count))?; }
        if self.voice_count != 0i32 { w.write_with_tag(224, |w| w.write_int32(*&self.voice_count))?; }
        if self.file_count != 0i32 { w.write_with_tag(232, |w| w.write_int32(*&self.file_count))?; }
        if self.link_count != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.link_count))?; }
        if self.board_count != 0i32 { w.write_with_tag(248, |w| w.write_int32(*&self.board_count))?; }
        if self.pined_count != 0i32 { w.write_with_tag(256, |w| w.write_int32(*&self.pined_count))?; }
        if self.likes_count != 0i32 { w.write_with_tag(264, |w| w.write_int32(*&self.likes_count))?; }
        if self.reshared_count != 0i32 { w.write_with_tag(272, |w| w.write_int32(*&self.reshared_count))?; }
        if self.last_post_time != 0i32 { w.write_with_tag(280, |w| w.write_int32(*&self.last_post_time))?; }
        if self.created_time != 0i32 { w.write_with_tag(288, |w| w.write_int32(*&self.created_time))?; }
        if self.version_time != 0i32 { w.write_with_tag(296, |w| w.write_int32(*&self.version_time))?; }
        if self.is_deleted != 0i32 { w.write_with_tag(304, |w| w.write_int32(*&self.is_deleted))?; }
        if self.is_banned != 0i32 { w.write_with_tag(312, |w| w.write_int32(*&self.is_banned))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UserRelation {
    pub rel_nano_id: i64,
    pub user_id: i32,
    pub peer_user_id: i32,
    pub follwing: i32,
    pub followed: i32,
    pub in_contacts: i32,
    pub mutual_contact: i32,
    pub is_favorite: i32,
    pub notify: i32,
}

impl<'a> MessageRead<'a> for UserRelation {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.rel_nano_id = r.read_int64(bytes)?,
                Ok(16) => msg.user_id = r.read_int32(bytes)?,
                Ok(24) => msg.peer_user_id = r.read_int32(bytes)?,
                Ok(32) => msg.follwing = r.read_int32(bytes)?,
                Ok(40) => msg.followed = r.read_int32(bytes)?,
                Ok(48) => msg.in_contacts = r.read_int32(bytes)?,
                Ok(56) => msg.mutual_contact = r.read_int32(bytes)?,
                Ok(64) => msg.is_favorite = r.read_int32(bytes)?,
                Ok(72) => msg.notify = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UserRelation {
    fn get_size(&self) -> usize {
        0
        + if self.rel_nano_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.rel_nano_id) as u64) }
        + if self.user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.peer_user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.peer_user_id) as u64) }
        + if self.follwing == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.follwing) as u64) }
        + if self.followed == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.followed) as u64) }
        + if self.in_contacts == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.in_contacts) as u64) }
        + if self.mutual_contact == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.mutual_contact) as u64) }
        + if self.is_favorite == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.is_favorite) as u64) }
        + if self.notify == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.notify) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.rel_nano_id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.rel_nano_id))?; }
        if self.user_id != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.user_id))?; }
        if self.peer_user_id != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.peer_user_id))?; }
        if self.follwing != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.follwing))?; }
        if self.followed != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.followed))?; }
        if self.in_contacts != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.in_contacts))?; }
        if self.mutual_contact != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.mutual_contact))?; }
        if self.is_favorite != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.is_favorite))?; }
        if self.notify != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.notify))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Chat {
    pub chat_id: i64,
    pub chat_key: String,
    pub room_key: String,
    pub room_type: i32,
    pub user_id: i32,
    pub peer_user_id: i32,
    pub group_id: i64,
    pub hash_tag_id: i64,
    pub title: String,
    pub pin_time_ms: i64,
    pub from_msg_id: i64,
    pub unseen_count: i32,
    pub seq: i32,
    pub last_msg_id: i64,
    pub last_my_msg_status: i32,
    pub my_last_seen_seq: i32,
    pub my_last_seen_msg_id: i64,
    pub peer_last_seen_msg_id: i64,
    pub my_last_delivered_seq: i32,
    pub my_last_delivered_msg_id: i64,
    pub peer_last_delivered_msg_id: i64,
    pub is_active: i32,
    pub is_left: i32,
    pub is_creator: i32,
    pub is_kicked: i32,
    pub is_admin: i32,
    pub is_deactivated: i32,
    pub is_muted: i32,
    pub mute_until: i32,
    pub version_time_ms: i64,
    pub version_seq: i32,
    pub order_time: i32,
    pub created_time: i32,
    pub draft_text: String,
    pub drat_reply_to_msg_id: i64,
}

impl<'a> MessageRead<'a> for Chat {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.chat_id = r.read_int64(bytes)?,
                Ok(18) => msg.chat_key = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.room_key = r.read_string(bytes)?.to_owned(),
                Ok(32) => msg.room_type = r.read_int32(bytes)?,
                Ok(40) => msg.user_id = r.read_int32(bytes)?,
                Ok(48) => msg.peer_user_id = r.read_int32(bytes)?,
                Ok(56) => msg.group_id = r.read_int64(bytes)?,
                Ok(64) => msg.hash_tag_id = r.read_int64(bytes)?,
                Ok(74) => msg.title = r.read_string(bytes)?.to_owned(),
                Ok(80) => msg.pin_time_ms = r.read_int64(bytes)?,
                Ok(88) => msg.from_msg_id = r.read_int64(bytes)?,
                Ok(96) => msg.unseen_count = r.read_int32(bytes)?,
                Ok(104) => msg.seq = r.read_int32(bytes)?,
                Ok(112) => msg.last_msg_id = r.read_int64(bytes)?,
                Ok(120) => msg.last_my_msg_status = r.read_int32(bytes)?,
                Ok(128) => msg.my_last_seen_seq = r.read_int32(bytes)?,
                Ok(136) => msg.my_last_seen_msg_id = r.read_int64(bytes)?,
                Ok(144) => msg.peer_last_seen_msg_id = r.read_int64(bytes)?,
                Ok(152) => msg.my_last_delivered_seq = r.read_int32(bytes)?,
                Ok(160) => msg.my_last_delivered_msg_id = r.read_int64(bytes)?,
                Ok(168) => msg.peer_last_delivered_msg_id = r.read_int64(bytes)?,
                Ok(176) => msg.is_active = r.read_int32(bytes)?,
                Ok(184) => msg.is_left = r.read_int32(bytes)?,
                Ok(192) => msg.is_creator = r.read_int32(bytes)?,
                Ok(200) => msg.is_kicked = r.read_int32(bytes)?,
                Ok(208) => msg.is_admin = r.read_int32(bytes)?,
                Ok(216) => msg.is_deactivated = r.read_int32(bytes)?,
                Ok(224) => msg.is_muted = r.read_int32(bytes)?,
                Ok(232) => msg.mute_until = r.read_int32(bytes)?,
                Ok(240) => msg.version_time_ms = r.read_int64(bytes)?,
                Ok(248) => msg.version_seq = r.read_int32(bytes)?,
                Ok(256) => msg.order_time = r.read_int32(bytes)?,
                Ok(264) => msg.created_time = r.read_int32(bytes)?,
                Ok(274) => msg.draft_text = r.read_string(bytes)?.to_owned(),
                Ok(280) => msg.drat_reply_to_msg_id = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Chat {
    fn get_size(&self) -> usize {
        0
        + if self.chat_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.chat_id) as u64) }
        + if self.chat_key == String::default() { 0 } else { 1 + sizeof_len((&self.chat_key).len()) }
        + if self.room_key == String::default() { 0 } else { 1 + sizeof_len((&self.room_key).len()) }
        + if self.room_type == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.room_type) as u64) }
        + if self.user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.peer_user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.peer_user_id) as u64) }
        + if self.group_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
        + if self.hash_tag_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.hash_tag_id) as u64) }
        + if self.title == String::default() { 0 } else { 1 + sizeof_len((&self.title).len()) }
        + if self.pin_time_ms == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.pin_time_ms) as u64) }
        + if self.from_msg_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.from_msg_id) as u64) }
        + if self.unseen_count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.unseen_count) as u64) }
        + if self.seq == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.last_msg_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_msg_id) as u64) }
        + if self.last_my_msg_status == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.last_my_msg_status) as u64) }
        + if self.my_last_seen_seq == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.my_last_seen_seq) as u64) }
        + if self.my_last_seen_msg_id == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.my_last_seen_msg_id) as u64) }
        + if self.peer_last_seen_msg_id == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.peer_last_seen_msg_id) as u64) }
        + if self.my_last_delivered_seq == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.my_last_delivered_seq) as u64) }
        + if self.my_last_delivered_msg_id == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.my_last_delivered_msg_id) as u64) }
        + if self.peer_last_delivered_msg_id == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.peer_last_delivered_msg_id) as u64) }
        + if self.is_active == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.is_active) as u64) }
        + if self.is_left == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.is_left) as u64) }
        + if self.is_creator == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.is_creator) as u64) }
        + if self.is_kicked == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.is_kicked) as u64) }
        + if self.is_admin == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.is_admin) as u64) }
        + if self.is_deactivated == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.is_deactivated) as u64) }
        + if self.is_muted == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.is_muted) as u64) }
        + if self.mute_until == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.mute_until) as u64) }
        + if self.version_time_ms == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.version_time_ms) as u64) }
        + if self.version_seq == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.version_seq) as u64) }
        + if self.order_time == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.order_time) as u64) }
        + if self.created_time == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.created_time) as u64) }
        + if self.draft_text == String::default() { 0 } else { 2 + sizeof_len((&self.draft_text).len()) }
        + if self.drat_reply_to_msg_id == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.drat_reply_to_msg_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.chat_id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.chat_id))?; }
        if self.chat_key != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.chat_key))?; }
        if self.room_key != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.room_key))?; }
        if self.room_type != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.room_type))?; }
        if self.user_id != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.user_id))?; }
        if self.peer_user_id != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.peer_user_id))?; }
        if self.group_id != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.group_id))?; }
        if self.hash_tag_id != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.hash_tag_id))?; }
        if self.title != String::default() { w.write_with_tag(74, |w| w.write_string(&**&self.title))?; }
        if self.pin_time_ms != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.pin_time_ms))?; }
        if self.from_msg_id != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.from_msg_id))?; }
        if self.unseen_count != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.unseen_count))?; }
        if self.seq != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.seq))?; }
        if self.last_msg_id != 0i64 { w.write_with_tag(112, |w| w.write_int64(*&self.last_msg_id))?; }
        if self.last_my_msg_status != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.last_my_msg_status))?; }
        if self.my_last_seen_seq != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.my_last_seen_seq))?; }
        if self.my_last_seen_msg_id != 0i64 { w.write_with_tag(136, |w| w.write_int64(*&self.my_last_seen_msg_id))?; }
        if self.peer_last_seen_msg_id != 0i64 { w.write_with_tag(144, |w| w.write_int64(*&self.peer_last_seen_msg_id))?; }
        if self.my_last_delivered_seq != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.my_last_delivered_seq))?; }
        if self.my_last_delivered_msg_id != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.my_last_delivered_msg_id))?; }
        if self.peer_last_delivered_msg_id != 0i64 { w.write_with_tag(168, |w| w.write_int64(*&self.peer_last_delivered_msg_id))?; }
        if self.is_active != 0i32 { w.write_with_tag(176, |w| w.write_int32(*&self.is_active))?; }
        if self.is_left != 0i32 { w.write_with_tag(184, |w| w.write_int32(*&self.is_left))?; }
        if self.is_creator != 0i32 { w.write_with_tag(192, |w| w.write_int32(*&self.is_creator))?; }
        if self.is_kicked != 0i32 { w.write_with_tag(200, |w| w.write_int32(*&self.is_kicked))?; }
        if self.is_admin != 0i32 { w.write_with_tag(208, |w| w.write_int32(*&self.is_admin))?; }
        if self.is_deactivated != 0i32 { w.write_with_tag(216, |w| w.write_int32(*&self.is_deactivated))?; }
        if self.is_muted != 0i32 { w.write_with_tag(224, |w| w.write_int32(*&self.is_muted))?; }
        if self.mute_until != 0i32 { w.write_with_tag(232, |w| w.write_int32(*&self.mute_until))?; }
        if self.version_time_ms != 0i64 { w.write_with_tag(240, |w| w.write_int64(*&self.version_time_ms))?; }
        if self.version_seq != 0i32 { w.write_with_tag(248, |w| w.write_int32(*&self.version_seq))?; }
        if self.order_time != 0i32 { w.write_with_tag(256, |w| w.write_int32(*&self.order_time))?; }
        if self.created_time != 0i32 { w.write_with_tag(264, |w| w.write_int32(*&self.created_time))?; }
        if self.draft_text != String::default() { w.write_with_tag(274, |w| w.write_string(&**&self.draft_text))?; }
        if self.drat_reply_to_msg_id != 0i64 { w.write_with_tag(280, |w| w.write_int64(*&self.drat_reply_to_msg_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Group {
    pub group_id: i64,
    pub group_key: String,
    pub group_name: String,
    pub user_name: String,
    pub is_super_group: i32,
    pub hash_tag_id: i64,
    pub creator_user_id: i32,
    pub group_privacy: i32,
    pub history_view_able: i32,
    pub seq: i64,
    pub last_msg_id: i64,
    pub pined_msg_id: i64,
    pub avatar_ref_id: i64,
    pub avatar_count: i32,
    pub about: String,
    pub invite_link_hash: String,
    pub members_count: i32,
    pub admins_count: i32,
    pub moderator_counts: i32,
    pub sort_time: i32,
    pub created_time: i32,
    pub is_mute: i32,
    pub is_deleted: i32,
    pub is_banned: i32,
}

impl<'a> MessageRead<'a> for Group {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_int64(bytes)?,
                Ok(18) => msg.group_key = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.group_name = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.user_name = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.is_super_group = r.read_int32(bytes)?,
                Ok(48) => msg.hash_tag_id = r.read_int64(bytes)?,
                Ok(56) => msg.creator_user_id = r.read_int32(bytes)?,
                Ok(64) => msg.group_privacy = r.read_int32(bytes)?,
                Ok(72) => msg.history_view_able = r.read_int32(bytes)?,
                Ok(80) => msg.seq = r.read_int64(bytes)?,
                Ok(88) => msg.last_msg_id = r.read_int64(bytes)?,
                Ok(96) => msg.pined_msg_id = r.read_int64(bytes)?,
                Ok(104) => msg.avatar_ref_id = r.read_int64(bytes)?,
                Ok(112) => msg.avatar_count = r.read_int32(bytes)?,
                Ok(122) => msg.about = r.read_string(bytes)?.to_owned(),
                Ok(130) => msg.invite_link_hash = r.read_string(bytes)?.to_owned(),
                Ok(136) => msg.members_count = r.read_int32(bytes)?,
                Ok(144) => msg.admins_count = r.read_int32(bytes)?,
                Ok(152) => msg.moderator_counts = r.read_int32(bytes)?,
                Ok(160) => msg.sort_time = r.read_int32(bytes)?,
                Ok(168) => msg.created_time = r.read_int32(bytes)?,
                Ok(176) => msg.is_mute = r.read_int32(bytes)?,
                Ok(184) => msg.is_deleted = r.read_int32(bytes)?,
                Ok(192) => msg.is_banned = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Group {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
        + if self.group_key == String::default() { 0 } else { 1 + sizeof_len((&self.group_key).len()) }
        + if self.group_name == String::default() { 0 } else { 1 + sizeof_len((&self.group_name).len()) }
        + if self.user_name == String::default() { 0 } else { 1 + sizeof_len((&self.user_name).len()) }
        + if self.is_super_group == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.is_super_group) as u64) }
        + if self.hash_tag_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.hash_tag_id) as u64) }
        + if self.creator_user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.creator_user_id) as u64) }
        + if self.group_privacy == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.group_privacy) as u64) }
        + if self.history_view_able == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.history_view_able) as u64) }
        + if self.seq == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.seq) as u64) }
        + if self.last_msg_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.last_msg_id) as u64) }
        + if self.pined_msg_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.pined_msg_id) as u64) }
        + if self.avatar_ref_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.avatar_ref_id) as u64) }
        + if self.avatar_count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.avatar_count) as u64) }
        + if self.about == String::default() { 0 } else { 1 + sizeof_len((&self.about).len()) }
        + if self.invite_link_hash == String::default() { 0 } else { 2 + sizeof_len((&self.invite_link_hash).len()) }
        + if self.members_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.members_count) as u64) }
        + if self.admins_count == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.admins_count) as u64) }
        + if self.moderator_counts == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.moderator_counts) as u64) }
        + if self.sort_time == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.sort_time) as u64) }
        + if self.created_time == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.created_time) as u64) }
        + if self.is_mute == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.is_mute) as u64) }
        + if self.is_deleted == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.is_deleted) as u64) }
        + if self.is_banned == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.is_banned) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.group_id))?; }
        if self.group_key != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.group_key))?; }
        if self.group_name != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.group_name))?; }
        if self.user_name != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.user_name))?; }
        if self.is_super_group != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.is_super_group))?; }
        if self.hash_tag_id != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.hash_tag_id))?; }
        if self.creator_user_id != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.creator_user_id))?; }
        if self.group_privacy != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.group_privacy))?; }
        if self.history_view_able != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.history_view_able))?; }
        if self.seq != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.seq))?; }
        if self.last_msg_id != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.last_msg_id))?; }
        if self.pined_msg_id != 0i64 { w.write_with_tag(96, |w| w.write_int64(*&self.pined_msg_id))?; }
        if self.avatar_ref_id != 0i64 { w.write_with_tag(104, |w| w.write_int64(*&self.avatar_ref_id))?; }
        if self.avatar_count != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.avatar_count))?; }
        if self.about != String::default() { w.write_with_tag(122, |w| w.write_string(&**&self.about))?; }
        if self.invite_link_hash != String::default() { w.write_with_tag(130, |w| w.write_string(&**&self.invite_link_hash))?; }
        if self.members_count != 0i32 { w.write_with_tag(136, |w| w.write_int32(*&self.members_count))?; }
        if self.admins_count != 0i32 { w.write_with_tag(144, |w| w.write_int32(*&self.admins_count))?; }
        if self.moderator_counts != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.moderator_counts))?; }
        if self.sort_time != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.sort_time))?; }
        if self.created_time != 0i32 { w.write_with_tag(168, |w| w.write_int32(*&self.created_time))?; }
        if self.is_mute != 0i32 { w.write_with_tag(176, |w| w.write_int32(*&self.is_mute))?; }
        if self.is_deleted != 0i32 { w.write_with_tag(184, |w| w.write_int32(*&self.is_deleted))?; }
        if self.is_banned != 0i32 { w.write_with_tag(192, |w| w.write_int32(*&self.is_banned))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupMember {
    pub order_id: i64,
    pub group_id: i64,
    pub user_id: i32,
    pub by_user_id: i32,
    pub group_role: i32,
    pub created_time: i32,
}

impl<'a> MessageRead<'a> for GroupMember {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.order_id = r.read_int64(bytes)?,
                Ok(16) => msg.group_id = r.read_int64(bytes)?,
                Ok(24) => msg.user_id = r.read_int32(bytes)?,
                Ok(32) => msg.by_user_id = r.read_int32(bytes)?,
                Ok(40) => msg.group_role = r.read_int32(bytes)?,
                Ok(48) => msg.created_time = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupMember {
    fn get_size(&self) -> usize {
        0
        + if self.order_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.order_id) as u64) }
        + if self.group_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
        + if self.user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.by_user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.by_user_id) as u64) }
        + if self.group_role == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.group_role) as u64) }
        + if self.created_time == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.order_id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.order_id))?; }
        if self.group_id != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.group_id))?; }
        if self.user_id != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.user_id))?; }
        if self.by_user_id != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.by_user_id))?; }
        if self.group_role != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.group_role))?; }
        if self.created_time != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileMsg {
    pub id: i64,
    pub access_hash: i32,
    pub file_type: i32,
    pub width: i32,
    pub height: i32,
    pub extension: String,
    pub user_id: i32,
    pub data_thumb: Vec<u8>,
    pub data: Vec<u8>,
}

impl<'a> MessageRead<'a> for FileMsg {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int64(bytes)?,
                Ok(16) => msg.access_hash = r.read_int32(bytes)?,
                Ok(24) => msg.file_type = r.read_int32(bytes)?,
                Ok(32) => msg.width = r.read_int32(bytes)?,
                Ok(40) => msg.height = r.read_int32(bytes)?,
                Ok(50) => msg.extension = r.read_string(bytes)?.to_owned(),
                Ok(56) => msg.user_id = r.read_int32(bytes)?,
                Ok(66) => msg.data_thumb = r.read_bytes(bytes)?.to_owned(),
                Ok(74) => msg.data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for FileMsg {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.access_hash == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.access_hash) as u64) }
        + if self.file_type == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.file_type) as u64) }
        + if self.width == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.width) as u64) }
        + if self.height == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.height) as u64) }
        + if self.extension == String::default() { 0 } else { 1 + sizeof_len((&self.extension).len()) }
        + if self.user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.user_id) as u64) }
        + if self.data_thumb == vec![] { 0 } else { 1 + sizeof_len((&self.data_thumb).len()) }
        + if self.data == vec![] { 0 } else { 1 + sizeof_len((&self.data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.id))?; }
        if self.access_hash != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.access_hash))?; }
        if self.file_type != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.file_type))?; }
        if self.width != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.width))?; }
        if self.height != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.height))?; }
        if self.extension != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.extension))?; }
        if self.user_id != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.user_id))?; }
        if self.data_thumb != vec![] { w.write_with_tag(66, |w| w.write_bytes(&**&self.data_thumb))?; }
        if self.data != vec![] { w.write_with_tag(74, |w| w.write_bytes(&**&self.data))?; }
        Ok(())
    }
}

