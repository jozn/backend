// Automatically generated rust module for 'rpc_general.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct EchoParam<'a> {
    pub text: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for EchoParam<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for EchoParam<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.text == "" { 0 } else { 1 + sizeof_len((&self.text).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.text != "" { w.write_with_tag(10, |w| w.write_string(&**&self.text))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct EchoResponse<'a> {
    pub done: bool,
    pub text: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for EchoResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.done = r.read_bool(bytes)?,
                Ok(18) => msg.text = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for EchoResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.done == false { 0 } else { 1 + sizeof_varint(*(&self.done) as u64) }
        + if self.text == "" { 0 } else { 1 + sizeof_len((&self.text).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.done != false { w.write_with_tag(8, |w| w.write_bool(*&self.done))?; }
        if self.text != "" { w.write_with_tag(18, |w| w.write_string(&**&self.text))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CheckUserNameParam<'a> {
    pub username: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for CheckUserNameParam<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.username = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CheckUserNameParam<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.username == "" { 0 } else { 1 + sizeof_len((&self.username).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.username != "" { w.write_with_tag(10, |w| w.write_string(&**&self.username))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CheckUserNameResponse<'a> {
    pub is_available: bool,
    pub username: Cow<'a, str>,
    pub show_message: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for CheckUserNameResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.is_available = r.read_bool(bytes)?,
                Ok(18) => msg.username = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.show_message = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CheckUserNameResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.is_available == false { 0 } else { 1 + sizeof_varint(*(&self.is_available) as u64) }
        + if self.username == "" { 0 } else { 1 + sizeof_len((&self.username).len()) }
        + if self.show_message == "" { 0 } else { 1 + sizeof_len((&self.show_message).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.is_available != false { w.write_with_tag(8, |w| w.write_bool(*&self.is_available))?; }
        if self.username != "" { w.write_with_tag(18, |w| w.write_string(&**&self.username))?; }
        if self.show_message != "" { w.write_with_tag(26, |w| w.write_string(&**&self.show_message))?; }
        Ok(())
    }
}


