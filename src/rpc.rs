use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::body;
use serde::{Deserialize, Serialize};
use quick_protobuf::{BytesReader, BytesWriter};
use quick_protobuf::{MessageRead,MessageWrite,Writer,deserialize_from_slice};

use crate::{pb,com,com::*, rpc_fns};

pub mod method_ids {
    // Service: RPC_Account
    pub const ChangePhoneNumber: u32 = 706069694;
    
    // Service: RPC_Auth
    pub const SendConfirmCode: u32 = 939965206;
    pub const ConfirmCode: u32 = 1740258084;
    pub const SingUp: u32 = 291193302;
    pub const SingIn: u32 = 1017957090;
    pub const LogOut: u32 = 1283119009;
    
    // Service: RPC_Channel
    pub const ChannelCreateChannel: u32 = 143251225;
    pub const ChannelEditChannel: u32 = 189471894;
    pub const ChannelDeleteChannel: u32 = 1494483355;
    pub const ChannelAddAuthor: u32 = 780397316;
    pub const ChannelChangeAuthorPermission: u32 = 93233821;
    pub const ChannelRemoveAuthor: u32 = 419542304;
    pub const ChannelFollowChannel: u32 = 744563779;
    pub const ChannelUnFollowChannel: u32 = 959512423;
    pub const ChannelRemoveFollowers: u32 = 869709257;
    pub const ChannelSubscribe: u32 = 1367898912;
    pub const ChannelUnSubscribe: u32 = 858172401;
    pub const ChannelRemoveSubscribers: u32 = 729024592;
    pub const ChannelChangePrivacy: u32 = 79012409;
    pub const ChannelChangeDefaultPermission: u32 = 1582638498;
    pub const ChannelRevokeLink: u32 = 1912530021;
    pub const ChannelChangeUsername: u32 = 983884462;
    pub const ChannelBlockChannel: u32 = 2037016989;
    pub const ChannelSendMessage: u32 = 1200751231;
    pub const ChannelEditMessage: u32 = 727437726;
    pub const ChannelPinMessage: u32 = 259263709;
    pub const ChannelUnPinMessage: u32 = 113943649;
    pub const ChannelDeleteMessage: u32 = 644189206;
    pub const ChannelDeleteMessages: u32 = 2124822181;
    pub const ChannelClearHistory: u32 = 1164398815;
    pub const ChannelAvatarAdd: u32 = 1021808696;
    pub const ChannelAvatarChange: u32 = 1968579501;
    pub const ChannelAvatarDelete: u32 = 1626010891;
    pub const ChannelAvatarGetList: u32 = 1925044843;
    pub const ChannelSendDoingAction: u32 = 973237257;
    pub const ChannelReportChannel: u32 = 792938145;
    pub const ChannelReportMessage: u32 = 2053528327;
    pub const ChannelGetFull: u32 = 1684531258;
    pub const ChannelGetMessagesList: u32 = 1339072968;
    pub const ChannelGetMediaList: u32 = 985772653;
    pub const ChannelGetAuthors: u32 = 1373284924;
    pub const ChannelGetFollowers: u32 = 1747172143;
    pub const ChannelGetFollowings: u32 = 1838438980;
    pub const ChannelGetSubscribers: u32 = 2146806736;
    pub const ChannelBlocked: u32 = 1674411747;
    pub const ChannelSetDraft: u32 = 1403193015;
    
    // Service: RPC_Chat
    pub const ChatSendMessage: u32 = 1131621475;
    pub const ChatEditMessage: u32 = 1806258329;
    pub const ChatDeleteMessages: u32 = 933526170;
    pub const ChatDeleteHistory: u32 = 1088992782;
    pub const ChatSendDoingAction: u32 = 1319324241;
    pub const ChatReportChat: u32 = 1345425871;
    pub const ChatGetFull: u32 = 1768678453;
    pub const ChatGetMessagesList: u32 = 121549718;
    pub const ChatGetMediaList: u32 = 1346774525;
    
