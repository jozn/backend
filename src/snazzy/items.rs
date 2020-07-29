// Automatically generated rust module for 'items.proto' file

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
use super::super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Shirt<'a> {
    pub color: Cow<'a, str>,
    pub size: snazzy::items::mod_Shirt::Size,
}

impl<'a> MessageRead<'a> for Shirt<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.color = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.size = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Shirt<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.color == "" { 0 } else { 1 + sizeof_len((&self.color).len()) }
        + if self.size == snazzy::items::mod_Shirt::Size::SMALL { 0 } else { 1 + sizeof_varint(*(&self.size) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.color != "" { w.write_with_tag(10, |w| w.write_string(&**&self.color))?; }
        if self.size != snazzy::items::mod_Shirt::Size::SMALL { w.write_with_tag(16, |w| w.write_enum(*&self.size as i32))?; }
        Ok(())
    }
}

pub mod mod_Shirt {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Size {
    SMALL = 0,
    MEDIUM = 1,
    LARGE = 2,
}

impl Default for Size {
    fn default() -> Self {
        Size::SMALL
    }
}

impl From<i32> for Size {
    fn from(i: i32) -> Self {
        match i {
            0 => Size::SMALL,
            1 => Size::MEDIUM,
            2 => Size::LARGE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Size {
    fn from(s: &'a str) -> Self {
        match s {
            "SMALL" => Size::SMALL,
            "MEDIUM" => Size::MEDIUM,
            "LARGE" => Size::LARGE,
            _ => Self::default(),
        }
    }
}

}

