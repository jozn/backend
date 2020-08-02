// Automatically generated rust module for 'pb_rpc_general.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{BytesReader, Result, MessageRead, MessageWrite};
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RPC_General_Types { }

impl<'a> MessageRead<'a> for RPC_General_Types {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RPC_General_Types { }

pub mod mod_RPC_General_Types {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Echo { }

impl<'a> MessageRead<'a> for Echo {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Echo { }

pub mod mod_Echo {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Text: String,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Text = r.read_string(bytes)?.to_owned(),
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
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Text != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.Text))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub Text: String,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.Text = r.read_string(bytes)?.to_owned(),
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
        + if self.Text == String::default() { 0 } else { 1 + sizeof_len((&self.Text).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if self.Text != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Text))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CheckUserName { }

impl<'a> MessageRead<'a> for CheckUserName {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for CheckUserName { }

pub mod mod_CheckUserName {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub UserName: String,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.UserName = r.read_string(bytes)?.to_owned(),
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
        + if self.UserName == String::default() { 0 } else { 1 + sizeof_len((&self.UserName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserName != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.UserName))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub IsAvailable: bool,
    pub UserName: String,
    pub ShowMessage: String,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.IsAvailable = r.read_bool(bytes)?,
                Ok(18) => msg.UserName = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.ShowMessage = r.read_string(bytes)?.to_owned(),
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
        + if self.IsAvailable == false { 0 } else { 1 + sizeof_varint(*(&self.IsAvailable) as u64) }
        + if self.UserName == String::default() { 0 } else { 1 + sizeof_len((&self.UserName).len()) }
        + if self.ShowMessage == String::default() { 0 } else { 1 + sizeof_len((&self.ShowMessage).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.IsAvailable != false { w.write_with_tag(8, |w| w.write_bool(*&self.IsAvailable))?; }
        if self.UserName != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.UserName))?; }
        if self.ShowMessage != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.ShowMessage))?; }
        Ok(())
    }
}

}

}


