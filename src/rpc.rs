use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::body;
use serde::{Deserialize, Serialize};
use quick_protobuf::{BytesReader, BytesWriter};
use quick_protobuf::{MessageRead,MessageWrite,Writer,deserialize_from_slice};

use crate::{pb,com,com::*, rpc_fns};

pub mod method_ids {
    // Service: RPC_Auth
    pub const SendConfirmCode: u32 = 99432981;
    pub const ConfirmCode: u32 = 1139849846;
    pub const SingUp: u32 = 344412789;
    pub const SingIn: u32 = 5700586;
    pub const LogOut: u32 = 548012719;
    
    // Service: RPC_Channel
    pub const ChannelCreateChannel: u32 = 740341541;
    pub const ChannelEditChannel: u32 = 329231200;
    pub const ChannelDeleteChannel: u32 = 177944127;
    pub const ChannelAddAuthor: u32 = 1711825211;
    pub const ChannelChangeAuthorPermission: u32 = 151376166;
    pub const ChannelRemoveAuthor: u32 = 738902973;
    pub const ChannelFollowChannel: u32 = 1627479123;
    pub const ChannelUnFollowChannel: u32 = 1037521736;
    pub const ChannelRemoveFollowers: u32 = 308740721;
    pub const ChannelSubscribe: u32 = 757900782;
    pub const ChannelUnSubscribe: u32 = 358413629;
    pub const ChannelRemoveSubscribers: u32 = 505495602;
    pub const ChannelChangePrivacy: u32 = 1733100870;
    pub const ChannelChangeDefaultPermission: u32 = 2017309544;
    pub const ChannelRevokeLink: u32 = 996833555;
    pub const ChannelChangeUsername: u32 = 1670162010;
    pub const ChannelBlockChannel: u32 = 712464279;
    pub const ChannelSendMessage: u32 = 1957690884;
    pub const ChannelEditMessage: u32 = 2096144443;
    pub const ChannelPinMessage: u32 = 1642528121;
    pub const ChannelUnPinMessage: u32 = 2083192996;
    pub const ChannelDeleteMessage: u32 = 1167266448;
    pub const ChannelDeleteMessages: u32 = 789593552;
    pub const ChannelClearHistory: u32 = 1169273718;
    pub const ChannelAvatarAdd: u32 = 1289673888;
    pub const ChannelAvatarChange: u32 = 886893206;
    pub const ChannelAvatarDelete: u32 = 1452867901;
    pub const ChannelAvatarGetList: u32 = 988947981;
    pub const ChannelSendDoingAction: u32 = 1633866869;
    pub const ChannelReportChannel: u32 = 1764489847;
    pub const ChannelReportMessage: u32 = 241326631;
    pub const ChannelGetFull: u32 = 259375337;
    pub const ChannelGetMessagesList: u32 = 427631986;
    pub const ChannelGetMediaList: u32 = 250698167;
    pub const ChannelGetAuthors: u32 = 1778309222;
    pub const ChannelGetFollowers: u32 = 1196343214;
    pub const ChannelGetFollowings: u32 = 35449020;
    pub const ChannelGetSubscribers: u32 = 1954004522;
    pub const ChannelBlocked: u32 = 1670519893;
    pub const ChannelSetDraft: u32 = 1563299078;
    
    // Service: RPC_Chat
    pub const ChatSendMessage: u32 = 872296057;
    pub const ChatEditMessage: u32 = 548752544;
    pub const ChatDeleteMessages: u32 = 475175726;
    pub const ChatDeleteHistory: u32 = 2090393303;
    pub const ChatSendDoingAction: u32 = 1654010294;
    pub const ChatReportChat: u32 = 1433006017;
    pub const ChatGetFull: u32 = 1399898814;
    pub const ChatGetMessagesList: u32 = 2031841962;
    pub const ChatGetMediaList: u32 = 162122201;
    
    // Service: RPC_Direct
    pub const DirectDeleteDirect: u32 = 95323214;
    pub const DirectChangeTitle: u32 = 250554551;
    pub const DirectSetCustomNotification: u32 = 458248252;
    pub const DirectSendActionDoing: u32 = 658009933;
    pub const DirectSetDraft: u32 = 712749707;
    pub const DirectDeleteDirects: u32 = 1197087360;
    pub const DirectMarkAsRead: u32 = 1429996148;
    pub const DirectMarkAsUnRead: u32 = 233319325;
    pub const DirectPinDirects: u32 = 1375511294;
    pub const DirectUnPinDirects: u32 = 293925235;
    pub const DirectArchiveDirects: u32 = 1416265881;
    pub const DirectUnArchiveDirects: u32 = 1615356451;
    pub const DirectClearHistories: u32 = 1518839919;
    pub const DirectMuteDirects: u32 = 179035912;
    pub const DirectUnMuteDirects: u32 = 719725946;
    pub const DirectCreateFolder: u32 = 2001408305;
    pub const DirectChangeFolder: u32 = 1814658660;
    pub const DirectRemoveFromFolder: u32 = 151098714;
    pub const DirectReordersFolder: u32 = 1957624017;
    pub const DirectDeleteFolder: u32 = 977344730;
    pub const DirectGetChatsList: u32 = 1003954339;
    pub const DirectGetGroupsList: u32 = 1110873556;
    pub const DirectGetChannelsList: u32 = 427433511;
    pub const DirectGetFoldersList: u32 = 1067480006;
    pub const DirectGetFoldersFullList: u32 = 1657882950;
    
