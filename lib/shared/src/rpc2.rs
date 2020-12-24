//#![rustfmt::skip]

use crate::pb::{EchoParam, EchoResponse};
use crate::{common, pb};
use crate::{errors::GenErr, UserParam};
use async_trait::async_trait;

use http::Version;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error as HyperError, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct RpcInvoke {
    method_id: i64, // correct data type should be i32,
    rpc_service: RpcServiceData,
}

#[derive(Debug)]
pub enum RpcServiceData {
    RPC_Auth(RPC_Auth_MethodData),
    RPC_Channel(RPC_Channel_MethodData),
    RPC_Chat(RPC_Chat_MethodData),
    RPC_Direct(RPC_Direct_MethodData),
    RPC_Group(RPC_Group_MethodData),
    RPC_Sample(RPC_Sample_MethodData),
    RPC_Shared(RPC_Shared_MethodData),
    RPC_Upload(RPC_Upload_MethodData),
    RPC_User(RPC_User_MethodData),
}

#[derive(Debug)]
pub enum RPC_Auth_MethodData {
    SendConfirmCode(pb::SendConfirmCodeParam),
    ConfirmCode(pb::ConfirmCodeParam),
    SingUp(pb::SingUpParam),
    SingIn(pb::SingInParam),
    LogOut(pb::LogOutParam),
}
#[derive(Debug)]
pub enum RPC_Channel_MethodData {
    ChannelCreateChannel(pb::ChannelCreateChannelParam),
    ChannelEditChannel(pb::ChannelEditChannelParam),
    ChannelDeleteChannel(pb::ChannelDeleteChannelParam),
    ChannelAddAuthor(pb::ChannelAddAuthorParam),
    ChannelChangeAuthorPermission(pb::ChannelChangeAuthorPermissionParam),
    ChannelRemoveAuthor(pb::ChannelRemoveAuthorParam),
    ChannelFollowChannel(pb::ChannelFollowChannelParam),
    ChannelUnFollowChannel(pb::ChannelUnFollowChannelParam),
    ChannelRemoveFollowers(pb::ChannelRemoveFollowersParam),
    ChannelSubscribe(pb::ChannelSubscribeParam),
    ChannelUnSubscribe(pb::ChannelUnSubscribeParam),
    ChannelRemoveSubscribers(pb::ChannelRemoveSubscribersParam),
    ChannelChangePrivacy(pb::ChannelChangePrivacyParam),
    ChannelChangeDefaultPermission(pb::ChannelChangeDefaultPermissionParam),
    ChannelRevokeLink(pb::ChannelRevokeLinkParam),
    ChannelChangeUsername(pb::ChannelChangeUsernameParam),
    ChannelBlockChannel(pb::ChannelBlockChannelParam),
    ChannelSendMessage(pb::ChannelSendMessageParam),
    ChannelEditMessage(pb::ChannelEditMessageParam),
    ChannelPinMessage(pb::ChannelPinMessageParam),
    ChannelUnPinMessage(pb::ChannelUnPinMessageParam),
    ChannelDeleteMessage(pb::ChannelDeleteMessageParam),
    ChannelDeleteMessages(pb::ChannelDeleteMessagesParam),
    ChannelClearHistory(pb::ChannelClearHistoryParam),
    ChannelAvatarAdd(pb::ChannelAvatarAddParam),
    ChannelAvatarChange(pb::ChannelAvatarChangeParam),
    ChannelAvatarDelete(pb::ChannelAvatarDeleteParam),
    ChannelAvatarGetList(pb::ChannelAvatarGetListParam),
    ChannelSendDoingAction(pb::ChannelSendDoingActionParam),
    ChannelReportChannel(pb::ChannelReportChannelParam),
    ChannelReportMessage(pb::ChannelReportMessageParam),
    ChannelGetFull(pb::ChannelGetFullParam),
    ChannelGetMessagesList(pb::ChannelGetMessagesListParam),
    ChannelGetMediaList(pb::ChannelGetMediaListParam),
    ChannelGetAuthors(pb::ChannelGetAuthorsParam),
    ChannelGetFollowers(pb::ChannelGetFollowersParam),
    ChannelGetFollowings(pb::ChannelGetFollowingsParam),
    ChannelGetSubscribers(pb::ChannelGetSubscribersParam),
    ChannelBlocked(pb::ChannelBlockedParam),
    ChannelSetDraft(pb::ChannelSetDraftParam),
}
#[derive(Debug)]
pub enum RPC_Chat_MethodData {
    ChatSendMessage(pb::ChatSendMessageParam),
    ChatEditMessage(pb::ChatEditMessageParam),
    ChatDeleteMessages(pb::ChatDeleteMessagesParam),
    ChatDeleteHistory(pb::ChatDeleteHistoryParam),
    ChatSendDoingAction(pb::ChatSendDoingActionParam),
    ChatReportChat(pb::ChatReportChatParam),
    ChatGetFull(pb::ChatGetFullMessageParam),
    ChatGetMessagesList(pb::ChatGetMessagesListParam),
    ChatGetMediaList(pb::ChatGetMediaListParam),
}
#[derive(Debug)]
pub enum RPC_Direct_MethodData {
    DirectDeleteDirect(pb::DirectDeleteDirectParam),
    DirectChangeTitle(pb::DirectChangeTitleParam),
    DirectSetCustomNotification(pb::DirectSetCustomNotificationParam),
    DirectSendActionDoing(pb::DirectSendActionDoingParam),
    DirectSetDraft(pb::DirectSetDraftParam),
    DirectDeleteDirects(pb::DirectDeleteDirectsParam),
    DirectMarkAsRead(pb::DirectMarkAsReadParam),
    DirectMarkAsUnRead(pb::DirectMarkAsUnReadParam),
    DirectPinDirects(pb::DirectPinDirectsParam),
    DirectUnPinDirects(pb::DirectUnPinDirectsParam),
    DirectArchiveDirects(pb::DirectArchiveDirectsParam),
    DirectUnArchiveDirects(pb::DirectUnArchiveDirectsParam),
    DirectClearHistories(pb::DirectClearHistoriesParam),
    DirectMuteDirects(pb::DirectMuteDirectsParam),
    DirectUnMuteDirects(pb::DirectUnMuteDirectsParam),
    DirectCreateFolder(pb::DirectCreateFolderParam),
    DirectChangeFolder(pb::DirectChangeFolderParam),
    DirectRemoveFromFolder(pb::DirectRemoveFromFolderParam),
    DirectReordersFolder(pb::DirectReordersFolderParam),
    DirectDeleteFolder(pb::DirectDeleteFolderParam),
    DirectGetChatsList(pb::DirectGetChatsListParam),
    DirectGetGroupsList(pb::DirectGetGroupsListParam),
    DirectGetChannelsList(pb::DirectGetChannelsListParam),
    DirectGetFoldersList(pb::DirectGetFoldersListParam),
    DirectGetFoldersFullList(pb::DirectGetFoldersFullListParam),
}
#[derive(Debug)]
pub enum RPC_Group_MethodData {
    GroupCreateGroup(pb::GroupCreateGroupParam),
    GroupEditGroup(pb::GroupEditGroupParam),
    GroupDeleteGroup(pb::GroupDeleteGroupParam),
    GroupAddAdmin(pb::GroupAddAdminParam),
    GroupAddMember(pb::GroupAddMemberParam),
    GroupRemoveMember(pb::GroupRemoveMemberParam),
    GroupChangeMemberLevel(pb::GroupChangeMemberLevelParam),
    GroupChangeMemberPermission(pb::GroupChangeMemberPermissionParam),
    GroupJoinGroup(pb::JoinGroupParam),
    GroupLeaveGroup(pb::GroupLeaveGroupParam),
    GroupBanMember(pb::GroupBanMemberParam),
    GroupChangePrivacy(pb::GroupChangePrivacyParam),
    GroupChangeDefaultPermission(pb::GroupChangeDefaultPermissionParam),
    GroupRevokeLink(pb::GroupRevokeLinkParam),
    GroupChangeUsername(pb::GroupChangeUsernameParam),
    GroupSendMessage(pb::GroupSendMessageParam),
    GroupEditMessage(pb::GroupEditMessageParam),
    GroupPinMessage(pb::GroupPinMessageParam),
    GroupUnPinMessage(pb::GroupUnPinMessageParam),
    GroupDeleteMessage(pb::GroupDeleteMessageParam),
    GroupDeleteMessages(pb::GroupDeleteMessagesParam),
    GroupDeleteHistory(pb::GroupDeleteHistoryParam),
    GroupClearHistory(pb::GroupClearHistoryParam),
    GroupAvatarAdd(pb::GroupAvatarAddParam),
    GroupAvatarChange(pb::GroupAvatarChangeParam),
    GroupAvatarDelete(pb::GroupAvatarDeleteParam),
    GroupAvatarGetList(pb::GroupAvatarGetListParam),
    GroupSendDoingAction(pb::GroupSendDoingActionParam),
    GroupReportGroup(pb::GroupReportGroupParam),
    GroupGetFull(pb::GroupGetFullMessageParam),
    GroupGetMessagesList(pb::GroupGetMessagesListParam),
    GroupGetMediaList(pb::GroupGetMediaListParam),
    GroupGetMembersList(pb::GroupGetMembersListParam),
    GroupGetAdminsList(pb::GroupGetAdminsListParam),
    GroupSetDraft(pb::GroupSetDraftParam),
}
#[derive(Debug)]
pub enum RPC_Sample_MethodData {
    GetUsers1(pb::GetUsers1Param),
    GetProfiles(pb::GetProfilesParam),
    GetChannels(pb::GetChannelsParam),
    GetDirects(pb::GetDirectsParam),
    GetMessages(pb::GetMessagesParam),
}
#[derive(Debug)]
pub enum RPC_Shared_MethodData {
    Echo(pb::EchoParam),
    CheckUserName(pb::CheckUserNameParam),
}
#[derive(Debug)]
pub enum RPC_Upload_MethodData {
    UploadFile(pb::UploadFileParam),
}
#[derive(Debug)]
pub enum RPC_User_MethodData {
    ChangePhoneNumber(pb::ChangePhoneNumberParam),
}