    // Service: RPC_Direct
    pub const DirectDeleteDirect: u32 = 1478067518;
    pub const DirectChangeTitle: u32 = 2041790485;
    pub const DirectSetCustomNotification: u32 = 548699291;
    pub const DirectSendActionDoing: u32 = 1417285757;
    pub const DirectSetDraft: u32 = 1860345925;
    pub const DirectDeleteDirects: u32 = 1291891637;
    pub const DirectMarkAsRead: u32 = 1801774787;
    pub const DirectMarkAsUnRead: u32 = 313746334;
    pub const DirectPinDirects: u32 = 1179089068;
    pub const DirectUnPinDirects: u32 = 1517245560;
    pub const DirectArchiveDirects: u32 = 1441782770;
    pub const DirectUnArchiveDirects: u32 = 1951553867;
    pub const DirectClearHistories: u32 = 904052140;
    pub const DirectMuteDirects: u32 = 1138477048;
    pub const DirectUnMuteDirects: u32 = 1691834263;
    pub const DirectCreateFolder: u32 = 1878673022;
    pub const DirectChangeFolder: u32 = 1861381591;
    pub const DirectRemoveFromFolder: u32 = 1818954127;
    pub const DirectReordersFolder: u32 = 1264591958;
    pub const DirectDeleteFolder: u32 = 962281627;
    pub const DirectGetChatsList: u32 = 1570934969;
    pub const DirectGetGroupsList: u32 = 545957996;
    pub const DirectGetChannelsList: u32 = 1608173619;
    pub const DirectGetFoldersList: u32 = 1384523712;
    pub const DirectGetFoldersFullList: u32 = 611850722;
    
    // Service: RPC_General
    pub const Echo: u32 = 101973561;
    pub const CheckUserName: u32 = 1897027349;
    
    // Service: RPC_Group
    pub const GroupCreateGroup: u32 = 1205960678;
    pub const GroupEditGroup: u32 = 1665019493;
    pub const GroupDeleteGroup: u32 = 365183375;
    pub const GroupAddAdmin: u32 = 958971956;
    pub const GroupAddMember: u32 = 676599227;
    pub const GroupRemoveMember: u32 = 2012702964;
    pub const GroupChangeMemberLevel: u32 = 589574238;
    pub const GroupChangeMemberPermission: u32 = 2132464067;
    pub const GroupJoinGroup: u32 = 591743429;
    pub const GroupLeaveGroup: u32 = 361834630;
    pub const GroupBanMember: u32 = 548504852;
    pub const GroupChangePrivacy: u32 = 1497988410;
    pub const GroupChangeDefaultPermission: u32 = 605792138;
    pub const GroupRevokeLink: u32 = 406592509;
    pub const GroupChangeUsername: u32 = 832997038;
    pub const GroupSendMessage: u32 = 599852950;
    pub const GroupEditMessage: u32 = 742937895;
    pub const GroupPinMessage: u32 = 184560027;
    pub const GroupUnPinMessage: u32 = 1290613173;
    pub const GroupDeleteMessage: u32 = 393991035;
    pub const GroupDeleteMessages: u32 = 276700675;
    pub const GroupDeleteHistory: u32 = 1270953793;
    pub const GroupClearHistory: u32 = 1352552449;
    pub const GroupAvatarAdd: u32 = 1202058216;
    pub const GroupAvatarChange: u32 = 108612523;
    pub const GroupAvatarDelete: u32 = 775862697;
    pub const GroupAvatarGetList: u32 = 939443722;
    pub const GroupSendDoingAction: u32 = 2022474356;
    pub const GroupReportGroup: u32 = 1759704420;
    pub const GroupGetFull: u32 = 200351324;
    pub const GroupGetMessagesList: u32 = 1541835459;
    pub const GroupGetMediaList: u32 = 2143016912;
    pub const GroupGetMembersList: u32 = 429215412;
    pub const GroupGetAdminsList: u32 = 332260610;
    pub const GroupSetDraft: u32 = 77668156;
    
    // Service: RPC_Sample
    pub const GetUsers1: u32 = 486248681;
    
