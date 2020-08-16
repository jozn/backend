// Automatically generated rust module for 'rpc_direct.proto' file

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
pub struct DirectAddMessageParam { }

impl<'a> MessageRead<'a> for DirectAddMessageParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for DirectAddMessageParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DirectAddMessageResponse {
    pub done: bool,
    pub text: String,
}

impl<'a> MessageRead<'a> for DirectAddMessageResponse {
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

impl MessageWrite for DirectAddMessageResponse {
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