#[async_trait]
pub trait RPC_Auth_Handler {
    async fn SendConfirmCode(
        up: &UserParam,
        param: pb::SendConfirmCodeParam,
    ) -> Result<pb::SendConfirmCodeResponse, GenErr> {
        Ok(pb::SendConfirmCodeResponse::default())
    }
    async fn ConfirmCode(
        up: &UserParam,
        param: pb::ConfirmCodeParam,
    ) -> Result<pb::ConfirmCodeResponse, GenErr> {
        Ok(pb::ConfirmCodeResponse::default())
    }
    async fn SingUp(up: &UserParam, param: pb::SingUpParam) -> Result<pb::SingUpResponse, GenErr> {
        Ok(pb::SingUpResponse::default())
    }
    async fn SingIn(up: &UserParam, param: pb::SingInParam) -> Result<pb::SingInResponse, GenErr> {
        Ok(pb::SingInResponse::default())
    }
    async fn LogOut(up: &UserParam, param: pb::LogOutParam) -> Result<pb::LogOutResponse, GenErr> {
        Ok(pb::LogOutResponse::default())
    }
}
#[async_trait]
pub trait RPC_Channel_Handler {
    async fn ChannelCreateChannel(
        up: &UserParam,
        param: pb::ChannelCreateChannelParam,
    ) -> Result<pb::ChannelCreateChannelResponse, GenErr> {
        Ok(pb::ChannelCreateChannelResponse::default())
    }
    async fn ChannelEditChannel(
        up: &UserParam,
        param: pb::ChannelEditChannelParam,
    ) -> Result<pb::ChannelEditChannelResponse, GenErr> {
        Ok(pb::ChannelEditChannelResponse::default())
    }
    async fn ChannelDeleteChannel(
        up: &UserParam,
        param: pb::ChannelDeleteChannelParam,
    ) -> Result<pb::ChannelDeleteChannelResponse, GenErr> {
        Ok(pb::ChannelDeleteChannelResponse::default())
    }
    async fn ChannelAddAuthor(
        up: &UserParam,
        param: pb::ChannelAddAuthorParam,
    ) -> Result<pb::ChannelAddAuthorResponse, GenErr> {
        Ok(pb::ChannelAddAuthorResponse::default())
    }
    async fn ChannelChangeAuthorPermission(
        up: &UserParam,
        param: pb::ChannelChangeAuthorPermissionParam,
    ) -> Result<pb::ChannelChangeAuthorPermissionResponse, GenErr> {
        Ok(pb::ChannelChangeAuthorPermissionResponse::default())
    }
    async fn ChannelRemoveAuthor(
        up: &UserParam,
        param: pb::ChannelRemoveAuthorParam,
    ) -> Result<pb::ChannelRemoveAuthorResponse, GenErr> {
        Ok(pb::ChannelRemoveAuthorResponse::default())
    }
    async fn ChannelFollowChannel(
        up: &UserParam,
        param: pb::ChannelFollowChannelParam,
    ) -> Result<pb::ChannelFollowChannelResponse, GenErr> {
        Ok(pb::ChannelFollowChannelResponse::default())
    }
    async fn ChannelUnFollowChannel(
        up: &UserParam,
        param: pb::ChannelUnFollowChannelParam,
    ) -> Result<pb::ChannelUnFollowChannelResponse, GenErr> {
        Ok(pb::ChannelUnFollowChannelResponse::default())
    }
    async fn ChannelRemoveFollowers(
        up: &UserParam,
        param: pb::ChannelRemoveFollowersParam,
    ) -> Result<pb::ChannelRemoveFollowersResponse, GenErr> {
        Ok(pb::ChannelRemoveFollowersResponse::default())
    }
    async fn ChannelSubscribe(
        up: &UserParam,
        param: pb::ChannelSubscribeParam,
    ) -> Result<pb::ChannelSubscribeResponse, GenErr> {
        Ok(pb::ChannelSubscribeResponse::default())
    }
    async fn ChannelUnSubscribe(
        up: &UserParam,
        param: pb::ChannelUnSubscribeParam,
    ) -> Result<pb::ChannelUnSubscribeResponse, GenErr> {
        Ok(pb::ChannelUnSubscribeResponse::default())
    }
    async fn ChannelRemoveSubscribers(
        up: &UserParam,
        param: pb::ChannelRemoveSubscribersParam,
    ) -> Result<pb::ChannelRemoveSubscribersResponse, GenErr> {
        Ok(pb::ChannelRemoveSubscribersResponse::default())
    }
    async fn ChannelChangePrivacy(
        up: &UserParam,
        param: pb::ChannelChangePrivacyParam,
    ) -> Result<pb::ChannelChangePrivacyResponse, GenErr> {
        Ok(pb::ChannelChangePrivacyResponse::default())
    }
    async fn ChannelChangeDefaultPermission(
        up: &UserParam,
        param: pb::ChannelChangeDefaultPermissionParam,
    ) -> Result<pb::ChannelChangeDefaultPermissionResponse, GenErr> {
        Ok(pb::ChannelChangeDefaultPermissionResponse::default())
    }
    async fn ChannelRevokeLink(
        up: &UserParam,
        param: pb::ChannelRevokeLinkParam,
    ) -> Result<pb::ChannelRevokeLinkResponse, GenErr> {
        Ok(pb::ChannelRevokeLinkResponse::default())
    }
    async fn ChannelChangeUsername(
        up: &UserParam,
        param: pb::ChannelChangeUsernameParam,
    ) -> Result<pb::ChannelChangeUsernameResponse, GenErr> {
        Ok(pb::ChannelChangeUsernameResponse::default())
    }
    async fn ChannelBlockChannel(
        up: &UserParam,
        param: pb::ChannelBlockChannelParam,
    ) -> Result<pb::ChannelBlockChannelResponse, GenErr> {
        Ok(pb::ChannelBlockChannelResponse::default())
    }
    async fn ChannelSendMessage(
        up: &UserParam,
        param: pb::ChannelSendMessageParam,
    ) -> Result<pb::ChannelSendMessageResponse, GenErr> {
        Ok(pb::ChannelSendMessageResponse::default())
    }
    async fn ChannelEditMessage(
        up: &UserParam,
        param: pb::ChannelEditMessageParam,
    ) -> Result<pb::ChannelEditMessageResponse, GenErr> {
        Ok(pb::ChannelEditMessageResponse::default())
    }
    async fn ChannelPinMessage(
        up: &UserParam,
        param: pb::ChannelPinMessageParam,
    ) -> Result<pb::ChannelPinMessageResponse, GenErr> {
        Ok(pb::ChannelPinMessageResponse::default())
    }
    async fn ChannelUnPinMessage(
        up: &UserParam,
        param: pb::ChannelUnPinMessageParam,
    ) -> Result<pb::ChannelUnPinMessageResponse, GenErr> {
        Ok(pb::ChannelUnPinMessageResponse::default())
    }
    async fn ChannelDeleteMessage(
        up: &UserParam,
        param: pb::ChannelDeleteMessageParam,
    ) -> Result<pb::ChannelDeleteMessageResponse, GenErr> {
        Ok(pb::ChannelDeleteMessageResponse::default())
    }
    async fn ChannelDeleteMessages(
        up: &UserParam,
        param: pb::ChannelDeleteMessagesParam,
    ) -> Result<pb::ChannelDeleteMessagesResponse, GenErr> {
        Ok(pb::ChannelDeleteMessagesResponse::default())
    }
    async fn ChannelClearHistory(
        up: &UserParam,
        param: pb::ChannelClearHistoryParam,
    ) -> Result<pb::ChannelClearHistoryResponse, GenErr> {
        Ok(pb::ChannelClearHistoryResponse::default())
    }
    async fn ChannelAvatarAdd(
        up: &UserParam,
        param: pb::ChannelAvatarAddParam,
    ) -> Result<pb::ChannelAvatarAddResponse, GenErr> {
        Ok(pb::ChannelAvatarAddResponse::default())
    }
    async fn ChannelAvatarChange(
        up: &UserParam,
        param: pb::ChannelAvatarChangeParam,
    ) -> Result<pb::ChannelAvatarChangeResponse, GenErr> {
        Ok(pb::ChannelAvatarChangeResponse::default())
    }
    async fn ChannelAvatarDelete(
        up: &UserParam,
        param: pb::ChannelAvatarDeleteParam,
    ) -> Result<pb::ChannelAvatarDeleteResponse, GenErr> {
        Ok(pb::ChannelAvatarDeleteResponse::default())
    }
    async fn ChannelAvatarGetList(
        up: &UserParam,
        param: pb::ChannelAvatarGetListParam,
    ) -> Result<pb::ChannelAvatarGetListResponse, GenErr> {
        Ok(pb::ChannelAvatarGetListResponse::default())
    }
    async fn ChannelSendDoingAction(
        up: &UserParam,
        param: pb::ChannelSendDoingActionParam,
    ) -> Result<pb::ChannelSendDoingActionResponse, GenErr> {
        Ok(pb::ChannelSendDoingActionResponse::default())
    }
    async fn ChannelReportChannel(
        up: &UserParam,
        param: pb::ChannelReportChannelParam,
    ) -> Result<pb::ChannelReportChannelResponse, GenErr> {
        Ok(pb::ChannelReportChannelResponse::default())
    }
    async fn ChannelReportMessage(
        up: &UserParam,
        param: pb::ChannelReportMessageParam,
    ) -> Result<pb::ChannelReportMessageResponse, GenErr> {
        Ok(pb::ChannelReportMessageResponse::default())
    }
    async fn ChannelGetFull(
        up: &UserParam,
        param: pb::ChannelGetFullParam,
    ) -> Result<pb::ChannelGetFullResponse, GenErr> {
        Ok(pb::ChannelGetFullResponse::default())
    }
    async fn ChannelGetMessagesList(
        up: &UserParam,
        param: pb::ChannelGetMessagesListParam,
    ) -> Result<pb::ChannelGetMessagesListResponse, GenErr> {
        Ok(pb::ChannelGetMessagesListResponse::default())
    }
    async fn ChannelGetMediaList(
        up: &UserParam,
        param: pb::ChannelGetMediaListParam,
    ) -> Result<pb::ChannelGetMediaListResponse, GenErr> {
        Ok(pb::ChannelGetMediaListResponse::default())
    }
    async fn ChannelGetAuthors(
        up: &UserParam,
        param: pb::ChannelGetAuthorsParam,
    ) -> Result<pb::ChannelGetAuthorsResponse, GenErr> {
        Ok(pb::ChannelGetAuthorsResponse::default())
    }
    async fn ChannelGetFollowers(
        up: &UserParam,
        param: pb::ChannelGetFollowersParam,
    ) -> Result<pb::ChannelGetFollowersResponse, GenErr> {
        Ok(pb::ChannelGetFollowersResponse::default())
    }
    async fn ChannelGetFollowings(
        up: &UserParam,
        param: pb::ChannelGetFollowingsParam,
    ) -> Result<pb::ChannelGetFollowingsResponse, GenErr> {
        Ok(pb::ChannelGetFollowingsResponse::default())
    }
    async fn ChannelGetSubscribers(
        up: &UserParam,
        param: pb::ChannelGetSubscribersParam,
    ) -> Result<pb::ChannelGetSubscribersResponse, GenErr> {
        Ok(pb::ChannelGetSubscribersResponse::default())
    }
    async fn ChannelBlocked(
        up: &UserParam,
        param: pb::ChannelBlockedParam,
    ) -> Result<pb::ChannelBlockedResponse, GenErr> {
        Ok(pb::ChannelBlockedResponse::default())
    }
    async fn ChannelSetDraft(
        up: &UserParam,
        param: pb::ChannelSetDraftParam,
    ) -> Result<pb::ChannelSetDraftResponse, GenErr> {
        Ok(pb::ChannelSetDraftResponse::default())
    }
}
#[async_trait]
pub trait RPC_Chat_Handler {
    async fn ChatSendMessage(
        up: &UserParam,
        param: pb::ChatSendMessageParam,
    ) -> Result<pb::ChatSendMessageResponse, GenErr> {
        Ok(pb::ChatSendMessageResponse::default())
    }
    async fn ChatEditMessage(
        up: &UserParam,
        param: pb::ChatEditMessageParam,
    ) -> Result<pb::ChatEditMessageResponse, GenErr> {
        Ok(pb::ChatEditMessageResponse::default())
    }
    async fn ChatDeleteMessages(
        up: &UserParam,
        param: pb::ChatDeleteMessagesParam,
    ) -> Result<pb::ChatDeleteMessagesResponse, GenErr> {
        Ok(pb::ChatDeleteMessagesResponse::default())
    }
    async fn ChatDeleteHistory(
        up: &UserParam,
        param: pb::ChatDeleteHistoryParam,
    ) -> Result<pb::ChatDeleteHistoryResponse, GenErr> {
        Ok(pb::ChatDeleteHistoryResponse::default())
    }
    async fn ChatSendDoingAction(
        up: &UserParam,
        param: pb::ChatSendDoingActionParam,
    ) -> Result<pb::ChatSendDoingActionResponse, GenErr> {
        Ok(pb::ChatSendDoingActionResponse::default())
    }
    async fn ChatReportChat(
        up: &UserParam,
        param: pb::ChatReportChatParam,
    ) -> Result<pb::ChatReportChatResponse, GenErr> {
        Ok(pb::ChatReportChatResponse::default())
    }
    async fn ChatGetFull(
        up: &UserParam,
        param: pb::ChatGetFullMessageParam,
    ) -> Result<pb::ChatGetFullMessageResponse, GenErr> {
        Ok(pb::ChatGetFullMessageResponse::default())
    }
    async fn ChatGetMessagesList(
        up: &UserParam,
        param: pb::ChatGetMessagesListParam,
    ) -> Result<pb::ChatGetMessagesListResponse, GenErr> {
        Ok(pb::ChatGetMessagesListResponse::default())
    }
    async fn ChatGetMediaList(
        up: &UserParam,
        param: pb::ChatGetMediaListParam,
    ) -> Result<pb::ChatGetMediaListResponse, GenErr> {
        Ok(pb::ChatGetMediaListResponse::default())
    }
}
#[async_trait]
pub trait RPC_Direct_Handler {
    async fn DirectDeleteDirect(
        up: &UserParam,
        param: pb::DirectDeleteDirectParam,
    ) -> Result<pb::DirectDeleteDirectResponse, GenErr> {
        Ok(pb::DirectDeleteDirectResponse::default())
    }
    async fn DirectChangeTitle(
        up: &UserParam,
        param: pb::DirectChangeTitleParam,
    ) -> Result<pb::DirectChangeTitleResponse, GenErr> {
        Ok(pb::DirectChangeTitleResponse::default())
    }
    async fn DirectSetCustomNotification(
        up: &UserParam,
        param: pb::DirectSetCustomNotificationParam,
    ) -> Result<pb::DirectSetCustomNotificationResponse, GenErr> {
        Ok(pb::DirectSetCustomNotificationResponse::default())
    }
    async fn DirectSendActionDoing(
        up: &UserParam,
        param: pb::DirectSendActionDoingParam,
    ) -> Result<pb::DirectSendActionDoingResponse, GenErr> {
        Ok(pb::DirectSendActionDoingResponse::default())
    }
    async fn DirectSetDraft(
        up: &UserParam,
        param: pb::DirectSetDraftParam,
    ) -> Result<pb::DirectSetDraftResponse, GenErr> {
        Ok(pb::DirectSetDraftResponse::default())
    }
    async fn DirectDeleteDirects(
        up: &UserParam,
        param: pb::DirectDeleteDirectsParam,
    ) -> Result<pb::DirectDeleteDirectsResponse, GenErr> {
        Ok(pb::DirectDeleteDirectsResponse::default())
    }
    async fn DirectMarkAsRead(
        up: &UserParam,
        param: pb::DirectMarkAsReadParam,
    ) -> Result<pb::DirectMarkAsReadResponse, GenErr> {
        Ok(pb::DirectMarkAsReadResponse::default())
    }
    async fn DirectMarkAsUnRead(
        up: &UserParam,
        param: pb::DirectMarkAsUnReadParam,
    ) -> Result<pb::DirectMarkAsUnReadResponse, GenErr> {
        Ok(pb::DirectMarkAsUnReadResponse::default())
    }
    async fn DirectPinDirects(
        up: &UserParam,
        param: pb::DirectPinDirectsParam,
    ) -> Result<pb::DirectPinDirectsResponse, GenErr> {
        Ok(pb::DirectPinDirectsResponse::default())
    }
    async fn DirectUnPinDirects(
        up: &UserParam,
        param: pb::DirectUnPinDirectsParam,
    ) -> Result<pb::DirectUnPinDirectsResponse, GenErr> {
        Ok(pb::DirectUnPinDirectsResponse::default())
    }
    async fn DirectArchiveDirects(
        up: &UserParam,
        param: pb::DirectArchiveDirectsParam,
    ) -> Result<pb::DirectArchiveDirectsResponse, GenErr> {
        Ok(pb::DirectArchiveDirectsResponse::default())
    }
    async fn DirectUnArchiveDirects(
        up: &UserParam,
        param: pb::DirectUnArchiveDirectsParam,
    ) -> Result<pb::DirectUnArchiveDirectsResponse, GenErr> {
        Ok(pb::DirectUnArchiveDirectsResponse::default())
    }
    async fn DirectClearHistories(
        up: &UserParam,
        param: pb::DirectClearHistoriesParam,
    ) -> Result<pb::DirectClearHistoriesResponse, GenErr> {
        Ok(pb::DirectClearHistoriesResponse::default())
    }
    async fn DirectMuteDirects(
        up: &UserParam,
        param: pb::DirectMuteDirectsParam,
    ) -> Result<pb::DirectMuteDirectsResponse, GenErr> {
        Ok(pb::DirectMuteDirectsResponse::default())
    }
    async fn DirectUnMuteDirects(
        up: &UserParam,
        param: pb::DirectUnMuteDirectsParam,
    ) -> Result<pb::DirectUnMuteDirectsResponse, GenErr> {
        Ok(pb::DirectUnMuteDirectsResponse::default())
    }
    async fn DirectCreateFolder(
        up: &UserParam,
        param: pb::DirectCreateFolderParam,
    ) -> Result<pb::DirectCreateFolderResponse, GenErr> {
        Ok(pb::DirectCreateFolderResponse::default())
    }
    async fn DirectChangeFolder(
        up: &UserParam,
        param: pb::DirectChangeFolderParam,
    ) -> Result<pb::DirectChangeFolderResponse, GenErr> {
        Ok(pb::DirectChangeFolderResponse::default())
    }
    async fn DirectRemoveFromFolder(
        up: &UserParam,
        param: pb::DirectRemoveFromFolderParam,
    ) -> Result<pb::DirectRemoveFromFolderResponse, GenErr> {
        Ok(pb::DirectRemoveFromFolderResponse::default())
    }
    async fn DirectReordersFolder(
        up: &UserParam,
        param: pb::DirectReordersFolderParam,
    ) -> Result<pb::DirectReordersFolderResponse, GenErr> {
        Ok(pb::DirectReordersFolderResponse::default())
    }
    async fn DirectDeleteFolder(
        up: &UserParam,
        param: pb::DirectDeleteFolderParam,
    ) -> Result<pb::DirectDeleteFolderResponse, GenErr> {
        Ok(pb::DirectDeleteFolderResponse::default())
    }
    async fn DirectGetChatsList(
        up: &UserParam,
        param: pb::DirectGetChatsListParam,
    ) -> Result<pb::DirectGetChatsListResponse, GenErr> {
        Ok(pb::DirectGetChatsListResponse::default())
    }
    async fn DirectGetGroupsList(
        up: &UserParam,
        param: pb::DirectGetGroupsListParam,
    ) -> Result<pb::DirectGetGroupsListResponse, GenErr> {
        Ok(pb::DirectGetGroupsListResponse::default())
    }
    async fn DirectGetChannelsList(
        up: &UserParam,
        param: pb::DirectGetChannelsListParam,
    ) -> Result<pb::DirectGetChannelsListResponse, GenErr> {
        Ok(pb::DirectGetChannelsListResponse::default())
    }
    async fn DirectGetFoldersList(
        up: &UserParam,
        param: pb::DirectGetFoldersListParam,
    ) -> Result<pb::DirectGetFoldersListResponse, GenErr> {
        Ok(pb::DirectGetFoldersListResponse::default())
    }
    async fn DirectGetFoldersFullList(
        up: &UserParam,
        param: pb::DirectGetFoldersFullListParam,
    ) -> Result<pb::DirectGetFoldersFullListResponse, GenErr> {
        Ok(pb::DirectGetFoldersFullListResponse::default())
    }
}
#[async_trait]
pub trait RPC_Group_Handler {
    async fn GroupCreateGroup(
        up: &UserParam,
        param: pb::GroupCreateGroupParam,
    ) -> Result<pb::GroupCreateGroupResponse, GenErr> {
        Ok(pb::GroupCreateGroupResponse::default())
    }
    async fn GroupEditGroup(
        up: &UserParam,
        param: pb::GroupEditGroupParam,
    ) -> Result<pb::GroupEditGroupResponse, GenErr> {
        Ok(pb::GroupEditGroupResponse::default())
    }
    async fn GroupDeleteGroup(
        up: &UserParam,
        param: pb::GroupDeleteGroupParam,
    ) -> Result<pb::GroupDeleteGroupResponse, GenErr> {
        Ok(pb::GroupDeleteGroupResponse::default())
    }
    async fn GroupAddAdmin(
        up: &UserParam,
        param: pb::GroupAddAdminParam,
    ) -> Result<pb::GroupAddAdminResponse, GenErr> {
        Ok(pb::GroupAddAdminResponse::default())
    }
    async fn GroupAddMember(
        up: &UserParam,
        param: pb::GroupAddMemberParam,
    ) -> Result<pb::GroupAddMemberResponse, GenErr> {
        Ok(pb::GroupAddMemberResponse::default())
    }
    async fn GroupRemoveMember(
        up: &UserParam,
        param: pb::GroupRemoveMemberParam,
    ) -> Result<pb::GroupRemoveMemberResponse, GenErr> {
        Ok(pb::GroupRemoveMemberResponse::default())
    }
    async fn GroupChangeMemberLevel(
        up: &UserParam,
        param: pb::GroupChangeMemberLevelParam,
    ) -> Result<pb::GroupChangeMemberLevelResponse, GenErr> {
        Ok(pb::GroupChangeMemberLevelResponse::default())
    }
    async fn GroupChangeMemberPermission(
        up: &UserParam,
        param: pb::GroupChangeMemberPermissionParam,
    ) -> Result<pb::GroupChangeMemberPermissionResponse, GenErr> {
        Ok(pb::GroupChangeMemberPermissionResponse::default())
    }
    async fn GroupJoinGroup(
        up: &UserParam,
        param: pb::JoinGroupParam,
    ) -> Result<pb::JoinGroupResponse, GenErr> {
        Ok(pb::JoinGroupResponse::default())
    }
    async fn GroupLeaveGroup(
        up: &UserParam,
        param: pb::GroupLeaveGroupParam,
    ) -> Result<pb::GroupLeaveGroupResponse, GenErr> {
        Ok(pb::GroupLeaveGroupResponse::default())
    }
    async fn GroupBanMember(
        up: &UserParam,
        param: pb::GroupBanMemberParam,
    ) -> Result<pb::GroupBanMemberResponse, GenErr> {
        Ok(pb::GroupBanMemberResponse::default())
    }
    async fn GroupChangePrivacy(
        up: &UserParam,
        param: pb::GroupChangePrivacyParam,
    ) -> Result<pb::GroupChangePrivacyResponse, GenErr> {
        Ok(pb::GroupChangePrivacyResponse::default())
    }
    async fn GroupChangeDefaultPermission(
        up: &UserParam,
        param: pb::GroupChangeDefaultPermissionParam,
    ) -> Result<pb::GroupChangeDefaultPermissionResponse, GenErr> {
        Ok(pb::GroupChangeDefaultPermissionResponse::default())
    }
    async fn GroupRevokeLink(
        up: &UserParam,
        param: pb::GroupRevokeLinkParam,
    ) -> Result<pb::GroupRevokeLinkResponse, GenErr> {
        Ok(pb::GroupRevokeLinkResponse::default())
    }
    async fn GroupChangeUsername(
        up: &UserParam,
        param: pb::GroupChangeUsernameParam,
    ) -> Result<pb::GroupChangeUsernameResponse, GenErr> {
        Ok(pb::GroupChangeUsernameResponse::default())
    }
    async fn GroupSendMessage(
        up: &UserParam,
        param: pb::GroupSendMessageParam,
    ) -> Result<pb::GroupSendMessageResponse, GenErr> {
        Ok(pb::GroupSendMessageResponse::default())
    }
    async fn GroupEditMessage(
        up: &UserParam,
        param: pb::GroupEditMessageParam,
    ) -> Result<pb::GroupEditMessageResponse, GenErr> {
        Ok(pb::GroupEditMessageResponse::default())
    }
    async fn GroupPinMessage(
        up: &UserParam,
        param: pb::GroupPinMessageParam,
    ) -> Result<pb::GroupPinMessageResponse, GenErr> {
        Ok(pb::GroupPinMessageResponse::default())
    }
    async fn GroupUnPinMessage(
        up: &UserParam,
        param: pb::GroupUnPinMessageParam,
    ) -> Result<pb::GroupUnPinMessageResponse, GenErr> {
        Ok(pb::GroupUnPinMessageResponse::default())
    }
    async fn GroupDeleteMessage(
        up: &UserParam,
        param: pb::GroupDeleteMessageParam,
    ) -> Result<pb::GroupDeleteMessageResponse, GenErr> {
        Ok(pb::GroupDeleteMessageResponse::default())
    }
    async fn GroupDeleteMessages(
        up: &UserParam,
        param: pb::GroupDeleteMessagesParam,
    ) -> Result<pb::GroupDeleteMessagesResponse, GenErr> {
        Ok(pb::GroupDeleteMessagesResponse::default())
    }
    async fn GroupDeleteHistory(
        up: &UserParam,
        param: pb::GroupDeleteHistoryParam,
    ) -> Result<pb::GroupDeleteHistoryResponse, GenErr> {
        Ok(pb::GroupDeleteHistoryResponse::default())
    }
    async fn GroupClearHistory(
        up: &UserParam,
        param: pb::GroupClearHistoryParam,
    ) -> Result<pb::GroupClearHistoryResponse, GenErr> {
        Ok(pb::GroupClearHistoryResponse::default())
    }
    async fn GroupAvatarAdd(
        up: &UserParam,
        param: pb::GroupAvatarAddParam,
    ) -> Result<pb::GroupAvatarAddResponse, GenErr> {
        Ok(pb::GroupAvatarAddResponse::default())
    }
    async fn GroupAvatarChange(
        up: &UserParam,
        param: pb::GroupAvatarChangeParam,
    ) -> Result<pb::GroupAvatarChangeResponse, GenErr> {
        Ok(pb::GroupAvatarChangeResponse::default())
    }
    async fn GroupAvatarDelete(
        up: &UserParam,
        param: pb::GroupAvatarDeleteParam,
    ) -> Result<pb::GroupAvatarDeleteResponse, GenErr> {
        Ok(pb::GroupAvatarDeleteResponse::default())
    }
    async fn GroupAvatarGetList(
        up: &UserParam,
        param: pb::GroupAvatarGetListParam,
    ) -> Result<pb::GroupAvatarGetListResponse, GenErr> {
        Ok(pb::GroupAvatarGetListResponse::default())
    }
    async fn GroupSendDoingAction(
        up: &UserParam,
        param: pb::GroupSendDoingActionParam,
    ) -> Result<pb::GroupSendDoingActionResponse, GenErr> {
        Ok(pb::GroupSendDoingActionResponse::default())
    }
    async fn GroupReportGroup(
        up: &UserParam,
        param: pb::GroupReportGroupParam,
    ) -> Result<pb::GroupReportGroupResponse, GenErr> {
        Ok(pb::GroupReportGroupResponse::default())
    }
    async fn GroupGetFull(
        up: &UserParam,
        param: pb::GroupGetFullMessageParam,
    ) -> Result<pb::GroupGetFullMessageResponse, GenErr> {
        Ok(pb::GroupGetFullMessageResponse::default())
    }
    async fn GroupGetMessagesList(
        up: &UserParam,
        param: pb::GroupGetMessagesListParam,
    ) -> Result<pb::GroupGetMessagesListResponse, GenErr> {
        Ok(pb::GroupGetMessagesListResponse::default())
    }
    async fn GroupGetMediaList(
        up: &UserParam,
        param: pb::GroupGetMediaListParam,
    ) -> Result<pb::GroupGetMediaListResponse, GenErr> {
        Ok(pb::GroupGetMediaListResponse::default())
    }
    async fn GroupGetMembersList(
        up: &UserParam,
        param: pb::GroupGetMembersListParam,
    ) -> Result<pb::GroupGetMembersListResponse, GenErr> {
        Ok(pb::GroupGetMembersListResponse::default())
    }
    async fn GroupGetAdminsList(
        up: &UserParam,
        param: pb::GroupGetAdminsListParam,
    ) -> Result<pb::GroupGetAdminsListResponse, GenErr> {
        Ok(pb::GroupGetAdminsListResponse::default())
    }
    async fn GroupSetDraft(
        up: &UserParam,
        param: pb::GroupSetDraftParam,
    ) -> Result<pb::GroupSetDraftResponse, GenErr> {
        Ok(pb::GroupSetDraftResponse::default())
    }
}
#[async_trait]
pub trait RPC_Sample_Handler {
    async fn GetUsers1(
        up: &UserParam,
        param: pb::GetUsers1Param,
    ) -> Result<pb::GetUsers1Response, GenErr> {
        Ok(pb::GetUsers1Response::default())
    }
    async fn GetProfiles(
        up: &UserParam,
        param: pb::GetProfilesParam,
    ) -> Result<pb::GetProfilesResponse, GenErr> {
        Ok(pb::GetProfilesResponse::default())
    }
    async fn GetChannels(
        up: &UserParam,
        param: pb::GetChannelsParam,
    ) -> Result<pb::GetChannelsResponse, GenErr> {
        Ok(pb::GetChannelsResponse::default())
    }
    async fn GetDirects(
        up: &UserParam,
        param: pb::GetDirectsParam,
    ) -> Result<pb::GetDirectsResponse, GenErr> {
        Ok(pb::GetDirectsResponse::default())
    }
    async fn GetMessages(
        up: &UserParam,
        param: pb::GetMessagesParam,
    ) -> Result<pb::GetMessagesResponse, GenErr> {
        Ok(pb::GetMessagesResponse::default())
    }
}
#[async_trait]
pub trait RPC_Shared_Handler {
    async fn Echo(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
    async fn CheckUserName(
        up: &UserParam,
        param: pb::CheckUserNameParam,
    ) -> Result<pb::CheckUserNameResponse, GenErr> {
        Ok(pb::CheckUserNameResponse::default())
    }
}
#[async_trait]
pub trait RPC_Upload_Handler {
    async fn UploadFile(
        up: &UserParam,
        param: pb::UploadFileParam,
    ) -> Result<pb::UploadFileResponse, GenErr> {
        Ok(pb::UploadFileResponse::default())
    }
}
#[async_trait]
pub trait RPC_User_Handler {
    async fn ChangePhoneNumber(
        up: &UserParam,
        param: pb::ChangePhoneNumberParam,
    ) -> Result<pb::ChangePhoneNumberResponse, GenErr> {
        Ok(pb::ChangePhoneNumberResponse::default())
    }
}

#[async_trait]
pub trait RPC_Auth_Handler2: Send + Sync {
    async fn SendConfirmCode(
        &self,
        param: pb::SendConfirmCodeParam,
    ) -> Result<pb::SendConfirmCodeResponse, GenErr> {
        Ok(pb::SendConfirmCodeResponse::default())
    }
    async fn ConfirmCode(
        &self,
        param: pb::ConfirmCodeParam,
    ) -> Result<pb::ConfirmCodeResponse, GenErr> {
        Ok(pb::ConfirmCodeResponse::default())
    }
    async fn SingUp(&self, param: pb::SingUpParam) -> Result<pb::SingUpResponse, GenErr> {
        Ok(pb::SingUpResponse::default())
    }
    async fn SingIn(&self, param: pb::SingInParam) -> Result<pb::SingInResponse, GenErr> {
        Ok(pb::SingInResponse::default())
    }
    async fn LogOut(&self, param: pb::LogOutParam) -> Result<pb::LogOutResponse, GenErr> {
        Ok(pb::LogOutResponse::default())
    }
}
#[async_trait]
pub trait RPC_Channel_Handler2: Send + Sync {
    async fn ChannelCreateChannel(
        &self,
        param: pb::ChannelCreateChannelParam,
    ) -> Result<pb::ChannelCreateChannelResponse, GenErr> {
        Ok(pb::ChannelCreateChannelResponse::default())
    }
    async fn ChannelEditChannel(
        &self,
        param: pb::ChannelEditChannelParam,
    ) -> Result<pb::ChannelEditChannelResponse, GenErr> {
        Ok(pb::ChannelEditChannelResponse::default())
    }
    async fn ChannelDeleteChannel(
        &self,
        param: pb::ChannelDeleteChannelParam,
    ) -> Result<pb::ChannelDeleteChannelResponse, GenErr> {
        Ok(pb::ChannelDeleteChannelResponse::default())
    }
    async fn ChannelAddAuthor(
        &self,
        param: pb::ChannelAddAuthorParam,
    ) -> Result<pb::ChannelAddAuthorResponse, GenErr> {
        Ok(pb::ChannelAddAuthorResponse::default())
    }
    async fn ChannelChangeAuthorPermission(
        &self,
        param: pb::ChannelChangeAuthorPermissionParam,
    ) -> Result<pb::ChannelChangeAuthorPermissionResponse, GenErr> {
        Ok(pb::ChannelChangeAuthorPermissionResponse::default())
    }
    async fn ChannelRemoveAuthor(
        &self,
        param: pb::ChannelRemoveAuthorParam,
    ) -> Result<pb::ChannelRemoveAuthorResponse, GenErr> {
        Ok(pb::ChannelRemoveAuthorResponse::default())
    }
    async fn ChannelFollowChannel(
        &self,
        param: pb::ChannelFollowChannelParam,
    ) -> Result<pb::ChannelFollowChannelResponse, GenErr> {
        Ok(pb::ChannelFollowChannelResponse::default())
    }
    async fn ChannelUnFollowChannel(
        &self,
        param: pb::ChannelUnFollowChannelParam,
    ) -> Result<pb::ChannelUnFollowChannelResponse, GenErr> {
        Ok(pb::ChannelUnFollowChannelResponse::default())
    }
    async fn ChannelRemoveFollowers(
        &self,
        param: pb::ChannelRemoveFollowersParam,
    ) -> Result<pb::ChannelRemoveFollowersResponse, GenErr> {
        Ok(pb::ChannelRemoveFollowersResponse::default())
    }
    async fn ChannelSubscribe(
        &self,
        param: pb::ChannelSubscribeParam,
    ) -> Result<pb::ChannelSubscribeResponse, GenErr> {
        Ok(pb::ChannelSubscribeResponse::default())
    }
    async fn ChannelUnSubscribe(
        &self,
        param: pb::ChannelUnSubscribeParam,
    ) -> Result<pb::ChannelUnSubscribeResponse, GenErr> {
        Ok(pb::ChannelUnSubscribeResponse::default())
    }
    async fn ChannelRemoveSubscribers(
        &self,
        param: pb::ChannelRemoveSubscribersParam,
    ) -> Result<pb::ChannelRemoveSubscribersResponse, GenErr> {
        Ok(pb::ChannelRemoveSubscribersResponse::default())
    }
    async fn ChannelChangePrivacy(
        &self,
        param: pb::ChannelChangePrivacyParam,
    ) -> Result<pb::ChannelChangePrivacyResponse, GenErr> {
        Ok(pb::ChannelChangePrivacyResponse::default())
    }
    async fn ChannelChangeDefaultPermission(
        &self,
        param: pb::ChannelChangeDefaultPermissionParam,
    ) -> Result<pb::ChannelChangeDefaultPermissionResponse, GenErr> {
        Ok(pb::ChannelChangeDefaultPermissionResponse::default())
    }
    async fn ChannelRevokeLink(
        &self,
        param: pb::ChannelRevokeLinkParam,
    ) -> Result<pb::ChannelRevokeLinkResponse, GenErr> {
        Ok(pb::ChannelRevokeLinkResponse::default())
    }
    async fn ChannelChangeUsername(
        &self,
        param: pb::ChannelChangeUsernameParam,
    ) -> Result<pb::ChannelChangeUsernameResponse, GenErr> {
        Ok(pb::ChannelChangeUsernameResponse::default())
    }
    async fn ChannelBlockChannel(
        &self,
        param: pb::ChannelBlockChannelParam,
    ) -> Result<pb::ChannelBlockChannelResponse, GenErr> {
        Ok(pb::ChannelBlockChannelResponse::default())
    }
    async fn ChannelSendMessage(
        &self,
        param: pb::ChannelSendMessageParam,
    ) -> Result<pb::ChannelSendMessageResponse, GenErr> {
        Ok(pb::ChannelSendMessageResponse::default())
    }
    async fn ChannelEditMessage(
        &self,
        param: pb::ChannelEditMessageParam,
    ) -> Result<pb::ChannelEditMessageResponse, GenErr> {
        Ok(pb::ChannelEditMessageResponse::default())
    }
    async fn ChannelPinMessage(
        &self,
        param: pb::ChannelPinMessageParam,
    ) -> Result<pb::ChannelPinMessageResponse, GenErr> {
        Ok(pb::ChannelPinMessageResponse::default())
    }
    async fn ChannelUnPinMessage(
        &self,
        param: pb::ChannelUnPinMessageParam,
    ) -> Result<pb::ChannelUnPinMessageResponse, GenErr> {
        Ok(pb::ChannelUnPinMessageResponse::default())
    }
    async fn ChannelDeleteMessage(
        &self,
        param: pb::ChannelDeleteMessageParam,
    ) -> Result<pb::ChannelDeleteMessageResponse, GenErr> {
        Ok(pb::ChannelDeleteMessageResponse::default())
    }
    async fn ChannelDeleteMessages(
        &self,
        param: pb::ChannelDeleteMessagesParam,
    ) -> Result<pb::ChannelDeleteMessagesResponse, GenErr> {
        Ok(pb::ChannelDeleteMessagesResponse::default())
    }
    async fn ChannelClearHistory(
        &self,
        param: pb::ChannelClearHistoryParam,
    ) -> Result<pb::ChannelClearHistoryResponse, GenErr> {
        Ok(pb::ChannelClearHistoryResponse::default())
    }
    async fn ChannelAvatarAdd(
        &self,
        param: pb::ChannelAvatarAddParam,
    ) -> Result<pb::ChannelAvatarAddResponse, GenErr> {
        Ok(pb::ChannelAvatarAddResponse::default())
    }
    async fn ChannelAvatarChange(
        &self,
        param: pb::ChannelAvatarChangeParam,
    ) -> Result<pb::ChannelAvatarChangeResponse, GenErr> {
        Ok(pb::ChannelAvatarChangeResponse::default())
    }
    async fn ChannelAvatarDelete(
        &self,
        param: pb::ChannelAvatarDeleteParam,
    ) -> Result<pb::ChannelAvatarDeleteResponse, GenErr> {
        Ok(pb::ChannelAvatarDeleteResponse::default())
    }
    async fn ChannelAvatarGetList(
        &self,
        param: pb::ChannelAvatarGetListParam,
    ) -> Result<pb::ChannelAvatarGetListResponse, GenErr> {
        Ok(pb::ChannelAvatarGetListResponse::default())
    }
    async fn ChannelSendDoingAction(
        &self,
        param: pb::ChannelSendDoingActionParam,
    ) -> Result<pb::ChannelSendDoingActionResponse, GenErr> {
        Ok(pb::ChannelSendDoingActionResponse::default())
    }
    async fn ChannelReportChannel(
        &self,
        param: pb::ChannelReportChannelParam,
    ) -> Result<pb::ChannelReportChannelResponse, GenErr> {
        Ok(pb::ChannelReportChannelResponse::default())
    }
    async fn ChannelReportMessage(
        &self,
        param: pb::ChannelReportMessageParam,
    ) -> Result<pb::ChannelReportMessageResponse, GenErr> {
        Ok(pb::ChannelReportMessageResponse::default())
    }
    async fn ChannelGetFull(
        &self,
        param: pb::ChannelGetFullParam,
    ) -> Result<pb::ChannelGetFullResponse, GenErr> {
        Ok(pb::ChannelGetFullResponse::default())
    }
    async fn ChannelGetMessagesList(
        &self,
        param: pb::ChannelGetMessagesListParam,
    ) -> Result<pb::ChannelGetMessagesListResponse, GenErr> {
        Ok(pb::ChannelGetMessagesListResponse::default())
    }
    async fn ChannelGetMediaList(
        &self,
        param: pb::ChannelGetMediaListParam,
    ) -> Result<pb::ChannelGetMediaListResponse, GenErr> {
        Ok(pb::ChannelGetMediaListResponse::default())
    }
    async fn ChannelGetAuthors(
        &self,
        param: pb::ChannelGetAuthorsParam,
    ) -> Result<pb::ChannelGetAuthorsResponse, GenErr> {
        Ok(pb::ChannelGetAuthorsResponse::default())
    }
    async fn ChannelGetFollowers(
        &self,
        param: pb::ChannelGetFollowersParam,
    ) -> Result<pb::ChannelGetFollowersResponse, GenErr> {
        Ok(pb::ChannelGetFollowersResponse::default())
    }
    async fn ChannelGetFollowings(
        &self,
        param: pb::ChannelGetFollowingsParam,
    ) -> Result<pb::ChannelGetFollowingsResponse, GenErr> {
        Ok(pb::ChannelGetFollowingsResponse::default())
    }
    async fn ChannelGetSubscribers(
        &self,
        param: pb::ChannelGetSubscribersParam,
    ) -> Result<pb::ChannelGetSubscribersResponse, GenErr> {
        Ok(pb::ChannelGetSubscribersResponse::default())
    }
    async fn ChannelBlocked(
        &self,
        param: pb::ChannelBlockedParam,
    ) -> Result<pb::ChannelBlockedResponse, GenErr> {
        Ok(pb::ChannelBlockedResponse::default())
    }
    async fn ChannelSetDraft(
        &self,
        param: pb::ChannelSetDraftParam,
    ) -> Result<pb::ChannelSetDraftResponse, GenErr> {
        Ok(pb::ChannelSetDraftResponse::default())
    }
}
#[async_trait]
pub trait RPC_Chat_Handler2: Send + Sync {
    async fn ChatSendMessage(
        &self,
        param: pb::ChatSendMessageParam,
    ) -> Result<pb::ChatSendMessageResponse, GenErr> {
        Ok(pb::ChatSendMessageResponse::default())
    }
    async fn ChatEditMessage(
        &self,
        param: pb::ChatEditMessageParam,
    ) -> Result<pb::ChatEditMessageResponse, GenErr> {
        Ok(pb::ChatEditMessageResponse::default())
    }
    async fn ChatDeleteMessages(
        &self,
        param: pb::ChatDeleteMessagesParam,
    ) -> Result<pb::ChatDeleteMessagesResponse, GenErr> {
        Ok(pb::ChatDeleteMessagesResponse::default())
    }
    async fn ChatDeleteHistory(
        &self,
        param: pb::ChatDeleteHistoryParam,
    ) -> Result<pb::ChatDeleteHistoryResponse, GenErr> {
        Ok(pb::ChatDeleteHistoryResponse::default())
    }
    async fn ChatSendDoingAction(
        &self,
        param: pb::ChatSendDoingActionParam,
    ) -> Result<pb::ChatSendDoingActionResponse, GenErr> {
        Ok(pb::ChatSendDoingActionResponse::default())
    }
    async fn ChatReportChat(
        &self,
        param: pb::ChatReportChatParam,
    ) -> Result<pb::ChatReportChatResponse, GenErr> {
        Ok(pb::ChatReportChatResponse::default())
    }
    async fn ChatGetFull(
        &self,
        param: pb::ChatGetFullMessageParam,
    ) -> Result<pb::ChatGetFullMessageResponse, GenErr> {
        Ok(pb::ChatGetFullMessageResponse::default())
    }
    async fn ChatGetMessagesList(
        &self,
        param: pb::ChatGetMessagesListParam,
    ) -> Result<pb::ChatGetMessagesListResponse, GenErr> {
        Ok(pb::ChatGetMessagesListResponse::default())
    }
    async fn ChatGetMediaList(
        &self,
        param: pb::ChatGetMediaListParam,
    ) -> Result<pb::ChatGetMediaListResponse, GenErr> {
        Ok(pb::ChatGetMediaListResponse::default())
    }
}
#[async_trait]
pub trait RPC_Direct_Handler2: Send + Sync {
    async fn DirectDeleteDirect(
        &self,
        param: pb::DirectDeleteDirectParam,
    ) -> Result<pb::DirectDeleteDirectResponse, GenErr> {
        Ok(pb::DirectDeleteDirectResponse::default())
    }
    async fn DirectChangeTitle(
        &self,
        param: pb::DirectChangeTitleParam,
    ) -> Result<pb::DirectChangeTitleResponse, GenErr> {
        Ok(pb::DirectChangeTitleResponse::default())
    }
    async fn DirectSetCustomNotification(
        &self,
        param: pb::DirectSetCustomNotificationParam,
    ) -> Result<pb::DirectSetCustomNotificationResponse, GenErr> {
        Ok(pb::DirectSetCustomNotificationResponse::default())
    }
    async fn DirectSendActionDoing(
        &self,
        param: pb::DirectSendActionDoingParam,
    ) -> Result<pb::DirectSendActionDoingResponse, GenErr> {
        Ok(pb::DirectSendActionDoingResponse::default())
    }
    async fn DirectSetDraft(
        &self,
        param: pb::DirectSetDraftParam,
    ) -> Result<pb::DirectSetDraftResponse, GenErr> {
        Ok(pb::DirectSetDraftResponse::default())
    }
    async fn DirectDeleteDirects(
        &self,
        param: pb::DirectDeleteDirectsParam,
    ) -> Result<pb::DirectDeleteDirectsResponse, GenErr> {
        Ok(pb::DirectDeleteDirectsResponse::default())
    }
    async fn DirectMarkAsRead(
        &self,
        param: pb::DirectMarkAsReadParam,
    ) -> Result<pb::DirectMarkAsReadResponse, GenErr> {
        Ok(pb::DirectMarkAsReadResponse::default())
    }
    async fn DirectMarkAsUnRead(
        &self,
        param: pb::DirectMarkAsUnReadParam,
    ) -> Result<pb::DirectMarkAsUnReadResponse, GenErr> {
        Ok(pb::DirectMarkAsUnReadResponse::default())
    }
    async fn DirectPinDirects(
        &self,
        param: pb::DirectPinDirectsParam,
    ) -> Result<pb::DirectPinDirectsResponse, GenErr> {
        Ok(pb::DirectPinDirectsResponse::default())
    }
    async fn DirectUnPinDirects(
        &self,
        param: pb::DirectUnPinDirectsParam,
    ) -> Result<pb::DirectUnPinDirectsResponse, GenErr> {
        Ok(pb::DirectUnPinDirectsResponse::default())
    }
    async fn DirectArchiveDirects(
        &self,
        param: pb::DirectArchiveDirectsParam,
    ) -> Result<pb::DirectArchiveDirectsResponse, GenErr> {
        Ok(pb::DirectArchiveDirectsResponse::default())
    }
    async fn DirectUnArchiveDirects(
        &self,
        param: pb::DirectUnArchiveDirectsParam,
    ) -> Result<pb::DirectUnArchiveDirectsResponse, GenErr> {
        Ok(pb::DirectUnArchiveDirectsResponse::default())
    }
    async fn DirectClearHistories(
        &self,
        param: pb::DirectClearHistoriesParam,
    ) -> Result<pb::DirectClearHistoriesResponse, GenErr> {
        Ok(pb::DirectClearHistoriesResponse::default())
    }
    async fn DirectMuteDirects(
        &self,
        param: pb::DirectMuteDirectsParam,
    ) -> Result<pb::DirectMuteDirectsResponse, GenErr> {
        Ok(pb::DirectMuteDirectsResponse::default())
    }
    async fn DirectUnMuteDirects(
        &self,
        param: pb::DirectUnMuteDirectsParam,
    ) -> Result<pb::DirectUnMuteDirectsResponse, GenErr> {
        Ok(pb::DirectUnMuteDirectsResponse::default())
    }
    async fn DirectCreateFolder(
        &self,
        param: pb::DirectCreateFolderParam,
    ) -> Result<pb::DirectCreateFolderResponse, GenErr> {
        Ok(pb::DirectCreateFolderResponse::default())
    }
    async fn DirectChangeFolder(
        &self,
        param: pb::DirectChangeFolderParam,
    ) -> Result<pb::DirectChangeFolderResponse, GenErr> {
        Ok(pb::DirectChangeFolderResponse::default())
    }
    async fn DirectRemoveFromFolder(
        &self,
        param: pb::DirectRemoveFromFolderParam,
    ) -> Result<pb::DirectRemoveFromFolderResponse, GenErr> {
        Ok(pb::DirectRemoveFromFolderResponse::default())
    }
    async fn DirectReordersFolder(
        &self,
        param: pb::DirectReordersFolderParam,
    ) -> Result<pb::DirectReordersFolderResponse, GenErr> {
        Ok(pb::DirectReordersFolderResponse::default())
    }
    async fn DirectDeleteFolder(
        &self,
        param: pb::DirectDeleteFolderParam,
    ) -> Result<pb::DirectDeleteFolderResponse, GenErr> {
        Ok(pb::DirectDeleteFolderResponse::default())
    }
    async fn DirectGetChatsList(
        &self,
        param: pb::DirectGetChatsListParam,
    ) -> Result<pb::DirectGetChatsListResponse, GenErr> {
        Ok(pb::DirectGetChatsListResponse::default())
    }
    async fn DirectGetGroupsList(
        &self,
        param: pb::DirectGetGroupsListParam,
    ) -> Result<pb::DirectGetGroupsListResponse, GenErr> {
        Ok(pb::DirectGetGroupsListResponse::default())
    }
    async fn DirectGetChannelsList(
        &self,
        param: pb::DirectGetChannelsListParam,
    ) -> Result<pb::DirectGetChannelsListResponse, GenErr> {
        Ok(pb::DirectGetChannelsListResponse::default())
    }
    async fn DirectGetFoldersList(
        &self,
        param: pb::DirectGetFoldersListParam,
    ) -> Result<pb::DirectGetFoldersListResponse, GenErr> {
        Ok(pb::DirectGetFoldersListResponse::default())
    }
    async fn DirectGetFoldersFullList(
        &self,
        param: pb::DirectGetFoldersFullListParam,
    ) -> Result<pb::DirectGetFoldersFullListResponse, GenErr> {
        Ok(pb::DirectGetFoldersFullListResponse::default())
    }
}
#[async_trait]
pub trait RPC_Group_Handler2: Send + Sync {
    async fn GroupCreateGroup(
        &self,
        param: pb::GroupCreateGroupParam,
    ) -> Result<pb::GroupCreateGroupResponse, GenErr> {
        Ok(pb::GroupCreateGroupResponse::default())
    }
    async fn GroupEditGroup(
        &self,
        param: pb::GroupEditGroupParam,
    ) -> Result<pb::GroupEditGroupResponse, GenErr> {
        Ok(pb::GroupEditGroupResponse::default())
    }
    async fn GroupDeleteGroup(
        &self,
        param: pb::GroupDeleteGroupParam,
    ) -> Result<pb::GroupDeleteGroupResponse, GenErr> {
        Ok(pb::GroupDeleteGroupResponse::default())
    }
    async fn GroupAddAdmin(
        &self,
        param: pb::GroupAddAdminParam,
    ) -> Result<pb::GroupAddAdminResponse, GenErr> {
        Ok(pb::GroupAddAdminResponse::default())
    }
    async fn GroupAddMember(
        &self,
        param: pb::GroupAddMemberParam,
    ) -> Result<pb::GroupAddMemberResponse, GenErr> {
        Ok(pb::GroupAddMemberResponse::default())
    }
    async fn GroupRemoveMember(
        &self,
        param: pb::GroupRemoveMemberParam,
    ) -> Result<pb::GroupRemoveMemberResponse, GenErr> {
        Ok(pb::GroupRemoveMemberResponse::default())
    }
    async fn GroupChangeMemberLevel(
        &self,
        param: pb::GroupChangeMemberLevelParam,
    ) -> Result<pb::GroupChangeMemberLevelResponse, GenErr> {
        Ok(pb::GroupChangeMemberLevelResponse::default())
    }
    async fn GroupChangeMemberPermission(
        &self,
        param: pb::GroupChangeMemberPermissionParam,
    ) -> Result<pb::GroupChangeMemberPermissionResponse, GenErr> {
        Ok(pb::GroupChangeMemberPermissionResponse::default())
    }
    async fn GroupJoinGroup(
        &self,
        param: pb::JoinGroupParam,
    ) -> Result<pb::JoinGroupResponse, GenErr> {
        Ok(pb::JoinGroupResponse::default())
    }
    async fn GroupLeaveGroup(
        &self,
        param: pb::GroupLeaveGroupParam,
    ) -> Result<pb::GroupLeaveGroupResponse, GenErr> {
        Ok(pb::GroupLeaveGroupResponse::default())
    }
    async fn GroupBanMember(
        &self,
        param: pb::GroupBanMemberParam,
    ) -> Result<pb::GroupBanMemberResponse, GenErr> {
        Ok(pb::GroupBanMemberResponse::default())
    }
    async fn GroupChangePrivacy(
        &self,
        param: pb::GroupChangePrivacyParam,
    ) -> Result<pb::GroupChangePrivacyResponse, GenErr> {
        Ok(pb::GroupChangePrivacyResponse::default())
    }
    async fn GroupChangeDefaultPermission(
        &self,
        param: pb::GroupChangeDefaultPermissionParam,
    ) -> Result<pb::GroupChangeDefaultPermissionResponse, GenErr> {
        Ok(pb::GroupChangeDefaultPermissionResponse::default())
    }
    async fn GroupRevokeLink(
        &self,
        param: pb::GroupRevokeLinkParam,
    ) -> Result<pb::GroupRevokeLinkResponse, GenErr> {
        Ok(pb::GroupRevokeLinkResponse::default())
    }
    async fn GroupChangeUsername(
        &self,
        param: pb::GroupChangeUsernameParam,
    ) -> Result<pb::GroupChangeUsernameResponse, GenErr> {
        Ok(pb::GroupChangeUsernameResponse::default())
    }
    async fn GroupSendMessage(
        &self,
        param: pb::GroupSendMessageParam,
    ) -> Result<pb::GroupSendMessageResponse, GenErr> {
        Ok(pb::GroupSendMessageResponse::default())
    }
    async fn GroupEditMessage(
        &self,
        param: pb::GroupEditMessageParam,
    ) -> Result<pb::GroupEditMessageResponse, GenErr> {
        Ok(pb::GroupEditMessageResponse::default())
    }
    async fn GroupPinMessage(
        &self,
        param: pb::GroupPinMessageParam,
    ) -> Result<pb::GroupPinMessageResponse, GenErr> {
        Ok(pb::GroupPinMessageResponse::default())
    }
    async fn GroupUnPinMessage(
        &self,
        param: pb::GroupUnPinMessageParam,
    ) -> Result<pb::GroupUnPinMessageResponse, GenErr> {
        Ok(pb::GroupUnPinMessageResponse::default())
    }
    async fn GroupDeleteMessage(
        &self,
        param: pb::GroupDeleteMessageParam,
    ) -> Result<pb::GroupDeleteMessageResponse, GenErr> {
        Ok(pb::GroupDeleteMessageResponse::default())
    }
    async fn GroupDeleteMessages(
        &self,
        param: pb::GroupDeleteMessagesParam,
    ) -> Result<pb::GroupDeleteMessagesResponse, GenErr> {
        Ok(pb::GroupDeleteMessagesResponse::default())
    }
    async fn GroupDeleteHistory(
        &self,
        param: pb::GroupDeleteHistoryParam,
    ) -> Result<pb::GroupDeleteHistoryResponse, GenErr> {
        Ok(pb::GroupDeleteHistoryResponse::default())
    }
    async fn GroupClearHistory(
        &self,
        param: pb::GroupClearHistoryParam,
    ) -> Result<pb::GroupClearHistoryResponse, GenErr> {
        Ok(pb::GroupClearHistoryResponse::default())
    }
    async fn GroupAvatarAdd(
        &self,
        param: pb::GroupAvatarAddParam,
    ) -> Result<pb::GroupAvatarAddResponse, GenErr> {
        Ok(pb::GroupAvatarAddResponse::default())
    }
    async fn GroupAvatarChange(
        &self,
        param: pb::GroupAvatarChangeParam,
    ) -> Result<pb::GroupAvatarChangeResponse, GenErr> {
        Ok(pb::GroupAvatarChangeResponse::default())
    }
    async fn GroupAvatarDelete(
        &self,
        param: pb::GroupAvatarDeleteParam,
    ) -> Result<pb::GroupAvatarDeleteResponse, GenErr> {
        Ok(pb::GroupAvatarDeleteResponse::default())
    }
    async fn GroupAvatarGetList(
        &self,
        param: pb::GroupAvatarGetListParam,
    ) -> Result<pb::GroupAvatarGetListResponse, GenErr> {
        Ok(pb::GroupAvatarGetListResponse::default())
    }
    async fn GroupSendDoingAction(
        &self,
        param: pb::GroupSendDoingActionParam,
    ) -> Result<pb::GroupSendDoingActionResponse, GenErr> {
        Ok(pb::GroupSendDoingActionResponse::default())
    }
    async fn GroupReportGroup(
        &self,
        param: pb::GroupReportGroupParam,
    ) -> Result<pb::GroupReportGroupResponse, GenErr> {
        Ok(pb::GroupReportGroupResponse::default())
    }
    async fn GroupGetFull(
        &self,
        param: pb::GroupGetFullMessageParam,
    ) -> Result<pb::GroupGetFullMessageResponse, GenErr> {
        Ok(pb::GroupGetFullMessageResponse::default())
    }
    async fn GroupGetMessagesList(
        &self,
        param: pb::GroupGetMessagesListParam,
    ) -> Result<pb::GroupGetMessagesListResponse, GenErr> {
        Ok(pb::GroupGetMessagesListResponse::default())
    }
    async fn GroupGetMediaList(
        &self,
        param: pb::GroupGetMediaListParam,
    ) -> Result<pb::GroupGetMediaListResponse, GenErr> {
        Ok(pb::GroupGetMediaListResponse::default())
    }
    async fn GroupGetMembersList(
        &self,
        param: pb::GroupGetMembersListParam,
    ) -> Result<pb::GroupGetMembersListResponse, GenErr> {
        Ok(pb::GroupGetMembersListResponse::default())
    }
    async fn GroupGetAdminsList(
        &self,
        param: pb::GroupGetAdminsListParam,
    ) -> Result<pb::GroupGetAdminsListResponse, GenErr> {
        Ok(pb::GroupGetAdminsListResponse::default())
    }
    async fn GroupSetDraft(
        &self,
        param: pb::GroupSetDraftParam,
    ) -> Result<pb::GroupSetDraftResponse, GenErr> {
        Ok(pb::GroupSetDraftResponse::default())
    }
}
#[async_trait]
pub trait RPC_Sample_Handler2: Send + Sync {
    async fn GetUsers1(&self, param: pb::GetUsers1Param) -> Result<pb::GetUsers1Response, GenErr> {
        Ok(pb::GetUsers1Response::default())
    }
    async fn GetProfiles(
        &self,
        param: pb::GetProfilesParam,
    ) -> Result<pb::GetProfilesResponse, GenErr> {
        Ok(pb::GetProfilesResponse::default())
    }
    async fn GetChannels(
        &self,
        param: pb::GetChannelsParam,
    ) -> Result<pb::GetChannelsResponse, GenErr> {
        Ok(pb::GetChannelsResponse::default())
    }
    async fn GetDirects(
        &self,
        param: pb::GetDirectsParam,
    ) -> Result<pb::GetDirectsResponse, GenErr> {
        Ok(pb::GetDirectsResponse::default())
    }
    async fn GetMessages(
        &self,
        param: pb::GetMessagesParam,
    ) -> Result<pb::GetMessagesResponse, GenErr> {
        Ok(pb::GetMessagesResponse::default())
    }
}
#[async_trait]
pub trait RPC_Shared_Handler2: Send + Sync {
    async fn Echo(&self, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
    async fn CheckUserName(
        &self,
        param: pb::CheckUserNameParam,
    ) -> Result<pb::CheckUserNameResponse, GenErr> {
        Ok(pb::CheckUserNameResponse::default())
    }
}
#[async_trait]
pub trait RPC_Upload_Handler2: Send + Sync {
    async fn UploadFile(
        &self,
        param: pb::UploadFileParam,
    ) -> Result<pb::UploadFileResponse, GenErr> {
        Ok(pb::UploadFileResponse::default())
    }
}
#[async_trait]
pub trait RPC_User_Handler2: Send + Sync {
    async fn ChangePhoneNumber(
        &self,
        param: pb::ChangePhoneNumberParam,
    ) -> Result<pb::ChangePhoneNumberResponse, GenErr> {
        Ok(pb::ChangePhoneNumberResponse::default())
    }
}

#[async_trait]
pub trait All_Rpc_Handler:
    RPC_Auth_Handler2
    + RPC_Channel_Handler2
    + RPC_Chat_Handler2
    + RPC_Direct_Handler2
    + RPC_Group_Handler2
    + RPC_Sample_Handler2
    + RPC_Shared_Handler2
    + RPC_Upload_Handler2
    + RPC_User_Handler2
    + Clone
    + Send
    + Sync
{
}

pub mod method_ids {
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
    pub const GetProfiles: u32 = 822554282;
    pub const GetChannels: u32 = 1734748927;
    pub const GetDirects: u32 = 558085683;
    pub const GetMessages: u32 = 1160951872;

