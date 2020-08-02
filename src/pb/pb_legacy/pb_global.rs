// Automatically generated rust module for 'pb_global.proto' file

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
pub struct PB_CommandToServer {
    pub ClientCallId: i64,
    pub Command: String,
    pub RespondReached: bool,
    pub Data: Vec<u8>,
}

impl<'a> MessageRead<'a> for PB_CommandToServer {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ClientCallId = r.read_int64(bytes)?,
                Ok(18) => msg.Command = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.RespondReached = r.read_bool(bytes)?,
                Ok(34) => msg.Data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_CommandToServer {
    fn get_size(&self) -> usize {
        0
        + if self.ClientCallId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ClientCallId) as u64) }
        + if self.Command == String::default() { 0 } else { 1 + sizeof_len((&self.Command).len()) }
        + if self.RespondReached == false { 0 } else { 1 + sizeof_varint(*(&self.RespondReached) as u64) }
        + if self.Data == vec![] { 0 } else { 1 + sizeof_len((&self.Data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ClientCallId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.ClientCallId))?; }
        if self.Command != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Command))?; }
        if self.RespondReached != false { w.write_with_tag(24, |w| w.write_bool(*&self.RespondReached))?; }
        if self.Data != vec![] { w.write_with_tag(34, |w| w.write_bytes(&**&self.Data))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_CommandToClient {
    pub ServerCallId: i64,
    pub Command: String,
    pub RespondReached: bool,
    pub Data: Vec<u8>,
}

impl<'a> MessageRead<'a> for PB_CommandToClient {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ServerCallId = r.read_int64(bytes)?,
                Ok(18) => msg.Command = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.RespondReached = r.read_bool(bytes)?,
                Ok(34) => msg.Data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_CommandToClient {
    fn get_size(&self) -> usize {
        0
        + if self.ServerCallId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ServerCallId) as u64) }
        + if self.Command == String::default() { 0 } else { 1 + sizeof_len((&self.Command).len()) }
        + if self.RespondReached == false { 0 } else { 1 + sizeof_varint(*(&self.RespondReached) as u64) }
        + if self.Data == vec![] { 0 } else { 1 + sizeof_len((&self.Data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ServerCallId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.ServerCallId))?; }
        if self.Command != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Command))?; }
        if self.RespondReached != false { w.write_with_tag(24, |w| w.write_bool(*&self.RespondReached))?; }
        if self.Data != vec![] { w.write_with_tag(34, |w| w.write_bytes(&**&self.Data))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_CommandReachedToServer {
    pub ClientCallId: i64,
}

impl<'a> MessageRead<'a> for PB_CommandReachedToServer {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ClientCallId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_CommandReachedToServer {
    fn get_size(&self) -> usize {
        0
        + if self.ClientCallId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ClientCallId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ClientCallId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.ClientCallId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_CommandReachedToClient {
    pub ServerCallId: i64,
}

impl<'a> MessageRead<'a> for PB_CommandReachedToClient {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ServerCallId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_CommandReachedToClient {
    fn get_size(&self) -> usize {
        0
        + if self.ServerCallId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ServerCallId) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ServerCallId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.ServerCallId))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PB_ResponseToClient {
    pub ClientCallId: i64,
    pub PBClass: String,
    pub RpcFullName: String,
    pub Data: Vec<u8>,
}

impl<'a> MessageRead<'a> for PB_ResponseToClient {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ClientCallId = r.read_int64(bytes)?,
                Ok(18) => msg.PBClass = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.RpcFullName = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.Data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PB_ResponseToClient {
    fn get_size(&self) -> usize {
        0
        + if self.ClientCallId == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ClientCallId) as u64) }
        + if self.PBClass == String::default() { 0 } else { 1 + sizeof_len((&self.PBClass).len()) }
        + if self.RpcFullName == String::default() { 0 } else { 1 + sizeof_len((&self.RpcFullName).len()) }
        + if self.Data == vec![] { 0 } else { 1 + sizeof_len((&self.Data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ClientCallId != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.ClientCallId))?; }
        if self.PBClass != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.PBClass))?; }
        if self.RpcFullName != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.RpcFullName))?; }
        if self.Data != vec![] { w.write_with_tag(34, |w| w.write_bytes(&**&self.Data))?; }
        Ok(())
    }
}

