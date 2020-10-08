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
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChangePhoneNumberParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChangePhoneNumber(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
    
    // service: RPC_Auth
        939965206 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::SendConfirmCodeParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::SendConfirmCode(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1740258084 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ConfirmCodeParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ConfirmCode(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        291193302 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::SingUpParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::SingUp(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1017957090 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::SingInParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::SingIn(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1283119009 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::LogOutParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::LogOut(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
    
    // service: RPC_Channel
        143251225 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelCreateChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelCreateChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        189471894 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelEditChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelEditChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1494483355 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelDeleteChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelDeleteChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        780397316 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelAddAuthorParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelAddAuthor(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        93233821 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelChangeAuthorPermissionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelChangeAuthorPermission(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        419542304 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelRemoveAuthorParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelRemoveAuthor(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        744563779 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelFollowChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelFollowChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        959512423 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelUnFollowChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelUnFollowChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        869709257 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelRemoveFollowersParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelRemoveFollowers(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1367898912 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelSubscribeParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelSubscribe(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        858172401 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelUnSubscribeParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelUnSubscribe(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        729024592 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelRemoveSubscribersParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelRemoveSubscribers(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        79012409 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelChangePrivacyParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelChangePrivacy(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1582638498 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelChangeDefaultPermissionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelChangeDefaultPermission(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1912530021 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelRevokeLinkParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelRevokeLink(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        983884462 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelChangeUsernameParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelChangeUsername(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        2037016989 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelBlockChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelBlockChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1200751231 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelSendMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelSendMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        727437726 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelEditMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelEditMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        259263709 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelPinMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelPinMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        113943649 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelUnPinMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelUnPinMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        644189206 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelDeleteMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelDeleteMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        2124822181 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelDeleteMessagesParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelDeleteMessages(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1164398815 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelClearHistoryParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelClearHistory(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1021808696 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelAvatarAddParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelAvatarAdd(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1968579501 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelAvatarChangeParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelAvatarChange(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1626010891 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelAvatarDeleteParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelAvatarDelete(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1925044843 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelAvatarGetListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelAvatarGetList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        973237257 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelSendDoingActionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelSendDoingAction(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        792938145 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelReportChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelReportChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        2053528327 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelReportMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelReportMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1684531258 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelGetFullParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelGetFull(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1339072968 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelGetMessagesListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelGetMessagesList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        985772653 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelGetMediaListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelGetMediaList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1373284924 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelGetAuthorsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelGetAuthors(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1747172143 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelGetFollowersParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelGetFollowers(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1838438980 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelGetFollowingsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelGetFollowings(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        2146806736 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelGetSubscribersParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelGetSubscribers(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1674411747 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelBlockedParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelBlocked(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1403193015 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChannelSetDraftParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChannelSetDraft(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
    
    // service: RPC_Chat
        1131621475 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChatSendMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChatSendMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1806258329 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChatEditMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChatEditMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        933526170 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChatDeleteMessagesParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChatDeleteMessages(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1088992782 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChatDeleteHistoryParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChatDeleteHistory(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1319324241 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChatSendDoingActionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChatSendDoingAction(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1345425871 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChatReportChatParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChatReportChat(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1768678453 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChatGetFullMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChatGetFull(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        121549718 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChatGetMessagesListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChatGetMessagesList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1346774525 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::ChatGetMediaListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::ChatGetMediaList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
    
    // service: RPC_Direct
        1478067518 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectDeleteDirectParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectDeleteDirect(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        2041790485 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectChangeTitleParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectChangeTitle(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        548699291 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectSetCustomNotificationParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectSetCustomNotification(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1417285757 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectSendActionDoingParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectSendActionDoing(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1860345925 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectSetDraftParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectSetDraft(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1291891637 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectDeleteDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectDeleteDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1801774787 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectMarkAsReadParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectMarkAsRead(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        313746334 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectMarkAsUnReadParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectMarkAsUnRead(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1179089068 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectPinDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectPinDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1517245560 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectUnPinDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectUnPinDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1441782770 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectArchiveDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectArchiveDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1951553867 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectUnArchiveDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectUnArchiveDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        904052140 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectClearHistoriesParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectClearHistories(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1138477048 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectMuteDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectMuteDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1691834263 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectUnMuteDirectsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectUnMuteDirects(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1878673022 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectCreateFolderParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectCreateFolder(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1861381591 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectChangeFolderParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectChangeFolder(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1818954127 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectRemoveFromFolderParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectRemoveFromFolder(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1264591958 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectReordersFolderParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectReordersFolder(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        962281627 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectDeleteFolderParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectDeleteFolder(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1570934969 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectGetChatsListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectGetChatsList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        545957996 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectGetGroupsListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectGetGroupsList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1608173619 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectGetChannelsListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectGetChannelsList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1384523712 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectGetFoldersListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectGetFoldersList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        611850722 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DirectGetFoldersFullListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DirectGetFoldersFullList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
    
    // service: RPC_General
        101973561 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::EchoParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::Echo(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1897027349 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::CheckUserNameParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::CheckUserName(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
    
    // service: RPC_Group
        1205960678 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupCreateGroupParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupCreateGroup(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1665019493 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupEditGroupParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupEditGroup(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        365183375 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupDeleteGroupParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupDeleteGroup(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        958971956 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupAddAdminParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupAddAdmin(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        676599227 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupAddMemberParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupAddMember(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        2012702964 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupRemoveMemberParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupRemoveMember(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        589574238 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupChangeMemberLevelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupChangeMemberLevel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        2132464067 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupChangeMemberPermissionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupChangeMemberPermission(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        591743429 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::JoinGroupParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupJoinGroup(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        361834630 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupLeaveGroupParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupLeaveGroup(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        548504852 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupBanMemberParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupBanMember(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1497988410 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupChangePrivacyParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupChangePrivacy(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        605792138 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupChangeDefaultPermissionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupChangeDefaultPermission(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        406592509 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupRevokeLinkParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupRevokeLink(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        832997038 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupChangeUsernameParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupChangeUsername(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        599852950 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupSendMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupSendMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        742937895 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupEditMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupEditMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        184560027 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupPinMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupPinMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1290613173 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupUnPinMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupUnPinMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        393991035 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupDeleteMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupDeleteMessage(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        276700675 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupDeleteMessagesParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupDeleteMessages(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1270953793 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupDeleteHistoryParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupDeleteHistory(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1352552449 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupClearHistoryParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupClearHistory(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1202058216 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupAvatarAddParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupAvatarAdd(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        108612523 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupAvatarChangeParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupAvatarChange(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        775862697 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupAvatarDeleteParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupAvatarDelete(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        939443722 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupAvatarGetListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupAvatarGetList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        2022474356 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupSendDoingActionParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupSendDoingAction(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1759704420 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupReportGroupParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupReportGroup(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        200351324 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupGetFullMessageParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupGetFull(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1541835459 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupGetMessagesListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupGetMessagesList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        2143016912 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupGetMediaListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupGetMediaList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        429215412 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupGetMembersListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupGetMembersList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        332260610 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupGetAdminsListParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupGetAdminsList(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        77668156 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::GroupSetDraftParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::GroupSetDraft(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
    
    // service: RPC_Social
        1222124115 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::AddCommentParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::AddComment(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1684680875 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::DeleteCommentParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::DeleteComment(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        527415306 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::EditCommentParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::EditComment(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        2086146002 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::LikeCommentParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::LikeComment(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1118533600 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::AddSeenPostsParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::AddSeenPosts(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1313969677 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::LikePostParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::LikePost(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1332796256 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::UnLikePostParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::UnLikePost(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        655898778 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::FollowChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::FollowChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        483078047 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::UnFollowChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::UnFollowChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1225489769 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::PinChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::PinChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1585401362 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::UnPinChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::UnPinChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        1902848482 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::BlockChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::BlockChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
        305468874 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::UnBlockChannelParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::UnBlockChannel(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
    
    // service: RPC_Upload
        1702285478 => {
            let vec :Vec<u8> = vec![];
            let rpc_param = BytesReader::from_bytes(&vec).read_message::<pb::UploadFileParam>(&act.rpc_data);

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let result = rpc::UploadFile(&up, param)?;

                let mut out_bytes = Vec::new();
                let _result = Writer::new(&mut out_bytes).write_message(&result);

                Ok(out_bytes)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        },
    
        _ => {
            Err(GenErr::NoRpcMatch)
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
    pub fn ChannelCreateChannel(up: &UserParam, param: pb::ChannelCreateChannelParam) -> Result<pb::ChannelCreateChannelResponse, GenErr> {
        Ok(pb::ChannelCreateChannelResponse::default())
    }
    pub fn ChannelEditChannel(up: &UserParam, param: pb::ChannelEditChannelParam) -> Result<pb::ChannelEditChannelResponse, GenErr> {
        Ok(pb::ChannelEditChannelResponse::default())
    }
    pub fn ChannelDeleteChannel(up: &UserParam, param: pb::ChannelDeleteChannelParam) -> Result<pb::ChannelDeleteChannelResponse, GenErr> {
        Ok(pb::ChannelDeleteChannelResponse::default())
    }
    pub fn ChannelAddAuthor(up: &UserParam, param: pb::ChannelAddAuthorParam) -> Result<pb::ChannelAddAuthorResponse, GenErr> {
        Ok(pb::ChannelAddAuthorResponse::default())
    }
    pub fn ChannelChangeAuthorPermission(up: &UserParam, param: pb::ChannelChangeAuthorPermissionParam) -> Result<pb::ChannelChangeAuthorPermissionResponse, GenErr> {
        Ok(pb::ChannelChangeAuthorPermissionResponse::default())
    }
    pub fn ChannelRemoveAuthor(up: &UserParam, param: pb::ChannelRemoveAuthorParam) -> Result<pb::ChannelRemoveAuthorResponse, GenErr> {
        Ok(pb::ChannelRemoveAuthorResponse::default())
    }
    pub fn ChannelFollowChannel(up: &UserParam, param: pb::ChannelFollowChannelParam) -> Result<pb::ChannelFollowChannelResponse, GenErr> {
        Ok(pb::ChannelFollowChannelResponse::default())
    }
    pub fn ChannelUnFollowChannel(up: &UserParam, param: pb::ChannelUnFollowChannelParam) -> Result<pb::ChannelUnFollowChannelResponse, GenErr> {
        Ok(pb::ChannelUnFollowChannelResponse::default())
    }
    pub fn ChannelRemoveFollowers(up: &UserParam, param: pb::ChannelRemoveFollowersParam) -> Result<pb::ChannelRemoveFollowersResponse, GenErr> {
        Ok(pb::ChannelRemoveFollowersResponse::default())
    }
    pub fn ChannelSubscribe(up: &UserParam, param: pb::ChannelSubscribeParam) -> Result<pb::ChannelSubscribeResponse, GenErr> {
        Ok(pb::ChannelSubscribeResponse::default())
    }
    pub fn ChannelUnSubscribe(up: &UserParam, param: pb::ChannelUnSubscribeParam) -> Result<pb::ChannelUnSubscribeResponse, GenErr> {
        Ok(pb::ChannelUnSubscribeResponse::default())
    }
    pub fn ChannelRemoveSubscribers(up: &UserParam, param: pb::ChannelRemoveSubscribersParam) -> Result<pb::ChannelRemoveSubscribersResponse, GenErr> {
        Ok(pb::ChannelRemoveSubscribersResponse::default())
    }
    pub fn ChannelChangePrivacy(up: &UserParam, param: pb::ChannelChangePrivacyParam) -> Result<pb::ChannelChangePrivacyResponse, GenErr> {
        Ok(pb::ChannelChangePrivacyResponse::default())
    }
    pub fn ChannelChangeDefaultPermission(up: &UserParam, param: pb::ChannelChangeDefaultPermissionParam) -> Result<pb::ChannelChangeDefaultPermissionResponse, GenErr> {
        Ok(pb::ChannelChangeDefaultPermissionResponse::default())
    }
    pub fn ChannelRevokeLink(up: &UserParam, param: pb::ChannelRevokeLinkParam) -> Result<pb::ChannelRevokeLinkResponse, GenErr> {
        Ok(pb::ChannelRevokeLinkResponse::default())
    }
    pub fn ChannelChangeUsername(up: &UserParam, param: pb::ChannelChangeUsernameParam) -> Result<pb::ChannelChangeUsernameResponse, GenErr> {
        Ok(pb::ChannelChangeUsernameResponse::default())
    }
    pub fn ChannelBlockChannel(up: &UserParam, param: pb::ChannelBlockChannelParam) -> Result<pb::ChannelBlockChannelResponse, GenErr> {
        Ok(pb::ChannelBlockChannelResponse::default())
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
    pub fn ChannelDeleteMessages(up: &UserParam, param: pb::ChannelDeleteMessagesParam) -> Result<pb::ChannelDeleteMessagesResponse, GenErr> {
        Ok(pb::ChannelDeleteMessagesResponse::default())
    }
    pub fn ChannelClearHistory(up: &UserParam, param: pb::ChannelClearHistoryParam) -> Result<pb::ChannelClearHistoryResponse, GenErr> {
        Ok(pb::ChannelClearHistoryResponse::default())
    }
    pub fn ChannelAvatarAdd(up: &UserParam, param: pb::ChannelAvatarAddParam) -> Result<pb::ChannelAvatarAddResponse, GenErr> {
        Ok(pb::ChannelAvatarAddResponse::default())
    }
    pub fn ChannelAvatarChange(up: &UserParam, param: pb::ChannelAvatarChangeParam) -> Result<pb::ChannelAvatarChangeResponse, GenErr> {
        Ok(pb::ChannelAvatarChangeResponse::default())
    }
    pub fn ChannelAvatarDelete(up: &UserParam, param: pb::ChannelAvatarDeleteParam) -> Result<pb::ChannelAvatarDeleteResponse, GenErr> {
        Ok(pb::ChannelAvatarDeleteResponse::default())
    }
    pub fn ChannelAvatarGetList(up: &UserParam, param: pb::ChannelAvatarGetListParam) -> Result<pb::ChannelAvatarGetListResponse, GenErr> {
        Ok(pb::ChannelAvatarGetListResponse::default())
    }
    pub fn ChannelSendDoingAction(up: &UserParam, param: pb::ChannelSendDoingActionParam) -> Result<pb::ChannelSendDoingActionResponse, GenErr> {
        Ok(pb::ChannelSendDoingActionResponse::default())
    }
    pub fn ChannelReportChannel(up: &UserParam, param: pb::ChannelReportChannelParam) -> Result<pb::ChannelReportChannelResponse, GenErr> {
        Ok(pb::ChannelReportChannelResponse::default())
    }
    pub fn ChannelReportMessage(up: &UserParam, param: pb::ChannelReportMessageParam) -> Result<pb::ChannelReportMessageResponse, GenErr> {
        Ok(pb::ChannelReportMessageResponse::default())
    }
    pub fn ChannelGetFull(up: &UserParam, param: pb::ChannelGetFullParam) -> Result<pb::ChannelGetFullResponse, GenErr> {
        Ok(pb::ChannelGetFullResponse::default())
    }
    pub fn ChannelGetMessagesList(up: &UserParam, param: pb::ChannelGetMessagesListParam) -> Result<pb::ChannelGetMessagesListResponse, GenErr> {
        Ok(pb::ChannelGetMessagesListResponse::default())
    }
    pub fn ChannelGetMediaList(up: &UserParam, param: pb::ChannelGetMediaListParam) -> Result<pb::ChannelGetMediaListResponse, GenErr> {
        Ok(pb::ChannelGetMediaListResponse::default())
    }
    pub fn ChannelGetAuthors(up: &UserParam, param: pb::ChannelGetAuthorsParam) -> Result<pb::ChannelGetAuthorsResponse, GenErr> {
        Ok(pb::ChannelGetAuthorsResponse::default())
    }
    pub fn ChannelGetFollowers(up: &UserParam, param: pb::ChannelGetFollowersParam) -> Result<pb::ChannelGetFollowersResponse, GenErr> {
        Ok(pb::ChannelGetFollowersResponse::default())
    }
    pub fn ChannelGetFollowings(up: &UserParam, param: pb::ChannelGetFollowingsParam) -> Result<pb::ChannelGetFollowingsResponse, GenErr> {
        Ok(pb::ChannelGetFollowingsResponse::default())
    }
    pub fn ChannelGetSubscribers(up: &UserParam, param: pb::ChannelGetSubscribersParam) -> Result<pb::ChannelGetSubscribersResponse, GenErr> {
        Ok(pb::ChannelGetSubscribersResponse::default())
    }
    pub fn ChannelBlocked(up: &UserParam, param: pb::ChannelBlockedParam) -> Result<pb::ChannelBlockedResponse, GenErr> {
        Ok(pb::ChannelBlockedResponse::default())
    }
    pub fn ChannelSetDraft(up: &UserParam, param: pb::ChannelSetDraftParam) -> Result<pb::ChannelSetDraftResponse, GenErr> {
        Ok(pb::ChannelSetDraftResponse::default())
    }
    
    // service: RPC_Chat
    pub fn ChatSendMessage(up: &UserParam, param: pb::ChatSendMessageParam) -> Result<pb::ChatSendMessageResponse, GenErr> {
        Ok(pb::ChatSendMessageResponse::default())
    }
    pub fn ChatEditMessage(up: &UserParam, param: pb::ChatEditMessageParam) -> Result<pb::ChatEditMessageResponse, GenErr> {
        Ok(pb::ChatEditMessageResponse::default())
    }
    pub fn ChatDeleteMessages(up: &UserParam, param: pb::ChatDeleteMessagesParam) -> Result<pb::ChatDeleteMessagesResponse, GenErr> {
        Ok(pb::ChatDeleteMessagesResponse::default())
    }
    pub fn ChatDeleteHistory(up: &UserParam, param: pb::ChatDeleteHistoryParam) -> Result<pb::ChatDeleteHistoryResponse, GenErr> {
        Ok(pb::ChatDeleteHistoryResponse::default())
    }
    pub fn ChatSendDoingAction(up: &UserParam, param: pb::ChatSendDoingActionParam) -> Result<pb::ChatSendDoingActionResponse, GenErr> {
        Ok(pb::ChatSendDoingActionResponse::default())
    }
    pub fn ChatReportChat(up: &UserParam, param: pb::ChatReportChatParam) -> Result<pb::ChatReportChatResponse, GenErr> {
        Ok(pb::ChatReportChatResponse::default())
    }
    pub fn ChatGetFull(up: &UserParam, param: pb::ChatGetFullMessageParam) -> Result<pb::ChatGetFullMessageResponse, GenErr> {
        Ok(pb::ChatGetFullMessageResponse::default())
    }
    pub fn ChatGetMessagesList(up: &UserParam, param: pb::ChatGetMessagesListParam) -> Result<pb::ChatGetMessagesListResponse, GenErr> {
        Ok(pb::ChatGetMessagesListResponse::default())
    }
    pub fn ChatGetMediaList(up: &UserParam, param: pb::ChatGetMediaListParam) -> Result<pb::ChatGetMediaListResponse, GenErr> {
        Ok(pb::ChatGetMediaListResponse::default())
    }
    
    // service: RPC_Direct
    pub fn DirectDeleteDirect(up: &UserParam, param: pb::DirectDeleteDirectParam) -> Result<pb::DirectDeleteDirectResponse, GenErr> {
        Ok(pb::DirectDeleteDirectResponse::default())
    }
    pub fn DirectChangeTitle(up: &UserParam, param: pb::DirectChangeTitleParam) -> Result<pb::DirectChangeTitleResponse, GenErr> {
        Ok(pb::DirectChangeTitleResponse::default())
    }
    pub fn DirectSetCustomNotification(up: &UserParam, param: pb::DirectSetCustomNotificationParam) -> Result<pb::DirectSetCustomNotificationResponse, GenErr> {
        Ok(pb::DirectSetCustomNotificationResponse::default())
    }
    pub fn DirectSendActionDoing(up: &UserParam, param: pb::DirectSendActionDoingParam) -> Result<pb::DirectSendActionDoingResponse, GenErr> {
        Ok(pb::DirectSendActionDoingResponse::default())
    }
    pub fn DirectSetDraft(up: &UserParam, param: pb::DirectSetDraftParam) -> Result<pb::DirectSetDraftResponse, GenErr> {
        Ok(pb::DirectSetDraftResponse::default())
    }
    pub fn DirectDeleteDirects(up: &UserParam, param: pb::DirectDeleteDirectsParam) -> Result<pb::DirectDeleteDirectsResponse, GenErr> {
        Ok(pb::DirectDeleteDirectsResponse::default())
    }
    pub fn DirectMarkAsRead(up: &UserParam, param: pb::DirectMarkAsReadParam) -> Result<pb::DirectMarkAsReadResponse, GenErr> {
        Ok(pb::DirectMarkAsReadResponse::default())
    }
    pub fn DirectMarkAsUnRead(up: &UserParam, param: pb::DirectMarkAsUnReadParam) -> Result<pb::DirectMarkAsUnReadResponse, GenErr> {
        Ok(pb::DirectMarkAsUnReadResponse::default())
    }
    pub fn DirectPinDirects(up: &UserParam, param: pb::DirectPinDirectsParam) -> Result<pb::DirectPinDirectsResponse, GenErr> {
        Ok(pb::DirectPinDirectsResponse::default())
    }
    pub fn DirectUnPinDirects(up: &UserParam, param: pb::DirectUnPinDirectsParam) -> Result<pb::DirectUnPinDirectsResponse, GenErr> {
        Ok(pb::DirectUnPinDirectsResponse::default())
    }
    pub fn DirectArchiveDirects(up: &UserParam, param: pb::DirectArchiveDirectsParam) -> Result<pb::DirectArchiveDirectsResponse, GenErr> {
        Ok(pb::DirectArchiveDirectsResponse::default())
    }
    pub fn DirectUnArchiveDirects(up: &UserParam, param: pb::DirectUnArchiveDirectsParam) -> Result<pb::DirectUnArchiveDirectsResponse, GenErr> {
        Ok(pb::DirectUnArchiveDirectsResponse::default())
    }
    pub fn DirectClearHistories(up: &UserParam, param: pb::DirectClearHistoriesParam) -> Result<pb::DirectClearHistoriesResponse, GenErr> {
        Ok(pb::DirectClearHistoriesResponse::default())
    }
    pub fn DirectMuteDirects(up: &UserParam, param: pb::DirectMuteDirectsParam) -> Result<pb::DirectMuteDirectsResponse, GenErr> {
        Ok(pb::DirectMuteDirectsResponse::default())
    }
    pub fn DirectUnMuteDirects(up: &UserParam, param: pb::DirectUnMuteDirectsParam) -> Result<pb::DirectUnMuteDirectsResponse, GenErr> {
        Ok(pb::DirectUnMuteDirectsResponse::default())
    }
    pub fn DirectCreateFolder(up: &UserParam, param: pb::DirectCreateFolderParam) -> Result<pb::DirectCreateFolderResponse, GenErr> {
        Ok(pb::DirectCreateFolderResponse::default())
    }
    pub fn DirectChangeFolder(up: &UserParam, param: pb::DirectChangeFolderParam) -> Result<pb::DirectChangeFolderResponse, GenErr> {
        Ok(pb::DirectChangeFolderResponse::default())
    }
    pub fn DirectRemoveFromFolder(up: &UserParam, param: pb::DirectRemoveFromFolderParam) -> Result<pb::DirectRemoveFromFolderResponse, GenErr> {
        Ok(pb::DirectRemoveFromFolderResponse::default())
    }
    pub fn DirectReordersFolder(up: &UserParam, param: pb::DirectReordersFolderParam) -> Result<pb::DirectReordersFolderResponse, GenErr> {
        Ok(pb::DirectReordersFolderResponse::default())
    }
    pub fn DirectDeleteFolder(up: &UserParam, param: pb::DirectDeleteFolderParam) -> Result<pb::DirectDeleteFolderResponse, GenErr> {
        Ok(pb::DirectDeleteFolderResponse::default())
    }
    pub fn DirectGetChatsList(up: &UserParam, param: pb::DirectGetChatsListParam) -> Result<pb::DirectGetChatsListResponse, GenErr> {
        Ok(pb::DirectGetChatsListResponse::default())
    }
    pub fn DirectGetGroupsList(up: &UserParam, param: pb::DirectGetGroupsListParam) -> Result<pb::DirectGetGroupsListResponse, GenErr> {
        Ok(pb::DirectGetGroupsListResponse::default())
    }
    pub fn DirectGetChannelsList(up: &UserParam, param: pb::DirectGetChannelsListParam) -> Result<pb::DirectGetChannelsListResponse, GenErr> {
        Ok(pb::DirectGetChannelsListResponse::default())
    }
    pub fn DirectGetFoldersList(up: &UserParam, param: pb::DirectGetFoldersListParam) -> Result<pb::DirectGetFoldersListResponse, GenErr> {
        Ok(pb::DirectGetFoldersListResponse::default())
    }
    pub fn DirectGetFoldersFullList(up: &UserParam, param: pb::DirectGetFoldersFullListParam) -> Result<pb::DirectGetFoldersFullListResponse, GenErr> {
        Ok(pb::DirectGetFoldersFullListResponse::default())
    }
    
    // service: RPC_General
    pub fn Echo(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
    pub fn CheckUserName(up: &UserParam, param: pb::CheckUserNameParam) -> Result<pb::CheckUserNameResponse, GenErr> {
        Ok(pb::CheckUserNameResponse::default())
    }
    
    // service: RPC_Group
    pub fn GroupCreateGroup(up: &UserParam, param: pb::GroupCreateGroupParam) -> Result<pb::GroupCreateGroupResponse, GenErr> {
        Ok(pb::GroupCreateGroupResponse::default())
    }
    pub fn GroupEditGroup(up: &UserParam, param: pb::GroupEditGroupParam) -> Result<pb::GroupEditGroupResponse, GenErr> {
        Ok(pb::GroupEditGroupResponse::default())
    }
    pub fn GroupDeleteGroup(up: &UserParam, param: pb::GroupDeleteGroupParam) -> Result<pb::GroupDeleteGroupResponse, GenErr> {
        Ok(pb::GroupDeleteGroupResponse::default())
    }
    pub fn GroupAddAdmin(up: &UserParam, param: pb::GroupAddAdminParam) -> Result<pb::GroupAddAdminResponse, GenErr> {
        Ok(pb::GroupAddAdminResponse::default())
    }
    pub fn GroupAddMember(up: &UserParam, param: pb::GroupAddMemberParam) -> Result<pb::GroupAddMemberResponse, GenErr> {
        Ok(pb::GroupAddMemberResponse::default())
    }
    pub fn GroupRemoveMember(up: &UserParam, param: pb::GroupRemoveMemberParam) -> Result<pb::GroupRemoveMemberResponse, GenErr> {
        Ok(pb::GroupRemoveMemberResponse::default())
    }
    pub fn GroupChangeMemberLevel(up: &UserParam, param: pb::GroupChangeMemberLevelParam) -> Result<pb::GroupChangeMemberLevelResponse, GenErr> {
        Ok(pb::GroupChangeMemberLevelResponse::default())
    }
    pub fn GroupChangeMemberPermission(up: &UserParam, param: pb::GroupChangeMemberPermissionParam) -> Result<pb::GroupChangeMemberPermissionResponse, GenErr> {
        Ok(pb::GroupChangeMemberPermissionResponse::default())
    }
    pub fn GroupJoinGroup(up: &UserParam, param: pb::JoinGroupParam) -> Result<pb::JoinGroupResponse, GenErr> {
        Ok(pb::JoinGroupResponse::default())
    }
    pub fn GroupLeaveGroup(up: &UserParam, param: pb::GroupLeaveGroupParam) -> Result<pb::GroupLeaveGroupResponse, GenErr> {
        Ok(pb::GroupLeaveGroupResponse::default())
    }
    pub fn GroupBanMember(up: &UserParam, param: pb::GroupBanMemberParam) -> Result<pb::GroupBanMemberResponse, GenErr> {
        Ok(pb::GroupBanMemberResponse::default())
    }
    pub fn GroupChangePrivacy(up: &UserParam, param: pb::GroupChangePrivacyParam) -> Result<pb::GroupChangePrivacyResponse, GenErr> {
        Ok(pb::GroupChangePrivacyResponse::default())
    }
    pub fn GroupChangeDefaultPermission(up: &UserParam, param: pb::GroupChangeDefaultPermissionParam) -> Result<pb::GroupChangeDefaultPermissionResponse, GenErr> {
        Ok(pb::GroupChangeDefaultPermissionResponse::default())
    }
    pub fn GroupRevokeLink(up: &UserParam, param: pb::GroupRevokeLinkParam) -> Result<pb::GroupRevokeLinkResponse, GenErr> {
        Ok(pb::GroupRevokeLinkResponse::default())
    }
    pub fn GroupChangeUsername(up: &UserParam, param: pb::GroupChangeUsernameParam) -> Result<pb::GroupChangeUsernameResponse, GenErr> {
        Ok(pb::GroupChangeUsernameResponse::default())
    }
    pub fn GroupSendMessage(up: &UserParam, param: pb::GroupSendMessageParam) -> Result<pb::GroupSendMessageResponse, GenErr> {
        Ok(pb::GroupSendMessageResponse::default())
    }
    pub fn GroupEditMessage(up: &UserParam, param: pb::GroupEditMessageParam) -> Result<pb::GroupEditMessageResponse, GenErr> {
        Ok(pb::GroupEditMessageResponse::default())
    }
    pub fn GroupPinMessage(up: &UserParam, param: pb::GroupPinMessageParam) -> Result<pb::GroupPinMessageResponse, GenErr> {
        Ok(pb::GroupPinMessageResponse::default())
    }
    pub fn GroupUnPinMessage(up: &UserParam, param: pb::GroupUnPinMessageParam) -> Result<pb::GroupUnPinMessageResponse, GenErr> {
        Ok(pb::GroupUnPinMessageResponse::default())
    }
    pub fn GroupDeleteMessage(up: &UserParam, param: pb::GroupDeleteMessageParam) -> Result<pb::GroupDeleteMessageResponse, GenErr> {
        Ok(pb::GroupDeleteMessageResponse::default())
    }
    pub fn GroupDeleteMessages(up: &UserParam, param: pb::GroupDeleteMessagesParam) -> Result<pb::GroupDeleteMessagesResponse, GenErr> {
        Ok(pb::GroupDeleteMessagesResponse::default())
    }
    pub fn GroupDeleteHistory(up: &UserParam, param: pb::GroupDeleteHistoryParam) -> Result<pb::GroupDeleteHistoryResponse, GenErr> {
        Ok(pb::GroupDeleteHistoryResponse::default())
    }
    pub fn GroupClearHistory(up: &UserParam, param: pb::GroupClearHistoryParam) -> Result<pb::GroupClearHistoryResponse, GenErr> {
        Ok(pb::GroupClearHistoryResponse::default())
    }
    pub fn GroupAvatarAdd(up: &UserParam, param: pb::GroupAvatarAddParam) -> Result<pb::GroupAvatarAddResponse, GenErr> {
        Ok(pb::GroupAvatarAddResponse::default())
    }
    pub fn GroupAvatarChange(up: &UserParam, param: pb::GroupAvatarChangeParam) -> Result<pb::GroupAvatarChangeResponse, GenErr> {
        Ok(pb::GroupAvatarChangeResponse::default())
    }
    pub fn GroupAvatarDelete(up: &UserParam, param: pb::GroupAvatarDeleteParam) -> Result<pb::GroupAvatarDeleteResponse, GenErr> {
        Ok(pb::GroupAvatarDeleteResponse::default())
    }
    pub fn GroupAvatarGetList(up: &UserParam, param: pb::GroupAvatarGetListParam) -> Result<pb::GroupAvatarGetListResponse, GenErr> {
        Ok(pb::GroupAvatarGetListResponse::default())
    }
    pub fn GroupSendDoingAction(up: &UserParam, param: pb::GroupSendDoingActionParam) -> Result<pb::GroupSendDoingActionResponse, GenErr> {
        Ok(pb::GroupSendDoingActionResponse::default())
    }
    pub fn GroupReportGroup(up: &UserParam, param: pb::GroupReportGroupParam) -> Result<pb::GroupReportGroupResponse, GenErr> {
        Ok(pb::GroupReportGroupResponse::default())
    }
    pub fn GroupGetFull(up: &UserParam, param: pb::GroupGetFullMessageParam) -> Result<pb::GroupGetFullMessageResponse, GenErr> {
        Ok(pb::GroupGetFullMessageResponse::default())
    }
    pub fn GroupGetMessagesList(up: &UserParam, param: pb::GroupGetMessagesListParam) -> Result<pb::GroupGetMessagesListResponse, GenErr> {
        Ok(pb::GroupGetMessagesListResponse::default())
    }
    pub fn GroupGetMediaList(up: &UserParam, param: pb::GroupGetMediaListParam) -> Result<pb::GroupGetMediaListResponse, GenErr> {
        Ok(pb::GroupGetMediaListResponse::default())
    }
    pub fn GroupGetMembersList(up: &UserParam, param: pb::GroupGetMembersListParam) -> Result<pb::GroupGetMembersListResponse, GenErr> {
        Ok(pb::GroupGetMembersListResponse::default())
    }
    pub fn GroupGetAdminsList(up: &UserParam, param: pb::GroupGetAdminsListParam) -> Result<pb::GroupGetAdminsListResponse, GenErr> {
        Ok(pb::GroupGetAdminsListResponse::default())
    }
    pub fn GroupSetDraft(up: &UserParam, param: pb::GroupSetDraftParam) -> Result<pb::GroupSetDraftResponse, GenErr> {
        Ok(pb::GroupSetDraftResponse::default())
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