    // Service: RPC_Shared
    pub const Echo: u32 = 101973561;
    pub const CheckUserName: u32 = 1897027349;

    // Service: RPC_Upload
    pub const UploadFile: u32 = 1702285478;

    // Service: RPC_User
    pub const ChangePhoneNumber: u32 = 706069694;

    pub const ExampleChangePhoneNumber8: u32 = 79874;
}

#[derive(Debug)]
pub enum MethodIds {
    // Service: RPC_Auth
    SendConfirmCode = 939965206,
    ConfirmCode = 1740258084,
    SingUp = 291193302,
    SingIn = 1017957090,
    LogOut = 1283119009,

    // Service: RPC_Channel
    ChannelCreateChannel = 143251225,
    ChannelEditChannel = 189471894,
    ChannelDeleteChannel = 1494483355,
    ChannelAddAuthor = 780397316,
    ChannelChangeAuthorPermission = 93233821,
    ChannelRemoveAuthor = 419542304,
    ChannelFollowChannel = 744563779,
    ChannelUnFollowChannel = 959512423,
    ChannelRemoveFollowers = 869709257,
    ChannelSubscribe = 1367898912,
    ChannelUnSubscribe = 858172401,
    ChannelRemoveSubscribers = 729024592,
    ChannelChangePrivacy = 79012409,
    ChannelChangeDefaultPermission = 1582638498,
    ChannelRevokeLink = 1912530021,
    ChannelChangeUsername = 983884462,
    ChannelBlockChannel = 2037016989,
    ChannelSendMessage = 1200751231,
    ChannelEditMessage = 727437726,
    ChannelPinMessage = 259263709,
    ChannelUnPinMessage = 113943649,
    ChannelDeleteMessage = 644189206,
    ChannelDeleteMessages = 2124822181,
    ChannelClearHistory = 1164398815,
    ChannelAvatarAdd = 1021808696,
    ChannelAvatarChange = 1968579501,
    ChannelAvatarDelete = 1626010891,
    ChannelAvatarGetList = 1925044843,
    ChannelSendDoingAction = 973237257,
    ChannelReportChannel = 792938145,
    ChannelReportMessage = 2053528327,
    ChannelGetFull = 1684531258,
    ChannelGetMessagesList = 1339072968,
    ChannelGetMediaList = 985772653,
    ChannelGetAuthors = 1373284924,
    ChannelGetFollowers = 1747172143,
    ChannelGetFollowings = 1838438980,
    ChannelGetSubscribers = 2146806736,
    ChannelBlocked = 1674411747,
    ChannelSetDraft = 1403193015,

