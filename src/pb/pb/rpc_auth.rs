// Automatically generated rust module for 'rpc_auth.proto' file

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

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SendConfirmCodeParam<'a> {
    pub Hash: Cow<'a, str>,
    pub Phone: Cow<'a, str>,
    pub CountryCode: Cow<'a, str>,
    pub Resend: bool,
}

impl<'a> MessageRead<'a> for SendConfirmCodeParam<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Hash = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.Phone = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.CountryCode = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.Resend = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SendConfirmCodeParam<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.Hash == "" { 0 } else { 1 + sizeof_len((&self.Hash).len()) }
        + if self.Phone == "" { 0 } else { 1 + sizeof_len((&self.Phone).len()) }
        + if self.CountryCode == "" { 0 } else { 1 + sizeof_len((&self.CountryCode).len()) }
        + if self.Resend == false { 0 } else { 1 + sizeof_varint(*(&self.Resend) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Hash != "" { w.write_with_tag(10, |w| w.write_string(&**&self.Hash))?; }
        if self.Phone != "" { w.write_with_tag(18, |w| w.write_string(&**&self.Phone))?; }
        if self.CountryCode != "" { w.write_with_tag(26, |w| w.write_string(&**&self.CountryCode))?; }
        if self.Resend != false { w.write_with_tag(32, |w| w.write_bool(*&self.Resend))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SendConfirmCodeResponse<'a> {
    pub Done: bool,
    pub ErrorMessage: Cow<'a, str>,
    pub JustEmailRegister: bool,
    pub SmsNumbers: Vec<Cow<'a, str>>,
    pub IsLogin: bool,
}

impl<'a> MessageRead<'a> for SendConfirmCodeResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.ErrorMessage = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.JustEmailRegister = r.read_bool(bytes)?,
                Ok(34) => msg.SmsNumbers.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.IsLogin = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SendConfirmCodeResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.Done == false { 0 } else { 1 + sizeof_varint(*(&self.Done) as u64) }
        + if self.ErrorMessage == "" { 0 } else { 1 + sizeof_len((&self.ErrorMessage).len()) }
        + if self.JustEmailRegister == false { 0 } else { 1 + sizeof_varint(*(&self.JustEmailRegister) as u64) }
        + self.SmsNumbers.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.IsLogin == false { 0 } else { 1 + sizeof_varint(*(&self.IsLogin) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if self.ErrorMessage != "" { w.write_with_tag(18, |w| w.write_string(&**&self.ErrorMessage))?; }
        if self.JustEmailRegister != false { w.write_with_tag(24, |w| w.write_bool(*&self.JustEmailRegister))?; }
        for s in &self.SmsNumbers { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if self.IsLogin != false { w.write_with_tag(40, |w| w.write_bool(*&self.IsLogin))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ConfirmCodeParam<'a> {
    pub Hash: Cow<'a, str>,
    pub Phone: Cow<'a, str>,
    pub Code: i32,
}

impl<'a> MessageRead<'a> for ConfirmCodeParam<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Hash = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.Phone = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.Code = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ConfirmCodeParam<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.Hash == "" { 0 } else { 1 + sizeof_len((&self.Hash).len()) }
        + if self.Phone == "" { 0 } else { 1 + sizeof_len((&self.Phone).len()) }
        + if self.Code == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.Code) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Hash != "" { w.write_with_tag(10, |w| w.write_string(&**&self.Hash))?; }
        if self.Phone != "" { w.write_with_tag(18, |w| w.write_string(&**&self.Phone))?; }
        if self.Code != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.Code))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ConfirmCodeResponse<'a> {
    pub Done: bool,
    pub ErrorMessage: Cow<'a, str>,
    pub SelfUserView: Option<views::SelfUserView<'a>>,
}

impl<'a> MessageRead<'a> for ConfirmCodeResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.ErrorMessage = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.SelfUserView = Some(r.read_message::<views::SelfUserView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ConfirmCodeResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.Done == false { 0 } else { 1 + sizeof_varint(*(&self.Done) as u64) }
        + if self.ErrorMessage == "" { 0 } else { 1 + sizeof_len((&self.ErrorMessage).len()) }
        + self.SelfUserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if self.ErrorMessage != "" { w.write_with_tag(18, |w| w.write_string(&**&self.ErrorMessage))?; }
        if let Some(ref s) = self.SelfUserView { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SingUpParam<'a> {
    pub Hash: Cow<'a, str>,
    pub FirstName: Cow<'a, str>,
    pub LastName: Cow<'a, str>,
    pub UserName: Cow<'a, str>,
    pub Phone: Cow<'a, str>,
    pub Email: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SingUpParam<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Hash = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.FirstName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.LastName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.UserName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.Phone = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.Email = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SingUpParam<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.Hash == "" { 0 } else { 1 + sizeof_len((&self.Hash).len()) }
        + if self.FirstName == "" { 0 } else { 1 + sizeof_len((&self.FirstName).len()) }
        + if self.LastName == "" { 0 } else { 1 + sizeof_len((&self.LastName).len()) }
        + if self.UserName == "" { 0 } else { 1 + sizeof_len((&self.UserName).len()) }
        + if self.Phone == "" { 0 } else { 1 + sizeof_len((&self.Phone).len()) }
        + if self.Email == "" { 0 } else { 1 + sizeof_len((&self.Email).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Hash != "" { w.write_with_tag(10, |w| w.write_string(&**&self.Hash))?; }
        if self.FirstName != "" { w.write_with_tag(18, |w| w.write_string(&**&self.FirstName))?; }
        if self.LastName != "" { w.write_with_tag(26, |w| w.write_string(&**&self.LastName))?; }
        if self.UserName != "" { w.write_with_tag(34, |w| w.write_string(&**&self.UserName))?; }
        if self.Phone != "" { w.write_with_tag(42, |w| w.write_string(&**&self.Phone))?; }
        if self.Email != "" { w.write_with_tag(50, |w| w.write_string(&**&self.Email))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SingUpResponse<'a> {
    pub Done: bool,
    pub ErrorMessage: Cow<'a, str>,
    pub SelfUserView: Option<views::SelfUserView<'a>>,
}

impl<'a> MessageRead<'a> for SingUpResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.ErrorMessage = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.SelfUserView = Some(r.read_message::<views::SelfUserView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SingUpResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.Done == false { 0 } else { 1 + sizeof_varint(*(&self.Done) as u64) }
        + if self.ErrorMessage == "" { 0 } else { 1 + sizeof_len((&self.ErrorMessage).len()) }
        + self.SelfUserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if self.ErrorMessage != "" { w.write_with_tag(18, |w| w.write_string(&**&self.ErrorMessage))?; }
        if let Some(ref s) = self.SelfUserView { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SingInParam<'a> {
    pub UserNamePhoneEmail: Cow<'a, str>,
    pub Password: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SingInParam<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(34) => msg.UserNamePhoneEmail = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.Password = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SingInParam<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.UserNamePhoneEmail == "" { 0 } else { 1 + sizeof_len((&self.UserNamePhoneEmail).len()) }
        + if self.Password == "" { 0 } else { 1 + sizeof_len((&self.Password).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserNamePhoneEmail != "" { w.write_with_tag(34, |w| w.write_string(&**&self.UserNamePhoneEmail))?; }
        if self.Password != "" { w.write_with_tag(42, |w| w.write_string(&**&self.Password))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SingInResponse<'a> {
    pub Done: bool,
    pub ErrorMessage: Cow<'a, str>,
    pub SelfUserView: Option<views::SelfUserView<'a>>,
}

impl<'a> MessageRead<'a> for SingInResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.ErrorMessage = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.SelfUserView = Some(r.read_message::<views::SelfUserView>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SingInResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.Done == false { 0 } else { 1 + sizeof_varint(*(&self.Done) as u64) }
        + if self.ErrorMessage == "" { 0 } else { 1 + sizeof_len((&self.ErrorMessage).len()) }
        + self.SelfUserView.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if self.ErrorMessage != "" { w.write_with_tag(18, |w| w.write_string(&**&self.ErrorMessage))?; }
        if let Some(ref s) = self.SelfUserView { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LogOutParam { }

impl<'a> MessageRead<'a> for LogOutParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for LogOutParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LogOutResponse<'a> {
    pub Done: bool,
    pub ErrorMessage: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for LogOutResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Done = r.read_bool(bytes)?,
                Ok(18) => msg.ErrorMessage = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LogOutResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.Done == false { 0 } else { 1 + sizeof_varint(*(&self.Done) as u64) }
        + if self.ErrorMessage == "" { 0 } else { 1 + sizeof_len((&self.ErrorMessage).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Done != false { w.write_with_tag(8, |w| w.write_bool(*&self.Done))?; }
        if self.ErrorMessage != "" { w.write_with_tag(18, |w| w.write_string(&**&self.ErrorMessage))?; }
        Ok(())
    }
}