    // Service: RPC_Social
    pub const AddComment: u32 = 1222124115;
    pub const DeleteComment: u32 = 1684680875;
    pub const EditComment: u32 = 527415306;
    pub const LikeComment: u32 = 2086146002;
    pub const AddSeenPosts: u32 = 1118533600;
    pub const LikePost: u32 = 1313969677;
    pub const UnLikePost: u32 = 1332796256;
    pub const FollowChannel: u32 = 655898778;
    pub const UnFollowChannel: u32 = 483078047;
    pub const PinChannel: u32 = 1225489769;
    pub const UnPinChannel: u32 = 1585401362;
    pub const BlockChannel: u32 = 1902848482;
    pub const UnBlockChannel: u32 = 305468874;
    
    // Service: RPC_Upload
    pub const UploadFile: u32 = 1702285478;
    
    pub const ChangePhoneNumber8 : u32 = 79874;
}

pub async fn server_rpc(act : pb::Invoke) -> Result<Vec<u8>,GenErr> {
    let up = UserParam{};

    match act.method {
    
    // service: RPC_Account
        method_ids::ChangePhoneNumber => { // 706069694
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
    
    // service: RPC_Auth
        method_ids::SendConfirmCode => { // 939965206
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
        method_ids::ConfirmCode => { // 1740258084
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
        method_ids::SingUp => { // 291193302
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
        method_ids::SingIn => { // 1017957090
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
        method_ids::LogOut => { // 1283119009
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
        method_ids::ChannelCreateChannel => { // 143251225
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
        method_ids::ChannelEditChannel => { // 189471894
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
        method_ids::ChannelDeleteChannel => { // 1494483355
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
        method_ids::ChannelAddAuthor => { // 780397316
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
        method_ids::ChannelChangeAuthorPermission => { // 93233821
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
        method_ids::ChannelRemoveAuthor => { // 419542304
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
        method_ids::ChannelFollowChannel => { // 744563779
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
        method_ids::ChannelUnFollowChannel => { // 959512423
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
        method_ids::ChannelRemoveFollowers => { // 869709257
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
        method_ids::ChannelSubscribe => { // 1367898912
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
        method_ids::ChannelUnSubscribe => { // 858172401
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
        method_ids::ChannelRemoveSubscribers => { // 729024592
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
        method_ids::ChannelChangePrivacy => { // 79012409
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
        method_ids::ChannelChangeDefaultPermission => { // 1582638498
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
        method_ids::ChannelRevokeLink => { // 1912530021
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
        method_ids::ChannelChangeUsername => { // 983884462
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
        method_ids::ChannelBlockChannel => { // 2037016989
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
        method_ids::ChannelSendMessage => { // 1200751231
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
        method_ids::ChannelEditMessage => { // 727437726
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
        method_ids::ChannelPinMessage => { // 259263709
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
        method_ids::ChannelUnPinMessage => { // 113943649
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
        method_ids::ChannelDeleteMessage => { // 644189206
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
        method_ids::ChannelDeleteMessages => { // 2124822181
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
        method_ids::ChannelClearHistory => { // 1164398815
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
        method_ids::ChannelAvatarAdd => { // 1021808696
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
        method_ids::ChannelAvatarChange => { // 1968579501
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
        method_ids::ChannelAvatarDelete => { // 1626010891
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
        method_ids::ChannelAvatarGetList => { // 1925044843
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
        method_ids::ChannelSendDoingAction => { // 973237257
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
        method_ids::ChannelReportChannel => { // 792938145
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
        method_ids::ChannelReportMessage => { // 2053528327
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
        method_ids::ChannelGetFull => { // 1684531258
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
        method_ids::ChannelGetMessagesList => { // 1339072968
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
        method_ids::ChannelGetMediaList => { // 985772653
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
        method_ids::ChannelGetAuthors => { // 1373284924
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
        method_ids::ChannelGetFollowers => { // 1747172143
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
        method_ids::ChannelGetFollowings => { // 1838438980
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
        method_ids::ChannelGetSubscribers => { // 2146806736
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
        method_ids::ChannelBlocked => { // 1674411747
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
        method_ids::ChannelSetDraft => { // 1403193015
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
        method_ids::ChatSendMessage => { // 1131621475
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
        method_ids::ChatEditMessage => { // 1806258329
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
        method_ids::ChatDeleteMessages => { // 933526170
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
        method_ids::ChatDeleteHistory => { // 1088992782
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
        method_ids::ChatSendDoingAction => { // 1319324241
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
        method_ids::ChatReportChat => { // 1345425871
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
        method_ids::ChatGetFull => { // 1768678453
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
        method_ids::ChatGetMessagesList => { // 121549718
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
        method_ids::ChatGetMediaList => { // 1346774525
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
        method_ids::DirectDeleteDirect => { // 1478067518
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
        method_ids::DirectChangeTitle => { // 2041790485
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
        method_ids::DirectSetCustomNotification => { // 548699291
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
        method_ids::DirectSendActionDoing => { // 1417285757
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
        method_ids::DirectSetDraft => { // 1860345925
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
        method_ids::DirectDeleteDirects => { // 1291891637
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
        method_ids::DirectMarkAsRead => { // 1801774787
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
        method_ids::DirectMarkAsUnRead => { // 313746334
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
        method_ids::DirectPinDirects => { // 1179089068
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
        method_ids::DirectUnPinDirects => { // 1517245560
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
        method_ids::DirectArchiveDirects => { // 1441782770
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
        method_ids::DirectUnArchiveDirects => { // 1951553867
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
        method_ids::DirectClearHistories => { // 904052140
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
        method_ids::DirectMuteDirects => { // 1138477048
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
        method_ids::DirectUnMuteDirects => { // 1691834263
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
        method_ids::DirectCreateFolder => { // 1878673022
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
        method_ids::DirectChangeFolder => { // 1861381591
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
        method_ids::DirectRemoveFromFolder => { // 1818954127
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
        method_ids::DirectReordersFolder => { // 1264591958
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
        method_ids::DirectDeleteFolder => { // 962281627
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
        method_ids::DirectGetChatsList => { // 1570934969
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
        method_ids::DirectGetGroupsList => { // 545957996
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
        method_ids::DirectGetChannelsList => { // 1608173619
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
        method_ids::DirectGetFoldersList => { // 1384523712
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
        method_ids::DirectGetFoldersFullList => { // 611850722
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
    
    // service: RPC_General
        method_ids::Echo => { // 101973561
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
        method_ids::CheckUserName => { // 1897027349
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
    
    // service: RPC_Group
        method_ids::GroupCreateGroup => { // 1205960678
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
        method_ids::GroupEditGroup => { // 1665019493
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
        method_ids::GroupDeleteGroup => { // 365183375
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
        method_ids::GroupAddAdmin => { // 958971956
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
        method_ids::GroupAddMember => { // 676599227
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
        method_ids::GroupRemoveMember => { // 2012702964
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
        method_ids::GroupChangeMemberLevel => { // 589574238
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
        method_ids::GroupChangeMemberPermission => { // 2132464067
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
        method_ids::GroupJoinGroup => { // 591743429
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
        method_ids::GroupLeaveGroup => { // 361834630
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
        method_ids::GroupBanMember => { // 548504852
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
        method_ids::GroupChangePrivacy => { // 1497988410
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
        method_ids::GroupChangeDefaultPermission => { // 605792138
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
        method_ids::GroupRevokeLink => { // 406592509
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
        method_ids::GroupChangeUsername => { // 832997038
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
        method_ids::GroupSendMessage => { // 599852950
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
        method_ids::GroupEditMessage => { // 742937895
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
        method_ids::GroupPinMessage => { // 184560027
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
        method_ids::GroupUnPinMessage => { // 1290613173
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
        method_ids::GroupDeleteMessage => { // 393991035
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
        method_ids::GroupDeleteMessages => { // 276700675
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
        method_ids::GroupDeleteHistory => { // 1270953793
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
        method_ids::GroupClearHistory => { // 1352552449
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
        method_ids::GroupAvatarAdd => { // 1202058216
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
        method_ids::GroupAvatarChange => { // 108612523
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
        method_ids::GroupAvatarDelete => { // 775862697
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
        method_ids::GroupAvatarGetList => { // 939443722
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
        method_ids::GroupSendDoingAction => { // 2022474356
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
        method_ids::GroupReportGroup => { // 1759704420
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
        method_ids::GroupGetFull => { // 200351324
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
        method_ids::GroupGetMessagesList => { // 1541835459
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
        method_ids::GroupGetMediaList => { // 2143016912
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
        method_ids::GroupGetMembersList => { // 429215412
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
        method_ids::GroupGetAdminsList => { // 332260610
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
        method_ids::GroupSetDraft => { // 77668156
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
        method_ids::GetUsers1 => { // 486248681
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
    
    // service: RPC_Social
        method_ids::AddComment => { // 1222124115
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::AddCommentParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::AddComment(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::DeleteComment => { // 1684680875
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::DeleteCommentParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::DeleteComment(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::EditComment => { // 527415306
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::EditCommentParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::EditComment(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::LikeComment => { // 2086146002
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::LikeCommentParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::LikeComment(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::AddSeenPosts => { // 1118533600
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::AddSeenPostsParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::AddSeenPosts(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::LikePost => { // 1313969677
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::LikePostParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::LikePost(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::UnLikePost => { // 1332796256
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::UnLikePostParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::UnLikePost(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::FollowChannel => { // 655898778
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::FollowChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::FollowChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::UnFollowChannel => { // 483078047
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::UnFollowChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::UnFollowChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::PinChannel => { // 1225489769
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::PinChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::PinChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::UnPinChannel => { // 1585401362
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::UnPinChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::UnPinChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::BlockChannel => { // 1902848482
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::BlockChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::BlockChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
        method_ids::UnBlockChannel => { // 305468874
            let vec: Vec<u8> = vec![];
            let rpc_param  : Result<pb::UnBlockChannelParam, ::prost::DecodeError> = prost::Message::decode(act.rpc_data.as_slice());

            if let Ok(param) = rpc_param {
                println!("param {:?}", param);
                let response = rpc_fns::UnBlockChannel(&up, param).await?;

                let mut buff =vec![];
                prost::Message::encode(&response, &mut buff)?;

                Ok(buff)
            } else {
                Err(GenErr::ReadingPbParam)
            }
        }
    
    // service: RPC_Upload
        method_ids::UploadFile => { // 1702285478
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

// service: RPC_Account
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
    
// service: RPC_General
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
    
// service: RPC_Social
    pub async fn AddComment (&self, param: pb::AddCommentParam) -> Result<pb::AddCommentResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::AddComment,
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
    
    pub async fn DeleteComment (&self, param: pb::DeleteCommentParam) -> Result<pb::DeleteCommentResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::DeleteComment,
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
    
    pub async fn EditComment (&self, param: pb::EditCommentParam) -> Result<pb::EditCommentResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::EditComment,
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
    
    pub async fn LikeComment (&self, param: pb::LikeCommentParam) -> Result<pb::LikeCommentResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::LikeComment,
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
    
    pub async fn AddSeenPosts (&self, param: pb::AddSeenPostsParam) -> Result<pb::AddSeenPostsResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::AddSeenPosts,
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
    
    pub async fn LikePost (&self, param: pb::LikePostParam) -> Result<pb::LikePostResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::LikePost,
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
    
    pub async fn UnLikePost (&self, param: pb::UnLikePostParam) -> Result<pb::UnLikePostResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::UnLikePost,
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
    
    pub async fn FollowChannel (&self, param: pb::FollowChannelParam) -> Result<pb::FollowChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::FollowChannel,
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
    
    pub async fn UnFollowChannel (&self, param: pb::UnFollowChannelParam) -> Result<pb::UnFollowChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::UnFollowChannel,
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
    
    pub async fn PinChannel (&self, param: pb::PinChannelParam) -> Result<pb::PinChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::PinChannel,
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
    
    pub async fn UnPinChannel (&self, param: pb::UnPinChannelParam) -> Result<pb::UnPinChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::UnPinChannel,
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
    
    pub async fn BlockChannel (&self, param: pb::BlockChannelParam) -> Result<pb::BlockChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::BlockChannel,
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
    
    pub async fn UnBlockChannel (&self, param: pb::UnBlockChannelParam) -> Result<pb::UnBlockChannelResponse,GenErr>{

        let mut buff =vec![];
        ::prost::Message::encode(&param, &mut buff)?;

        let invoke = pb::Invoke {
            namespace: 0,
            method: method_ids::UnBlockChannel,
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
    
}