    // Service: RPC_Chat
    ChatSendMessage = 1131621475,
    ChatEditMessage = 1806258329,
    ChatDeleteMessages = 933526170,
    ChatDeleteHistory = 1088992782,
    ChatSendDoingAction = 1319324241,
    ChatReportChat = 1345425871,
    ChatGetFull = 1768678453,
    ChatGetMessagesList = 121549718,
    ChatGetMediaList = 1346774525,

    // Service: RPC_Direct
    DirectDeleteDirect = 1478067518,
    DirectChangeTitle = 2041790485,
    DirectSetCustomNotification = 548699291,
    DirectSendActionDoing = 1417285757,
    DirectSetDraft = 1860345925,
    DirectDeleteDirects = 1291891637,
    DirectMarkAsRead = 1801774787,
    DirectMarkAsUnRead = 313746334,
    DirectPinDirects = 1179089068,
    DirectUnPinDirects = 1517245560,
    DirectArchiveDirects = 1441782770,
    DirectUnArchiveDirects = 1951553867,
    DirectClearHistories = 904052140,
    DirectMuteDirects = 1138477048,
    DirectUnMuteDirects = 1691834263,
    DirectCreateFolder = 1878673022,
    DirectChangeFolder = 1861381591,
    DirectRemoveFromFolder = 1818954127,
    DirectReordersFolder = 1264591958,
    DirectDeleteFolder = 962281627,
    DirectGetChatsList = 1570934969,
    DirectGetGroupsList = 545957996,
    DirectGetChannelsList = 1608173619,
    DirectGetFoldersList = 1384523712,
    DirectGetFoldersFullList = 611850722,

    // Service: RPC_Group
    GroupCreateGroup = 1205960678,
    GroupEditGroup = 1665019493,
    GroupDeleteGroup = 365183375,
    GroupAddAdmin = 958971956,
    GroupAddMember = 676599227,
    GroupRemoveMember = 2012702964,
    GroupChangeMemberLevel = 589574238,
    GroupChangeMemberPermission = 2132464067,
    GroupJoinGroup = 591743429,
    GroupLeaveGroup = 361834630,
    GroupBanMember = 548504852,
    GroupChangePrivacy = 1497988410,
    GroupChangeDefaultPermission = 605792138,
    GroupRevokeLink = 406592509,
    GroupChangeUsername = 832997038,
    GroupSendMessage = 599852950,
    GroupEditMessage = 742937895,
    GroupPinMessage = 184560027,
    GroupUnPinMessage = 1290613173,
    GroupDeleteMessage = 393991035,
    GroupDeleteMessages = 276700675,
    GroupDeleteHistory = 1270953793,
    GroupClearHistory = 1352552449,
    GroupAvatarAdd = 1202058216,
    GroupAvatarChange = 108612523,
    GroupAvatarDelete = 775862697,
    GroupAvatarGetList = 939443722,
    GroupSendDoingAction = 2022474356,
    GroupReportGroup = 1759704420,
    GroupGetFull = 200351324,
    GroupGetMessagesList = 1541835459,
    GroupGetMediaList = 2143016912,
    GroupGetMembersList = 429215412,
    GroupGetAdminsList = 332260610,
    GroupSetDraft = 77668156,

    // Service: RPC_Sample
    GetUsers1 = 486248681,
    GetProfiles = 822554282,
    GetChannels = 1734748927,
    GetDirects = 558085683,
    GetMessages = 1160951872,

    // Service: RPC_Shared
    Echo = 101973561,
    CheckUserName = 1897027349,

    // Service: RPC_Upload
    UploadFile = 1702285478,

    // Service: RPC_User
    ChangePhoneNumber = 706069694,
}

pub fn invoke_to_parsed(invoke: &pb::Invoke) -> Result<RpcInvoke, GenErr> {
    use RpcServiceData::*;
    let rpc = match invoke.method {
        // RPC_Auth
        method_ids::SendConfirmCode => {
            let rpc_param: pb::SendConfirmCodeParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 939965206 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::SendConfirmCode(rpc_param)),
            }
        }

        method_ids::ConfirmCode => {
            let rpc_param: pb::ConfirmCodeParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1740258084 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::ConfirmCode(rpc_param)),
            }
        }

        method_ids::SingUp => {
            let rpc_param: pb::SingUpParam = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 291193302 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::SingUp(rpc_param)),
            }
        }

        method_ids::SingIn => {
            let rpc_param: pb::SingInParam = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1017957090 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::SingIn(rpc_param)),
            }
        }

        method_ids::LogOut => {
            let rpc_param: pb::LogOutParam = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1283119009 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::LogOut(rpc_param)),
            }
        }

        // RPC_Channel
        method_ids::ChannelCreateChannel => {
            let rpc_param: pb::ChannelCreateChannelParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 143251225 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelCreateChannel(rpc_param)),
            }
        }

        method_ids::ChannelEditChannel => {
            let rpc_param: pb::ChannelEditChannelParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 189471894 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelEditChannel(rpc_param)),
            }
        }

        method_ids::ChannelDeleteChannel => {
            let rpc_param: pb::ChannelDeleteChannelParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1494483355 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelDeleteChannel(rpc_param)),
            }
        }

        method_ids::ChannelAddAuthor => {
            let rpc_param: pb::ChannelAddAuthorParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 780397316 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAddAuthor(rpc_param)),
            }
        }

        method_ids::ChannelChangeAuthorPermission => {
            let rpc_param: pb::ChannelChangeAuthorPermissionParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 93233821 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelChangeAuthorPermission(
                    rpc_param,
                )),
            }
        }

        method_ids::ChannelRemoveAuthor => {
            let rpc_param: pb::ChannelRemoveAuthorParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 419542304 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelRemoveAuthor(rpc_param)),
            }
        }

        method_ids::ChannelFollowChannel => {
            let rpc_param: pb::ChannelFollowChannelParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 744563779 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelFollowChannel(rpc_param)),
            }
        }

        method_ids::ChannelUnFollowChannel => {
            let rpc_param: pb::ChannelUnFollowChannelParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 959512423 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelUnFollowChannel(rpc_param)),
            }
        }

        method_ids::ChannelRemoveFollowers => {
            let rpc_param: pb::ChannelRemoveFollowersParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 869709257 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelRemoveFollowers(rpc_param)),
            }
        }

        method_ids::ChannelSubscribe => {
            let rpc_param: pb::ChannelSubscribeParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1367898912 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelSubscribe(rpc_param)),
            }
        }

        method_ids::ChannelUnSubscribe => {
            let rpc_param: pb::ChannelUnSubscribeParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 858172401 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelUnSubscribe(rpc_param)),
            }
        }

        method_ids::ChannelRemoveSubscribers => {
            let rpc_param: pb::ChannelRemoveSubscribersParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 729024592 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelRemoveSubscribers(
                    rpc_param,
                )),
            }
        }

        method_ids::ChannelChangePrivacy => {
            let rpc_param: pb::ChannelChangePrivacyParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 79012409 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelChangePrivacy(rpc_param)),
            }
        }

        method_ids::ChannelChangeDefaultPermission => {
            let rpc_param: pb::ChannelChangeDefaultPermissionParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1582638498 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelChangeDefaultPermission(
                    rpc_param,
                )),
            }
        }

        method_ids::ChannelRevokeLink => {
            let rpc_param: pb::ChannelRevokeLinkParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1912530021 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelRevokeLink(rpc_param)),
            }
        }

        method_ids::ChannelChangeUsername => {
            let rpc_param: pb::ChannelChangeUsernameParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 983884462 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelChangeUsername(rpc_param)),
            }
        }

        method_ids::ChannelBlockChannel => {
            let rpc_param: pb::ChannelBlockChannelParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2037016989 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelBlockChannel(rpc_param)),
            }
        }

        method_ids::ChannelSendMessage => {
            let rpc_param: pb::ChannelSendMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1200751231 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelSendMessage(rpc_param)),
            }
        }

        method_ids::ChannelEditMessage => {
            let rpc_param: pb::ChannelEditMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 727437726 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelEditMessage(rpc_param)),
            }
        }

        method_ids::ChannelPinMessage => {
            let rpc_param: pb::ChannelPinMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 259263709 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelPinMessage(rpc_param)),
            }
        }

        method_ids::ChannelUnPinMessage => {
            let rpc_param: pb::ChannelUnPinMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 113943649 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelUnPinMessage(rpc_param)),
            }
        }

        method_ids::ChannelDeleteMessage => {
            let rpc_param: pb::ChannelDeleteMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 644189206 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelDeleteMessage(rpc_param)),
            }
        }

        method_ids::ChannelDeleteMessages => {
            let rpc_param: pb::ChannelDeleteMessagesParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2124822181 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelDeleteMessages(rpc_param)),
            }
        }

        method_ids::ChannelClearHistory => {
            let rpc_param: pb::ChannelClearHistoryParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1164398815 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelClearHistory(rpc_param)),
            }
        }

        method_ids::ChannelAvatarAdd => {
            let rpc_param: pb::ChannelAvatarAddParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1021808696 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAvatarAdd(rpc_param)),
            }
        }

        method_ids::ChannelAvatarChange => {
            let rpc_param: pb::ChannelAvatarChangeParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1968579501 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAvatarChange(rpc_param)),
            }
        }

        method_ids::ChannelAvatarDelete => {
            let rpc_param: pb::ChannelAvatarDeleteParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1626010891 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAvatarDelete(rpc_param)),
            }
        }

        method_ids::ChannelAvatarGetList => {
            let rpc_param: pb::ChannelAvatarGetListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1925044843 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAvatarGetList(rpc_param)),
            }
        }

        method_ids::ChannelSendDoingAction => {
            let rpc_param: pb::ChannelSendDoingActionParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 973237257 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelSendDoingAction(rpc_param)),
            }
        }

        method_ids::ChannelReportChannel => {
            let rpc_param: pb::ChannelReportChannelParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 792938145 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelReportChannel(rpc_param)),
            }
        }

        method_ids::ChannelReportMessage => {
            let rpc_param: pb::ChannelReportMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2053528327 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelReportMessage(rpc_param)),
            }
        }

        method_ids::ChannelGetFull => {
            let rpc_param: pb::ChannelGetFullParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1684531258 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetFull(rpc_param)),
            }
        }

        method_ids::ChannelGetMessagesList => {
            let rpc_param: pb::ChannelGetMessagesListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1339072968 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetMessagesList(rpc_param)),
            }
        }

        method_ids::ChannelGetMediaList => {
            let rpc_param: pb::ChannelGetMediaListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 985772653 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetMediaList(rpc_param)),
            }
        }

        method_ids::ChannelGetAuthors => {
            let rpc_param: pb::ChannelGetAuthorsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1373284924 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetAuthors(rpc_param)),
            }
        }

        method_ids::ChannelGetFollowers => {
            let rpc_param: pb::ChannelGetFollowersParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1747172143 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetFollowers(rpc_param)),
            }
        }

        method_ids::ChannelGetFollowings => {
            let rpc_param: pb::ChannelGetFollowingsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1838438980 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetFollowings(rpc_param)),
            }
        }

        method_ids::ChannelGetSubscribers => {
            let rpc_param: pb::ChannelGetSubscribersParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2146806736 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetSubscribers(rpc_param)),
            }
        }

        method_ids::ChannelBlocked => {
            let rpc_param: pb::ChannelBlockedParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1674411747 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelBlocked(rpc_param)),
            }
        }

        method_ids::ChannelSetDraft => {
            let rpc_param: pb::ChannelSetDraftParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1403193015 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelSetDraft(rpc_param)),
            }
        }

        // RPC_Chat
        method_ids::ChatSendMessage => {
            let rpc_param: pb::ChatSendMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1131621475 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatSendMessage(rpc_param)),
            }
        }

        method_ids::ChatEditMessage => {
            let rpc_param: pb::ChatEditMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1806258329 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatEditMessage(rpc_param)),
            }
        }

        method_ids::ChatDeleteMessages => {
            let rpc_param: pb::ChatDeleteMessagesParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 933526170 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatDeleteMessages(rpc_param)),
            }
        }

        method_ids::ChatDeleteHistory => {
            let rpc_param: pb::ChatDeleteHistoryParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1088992782 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatDeleteHistory(rpc_param)),
            }
        }

        method_ids::ChatSendDoingAction => {
            let rpc_param: pb::ChatSendDoingActionParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1319324241 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatSendDoingAction(rpc_param)),
            }
        }

        method_ids::ChatReportChat => {
            let rpc_param: pb::ChatReportChatParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1345425871 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatReportChat(rpc_param)),
            }
        }

        method_ids::ChatGetFull => {
            let rpc_param: pb::ChatGetFullMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1768678453 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatGetFull(rpc_param)),
            }
        }

        method_ids::ChatGetMessagesList => {
            let rpc_param: pb::ChatGetMessagesListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 121549718 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatGetMessagesList(rpc_param)),
            }
        }

        method_ids::ChatGetMediaList => {
            let rpc_param: pb::ChatGetMediaListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1346774525 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatGetMediaList(rpc_param)),
            }
        }

        // RPC_Direct
        method_ids::DirectDeleteDirect => {
            let rpc_param: pb::DirectDeleteDirectParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1478067518 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectDeleteDirect(rpc_param)),
            }
        }

        method_ids::DirectChangeTitle => {
            let rpc_param: pb::DirectChangeTitleParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2041790485 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectChangeTitle(rpc_param)),
            }
        }

        method_ids::DirectSetCustomNotification => {
            let rpc_param: pb::DirectSetCustomNotificationParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 548699291 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectSetCustomNotification(
                    rpc_param,
                )),
            }
        }

        method_ids::DirectSendActionDoing => {
            let rpc_param: pb::DirectSendActionDoingParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1417285757 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectSendActionDoing(rpc_param)),
            }
        }

        method_ids::DirectSetDraft => {
            let rpc_param: pb::DirectSetDraftParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1860345925 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectSetDraft(rpc_param)),
            }
        }

        method_ids::DirectDeleteDirects => {
            let rpc_param: pb::DirectDeleteDirectsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1291891637 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectDeleteDirects(rpc_param)),
            }
        }

        method_ids::DirectMarkAsRead => {
            let rpc_param: pb::DirectMarkAsReadParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1801774787 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectMarkAsRead(rpc_param)),
            }
        }

        method_ids::DirectMarkAsUnRead => {
            let rpc_param: pb::DirectMarkAsUnReadParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 313746334 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectMarkAsUnRead(rpc_param)),
            }
        }

        method_ids::DirectPinDirects => {
            let rpc_param: pb::DirectPinDirectsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1179089068 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectPinDirects(rpc_param)),
            }
        }

        method_ids::DirectUnPinDirects => {
            let rpc_param: pb::DirectUnPinDirectsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1517245560 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectUnPinDirects(rpc_param)),
            }
        }

        method_ids::DirectArchiveDirects => {
            let rpc_param: pb::DirectArchiveDirectsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1441782770 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectArchiveDirects(rpc_param)),
            }
        }

        method_ids::DirectUnArchiveDirects => {
            let rpc_param: pb::DirectUnArchiveDirectsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1951553867 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectUnArchiveDirects(rpc_param)),
            }
        }

        method_ids::DirectClearHistories => {
            let rpc_param: pb::DirectClearHistoriesParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 904052140 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectClearHistories(rpc_param)),
            }
        }

        method_ids::DirectMuteDirects => {
            let rpc_param: pb::DirectMuteDirectsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1138477048 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectMuteDirects(rpc_param)),
            }
        }

        method_ids::DirectUnMuteDirects => {
            let rpc_param: pb::DirectUnMuteDirectsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1691834263 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectUnMuteDirects(rpc_param)),
            }
        }

        method_ids::DirectCreateFolder => {
            let rpc_param: pb::DirectCreateFolderParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1878673022 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectCreateFolder(rpc_param)),
            }
        }

        method_ids::DirectChangeFolder => {
            let rpc_param: pb::DirectChangeFolderParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1861381591 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectChangeFolder(rpc_param)),
            }
        }

        method_ids::DirectRemoveFromFolder => {
            let rpc_param: pb::DirectRemoveFromFolderParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1818954127 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectRemoveFromFolder(rpc_param)),
            }
        }

        method_ids::DirectReordersFolder => {
            let rpc_param: pb::DirectReordersFolderParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1264591958 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectReordersFolder(rpc_param)),
            }
        }

        method_ids::DirectDeleteFolder => {
            let rpc_param: pb::DirectDeleteFolderParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 962281627 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectDeleteFolder(rpc_param)),
            }
        }

        method_ids::DirectGetChatsList => {
            let rpc_param: pb::DirectGetChatsListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1570934969 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectGetChatsList(rpc_param)),
            }
        }

        method_ids::DirectGetGroupsList => {
            let rpc_param: pb::DirectGetGroupsListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 545957996 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectGetGroupsList(rpc_param)),
            }
        }

        method_ids::DirectGetChannelsList => {
            let rpc_param: pb::DirectGetChannelsListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1608173619 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectGetChannelsList(rpc_param)),
            }
        }

        method_ids::DirectGetFoldersList => {
            let rpc_param: pb::DirectGetFoldersListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1384523712 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectGetFoldersList(rpc_param)),
            }
        }

        method_ids::DirectGetFoldersFullList => {
            let rpc_param: pb::DirectGetFoldersFullListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 611850722 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectGetFoldersFullList(rpc_param)),
            }
        }

        // RPC_Group
        method_ids::GroupCreateGroup => {
            let rpc_param: pb::GroupCreateGroupParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1205960678 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupCreateGroup(rpc_param)),
            }
        }

        method_ids::GroupEditGroup => {
            let rpc_param: pb::GroupEditGroupParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1665019493 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupEditGroup(rpc_param)),
            }
        }

        method_ids::GroupDeleteGroup => {
            let rpc_param: pb::GroupDeleteGroupParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 365183375 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupDeleteGroup(rpc_param)),
            }
        }

        method_ids::GroupAddAdmin => {
            let rpc_param: pb::GroupAddAdminParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 958971956 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAddAdmin(rpc_param)),
            }
        }

        method_ids::GroupAddMember => {
            let rpc_param: pb::GroupAddMemberParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 676599227 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAddMember(rpc_param)),
            }
        }

        method_ids::GroupRemoveMember => {
            let rpc_param: pb::GroupRemoveMemberParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2012702964 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupRemoveMember(rpc_param)),
            }
        }

        method_ids::GroupChangeMemberLevel => {
            let rpc_param: pb::GroupChangeMemberLevelParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 589574238 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupChangeMemberLevel(rpc_param)),
            }
        }

        method_ids::GroupChangeMemberPermission => {
            let rpc_param: pb::GroupChangeMemberPermissionParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2132464067 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupChangeMemberPermission(
                    rpc_param,
                )),
            }
        }

        method_ids::GroupJoinGroup => {
            let rpc_param: pb::JoinGroupParam = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 591743429 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupJoinGroup(rpc_param)),
            }
        }

        method_ids::GroupLeaveGroup => {
            let rpc_param: pb::GroupLeaveGroupParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 361834630 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupLeaveGroup(rpc_param)),
            }
        }

        method_ids::GroupBanMember => {
            let rpc_param: pb::GroupBanMemberParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 548504852 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupBanMember(rpc_param)),
            }
        }

        method_ids::GroupChangePrivacy => {
            let rpc_param: pb::GroupChangePrivacyParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1497988410 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupChangePrivacy(rpc_param)),
            }
        }

        method_ids::GroupChangeDefaultPermission => {
            let rpc_param: pb::GroupChangeDefaultPermissionParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 605792138 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupChangeDefaultPermission(
                    rpc_param,
                )),
            }
        }

        method_ids::GroupRevokeLink => {
            let rpc_param: pb::GroupRevokeLinkParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 406592509 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupRevokeLink(rpc_param)),
            }
        }

        method_ids::GroupChangeUsername => {
            let rpc_param: pb::GroupChangeUsernameParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 832997038 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupChangeUsername(rpc_param)),
            }
        }

        method_ids::GroupSendMessage => {
            let rpc_param: pb::GroupSendMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 599852950 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupSendMessage(rpc_param)),
            }
        }

        method_ids::GroupEditMessage => {
            let rpc_param: pb::GroupEditMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 742937895 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupEditMessage(rpc_param)),
            }
        }

        method_ids::GroupPinMessage => {
            let rpc_param: pb::GroupPinMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 184560027 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupPinMessage(rpc_param)),
            }
        }

        method_ids::GroupUnPinMessage => {
            let rpc_param: pb::GroupUnPinMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1290613173 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupUnPinMessage(rpc_param)),
            }
        }

        method_ids::GroupDeleteMessage => {
            let rpc_param: pb::GroupDeleteMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 393991035 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupDeleteMessage(rpc_param)),
            }
        }

        method_ids::GroupDeleteMessages => {
            let rpc_param: pb::GroupDeleteMessagesParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 276700675 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupDeleteMessages(rpc_param)),
            }
        }

        method_ids::GroupDeleteHistory => {
            let rpc_param: pb::GroupDeleteHistoryParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1270953793 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupDeleteHistory(rpc_param)),
            }
        }

        method_ids::GroupClearHistory => {
            let rpc_param: pb::GroupClearHistoryParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1352552449 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupClearHistory(rpc_param)),
            }
        }

        method_ids::GroupAvatarAdd => {
            let rpc_param: pb::GroupAvatarAddParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1202058216 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAvatarAdd(rpc_param)),
            }
        }

        method_ids::GroupAvatarChange => {
            let rpc_param: pb::GroupAvatarChangeParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 108612523 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAvatarChange(rpc_param)),
            }
        }

        method_ids::GroupAvatarDelete => {
            let rpc_param: pb::GroupAvatarDeleteParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 775862697 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAvatarDelete(rpc_param)),
            }
        }

        method_ids::GroupAvatarGetList => {
            let rpc_param: pb::GroupAvatarGetListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 939443722 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAvatarGetList(rpc_param)),
            }
        }

        method_ids::GroupSendDoingAction => {
            let rpc_param: pb::GroupSendDoingActionParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2022474356 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupSendDoingAction(rpc_param)),
            }
        }

        method_ids::GroupReportGroup => {
            let rpc_param: pb::GroupReportGroupParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1759704420 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupReportGroup(rpc_param)),
            }
        }

        method_ids::GroupGetFull => {
            let rpc_param: pb::GroupGetFullMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 200351324 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupGetFull(rpc_param)),
            }
        }

        method_ids::GroupGetMessagesList => {
            let rpc_param: pb::GroupGetMessagesListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1541835459 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupGetMessagesList(rpc_param)),
            }
        }

        method_ids::GroupGetMediaList => {
            let rpc_param: pb::GroupGetMediaListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2143016912 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupGetMediaList(rpc_param)),
            }
        }

        method_ids::GroupGetMembersList => {
            let rpc_param: pb::GroupGetMembersListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 429215412 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupGetMembersList(rpc_param)),
            }
        }

        method_ids::GroupGetAdminsList => {
            let rpc_param: pb::GroupGetAdminsListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 332260610 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupGetAdminsList(rpc_param)),
            }
        }

        method_ids::GroupSetDraft => {
            let rpc_param: pb::GroupSetDraftParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 77668156 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupSetDraft(rpc_param)),
            }
        }

        // RPC_Sample
        method_ids::GetUsers1 => {
            let rpc_param: pb::GetUsers1Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 486248681 as i64,
                rpc_service: RPC_Sample(RPC_Sample_MethodData::GetUsers1(rpc_param)),
            }
        }

        method_ids::GetProfiles => {
            let rpc_param: pb::GetProfilesParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 822554282 as i64,
                rpc_service: RPC_Sample(RPC_Sample_MethodData::GetProfiles(rpc_param)),
            }
        }

        method_ids::GetChannels => {
            let rpc_param: pb::GetChannelsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1734748927 as i64,
                rpc_service: RPC_Sample(RPC_Sample_MethodData::GetChannels(rpc_param)),
            }
        }

        method_ids::GetDirects => {
            let rpc_param: pb::GetDirectsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 558085683 as i64,
                rpc_service: RPC_Sample(RPC_Sample_MethodData::GetDirects(rpc_param)),
            }
        }

        method_ids::GetMessages => {
            let rpc_param: pb::GetMessagesParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1160951872 as i64,
                rpc_service: RPC_Sample(RPC_Sample_MethodData::GetMessages(rpc_param)),
            }
        }

        // RPC_Shared
        method_ids::Echo => {
            let rpc_param: pb::EchoParam = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 101973561 as i64,
                rpc_service: RPC_Shared(RPC_Shared_MethodData::Echo(rpc_param)),
            }
        }

        method_ids::CheckUserName => {
            let rpc_param: pb::CheckUserNameParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1897027349 as i64,
                rpc_service: RPC_Shared(RPC_Shared_MethodData::CheckUserName(rpc_param)),
            }
        }

        // RPC_Upload
        method_ids::UploadFile => {
            let rpc_param: pb::UploadFileParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1702285478 as i64,
                rpc_service: RPC_Upload(RPC_Upload_MethodData::UploadFile(rpc_param)),
            }
        }

        // RPC_User
        method_ids::ChangePhoneNumber => {
            let rpc_param: pb::ChangePhoneNumberParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 706069694 as i64,
                rpc_service: RPC_User(RPC_User_MethodData::ChangePhoneNumber(rpc_param)),
            }
        }

        _ => panic!("sdf"),
    };
    Ok(rpc)
}

