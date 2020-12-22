//#![rustfmt::skip]

use crate::pb;
use crate::pb::{EchoParam, EchoResponse};
use crate::{errors::GenErr, UserParam};
use async_trait::async_trait;

use http::Version;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error as HyperError, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

pub struct RpcInvoke {
    pub method_id: i64, // correct data type should be i32,
    pub rpc_service: RpcServiceData,
}

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

pub enum RPC_Auth_MethodData {
    SendConfirmCode(pb::SendConfirmCodeParam),
    ConfirmCode(pb::ConfirmCodeParam),
    SingUp(pb::SingUpParam),
    SingIn(pb::SingInParam),
    LogOut(pb::LogOutParam),
}
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
pub enum RPC_Sample_MethodData {
    GetUsers1(pb::GetUsers1Param),
    GetProfiles(pb::GetProfilesParam),
    GetChannels(pb::GetChannelsParam),
    GetDirects(pb::GetDirectsParam),
    GetMessages(pb::GetMessagesParam),
}
pub enum RPC_Shared_MethodData {
    Echo(pb::EchoParam),
    CheckUserName(pb::CheckUserNameParam),
}
pub enum RPC_Upload_MethodData {
    UploadFile(pb::UploadFileParam),
}
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
            let rpc_param: Result<pb::SendConfirmCodeParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 939965206 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::SendConfirmCode(rpc_param)),
            }
        }

        method_ids::ConfirmCode => {
            let rpc_param: Result<pb::ConfirmCodeParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1740258084 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::ConfirmCode(rpc_param)),
            }
        }

        method_ids::SingUp => {
            let rpc_param: Result<pb::SingUpParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 291193302 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::SingUp(rpc_param)),
            }
        }

        method_ids::SingIn => {
            let rpc_param: Result<pb::SingInParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1017957090 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::SingIn(rpc_param)),
            }
        }

        method_ids::LogOut => {
            let rpc_param: Result<pb::LogOutParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1283119009 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::LogOut(rpc_param)),
            }
        }

        // RPC_Channel
        method_ids::ChannelCreateChannel => {
            let rpc_param: Result<pb::ChannelCreateChannelParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 143251225 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelCreateChannel(rpc_param)),
            }
        }

        method_ids::ChannelEditChannel => {
            let rpc_param: Result<pb::ChannelEditChannelParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 189471894 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelEditChannel(rpc_param)),
            }
        }

        method_ids::ChannelDeleteChannel => {
            let rpc_param: Result<pb::ChannelDeleteChannelParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1494483355 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelDeleteChannel(rpc_param)),
            }
        }

        method_ids::ChannelAddAuthor => {
            let rpc_param: Result<pb::ChannelAddAuthorParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 780397316 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAddAuthor(rpc_param)),
            }
        }

        method_ids::ChannelChangeAuthorPermission => {
            let rpc_param: Result<pb::ChannelChangeAuthorPermissionParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 93233821 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelChangeAuthorPermission(
                    rpc_param,
                )),
            }
        }

        method_ids::ChannelRemoveAuthor => {
            let rpc_param: Result<pb::ChannelRemoveAuthorParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 419542304 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelRemoveAuthor(rpc_param)),
            }
        }

        method_ids::ChannelFollowChannel => {
            let rpc_param: Result<pb::ChannelFollowChannelParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 744563779 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelFollowChannel(rpc_param)),
            }
        }

        method_ids::ChannelUnFollowChannel => {
            let rpc_param: Result<pb::ChannelUnFollowChannelParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 959512423 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelUnFollowChannel(rpc_param)),
            }
        }

        method_ids::ChannelRemoveFollowers => {
            let rpc_param: Result<pb::ChannelRemoveFollowersParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 869709257 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelRemoveFollowers(rpc_param)),
            }
        }

        method_ids::ChannelSubscribe => {
            let rpc_param: Result<pb::ChannelSubscribeParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1367898912 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelSubscribe(rpc_param)),
            }
        }

        method_ids::ChannelUnSubscribe => {
            let rpc_param: Result<pb::ChannelUnSubscribeParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 858172401 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelUnSubscribe(rpc_param)),
            }
        }

        method_ids::ChannelRemoveSubscribers => {
            let rpc_param: Result<pb::ChannelRemoveSubscribersParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 729024592 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelRemoveSubscribers(
                    rpc_param,
                )),
            }
        }

        method_ids::ChannelChangePrivacy => {
            let rpc_param: Result<pb::ChannelChangePrivacyParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 79012409 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelChangePrivacy(rpc_param)),
            }
        }

        method_ids::ChannelChangeDefaultPermission => {
            let rpc_param: Result<pb::ChannelChangeDefaultPermissionParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1582638498 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelChangeDefaultPermission(
                    rpc_param,
                )),
            }
        }

        method_ids::ChannelRevokeLink => {
            let rpc_param: Result<pb::ChannelRevokeLinkParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1912530021 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelRevokeLink(rpc_param)),
            }
        }

        method_ids::ChannelChangeUsername => {
            let rpc_param: Result<pb::ChannelChangeUsernameParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 983884462 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelChangeUsername(rpc_param)),
            }
        }

        method_ids::ChannelBlockChannel => {
            let rpc_param: Result<pb::ChannelBlockChannelParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 2037016989 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelBlockChannel(rpc_param)),
            }
        }

        method_ids::ChannelSendMessage => {
            let rpc_param: Result<pb::ChannelSendMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1200751231 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelSendMessage(rpc_param)),
            }
        }

        method_ids::ChannelEditMessage => {
            let rpc_param: Result<pb::ChannelEditMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 727437726 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelEditMessage(rpc_param)),
            }
        }

        method_ids::ChannelPinMessage => {
            let rpc_param: Result<pb::ChannelPinMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 259263709 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelPinMessage(rpc_param)),
            }
        }

        method_ids::ChannelUnPinMessage => {
            let rpc_param: Result<pb::ChannelUnPinMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 113943649 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelUnPinMessage(rpc_param)),
            }
        }

        method_ids::ChannelDeleteMessage => {
            let rpc_param: Result<pb::ChannelDeleteMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 644189206 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelDeleteMessage(rpc_param)),
            }
        }

        method_ids::ChannelDeleteMessages => {
            let rpc_param: Result<pb::ChannelDeleteMessagesParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 2124822181 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelDeleteMessages(rpc_param)),
            }
        }

        method_ids::ChannelClearHistory => {
            let rpc_param: Result<pb::ChannelClearHistoryParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1164398815 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelClearHistory(rpc_param)),
            }
        }

        method_ids::ChannelAvatarAdd => {
            let rpc_param: Result<pb::ChannelAvatarAddParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1021808696 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAvatarAdd(rpc_param)),
            }
        }

        method_ids::ChannelAvatarChange => {
            let rpc_param: Result<pb::ChannelAvatarChangeParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1968579501 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAvatarChange(rpc_param)),
            }
        }

        method_ids::ChannelAvatarDelete => {
            let rpc_param: Result<pb::ChannelAvatarDeleteParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1626010891 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAvatarDelete(rpc_param)),
            }
        }

        method_ids::ChannelAvatarGetList => {
            let rpc_param: Result<pb::ChannelAvatarGetListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1925044843 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAvatarGetList(rpc_param)),
            }
        }

        method_ids::ChannelSendDoingAction => {
            let rpc_param: Result<pb::ChannelSendDoingActionParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 973237257 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelSendDoingAction(rpc_param)),
            }
        }

        method_ids::ChannelReportChannel => {
            let rpc_param: Result<pb::ChannelReportChannelParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 792938145 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelReportChannel(rpc_param)),
            }
        }

        method_ids::ChannelReportMessage => {
            let rpc_param: Result<pb::ChannelReportMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 2053528327 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelReportMessage(rpc_param)),
            }
        }

        method_ids::ChannelGetFull => {
            let rpc_param: Result<pb::ChannelGetFullParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1684531258 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetFull(rpc_param)),
            }
        }

        method_ids::ChannelGetMessagesList => {
            let rpc_param: Result<pb::ChannelGetMessagesListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1339072968 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetMessagesList(rpc_param)),
            }
        }

        method_ids::ChannelGetMediaList => {
            let rpc_param: Result<pb::ChannelGetMediaListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 985772653 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetMediaList(rpc_param)),
            }
        }

        method_ids::ChannelGetAuthors => {
            let rpc_param: Result<pb::ChannelGetAuthorsParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1373284924 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetAuthors(rpc_param)),
            }
        }

        method_ids::ChannelGetFollowers => {
            let rpc_param: Result<pb::ChannelGetFollowersParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1747172143 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetFollowers(rpc_param)),
            }
        }

        method_ids::ChannelGetFollowings => {
            let rpc_param: Result<pb::ChannelGetFollowingsParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1838438980 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetFollowings(rpc_param)),
            }
        }

        method_ids::ChannelGetSubscribers => {
            let rpc_param: Result<pb::ChannelGetSubscribersParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 2146806736 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetSubscribers(rpc_param)),
            }
        }

        method_ids::ChannelBlocked => {
            let rpc_param: Result<pb::ChannelBlockedParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1674411747 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelBlocked(rpc_param)),
            }
        }

        method_ids::ChannelSetDraft => {
            let rpc_param: Result<pb::ChannelSetDraftParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1403193015 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelSetDraft(rpc_param)),
            }
        }

        // RPC_Chat
        method_ids::ChatSendMessage => {
            let rpc_param: Result<pb::ChatSendMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1131621475 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatSendMessage(rpc_param)),
            }
        }

        method_ids::ChatEditMessage => {
            let rpc_param: Result<pb::ChatEditMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1806258329 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatEditMessage(rpc_param)),
            }
        }

        method_ids::ChatDeleteMessages => {
            let rpc_param: Result<pb::ChatDeleteMessagesParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 933526170 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatDeleteMessages(rpc_param)),
            }
        }

        method_ids::ChatDeleteHistory => {
            let rpc_param: Result<pb::ChatDeleteHistoryParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1088992782 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatDeleteHistory(rpc_param)),
            }
        }

        method_ids::ChatSendDoingAction => {
            let rpc_param: Result<pb::ChatSendDoingActionParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1319324241 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatSendDoingAction(rpc_param)),
            }
        }

        method_ids::ChatReportChat => {
            let rpc_param: Result<pb::ChatReportChatParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1345425871 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatReportChat(rpc_param)),
            }
        }

        method_ids::ChatGetFull => {
            let rpc_param: Result<pb::ChatGetFullMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1768678453 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatGetFull(rpc_param)),
            }
        }

        method_ids::ChatGetMessagesList => {
            let rpc_param: Result<pb::ChatGetMessagesListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 121549718 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatGetMessagesList(rpc_param)),
            }
        }

        method_ids::ChatGetMediaList => {
            let rpc_param: Result<pb::ChatGetMediaListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1346774525 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatGetMediaList(rpc_param)),
            }
        }

        // RPC_Direct
        method_ids::DirectDeleteDirect => {
            let rpc_param: Result<pb::DirectDeleteDirectParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1478067518 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectDeleteDirect(rpc_param)),
            }
        }

        method_ids::DirectChangeTitle => {
            let rpc_param: Result<pb::DirectChangeTitleParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 2041790485 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectChangeTitle(rpc_param)),
            }
        }

        method_ids::DirectSetCustomNotification => {
            let rpc_param: Result<pb::DirectSetCustomNotificationParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 548699291 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectSetCustomNotification(
                    rpc_param,
                )),
            }
        }

        method_ids::DirectSendActionDoing => {
            let rpc_param: Result<pb::DirectSendActionDoingParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1417285757 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectSendActionDoing(rpc_param)),
            }
        }

        method_ids::DirectSetDraft => {
            let rpc_param: Result<pb::DirectSetDraftParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1860345925 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectSetDraft(rpc_param)),
            }
        }

        method_ids::DirectDeleteDirects => {
            let rpc_param: Result<pb::DirectDeleteDirectsParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1291891637 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectDeleteDirects(rpc_param)),
            }
        }

        method_ids::DirectMarkAsRead => {
            let rpc_param: Result<pb::DirectMarkAsReadParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1801774787 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectMarkAsRead(rpc_param)),
            }
        }

        method_ids::DirectMarkAsUnRead => {
            let rpc_param: Result<pb::DirectMarkAsUnReadParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 313746334 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectMarkAsUnRead(rpc_param)),
            }
        }

        method_ids::DirectPinDirects => {
            let rpc_param: Result<pb::DirectPinDirectsParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1179089068 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectPinDirects(rpc_param)),
            }
        }

        method_ids::DirectUnPinDirects => {
            let rpc_param: Result<pb::DirectUnPinDirectsParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1517245560 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectUnPinDirects(rpc_param)),
            }
        }

        method_ids::DirectArchiveDirects => {
            let rpc_param: Result<pb::DirectArchiveDirectsParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1441782770 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectArchiveDirects(rpc_param)),
            }
        }

        method_ids::DirectUnArchiveDirects => {
            let rpc_param: Result<pb::DirectUnArchiveDirectsParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1951553867 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectUnArchiveDirects(rpc_param)),
            }
        }

        method_ids::DirectClearHistories => {
            let rpc_param: Result<pb::DirectClearHistoriesParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 904052140 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectClearHistories(rpc_param)),
            }
        }

        method_ids::DirectMuteDirects => {
            let rpc_param: Result<pb::DirectMuteDirectsParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1138477048 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectMuteDirects(rpc_param)),
            }
        }

        method_ids::DirectUnMuteDirects => {
            let rpc_param: Result<pb::DirectUnMuteDirectsParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1691834263 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectUnMuteDirects(rpc_param)),
            }
        }

        method_ids::DirectCreateFolder => {
            let rpc_param: Result<pb::DirectCreateFolderParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1878673022 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectCreateFolder(rpc_param)),
            }
        }

        method_ids::DirectChangeFolder => {
            let rpc_param: Result<pb::DirectChangeFolderParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1861381591 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectChangeFolder(rpc_param)),
            }
        }

        method_ids::DirectRemoveFromFolder => {
            let rpc_param: Result<pb::DirectRemoveFromFolderParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1818954127 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectRemoveFromFolder(rpc_param)),
            }
        }

        method_ids::DirectReordersFolder => {
            let rpc_param: Result<pb::DirectReordersFolderParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1264591958 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectReordersFolder(rpc_param)),
            }
        }

        method_ids::DirectDeleteFolder => {
            let rpc_param: Result<pb::DirectDeleteFolderParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 962281627 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectDeleteFolder(rpc_param)),
            }
        }

        method_ids::DirectGetChatsList => {
            let rpc_param: Result<pb::DirectGetChatsListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1570934969 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectGetChatsList(rpc_param)),
            }
        }

        method_ids::DirectGetGroupsList => {
            let rpc_param: Result<pb::DirectGetGroupsListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 545957996 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectGetGroupsList(rpc_param)),
            }
        }

        method_ids::DirectGetChannelsList => {
            let rpc_param: Result<pb::DirectGetChannelsListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1608173619 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectGetChannelsList(rpc_param)),
            }
        }

        method_ids::DirectGetFoldersList => {
            let rpc_param: Result<pb::DirectGetFoldersListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1384523712 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectGetFoldersList(rpc_param)),
            }
        }

        method_ids::DirectGetFoldersFullList => {
            let rpc_param: Result<pb::DirectGetFoldersFullListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 611850722 as i64,
                rpc_service: RPC_Direct(RPC_Direct_MethodData::DirectGetFoldersFullList(rpc_param)),
            }
        }

        // RPC_Group
        method_ids::GroupCreateGroup => {
            let rpc_param: Result<pb::GroupCreateGroupParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1205960678 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupCreateGroup(rpc_param)),
            }
        }

        method_ids::GroupEditGroup => {
            let rpc_param: Result<pb::GroupEditGroupParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1665019493 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupEditGroup(rpc_param)),
            }
        }

        method_ids::GroupDeleteGroup => {
            let rpc_param: Result<pb::GroupDeleteGroupParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 365183375 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupDeleteGroup(rpc_param)),
            }
        }

        method_ids::GroupAddAdmin => {
            let rpc_param: Result<pb::GroupAddAdminParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 958971956 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAddAdmin(rpc_param)),
            }
        }

        method_ids::GroupAddMember => {
            let rpc_param: Result<pb::GroupAddMemberParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 676599227 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAddMember(rpc_param)),
            }
        }

        method_ids::GroupRemoveMember => {
            let rpc_param: Result<pb::GroupRemoveMemberParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 2012702964 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupRemoveMember(rpc_param)),
            }
        }

        method_ids::GroupChangeMemberLevel => {
            let rpc_param: Result<pb::GroupChangeMemberLevelParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 589574238 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupChangeMemberLevel(rpc_param)),
            }
        }

        method_ids::GroupChangeMemberPermission => {
            let rpc_param: Result<pb::GroupChangeMemberPermissionParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 2132464067 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupChangeMemberPermission(
                    rpc_param,
                )),
            }
        }

        method_ids::GroupJoinGroup => {
            let rpc_param: Result<pb::JoinGroupParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 591743429 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupJoinGroup(rpc_param)),
            }
        }

        method_ids::GroupLeaveGroup => {
            let rpc_param: Result<pb::GroupLeaveGroupParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 361834630 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupLeaveGroup(rpc_param)),
            }
        }

        method_ids::GroupBanMember => {
            let rpc_param: Result<pb::GroupBanMemberParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 548504852 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupBanMember(rpc_param)),
            }
        }

        method_ids::GroupChangePrivacy => {
            let rpc_param: Result<pb::GroupChangePrivacyParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1497988410 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupChangePrivacy(rpc_param)),
            }
        }

        method_ids::GroupChangeDefaultPermission => {
            let rpc_param: Result<pb::GroupChangeDefaultPermissionParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 605792138 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupChangeDefaultPermission(
                    rpc_param,
                )),
            }
        }

        method_ids::GroupRevokeLink => {
            let rpc_param: Result<pb::GroupRevokeLinkParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 406592509 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupRevokeLink(rpc_param)),
            }
        }

        method_ids::GroupChangeUsername => {
            let rpc_param: Result<pb::GroupChangeUsernameParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 832997038 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupChangeUsername(rpc_param)),
            }
        }

        method_ids::GroupSendMessage => {
            let rpc_param: Result<pb::GroupSendMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 599852950 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupSendMessage(rpc_param)),
            }
        }

        method_ids::GroupEditMessage => {
            let rpc_param: Result<pb::GroupEditMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 742937895 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupEditMessage(rpc_param)),
            }
        }

        method_ids::GroupPinMessage => {
            let rpc_param: Result<pb::GroupPinMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 184560027 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupPinMessage(rpc_param)),
            }
        }

        method_ids::GroupUnPinMessage => {
            let rpc_param: Result<pb::GroupUnPinMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1290613173 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupUnPinMessage(rpc_param)),
            }
        }

        method_ids::GroupDeleteMessage => {
            let rpc_param: Result<pb::GroupDeleteMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 393991035 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupDeleteMessage(rpc_param)),
            }
        }

        method_ids::GroupDeleteMessages => {
            let rpc_param: Result<pb::GroupDeleteMessagesParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 276700675 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupDeleteMessages(rpc_param)),
            }
        }

        method_ids::GroupDeleteHistory => {
            let rpc_param: Result<pb::GroupDeleteHistoryParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1270953793 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupDeleteHistory(rpc_param)),
            }
        }

        method_ids::GroupClearHistory => {
            let rpc_param: Result<pb::GroupClearHistoryParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1352552449 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupClearHistory(rpc_param)),
            }
        }

        method_ids::GroupAvatarAdd => {
            let rpc_param: Result<pb::GroupAvatarAddParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1202058216 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAvatarAdd(rpc_param)),
            }
        }

        method_ids::GroupAvatarChange => {
            let rpc_param: Result<pb::GroupAvatarChangeParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 108612523 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAvatarChange(rpc_param)),
            }
        }

        method_ids::GroupAvatarDelete => {
            let rpc_param: Result<pb::GroupAvatarDeleteParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 775862697 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAvatarDelete(rpc_param)),
            }
        }

        method_ids::GroupAvatarGetList => {
            let rpc_param: Result<pb::GroupAvatarGetListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 939443722 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAvatarGetList(rpc_param)),
            }
        }

        method_ids::GroupSendDoingAction => {
            let rpc_param: Result<pb::GroupSendDoingActionParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 2022474356 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupSendDoingAction(rpc_param)),
            }
        }

        method_ids::GroupReportGroup => {
            let rpc_param: Result<pb::GroupReportGroupParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1759704420 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupReportGroup(rpc_param)),
            }
        }

        method_ids::GroupGetFull => {
            let rpc_param: Result<pb::GroupGetFullMessageParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 200351324 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupGetFull(rpc_param)),
            }
        }

        method_ids::GroupGetMessagesList => {
            let rpc_param: Result<pb::GroupGetMessagesListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1541835459 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupGetMessagesList(rpc_param)),
            }
        }

        method_ids::GroupGetMediaList => {
            let rpc_param: Result<pb::GroupGetMediaListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 2143016912 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupGetMediaList(rpc_param)),
            }
        }

        method_ids::GroupGetMembersList => {
            let rpc_param: Result<pb::GroupGetMembersListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 429215412 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupGetMembersList(rpc_param)),
            }
        }

        method_ids::GroupGetAdminsList => {
            let rpc_param: Result<pb::GroupGetAdminsListParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 332260610 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupGetAdminsList(rpc_param)),
            }
        }

        method_ids::GroupSetDraft => {
            let rpc_param: Result<pb::GroupSetDraftParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 77668156 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupSetDraft(rpc_param)),
            }
        }

        // RPC_Sample
        method_ids::GetUsers1 => {
            let rpc_param: Result<pb::GetUsers1Param, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 486248681 as i64,
                rpc_service: RPC_Sample(RPC_Sample_MethodData::GetUsers1(rpc_param)),
            }
        }

        method_ids::GetProfiles => {
            let rpc_param: Result<pb::GetProfilesParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 822554282 as i64,
                rpc_service: RPC_Sample(RPC_Sample_MethodData::GetProfiles(rpc_param)),
            }
        }

        method_ids::GetChannels => {
            let rpc_param: Result<pb::GetChannelsParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1734748927 as i64,
                rpc_service: RPC_Sample(RPC_Sample_MethodData::GetChannels(rpc_param)),
            }
        }

        method_ids::GetDirects => {
            let rpc_param: Result<pb::GetDirectsParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 558085683 as i64,
                rpc_service: RPC_Sample(RPC_Sample_MethodData::GetDirects(rpc_param)),
            }
        }

        method_ids::GetMessages => {
            let rpc_param: Result<pb::GetMessagesParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1160951872 as i64,
                rpc_service: RPC_Sample(RPC_Sample_MethodData::GetMessages(rpc_param)),
            }
        }

        // RPC_Shared
        method_ids::Echo => {
            let rpc_param: Result<pb::EchoParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 101973561 as i64,
                rpc_service: RPC_Shared(RPC_Shared_MethodData::Echo(rpc_param)),
            }
        }

        method_ids::CheckUserName => {
            let rpc_param: Result<pb::CheckUserNameParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1897027349 as i64,
                rpc_service: RPC_Shared(RPC_Shared_MethodData::CheckUserName(rpc_param)),
            }
        }

        // RPC_Upload
        method_ids::UploadFile => {
            let rpc_param: Result<pb::UploadFileParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 1702285478 as i64,
                rpc_service: RPC_Upload(RPC_Upload_MethodData::UploadFile(rpc_param)),
            }
        }

        // RPC_User
        method_ids::ChangePhoneNumber => {
            let rpc_param: Result<pb::ChangePhoneNumberParam, ::prost::DecodeError> =
                prost::Message::decode(invoke.rpc_data.as_slice());
            let rpc_param = rpc_param.unwrap();
            RpcInvoke {
                method_id: 706069694 as i64,
                rpc_service: RPC_User(RPC_User_MethodData::ChangePhoneNumber(rpc_param)),
            }
        }

        _ => panic!("sdf"),
    };
    Ok(rpc)
}

