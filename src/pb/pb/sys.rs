// Automatically generated rust module for 'sys.proto' file

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
pub struct Event {
    pub event_id: i64,
    pub event_type: i32,
    pub by_user_id: i64,
    pub peer_user_id: i64,
    pub post_id: i64,
    pub comment_id: i64,
    pub hash_tag_id: i64,
    pub group_id: i64,
    pub action_id: i64,
    pub chat_id: i64,
    pub chat_key: String,
    pub message_id: i64,
    pub re_shared_id: i64,
    pub murmur64_hash: i64,
}

impl<'a> MessageRead<'a> for Event {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.event_id = r.read_int64(bytes)?,
                Ok(16) => msg.event_type = r.read_int32(bytes)?,
                Ok(24) => msg.by_user_id = r.read_int64(bytes)?,
                Ok(32) => msg.peer_user_id = r.read_int64(bytes)?,
                Ok(40) => msg.post_id = r.read_int64(bytes)?,
                Ok(48) => msg.comment_id = r.read_int64(bytes)?,
                Ok(56) => msg.hash_tag_id = r.read_int64(bytes)?,
                Ok(64) => msg.group_id = r.read_int64(bytes)?,
                Ok(72) => msg.action_id = r.read_int64(bytes)?,
                Ok(80) => msg.chat_id = r.read_int64(bytes)?,
                Ok(90) => msg.chat_key = r.read_string(bytes)?.to_owned(),
                Ok(96) => msg.message_id = r.read_int64(bytes)?,
                Ok(104) => msg.re_shared_id = r.read_int64(bytes)?,
                Ok(112) => msg.murmur64_hash = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Event {
    fn get_size(&self) -> usize {
        0
        + if self.event_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.event_id) as u64) }
        + if self.event_type == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.event_type) as u64) }
        + if self.by_user_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.by_user_id) as u64) }
        + if self.peer_user_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.peer_user_id) as u64) }
        + if self.post_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.post_id) as u64) }
        + if self.comment_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.comment_id) as u64) }
        + if self.hash_tag_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.hash_tag_id) as u64) }
        + if self.group_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
        + if self.action_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.action_id) as u64) }
        + if self.chat_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.chat_id) as u64) }
        + if self.chat_key == String::default() { 0 } else { 1 + sizeof_len((&self.chat_key).len()) }
        + if self.message_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.message_id) as u64) }
        + if self.re_shared_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.re_shared_id) as u64) }
        + if self.murmur64_hash == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.murmur64_hash) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.event_id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.event_id))?; }
        if self.event_type != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.event_type))?; }
        if self.by_user_id != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.by_user_id))?; }
        if self.peer_user_id != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.peer_user_id))?; }
        if self.post_id != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.post_id))?; }
        if self.comment_id != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.comment_id))?; }
        if self.hash_tag_id != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.hash_tag_id))?; }
        if self.group_id != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.group_id))?; }
        if self.action_id != 0i64 { w.write_with_tag(72, |w| w.write_int64(*&self.action_id))?; }
        if self.chat_id != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.chat_id))?; }
        if self.chat_key != String::default() { w.write_with_tag(90, |w| w.write_string(&**&self.chat_key))?; }
        if self.message_id != 0i64 { w.write_with_tag(96, |w| w.write_int64(*&self.message_id))?; }
        if self.re_shared_id != 0i64 { w.write_with_tag(104, |w| w.write_int64(*&self.re_shared_id))?; }
        if self.murmur64_hash != 0i64 { w.write_with_tag(112, |w| w.write_int64(*&self.murmur64_hash))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_Notify {
    pub notify_id: i64,
    pub for_user_id: i32,
    pub actor_user_id: i32,
    pub notify_type: i32,
    pub post_id: i64,
    pub comment_id: i64,
    pub peer_user_id: i32,
    pub murmur64_hash: i64,
    pub seen_status: i32,
    pub created_time: i32,
}

impl<'a> MessageRead<'a> for PB_Notify {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.notify_id = r.read_int64(bytes)?,
                Ok(16) => msg.for_user_id = r.read_int32(bytes)?,
                Ok(24) => msg.actor_user_id = r.read_int32(bytes)?,
                Ok(32) => msg.notify_type = r.read_int32(bytes)?,
                Ok(40) => msg.post_id = r.read_int64(bytes)?,
                Ok(48) => msg.comment_id = r.read_int64(bytes)?,
                Ok(56) => msg.peer_user_id = r.read_int32(bytes)?,
                Ok(64) => msg.murmur64_hash = r.read_int64(bytes)?,
                Ok(72) => msg.seen_status = r.read_int32(bytes)?,
                Ok(80) => msg.created_time = r.read_int32(bytes)?,
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
        + if self.notify_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.notify_id) as u64) }
        + if self.for_user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.for_user_id) as u64) }
        + if self.actor_user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.actor_user_id) as u64) }
        + if self.notify_type == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.notify_type) as u64) }
        + if self.post_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.post_id) as u64) }
        + if self.comment_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.comment_id) as u64) }
        + if self.peer_user_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.peer_user_id) as u64) }
        + if self.murmur64_hash == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.murmur64_hash) as u64) }
        + if self.seen_status == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.seen_status) as u64) }
        + if self.created_time == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.created_time) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.notify_id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.notify_id))?; }
        if self.for_user_id != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.for_user_id))?; }
        if self.actor_user_id != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.actor_user_id))?; }
        if self.notify_type != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.notify_type))?; }
        if self.post_id != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.post_id))?; }
        if self.comment_id != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.comment_id))?; }
        if self.peer_user_id != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.peer_user_id))?; }
        if self.murmur64_hash != 0i64 { w.write_with_tag(64, |w| w.write_int64(*&self.murmur64_hash))?; }
        if self.seen_status != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.seen_status))?; }
        if self.created_time != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.created_time))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Invoke {
    pub method: u32,
    pub action_id: u64,
    pub is_response: bool,
    pub rpc_data: Vec<u8>,
}

impl<'a> MessageRead<'a> for Invoke {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.method = r.read_uint32(bytes)?,
                Ok(16) => msg.action_id = r.read_uint64(bytes)?,
                Ok(24) => msg.is_response = r.read_bool(bytes)?,
                Ok(34) => msg.rpc_data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Invoke {
    fn get_size(&self) -> usize {
        0
        + if self.method == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.method) as u64) }
        + if self.action_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.action_id) as u64) }
        + if self.is_response == false { 0 } else { 1 + sizeof_varint(*(&self.is_response) as u64) }
        + if self.rpc_data == vec![] { 0 } else { 1 + sizeof_len((&self.rpc_data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.method != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.method))?; }
        if self.action_id != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.action_id))?; }
        if self.is_response != false { w.write_with_tag(24, |w| w.write_bool(*&self.is_response))?; }
        if self.rpc_data != vec![] { w.write_with_tag(34, |w| w.write_bytes(&**&self.rpc_data))?; }
        Ok(())
    }
}

