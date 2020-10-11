// Automatically generated rust module for 'rpc_group.proto' file

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
pub struct GroupAvatarAddParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupAvatarAddParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupAvatarAddParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupAvatarAddResponse { }

impl<'a> MessageRead<'a> for GroupAvatarAddResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupAvatarAddResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupAvatarDeleteParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupAvatarDeleteParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupAvatarDeleteParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupAvatarDeleteResponse { }

impl<'a> MessageRead<'a> for GroupAvatarDeleteResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupAvatarDeleteResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupAvatarGetListParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupAvatarGetListParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupAvatarGetListParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupAvatarGetListResponse { }

impl<'a> MessageRead<'a> for GroupAvatarGetListResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupAvatarGetListResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupSendMessageParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupSendMessageParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupSendMessageParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupSendMessageResponse { }

impl<'a> MessageRead<'a> for GroupSendMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupSendMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupEditMessageParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupEditMessageParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupEditMessageParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupEditMessageResponse { }

impl<'a> MessageRead<'a> for GroupEditMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupEditMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupPinMessageParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupPinMessageParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupPinMessageParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupPinMessageResponse { }

impl<'a> MessageRead<'a> for GroupPinMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupPinMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupUnPinMessageParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupUnPinMessageParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupUnPinMessageParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupUnPinMessageResponse { }

impl<'a> MessageRead<'a> for GroupUnPinMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupUnPinMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupDeleteMessageParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupDeleteMessageParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupDeleteMessageParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupDeleteMessageResponse { }

impl<'a> MessageRead<'a> for GroupDeleteMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupDeleteMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupSetDraftParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupSetDraftParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupSetDraftParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupSetDraftResponse { }

impl<'a> MessageRead<'a> for GroupSetDraftResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupSetDraftResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupGetFullMessageParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupGetFullMessageParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupGetFullMessageParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupGetFullMessageResponse { }

impl<'a> MessageRead<'a> for GroupGetFullMessageResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupGetFullMessageResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupCreateGroupParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupCreateGroupParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupCreateGroupParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupCreateGroupResponse { }

impl<'a> MessageRead<'a> for GroupCreateGroupResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupCreateGroupResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupEditGroupParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupEditGroupParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupEditGroupParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupEditGroupResponse { }

impl<'a> MessageRead<'a> for GroupEditGroupResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupEditGroupResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupDeleteGroupParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupDeleteGroupParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupDeleteGroupParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupDeleteGroupResponse { }

impl<'a> MessageRead<'a> for GroupDeleteGroupResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupDeleteGroupResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupAddAdminParam { }

impl<'a> MessageRead<'a> for GroupAddAdminParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupAddAdminParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupAddAdminResponse { }

impl<'a> MessageRead<'a> for GroupAddAdminResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupAddAdminResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupAddMemberParam { }

impl<'a> MessageRead<'a> for GroupAddMemberParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupAddMemberParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupAddMemberResponse { }

impl<'a> MessageRead<'a> for GroupAddMemberResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupAddMemberResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupRemoveMemberParam { }

impl<'a> MessageRead<'a> for GroupRemoveMemberParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupRemoveMemberParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupRemoveMemberResponse { }

impl<'a> MessageRead<'a> for GroupRemoveMemberResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupRemoveMemberResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupChangeMemberLevelParam { }

impl<'a> MessageRead<'a> for GroupChangeMemberLevelParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupChangeMemberLevelParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupChangeMemberLevelResponse { }

impl<'a> MessageRead<'a> for GroupChangeMemberLevelResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupChangeMemberLevelResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupChangeMemberPermissionParam { }

impl<'a> MessageRead<'a> for GroupChangeMemberPermissionParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupChangeMemberPermissionParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupChangeMemberPermissionResponse { }

impl<'a> MessageRead<'a> for GroupChangeMemberPermissionResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupChangeMemberPermissionResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct JoinGroupParam { }

impl<'a> MessageRead<'a> for JoinGroupParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for JoinGroupParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct JoinGroupResponse { }

impl<'a> MessageRead<'a> for JoinGroupResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for JoinGroupResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupLeaveGroupParam { }

impl<'a> MessageRead<'a> for GroupLeaveGroupParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupLeaveGroupParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupLeaveGroupResponse { }

impl<'a> MessageRead<'a> for GroupLeaveGroupResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupLeaveGroupResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupBanMemberParam { }

impl<'a> MessageRead<'a> for GroupBanMemberParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupBanMemberParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupBanMemberResponse { }

impl<'a> MessageRead<'a> for GroupBanMemberResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupBanMemberResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupChangePrivacyParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupChangePrivacyParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupChangePrivacyParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupChangePrivacyResponse { }

impl<'a> MessageRead<'a> for GroupChangePrivacyResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupChangePrivacyResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupChangeDefaultPermissionParam { }

impl<'a> MessageRead<'a> for GroupChangeDefaultPermissionParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupChangeDefaultPermissionParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupChangeDefaultPermissionResponse { }

impl<'a> MessageRead<'a> for GroupChangeDefaultPermissionResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupChangeDefaultPermissionResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupRevokeLinkParam { }

impl<'a> MessageRead<'a> for GroupRevokeLinkParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupRevokeLinkParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupRevokeLinkResponse { }

impl<'a> MessageRead<'a> for GroupRevokeLinkResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupRevokeLinkResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupChangeUsernameParam { }

impl<'a> MessageRead<'a> for GroupChangeUsernameParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupChangeUsernameParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupChangeUsernameResponse { }

impl<'a> MessageRead<'a> for GroupChangeUsernameResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupChangeUsernameResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupDeleteMessagesParam {
    pub group_id: u32,
}

impl<'a> MessageRead<'a> for GroupDeleteMessagesParam {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.group_id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GroupDeleteMessagesParam {
    fn get_size(&self) -> usize {
        0
        + if self.group_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.group_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.group_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupDeleteMessagesResponse { }

impl<'a> MessageRead<'a> for GroupDeleteMessagesResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupDeleteMessagesResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupDeleteHistoryParam { }

impl<'a> MessageRead<'a> for GroupDeleteHistoryParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupDeleteHistoryParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupDeleteHistoryResponse { }

impl<'a> MessageRead<'a> for GroupDeleteHistoryResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupDeleteHistoryResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupClearHistoryParam { }

impl<'a> MessageRead<'a> for GroupClearHistoryParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupClearHistoryParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupClearHistoryResponse { }

impl<'a> MessageRead<'a> for GroupClearHistoryResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupClearHistoryResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupAvatarChangeParam { }

impl<'a> MessageRead<'a> for GroupAvatarChangeParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupAvatarChangeParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupAvatarChangeResponse { }

impl<'a> MessageRead<'a> for GroupAvatarChangeResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupAvatarChangeResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupSendDoingActionParam { }

impl<'a> MessageRead<'a> for GroupSendDoingActionParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupSendDoingActionParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupSendDoingActionResponse { }

impl<'a> MessageRead<'a> for GroupSendDoingActionResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupSendDoingActionResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupReportGroupParam { }

impl<'a> MessageRead<'a> for GroupReportGroupParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupReportGroupParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupReportGroupResponse { }

impl<'a> MessageRead<'a> for GroupReportGroupResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupReportGroupResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupGetMessagesListParam { }

impl<'a> MessageRead<'a> for GroupGetMessagesListParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupGetMessagesListParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupGetMessagesListResponse { }

impl<'a> MessageRead<'a> for GroupGetMessagesListResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupGetMessagesListResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupGetMediaListParam { }

impl<'a> MessageRead<'a> for GroupGetMediaListParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupGetMediaListParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupGetMediaListResponse { }

impl<'a> MessageRead<'a> for GroupGetMediaListResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupGetMediaListResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupGetMembersListParam { }

impl<'a> MessageRead<'a> for GroupGetMembersListParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupGetMembersListParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupGetMembersListResponse { }

impl<'a> MessageRead<'a> for GroupGetMembersListResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupGetMembersListResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupGetAdminsListParam { }

impl<'a> MessageRead<'a> for GroupGetAdminsListParam {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupGetAdminsListParam { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupGetAdminsListResponse { }

impl<'a> MessageRead<'a> for GroupGetAdminsListResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for GroupGetAdminsListResponse { }