    // Service: RPC_Group
    pub const GroupCreateGroup: u32 = 1140942978;
    pub const GroupEditGroup: u32 = 1880514193;
    pub const GroupDeleteGroup: u32 = 1459388602;
    pub const GroupAddAdmin: u32 = 2134185330;
    pub const GroupAddMember: u32 = 606751933;
    pub const GroupRemoveMember: u32 = 1837375022;
    pub const GroupChangeMemberLevel: u32 = 1539340173;
    pub const GroupChangeMemberPermission: u32 = 483512268;
    pub const GroupJoinGroup: u32 = 655613339;
    pub const GroupLeaveGroup: u32 = 579241895;
    pub const GroupBanMember: u32 = 1045673431;
    pub const GroupChangePrivacy: u32 = 600263304;
    pub const GroupChangeDefaultPermission: u32 = 954689199;
    pub const GroupRevokeLink: u32 = 1764467684;
    pub const GroupChangeUsername: u32 = 1191090261;
    pub const GroupSendMessage: u32 = 684584436;
    pub const GroupEditMessage: u32 = 1787622934;
    pub const GroupPinMessage: u32 = 1238271536;
    pub const GroupUnPinMessage: u32 = 631852378;
    pub const GroupDeleteMessage: u32 = 1194346075;
    pub const GroupDeleteMessages: u32 = 1584921307;
    pub const GroupDeleteHistory: u32 = 953107081;
    pub const GroupClearHistory: u32 = 1430468467;
    pub const GroupAvatarAdd: u32 = 1167941254;
    pub const GroupAvatarChange: u32 = 406803098;
    pub const GroupAvatarDelete: u32 = 854193498;
    pub const GroupAvatarGetList: u32 = 1102046093;
    pub const GroupSendDoingAction: u32 = 1686585448;
    pub const GroupReportGroup: u32 = 220798382;
    pub const GroupGetFull: u32 = 499140930;
    pub const GroupGetMessagesList: u32 = 1617930178;
    pub const GroupGetMediaList: u32 = 1490669886;
    pub const GroupGetMembersList: u32 = 502288128;
    pub const GroupGetAdminsList: u32 = 1772981789;
    pub const GroupSetDraft: u32 = 411748258;
    
    // Service: RPC_Sample
    pub const GetUsers1: u32 = 920502617;
    pub const GetProfiles: u32 = 770967729;
    pub const GetChannels: u32 = 88100100;
    pub const GetDirects: u32 = 393387904;
    pub const GetMessages: u32 = 1834253004;
    
    // Service: RPC_Shared
    pub const Echo: u32 = 1239211125;
    pub const CheckUserName: u32 = 1759322581;
    
    // Service: RPC_Upload
    pub const UploadFile: u32 = 422068969;
    
    // Service: RPC_User
    pub const ChangePhoneNumber: u32 = 605934586;
    
    pub const ChangePhoneNumber8 : u32 = 79874;
}

