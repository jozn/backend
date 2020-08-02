// Automatically generated rust module for 'pb_rpc_auth2.proto' file

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
pub struct RPC_Auth_Types { }

impl<'a> MessageRead<'a> for RPC_Auth_Types {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RPC_Auth_Types { }

pub mod mod_RPC_Auth_Types {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SendConfirmCode { }

impl<'a> MessageRead<'a> for SendConfirmCode {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for SendConfirmCode { }

pub mod mod_SendConfirmCode {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Hash: String,
    pub Phone: String,
    pub CountryCode: String,
    pub Resend: bool,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Hash = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.Phone = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.CountryCode = r.read_string(bytes)?.to_owned(),
                Ok(32) => msg.Resend = r.read_bool(bytes)?,
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
        + if self.Hash == String::default() { 0 } else { 1 + sizeof_len((&self.Hash).len()) }
        + if self.Phone == String::default() { 0 } else { 1 + sizeof_len((&self.Phone).len()) }
        + if self.CountryCode == String::default() { 0 } else { 1 + sizeof_len((&self.CountryCode).len()) }
        + if self.Resend == false { 0 } else { 1 + sizeof_varint(*(&self.Resend) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Hash != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.Hash))?; }
        if self.Phone != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Phone))?; }
        if self.CountryCode != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.CountryCode))?; }
        if self.Resend != false { w.write_with_tag(32, |w| w.write_bool(*&self.Resend))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub ErrorMessage: String,
    pub JustEmailRegister: bool,
    pub SmsNumbers: Vec<String>,
    pub IsLogin: bool,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.ErrorMessage = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.JustEmailRegister = r.read_bool(bytes)?,
                Ok(34) => msg.SmsNumbers.push(r.read_string(bytes)?.to_owned()),
                Ok(40) => msg.IsLogin = r.read_bool(bytes)?,
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
        + if self.ErrorMessage == String::default() { 0 } else { 1 + sizeof_len((&self.ErrorMessage).len()) }
        + if self.JustEmailRegister == false { 0 } else { 1 + sizeof_varint(*(&self.JustEmailRegister) as u64) }
        + self.SmsNumbers.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.IsLogin == false { 0 } else { 1 + sizeof_varint(*(&self.IsLogin) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if self.ErrorMessage != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.ErrorMessage))?; }
        if self.JustEmailRegister != false { w.write_with_tag(24, |w| w.write_bool(*&self.JustEmailRegister))?; }
        for s in &self.SmsNumbers { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if self.IsLogin != false { w.write_with_tag(40, |w| w.write_bool(*&self.IsLogin))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SendConfirmCodeTypeEnum {
    SEND_CODE_OK = 0,
    SEND_CODE_EMAIL = 1,
}

impl Default for SendConfirmCodeTypeEnum {
    fn default() -> Self {
        SendConfirmCodeTypeEnum::SEND_CODE_OK
    }
}

impl From<i32> for SendConfirmCodeTypeEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => SendConfirmCodeTypeEnum::SEND_CODE_OK,
            1 => SendConfirmCodeTypeEnum::SEND_CODE_EMAIL,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SendConfirmCodeTypeEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "SEND_CODE_OK" => SendConfirmCodeTypeEnum::SEND_CODE_OK,
            "SEND_CODE_EMAIL" => SendConfirmCodeTypeEnum::SEND_CODE_EMAIL,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ConfirmCode { }

impl<'a> MessageRead<'a> for ConfirmCode {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ConfirmCode { }

pub mod mod_ConfirmCode {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Hash: String,
    pub Phone: String,
    pub Code: i32,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Hash = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.Phone = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.Code = r.read_int32(bytes)?,
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
        + if self.Hash == String::default() { 0 } else { 1 + sizeof_len((&self.Hash).len()) }
        + if self.Phone == String::default() { 0 } else { 1 + sizeof_len((&self.Phone).len()) }
        + if self.Code == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Code) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Hash != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.Hash))?; }
        if self.Phone != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.Phone))?; }
        if self.Code != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.Code))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub ErrorMessage: String,
    pub SelfUserView: Option<pb_views::PB_SelfUserView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.ErrorMessage = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.SelfUserView = Some(r.read_message::<pb_views::PB_SelfUserView>(bytes)?),
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
        + if self.ErrorMessage == String::default() { 0 } else { 1 + sizeof_len((&self.ErrorMessage).len()) }
        + self.SelfUserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if self.ErrorMessage != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.ErrorMessage))?; }
        if let Some(ref s) = self.SelfUserView { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SingUp { }

impl<'a> MessageRead<'a> for SingUp {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for SingUp { }

pub mod mod_SingUp {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub Hash: String,
    pub FirstName: String,
    pub LastName: String,
    pub UserName: String,
    pub Phone: String,
    pub Email: String,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Hash = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.FirstName = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.LastName = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.UserName = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.Phone = r.read_string(bytes)?.to_owned(),
                Ok(50) => msg.Email = r.read_string(bytes)?.to_owned(),
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
        + if self.Hash == String::default() { 0 } else { 1 + sizeof_len((&self.Hash).len()) }
        + if self.FirstName == String::default() { 0 } else { 1 + sizeof_len((&self.FirstName).len()) }
        + if self.LastName == String::default() { 0 } else { 1 + sizeof_len((&self.LastName).len()) }
        + if self.UserName == String::default() { 0 } else { 1 + sizeof_len((&self.UserName).len()) }
        + if self.Phone == String::default() { 0 } else { 1 + sizeof_len((&self.Phone).len()) }
        + if self.Email == String::default() { 0 } else { 1 + sizeof_len((&self.Email).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Hash != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.Hash))?; }
        if self.FirstName != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.FirstName))?; }
        if self.LastName != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.LastName))?; }
        if self.UserName != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.UserName))?; }
        if self.Phone != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.Phone))?; }
        if self.Email != String::default() { w.write_with_tag(50, |w| w.write_string(&**&self.Email))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub ErrorMessage: String,
    pub SelfUserView: Option<pb_views::PB_SelfUserView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.ErrorMessage = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.SelfUserView = Some(r.read_message::<pb_views::PB_SelfUserView>(bytes)?),
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
        + if self.ErrorMessage == String::default() { 0 } else { 1 + sizeof_len((&self.ErrorMessage).len()) }
        + self.SelfUserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if self.ErrorMessage != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.ErrorMessage))?; }
        if let Some(ref s) = self.SelfUserView { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SingIn { }

impl<'a> MessageRead<'a> for SingIn {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for SingIn { }

pub mod mod_SingIn {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param {
    pub UserNamePhoneEmail: String,
    pub Password: String,
}

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(34) => msg.UserNamePhoneEmail = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.Password = r.read_string(bytes)?.to_owned(),
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
        + if self.UserNamePhoneEmail == String::default() { 0 } else { 1 + sizeof_len((&self.UserNamePhoneEmail).len()) }
        + if self.Password == String::default() { 0 } else { 1 + sizeof_len((&self.Password).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserNamePhoneEmail != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.UserNamePhoneEmail))?; }
        if self.Password != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.Password))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub ErrorMessage: String,
    pub SelfUserView: Option<pb_views::PB_SelfUserView>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.ErrorMessage = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.SelfUserView = Some(r.read_message::<pb_views::PB_SelfUserView>(bytes)?),
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
        + if self.ErrorMessage == String::default() { 0 } else { 1 + sizeof_len((&self.ErrorMessage).len()) }
        + self.SelfUserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if self.ErrorMessage != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.ErrorMessage))?; }
        if let Some(ref s) = self.SelfUserView { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LogOut { }

impl<'a> MessageRead<'a> for LogOut {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for LogOut { }

pub mod mod_LogOut {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Param { }

impl<'a> MessageRead<'a> for Param {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Param { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub Done: bool,
    pub ErrorMessage: String,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.ErrorMessage = r.read_string(bytes)?.to_owned(),
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
        + if self.ErrorMessage == String::default() { 0 } else { 1 + sizeof_len((&self.ErrorMessage).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if self.ErrorMessage != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.ErrorMessage))?; }
        Ok(())
    }
}

}

}