pub async fn server_rpc(act: RpcInvoke, reg: impl All_Rpc_Handler) -> Result<Vec<u8>, GenErr> {
    let res_v8 = match act.rpc_service {
        RpcServiceData::RPC_Auth(method) => match method {
            RPC_Auth_MethodData::SendConfirmCode(param) => {
                let reg = reg.clone();
                let response = reg.SendConfirmCode(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Auth_MethodData::ConfirmCode(param) => {
                let reg = reg.clone();
                let response = reg.ConfirmCode(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Auth_MethodData::SingUp(param) => {
                let reg = reg.clone();
                let response = reg.SingUp(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Auth_MethodData::SingIn(param) => {
                let reg = reg.clone();
                let response = reg.SingIn(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Auth_MethodData::LogOut(param) => {
                let reg = reg.clone();
                let response = reg.LogOut(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Channel(method) => match method {
            RPC_Channel_MethodData::ChannelCreateChannel(param) => {
                let reg = reg.clone();
                let response = reg.ChannelCreateChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelEditChannel(param) => {
                let reg = reg.clone();
                let response = reg.ChannelEditChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelDeleteChannel(param) => {
                let reg = reg.clone();
                let response = reg.ChannelDeleteChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelAddAuthor(param) => {
                let reg = reg.clone();
                let response = reg.ChannelAddAuthor(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelChangeAuthorPermission(param) => {
                let reg = reg.clone();
                let response = reg.ChannelChangeAuthorPermission(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelRemoveAuthor(param) => {
                let reg = reg.clone();
                let response = reg.ChannelRemoveAuthor(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelFollowChannel(param) => {
                let reg = reg.clone();
                let response = reg.ChannelFollowChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelUnFollowChannel(param) => {
                let reg = reg.clone();
                let response = reg.ChannelUnFollowChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelRemoveFollowers(param) => {
                let reg = reg.clone();
                let response = reg.ChannelRemoveFollowers(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelSubscribe(param) => {
                let reg = reg.clone();
                let response = reg.ChannelSubscribe(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelUnSubscribe(param) => {
                let reg = reg.clone();
                let response = reg.ChannelUnSubscribe(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelRemoveSubscribers(param) => {
                let reg = reg.clone();
                let response = reg.ChannelRemoveSubscribers(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelChangePrivacy(param) => {
                let reg = reg.clone();
                let response = reg.ChannelChangePrivacy(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelChangeDefaultPermission(param) => {
                let reg = reg.clone();
                let response = reg.ChannelChangeDefaultPermission(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelRevokeLink(param) => {
                let reg = reg.clone();
                let response = reg.ChannelRevokeLink(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelChangeUsername(param) => {
                let reg = reg.clone();
                let response = reg.ChannelChangeUsername(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelBlockChannel(param) => {
                let reg = reg.clone();
                let response = reg.ChannelBlockChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelSendMessage(param) => {
                let reg = reg.clone();
                let response = reg.ChannelSendMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelEditMessage(param) => {
                let reg = reg.clone();
                let response = reg.ChannelEditMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelPinMessage(param) => {
                let reg = reg.clone();
                let response = reg.ChannelPinMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelUnPinMessage(param) => {
                let reg = reg.clone();
                let response = reg.ChannelUnPinMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelDeleteMessage(param) => {
                let reg = reg.clone();
                let response = reg.ChannelDeleteMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelDeleteMessages(param) => {
                let reg = reg.clone();
                let response = reg.ChannelDeleteMessages(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelClearHistory(param) => {
                let reg = reg.clone();
                let response = reg.ChannelClearHistory(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelAvatarAdd(param) => {
                let reg = reg.clone();
                let response = reg.ChannelAvatarAdd(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelAvatarChange(param) => {
                let reg = reg.clone();
                let response = reg.ChannelAvatarChange(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelAvatarDelete(param) => {
                let reg = reg.clone();
                let response = reg.ChannelAvatarDelete(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelAvatarGetList(param) => {
                let reg = reg.clone();
                let response = reg.ChannelAvatarGetList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelSendDoingAction(param) => {
                let reg = reg.clone();
                let response = reg.ChannelSendDoingAction(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelReportChannel(param) => {
                let reg = reg.clone();
                let response = reg.ChannelReportChannel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelReportMessage(param) => {
                let reg = reg.clone();
                let response = reg.ChannelReportMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetFull(param) => {
                let reg = reg.clone();
                let response = reg.ChannelGetFull(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetMessagesList(param) => {
                let reg = reg.clone();
                let response = reg.ChannelGetMessagesList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetMediaList(param) => {
                let reg = reg.clone();
                let response = reg.ChannelGetMediaList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetAuthors(param) => {
                let reg = reg.clone();
                let response = reg.ChannelGetAuthors(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetFollowers(param) => {
                let reg = reg.clone();
                let response = reg.ChannelGetFollowers(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetFollowings(param) => {
                let reg = reg.clone();
                let response = reg.ChannelGetFollowings(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetSubscribers(param) => {
                let reg = reg.clone();
                let response = reg.ChannelGetSubscribers(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelBlocked(param) => {
                let reg = reg.clone();
                let response = reg.ChannelBlocked(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelSetDraft(param) => {
                let reg = reg.clone();
                let response = reg.ChannelSetDraft(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Chat(method) => match method {
            RPC_Chat_MethodData::ChatSendMessage(param) => {
                let reg = reg.clone();
                let response = reg.ChatSendMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatEditMessage(param) => {
                let reg = reg.clone();
                let response = reg.ChatEditMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatDeleteMessages(param) => {
                let reg = reg.clone();
                let response = reg.ChatDeleteMessages(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatDeleteHistory(param) => {
                let reg = reg.clone();
                let response = reg.ChatDeleteHistory(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatSendDoingAction(param) => {
                let reg = reg.clone();
                let response = reg.ChatSendDoingAction(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatReportChat(param) => {
                let reg = reg.clone();
                let response = reg.ChatReportChat(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatGetFull(param) => {
                let reg = reg.clone();
                let response = reg.ChatGetFull(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatGetMessagesList(param) => {
                let reg = reg.clone();
                let response = reg.ChatGetMessagesList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Chat_MethodData::ChatGetMediaList(param) => {
                let reg = reg.clone();
                let response = reg.ChatGetMediaList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Direct(method) => match method {
            RPC_Direct_MethodData::DirectDeleteDirect(param) => {
                let reg = reg.clone();
                let response = reg.DirectDeleteDirect(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectChangeTitle(param) => {
                let reg = reg.clone();
                let response = reg.DirectChangeTitle(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectSetCustomNotification(param) => {
                let reg = reg.clone();
                let response = reg.DirectSetCustomNotification(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectSendActionDoing(param) => {
                let reg = reg.clone();
                let response = reg.DirectSendActionDoing(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectSetDraft(param) => {
                let reg = reg.clone();
                let response = reg.DirectSetDraft(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectDeleteDirects(param) => {
                let reg = reg.clone();
                let response = reg.DirectDeleteDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectMarkAsRead(param) => {
                let reg = reg.clone();
                let response = reg.DirectMarkAsRead(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectMarkAsUnRead(param) => {
                let reg = reg.clone();
                let response = reg.DirectMarkAsUnRead(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectPinDirects(param) => {
                let reg = reg.clone();
                let response = reg.DirectPinDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectUnPinDirects(param) => {
                let reg = reg.clone();
                let response = reg.DirectUnPinDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectArchiveDirects(param) => {
                let reg = reg.clone();
                let response = reg.DirectArchiveDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectUnArchiveDirects(param) => {
                let reg = reg.clone();
                let response = reg.DirectUnArchiveDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectClearHistories(param) => {
                let reg = reg.clone();
                let response = reg.DirectClearHistories(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectMuteDirects(param) => {
                let reg = reg.clone();
                let response = reg.DirectMuteDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectUnMuteDirects(param) => {
                let reg = reg.clone();
                let response = reg.DirectUnMuteDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectCreateFolder(param) => {
                let reg = reg.clone();
                let response = reg.DirectCreateFolder(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectChangeFolder(param) => {
                let reg = reg.clone();
                let response = reg.DirectChangeFolder(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectRemoveFromFolder(param) => {
                let reg = reg.clone();
                let response = reg.DirectRemoveFromFolder(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectReordersFolder(param) => {
                let reg = reg.clone();
                let response = reg.DirectReordersFolder(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectDeleteFolder(param) => {
                let reg = reg.clone();
                let response = reg.DirectDeleteFolder(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectGetChatsList(param) => {
                let reg = reg.clone();
                let response = reg.DirectGetChatsList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectGetGroupsList(param) => {
                let reg = reg.clone();
                let response = reg.DirectGetGroupsList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectGetChannelsList(param) => {
                let reg = reg.clone();
                let response = reg.DirectGetChannelsList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectGetFoldersList(param) => {
                let reg = reg.clone();
                let response = reg.DirectGetFoldersList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Direct_MethodData::DirectGetFoldersFullList(param) => {
                let reg = reg.clone();
                let response = reg.DirectGetFoldersFullList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Group(method) => match method {
            RPC_Group_MethodData::GroupCreateGroup(param) => {
                let reg = reg.clone();
                let response = reg.GroupCreateGroup(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupEditGroup(param) => {
                let reg = reg.clone();
                let response = reg.GroupEditGroup(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupDeleteGroup(param) => {
                let reg = reg.clone();
                let response = reg.GroupDeleteGroup(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAddAdmin(param) => {
                let reg = reg.clone();
                let response = reg.GroupAddAdmin(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAddMember(param) => {
                let reg = reg.clone();
                let response = reg.GroupAddMember(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupRemoveMember(param) => {
                let reg = reg.clone();
                let response = reg.GroupRemoveMember(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupChangeMemberLevel(param) => {
                let reg = reg.clone();
                let response = reg.GroupChangeMemberLevel(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupChangeMemberPermission(param) => {
                let reg = reg.clone();
                let response = reg.GroupChangeMemberPermission(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupJoinGroup(param) => {
                let reg = reg.clone();
                let response = reg.GroupJoinGroup(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupLeaveGroup(param) => {
                let reg = reg.clone();
                let response = reg.GroupLeaveGroup(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupBanMember(param) => {
                let reg = reg.clone();
                let response = reg.GroupBanMember(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupChangePrivacy(param) => {
                let reg = reg.clone();
                let response = reg.GroupChangePrivacy(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupChangeDefaultPermission(param) => {
                let reg = reg.clone();
                let response = reg.GroupChangeDefaultPermission(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupRevokeLink(param) => {
                let reg = reg.clone();
                let response = reg.GroupRevokeLink(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupChangeUsername(param) => {
                let reg = reg.clone();
                let response = reg.GroupChangeUsername(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupSendMessage(param) => {
                let reg = reg.clone();
                let response = reg.GroupSendMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupEditMessage(param) => {
                let reg = reg.clone();
                let response = reg.GroupEditMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupPinMessage(param) => {
                let reg = reg.clone();
                let response = reg.GroupPinMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupUnPinMessage(param) => {
                let reg = reg.clone();
                let response = reg.GroupUnPinMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupDeleteMessage(param) => {
                let reg = reg.clone();
                let response = reg.GroupDeleteMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupDeleteMessages(param) => {
                let reg = reg.clone();
                let response = reg.GroupDeleteMessages(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupDeleteHistory(param) => {
                let reg = reg.clone();
                let response = reg.GroupDeleteHistory(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupClearHistory(param) => {
                let reg = reg.clone();
                let response = reg.GroupClearHistory(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAvatarAdd(param) => {
                let reg = reg.clone();
                let response = reg.GroupAvatarAdd(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAvatarChange(param) => {
                let reg = reg.clone();
                let response = reg.GroupAvatarChange(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAvatarDelete(param) => {
                let reg = reg.clone();
                let response = reg.GroupAvatarDelete(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAvatarGetList(param) => {
                let reg = reg.clone();
                let response = reg.GroupAvatarGetList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupSendDoingAction(param) => {
                let reg = reg.clone();
                let response = reg.GroupSendDoingAction(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupReportGroup(param) => {
                let reg = reg.clone();
                let response = reg.GroupReportGroup(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupGetFull(param) => {
                let reg = reg.clone();
                let response = reg.GroupGetFull(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupGetMessagesList(param) => {
                let reg = reg.clone();
                let response = reg.GroupGetMessagesList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupGetMediaList(param) => {
                let reg = reg.clone();
                let response = reg.GroupGetMediaList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupGetMembersList(param) => {
                let reg = reg.clone();
                let response = reg.GroupGetMembersList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupGetAdminsList(param) => {
                let reg = reg.clone();
                let response = reg.GroupGetAdminsList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupSetDraft(param) => {
                let reg = reg.clone();
                let response = reg.GroupSetDraft(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Sample(method) => match method {
            RPC_Sample_MethodData::GetUsers1(param) => {
                let reg = reg.clone();
                let response = reg.GetUsers1(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Sample_MethodData::GetProfiles(param) => {
                let reg = reg.clone();
                let response = reg.GetProfiles(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Sample_MethodData::GetChannels(param) => {
                let reg = reg.clone();
                let response = reg.GetChannels(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Sample_MethodData::GetDirects(param) => {
                let reg = reg.clone();
                let response = reg.GetDirects(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Sample_MethodData::GetMessages(param) => {
                let reg = reg.clone();
                let response = reg.GetMessages(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Shared(method) => match method {
            RPC_Shared_MethodData::Echo(param) => {
                let reg = reg.clone();
                let response = reg.Echo(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shared_MethodData::CheckUserName(param) => {
                let reg = reg.clone();
                let response = reg.CheckUserName(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Upload(method) => match method {
            RPC_Upload_MethodData::UploadFile(param) => {
                let reg = reg.clone();
                let response = reg.UploadFile(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_User(method) => match method {
            RPC_User_MethodData::ChangePhoneNumber(param) => {
                let reg = reg.clone();
                let response = reg.ChangePhoneNumber(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },
    };

    Ok(res_v8)
}

pub struct RPC_Registry {
    // RPC_Shared: RPC_Shared,
// RPC_Chat: RPC_Chat,
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
