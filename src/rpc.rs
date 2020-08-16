use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::body;
use serde::{Deserialize, Serialize};
use quick_protobuf::{BytesReader, BytesWriter};
use quick_protobuf::{MessageRead,MessageWrite,Writer};

use crate::{pb,com, pb::sys::Invoke,com::*};

pub fn server_rpc(act : Invoke) -> Result<Vec<u8>,GenErr> {
    let up = UserParam{};

    match act.method {
    
    // service: RPC_Account
        706069694 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChangePhoneNumberParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChangePhoneNumber(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
    
    // service: RPC_Auth
        939965206 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::SendConfirmCodeParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::SendConfirmCode(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1740258084 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ConfirmCodeParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ConfirmCode(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        291193302 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::SingUpParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::SingUp(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1017957090 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::SingInParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::SingIn(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1283119009 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::LogOutParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::LogOut(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
    
    // service: RPC_Channel
        1021808696 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelAvatarAddParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelAvatarAdd(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1626010891 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelAvatarDeleteParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelAvatarDelete(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1925044843 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelAvatarGetListParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelAvatarGetList(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        541983522 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelCreateParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelCreate(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1322894346 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelEditParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelEdit(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1665963658 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelDeleteParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelDelete(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1200751231 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelSendMessageParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelSendMessage(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        727437726 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelEditMessageParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelEditMessage(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        259263709 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelPinMessageParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelPinMessage(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        113943649 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelUnPinMessageParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelUnPinMessage(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        644189206 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelDeleteMessageParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelDeleteMessage(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1403193015 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelSetDraftParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelSetDraft(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1606133811 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelChangePrivacyParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelChangePrivacyDraft(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1684531258 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ChannelGetFullMessageParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ChannelGetFull(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
    
    // service: RPC_Chat
        911990565 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::DeleteChatParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::DeleteChat(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
    
    // service: RPC_Direct
        458029812 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::DirectAddMessageParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::DirectAddMessage(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
    
    // service: RPC_General
        101973561 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::EchoParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::Echo(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1897027349 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::CheckUserNameParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::CheckUserName(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
    
    // service: RPC_Social
        1222124115 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::AddCommentParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::AddComment(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1684680875 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::DeleteCommentParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::DeleteComment(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        527415306 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::EditCommentParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::EditComment(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        2086146002 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::LikeCommentParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::LikeComment(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1118533600 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::AddSeenPostsParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::AddSeenPosts(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1313969677 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::LikePostParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::LikePost(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1332796256 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::UnLikePostParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::UnLikePost(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        655898778 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::FollowChannelParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::FollowChannel(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        483078047 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::UnFollowChannelParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::UnFollowChannel(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1225489769 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::PinChannelParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::PinChannel(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1585401362 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::UnPinChannelParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::UnPinChannel(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1902848482 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::BlockChannelParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::BlockChannel(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        305468874 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::UnBlockChannelParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::UnBlockChannel(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
    
    // service: RPC_Upload
        1702285478 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::UploadFileParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::UploadFile(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
    
        _ => {
           Err(GenErr{})
        }
    }
}

pub mod rpc {
    use super::*;

    
    // service: RPC_Account
    pub fn ChangePhoneNumber(up: &UserParam, param: pb::ChangePhoneNumberParam) -> Result<pb::ChangePhoneNumberResponse, GenErr> {
        Ok(pb::ChangePhoneNumberResponse::default())
    }
    
    // service: RPC_Auth
    pub fn SendConfirmCode(up: &UserParam, param: pb::SendConfirmCodeParam) -> Result<pb::SendConfirmCodeResponse, GenErr> {
        Ok(pb::SendConfirmCodeResponse::default())
    }
    pub fn ConfirmCode(up: &UserParam, param: pb::ConfirmCodeParam) -> Result<pb::ConfirmCodeResponse, GenErr> {
        Ok(pb::ConfirmCodeResponse::default())
    }
    pub fn SingUp(up: &UserParam, param: pb::SingUpParam) -> Result<pb::SingUpResponse, GenErr> {
        Ok(pb::SingUpResponse::default())
    }
    pub fn SingIn(up: &UserParam, param: pb::SingInParam) -> Result<pb::SingInResponse, GenErr> {
        Ok(pb::SingInResponse::default())
    }
    pub fn LogOut(up: &UserParam, param: pb::LogOutParam) -> Result<pb::LogOutResponse, GenErr> {
        Ok(pb::LogOutResponse::default())
    }
    
    // service: RPC_Channel
    pub fn ChannelAvatarAdd(up: &UserParam, param: pb::ChannelAvatarAddParam) -> Result<pb::ChannelAvatarAddResponse, GenErr> {
        Ok(pb::ChannelAvatarAddResponse::default())
    }
    pub fn ChannelAvatarDelete(up: &UserParam, param: pb::ChannelAvatarDeleteParam) -> Result<pb::ChannelAvatarDeleteResponse, GenErr> {
        Ok(pb::ChannelAvatarDeleteResponse::default())
    }
    pub fn ChannelAvatarGetList(up: &UserParam, param: pb::ChannelAvatarGetListParam) -> Result<pb::ChannelAvatarGetListResponse, GenErr> {
        Ok(pb::ChannelAvatarGetListResponse::default())
    }
    pub fn ChannelCreate(up: &UserParam, param: pb::ChannelCreateParam) -> Result<pb::ChannelCreateResponse, GenErr> {
        Ok(pb::ChannelCreateResponse::default())
    }
    pub fn ChannelEdit(up: &UserParam, param: pb::ChannelEditParam) -> Result<pb::ChannelEditResponse, GenErr> {
        Ok(pb::ChannelEditResponse::default())
    }
    pub fn ChannelDelete(up: &UserParam, param: pb::ChannelDeleteParam) -> Result<pb::ChannelDeleteResponse, GenErr> {
        Ok(pb::ChannelDeleteResponse::default())
    }
    pub fn ChannelSendMessage(up: &UserParam, param: pb::ChannelSendMessageParam) -> Result<pb::ChannelSendMessageResponse, GenErr> {
        Ok(pb::ChannelSendMessageResponse::default())
    }
    pub fn ChannelEditMessage(up: &UserParam, param: pb::ChannelEditMessageParam) -> Result<pb::ChannelEditMessageResponse, GenErr> {
        Ok(pb::ChannelEditMessageResponse::default())
    }
    pub fn ChannelPinMessage(up: &UserParam, param: pb::ChannelPinMessageParam) -> Result<pb::ChannelPinMessageResponse, GenErr> {
        Ok(pb::ChannelPinMessageResponse::default())
    }
    pub fn ChannelUnPinMessage(up: &UserParam, param: pb::ChannelUnPinMessageParam) -> Result<pb::ChannelUnPinMessageResponse, GenErr> {
        Ok(pb::ChannelUnPinMessageResponse::default())
    }
    pub fn ChannelDeleteMessage(up: &UserParam, param: pb::ChannelDeleteMessageParam) -> Result<pb::ChannelDeleteMessageResponse, GenErr> {
        Ok(pb::ChannelDeleteMessageResponse::default())
    }
    pub fn ChannelSetDraft(up: &UserParam, param: pb::ChannelSetDraftParam) -> Result<pb::ChannelSetDraftResponse, GenErr> {
        Ok(pb::ChannelSetDraftResponse::default())
    }
    pub fn ChannelChangePrivacyDraft(up: &UserParam, param: pb::ChannelChangePrivacyParam) -> Result<pb::ChannelChangePrivacyResponse, GenErr> {
        Ok(pb::ChannelChangePrivacyResponse::default())
    }
    pub fn ChannelGetFull(up: &UserParam, param: pb::ChannelGetFullMessageParam) -> Result<pb::ChannelGetFullMessageResponse, GenErr> {
        Ok(pb::ChannelGetFullMessageResponse::default())
    }
    
    // service: RPC_Chat
    pub fn DeleteChat(up: &UserParam, param: pb::DeleteChatParam) -> Result<pb::DeleteChatResponse, GenErr> {
        Ok(pb::DeleteChatResponse::default())
    }
    
    // service: RPC_Direct
    pub fn DirectAddMessage(up: &UserParam, param: pb::DirectAddMessageParam) -> Result<pb::DirectAddMessageResponse, GenErr> {
        Ok(pb::DirectAddMessageResponse::default())
    }
    
    // service: RPC_General
    pub fn Echo(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
    pub fn CheckUserName(up: &UserParam, param: pb::CheckUserNameParam) -> Result<pb::CheckUserNameResponse, GenErr> {
        Ok(pb::CheckUserNameResponse::default())
    }
    
    // service: RPC_Social
    pub fn AddComment(up: &UserParam, param: pb::AddCommentParam) -> Result<pb::AddCommentResponse, GenErr> {
        Ok(pb::AddCommentResponse::default())
    }
    pub fn DeleteComment(up: &UserParam, param: pb::DeleteCommentParam) -> Result<pb::DeleteCommentResponse, GenErr> {
        Ok(pb::DeleteCommentResponse::default())
    }
    pub fn EditComment(up: &UserParam, param: pb::EditCommentParam) -> Result<pb::EditCommentResponse, GenErr> {
        Ok(pb::EditCommentResponse::default())
    }
    pub fn LikeComment(up: &UserParam, param: pb::LikeCommentParam) -> Result<pb::LikeCommentResponse, GenErr> {
        Ok(pb::LikeCommentResponse::default())
    }
    pub fn AddSeenPosts(up: &UserParam, param: pb::AddSeenPostsParam) -> Result<pb::AddSeenPostsResponse, GenErr> {
        Ok(pb::AddSeenPostsResponse::default())
    }
    pub fn LikePost(up: &UserParam, param: pb::LikePostParam) -> Result<pb::LikePostResponse, GenErr> {
        Ok(pb::LikePostResponse::default())
    }
    pub fn UnLikePost(up: &UserParam, param: pb::UnLikePostParam) -> Result<pb::UnLikePostResponse, GenErr> {
        Ok(pb::UnLikePostResponse::default())
    }
    pub fn FollowChannel(up: &UserParam, param: pb::FollowChannelParam) -> Result<pb::FollowChannelResponse, GenErr> {
        Ok(pb::FollowChannelResponse::default())
    }
    pub fn UnFollowChannel(up: &UserParam, param: pb::UnFollowChannelParam) -> Result<pb::UnFollowChannelResponse, GenErr> {
        Ok(pb::UnFollowChannelResponse::default())
    }
    pub fn PinChannel(up: &UserParam, param: pb::PinChannelParam) -> Result<pb::PinChannelResponse, GenErr> {
        Ok(pb::PinChannelResponse::default())
    }
    pub fn UnPinChannel(up: &UserParam, param: pb::UnPinChannelParam) -> Result<pb::UnPinChannelResponse, GenErr> {
        Ok(pb::UnPinChannelResponse::default())
    }
    pub fn BlockChannel(up: &UserParam, param: pb::BlockChannelParam) -> Result<pb::BlockChannelResponse, GenErr> {
        Ok(pb::BlockChannelResponse::default())
    }
    pub fn UnBlockChannel(up: &UserParam, param: pb::UnBlockChannelParam) -> Result<pb::UnBlockChannelResponse, GenErr> {
        Ok(pb::UnBlockChannelResponse::default())
    }
    
    // service: RPC_Upload
    pub fn UploadFile(up: &UserParam, param: pb::UploadFileParam) -> Result<pb::UploadFileResponse, GenErr> {
        Ok(pb::UploadFileResponse::default())
    }
    

    pub fn check_username(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
}