pub async fn server_rpc(act : pb::Invoke) -> Result<Vec<u8>,GenErr> {
    let up = UserParam{};

    match act.method {
    
    // service: RPC_Auth
        method_ids::SendConfirmCode => { // 99432981
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::SendConfirmCodeParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::SendConfirmCode(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ConfirmCode => { // 1139849846
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ConfirmCodeParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ConfirmCode(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::SingUp => { // 344412789
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::SingUpParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::SingUp(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::SingIn => { // 5700586
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::SingInParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::SingIn(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::LogOut => { // 548012719
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::LogOutParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::LogOut(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Channel
        method_ids::ChannelCreateChannel => { // 740341541
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelCreateChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelCreateChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelEditChannel => { // 329231200
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelEditChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelEditChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelDeleteChannel => { // 177944127
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelDeleteChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelDeleteChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelAddAuthor => { // 1711825211
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelAddAuthorParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelAddAuthor(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelChangeAuthorPermission => { // 151376166
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelChangeAuthorPermissionParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelChangeAuthorPermission(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelRemoveAuthor => { // 738902973
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelRemoveAuthorParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelRemoveAuthor(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelFollowChannel => { // 1627479123
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelFollowChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelFollowChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelUnFollowChannel => { // 1037521736
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelUnFollowChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelUnFollowChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelRemoveFollowers => { // 308740721
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelRemoveFollowersParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelRemoveFollowers(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelSubscribe => { // 757900782
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelSubscribeParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelSubscribe(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelUnSubscribe => { // 358413629
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelUnSubscribeParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelUnSubscribe(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelRemoveSubscribers => { // 505495602
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelRemoveSubscribersParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelRemoveSubscribers(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelChangePrivacy => { // 1733100870
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelChangePrivacyParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelChangePrivacy(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelChangeDefaultPermission => { // 2017309544
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelChangeDefaultPermissionParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelChangeDefaultPermission(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelRevokeLink => { // 996833555
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelRevokeLinkParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelRevokeLink(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelChangeUsername => { // 1670162010
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelChangeUsernameParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelChangeUsername(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelBlockChannel => { // 712464279
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelBlockChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelBlockChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelSendMessage => { // 1957690884
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelSendMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelSendMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelEditMessage => { // 2096144443
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelEditMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelEditMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelPinMessage => { // 1642528121
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelPinMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelPinMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelUnPinMessage => { // 2083192996
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelUnPinMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelUnPinMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelDeleteMessage => { // 1167266448
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelDeleteMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelDeleteMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelDeleteMessages => { // 789593552
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelDeleteMessagesParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelDeleteMessages(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelClearHistory => { // 1169273718
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelClearHistoryParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelClearHistory(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelAvatarAdd => { // 1289673888
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelAvatarAddParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelAvatarAdd(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelAvatarChange => { // 886893206
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelAvatarChangeParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelAvatarChange(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelAvatarDelete => { // 1452867901
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelAvatarDeleteParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelAvatarDelete(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelAvatarGetList => { // 988947981
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelAvatarGetListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelAvatarGetList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelSendDoingAction => { // 1633866869
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelSendDoingActionParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelSendDoingAction(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelReportChannel => { // 1764489847
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelReportChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelReportChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelReportMessage => { // 241326631
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelReportMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelReportMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelGetFull => { // 259375337
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelGetFullParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelGetFull(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelGetMessagesList => { // 427631986
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelGetMessagesListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelGetMessagesList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelGetMediaList => { // 250698167
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelGetMediaListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelGetMediaList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelGetAuthors => { // 1778309222
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelGetAuthorsParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelGetAuthors(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelGetFollowers => { // 1196343214
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelGetFollowersParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelGetFollowers(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelGetFollowings => { // 35449020
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelGetFollowingsParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelGetFollowings(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelGetSubscribers => { // 1954004522
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelGetSubscribersParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelGetSubscribers(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelBlocked => { // 1670519893
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelBlockedParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelBlocked(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChannelSetDraft => { // 1563299078
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChannelSetDraftParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChannelSetDraft(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Chat
        method_ids::ChatSendMessage => { // 872296057
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChatSendMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChatSendMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChatEditMessage => { // 548752544
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChatEditMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChatEditMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChatDeleteMessages => { // 475175726
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChatDeleteMessagesParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChatDeleteMessages(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChatDeleteHistory => { // 2090393303
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChatDeleteHistoryParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChatDeleteHistory(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChatSendDoingAction => { // 1654010294
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChatSendDoingActionParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChatSendDoingAction(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChatReportChat => { // 1433006017
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChatReportChatParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChatReportChat(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChatGetFull => { // 1399898814
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChatGetFullMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChatGetFull(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChatGetMessagesList => { // 2031841962
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChatGetMessagesListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChatGetMessagesList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::ChatGetMediaList => { // 162122201
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChatGetMediaListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChatGetMediaList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Direct
        method_ids::DirectDeleteDirect => { // 95323214
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectDeleteDirectParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectDeleteDirect(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectChangeTitle => { // 250554551
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectChangeTitleParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectChangeTitle(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectSetCustomNotification => { // 458248252
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectSetCustomNotificationParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectSetCustomNotification(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectSendActionDoing => { // 658009933
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectSendActionDoingParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectSendActionDoing(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectSetDraft => { // 712749707
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectSetDraftParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectSetDraft(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectDeleteDirects => { // 1197087360
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectDeleteDirectsParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectDeleteDirects(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectMarkAsRead => { // 1429996148
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectMarkAsReadParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectMarkAsRead(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectMarkAsUnRead => { // 233319325
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectMarkAsUnReadParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectMarkAsUnRead(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectPinDirects => { // 1375511294
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectPinDirectsParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectPinDirects(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectUnPinDirects => { // 293925235
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectUnPinDirectsParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectUnPinDirects(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectArchiveDirects => { // 1416265881
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectArchiveDirectsParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectArchiveDirects(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectUnArchiveDirects => { // 1615356451
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectUnArchiveDirectsParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectUnArchiveDirects(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectClearHistories => { // 1518839919
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectClearHistoriesParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectClearHistories(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectMuteDirects => { // 179035912
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectMuteDirectsParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectMuteDirects(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectUnMuteDirects => { // 719725946
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectUnMuteDirectsParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectUnMuteDirects(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectCreateFolder => { // 2001408305
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectCreateFolderParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectCreateFolder(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectChangeFolder => { // 1814658660
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectChangeFolderParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectChangeFolder(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectRemoveFromFolder => { // 151098714
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectRemoveFromFolderParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectRemoveFromFolder(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectReordersFolder => { // 1957624017
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectReordersFolderParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectReordersFolder(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectDeleteFolder => { // 977344730
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectDeleteFolderParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectDeleteFolder(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectGetChatsList => { // 1003954339
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectGetChatsListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectGetChatsList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectGetGroupsList => { // 1110873556
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectGetGroupsListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectGetGroupsList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectGetChannelsList => { // 427433511
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectGetChannelsListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectGetChannelsList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectGetFoldersList => { // 1067480006
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectGetFoldersListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectGetFoldersList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DirectGetFoldersFullList => { // 1657882950
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DirectGetFoldersFullListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DirectGetFoldersFullList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Group
        method_ids::GroupCreateGroup => { // 1140942978
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupCreateGroupParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupCreateGroup(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupEditGroup => { // 1880514193
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupEditGroupParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupEditGroup(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupDeleteGroup => { // 1459388602
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupDeleteGroupParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupDeleteGroup(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupAddAdmin => { // 2134185330
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupAddAdminParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupAddAdmin(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupAddMember => { // 606751933
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupAddMemberParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupAddMember(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupRemoveMember => { // 1837375022
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupRemoveMemberParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupRemoveMember(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupChangeMemberLevel => { // 1539340173
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupChangeMemberLevelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupChangeMemberLevel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupChangeMemberPermission => { // 483512268
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupChangeMemberPermissionParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupChangeMemberPermission(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupJoinGroup => { // 655613339
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::JoinGroupParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupJoinGroup(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupLeaveGroup => { // 579241895
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupLeaveGroupParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupLeaveGroup(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupBanMember => { // 1045673431
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupBanMemberParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupBanMember(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupChangePrivacy => { // 600263304
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupChangePrivacyParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupChangePrivacy(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupChangeDefaultPermission => { // 954689199
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupChangeDefaultPermissionParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupChangeDefaultPermission(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupRevokeLink => { // 1764467684
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupRevokeLinkParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupRevokeLink(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupChangeUsername => { // 1191090261
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupChangeUsernameParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupChangeUsername(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupSendMessage => { // 684584436
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupSendMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupSendMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupEditMessage => { // 1787622934
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupEditMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupEditMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupPinMessage => { // 1238271536
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupPinMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupPinMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupUnPinMessage => { // 631852378
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupUnPinMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupUnPinMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupDeleteMessage => { // 1194346075
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupDeleteMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupDeleteMessage(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupDeleteMessages => { // 1584921307
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupDeleteMessagesParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupDeleteMessages(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupDeleteHistory => { // 953107081
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupDeleteHistoryParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupDeleteHistory(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupClearHistory => { // 1430468467
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupClearHistoryParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupClearHistory(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupAvatarAdd => { // 1167941254
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupAvatarAddParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupAvatarAdd(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupAvatarChange => { // 406803098
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupAvatarChangeParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupAvatarChange(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupAvatarDelete => { // 854193498
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupAvatarDeleteParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupAvatarDelete(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupAvatarGetList => { // 1102046093
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupAvatarGetListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupAvatarGetList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupSendDoingAction => { // 1686585448
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupSendDoingActionParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupSendDoingAction(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupReportGroup => { // 220798382
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupReportGroupParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupReportGroup(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupGetFull => { // 499140930
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupGetFullMessageParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupGetFull(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupGetMessagesList => { // 1617930178
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupGetMessagesListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupGetMessagesList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupGetMediaList => { // 1490669886
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupGetMediaListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupGetMediaList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupGetMembersList => { // 502288128
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupGetMembersListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupGetMembersList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupGetAdminsList => { // 1772981789
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupGetAdminsListParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupGetAdminsList(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GroupSetDraft => { // 411748258
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GroupSetDraftParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GroupSetDraft(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Sample
        method_ids::GetUsers1 => { // 920502617
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GetUsers1Param, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GetUsers1(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GetProfiles => { // 770967729
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GetProfilesParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GetProfiles(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GetChannels => { // 88100100
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GetChannelsParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GetChannels(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GetDirects => { // 393387904
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GetDirectsParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GetDirects(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::GetMessages => { // 1834253004
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::GetMessagesParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::GetMessages(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Shared
        method_ids::Echo => { // 1239211125
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::EchoParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::Echo(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::CheckUserName => { // 1759322581
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::CheckUserNameParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::CheckUserName(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Upload
        method_ids::UploadFile => { // 422068969
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::UploadFileParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::UploadFile(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_User
        method_ids::ChangePhoneNumber => { // 605934586
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::ChangePhoneNumberParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::ChangePhoneNumber(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
        _ => {
            Err(GenErr::NoRpcMatch)
        }
    }
}

pub struct RpcClient {
    endpoint: &'static str,
}

impl RpcClient {
    pub fn new(endpoint: &'static str) -> Self {
        RpcClient{
            endpoint: endpoint,
        }
    }

    fn get_next_action_id(&self) -> u64 {
        8
    }

// service: RPC_Auth
    pub async fn SendConfirmCode (&self, param: pb::SendConfirmCodeParam) -> Result<pb::SendConfirmCodeResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::SendConfirmCode,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ConfirmCode (&self, param: pb::ConfirmCodeParam) -> Result<pb::ConfirmCodeResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ConfirmCode,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn SingUp (&self, param: pb::SingUpParam) -> Result<pb::SingUpResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::SingUp,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn SingIn (&self, param: pb::SingInParam) -> Result<pb::SingInResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::SingIn,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn LogOut (&self, param: pb::LogOutParam) -> Result<pb::LogOutResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::LogOut,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
// service: RPC_Channel
    pub async fn ChannelCreateChannel (&self, param: pb::ChannelCreateChannelParam) -> Result<pb::ChannelCreateChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelCreateChannel,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelEditChannel (&self, param: pb::ChannelEditChannelParam) -> Result<pb::ChannelEditChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelEditChannel,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelDeleteChannel (&self, param: pb::ChannelDeleteChannelParam) -> Result<pb::ChannelDeleteChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelDeleteChannel,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelAddAuthor (&self, param: pb::ChannelAddAuthorParam) -> Result<pb::ChannelAddAuthorResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelAddAuthor,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelChangeAuthorPermission (&self, param: pb::ChannelChangeAuthorPermissionParam) -> Result<pb::ChannelChangeAuthorPermissionResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelChangeAuthorPermission,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelRemoveAuthor (&self, param: pb::ChannelRemoveAuthorParam) -> Result<pb::ChannelRemoveAuthorResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelRemoveAuthor,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelFollowChannel (&self, param: pb::ChannelFollowChannelParam) -> Result<pb::ChannelFollowChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelFollowChannel,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelUnFollowChannel (&self, param: pb::ChannelUnFollowChannelParam) -> Result<pb::ChannelUnFollowChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelUnFollowChannel,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelRemoveFollowers (&self, param: pb::ChannelRemoveFollowersParam) -> Result<pb::ChannelRemoveFollowersResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelRemoveFollowers,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelSubscribe (&self, param: pb::ChannelSubscribeParam) -> Result<pb::ChannelSubscribeResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelSubscribe,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelUnSubscribe (&self, param: pb::ChannelUnSubscribeParam) -> Result<pb::ChannelUnSubscribeResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelUnSubscribe,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelRemoveSubscribers (&self, param: pb::ChannelRemoveSubscribersParam) -> Result<pb::ChannelRemoveSubscribersResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelRemoveSubscribers,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelChangePrivacy (&self, param: pb::ChannelChangePrivacyParam) -> Result<pb::ChannelChangePrivacyResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelChangePrivacy,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelChangeDefaultPermission (&self, param: pb::ChannelChangeDefaultPermissionParam) -> Result<pb::ChannelChangeDefaultPermissionResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelChangeDefaultPermission,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelRevokeLink (&self, param: pb::ChannelRevokeLinkParam) -> Result<pb::ChannelRevokeLinkResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelRevokeLink,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelChangeUsername (&self, param: pb::ChannelChangeUsernameParam) -> Result<pb::ChannelChangeUsernameResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelChangeUsername,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelBlockChannel (&self, param: pb::ChannelBlockChannelParam) -> Result<pb::ChannelBlockChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelBlockChannel,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelSendMessage (&self, param: pb::ChannelSendMessageParam) -> Result<pb::ChannelSendMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelSendMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelEditMessage (&self, param: pb::ChannelEditMessageParam) -> Result<pb::ChannelEditMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelEditMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelPinMessage (&self, param: pb::ChannelPinMessageParam) -> Result<pb::ChannelPinMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelPinMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelUnPinMessage (&self, param: pb::ChannelUnPinMessageParam) -> Result<pb::ChannelUnPinMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelUnPinMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelDeleteMessage (&self, param: pb::ChannelDeleteMessageParam) -> Result<pb::ChannelDeleteMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelDeleteMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelDeleteMessages (&self, param: pb::ChannelDeleteMessagesParam) -> Result<pb::ChannelDeleteMessagesResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelDeleteMessages,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelClearHistory (&self, param: pb::ChannelClearHistoryParam) -> Result<pb::ChannelClearHistoryResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelClearHistory,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelAvatarAdd (&self, param: pb::ChannelAvatarAddParam) -> Result<pb::ChannelAvatarAddResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelAvatarAdd,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelAvatarChange (&self, param: pb::ChannelAvatarChangeParam) -> Result<pb::ChannelAvatarChangeResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelAvatarChange,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelAvatarDelete (&self, param: pb::ChannelAvatarDeleteParam) -> Result<pb::ChannelAvatarDeleteResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelAvatarDelete,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelAvatarGetList (&self, param: pb::ChannelAvatarGetListParam) -> Result<pb::ChannelAvatarGetListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelAvatarGetList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelSendDoingAction (&self, param: pb::ChannelSendDoingActionParam) -> Result<pb::ChannelSendDoingActionResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelSendDoingAction,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelReportChannel (&self, param: pb::ChannelReportChannelParam) -> Result<pb::ChannelReportChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelReportChannel,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelReportMessage (&self, param: pb::ChannelReportMessageParam) -> Result<pb::ChannelReportMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelReportMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelGetFull (&self, param: pb::ChannelGetFullParam) -> Result<pb::ChannelGetFullResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelGetFull,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelGetMessagesList (&self, param: pb::ChannelGetMessagesListParam) -> Result<pb::ChannelGetMessagesListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelGetMessagesList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelGetMediaList (&self, param: pb::ChannelGetMediaListParam) -> Result<pb::ChannelGetMediaListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelGetMediaList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelGetAuthors (&self, param: pb::ChannelGetAuthorsParam) -> Result<pb::ChannelGetAuthorsResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelGetAuthors,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelGetFollowers (&self, param: pb::ChannelGetFollowersParam) -> Result<pb::ChannelGetFollowersResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelGetFollowers,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelGetFollowings (&self, param: pb::ChannelGetFollowingsParam) -> Result<pb::ChannelGetFollowingsResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelGetFollowings,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelGetSubscribers (&self, param: pb::ChannelGetSubscribersParam) -> Result<pb::ChannelGetSubscribersResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelGetSubscribers,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelBlocked (&self, param: pb::ChannelBlockedParam) -> Result<pb::ChannelBlockedResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelBlocked,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChannelSetDraft (&self, param: pb::ChannelSetDraftParam) -> Result<pb::ChannelSetDraftResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChannelSetDraft,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
// service: RPC_Chat
    pub async fn ChatSendMessage (&self, param: pb::ChatSendMessageParam) -> Result<pb::ChatSendMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChatSendMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChatEditMessage (&self, param: pb::ChatEditMessageParam) -> Result<pb::ChatEditMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChatEditMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChatDeleteMessages (&self, param: pb::ChatDeleteMessagesParam) -> Result<pb::ChatDeleteMessagesResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChatDeleteMessages,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChatDeleteHistory (&self, param: pb::ChatDeleteHistoryParam) -> Result<pb::ChatDeleteHistoryResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChatDeleteHistory,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChatSendDoingAction (&self, param: pb::ChatSendDoingActionParam) -> Result<pb::ChatSendDoingActionResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChatSendDoingAction,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChatReportChat (&self, param: pb::ChatReportChatParam) -> Result<pb::ChatReportChatResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChatReportChat,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChatGetFull (&self, param: pb::ChatGetFullMessageParam) -> Result<pb::ChatGetFullMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChatGetFull,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChatGetMessagesList (&self, param: pb::ChatGetMessagesListParam) -> Result<pb::ChatGetMessagesListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChatGetMessagesList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn ChatGetMediaList (&self, param: pb::ChatGetMediaListParam) -> Result<pb::ChatGetMediaListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChatGetMediaList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
// service: RPC_Direct
    pub async fn DirectDeleteDirect (&self, param: pb::DirectDeleteDirectParam) -> Result<pb::DirectDeleteDirectResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectDeleteDirect,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectChangeTitle (&self, param: pb::DirectChangeTitleParam) -> Result<pb::DirectChangeTitleResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectChangeTitle,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectSetCustomNotification (&self, param: pb::DirectSetCustomNotificationParam) -> Result<pb::DirectSetCustomNotificationResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectSetCustomNotification,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectSendActionDoing (&self, param: pb::DirectSendActionDoingParam) -> Result<pb::DirectSendActionDoingResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectSendActionDoing,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectSetDraft (&self, param: pb::DirectSetDraftParam) -> Result<pb::DirectSetDraftResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectSetDraft,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectDeleteDirects (&self, param: pb::DirectDeleteDirectsParam) -> Result<pb::DirectDeleteDirectsResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectDeleteDirects,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectMarkAsRead (&self, param: pb::DirectMarkAsReadParam) -> Result<pb::DirectMarkAsReadResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectMarkAsRead,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectMarkAsUnRead (&self, param: pb::DirectMarkAsUnReadParam) -> Result<pb::DirectMarkAsUnReadResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectMarkAsUnRead,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectPinDirects (&self, param: pb::DirectPinDirectsParam) -> Result<pb::DirectPinDirectsResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectPinDirects,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectUnPinDirects (&self, param: pb::DirectUnPinDirectsParam) -> Result<pb::DirectUnPinDirectsResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectUnPinDirects,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectArchiveDirects (&self, param: pb::DirectArchiveDirectsParam) -> Result<pb::DirectArchiveDirectsResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectArchiveDirects,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectUnArchiveDirects (&self, param: pb::DirectUnArchiveDirectsParam) -> Result<pb::DirectUnArchiveDirectsResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectUnArchiveDirects,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectClearHistories (&self, param: pb::DirectClearHistoriesParam) -> Result<pb::DirectClearHistoriesResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectClearHistories,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectMuteDirects (&self, param: pb::DirectMuteDirectsParam) -> Result<pb::DirectMuteDirectsResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectMuteDirects,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectUnMuteDirects (&self, param: pb::DirectUnMuteDirectsParam) -> Result<pb::DirectUnMuteDirectsResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectUnMuteDirects,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectCreateFolder (&self, param: pb::DirectCreateFolderParam) -> Result<pb::DirectCreateFolderResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectCreateFolder,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectChangeFolder (&self, param: pb::DirectChangeFolderParam) -> Result<pb::DirectChangeFolderResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectChangeFolder,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectRemoveFromFolder (&self, param: pb::DirectRemoveFromFolderParam) -> Result<pb::DirectRemoveFromFolderResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectRemoveFromFolder,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectReordersFolder (&self, param: pb::DirectReordersFolderParam) -> Result<pb::DirectReordersFolderResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectReordersFolder,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectDeleteFolder (&self, param: pb::DirectDeleteFolderParam) -> Result<pb::DirectDeleteFolderResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectDeleteFolder,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectGetChatsList (&self, param: pb::DirectGetChatsListParam) -> Result<pb::DirectGetChatsListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectGetChatsList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectGetGroupsList (&self, param: pb::DirectGetGroupsListParam) -> Result<pb::DirectGetGroupsListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectGetGroupsList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectGetChannelsList (&self, param: pb::DirectGetChannelsListParam) -> Result<pb::DirectGetChannelsListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectGetChannelsList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectGetFoldersList (&self, param: pb::DirectGetFoldersListParam) -> Result<pb::DirectGetFoldersListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectGetFoldersList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn DirectGetFoldersFullList (&self, param: pb::DirectGetFoldersFullListParam) -> Result<pb::DirectGetFoldersFullListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DirectGetFoldersFullList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
// service: RPC_Group
    pub async fn GroupCreateGroup (&self, param: pb::GroupCreateGroupParam) -> Result<pb::GroupCreateGroupResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupCreateGroup,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupEditGroup (&self, param: pb::GroupEditGroupParam) -> Result<pb::GroupEditGroupResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupEditGroup,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupDeleteGroup (&self, param: pb::GroupDeleteGroupParam) -> Result<pb::GroupDeleteGroupResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupDeleteGroup,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupAddAdmin (&self, param: pb::GroupAddAdminParam) -> Result<pb::GroupAddAdminResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupAddAdmin,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupAddMember (&self, param: pb::GroupAddMemberParam) -> Result<pb::GroupAddMemberResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupAddMember,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupRemoveMember (&self, param: pb::GroupRemoveMemberParam) -> Result<pb::GroupRemoveMemberResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupRemoveMember,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupChangeMemberLevel (&self, param: pb::GroupChangeMemberLevelParam) -> Result<pb::GroupChangeMemberLevelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupChangeMemberLevel,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupChangeMemberPermission (&self, param: pb::GroupChangeMemberPermissionParam) -> Result<pb::GroupChangeMemberPermissionResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupChangeMemberPermission,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupJoinGroup (&self, param: pb::JoinGroupParam) -> Result<pb::JoinGroupResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupJoinGroup,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupLeaveGroup (&self, param: pb::GroupLeaveGroupParam) -> Result<pb::GroupLeaveGroupResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupLeaveGroup,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupBanMember (&self, param: pb::GroupBanMemberParam) -> Result<pb::GroupBanMemberResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupBanMember,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupChangePrivacy (&self, param: pb::GroupChangePrivacyParam) -> Result<pb::GroupChangePrivacyResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupChangePrivacy,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupChangeDefaultPermission (&self, param: pb::GroupChangeDefaultPermissionParam) -> Result<pb::GroupChangeDefaultPermissionResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupChangeDefaultPermission,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupRevokeLink (&self, param: pb::GroupRevokeLinkParam) -> Result<pb::GroupRevokeLinkResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupRevokeLink,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupChangeUsername (&self, param: pb::GroupChangeUsernameParam) -> Result<pb::GroupChangeUsernameResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupChangeUsername,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupSendMessage (&self, param: pb::GroupSendMessageParam) -> Result<pb::GroupSendMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupSendMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupEditMessage (&self, param: pb::GroupEditMessageParam) -> Result<pb::GroupEditMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupEditMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupPinMessage (&self, param: pb::GroupPinMessageParam) -> Result<pb::GroupPinMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupPinMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupUnPinMessage (&self, param: pb::GroupUnPinMessageParam) -> Result<pb::GroupUnPinMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupUnPinMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupDeleteMessage (&self, param: pb::GroupDeleteMessageParam) -> Result<pb::GroupDeleteMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupDeleteMessage,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupDeleteMessages (&self, param: pb::GroupDeleteMessagesParam) -> Result<pb::GroupDeleteMessagesResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupDeleteMessages,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupDeleteHistory (&self, param: pb::GroupDeleteHistoryParam) -> Result<pb::GroupDeleteHistoryResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupDeleteHistory,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupClearHistory (&self, param: pb::GroupClearHistoryParam) -> Result<pb::GroupClearHistoryResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupClearHistory,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupAvatarAdd (&self, param: pb::GroupAvatarAddParam) -> Result<pb::GroupAvatarAddResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupAvatarAdd,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupAvatarChange (&self, param: pb::GroupAvatarChangeParam) -> Result<pb::GroupAvatarChangeResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupAvatarChange,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupAvatarDelete (&self, param: pb::GroupAvatarDeleteParam) -> Result<pb::GroupAvatarDeleteResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupAvatarDelete,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupAvatarGetList (&self, param: pb::GroupAvatarGetListParam) -> Result<pb::GroupAvatarGetListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupAvatarGetList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupSendDoingAction (&self, param: pb::GroupSendDoingActionParam) -> Result<pb::GroupSendDoingActionResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupSendDoingAction,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupReportGroup (&self, param: pb::GroupReportGroupParam) -> Result<pb::GroupReportGroupResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupReportGroup,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupGetFull (&self, param: pb::GroupGetFullMessageParam) -> Result<pb::GroupGetFullMessageResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupGetFull,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupGetMessagesList (&self, param: pb::GroupGetMessagesListParam) -> Result<pb::GroupGetMessagesListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupGetMessagesList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupGetMediaList (&self, param: pb::GroupGetMediaListParam) -> Result<pb::GroupGetMediaListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupGetMediaList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupGetMembersList (&self, param: pb::GroupGetMembersListParam) -> Result<pb::GroupGetMembersListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupGetMembersList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupGetAdminsList (&self, param: pb::GroupGetAdminsListParam) -> Result<pb::GroupGetAdminsListResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupGetAdminsList,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GroupSetDraft (&self, param: pb::GroupSetDraftParam) -> Result<pb::GroupSetDraftResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GroupSetDraft,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
// service: RPC_Sample
    pub async fn GetUsers1 (&self, param: pb::GetUsers1Param) -> Result<pb::GetUsers1Response,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GetUsers1,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GetProfiles (&self, param: pb::GetProfilesParam) -> Result<pb::GetProfilesResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GetProfiles,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GetChannels (&self, param: pb::GetChannelsParam) -> Result<pb::GetChannelsResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GetChannels,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GetDirects (&self, param: pb::GetDirectsParam) -> Result<pb::GetDirectsResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GetDirects,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn GetMessages (&self, param: pb::GetMessagesParam) -> Result<pb::GetMessagesResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::GetMessages,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
// service: RPC_Shared
    pub async fn Echo (&self, param: pb::EchoParam) -> Result<pb::EchoResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::Echo,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
    pub async fn CheckUserName (&self, param: pb::CheckUserNameParam) -> Result<pb::CheckUserNameResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::CheckUserName,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
// service: RPC_Upload
    pub async fn UploadFile (&self, param: pb::UploadFileParam) -> Result<pb::UploadFileResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::UploadFile,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
// service: RPC_User
    pub async fn ChangePhoneNumber (&self, param: pb::ChangePhoneNumberParam) -> Result<pb::ChangePhoneNumberResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::ChangePhoneNumber,
            action_id: self.get_next_action_id() ,
            is_response: false,
            rpc_data: buff,
        };

        let mut buff =vec![];
        let m = prost::Message::encode(&invoke, &mut buff);

        let mut buff = Vec::new();
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res = ::prost::Message::decode(res_bytes.as_slice())?;
        Ok(pb_res)
    }
    
}