pub async fn server_rpc(act: RpcInvoke, reg: &RPC_Registry) -> Result<Vec<u8>, GenErr> {
    let res_v8 = match act.rpc_service {
        RpcServiceData::RPC_Auth(method) => match method {
            RPC_Auth_MethodData::SendConfirmCode(param) => {
                let handler = eror(&reg.RPC_Auth)?;
                let response = handler.SendConfirmCode(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Auth_MethodData::ConfirmCode(param) => {
                let handler = eror(&reg.RPC_Auth)?;
                let response = handler.ConfirmCode(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Auth_MethodData::SingUp(param) => {
                let handler = eror(&reg.RPC_Auth)?;
                let response = handler.SingUp(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Auth_MethodData::SingIn(param) => {
                let handler = eror(&reg.RPC_Auth)?;
                let response = handler.SingIn(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Auth_MethodData::LogOut(param) => {
                let handler = eror(&reg.RPC_Auth)?;
                let response = handler.LogOut(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Channel(method) => match method {
            RPC_Channel_MethodData::ChannelCreateChannel(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelCreateChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelEditChannel(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelEditChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelDeleteChannel(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelDeleteChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelAddAuthor(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelAddAuthor(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelChangeAuthorPermission(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelChangeAuthorPermission(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelRemoveAuthor(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelRemoveAuthor(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelFollowChannel(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelFollowChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelUnFollowChannel(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelUnFollowChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelRemoveFollowers(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelRemoveFollowers(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelSubscribe(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelSubscribe(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelUnSubscribe(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelUnSubscribe(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelRemoveSubscribers(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelRemoveSubscribers(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelChangePrivacy(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelChangePrivacy(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelChangeDefaultPermission(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelChangeDefaultPermission(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelRevokeLink(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelRevokeLink(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelChangeUsername(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelChangeUsername(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelBlockChannel(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelBlockChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelSendMessage(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelSendMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelEditMessage(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelEditMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelPinMessage(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelPinMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelUnPinMessage(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelUnPinMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelDeleteMessage(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelDeleteMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelDeleteMessages(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelDeleteMessages(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelClearHistory(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelClearHistory(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelAvatarAdd(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelAvatarAdd(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelAvatarChange(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelAvatarChange(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelAvatarDelete(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelAvatarDelete(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelAvatarGetList(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelAvatarGetList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelSendDoingAction(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelSendDoingAction(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelReportChannel(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelReportChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelReportMessage(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelReportMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetFull(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelGetFull(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetMessagesList(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelGetMessagesList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetMediaList(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelGetMediaList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetAuthors(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelGetAuthors(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetFollowers(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelGetFollowers(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetFollowings(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelGetFollowings(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetSubscribers(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelGetSubscribers(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelBlocked(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelBlocked(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelSetDraft(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelSetDraft(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Chat(method) => match method {
            RPC_Chat_MethodData::ChatSendMessage(param) => {
                let handler = eror(&reg.RPC_Chat)?;
                let response = handler.ChatSendMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatEditMessage(param) => {
                let handler = eror(&reg.RPC_Chat)?;
                let response = handler.ChatEditMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatDeleteMessages(param) => {
                let handler = eror(&reg.RPC_Chat)?;
                let response = handler.ChatDeleteMessages(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatDeleteHistory(param) => {
                let handler = eror(&reg.RPC_Chat)?;
                let response = handler.ChatDeleteHistory(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatSendDoingAction(param) => {
                let handler = eror(&reg.RPC_Chat)?;
                let response = handler.ChatSendDoingAction(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatReportChat(param) => {
                let handler = eror(&reg.RPC_Chat)?;
                let response = handler.ChatReportChat(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatGetFull(param) => {
                let handler = eror(&reg.RPC_Chat)?;
                let response = handler.ChatGetFull(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatGetMessagesList(param) => {
                let handler = eror(&reg.RPC_Chat)?;
                let response = handler.ChatGetMessagesList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatGetMediaList(param) => {
                let handler = eror(&reg.RPC_Chat)?;
                let response = handler.ChatGetMediaList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Direct(method) => match method {
            RPC_Direct_MethodData::DirectDeleteDirect(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectDeleteDirect(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectChangeTitle(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectChangeTitle(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectSetCustomNotification(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectSetCustomNotification(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectSendActionDoing(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectSendActionDoing(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectSetDraft(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectSetDraft(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectDeleteDirects(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectDeleteDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectMarkAsRead(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectMarkAsRead(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectMarkAsUnRead(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectMarkAsUnRead(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectPinDirects(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectPinDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectUnPinDirects(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectUnPinDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectArchiveDirects(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectArchiveDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectUnArchiveDirects(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectUnArchiveDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectClearHistories(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectClearHistories(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectMuteDirects(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectMuteDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectUnMuteDirects(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectUnMuteDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectCreateFolder(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectCreateFolder(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectChangeFolder(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectChangeFolder(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectRemoveFromFolder(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectRemoveFromFolder(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectReordersFolder(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectReordersFolder(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectDeleteFolder(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectDeleteFolder(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectGetChatsList(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectGetChatsList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectGetGroupsList(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectGetGroupsList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectGetChannelsList(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectGetChannelsList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectGetFoldersList(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectGetFoldersList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectGetFoldersFullList(param) => {
                let handler = eror(&reg.RPC_Direct)?;
                let response = handler.DirectGetFoldersFullList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Group(method) => match method {
            RPC_Group_MethodData::GroupCreateGroup(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupCreateGroup(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupEditGroup(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupEditGroup(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupDeleteGroup(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupDeleteGroup(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAddAdmin(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupAddAdmin(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAddMember(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupAddMember(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupRemoveMember(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupRemoveMember(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupChangeMemberLevel(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupChangeMemberLevel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupChangeMemberPermission(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupChangeMemberPermission(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupJoinGroup(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupJoinGroup(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupLeaveGroup(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupLeaveGroup(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupBanMember(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupBanMember(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupChangePrivacy(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupChangePrivacy(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupChangeDefaultPermission(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupChangeDefaultPermission(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupRevokeLink(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupRevokeLink(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupChangeUsername(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupChangeUsername(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupSendMessage(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupSendMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupEditMessage(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupEditMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupPinMessage(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupPinMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupUnPinMessage(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupUnPinMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupDeleteMessage(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupDeleteMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupDeleteMessages(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupDeleteMessages(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupDeleteHistory(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupDeleteHistory(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupClearHistory(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupClearHistory(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAvatarAdd(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupAvatarAdd(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAvatarChange(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupAvatarChange(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAvatarDelete(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupAvatarDelete(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAvatarGetList(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupAvatarGetList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupSendDoingAction(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupSendDoingAction(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupReportGroup(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupReportGroup(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupGetFull(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupGetFull(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupGetMessagesList(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupGetMessagesList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupGetMediaList(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupGetMediaList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupGetMembersList(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupGetMembersList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupGetAdminsList(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupGetAdminsList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupSetDraft(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupSetDraft(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Sample(method) => match method {
            RPC_Sample_MethodData::GetUsers1(param) => {
                let handler = eror(&reg.RPC_Sample)?;
                let response = handler.GetUsers1(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Sample_MethodData::GetProfiles(param) => {
                let handler = eror(&reg.RPC_Sample)?;
                let response = handler.GetProfiles(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Sample_MethodData::GetChannels(param) => {
                let handler = eror(&reg.RPC_Sample)?;
                let response = handler.GetChannels(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Sample_MethodData::GetDirects(param) => {
                let handler = eror(&reg.RPC_Sample)?;
                let response = handler.GetDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Sample_MethodData::GetMessages(param) => {
                let handler = eror(&reg.RPC_Sample)?;
                let response = handler.GetMessages(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Shared(method) => match method {
            RPC_Shared_MethodData::Echo(param) => {
                let handler = eror(&reg.RPC_Shared)?;
                let response = handler.Echo(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shared_MethodData::CheckUserName(param) => {
                let handler = eror(&reg.RPC_Shared)?;
                let response = handler.CheckUserName(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Upload(method) => match method {
            RPC_Upload_MethodData::UploadFile(param) => {
                let handler = eror(&reg.RPC_Upload)?;
                let response = handler.UploadFile(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_User(method) => match method {
            RPC_User_MethodData::ChangePhoneNumber(param) => {
                let handler = eror(&reg.RPC_User)?;
                let response = handler.ChangePhoneNumber(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },
    };

    Ok(res_v8)
}

#[derive(Default)]
pub struct RPC_Registry {
    pub RPC_Auth: Option<Box<RPC_Auth_Handler2>>,
    pub RPC_Channel: Option<Box<RPC_Channel_Handler2>>,
    pub RPC_Chat: Option<Box<RPC_Chat_Handler2>>,
    pub RPC_Direct: Option<Box<RPC_Direct_Handler2>>,
    pub RPC_Group: Option<Box<RPC_Group_Handler2>>,
    pub RPC_Sample: Option<Box<RPC_Sample_Handler2>>,
    pub RPC_Shared: Option<Box<RPC_Shared_Handler2>>,
    pub RPC_Upload: Option<Box<RPC_Upload_Handler2>>,
    pub RPC_User: Option<Box<RPC_User_Handler2>>,
}

impl RPC_Auth_Handler for RPC_Registry {}
impl RPC_Auth_Handler2 for RPC_Registry {}
impl RPC_Channel_Handler for RPC_Registry {}
impl RPC_Channel_Handler2 for RPC_Registry {}
impl RPC_Chat_Handler for RPC_Registry {}
impl RPC_Chat_Handler2 for RPC_Registry {}
impl RPC_Direct_Handler for RPC_Registry {}
impl RPC_Direct_Handler2 for RPC_Registry {}
impl RPC_Group_Handler for RPC_Registry {}
impl RPC_Group_Handler2 for RPC_Registry {}
impl RPC_Sample_Handler for RPC_Registry {}
impl RPC_Sample_Handler2 for RPC_Registry {}
impl RPC_Shared_Handler for RPC_Registry {}
impl RPC_Shared_Handler2 for RPC_Registry {}
impl RPC_Upload_Handler for RPC_Registry {}
impl RPC_Upload_Handler2 for RPC_Registry {}
impl RPC_User_Handler for RPC_Registry {}
impl RPC_User_Handler2 for RPC_Registry {}

fn to_vev8(msg: &impl prost::Message) -> Result<Vec<u8>, GenErr> {
    let mut buff = vec![];
    prost::Message::encode(msg, &mut buff)?;
    Ok(buff)
}

fn eror<T>(input: &Option<T>) -> Result<&T, GenErr> {
    match input {
        Some(inbox) => Ok(inbox),
        None => Err(GenErr::NoRpcRegistry),
    }
}

impl common::RpcClient {
    // service: RPC_Auth
    pub async fn SendConfirmCode(
        &self,
        param: pb::SendConfirmCodeParam,
    ) -> Result<pb::SendConfirmCodeResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::SendConfirmCode).await?;
        Ok(pb_res)
    }

    pub async fn ConfirmCode(
        &self,
        param: pb::ConfirmCodeParam,
    ) -> Result<pb::ConfirmCodeResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ConfirmCode).await?;
        Ok(pb_res)
    }

    pub async fn SingUp(&self, param: pb::SingUpParam) -> Result<pb::SingUpResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::SingUp).await?;
        Ok(pb_res)
    }

    pub async fn SingIn(&self, param: pb::SingInParam) -> Result<pb::SingInResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::SingIn).await?;
        Ok(pb_res)
    }

    pub async fn LogOut(&self, param: pb::LogOutParam) -> Result<pb::LogOutResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::LogOut).await?;
        Ok(pb_res)
    }

    // service: RPC_Channel
    pub async fn ChannelCreateChannel(
        &self,
        param: pb::ChannelCreateChannelParam,
    ) -> Result<pb::ChannelCreateChannelResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelCreateChannel)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelEditChannel(
        &self,
        param: pb::ChannelEditChannelParam,
    ) -> Result<pb::ChannelEditChannelResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelEditChannel)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelDeleteChannel(
        &self,
        param: pb::ChannelDeleteChannelParam,
    ) -> Result<pb::ChannelDeleteChannelResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelDeleteChannel)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelAddAuthor(
        &self,
        param: pb::ChannelAddAuthorParam,
    ) -> Result<pb::ChannelAddAuthorResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelAddAuthor)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelChangeAuthorPermission(
        &self,
        param: pb::ChannelChangeAuthorPermissionParam,
    ) -> Result<pb::ChannelChangeAuthorPermissionResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelChangeAuthorPermission)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelRemoveAuthor(
        &self,
        param: pb::ChannelRemoveAuthorParam,
    ) -> Result<pb::ChannelRemoveAuthorResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelRemoveAuthor)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelFollowChannel(
        &self,
        param: pb::ChannelFollowChannelParam,
    ) -> Result<pb::ChannelFollowChannelResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelFollowChannel)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelUnFollowChannel(
        &self,
        param: pb::ChannelUnFollowChannelParam,
    ) -> Result<pb::ChannelUnFollowChannelResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelUnFollowChannel)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelRemoveFollowers(
        &self,
        param: pb::ChannelRemoveFollowersParam,
    ) -> Result<pb::ChannelRemoveFollowersResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelRemoveFollowers)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelSubscribe(
        &self,
        param: pb::ChannelSubscribeParam,
    ) -> Result<pb::ChannelSubscribeResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelSubscribe)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelUnSubscribe(
        &self,
        param: pb::ChannelUnSubscribeParam,
    ) -> Result<pb::ChannelUnSubscribeResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelUnSubscribe)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelRemoveSubscribers(
        &self,
        param: pb::ChannelRemoveSubscribersParam,
    ) -> Result<pb::ChannelRemoveSubscribersResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelRemoveSubscribers)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelChangePrivacy(
        &self,
        param: pb::ChannelChangePrivacyParam,
    ) -> Result<pb::ChannelChangePrivacyResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelChangePrivacy)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelChangeDefaultPermission(
        &self,
        param: pb::ChannelChangeDefaultPermissionParam,
    ) -> Result<pb::ChannelChangeDefaultPermissionResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelChangeDefaultPermission)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelRevokeLink(
        &self,
        param: pb::ChannelRevokeLinkParam,
    ) -> Result<pb::ChannelRevokeLinkResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelRevokeLink)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelChangeUsername(
        &self,
        param: pb::ChannelChangeUsernameParam,
    ) -> Result<pb::ChannelChangeUsernameResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelChangeUsername)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelBlockChannel(
        &self,
        param: pb::ChannelBlockChannelParam,
    ) -> Result<pb::ChannelBlockChannelResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelBlockChannel)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelSendMessage(
        &self,
        param: pb::ChannelSendMessageParam,
    ) -> Result<pb::ChannelSendMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelSendMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelEditMessage(
        &self,
        param: pb::ChannelEditMessageParam,
    ) -> Result<pb::ChannelEditMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelEditMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelPinMessage(
        &self,
        param: pb::ChannelPinMessageParam,
    ) -> Result<pb::ChannelPinMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelPinMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelUnPinMessage(
        &self,
        param: pb::ChannelUnPinMessageParam,
    ) -> Result<pb::ChannelUnPinMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelUnPinMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelDeleteMessage(
        &self,
        param: pb::ChannelDeleteMessageParam,
    ) -> Result<pb::ChannelDeleteMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelDeleteMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelDeleteMessages(
        &self,
        param: pb::ChannelDeleteMessagesParam,
    ) -> Result<pb::ChannelDeleteMessagesResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelDeleteMessages)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelClearHistory(
        &self,
        param: pb::ChannelClearHistoryParam,
    ) -> Result<pb::ChannelClearHistoryResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelClearHistory)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelAvatarAdd(
        &self,
        param: pb::ChannelAvatarAddParam,
    ) -> Result<pb::ChannelAvatarAddResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelAvatarAdd)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelAvatarChange(
        &self,
        param: pb::ChannelAvatarChangeParam,
    ) -> Result<pb::ChannelAvatarChangeResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelAvatarChange)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelAvatarDelete(
        &self,
        param: pb::ChannelAvatarDeleteParam,
    ) -> Result<pb::ChannelAvatarDeleteResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelAvatarDelete)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelAvatarGetList(
        &self,
        param: pb::ChannelAvatarGetListParam,
    ) -> Result<pb::ChannelAvatarGetListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelAvatarGetList)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelSendDoingAction(
        &self,
        param: pb::ChannelSendDoingActionParam,
    ) -> Result<pb::ChannelSendDoingActionResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelSendDoingAction)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelReportChannel(
        &self,
        param: pb::ChannelReportChannelParam,
    ) -> Result<pb::ChannelReportChannelResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelReportChannel)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelReportMessage(
        &self,
        param: pb::ChannelReportMessageParam,
    ) -> Result<pb::ChannelReportMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelReportMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelGetFull(
        &self,
        param: pb::ChannelGetFullParam,
    ) -> Result<pb::ChannelGetFullResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ChannelGetFull).await?;
        Ok(pb_res)
    }

    pub async fn ChannelGetMessagesList(
        &self,
        param: pb::ChannelGetMessagesListParam,
    ) -> Result<pb::ChannelGetMessagesListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelGetMessagesList)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelGetMediaList(
        &self,
        param: pb::ChannelGetMediaListParam,
    ) -> Result<pb::ChannelGetMediaListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelGetMediaList)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelGetAuthors(
        &self,
        param: pb::ChannelGetAuthorsParam,
    ) -> Result<pb::ChannelGetAuthorsResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelGetAuthors)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelGetFollowers(
        &self,
        param: pb::ChannelGetFollowersParam,
    ) -> Result<pb::ChannelGetFollowersResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelGetFollowers)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelGetFollowings(
        &self,
        param: pb::ChannelGetFollowingsParam,
    ) -> Result<pb::ChannelGetFollowingsResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelGetFollowings)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelGetSubscribers(
        &self,
        param: pb::ChannelGetSubscribersParam,
    ) -> Result<pb::ChannelGetSubscribersResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelGetSubscribers)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelBlocked(
        &self,
        param: pb::ChannelBlockedParam,
    ) -> Result<pb::ChannelBlockedResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ChannelBlocked).await?;
        Ok(pb_res)
    }

    pub async fn ChannelSetDraft(
        &self,
        param: pb::ChannelSetDraftParam,
    ) -> Result<pb::ChannelSetDraftResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ChannelSetDraft).await?;
        Ok(pb_res)
    }

    // service: RPC_Chat
    pub async fn ChatSendMessage(
        &self,
        param: pb::ChatSendMessageParam,
    ) -> Result<pb::ChatSendMessageResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ChatSendMessage).await?;
        Ok(pb_res)
    }

    pub async fn ChatEditMessage(
        &self,
        param: pb::ChatEditMessageParam,
    ) -> Result<pb::ChatEditMessageResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ChatEditMessage).await?;
        Ok(pb_res)
    }

    pub async fn ChatDeleteMessages(
        &self,
        param: pb::ChatDeleteMessagesParam,
    ) -> Result<pb::ChatDeleteMessagesResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChatDeleteMessages)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChatDeleteHistory(
        &self,
        param: pb::ChatDeleteHistoryParam,
    ) -> Result<pb::ChatDeleteHistoryResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChatDeleteHistory)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChatSendDoingAction(
        &self,
        param: pb::ChatSendDoingActionParam,
    ) -> Result<pb::ChatSendDoingActionResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChatSendDoingAction)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChatReportChat(
        &self,
        param: pb::ChatReportChatParam,
    ) -> Result<pb::ChatReportChatResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ChatReportChat).await?;
        Ok(pb_res)
    }

    pub async fn ChatGetFull(
        &self,
        param: pb::ChatGetFullMessageParam,
    ) -> Result<pb::ChatGetFullMessageResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ChatGetFull).await?;
        Ok(pb_res)
    }

    pub async fn ChatGetMessagesList(
        &self,
        param: pb::ChatGetMessagesListParam,
    ) -> Result<pb::ChatGetMessagesListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChatGetMessagesList)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChatGetMediaList(
        &self,
        param: pb::ChatGetMediaListParam,
    ) -> Result<pb::ChatGetMediaListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChatGetMediaList)
            .await?;
        Ok(pb_res)
    }

    // service: RPC_Direct
    pub async fn DirectDeleteDirect(
        &self,
        param: pb::DirectDeleteDirectParam,
    ) -> Result<pb::DirectDeleteDirectResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectDeleteDirect)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectChangeTitle(
        &self,
        param: pb::DirectChangeTitleParam,
    ) -> Result<pb::DirectChangeTitleResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectChangeTitle)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectSetCustomNotification(
        &self,
        param: pb::DirectSetCustomNotificationParam,
    ) -> Result<pb::DirectSetCustomNotificationResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectSetCustomNotification)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectSendActionDoing(
        &self,
        param: pb::DirectSendActionDoingParam,
    ) -> Result<pb::DirectSendActionDoingResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectSendActionDoing)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectSetDraft(
        &self,
        param: pb::DirectSetDraftParam,
    ) -> Result<pb::DirectSetDraftResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::DirectSetDraft).await?;
        Ok(pb_res)
    }

    pub async fn DirectDeleteDirects(
        &self,
        param: pb::DirectDeleteDirectsParam,
    ) -> Result<pb::DirectDeleteDirectsResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectDeleteDirects)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectMarkAsRead(
        &self,
        param: pb::DirectMarkAsReadParam,
    ) -> Result<pb::DirectMarkAsReadResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectMarkAsRead)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectMarkAsUnRead(
        &self,
        param: pb::DirectMarkAsUnReadParam,
    ) -> Result<pb::DirectMarkAsUnReadResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectMarkAsUnRead)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectPinDirects(
        &self,
        param: pb::DirectPinDirectsParam,
    ) -> Result<pb::DirectPinDirectsResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectPinDirects)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectUnPinDirects(
        &self,
        param: pb::DirectUnPinDirectsParam,
    ) -> Result<pb::DirectUnPinDirectsResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectUnPinDirects)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectArchiveDirects(
        &self,
        param: pb::DirectArchiveDirectsParam,
    ) -> Result<pb::DirectArchiveDirectsResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectArchiveDirects)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectUnArchiveDirects(
        &self,
        param: pb::DirectUnArchiveDirectsParam,
    ) -> Result<pb::DirectUnArchiveDirectsResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectUnArchiveDirects)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectClearHistories(
        &self,
        param: pb::DirectClearHistoriesParam,
    ) -> Result<pb::DirectClearHistoriesResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectClearHistories)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectMuteDirects(
        &self,
        param: pb::DirectMuteDirectsParam,
    ) -> Result<pb::DirectMuteDirectsResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectMuteDirects)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectUnMuteDirects(
        &self,
        param: pb::DirectUnMuteDirectsParam,
    ) -> Result<pb::DirectUnMuteDirectsResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectUnMuteDirects)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectCreateFolder(
        &self,
        param: pb::DirectCreateFolderParam,
    ) -> Result<pb::DirectCreateFolderResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectCreateFolder)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectChangeFolder(
        &self,
        param: pb::DirectChangeFolderParam,
    ) -> Result<pb::DirectChangeFolderResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectChangeFolder)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectRemoveFromFolder(
        &self,
        param: pb::DirectRemoveFromFolderParam,
    ) -> Result<pb::DirectRemoveFromFolderResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectRemoveFromFolder)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectReordersFolder(
        &self,
        param: pb::DirectReordersFolderParam,
    ) -> Result<pb::DirectReordersFolderResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectReordersFolder)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectDeleteFolder(
        &self,
        param: pb::DirectDeleteFolderParam,
    ) -> Result<pb::DirectDeleteFolderResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectDeleteFolder)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectGetChatsList(
        &self,
        param: pb::DirectGetChatsListParam,
    ) -> Result<pb::DirectGetChatsListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectGetChatsList)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectGetGroupsList(
        &self,
        param: pb::DirectGetGroupsListParam,
    ) -> Result<pb::DirectGetGroupsListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectGetGroupsList)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectGetChannelsList(
        &self,
        param: pb::DirectGetChannelsListParam,
    ) -> Result<pb::DirectGetChannelsListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectGetChannelsList)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectGetFoldersList(
        &self,
        param: pb::DirectGetFoldersListParam,
    ) -> Result<pb::DirectGetFoldersListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectGetFoldersList)
            .await?;
        Ok(pb_res)
    }

    pub async fn DirectGetFoldersFullList(
        &self,
        param: pb::DirectGetFoldersFullListParam,
    ) -> Result<pb::DirectGetFoldersFullListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::DirectGetFoldersFullList)
            .await?;
        Ok(pb_res)
    }

    // service: RPC_Group
    pub async fn GroupCreateGroup(
        &self,
        param: pb::GroupCreateGroupParam,
    ) -> Result<pb::GroupCreateGroupResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupCreateGroup)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupEditGroup(
        &self,
        param: pb::GroupEditGroupParam,
    ) -> Result<pb::GroupEditGroupResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupEditGroup).await?;
        Ok(pb_res)
    }

    pub async fn GroupDeleteGroup(
        &self,
        param: pb::GroupDeleteGroupParam,
    ) -> Result<pb::GroupDeleteGroupResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupDeleteGroup)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupAddAdmin(
        &self,
        param: pb::GroupAddAdminParam,
    ) -> Result<pb::GroupAddAdminResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupAddAdmin).await?;
        Ok(pb_res)
    }

    pub async fn GroupAddMember(
        &self,
        param: pb::GroupAddMemberParam,
    ) -> Result<pb::GroupAddMemberResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupAddMember).await?;
        Ok(pb_res)
    }

    pub async fn GroupRemoveMember(
        &self,
        param: pb::GroupRemoveMemberParam,
    ) -> Result<pb::GroupRemoveMemberResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupRemoveMember)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupChangeMemberLevel(
        &self,
        param: pb::GroupChangeMemberLevelParam,
    ) -> Result<pb::GroupChangeMemberLevelResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupChangeMemberLevel)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupChangeMemberPermission(
        &self,
        param: pb::GroupChangeMemberPermissionParam,
    ) -> Result<pb::GroupChangeMemberPermissionResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupChangeMemberPermission)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupJoinGroup(
        &self,
        param: pb::JoinGroupParam,
    ) -> Result<pb::JoinGroupResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupJoinGroup).await?;
        Ok(pb_res)
    }

    pub async fn GroupLeaveGroup(
        &self,
        param: pb::GroupLeaveGroupParam,
    ) -> Result<pb::GroupLeaveGroupResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupLeaveGroup).await?;
        Ok(pb_res)
    }

    pub async fn GroupBanMember(
        &self,
        param: pb::GroupBanMemberParam,
    ) -> Result<pb::GroupBanMemberResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupBanMember).await?;
        Ok(pb_res)
    }

    pub async fn GroupChangePrivacy(
        &self,
        param: pb::GroupChangePrivacyParam,
    ) -> Result<pb::GroupChangePrivacyResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupChangePrivacy)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupChangeDefaultPermission(
        &self,
        param: pb::GroupChangeDefaultPermissionParam,
    ) -> Result<pb::GroupChangeDefaultPermissionResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupChangeDefaultPermission)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupRevokeLink(
        &self,
        param: pb::GroupRevokeLinkParam,
    ) -> Result<pb::GroupRevokeLinkResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupRevokeLink).await?;
        Ok(pb_res)
    }

    pub async fn GroupChangeUsername(
        &self,
        param: pb::GroupChangeUsernameParam,
    ) -> Result<pb::GroupChangeUsernameResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupChangeUsername)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupSendMessage(
        &self,
        param: pb::GroupSendMessageParam,
    ) -> Result<pb::GroupSendMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupSendMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupEditMessage(
        &self,
        param: pb::GroupEditMessageParam,
    ) -> Result<pb::GroupEditMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupEditMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupPinMessage(
        &self,
        param: pb::GroupPinMessageParam,
    ) -> Result<pb::GroupPinMessageResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupPinMessage).await?;
        Ok(pb_res)
    }

    pub async fn GroupUnPinMessage(
        &self,
        param: pb::GroupUnPinMessageParam,
    ) -> Result<pb::GroupUnPinMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupUnPinMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupDeleteMessage(
        &self,
        param: pb::GroupDeleteMessageParam,
    ) -> Result<pb::GroupDeleteMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupDeleteMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupDeleteMessages(
        &self,
        param: pb::GroupDeleteMessagesParam,
    ) -> Result<pb::GroupDeleteMessagesResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupDeleteMessages)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupDeleteHistory(
        &self,
        param: pb::GroupDeleteHistoryParam,
    ) -> Result<pb::GroupDeleteHistoryResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupDeleteHistory)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupClearHistory(
        &self,
        param: pb::GroupClearHistoryParam,
    ) -> Result<pb::GroupClearHistoryResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupClearHistory)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupAvatarAdd(
        &self,
        param: pb::GroupAvatarAddParam,
    ) -> Result<pb::GroupAvatarAddResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupAvatarAdd).await?;
        Ok(pb_res)
    }

    pub async fn GroupAvatarChange(
        &self,
        param: pb::GroupAvatarChangeParam,
    ) -> Result<pb::GroupAvatarChangeResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupAvatarChange)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupAvatarDelete(
        &self,
        param: pb::GroupAvatarDeleteParam,
    ) -> Result<pb::GroupAvatarDeleteResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupAvatarDelete)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupAvatarGetList(
        &self,
        param: pb::GroupAvatarGetListParam,
    ) -> Result<pb::GroupAvatarGetListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupAvatarGetList)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupSendDoingAction(
        &self,
        param: pb::GroupSendDoingActionParam,
    ) -> Result<pb::GroupSendDoingActionResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupSendDoingAction)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupReportGroup(
        &self,
        param: pb::GroupReportGroupParam,
    ) -> Result<pb::GroupReportGroupResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupReportGroup)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupGetFull(
        &self,
        param: pb::GroupGetFullMessageParam,
    ) -> Result<pb::GroupGetFullMessageResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupGetFull).await?;
        Ok(pb_res)
    }

    pub async fn GroupGetMessagesList(
        &self,
        param: pb::GroupGetMessagesListParam,
    ) -> Result<pb::GroupGetMessagesListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupGetMessagesList)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupGetMediaList(
        &self,
        param: pb::GroupGetMediaListParam,
    ) -> Result<pb::GroupGetMediaListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupGetMediaList)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupGetMembersList(
        &self,
        param: pb::GroupGetMembersListParam,
    ) -> Result<pb::GroupGetMembersListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupGetMembersList)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupGetAdminsList(
        &self,
        param: pb::GroupGetAdminsListParam,
    ) -> Result<pb::GroupGetAdminsListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupGetAdminsList)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupSetDraft(
        &self,
        param: pb::GroupSetDraftParam,
    ) -> Result<pb::GroupSetDraftResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupSetDraft).await?;
        Ok(pb_res)
    }

    // service: RPC_Sample
    pub async fn GetUsers1(
        &self,
        param: pb::GetUsers1Param,
    ) -> Result<pb::GetUsers1Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GetUsers1).await?;
        Ok(pb_res)
    }

    pub async fn GetProfiles(
        &self,
        param: pb::GetProfilesParam,
    ) -> Result<pb::GetProfilesResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GetProfiles).await?;
        Ok(pb_res)
    }

    pub async fn GetChannels(
        &self,
        param: pb::GetChannelsParam,
    ) -> Result<pb::GetChannelsResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GetChannels).await?;
        Ok(pb_res)
    }

    pub async fn GetDirects(
        &self,
        param: pb::GetDirectsParam,
    ) -> Result<pb::GetDirectsResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GetDirects).await?;
        Ok(pb_res)
    }

    pub async fn GetMessages(
        &self,
        param: pb::GetMessagesParam,
    ) -> Result<pb::GetMessagesResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GetMessages).await?;
        Ok(pb_res)
    }

    // service: RPC_Shared
    pub async fn Echo(&self, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::Echo).await?;
        Ok(pb_res)
    }

    pub async fn CheckUserName(
        &self,
        param: pb::CheckUserNameParam,
    ) -> Result<pb::CheckUserNameResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::CheckUserName).await?;
        Ok(pb_res)
    }

    // service: RPC_Upload
    pub async fn UploadFile(
        &self,
        param: pb::UploadFileParam,
    ) -> Result<pb::UploadFileResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::UploadFile).await?;
        Ok(pb_res)
    }

    // service: RPC_User
    pub async fn ChangePhoneNumber(
        &self,
        param: pb::ChangePhoneNumberParam,
    ) -> Result<pb::ChangePhoneNumberResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChangePhoneNumber)
            .await?;
        Ok(pb_res)
    }
}

/////////////////////// Code gen for def rpc //////////////
struct _RRR_ {}

#[async_trait]
impl RPC_Auth_Handler2 for _RRR_ {
    async fn SendConfirmCode(
        &self,
        param: pb::SendConfirmCodeParam,
    ) -> Result<pb::SendConfirmCodeResponse, GenErr> {
        println!("called SendConfirmCode in the impl code.");
        Ok(pb::SendConfirmCodeResponse::default())
    }
    async fn ConfirmCode(
        &self,
        param: pb::ConfirmCodeParam,
    ) -> Result<pb::ConfirmCodeResponse, GenErr> {
        println!("called ConfirmCode in the impl code.");
        Ok(pb::ConfirmCodeResponse::default())
    }
    async fn SingUp(&self, param: pb::SingUpParam) -> Result<pb::SingUpResponse, GenErr> {
        println!("called SingUp in the impl code.");
        Ok(pb::SingUpResponse::default())
    }
    async fn SingIn(&self, param: pb::SingInParam) -> Result<pb::SingInResponse, GenErr> {
        println!("called SingIn in the impl code.");
        Ok(pb::SingInResponse::default())
    }
    async fn LogOut(&self, param: pb::LogOutParam) -> Result<pb::LogOutResponse, GenErr> {
        println!("called LogOut in the impl code.");
        Ok(pb::LogOutResponse::default())
    }
}
#[async_trait]
impl RPC_Channel_Handler2 for _RRR_ {
    async fn ChannelCreateChannel(
        &self,
        param: pb::ChannelCreateChannelParam,
    ) -> Result<pb::ChannelCreateChannelResponse, GenErr> {
        println!("called ChannelCreateChannel in the impl code.");
        Ok(pb::ChannelCreateChannelResponse::default())
    }
    async fn ChannelEditChannel(
        &self,
        param: pb::ChannelEditChannelParam,
    ) -> Result<pb::ChannelEditChannelResponse, GenErr> {
        println!("called ChannelEditChannel in the impl code.");
        Ok(pb::ChannelEditChannelResponse::default())
    }
    async fn ChannelDeleteChannel(
        &self,
        param: pb::ChannelDeleteChannelParam,
    ) -> Result<pb::ChannelDeleteChannelResponse, GenErr> {
        println!("called ChannelDeleteChannel in the impl code.");
        Ok(pb::ChannelDeleteChannelResponse::default())
    }
    async fn ChannelAddAuthor(
        &self,
        param: pb::ChannelAddAuthorParam,
    ) -> Result<pb::ChannelAddAuthorResponse, GenErr> {
        println!("called ChannelAddAuthor in the impl code.");
        Ok(pb::ChannelAddAuthorResponse::default())
    }
    async fn ChannelChangeAuthorPermission(
        &self,
        param: pb::ChannelChangeAuthorPermissionParam,
    ) -> Result<pb::ChannelChangeAuthorPermissionResponse, GenErr> {
        println!("called ChannelChangeAuthorPermission in the impl code.");
        Ok(pb::ChannelChangeAuthorPermissionResponse::default())
    }
    async fn ChannelRemoveAuthor(
        &self,
        param: pb::ChannelRemoveAuthorParam,
    ) -> Result<pb::ChannelRemoveAuthorResponse, GenErr> {
        println!("called ChannelRemoveAuthor in the impl code.");
        Ok(pb::ChannelRemoveAuthorResponse::default())
    }
    async fn ChannelFollowChannel(
        &self,
        param: pb::ChannelFollowChannelParam,
    ) -> Result<pb::ChannelFollowChannelResponse, GenErr> {
        println!("called ChannelFollowChannel in the impl code.");
        Ok(pb::ChannelFollowChannelResponse::default())
    }
    async fn ChannelUnFollowChannel(
        &self,
        param: pb::ChannelUnFollowChannelParam,
    ) -> Result<pb::ChannelUnFollowChannelResponse, GenErr> {
        println!("called ChannelUnFollowChannel in the impl code.");
        Ok(pb::ChannelUnFollowChannelResponse::default())
    }
    async fn ChannelRemoveFollowers(
        &self,
        param: pb::ChannelRemoveFollowersParam,
    ) -> Result<pb::ChannelRemoveFollowersResponse, GenErr> {
        println!("called ChannelRemoveFollowers in the impl code.");
        Ok(pb::ChannelRemoveFollowersResponse::default())
    }
    async fn ChannelSubscribe(
        &self,
        param: pb::ChannelSubscribeParam,
    ) -> Result<pb::ChannelSubscribeResponse, GenErr> {
        println!("called ChannelSubscribe in the impl code.");
        Ok(pb::ChannelSubscribeResponse::default())
    }
    async fn ChannelUnSubscribe(
        &self,
        param: pb::ChannelUnSubscribeParam,
    ) -> Result<pb::ChannelUnSubscribeResponse, GenErr> {
        println!("called ChannelUnSubscribe in the impl code.");
        Ok(pb::ChannelUnSubscribeResponse::default())
    }
    async fn ChannelRemoveSubscribers(
        &self,
        param: pb::ChannelRemoveSubscribersParam,
    ) -> Result<pb::ChannelRemoveSubscribersResponse, GenErr> {
        println!("called ChannelRemoveSubscribers in the impl code.");
        Ok(pb::ChannelRemoveSubscribersResponse::default())
    }
    async fn ChannelChangePrivacy(
        &self,
        param: pb::ChannelChangePrivacyParam,
    ) -> Result<pb::ChannelChangePrivacyResponse, GenErr> {
        println!("called ChannelChangePrivacy in the impl code.");
        Ok(pb::ChannelChangePrivacyResponse::default())
    }
    async fn ChannelChangeDefaultPermission(
        &self,
        param: pb::ChannelChangeDefaultPermissionParam,
    ) -> Result<pb::ChannelChangeDefaultPermissionResponse, GenErr> {
        println!("called ChannelChangeDefaultPermission in the impl code.");
        Ok(pb::ChannelChangeDefaultPermissionResponse::default())
    }
    async fn ChannelRevokeLink(
        &self,
        param: pb::ChannelRevokeLinkParam,
    ) -> Result<pb::ChannelRevokeLinkResponse, GenErr> {
        println!("called ChannelRevokeLink in the impl code.");
        Ok(pb::ChannelRevokeLinkResponse::default())
    }
    async fn ChannelChangeUsername(
        &self,
        param: pb::ChannelChangeUsernameParam,
    ) -> Result<pb::ChannelChangeUsernameResponse, GenErr> {
        println!("called ChannelChangeUsername in the impl code.");
        Ok(pb::ChannelChangeUsernameResponse::default())
    }
    async fn ChannelBlockChannel(
        &self,
        param: pb::ChannelBlockChannelParam,
    ) -> Result<pb::ChannelBlockChannelResponse, GenErr> {
        println!("called ChannelBlockChannel in the impl code.");
        Ok(pb::ChannelBlockChannelResponse::default())
    }
    async fn ChannelSendMessage(
        &self,
        param: pb::ChannelSendMessageParam,
    ) -> Result<pb::ChannelSendMessageResponse, GenErr> {
        println!("called ChannelSendMessage in the impl code.");
        Ok(pb::ChannelSendMessageResponse::default())
    }
    async fn ChannelEditMessage(
        &self,
        param: pb::ChannelEditMessageParam,
    ) -> Result<pb::ChannelEditMessageResponse, GenErr> {
        println!("called ChannelEditMessage in the impl code.");
        Ok(pb::ChannelEditMessageResponse::default())
    }
    async fn ChannelPinMessage(
        &self,
        param: pb::ChannelPinMessageParam,
    ) -> Result<pb::ChannelPinMessageResponse, GenErr> {
        println!("called ChannelPinMessage in the impl code.");
        Ok(pb::ChannelPinMessageResponse::default())
    }
    async fn ChannelUnPinMessage(
        &self,
        param: pb::ChannelUnPinMessageParam,
    ) -> Result<pb::ChannelUnPinMessageResponse, GenErr> {
        println!("called ChannelUnPinMessage in the impl code.");
        Ok(pb::ChannelUnPinMessageResponse::default())
    }
    async fn ChannelDeleteMessage(
        &self,
        param: pb::ChannelDeleteMessageParam,
    ) -> Result<pb::ChannelDeleteMessageResponse, GenErr> {
        println!("called ChannelDeleteMessage in the impl code.");
        Ok(pb::ChannelDeleteMessageResponse::default())
    }
    async fn ChannelDeleteMessages(
        &self,
        param: pb::ChannelDeleteMessagesParam,
    ) -> Result<pb::ChannelDeleteMessagesResponse, GenErr> {
        println!("called ChannelDeleteMessages in the impl code.");
        Ok(pb::ChannelDeleteMessagesResponse::default())
    }
    async fn ChannelClearHistory(
        &self,
        param: pb::ChannelClearHistoryParam,
    ) -> Result<pb::ChannelClearHistoryResponse, GenErr> {
        println!("called ChannelClearHistory in the impl code.");
        Ok(pb::ChannelClearHistoryResponse::default())
    }
    async fn ChannelAvatarAdd(
        &self,
        param: pb::ChannelAvatarAddParam,
    ) -> Result<pb::ChannelAvatarAddResponse, GenErr> {
        println!("called ChannelAvatarAdd in the impl code.");
        Ok(pb::ChannelAvatarAddResponse::default())
    }
    async fn ChannelAvatarChange(
        &self,
        param: pb::ChannelAvatarChangeParam,
    ) -> Result<pb::ChannelAvatarChangeResponse, GenErr> {
        println!("called ChannelAvatarChange in the impl code.");
        Ok(pb::ChannelAvatarChangeResponse::default())
    }
    async fn ChannelAvatarDelete(
        &self,
        param: pb::ChannelAvatarDeleteParam,
    ) -> Result<pb::ChannelAvatarDeleteResponse, GenErr> {
        println!("called ChannelAvatarDelete in the impl code.");
        Ok(pb::ChannelAvatarDeleteResponse::default())
    }
    async fn ChannelAvatarGetList(
        &self,
        param: pb::ChannelAvatarGetListParam,
    ) -> Result<pb::ChannelAvatarGetListResponse, GenErr> {
        println!("called ChannelAvatarGetList in the impl code.");
        Ok(pb::ChannelAvatarGetListResponse::default())
    }
    async fn ChannelSendDoingAction(
        &self,
        param: pb::ChannelSendDoingActionParam,
    ) -> Result<pb::ChannelSendDoingActionResponse, GenErr> {
        println!("called ChannelSendDoingAction in the impl code.");
        Ok(pb::ChannelSendDoingActionResponse::default())
    }
    async fn ChannelReportChannel(
        &self,
        param: pb::ChannelReportChannelParam,
    ) -> Result<pb::ChannelReportChannelResponse, GenErr> {
        println!("called ChannelReportChannel in the impl code.");
        Ok(pb::ChannelReportChannelResponse::default())
    }
    async fn ChannelReportMessage(
        &self,
        param: pb::ChannelReportMessageParam,
    ) -> Result<pb::ChannelReportMessageResponse, GenErr> {
        println!("called ChannelReportMessage in the impl code.");
        Ok(pb::ChannelReportMessageResponse::default())
    }
    async fn ChannelGetFull(
        &self,
        param: pb::ChannelGetFullParam,
    ) -> Result<pb::ChannelGetFullResponse, GenErr> {
        println!("called ChannelGetFull in the impl code.");
        Ok(pb::ChannelGetFullResponse::default())
    }
    async fn ChannelGetMessagesList(
        &self,
        param: pb::ChannelGetMessagesListParam,
    ) -> Result<pb::ChannelGetMessagesListResponse, GenErr> {
        println!("called ChannelGetMessagesList in the impl code.");
        Ok(pb::ChannelGetMessagesListResponse::default())
    }
    async fn ChannelGetMediaList(
        &self,
        param: pb::ChannelGetMediaListParam,
    ) -> Result<pb::ChannelGetMediaListResponse, GenErr> {
        println!("called ChannelGetMediaList in the impl code.");
        Ok(pb::ChannelGetMediaListResponse::default())
    }
    async fn ChannelGetAuthors(
        &self,
        param: pb::ChannelGetAuthorsParam,
    ) -> Result<pb::ChannelGetAuthorsResponse, GenErr> {
        println!("called ChannelGetAuthors in the impl code.");
        Ok(pb::ChannelGetAuthorsResponse::default())
    }
    async fn ChannelGetFollowers(
        &self,
        param: pb::ChannelGetFollowersParam,
    ) -> Result<pb::ChannelGetFollowersResponse, GenErr> {
        println!("called ChannelGetFollowers in the impl code.");
        Ok(pb::ChannelGetFollowersResponse::default())
    }
    async fn ChannelGetFollowings(
        &self,
        param: pb::ChannelGetFollowingsParam,
    ) -> Result<pb::ChannelGetFollowingsResponse, GenErr> {
        println!("called ChannelGetFollowings in the impl code.");
        Ok(pb::ChannelGetFollowingsResponse::default())
    }
    async fn ChannelGetSubscribers(
        &self,
        param: pb::ChannelGetSubscribersParam,
    ) -> Result<pb::ChannelGetSubscribersResponse, GenErr> {
        println!("called ChannelGetSubscribers in the impl code.");
        Ok(pb::ChannelGetSubscribersResponse::default())
    }
    async fn ChannelBlocked(
        &self,
        param: pb::ChannelBlockedParam,
    ) -> Result<pb::ChannelBlockedResponse, GenErr> {
        println!("called ChannelBlocked in the impl code.");
        Ok(pb::ChannelBlockedResponse::default())
    }
    async fn ChannelSetDraft(
        &self,
        param: pb::ChannelSetDraftParam,
    ) -> Result<pb::ChannelSetDraftResponse, GenErr> {
        println!("called ChannelSetDraft in the impl code.");
        Ok(pb::ChannelSetDraftResponse::default())
    }
}
#[async_trait]
impl RPC_Chat_Handler2 for _RRR_ {
    async fn ChatSendMessage(
        &self,
        param: pb::ChatSendMessageParam,
    ) -> Result<pb::ChatSendMessageResponse, GenErr> {
        println!("called ChatSendMessage in the impl code.");
        Ok(pb::ChatSendMessageResponse::default())
    }
    async fn ChatEditMessage(
        &self,
        param: pb::ChatEditMessageParam,
    ) -> Result<pb::ChatEditMessageResponse, GenErr> {
        println!("called ChatEditMessage in the impl code.");
        Ok(pb::ChatEditMessageResponse::default())
    }
    async fn ChatDeleteMessages(
        &self,
        param: pb::ChatDeleteMessagesParam,
    ) -> Result<pb::ChatDeleteMessagesResponse, GenErr> {
        println!("called ChatDeleteMessages in the impl code.");
        Ok(pb::ChatDeleteMessagesResponse::default())
    }
    async fn ChatDeleteHistory(
        &self,
        param: pb::ChatDeleteHistoryParam,
    ) -> Result<pb::ChatDeleteHistoryResponse, GenErr> {
        println!("called ChatDeleteHistory in the impl code.");
        Ok(pb::ChatDeleteHistoryResponse::default())
    }
    async fn ChatSendDoingAction(
        &self,
        param: pb::ChatSendDoingActionParam,
    ) -> Result<pb::ChatSendDoingActionResponse, GenErr> {
        println!("called ChatSendDoingAction in the impl code.");
        Ok(pb::ChatSendDoingActionResponse::default())
    }
    async fn ChatReportChat(
        &self,
        param: pb::ChatReportChatParam,
    ) -> Result<pb::ChatReportChatResponse, GenErr> {
        println!("called ChatReportChat in the impl code.");
        Ok(pb::ChatReportChatResponse::default())
    }
    async fn ChatGetFull(
        &self,
        param: pb::ChatGetFullMessageParam,
    ) -> Result<pb::ChatGetFullMessageResponse, GenErr> {
        println!("called ChatGetFull in the impl code.");
        Ok(pb::ChatGetFullMessageResponse::default())
    }
    async fn ChatGetMessagesList(
        &self,
        param: pb::ChatGetMessagesListParam,
    ) -> Result<pb::ChatGetMessagesListResponse, GenErr> {
        println!("called ChatGetMessagesList in the impl code.");
        Ok(pb::ChatGetMessagesListResponse::default())
    }
    async fn ChatGetMediaList(
        &self,
        param: pb::ChatGetMediaListParam,
    ) -> Result<pb::ChatGetMediaListResponse, GenErr> {
        println!("called ChatGetMediaList in the impl code.");
        Ok(pb::ChatGetMediaListResponse::default())
    }
}
#[async_trait]
impl RPC_Direct_Handler2 for _RRR_ {
    async fn DirectDeleteDirect(
        &self,
        param: pb::DirectDeleteDirectParam,
    ) -> Result<pb::DirectDeleteDirectResponse, GenErr> {
        println!("called DirectDeleteDirect in the impl code.");
        Ok(pb::DirectDeleteDirectResponse::default())
    }
    async fn DirectChangeTitle(
        &self,
        param: pb::DirectChangeTitleParam,
    ) -> Result<pb::DirectChangeTitleResponse, GenErr> {
        println!("called DirectChangeTitle in the impl code.");
        Ok(pb::DirectChangeTitleResponse::default())
    }
    async fn DirectSetCustomNotification(
        &self,
        param: pb::DirectSetCustomNotificationParam,
    ) -> Result<pb::DirectSetCustomNotificationResponse, GenErr> {
        println!("called DirectSetCustomNotification in the impl code.");
        Ok(pb::DirectSetCustomNotificationResponse::default())
    }
    async fn DirectSendActionDoing(
        &self,
        param: pb::DirectSendActionDoingParam,
    ) -> Result<pb::DirectSendActionDoingResponse, GenErr> {
        println!("called DirectSendActionDoing in the impl code.");
        Ok(pb::DirectSendActionDoingResponse::default())
    }
    async fn DirectSetDraft(
        &self,
        param: pb::DirectSetDraftParam,
    ) -> Result<pb::DirectSetDraftResponse, GenErr> {
        println!("called DirectSetDraft in the impl code.");
        Ok(pb::DirectSetDraftResponse::default())
    }
    async fn DirectDeleteDirects(
        &self,
        param: pb::DirectDeleteDirectsParam,
    ) -> Result<pb::DirectDeleteDirectsResponse, GenErr> {
        println!("called DirectDeleteDirects in the impl code.");
        Ok(pb::DirectDeleteDirectsResponse::default())
    }
    async fn DirectMarkAsRead(
        &self,
        param: pb::DirectMarkAsReadParam,
    ) -> Result<pb::DirectMarkAsReadResponse, GenErr> {
        println!("called DirectMarkAsRead in the impl code.");
        Ok(pb::DirectMarkAsReadResponse::default())
    }
    async fn DirectMarkAsUnRead(
        &self,
        param: pb::DirectMarkAsUnReadParam,
    ) -> Result<pb::DirectMarkAsUnReadResponse, GenErr> {
        println!("called DirectMarkAsUnRead in the impl code.");
        Ok(pb::DirectMarkAsUnReadResponse::default())
    }
    async fn DirectPinDirects(
        &self,
        param: pb::DirectPinDirectsParam,
    ) -> Result<pb::DirectPinDirectsResponse, GenErr> {
        println!("called DirectPinDirects in the impl code.");
        Ok(pb::DirectPinDirectsResponse::default())
    }
    async fn DirectUnPinDirects(
        &self,
        param: pb::DirectUnPinDirectsParam,
    ) -> Result<pb::DirectUnPinDirectsResponse, GenErr> {
        println!("called DirectUnPinDirects in the impl code.");
        Ok(pb::DirectUnPinDirectsResponse::default())
    }
    async fn DirectArchiveDirects(
        &self,
        param: pb::DirectArchiveDirectsParam,
    ) -> Result<pb::DirectArchiveDirectsResponse, GenErr> {
        println!("called DirectArchiveDirects in the impl code.");
        Ok(pb::DirectArchiveDirectsResponse::default())
    }
    async fn DirectUnArchiveDirects(
        &self,
        param: pb::DirectUnArchiveDirectsParam,
    ) -> Result<pb::DirectUnArchiveDirectsResponse, GenErr> {
        println!("called DirectUnArchiveDirects in the impl code.");
        Ok(pb::DirectUnArchiveDirectsResponse::default())
    }
    async fn DirectClearHistories(
        &self,
        param: pb::DirectClearHistoriesParam,
    ) -> Result<pb::DirectClearHistoriesResponse, GenErr> {
        println!("called DirectClearHistories in the impl code.");
        Ok(pb::DirectClearHistoriesResponse::default())
    }
    async fn DirectMuteDirects(
        &self,
        param: pb::DirectMuteDirectsParam,
    ) -> Result<pb::DirectMuteDirectsResponse, GenErr> {
        println!("called DirectMuteDirects in the impl code.");
        Ok(pb::DirectMuteDirectsResponse::default())
    }
    async fn DirectUnMuteDirects(
        &self,
        param: pb::DirectUnMuteDirectsParam,
    ) -> Result<pb::DirectUnMuteDirectsResponse, GenErr> {
        println!("called DirectUnMuteDirects in the impl code.");
        Ok(pb::DirectUnMuteDirectsResponse::default())
    }
    async fn DirectCreateFolder(
        &self,
        param: pb::DirectCreateFolderParam,
    ) -> Result<pb::DirectCreateFolderResponse, GenErr> {
        println!("called DirectCreateFolder in the impl code.");
        Ok(pb::DirectCreateFolderResponse::default())
    }
    async fn DirectChangeFolder(
        &self,
        param: pb::DirectChangeFolderParam,
    ) -> Result<pb::DirectChangeFolderResponse, GenErr> {
        println!("called DirectChangeFolder in the impl code.");
        Ok(pb::DirectChangeFolderResponse::default())
    }
    async fn DirectRemoveFromFolder(
        &self,
        param: pb::DirectRemoveFromFolderParam,
    ) -> Result<pb::DirectRemoveFromFolderResponse, GenErr> {
        println!("called DirectRemoveFromFolder in the impl code.");
        Ok(pb::DirectRemoveFromFolderResponse::default())
    }
    async fn DirectReordersFolder(
        &self,
        param: pb::DirectReordersFolderParam,
    ) -> Result<pb::DirectReordersFolderResponse, GenErr> {
        println!("called DirectReordersFolder in the impl code.");
        Ok(pb::DirectReordersFolderResponse::default())
    }
    async fn DirectDeleteFolder(
        &self,
        param: pb::DirectDeleteFolderParam,
    ) -> Result<pb::DirectDeleteFolderResponse, GenErr> {
        println!("called DirectDeleteFolder in the impl code.");
        Ok(pb::DirectDeleteFolderResponse::default())
    }
    async fn DirectGetChatsList(
        &self,
        param: pb::DirectGetChatsListParam,
    ) -> Result<pb::DirectGetChatsListResponse, GenErr> {
        println!("called DirectGetChatsList in the impl code.");
        Ok(pb::DirectGetChatsListResponse::default())
    }
    async fn DirectGetGroupsList(
        &self,
        param: pb::DirectGetGroupsListParam,
    ) -> Result<pb::DirectGetGroupsListResponse, GenErr> {
        println!("called DirectGetGroupsList in the impl code.");
        Ok(pb::DirectGetGroupsListResponse::default())
    }
    async fn DirectGetChannelsList(
        &self,
        param: pb::DirectGetChannelsListParam,
    ) -> Result<pb::DirectGetChannelsListResponse, GenErr> {
        println!("called DirectGetChannelsList in the impl code.");
        Ok(pb::DirectGetChannelsListResponse::default())
    }
    async fn DirectGetFoldersList(
        &self,
        param: pb::DirectGetFoldersListParam,
    ) -> Result<pb::DirectGetFoldersListResponse, GenErr> {
        println!("called DirectGetFoldersList in the impl code.");
        Ok(pb::DirectGetFoldersListResponse::default())
    }
    async fn DirectGetFoldersFullList(
        &self,
        param: pb::DirectGetFoldersFullListParam,
    ) -> Result<pb::DirectGetFoldersFullListResponse, GenErr> {
        println!("called DirectGetFoldersFullList in the impl code.");
        Ok(pb::DirectGetFoldersFullListResponse::default())
    }
}
#[async_trait]
impl RPC_Group_Handler2 for _RRR_ {
    async fn GroupCreateGroup(
        &self,
        param: pb::GroupCreateGroupParam,
    ) -> Result<pb::GroupCreateGroupResponse, GenErr> {
        println!("called GroupCreateGroup in the impl code.");
        Ok(pb::GroupCreateGroupResponse::default())
    }
    async fn GroupEditGroup(
        &self,
        param: pb::GroupEditGroupParam,
    ) -> Result<pb::GroupEditGroupResponse, GenErr> {
        println!("called GroupEditGroup in the impl code.");
        Ok(pb::GroupEditGroupResponse::default())
    }
    async fn GroupDeleteGroup(
        &self,
        param: pb::GroupDeleteGroupParam,
    ) -> Result<pb::GroupDeleteGroupResponse, GenErr> {
        println!("called GroupDeleteGroup in the impl code.");
        Ok(pb::GroupDeleteGroupResponse::default())
    }
    async fn GroupAddAdmin(
        &self,
        param: pb::GroupAddAdminParam,
    ) -> Result<pb::GroupAddAdminResponse, GenErr> {
        println!("called GroupAddAdmin in the impl code.");
        Ok(pb::GroupAddAdminResponse::default())
    }
    async fn GroupAddMember(
        &self,
        param: pb::GroupAddMemberParam,
    ) -> Result<pb::GroupAddMemberResponse, GenErr> {
        println!("called GroupAddMember in the impl code.");
        Ok(pb::GroupAddMemberResponse::default())
    }
    async fn GroupRemoveMember(
        &self,
        param: pb::GroupRemoveMemberParam,
    ) -> Result<pb::GroupRemoveMemberResponse, GenErr> {
        println!("called GroupRemoveMember in the impl code.");
        Ok(pb::GroupRemoveMemberResponse::default())
    }
    async fn GroupChangeMemberLevel(
        &self,
        param: pb::GroupChangeMemberLevelParam,
    ) -> Result<pb::GroupChangeMemberLevelResponse, GenErr> {
        println!("called GroupChangeMemberLevel in the impl code.");
        Ok(pb::GroupChangeMemberLevelResponse::default())
    }
    async fn GroupChangeMemberPermission(
        &self,
        param: pb::GroupChangeMemberPermissionParam,
    ) -> Result<pb::GroupChangeMemberPermissionResponse, GenErr> {
        println!("called GroupChangeMemberPermission in the impl code.");
        Ok(pb::GroupChangeMemberPermissionResponse::default())
    }
    async fn GroupJoinGroup(
        &self,
        param: pb::JoinGroupParam,
    ) -> Result<pb::JoinGroupResponse, GenErr> {
        println!("called GroupJoinGroup in the impl code.");
        Ok(pb::JoinGroupResponse::default())
    }
    async fn GroupLeaveGroup(
        &self,
        param: pb::GroupLeaveGroupParam,
    ) -> Result<pb::GroupLeaveGroupResponse, GenErr> {
        println!("called GroupLeaveGroup in the impl code.");
        Ok(pb::GroupLeaveGroupResponse::default())
    }
    async fn GroupBanMember(
        &self,
        param: pb::GroupBanMemberParam,
    ) -> Result<pb::GroupBanMemberResponse, GenErr> {
        println!("called GroupBanMember in the impl code.");
        Ok(pb::GroupBanMemberResponse::default())
    }
    async fn GroupChangePrivacy(
        &self,
        param: pb::GroupChangePrivacyParam,
    ) -> Result<pb::GroupChangePrivacyResponse, GenErr> {
        println!("called GroupChangePrivacy in the impl code.");
        Ok(pb::GroupChangePrivacyResponse::default())
    }
    async fn GroupChangeDefaultPermission(
        &self,
        param: pb::GroupChangeDefaultPermissionParam,
    ) -> Result<pb::GroupChangeDefaultPermissionResponse, GenErr> {
        println!("called GroupChangeDefaultPermission in the impl code.");
        Ok(pb::GroupChangeDefaultPermissionResponse::default())
    }
    async fn GroupRevokeLink(
        &self,
        param: pb::GroupRevokeLinkParam,
    ) -> Result<pb::GroupRevokeLinkResponse, GenErr> {
        println!("called GroupRevokeLink in the impl code.");
        Ok(pb::GroupRevokeLinkResponse::default())
    }
    async fn GroupChangeUsername(
        &self,
        param: pb::GroupChangeUsernameParam,
    ) -> Result<pb::GroupChangeUsernameResponse, GenErr> {
        println!("called GroupChangeUsername in the impl code.");
        Ok(pb::GroupChangeUsernameResponse::default())
    }
    async fn GroupSendMessage(
        &self,
        param: pb::GroupSendMessageParam,
    ) -> Result<pb::GroupSendMessageResponse, GenErr> {
        println!("called GroupSendMessage in the impl code.");
        Ok(pb::GroupSendMessageResponse::default())
    }
    async fn GroupEditMessage(
        &self,
        param: pb::GroupEditMessageParam,
    ) -> Result<pb::GroupEditMessageResponse, GenErr> {
        println!("called GroupEditMessage in the impl code.");
        Ok(pb::GroupEditMessageResponse::default())
    }
    async fn GroupPinMessage(
        &self,
        param: pb::GroupPinMessageParam,
    ) -> Result<pb::GroupPinMessageResponse, GenErr> {
        println!("called GroupPinMessage in the impl code.");
        Ok(pb::GroupPinMessageResponse::default())
    }
    async fn GroupUnPinMessage(
        &self,
        param: pb::GroupUnPinMessageParam,
    ) -> Result<pb::GroupUnPinMessageResponse, GenErr> {
        println!("called GroupUnPinMessage in the impl code.");
        Ok(pb::GroupUnPinMessageResponse::default())
    }
    async fn GroupDeleteMessage(
        &self,
        param: pb::GroupDeleteMessageParam,
    ) -> Result<pb::GroupDeleteMessageResponse, GenErr> {
        println!("called GroupDeleteMessage in the impl code.");
        Ok(pb::GroupDeleteMessageResponse::default())
    }
    async fn GroupDeleteMessages(
        &self,
        param: pb::GroupDeleteMessagesParam,
    ) -> Result<pb::GroupDeleteMessagesResponse, GenErr> {
        println!("called GroupDeleteMessages in the impl code.");
        Ok(pb::GroupDeleteMessagesResponse::default())
    }
    async fn GroupDeleteHistory(
        &self,
        param: pb::GroupDeleteHistoryParam,
    ) -> Result<pb::GroupDeleteHistoryResponse, GenErr> {
        println!("called GroupDeleteHistory in the impl code.");
        Ok(pb::GroupDeleteHistoryResponse::default())
    }
    async fn GroupClearHistory(
        &self,
        param: pb::GroupClearHistoryParam,
    ) -> Result<pb::GroupClearHistoryResponse, GenErr> {
        println!("called GroupClearHistory in the impl code.");
        Ok(pb::GroupClearHistoryResponse::default())
    }
    async fn GroupAvatarAdd(
        &self,
        param: pb::GroupAvatarAddParam,
    ) -> Result<pb::GroupAvatarAddResponse, GenErr> {
        println!("called GroupAvatarAdd in the impl code.");
        Ok(pb::GroupAvatarAddResponse::default())
    }
    async fn GroupAvatarChange(
        &self,
        param: pb::GroupAvatarChangeParam,
    ) -> Result<pb::GroupAvatarChangeResponse, GenErr> {
        println!("called GroupAvatarChange in the impl code.");
        Ok(pb::GroupAvatarChangeResponse::default())
    }
    async fn GroupAvatarDelete(
        &self,
        param: pb::GroupAvatarDeleteParam,
    ) -> Result<pb::GroupAvatarDeleteResponse, GenErr> {
        println!("called GroupAvatarDelete in the impl code.");
        Ok(pb::GroupAvatarDeleteResponse::default())
    }
    async fn GroupAvatarGetList(
        &self,
        param: pb::GroupAvatarGetListParam,
    ) -> Result<pb::GroupAvatarGetListResponse, GenErr> {
        println!("called GroupAvatarGetList in the impl code.");
        Ok(pb::GroupAvatarGetListResponse::default())
    }
    async fn GroupSendDoingAction(
        &self,
        param: pb::GroupSendDoingActionParam,
    ) -> Result<pb::GroupSendDoingActionResponse, GenErr> {
        println!("called GroupSendDoingAction in the impl code.");
        Ok(pb::GroupSendDoingActionResponse::default())
    }
    async fn GroupReportGroup(
        &self,
        param: pb::GroupReportGroupParam,
    ) -> Result<pb::GroupReportGroupResponse, GenErr> {
        println!("called GroupReportGroup in the impl code.");
        Ok(pb::GroupReportGroupResponse::default())
    }
    async fn GroupGetFull(
        &self,
        param: pb::GroupGetFullMessageParam,
    ) -> Result<pb::GroupGetFullMessageResponse, GenErr> {
        println!("called GroupGetFull in the impl code.");
        Ok(pb::GroupGetFullMessageResponse::default())
    }
    async fn GroupGetMessagesList(
        &self,
        param: pb::GroupGetMessagesListParam,
    ) -> Result<pb::GroupGetMessagesListResponse, GenErr> {
        println!("called GroupGetMessagesList in the impl code.");
        Ok(pb::GroupGetMessagesListResponse::default())
    }
    async fn GroupGetMediaList(
        &self,
        param: pb::GroupGetMediaListParam,
    ) -> Result<pb::GroupGetMediaListResponse, GenErr> {
        println!("called GroupGetMediaList in the impl code.");
        Ok(pb::GroupGetMediaListResponse::default())
    }
    async fn GroupGetMembersList(
        &self,
        param: pb::GroupGetMembersListParam,
    ) -> Result<pb::GroupGetMembersListResponse, GenErr> {
        println!("called GroupGetMembersList in the impl code.");
        Ok(pb::GroupGetMembersListResponse::default())
    }
    async fn GroupGetAdminsList(
        &self,
        param: pb::GroupGetAdminsListParam,
    ) -> Result<pb::GroupGetAdminsListResponse, GenErr> {
        println!("called GroupGetAdminsList in the impl code.");
        Ok(pb::GroupGetAdminsListResponse::default())
    }
    async fn GroupSetDraft(
        &self,
        param: pb::GroupSetDraftParam,
    ) -> Result<pb::GroupSetDraftResponse, GenErr> {
        println!("called GroupSetDraft in the impl code.");
        Ok(pb::GroupSetDraftResponse::default())
    }
}
#[async_trait]
impl RPC_Sample_Handler2 for _RRR_ {
    async fn GetUsers1(&self, param: pb::GetUsers1Param) -> Result<pb::GetUsers1Response, GenErr> {
        println!("called GetUsers1 in the impl code.");
        Ok(pb::GetUsers1Response::default())
    }
    async fn GetProfiles(
        &self,
        param: pb::GetProfilesParam,
    ) -> Result<pb::GetProfilesResponse, GenErr> {
        println!("called GetProfiles in the impl code.");
        Ok(pb::GetProfilesResponse::default())
    }
    async fn GetChannels(
        &self,
        param: pb::GetChannelsParam,
    ) -> Result<pb::GetChannelsResponse, GenErr> {
        println!("called GetChannels in the impl code.");
        Ok(pb::GetChannelsResponse::default())
    }
    async fn GetDirects(
        &self,
        param: pb::GetDirectsParam,
    ) -> Result<pb::GetDirectsResponse, GenErr> {
        println!("called GetDirects in the impl code.");
        Ok(pb::GetDirectsResponse::default())
    }
    async fn GetMessages(
        &self,
        param: pb::GetMessagesParam,
    ) -> Result<pb::GetMessagesResponse, GenErr> {
        println!("called GetMessages in the impl code.");
        Ok(pb::GetMessagesResponse::default())
    }
}
#[async_trait]
impl RPC_Shared_Handler2 for _RRR_ {
    async fn Echo(&self, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        println!("called Echo in the impl code.");
        Ok(pb::EchoResponse::default())
    }
    async fn CheckUserName(
        &self,
        param: pb::CheckUserNameParam,
    ) -> Result<pb::CheckUserNameResponse, GenErr> {
        println!("called CheckUserName in the impl code.");
        Ok(pb::CheckUserNameResponse::default())
    }
}
#[async_trait]
impl RPC_Upload_Handler2 for _RRR_ {
    async fn UploadFile(
        &self,
        param: pb::UploadFileParam,
    ) -> Result<pb::UploadFileResponse, GenErr> {
        println!("called UploadFile in the impl code.");
        Ok(pb::UploadFileResponse::default())
    }
}
#[async_trait]
impl RPC_User_Handler2 for _RRR_ {
    async fn ChangePhoneNumber(
        &self,
        param: pb::ChangePhoneNumberParam,
    ) -> Result<pb::ChangePhoneNumberResponse, GenErr> {
        println!("called ChangePhoneNumber in the impl code.");
        Ok(pb::ChangePhoneNumberResponse::default())
    }
}

// DEAD CODE
/*
///////////////////////////////// Rpc Client ///////////////////////
#[derive(Debug)]
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
    pub async fn SendConfirmCode (&self, param: pb::SendConfirmCodeParam)
        -> Result<pb::SendConfirmCodeResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ConfirmCode (&self, param: pb::ConfirmCodeParam)
        -> Result<pb::ConfirmCodeResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn SingUp (&self, param: pb::SingUpParam)
        -> Result<pb::SingUpResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn SingIn (&self, param: pb::SingInParam)
        -> Result<pb::SingInResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn LogOut (&self, param: pb::LogOutParam)
        -> Result<pb::LogOutResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

// service: RPC_Channel
    pub async fn ChannelCreateChannel (&self, param: pb::ChannelCreateChannelParam)
        -> Result<pb::ChannelCreateChannelResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelEditChannel (&self, param: pb::ChannelEditChannelParam)
        -> Result<pb::ChannelEditChannelResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelDeleteChannel (&self, param: pb::ChannelDeleteChannelParam)
        -> Result<pb::ChannelDeleteChannelResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelAddAuthor (&self, param: pb::ChannelAddAuthorParam)
        -> Result<pb::ChannelAddAuthorResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelChangeAuthorPermission (&self, param: pb::ChannelChangeAuthorPermissionParam)
        -> Result<pb::ChannelChangeAuthorPermissionResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelRemoveAuthor (&self, param: pb::ChannelRemoveAuthorParam)
        -> Result<pb::ChannelRemoveAuthorResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelFollowChannel (&self, param: pb::ChannelFollowChannelParam)
        -> Result<pb::ChannelFollowChannelResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelUnFollowChannel (&self, param: pb::ChannelUnFollowChannelParam)
        -> Result<pb::ChannelUnFollowChannelResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelRemoveFollowers (&self, param: pb::ChannelRemoveFollowersParam)
        -> Result<pb::ChannelRemoveFollowersResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelSubscribe (&self, param: pb::ChannelSubscribeParam)
        -> Result<pb::ChannelSubscribeResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelUnSubscribe (&self, param: pb::ChannelUnSubscribeParam)
        -> Result<pb::ChannelUnSubscribeResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelRemoveSubscribers (&self, param: pb::ChannelRemoveSubscribersParam)
        -> Result<pb::ChannelRemoveSubscribersResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelChangePrivacy (&self, param: pb::ChannelChangePrivacyParam)
        -> Result<pb::ChannelChangePrivacyResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelChangeDefaultPermission (&self, param: pb::ChannelChangeDefaultPermissionParam)
        -> Result<pb::ChannelChangeDefaultPermissionResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelRevokeLink (&self, param: pb::ChannelRevokeLinkParam)
        -> Result<pb::ChannelRevokeLinkResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelChangeUsername (&self, param: pb::ChannelChangeUsernameParam)
        -> Result<pb::ChannelChangeUsernameResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelBlockChannel (&self, param: pb::ChannelBlockChannelParam)
        -> Result<pb::ChannelBlockChannelResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelSendMessage (&self, param: pb::ChannelSendMessageParam)
        -> Result<pb::ChannelSendMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelEditMessage (&self, param: pb::ChannelEditMessageParam)
        -> Result<pb::ChannelEditMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelPinMessage (&self, param: pb::ChannelPinMessageParam)
        -> Result<pb::ChannelPinMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelUnPinMessage (&self, param: pb::ChannelUnPinMessageParam)
        -> Result<pb::ChannelUnPinMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelDeleteMessage (&self, param: pb::ChannelDeleteMessageParam)
        -> Result<pb::ChannelDeleteMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelDeleteMessages (&self, param: pb::ChannelDeleteMessagesParam)
        -> Result<pb::ChannelDeleteMessagesResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelClearHistory (&self, param: pb::ChannelClearHistoryParam)
        -> Result<pb::ChannelClearHistoryResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelAvatarAdd (&self, param: pb::ChannelAvatarAddParam)
        -> Result<pb::ChannelAvatarAddResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelAvatarChange (&self, param: pb::ChannelAvatarChangeParam)
        -> Result<pb::ChannelAvatarChangeResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelAvatarDelete (&self, param: pb::ChannelAvatarDeleteParam)
        -> Result<pb::ChannelAvatarDeleteResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelAvatarGetList (&self, param: pb::ChannelAvatarGetListParam)
        -> Result<pb::ChannelAvatarGetListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelSendDoingAction (&self, param: pb::ChannelSendDoingActionParam)
        -> Result<pb::ChannelSendDoingActionResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelReportChannel (&self, param: pb::ChannelReportChannelParam)
        -> Result<pb::ChannelReportChannelResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelReportMessage (&self, param: pb::ChannelReportMessageParam)
        -> Result<pb::ChannelReportMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelGetFull (&self, param: pb::ChannelGetFullParam)
        -> Result<pb::ChannelGetFullResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelGetMessagesList (&self, param: pb::ChannelGetMessagesListParam)
        -> Result<pb::ChannelGetMessagesListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelGetMediaList (&self, param: pb::ChannelGetMediaListParam)
        -> Result<pb::ChannelGetMediaListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelGetAuthors (&self, param: pb::ChannelGetAuthorsParam)
        -> Result<pb::ChannelGetAuthorsResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelGetFollowers (&self, param: pb::ChannelGetFollowersParam)
        -> Result<pb::ChannelGetFollowersResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelGetFollowings (&self, param: pb::ChannelGetFollowingsParam)
        -> Result<pb::ChannelGetFollowingsResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelGetSubscribers (&self, param: pb::ChannelGetSubscribersParam)
        -> Result<pb::ChannelGetSubscribersResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelBlocked (&self, param: pb::ChannelBlockedParam)
        -> Result<pb::ChannelBlockedResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChannelSetDraft (&self, param: pb::ChannelSetDraftParam)
        -> Result<pb::ChannelSetDraftResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

// service: RPC_Chat
    pub async fn ChatSendMessage (&self, param: pb::ChatSendMessageParam)
        -> Result<pb::ChatSendMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChatEditMessage (&self, param: pb::ChatEditMessageParam)
        -> Result<pb::ChatEditMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChatDeleteMessages (&self, param: pb::ChatDeleteMessagesParam)
        -> Result<pb::ChatDeleteMessagesResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChatDeleteHistory (&self, param: pb::ChatDeleteHistoryParam)
        -> Result<pb::ChatDeleteHistoryResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChatSendDoingAction (&self, param: pb::ChatSendDoingActionParam)
        -> Result<pb::ChatSendDoingActionResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChatReportChat (&self, param: pb::ChatReportChatParam)
        -> Result<pb::ChatReportChatResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChatGetFull (&self, param: pb::ChatGetFullMessageParam)
        -> Result<pb::ChatGetFullMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChatGetMessagesList (&self, param: pb::ChatGetMessagesListParam)
        -> Result<pb::ChatGetMessagesListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn ChatGetMediaList (&self, param: pb::ChatGetMediaListParam)
        -> Result<pb::ChatGetMediaListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

// service: RPC_Direct
    pub async fn DirectDeleteDirect (&self, param: pb::DirectDeleteDirectParam)
        -> Result<pb::DirectDeleteDirectResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectChangeTitle (&self, param: pb::DirectChangeTitleParam)
        -> Result<pb::DirectChangeTitleResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectSetCustomNotification (&self, param: pb::DirectSetCustomNotificationParam)
        -> Result<pb::DirectSetCustomNotificationResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectSendActionDoing (&self, param: pb::DirectSendActionDoingParam)
        -> Result<pb::DirectSendActionDoingResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectSetDraft (&self, param: pb::DirectSetDraftParam)
        -> Result<pb::DirectSetDraftResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectDeleteDirects (&self, param: pb::DirectDeleteDirectsParam)
        -> Result<pb::DirectDeleteDirectsResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectMarkAsRead (&self, param: pb::DirectMarkAsReadParam)
        -> Result<pb::DirectMarkAsReadResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectMarkAsUnRead (&self, param: pb::DirectMarkAsUnReadParam)
        -> Result<pb::DirectMarkAsUnReadResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectPinDirects (&self, param: pb::DirectPinDirectsParam)
        -> Result<pb::DirectPinDirectsResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectUnPinDirects (&self, param: pb::DirectUnPinDirectsParam)
        -> Result<pb::DirectUnPinDirectsResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectArchiveDirects (&self, param: pb::DirectArchiveDirectsParam)
        -> Result<pb::DirectArchiveDirectsResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectUnArchiveDirects (&self, param: pb::DirectUnArchiveDirectsParam)
        -> Result<pb::DirectUnArchiveDirectsResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectClearHistories (&self, param: pb::DirectClearHistoriesParam)
        -> Result<pb::DirectClearHistoriesResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectMuteDirects (&self, param: pb::DirectMuteDirectsParam)
        -> Result<pb::DirectMuteDirectsResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectUnMuteDirects (&self, param: pb::DirectUnMuteDirectsParam)
        -> Result<pb::DirectUnMuteDirectsResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectCreateFolder (&self, param: pb::DirectCreateFolderParam)
        -> Result<pb::DirectCreateFolderResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectChangeFolder (&self, param: pb::DirectChangeFolderParam)
        -> Result<pb::DirectChangeFolderResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectRemoveFromFolder (&self, param: pb::DirectRemoveFromFolderParam)
        -> Result<pb::DirectRemoveFromFolderResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectReordersFolder (&self, param: pb::DirectReordersFolderParam)
        -> Result<pb::DirectReordersFolderResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectDeleteFolder (&self, param: pb::DirectDeleteFolderParam)
        -> Result<pb::DirectDeleteFolderResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectGetChatsList (&self, param: pb::DirectGetChatsListParam)
        -> Result<pb::DirectGetChatsListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectGetGroupsList (&self, param: pb::DirectGetGroupsListParam)
        -> Result<pb::DirectGetGroupsListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectGetChannelsList (&self, param: pb::DirectGetChannelsListParam)
        -> Result<pb::DirectGetChannelsListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectGetFoldersList (&self, param: pb::DirectGetFoldersListParam)
        -> Result<pb::DirectGetFoldersListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn DirectGetFoldersFullList (&self, param: pb::DirectGetFoldersFullListParam)
        -> Result<pb::DirectGetFoldersFullListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

// service: RPC_Group
    pub async fn GroupCreateGroup (&self, param: pb::GroupCreateGroupParam)
        -> Result<pb::GroupCreateGroupResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupEditGroup (&self, param: pb::GroupEditGroupParam)
        -> Result<pb::GroupEditGroupResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupDeleteGroup (&self, param: pb::GroupDeleteGroupParam)
        -> Result<pb::GroupDeleteGroupResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupAddAdmin (&self, param: pb::GroupAddAdminParam)
        -> Result<pb::GroupAddAdminResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupAddMember (&self, param: pb::GroupAddMemberParam)
        -> Result<pb::GroupAddMemberResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupRemoveMember (&self, param: pb::GroupRemoveMemberParam)
        -> Result<pb::GroupRemoveMemberResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupChangeMemberLevel (&self, param: pb::GroupChangeMemberLevelParam)
        -> Result<pb::GroupChangeMemberLevelResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupChangeMemberPermission (&self, param: pb::GroupChangeMemberPermissionParam)
        -> Result<pb::GroupChangeMemberPermissionResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupJoinGroup (&self, param: pb::JoinGroupParam)
        -> Result<pb::JoinGroupResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupLeaveGroup (&self, param: pb::GroupLeaveGroupParam)
        -> Result<pb::GroupLeaveGroupResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupBanMember (&self, param: pb::GroupBanMemberParam)
        -> Result<pb::GroupBanMemberResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupChangePrivacy (&self, param: pb::GroupChangePrivacyParam)
        -> Result<pb::GroupChangePrivacyResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupChangeDefaultPermission (&self, param: pb::GroupChangeDefaultPermissionParam)
        -> Result<pb::GroupChangeDefaultPermissionResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupRevokeLink (&self, param: pb::GroupRevokeLinkParam)
        -> Result<pb::GroupRevokeLinkResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupChangeUsername (&self, param: pb::GroupChangeUsernameParam)
        -> Result<pb::GroupChangeUsernameResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupSendMessage (&self, param: pb::GroupSendMessageParam)
        -> Result<pb::GroupSendMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupEditMessage (&self, param: pb::GroupEditMessageParam)
        -> Result<pb::GroupEditMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupPinMessage (&self, param: pb::GroupPinMessageParam)
        -> Result<pb::GroupPinMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupUnPinMessage (&self, param: pb::GroupUnPinMessageParam)
        -> Result<pb::GroupUnPinMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupDeleteMessage (&self, param: pb::GroupDeleteMessageParam)
        -> Result<pb::GroupDeleteMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupDeleteMessages (&self, param: pb::GroupDeleteMessagesParam)
        -> Result<pb::GroupDeleteMessagesResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupDeleteHistory (&self, param: pb::GroupDeleteHistoryParam)
        -> Result<pb::GroupDeleteHistoryResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupClearHistory (&self, param: pb::GroupClearHistoryParam)
        -> Result<pb::GroupClearHistoryResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupAvatarAdd (&self, param: pb::GroupAvatarAddParam)
        -> Result<pb::GroupAvatarAddResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupAvatarChange (&self, param: pb::GroupAvatarChangeParam)
        -> Result<pb::GroupAvatarChangeResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupAvatarDelete (&self, param: pb::GroupAvatarDeleteParam)
        -> Result<pb::GroupAvatarDeleteResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupAvatarGetList (&self, param: pb::GroupAvatarGetListParam)
        -> Result<pb::GroupAvatarGetListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupSendDoingAction (&self, param: pb::GroupSendDoingActionParam)
        -> Result<pb::GroupSendDoingActionResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupReportGroup (&self, param: pb::GroupReportGroupParam)
        -> Result<pb::GroupReportGroupResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupGetFull (&self, param: pb::GroupGetFullMessageParam)
        -> Result<pb::GroupGetFullMessageResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupGetMessagesList (&self, param: pb::GroupGetMessagesListParam)
        -> Result<pb::GroupGetMessagesListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupGetMediaList (&self, param: pb::GroupGetMediaListParam)
        -> Result<pb::GroupGetMediaListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupGetMembersList (&self, param: pb::GroupGetMembersListParam)
        -> Result<pb::GroupGetMembersListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupGetAdminsList (&self, param: pb::GroupGetAdminsListParam)
        -> Result<pb::GroupGetAdminsListResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GroupSetDraft (&self, param: pb::GroupSetDraftParam)
        -> Result<pb::GroupSetDraftResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

// service: RPC_Sample
    pub async fn GetUsers1 (&self, param: pb::GetUsers1Param)
        -> Result<pb::GetUsers1Response,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GetProfiles (&self, param: pb::GetProfilesParam)
        -> Result<pb::GetProfilesResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GetChannels (&self, param: pb::GetChannelsParam)
        -> Result<pb::GetChannelsResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GetDirects (&self, param: pb::GetDirectsParam)
        -> Result<pb::GetDirectsResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn GetMessages (&self, param: pb::GetMessagesParam)
        -> Result<pb::GetMessagesResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

// service: RPC_Shared
    pub async fn Echo (&self, param: pb::EchoParam)
        -> Result<pb::EchoResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

    pub async fn CheckUserName (&self, param: pb::CheckUserNameParam)
        -> Result<pb::CheckUserNameResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

// service: RPC_Upload
    pub async fn UploadFile (&self, param: pb::UploadFileParam)
        -> Result<pb::UploadFileResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

// service: RPC_User
    pub async fn ChangePhoneNumber (&self, param: pb::ChangePhoneNumberParam)
        -> Result<pb::ChangePhoneNumberResponse,GenErr>{

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
        ::prost::Message::encode(&invoke, &mut buff)?;

        let req = reqwest::Client::new()
            .post(self.endpoint)
            .body(buff)
            .send()
            .await?;

        let res_bytes = req.bytes().await?;
        let res_bytes = res_bytes.to_vec();

        let pb_res_invoke: pb::Invoke = ::prost::Message::decode(res_bytes.as_slice())?;
        let pb_res = ::prost::Message::decode(pb_res_invoke.rpc_data.as_slice())?;
        Ok(pb_res)
    }

}
*/
