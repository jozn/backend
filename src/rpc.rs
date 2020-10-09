use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::body;
use serde::{Deserialize, Serialize};
use quick_protobuf::{BytesReader, BytesWriter};
use quick_protobuf::{MessageRead,MessageWrite,Writer};

use crate::{pb,com, pb::sys::Invoke,com::*, rpc_fns};

pub fn server_rpc(act : Invoke) -> Result<Vec<u8>,GenErr> {
    let up = UserParam{};

    match act.method {
    
    // service: RPC_Account
        706069694 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChangePhoneNumberParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChangePhoneNumber(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Auth
        939965206 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::SendConfirmCodeParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::SendConfirmCode(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1740258084 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ConfirmCodeParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ConfirmCode(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        291193302 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::SingUpParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::SingUp(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1017957090 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::SingInParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::SingIn(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1283119009 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::LogOutParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::LogOut(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Channel
        143251225 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelCreateChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelCreateChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        189471894 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelEditChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelEditChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1494483355 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelDeleteChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelDeleteChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        780397316 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelAddAuthorParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelAddAuthor(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        93233821 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelChangeAuthorPermissionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelChangeAuthorPermission(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        419542304 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelRemoveAuthorParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelRemoveAuthor(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        744563779 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelFollowChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelFollowChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        959512423 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelUnFollowChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelUnFollowChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        869709257 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelRemoveFollowersParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelRemoveFollowers(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1367898912 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelSubscribeParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelSubscribe(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        858172401 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelUnSubscribeParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelUnSubscribe(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        729024592 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelRemoveSubscribersParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelRemoveSubscribers(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        79012409 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelChangePrivacyParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelChangePrivacy(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1582638498 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelChangeDefaultPermissionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelChangeDefaultPermission(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1912530021 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelRevokeLinkParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelRevokeLink(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        983884462 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelChangeUsernameParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelChangeUsername(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        2037016989 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelBlockChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelBlockChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1200751231 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelSendMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelSendMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        727437726 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelEditMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelEditMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        259263709 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelPinMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelPinMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        113943649 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelUnPinMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelUnPinMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        644189206 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelDeleteMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelDeleteMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        2124822181 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelDeleteMessagesParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelDeleteMessages(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1164398815 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelClearHistoryParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelClearHistory(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1021808696 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelAvatarAddParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelAvatarAdd(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1968579501 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelAvatarChangeParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelAvatarChange(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1626010891 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelAvatarDeleteParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelAvatarDelete(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1925044843 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelAvatarGetListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelAvatarGetList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        973237257 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelSendDoingActionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelSendDoingAction(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        792938145 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelReportChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelReportChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        2053528327 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelReportMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelReportMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1684531258 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelGetFullParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelGetFull(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1339072968 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelGetMessagesListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelGetMessagesList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        985772653 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelGetMediaListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelGetMediaList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1373284924 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelGetAuthorsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelGetAuthors(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1747172143 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelGetFollowersParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelGetFollowers(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1838438980 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelGetFollowingsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelGetFollowings(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        2146806736 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelGetSubscribersParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelGetSubscribers(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1674411747 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelBlockedParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelBlocked(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1403193015 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChannelSetDraftParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChannelSetDraft(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Chat
        1131621475 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChatSendMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChatSendMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1806258329 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChatEditMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChatEditMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        933526170 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChatDeleteMessagesParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChatDeleteMessages(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1088992782 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChatDeleteHistoryParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChatDeleteHistory(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1319324241 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChatSendDoingActionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChatSendDoingAction(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1345425871 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChatReportChatParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChatReportChat(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1768678453 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChatGetFullMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChatGetFull(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        121549718 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChatGetMessagesListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChatGetMessagesList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1346774525 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::ChatGetMediaListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::ChatGetMediaList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Direct
        1478067518 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectDeleteDirectParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectDeleteDirect(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        2041790485 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectChangeTitleParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectChangeTitle(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        548699291 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectSetCustomNotificationParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectSetCustomNotification(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1417285757 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectSendActionDoingParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectSendActionDoing(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1860345925 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectSetDraftParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectSetDraft(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1291891637 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectDeleteDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectDeleteDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1801774787 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectMarkAsReadParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectMarkAsRead(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        313746334 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectMarkAsUnReadParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectMarkAsUnRead(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1179089068 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectPinDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectPinDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1517245560 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectUnPinDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectUnPinDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1441782770 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectArchiveDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectArchiveDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1951553867 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectUnArchiveDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectUnArchiveDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        904052140 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectClearHistoriesParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectClearHistories(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1138477048 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectMuteDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectMuteDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1691834263 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectUnMuteDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectUnMuteDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1878673022 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectCreateFolderParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectCreateFolder(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1861381591 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectChangeFolderParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectChangeFolder(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1818954127 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectRemoveFromFolderParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectRemoveFromFolder(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1264591958 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectReordersFolderParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectReordersFolder(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        962281627 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectDeleteFolderParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectDeleteFolder(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1570934969 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectGetChatsListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectGetChatsList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        545957996 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectGetGroupsListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectGetGroupsList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1608173619 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectGetChannelsListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectGetChannelsList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1384523712 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectGetFoldersListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectGetFoldersList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        611850722 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DirectGetFoldersFullListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DirectGetFoldersFullList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_General
        101973561 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::EchoParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::Echo(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1897027349 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::CheckUserNameParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::CheckUserName(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Group
        1205960678 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupCreateGroupParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupCreateGroup(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1665019493 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupEditGroupParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupEditGroup(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        365183375 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupDeleteGroupParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupDeleteGroup(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        958971956 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupAddAdminParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupAddAdmin(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        676599227 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupAddMemberParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupAddMember(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        2012702964 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupRemoveMemberParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupRemoveMember(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        589574238 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupChangeMemberLevelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupChangeMemberLevel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        2132464067 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupChangeMemberPermissionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupChangeMemberPermission(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        591743429 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::JoinGroupParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupJoinGroup(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        361834630 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupLeaveGroupParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupLeaveGroup(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        548504852 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupBanMemberParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupBanMember(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1497988410 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupChangePrivacyParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupChangePrivacy(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        605792138 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupChangeDefaultPermissionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupChangeDefaultPermission(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        406592509 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupRevokeLinkParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupRevokeLink(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        832997038 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupChangeUsernameParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupChangeUsername(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        599852950 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupSendMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupSendMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        742937895 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupEditMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupEditMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        184560027 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupPinMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupPinMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1290613173 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupUnPinMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupUnPinMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        393991035 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupDeleteMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupDeleteMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        276700675 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupDeleteMessagesParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupDeleteMessages(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1270953793 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupDeleteHistoryParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupDeleteHistory(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1352552449 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupClearHistoryParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupClearHistory(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1202058216 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupAvatarAddParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupAvatarAdd(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        108612523 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupAvatarChangeParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupAvatarChange(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        775862697 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupAvatarDeleteParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupAvatarDelete(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        939443722 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupAvatarGetListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupAvatarGetList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        2022474356 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupSendDoingActionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupSendDoingAction(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1759704420 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupReportGroupParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupReportGroup(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        200351324 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupGetFullMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupGetFull(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1541835459 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupGetMessagesListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupGetMessagesList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        2143016912 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupGetMediaListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupGetMediaList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        429215412 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupGetMembersListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupGetMembersList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        332260610 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupGetAdminsListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupGetAdminsList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        77668156 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::GroupSetDraftParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::GroupSetDraft(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Social
        1222124115 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::AddCommentParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::AddComment(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1684680875 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::DeleteCommentParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::DeleteComment(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        527415306 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::EditCommentParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::EditComment(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        2086146002 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::LikeCommentParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::LikeComment(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1118533600 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::AddSeenPostsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::AddSeenPosts(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1313969677 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::LikePostParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::LikePost(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1332796256 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::UnLikePostParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::UnLikePost(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        655898778 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::FollowChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::FollowChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        483078047 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::UnFollowChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::UnFollowChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1225489769 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::PinChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::PinChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1585401362 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::UnPinChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::UnPinChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        1902848482 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::BlockChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::BlockChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        305468874 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::UnBlockChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::UnBlockChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Upload
        1702285478 => {
            let vec: Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec)
                .read_message::<pb::UploadFileParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc_fns::UploadFile(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
        _ => {
            Err(GenErr::NoRpcMatch)
        }
    }
}
