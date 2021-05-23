//#![rustfmt::skip]

use crate::{common, pb};
use crate::{errors::GenErr, UserParam};
use async_trait::async_trait;

use http::Version;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error as HyperError, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug)]
pub struct RpcInvoke {
    method_id: i64, // correct data type should be i32,
    rpc_service: RpcServiceData,
}

#[derive(Debug)]
pub enum RpcServiceData {
    IPC_CMaster(IPC_CMaster_MethodData),
    RPC_Auth(RPC_Auth_MethodData),
    RPC_Channel(RPC_Channel_MethodData),
    RPC_Chat(RPC_Chat_MethodData),
    RPC_Group(RPC_Group_MethodData),
    RPC_Profile(RPC_Profile_MethodData),
    RPC_Sample(RPC_Sample_MethodData),
    RPC_Shared(RPC_Shared_MethodData),
    RPC_Shop(RPC_Shop_MethodData),
    RPC_Upload(RPC_Upload_MethodData),
    RPC_User(RPC_User_MethodData),
}

#[derive(Debug)]
pub enum IPC_CMaster_MethodData {
    GetNextId(pb::GetNextIdParam),
}
#[derive(Debug)]
pub enum RPC_Auth_MethodData {
    AuthSendCode(pb::AuthSendCodeParam),
    AuthLogIn(pb::AuthLogInParam),
    AuthLogOut(pb::AuthLogOutParam),
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
    ChannelBlockProfile(pb::ChannelBlockProfileParam),
    ChannelSendMessage(pb::ChannelSendMessageParam),
    ChannelEditMessage(pb::ChannelEditMessageParam),
    ChannelDeleteMessages(pb::ChannelDeleteMessagesParam),
    ChannelLikeMessage(pb::ChannelLikeMessageParam),
    ChannelUnLikeMessage(pb::ChannelUnLikeMessageParam),
    ChannelReShareMessage(pb::ChannelReShareMessageParam),
    ChannelUnReShareMessage(pb::ChannelUnReShareMessageParam),
    ChannelAddComment(pb::ChannelAddCommentParam),
    ChannelDeleteComment(pb::ChannelDeleteCommentParam),
    ChannelPinMessage(pb::ChannelPinMessageParam),
    ChannelUnPinMessage(pb::ChannelUnPinMessageParam),
    ChannelAvatarAdd(pb::ChannelAvatarAddParam),
    ChannelAvatarDelete(pb::ChannelAvatarDeleteParam),
    ChannelSendDoingAction(pb::ChannelSendDoingActionParam),
    ChannelReportChannel(pb::ChannelReportChannelParam),
    ChannelReportMessage(pb::ChannelReportMessageParam),
    ChannelGetFull(pb::ChannelGetFullParam),
    ChannelGetMessagesList(pb::ChannelGetMessagesListParam),
    ChannelGetMediaList(pb::ChannelGetMediaListParam),
    ChannelGetAuthors(pb::ChannelGetAuthorsParam),
    ChannelGetFollowers(pb::ChannelGetFollowersParam),
    ChannelGetSubscribers(pb::ChannelGetSubscribersParam),
    ChannelBlocked(pb::ChannelBlockedParam),
    ChannelAvatarGetList(pb::ChannelAvatarGetListParam),
    ChannelGetInbox(pb::ChannelGetInboxParam),
    ChannelGetFollowings(pb::ChannelGetFollowingsParam),
}
#[derive(Debug)]
pub enum RPC_Chat_MethodData {
    ChatDeleteChat(pb::ChatDeleteChatParam),
    ChatSendMessage(pb::ChatSendMessageParam),
    ChatEditMessage(pb::ChatEditMessageParam),
    ChatDeleteMessages(pb::ChatDeleteMessagesParam),
    ChatDeleteHistory(pb::ChatDeleteHistoryParam),
    ChatSetNotification(pb::ChatSetNotificationParam),
    ChatSendDoingAction(pb::ChatSendDoingActionParam),
    ChatReportChat(pb::ChatReportChatParam),
    ChatGetFull(pb::ChatGetFullMessageParam),
    ChatGetMessagesList(pb::ChatGetMessagesListParam),
    ChatGetMediaList(pb::ChatGetMediaListParam),
    ChatGetInbox(pb::ChatGetInboxParam),
}
#[derive(Debug)]
pub enum RPC_Group_MethodData {
    GroupCreateGroup(pb::GroupCreateGroupParam),
    GroupEditGroup(pb::GroupEditGroupParam),
    GroupDeleteGroup(pb::GroupDeleteGroupParam),
    GroupAddAdmin(pb::GroupAddAdminParam),
    GroupRemoveMember(pb::GroupRemoveMemberParam),
    GroupChangeMemberLevel(pb::GroupChangeMemberLevelParam),
    GroupChangeMemberPermission(pb::GroupChangeMemberPermissionParam),
    GroupBanMember(pb::GroupBanMemberParam),
    GroupJoinGroup(pb::JoinGroupParam),
    GroupLeaveGroup(pb::GroupLeaveGroupParam),
    GroupAddMember(pb::GroupAddMemberParam),
    GroupSetNotification(pb::GroupSetNotificationParam),
    GroupChangePrivacy(pb::GroupChangePrivacyParam),
    GroupChangeDefaultPermission(pb::GroupChangeDefaultPermissionParam),
    GroupRevokeLink(pb::GroupRevokeLinkParam),
    GroupChangeUsername(pb::GroupChangeUsernameParam),
    GroupSendMessage(pb::GroupSendMessageParam),
    GroupEditMessage(pb::GroupEditMessageParam),
    GroupDeleteMessages(pb::GroupDeleteMessagesParam),
    GroupDeleteHistory(pb::GroupDeleteHistoryParam),
    GroupPinMessage(pb::GroupPinMessageParam),
    GroupUnPinMessage(pb::GroupUnPinMessageParam),
    GroupAvatarAdd(pb::GroupAvatarAddParam),
    GroupAvatarDelete(pb::GroupAvatarDeleteParam),
    GroupSendDoingAction(pb::GroupSendDoingActionParam),
    GroupReportGroup(pb::GroupReportGroupParam),
    GroupGetFull(pb::GroupGetFullParam),
    GroupGetMessagesList(pb::GroupGetMessagesListParam),
    GroupGetMediaList(pb::GroupGetMediaListParam),
    GroupGetMembersList(pb::GroupGetMembersListParam),
    GroupGetAdminsList(pb::GroupGetAdminsListParam),
    GroupAvatarGetList(pb::GroupAvatarGetListParam),
    GroupGetInbox(pb::GroupGetInboxParam),
}
#[derive(Debug)]
pub enum RPC_Profile_MethodData {
    ProfileSetSettings(pb::ProfileSetSettingsParam),
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
    SharedEcho(pb::SharedEchoParam),
    SharedCheckUserName(pb::SharedCheckUserNameParam),
}
#[derive(Debug)]
pub enum RPC_Shop_MethodData {
    ShopEEE(pb::Param),
    ShopCreateShop(pb::Param),
    ShopEditShop(pb::Param),
    ShopPauseShop(pb::Param),
    ShopTerminateShop(pb::Param),
    ShopAddProduct(pb::Param),
    ShopEditProduct(pb::Param),
    ShopDeleteProduct(pb::Param),
    ShopAddProductToBasket(pb::Param),
    ShopRemoveProductFromBasket(pb::Param),
    ShopCheckoutBasket(pb::Param),
    ShopLikeProduct(pb::Param),
    ShopUnLikeProduct(pb::Param),
    ShopLikeShop(pb::Param),
    ShopUnLikeShop(pb::Param),
    ShopGetFull(pb::Param),
    ShopProductList(pb::Param),
    ShopGetBasketList(pb::Param),
    ShopGetLikedProductsList(pb::Param),
    ShopGetLikedShopsList(pb::Param),
    ShopGetOrderList(pb::Param),
    ShopGetRefundList(pb::Param),
    ShopCancelOrder(pb::Param),
    ShopEditProductInfo(pb::Param),
    ShopEditProductPrice(pb::Param),
    ShopEditProductInventory(pb::Param),
}
#[derive(Debug)]
pub enum RPC_Upload_MethodData {
    UploadFile(pb::UploadFileParam),
}
#[derive(Debug)]
pub enum RPC_User_MethodData {
    UserRegisterUser(pb::UserRegisterUserParam),
    UserEditUser(pb::UserEditUserParam),
    UserDeleteSendCode(pb::UserDeleteSendCodeParam),
    UserDeleteConfirmCode(pb::UserDeleteConfirmCodeParam),
    UserDeleteUser(pb::UserDeleteUserParam),
    UserChangePhoneNumber(pb::UserChangePhoneNumberParam),
    UserRemoveSession(pb::UserRemoveSessionParam),
    UserRemoveOtherSessions(pb::UserRemoveOtherParam),
    UserGetMe(pb::UserGetMeParam),
    UserGetActiveSessions(pb::UserGetActiveSessionsParam),
}

#[async_trait]
pub trait IPC_CMaster_Handler {
    async fn GetNextId(
        up: &UserParam,
        param: pb::GetNextIdParam,
    ) -> Result<pb::GetNextIdResponse, GenErr> {
        Ok(pb::GetNextIdResponse::default())
    }
}
#[async_trait]
pub trait RPC_Auth_Handler {
    async fn AuthSendCode(
        up: &UserParam,
        param: pb::AuthSendCodeParam,
    ) -> Result<pb::AuthSendCodeResponse, GenErr> {
        Ok(pb::AuthSendCodeResponse::default())
    }
    async fn AuthLogIn(
        up: &UserParam,
        param: pb::AuthLogInParam,
    ) -> Result<pb::AuthLogInResponse, GenErr> {
        Ok(pb::AuthLogInResponse::default())
    }
    async fn AuthLogOut(
        up: &UserParam,
        param: pb::AuthLogOutParam,
    ) -> Result<pb::AuthLogOutResponse, GenErr> {
        Ok(pb::AuthLogOutResponse::default())
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
    async fn ChannelBlockProfile(
        up: &UserParam,
        param: pb::ChannelBlockProfileParam,
    ) -> Result<pb::ChannelBlockProfileResponse, GenErr> {
        Ok(pb::ChannelBlockProfileResponse::default())
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
    async fn ChannelDeleteMessages(
        up: &UserParam,
        param: pb::ChannelDeleteMessagesParam,
    ) -> Result<pb::ChannelDeleteMessagesResponse, GenErr> {
        Ok(pb::ChannelDeleteMessagesResponse::default())
    }
    async fn ChannelLikeMessage(
        up: &UserParam,
        param: pb::ChannelLikeMessageParam,
    ) -> Result<pb::ChannelLikeMessageResponse, GenErr> {
        Ok(pb::ChannelLikeMessageResponse::default())
    }
    async fn ChannelUnLikeMessage(
        up: &UserParam,
        param: pb::ChannelUnLikeMessageParam,
    ) -> Result<pb::ChannelUnLikeMessageResponse, GenErr> {
        Ok(pb::ChannelUnLikeMessageResponse::default())
    }
    async fn ChannelReShareMessage(
        up: &UserParam,
        param: pb::ChannelReShareMessageParam,
    ) -> Result<pb::ChannelReShareMessageResponse, GenErr> {
        Ok(pb::ChannelReShareMessageResponse::default())
    }
    async fn ChannelUnReShareMessage(
        up: &UserParam,
        param: pb::ChannelUnReShareMessageParam,
    ) -> Result<pb::ChannelUnReShareMessageResponse, GenErr> {
        Ok(pb::ChannelUnReShareMessageResponse::default())
    }
    async fn ChannelAddComment(
        up: &UserParam,
        param: pb::ChannelAddCommentParam,
    ) -> Result<pb::ChannelAddCommentResponse, GenErr> {
        Ok(pb::ChannelAddCommentResponse::default())
    }
    async fn ChannelDeleteComment(
        up: &UserParam,
        param: pb::ChannelDeleteCommentParam,
    ) -> Result<pb::ChannelDeleteCommentResponse, GenErr> {
        Ok(pb::ChannelDeleteCommentResponse::default())
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
    async fn ChannelAvatarAdd(
        up: &UserParam,
        param: pb::ChannelAvatarAddParam,
    ) -> Result<pb::ChannelAvatarAddResponse, GenErr> {
        Ok(pb::ChannelAvatarAddResponse::default())
    }
    async fn ChannelAvatarDelete(
        up: &UserParam,
        param: pb::ChannelAvatarDeleteParam,
    ) -> Result<pb::ChannelAvatarDeleteResponse, GenErr> {
        Ok(pb::ChannelAvatarDeleteResponse::default())
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
    async fn ChannelAvatarGetList(
        up: &UserParam,
        param: pb::ChannelAvatarGetListParam,
    ) -> Result<pb::ChannelAvatarGetListResponse, GenErr> {
        Ok(pb::ChannelAvatarGetListResponse::default())
    }
    async fn ChannelGetInbox(
        up: &UserParam,
        param: pb::ChannelGetInboxParam,
    ) -> Result<pb::ChannelGetInboxResponse, GenErr> {
        Ok(pb::ChannelGetInboxResponse::default())
    }
    async fn ChannelGetFollowings(
        up: &UserParam,
        param: pb::ChannelGetFollowingsParam,
    ) -> Result<pb::ChannelGetFollowingsResponse, GenErr> {
        Ok(pb::ChannelGetFollowingsResponse::default())
    }
}
#[async_trait]
pub trait RPC_Chat_Handler {
    async fn ChatDeleteChat(
        up: &UserParam,
        param: pb::ChatDeleteChatParam,
    ) -> Result<pb::ChatDeleteChatResponse, GenErr> {
        Ok(pb::ChatDeleteChatResponse::default())
    }
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
    async fn ChatSetNotification(
        up: &UserParam,
        param: pb::ChatSetNotificationParam,
    ) -> Result<pb::ChatSetNotificationResponse, GenErr> {
        Ok(pb::ChatSetNotificationResponse::default())
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
    async fn ChatGetInbox(
        up: &UserParam,
        param: pb::ChatGetInboxParam,
    ) -> Result<pb::ChatGetInboxResponse, GenErr> {
        Ok(pb::ChatGetInboxResponse::default())
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
    async fn GroupBanMember(
        up: &UserParam,
        param: pb::GroupBanMemberParam,
    ) -> Result<pb::GroupBanMemberResponse, GenErr> {
        Ok(pb::GroupBanMemberResponse::default())
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
    async fn GroupAddMember(
        up: &UserParam,
        param: pb::GroupAddMemberParam,
    ) -> Result<pb::GroupAddMemberResponse, GenErr> {
        Ok(pb::GroupAddMemberResponse::default())
    }
    async fn GroupSetNotification(
        up: &UserParam,
        param: pb::GroupSetNotificationParam,
    ) -> Result<pb::GroupSetNotificationResponse, GenErr> {
        Ok(pb::GroupSetNotificationResponse::default())
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
    async fn GroupAvatarAdd(
        up: &UserParam,
        param: pb::GroupAvatarAddParam,
    ) -> Result<pb::GroupAvatarAddResponse, GenErr> {
        Ok(pb::GroupAvatarAddResponse::default())
    }
    async fn GroupAvatarDelete(
        up: &UserParam,
        param: pb::GroupAvatarDeleteParam,
    ) -> Result<pb::GroupAvatarDeleteResponse, GenErr> {
        Ok(pb::GroupAvatarDeleteResponse::default())
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
        param: pb::GroupGetFullParam,
    ) -> Result<pb::GroupGetFullResponse, GenErr> {
        Ok(pb::GroupGetFullResponse::default())
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
    async fn GroupAvatarGetList(
        up: &UserParam,
        param: pb::GroupAvatarGetListParam,
    ) -> Result<pb::GroupAvatarGetListResponse, GenErr> {
        Ok(pb::GroupAvatarGetListResponse::default())
    }
    async fn GroupGetInbox(
        up: &UserParam,
        param: pb::GroupGetInboxParam,
    ) -> Result<pb::GroupGetInboxResponse, GenErr> {
        Ok(pb::GroupGetInboxResponse::default())
    }
}
#[async_trait]
pub trait RPC_Profile_Handler {
    async fn ProfileSetSettings(
        up: &UserParam,
        param: pb::ProfileSetSettingsParam,
    ) -> Result<pb::ProfileSetSettingsResponse, GenErr> {
        Ok(pb::ProfileSetSettingsResponse::default())
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
    async fn SharedEcho(
        up: &UserParam,
        param: pb::SharedEchoParam,
    ) -> Result<pb::SharedEchoResponse, GenErr> {
        Ok(pb::SharedEchoResponse::default())
    }
    async fn SharedCheckUserName(
        up: &UserParam,
        param: pb::SharedCheckUserNameParam,
    ) -> Result<pb::SharedCheckUserNameResponse, GenErr> {
        Ok(pb::SharedCheckUserNameResponse::default())
    }
}
#[async_trait]
pub trait RPC_Shop_Handler {
    async fn ShopEEE(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopCreateShop(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopEditShop(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopPauseShop(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopTerminateShop(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopAddProduct(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopEditProduct(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopDeleteProduct(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopAddProductToBasket(
        up: &UserParam,
        param: pb::Param,
    ) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopRemoveProductFromBasket(
        up: &UserParam,
        param: pb::Param,
    ) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopCheckoutBasket(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopLikeProduct(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopUnLikeProduct(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopLikeShop(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopUnLikeShop(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopGetFull(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopProductList(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopGetBasketList(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopGetLikedProductsList(
        up: &UserParam,
        param: pb::Param,
    ) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopGetLikedShopsList(
        up: &UserParam,
        param: pb::Param,
    ) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopGetOrderList(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopGetRefundList(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopCancelOrder(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopEditProductInfo(up: &UserParam, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopEditProductPrice(
        up: &UserParam,
        param: pb::Param,
    ) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopEditProductInventory(
        up: &UserParam,
        param: pb::Param,
    ) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
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
    async fn UserRegisterUser(
        up: &UserParam,
        param: pb::UserRegisterUserParam,
    ) -> Result<pb::UserRegisterUserResponse, GenErr> {
        Ok(pb::UserRegisterUserResponse::default())
    }
    async fn UserEditUser(
        up: &UserParam,
        param: pb::UserEditUserParam,
    ) -> Result<pb::UserEditUserResponse, GenErr> {
        Ok(pb::UserEditUserResponse::default())
    }
    async fn UserDeleteSendCode(
        up: &UserParam,
        param: pb::UserDeleteSendCodeParam,
    ) -> Result<pb::UserDeleteSendCodeResponse, GenErr> {
        Ok(pb::UserDeleteSendCodeResponse::default())
    }
    async fn UserDeleteConfirmCode(
        up: &UserParam,
        param: pb::UserDeleteConfirmCodeParam,
    ) -> Result<pb::UserDeleteConfirmCodeResponse, GenErr> {
        Ok(pb::UserDeleteConfirmCodeResponse::default())
    }
    async fn UserDeleteUser(
        up: &UserParam,
        param: pb::UserDeleteUserParam,
    ) -> Result<pb::UserDeleteUserResponse, GenErr> {
        Ok(pb::UserDeleteUserResponse::default())
    }
    async fn UserChangePhoneNumber(
        up: &UserParam,
        param: pb::UserChangePhoneNumberParam,
    ) -> Result<pb::UserChangePhoneNumberResponse, GenErr> {
        Ok(pb::UserChangePhoneNumberResponse::default())
    }
    async fn UserRemoveSession(
        up: &UserParam,
        param: pb::UserRemoveSessionParam,
    ) -> Result<pb::UserRemoveSessionResponse, GenErr> {
        Ok(pb::UserRemoveSessionResponse::default())
    }
    async fn UserRemoveOtherSessions(
        up: &UserParam,
        param: pb::UserRemoveOtherParam,
    ) -> Result<pb::UserRemoveOtherResponse, GenErr> {
        Ok(pb::UserRemoveOtherResponse::default())
    }
    async fn UserGetMe(
        up: &UserParam,
        param: pb::UserGetMeParam,
    ) -> Result<pb::UserGetMeResponse, GenErr> {
        Ok(pb::UserGetMeResponse::default())
    }
    async fn UserGetActiveSessions(
        up: &UserParam,
        param: pb::UserGetActiveSessionsParam,
    ) -> Result<pb::UserGetActiveSessionsResponse, GenErr> {
        Ok(pb::UserGetActiveSessionsResponse::default())
    }
}

#[async_trait]
pub trait IPC_CMaster_Handler2: Send + Sync {
    async fn GetNextId(&self, param: pb::GetNextIdParam) -> Result<pb::GetNextIdResponse, GenErr> {
        Ok(pb::GetNextIdResponse::default())
    }
}
#[async_trait]
pub trait RPC_Auth_Handler2: Send + Sync {
    async fn AuthSendCode(
        &self,
        param: pb::AuthSendCodeParam,
    ) -> Result<pb::AuthSendCodeResponse, GenErr> {
        Ok(pb::AuthSendCodeResponse::default())
    }
    async fn AuthLogIn(&self, param: pb::AuthLogInParam) -> Result<pb::AuthLogInResponse, GenErr> {
        Ok(pb::AuthLogInResponse::default())
    }
    async fn AuthLogOut(
        &self,
        param: pb::AuthLogOutParam,
    ) -> Result<pb::AuthLogOutResponse, GenErr> {
        Ok(pb::AuthLogOutResponse::default())
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
    async fn ChannelBlockProfile(
        &self,
        param: pb::ChannelBlockProfileParam,
    ) -> Result<pb::ChannelBlockProfileResponse, GenErr> {
        Ok(pb::ChannelBlockProfileResponse::default())
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
    async fn ChannelDeleteMessages(
        &self,
        param: pb::ChannelDeleteMessagesParam,
    ) -> Result<pb::ChannelDeleteMessagesResponse, GenErr> {
        Ok(pb::ChannelDeleteMessagesResponse::default())
    }
    async fn ChannelLikeMessage(
        &self,
        param: pb::ChannelLikeMessageParam,
    ) -> Result<pb::ChannelLikeMessageResponse, GenErr> {
        Ok(pb::ChannelLikeMessageResponse::default())
    }
    async fn ChannelUnLikeMessage(
        &self,
        param: pb::ChannelUnLikeMessageParam,
    ) -> Result<pb::ChannelUnLikeMessageResponse, GenErr> {
        Ok(pb::ChannelUnLikeMessageResponse::default())
    }
    async fn ChannelReShareMessage(
        &self,
        param: pb::ChannelReShareMessageParam,
    ) -> Result<pb::ChannelReShareMessageResponse, GenErr> {
        Ok(pb::ChannelReShareMessageResponse::default())
    }
    async fn ChannelUnReShareMessage(
        &self,
        param: pb::ChannelUnReShareMessageParam,
    ) -> Result<pb::ChannelUnReShareMessageResponse, GenErr> {
        Ok(pb::ChannelUnReShareMessageResponse::default())
    }
    async fn ChannelAddComment(
        &self,
        param: pb::ChannelAddCommentParam,
    ) -> Result<pb::ChannelAddCommentResponse, GenErr> {
        Ok(pb::ChannelAddCommentResponse::default())
    }
    async fn ChannelDeleteComment(
        &self,
        param: pb::ChannelDeleteCommentParam,
    ) -> Result<pb::ChannelDeleteCommentResponse, GenErr> {
        Ok(pb::ChannelDeleteCommentResponse::default())
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
    async fn ChannelAvatarAdd(
        &self,
        param: pb::ChannelAvatarAddParam,
    ) -> Result<pb::ChannelAvatarAddResponse, GenErr> {
        Ok(pb::ChannelAvatarAddResponse::default())
    }
    async fn ChannelAvatarDelete(
        &self,
        param: pb::ChannelAvatarDeleteParam,
    ) -> Result<pb::ChannelAvatarDeleteResponse, GenErr> {
        Ok(pb::ChannelAvatarDeleteResponse::default())
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
    async fn ChannelAvatarGetList(
        &self,
        param: pb::ChannelAvatarGetListParam,
    ) -> Result<pb::ChannelAvatarGetListResponse, GenErr> {
        Ok(pb::ChannelAvatarGetListResponse::default())
    }
    async fn ChannelGetInbox(
        &self,
        param: pb::ChannelGetInboxParam,
    ) -> Result<pb::ChannelGetInboxResponse, GenErr> {
        Ok(pb::ChannelGetInboxResponse::default())
    }
    async fn ChannelGetFollowings(
        &self,
        param: pb::ChannelGetFollowingsParam,
    ) -> Result<pb::ChannelGetFollowingsResponse, GenErr> {
        Ok(pb::ChannelGetFollowingsResponse::default())
    }
}
#[async_trait]
pub trait RPC_Chat_Handler2: Send + Sync {
    async fn ChatDeleteChat(
        &self,
        param: pb::ChatDeleteChatParam,
    ) -> Result<pb::ChatDeleteChatResponse, GenErr> {
        Ok(pb::ChatDeleteChatResponse::default())
    }
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
    async fn ChatSetNotification(
        &self,
        param: pb::ChatSetNotificationParam,
    ) -> Result<pb::ChatSetNotificationResponse, GenErr> {
        Ok(pb::ChatSetNotificationResponse::default())
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
    async fn ChatGetInbox(
        &self,
        param: pb::ChatGetInboxParam,
    ) -> Result<pb::ChatGetInboxResponse, GenErr> {
        Ok(pb::ChatGetInboxResponse::default())
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
    async fn GroupBanMember(
        &self,
        param: pb::GroupBanMemberParam,
    ) -> Result<pb::GroupBanMemberResponse, GenErr> {
        Ok(pb::GroupBanMemberResponse::default())
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
    async fn GroupAddMember(
        &self,
        param: pb::GroupAddMemberParam,
    ) -> Result<pb::GroupAddMemberResponse, GenErr> {
        Ok(pb::GroupAddMemberResponse::default())
    }
    async fn GroupSetNotification(
        &self,
        param: pb::GroupSetNotificationParam,
    ) -> Result<pb::GroupSetNotificationResponse, GenErr> {
        Ok(pb::GroupSetNotificationResponse::default())
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
    async fn GroupAvatarAdd(
        &self,
        param: pb::GroupAvatarAddParam,
    ) -> Result<pb::GroupAvatarAddResponse, GenErr> {
        Ok(pb::GroupAvatarAddResponse::default())
    }
    async fn GroupAvatarDelete(
        &self,
        param: pb::GroupAvatarDeleteParam,
    ) -> Result<pb::GroupAvatarDeleteResponse, GenErr> {
        Ok(pb::GroupAvatarDeleteResponse::default())
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
        param: pb::GroupGetFullParam,
    ) -> Result<pb::GroupGetFullResponse, GenErr> {
        Ok(pb::GroupGetFullResponse::default())
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
    async fn GroupAvatarGetList(
        &self,
        param: pb::GroupAvatarGetListParam,
    ) -> Result<pb::GroupAvatarGetListResponse, GenErr> {
        Ok(pb::GroupAvatarGetListResponse::default())
    }
    async fn GroupGetInbox(
        &self,
        param: pb::GroupGetInboxParam,
    ) -> Result<pb::GroupGetInboxResponse, GenErr> {
        Ok(pb::GroupGetInboxResponse::default())
    }
}
#[async_trait]
pub trait RPC_Profile_Handler2: Send + Sync {
    async fn ProfileSetSettings(
        &self,
        param: pb::ProfileSetSettingsParam,
    ) -> Result<pb::ProfileSetSettingsResponse, GenErr> {
        Ok(pb::ProfileSetSettingsResponse::default())
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
    async fn SharedEcho(
        &self,
        param: pb::SharedEchoParam,
    ) -> Result<pb::SharedEchoResponse, GenErr> {
        Ok(pb::SharedEchoResponse::default())
    }
    async fn SharedCheckUserName(
        &self,
        param: pb::SharedCheckUserNameParam,
    ) -> Result<pb::SharedCheckUserNameResponse, GenErr> {
        Ok(pb::SharedCheckUserNameResponse::default())
    }
}
#[async_trait]
pub trait RPC_Shop_Handler2: Send + Sync {
    async fn ShopEEE(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopCreateShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopEditShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopPauseShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopTerminateShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopAddProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopEditProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopDeleteProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopAddProductToBasket(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopRemoveProductFromBasket(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopCheckoutBasket(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopLikeProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopUnLikeProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopLikeShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopUnLikeShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopGetFull(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopProductList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopGetBasketList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopGetLikedProductsList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopGetLikedShopsList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopGetOrderList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopGetRefundList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopCancelOrder(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopEditProductInfo(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopEditProductPrice(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
    }
    async fn ShopEditProductInventory(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        Ok(pb::Response::default())
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
    async fn UserRegisterUser(
        &self,
        param: pb::UserRegisterUserParam,
    ) -> Result<pb::UserRegisterUserResponse, GenErr> {
        Ok(pb::UserRegisterUserResponse::default())
    }
    async fn UserEditUser(
        &self,
        param: pb::UserEditUserParam,
    ) -> Result<pb::UserEditUserResponse, GenErr> {
        Ok(pb::UserEditUserResponse::default())
    }
    async fn UserDeleteSendCode(
        &self,
        param: pb::UserDeleteSendCodeParam,
    ) -> Result<pb::UserDeleteSendCodeResponse, GenErr> {
        Ok(pb::UserDeleteSendCodeResponse::default())
    }
    async fn UserDeleteConfirmCode(
        &self,
        param: pb::UserDeleteConfirmCodeParam,
    ) -> Result<pb::UserDeleteConfirmCodeResponse, GenErr> {
        Ok(pb::UserDeleteConfirmCodeResponse::default())
    }
    async fn UserDeleteUser(
        &self,
        param: pb::UserDeleteUserParam,
    ) -> Result<pb::UserDeleteUserResponse, GenErr> {
        Ok(pb::UserDeleteUserResponse::default())
    }
    async fn UserChangePhoneNumber(
        &self,
        param: pb::UserChangePhoneNumberParam,
    ) -> Result<pb::UserChangePhoneNumberResponse, GenErr> {
        Ok(pb::UserChangePhoneNumberResponse::default())
    }
    async fn UserRemoveSession(
        &self,
        param: pb::UserRemoveSessionParam,
    ) -> Result<pb::UserRemoveSessionResponse, GenErr> {
        Ok(pb::UserRemoveSessionResponse::default())
    }
    async fn UserRemoveOtherSessions(
        &self,
        param: pb::UserRemoveOtherParam,
    ) -> Result<pb::UserRemoveOtherResponse, GenErr> {
        Ok(pb::UserRemoveOtherResponse::default())
    }
    async fn UserGetMe(&self, param: pb::UserGetMeParam) -> Result<pb::UserGetMeResponse, GenErr> {
        Ok(pb::UserGetMeResponse::default())
    }
    async fn UserGetActiveSessions(
        &self,
        param: pb::UserGetActiveSessionsParam,
    ) -> Result<pb::UserGetActiveSessionsResponse, GenErr> {
        Ok(pb::UserGetActiveSessionsResponse::default())
    }
}

#[async_trait]
pub trait All_Rpc_Handler:
    IPC_CMaster_Handler2
    + RPC_Auth_Handler2
    + RPC_Channel_Handler2
    + RPC_Chat_Handler2
    + RPC_Group_Handler2
    + RPC_Profile_Handler2
    + RPC_Sample_Handler2
    + RPC_Shared_Handler2
    + RPC_Shop_Handler2
    + RPC_Upload_Handler2
    + RPC_User_Handler2
    + Clone
    + Send
    + Sync
{
}

pub mod method_ids {
    // Service: IPC_CMaster
    pub const GetNextId: u32 = 929964228;

    // Service: RPC_Auth
    pub const AuthSendCode: u32 = 331194686;
    pub const AuthLogIn: u32 = 1175605097;
    pub const AuthLogOut: u32 = 370097782;

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
    pub const ChannelBlockProfile: u32 = 782736993;
    pub const ChannelSendMessage: u32 = 1200751231;
    pub const ChannelEditMessage: u32 = 727437726;
    pub const ChannelDeleteMessages: u32 = 2124822181;
    pub const ChannelLikeMessage: u32 = 1349412095;
    pub const ChannelUnLikeMessage: u32 = 1770909427;
    pub const ChannelReShareMessage: u32 = 517136377;
    pub const ChannelUnReShareMessage: u32 = 1173971596;
    pub const ChannelAddComment: u32 = 58338736;
    pub const ChannelDeleteComment: u32 = 1510744722;
    pub const ChannelPinMessage: u32 = 259263709;
    pub const ChannelUnPinMessage: u32 = 113943649;
    pub const ChannelAvatarAdd: u32 = 1021808696;
    pub const ChannelAvatarDelete: u32 = 1626010891;
    pub const ChannelSendDoingAction: u32 = 973237257;
    pub const ChannelReportChannel: u32 = 792938145;
    pub const ChannelReportMessage: u32 = 2053528327;
    pub const ChannelGetFull: u32 = 1684531258;
    pub const ChannelGetMessagesList: u32 = 1339072968;
    pub const ChannelGetMediaList: u32 = 985772653;
    pub const ChannelGetAuthors: u32 = 1373284924;
    pub const ChannelGetFollowers: u32 = 1747172143;
    pub const ChannelGetSubscribers: u32 = 2146806736;
    pub const ChannelBlocked: u32 = 1674411747;
    pub const ChannelAvatarGetList: u32 = 1925044843;
    pub const ChannelGetInbox: u32 = 811688572;
    pub const ChannelGetFollowings: u32 = 1838438980;

    // Service: RPC_Chat
    pub const ChatDeleteChat: u32 = 130139014;
    pub const ChatSendMessage: u32 = 1131621475;
    pub const ChatEditMessage: u32 = 1806258329;
    pub const ChatDeleteMessages: u32 = 933526170;
    pub const ChatDeleteHistory: u32 = 1088992782;
    pub const ChatSetNotification: u32 = 740262999;
    pub const ChatSendDoingAction: u32 = 1319324241;
    pub const ChatReportChat: u32 = 1345425871;
    pub const ChatGetFull: u32 = 1768678453;
    pub const ChatGetMessagesList: u32 = 121549718;
    pub const ChatGetMediaList: u32 = 1346774525;
    pub const ChatGetInbox: u32 = 1556609489;

    // Service: RPC_Group
    pub const GroupCreateGroup: u32 = 1205960678;
    pub const GroupEditGroup: u32 = 1665019493;
    pub const GroupDeleteGroup: u32 = 365183375;
    pub const GroupAddAdmin: u32 = 958971956;
    pub const GroupRemoveMember: u32 = 2012702964;
    pub const GroupChangeMemberLevel: u32 = 589574238;
    pub const GroupChangeMemberPermission: u32 = 2132464067;
    pub const GroupBanMember: u32 = 548504852;
    pub const GroupJoinGroup: u32 = 591743429;
    pub const GroupLeaveGroup: u32 = 361834630;
    pub const GroupAddMember: u32 = 676599227;
    pub const GroupSetNotification: u32 = 1169707665;
    pub const GroupChangePrivacy: u32 = 1497988410;
    pub const GroupChangeDefaultPermission: u32 = 605792138;
    pub const GroupRevokeLink: u32 = 406592509;
    pub const GroupChangeUsername: u32 = 832997038;
    pub const GroupSendMessage: u32 = 599852950;
    pub const GroupEditMessage: u32 = 742937895;
    pub const GroupDeleteMessages: u32 = 276700675;
    pub const GroupDeleteHistory: u32 = 1270953793;
    pub const GroupPinMessage: u32 = 184560027;
    pub const GroupUnPinMessage: u32 = 1290613173;
    pub const GroupAvatarAdd: u32 = 1202058216;
    pub const GroupAvatarDelete: u32 = 775862697;
    pub const GroupSendDoingAction: u32 = 2022474356;
    pub const GroupReportGroup: u32 = 1759704420;
    pub const GroupGetFull: u32 = 200351324;
    pub const GroupGetMessagesList: u32 = 1541835459;
    pub const GroupGetMediaList: u32 = 2143016912;
    pub const GroupGetMembersList: u32 = 429215412;
    pub const GroupGetAdminsList: u32 = 332260610;
    pub const GroupAvatarGetList: u32 = 939443722;
    pub const GroupGetInbox: u32 = 1722848299;

    // Service: RPC_Profile
    pub const ProfileSetSettings: u32 = 308739811;

    // Service: RPC_Sample
    pub const GetUsers1: u32 = 486248681;
    pub const GetProfiles: u32 = 822554282;
    pub const GetChannels: u32 = 1734748927;
    pub const GetDirects: u32 = 558085683;
    pub const GetMessages: u32 = 1160951872;

    // Service: RPC_Shared
    pub const SharedEcho: u32 = 57075660;
    pub const SharedCheckUserName: u32 = 435850322;

    // Service: RPC_Shop
    pub const ShopEEE: u32 = 912385050;
    pub const ShopCreateShop: u32 = 1986531780;
    pub const ShopEditShop: u32 = 455084262;
    pub const ShopPauseShop: u32 = 1673552241;
    pub const ShopTerminateShop: u32 = 1507072614;
    pub const ShopAddProduct: u32 = 1965565139;
    pub const ShopEditProduct: u32 = 1399457308;
    pub const ShopDeleteProduct: u32 = 847818259;
    pub const ShopAddProductToBasket: u32 = 968515528;
    pub const ShopRemoveProductFromBasket: u32 = 1393049106;
    pub const ShopCheckoutBasket: u32 = 1428199101;
    pub const ShopLikeProduct: u32 = 1327382465;
    pub const ShopUnLikeProduct: u32 = 280445800;
    pub const ShopLikeShop: u32 = 159832049;
    pub const ShopUnLikeShop: u32 = 2039870177;
    pub const ShopGetFull: u32 = 1805685529;
    pub const ShopProductList: u32 = 92142795;
    pub const ShopGetBasketList: u32 = 1792928614;
    pub const ShopGetLikedProductsList: u32 = 46651378;
    pub const ShopGetLikedShopsList: u32 = 1765558200;
    pub const ShopGetOrderList: u32 = 615052791;
    pub const ShopGetRefundList: u32 = 1024605829;
    pub const ShopCancelOrder: u32 = 1984969964;
    pub const ShopEditProductInfo: u32 = 1420581989;
    pub const ShopEditProductPrice: u32 = 735302162;
    pub const ShopEditProductInventory: u32 = 395598602;

    // Service: RPC_Upload
    pub const UploadFile: u32 = 1702285478;

    // Service: RPC_User
    pub const UserRegisterUser: u32 = 1322893071;
    pub const UserEditUser: u32 = 1645828453;
    pub const UserDeleteSendCode: u32 = 1007116723;
    pub const UserDeleteConfirmCode: u32 = 2120919452;
    pub const UserDeleteUser: u32 = 1206281254;
    pub const UserChangePhoneNumber: u32 = 51450414;
    pub const UserRemoveSession: u32 = 1173893234;
    pub const UserRemoveOtherSessions: u32 = 2042311148;
    pub const UserGetMe: u32 = 1362817625;
    pub const UserGetActiveSessions: u32 = 214259267;

    pub const ExampleChangePhoneNumber8: u32 = 79874;
}

#[derive(Debug)]
pub enum MethodIds {
    // Service: IPC_CMaster
    GetNextId = 929964228,

    // Service: RPC_Auth
    AuthSendCode = 331194686,
    AuthLogIn = 1175605097,
    AuthLogOut = 370097782,

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
    ChannelBlockProfile = 782736993,
    ChannelSendMessage = 1200751231,
    ChannelEditMessage = 727437726,
    ChannelDeleteMessages = 2124822181,
    ChannelLikeMessage = 1349412095,
    ChannelUnLikeMessage = 1770909427,
    ChannelReShareMessage = 517136377,
    ChannelUnReShareMessage = 1173971596,
    ChannelAddComment = 58338736,
    ChannelDeleteComment = 1510744722,
    ChannelPinMessage = 259263709,
    ChannelUnPinMessage = 113943649,
    ChannelAvatarAdd = 1021808696,
    ChannelAvatarDelete = 1626010891,
    ChannelSendDoingAction = 973237257,
    ChannelReportChannel = 792938145,
    ChannelReportMessage = 2053528327,
    ChannelGetFull = 1684531258,
    ChannelGetMessagesList = 1339072968,
    ChannelGetMediaList = 985772653,
    ChannelGetAuthors = 1373284924,
    ChannelGetFollowers = 1747172143,
    ChannelGetSubscribers = 2146806736,
    ChannelBlocked = 1674411747,
    ChannelAvatarGetList = 1925044843,
    ChannelGetInbox = 811688572,
    ChannelGetFollowings = 1838438980,

    // Service: RPC_Chat
    ChatDeleteChat = 130139014,
    ChatSendMessage = 1131621475,
    ChatEditMessage = 1806258329,
    ChatDeleteMessages = 933526170,
    ChatDeleteHistory = 1088992782,
    ChatSetNotification = 740262999,
    ChatSendDoingAction = 1319324241,
    ChatReportChat = 1345425871,
    ChatGetFull = 1768678453,
    ChatGetMessagesList = 121549718,
    ChatGetMediaList = 1346774525,
    ChatGetInbox = 1556609489,

    // Service: RPC_Group
    GroupCreateGroup = 1205960678,
    GroupEditGroup = 1665019493,
    GroupDeleteGroup = 365183375,
    GroupAddAdmin = 958971956,
    GroupRemoveMember = 2012702964,
    GroupChangeMemberLevel = 589574238,
    GroupChangeMemberPermission = 2132464067,
    GroupBanMember = 548504852,
    GroupJoinGroup = 591743429,
    GroupLeaveGroup = 361834630,
    GroupAddMember = 676599227,
    GroupSetNotification = 1169707665,
    GroupChangePrivacy = 1497988410,
    GroupChangeDefaultPermission = 605792138,
    GroupRevokeLink = 406592509,
    GroupChangeUsername = 832997038,
    GroupSendMessage = 599852950,
    GroupEditMessage = 742937895,
    GroupDeleteMessages = 276700675,
    GroupDeleteHistory = 1270953793,
    GroupPinMessage = 184560027,
    GroupUnPinMessage = 1290613173,
    GroupAvatarAdd = 1202058216,
    GroupAvatarDelete = 775862697,
    GroupSendDoingAction = 2022474356,
    GroupReportGroup = 1759704420,
    GroupGetFull = 200351324,
    GroupGetMessagesList = 1541835459,
    GroupGetMediaList = 2143016912,
    GroupGetMembersList = 429215412,
    GroupGetAdminsList = 332260610,
    GroupAvatarGetList = 939443722,
    GroupGetInbox = 1722848299,

    // Service: RPC_Profile
    ProfileSetSettings = 308739811,

    // Service: RPC_Sample
    GetUsers1 = 486248681,
    GetProfiles = 822554282,
    GetChannels = 1734748927,
    GetDirects = 558085683,
    GetMessages = 1160951872,

    // Service: RPC_Shared
    SharedEcho = 57075660,
    SharedCheckUserName = 435850322,

    // Service: RPC_Shop
    ShopEEE = 912385050,
    ShopCreateShop = 1986531780,
    ShopEditShop = 455084262,
    ShopPauseShop = 1673552241,
    ShopTerminateShop = 1507072614,
    ShopAddProduct = 1965565139,
    ShopEditProduct = 1399457308,
    ShopDeleteProduct = 847818259,
    ShopAddProductToBasket = 968515528,
    ShopRemoveProductFromBasket = 1393049106,
    ShopCheckoutBasket = 1428199101,
    ShopLikeProduct = 1327382465,
    ShopUnLikeProduct = 280445800,
    ShopLikeShop = 159832049,
    ShopUnLikeShop = 2039870177,
    ShopGetFull = 1805685529,
    ShopProductList = 92142795,
    ShopGetBasketList = 1792928614,
    ShopGetLikedProductsList = 46651378,
    ShopGetLikedShopsList = 1765558200,
    ShopGetOrderList = 615052791,
    ShopGetRefundList = 1024605829,
    ShopCancelOrder = 1984969964,
    ShopEditProductInfo = 1420581989,
    ShopEditProductPrice = 735302162,
    ShopEditProductInventory = 395598602,

    // Service: RPC_Upload
    UploadFile = 1702285478,

    // Service: RPC_User
    UserRegisterUser = 1322893071,
    UserEditUser = 1645828453,
    UserDeleteSendCode = 1007116723,
    UserDeleteConfirmCode = 2120919452,
    UserDeleteUser = 1206281254,
    UserChangePhoneNumber = 51450414,
    UserRemoveSession = 1173893234,
    UserRemoveOtherSessions = 2042311148,
    UserGetMe = 1362817625,
    UserGetActiveSessions = 214259267,
}

pub fn invoke_to_parsed(invoke: &pb::Invoke) -> Result<RpcInvoke, GenErr> {
    use RpcServiceData::*;
    let rpc = match invoke.method {
        // IPC_CMaster
        method_ids::GetNextId => {
            let rpc_param: pb::GetNextIdParam = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 929964228 as i64,
                rpc_service: IPC_CMaster(IPC_CMaster_MethodData::GetNextId(rpc_param)),
            }
        }

        // RPC_Auth
        method_ids::AuthSendCode => {
            let rpc_param: pb::AuthSendCodeParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 331194686 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::AuthSendCode(rpc_param)),
            }
        }

        method_ids::AuthLogIn => {
            let rpc_param: pb::AuthLogInParam = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1175605097 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::AuthLogIn(rpc_param)),
            }
        }

        method_ids::AuthLogOut => {
            let rpc_param: pb::AuthLogOutParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 370097782 as i64,
                rpc_service: RPC_Auth(RPC_Auth_MethodData::AuthLogOut(rpc_param)),
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

        method_ids::ChannelBlockProfile => {
            let rpc_param: pb::ChannelBlockProfileParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 782736993 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelBlockProfile(rpc_param)),
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

        method_ids::ChannelDeleteMessages => {
            let rpc_param: pb::ChannelDeleteMessagesParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2124822181 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelDeleteMessages(rpc_param)),
            }
        }

        method_ids::ChannelLikeMessage => {
            let rpc_param: pb::ChannelLikeMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1349412095 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelLikeMessage(rpc_param)),
            }
        }

        method_ids::ChannelUnLikeMessage => {
            let rpc_param: pb::ChannelUnLikeMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1770909427 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelUnLikeMessage(rpc_param)),
            }
        }

        method_ids::ChannelReShareMessage => {
            let rpc_param: pb::ChannelReShareMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 517136377 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelReShareMessage(rpc_param)),
            }
        }

        method_ids::ChannelUnReShareMessage => {
            let rpc_param: pb::ChannelUnReShareMessageParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1173971596 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelUnReShareMessage(
                    rpc_param,
                )),
            }
        }

        method_ids::ChannelAddComment => {
            let rpc_param: pb::ChannelAddCommentParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 58338736 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAddComment(rpc_param)),
            }
        }

        method_ids::ChannelDeleteComment => {
            let rpc_param: pb::ChannelDeleteCommentParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1510744722 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelDeleteComment(rpc_param)),
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

        method_ids::ChannelAvatarAdd => {
            let rpc_param: pb::ChannelAvatarAddParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1021808696 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAvatarAdd(rpc_param)),
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

        method_ids::ChannelAvatarGetList => {
            let rpc_param: pb::ChannelAvatarGetListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1925044843 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelAvatarGetList(rpc_param)),
            }
        }

        method_ids::ChannelGetInbox => {
            let rpc_param: pb::ChannelGetInboxParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 811688572 as i64,
                rpc_service: RPC_Channel(RPC_Channel_MethodData::ChannelGetInbox(rpc_param)),
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

        // RPC_Chat
        method_ids::ChatDeleteChat => {
            let rpc_param: pb::ChatDeleteChatParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 130139014 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatDeleteChat(rpc_param)),
            }
        }

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

        method_ids::ChatSetNotification => {
            let rpc_param: pb::ChatSetNotificationParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 740262999 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatSetNotification(rpc_param)),
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

        method_ids::ChatGetInbox => {
            let rpc_param: pb::ChatGetInboxParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1556609489 as i64,
                rpc_service: RPC_Chat(RPC_Chat_MethodData::ChatGetInbox(rpc_param)),
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

        method_ids::GroupBanMember => {
            let rpc_param: pb::GroupBanMemberParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 548504852 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupBanMember(rpc_param)),
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

        method_ids::GroupAddMember => {
            let rpc_param: pb::GroupAddMemberParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 676599227 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAddMember(rpc_param)),
            }
        }

        method_ids::GroupSetNotification => {
            let rpc_param: pb::GroupSetNotificationParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1169707665 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupSetNotification(rpc_param)),
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

        method_ids::GroupAvatarAdd => {
            let rpc_param: pb::GroupAvatarAddParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1202058216 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAvatarAdd(rpc_param)),
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
            let rpc_param: pb::GroupGetFullParam =
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

        method_ids::GroupAvatarGetList => {
            let rpc_param: pb::GroupAvatarGetListParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 939443722 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupAvatarGetList(rpc_param)),
            }
        }

        method_ids::GroupGetInbox => {
            let rpc_param: pb::GroupGetInboxParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1722848299 as i64,
                rpc_service: RPC_Group(RPC_Group_MethodData::GroupGetInbox(rpc_param)),
            }
        }

        // RPC_Profile
        method_ids::ProfileSetSettings => {
            let rpc_param: pb::ProfileSetSettingsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 308739811 as i64,
                rpc_service: RPC_Profile(RPC_Profile_MethodData::ProfileSetSettings(rpc_param)),
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
        method_ids::SharedEcho => {
            let rpc_param: pb::SharedEchoParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 57075660 as i64,
                rpc_service: RPC_Shared(RPC_Shared_MethodData::SharedEcho(rpc_param)),
            }
        }

        method_ids::SharedCheckUserName => {
            let rpc_param: pb::SharedCheckUserNameParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 435850322 as i64,
                rpc_service: RPC_Shared(RPC_Shared_MethodData::SharedCheckUserName(rpc_param)),
            }
        }

        // RPC_Shop
        method_ids::ShopEEE => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 912385050 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopEEE(rpc_param)),
            }
        }

        method_ids::ShopCreateShop => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1986531780 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopCreateShop(rpc_param)),
            }
        }

        method_ids::ShopEditShop => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 455084262 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopEditShop(rpc_param)),
            }
        }

        method_ids::ShopPauseShop => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1673552241 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopPauseShop(rpc_param)),
            }
        }

        method_ids::ShopTerminateShop => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1507072614 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopTerminateShop(rpc_param)),
            }
        }

        method_ids::ShopAddProduct => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1965565139 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopAddProduct(rpc_param)),
            }
        }

        method_ids::ShopEditProduct => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1399457308 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopEditProduct(rpc_param)),
            }
        }

        method_ids::ShopDeleteProduct => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 847818259 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopDeleteProduct(rpc_param)),
            }
        }

        method_ids::ShopAddProductToBasket => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 968515528 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopAddProductToBasket(rpc_param)),
            }
        }

        method_ids::ShopRemoveProductFromBasket => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1393049106 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopRemoveProductFromBasket(rpc_param)),
            }
        }

        method_ids::ShopCheckoutBasket => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1428199101 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopCheckoutBasket(rpc_param)),
            }
        }

        method_ids::ShopLikeProduct => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1327382465 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopLikeProduct(rpc_param)),
            }
        }

        method_ids::ShopUnLikeProduct => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 280445800 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopUnLikeProduct(rpc_param)),
            }
        }

        method_ids::ShopLikeShop => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 159832049 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopLikeShop(rpc_param)),
            }
        }

        method_ids::ShopUnLikeShop => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2039870177 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopUnLikeShop(rpc_param)),
            }
        }

        method_ids::ShopGetFull => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1805685529 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopGetFull(rpc_param)),
            }
        }

        method_ids::ShopProductList => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 92142795 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopProductList(rpc_param)),
            }
        }

        method_ids::ShopGetBasketList => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1792928614 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopGetBasketList(rpc_param)),
            }
        }

        method_ids::ShopGetLikedProductsList => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 46651378 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopGetLikedProductsList(rpc_param)),
            }
        }

        method_ids::ShopGetLikedShopsList => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1765558200 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopGetLikedShopsList(rpc_param)),
            }
        }

        method_ids::ShopGetOrderList => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 615052791 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopGetOrderList(rpc_param)),
            }
        }

        method_ids::ShopGetRefundList => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1024605829 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopGetRefundList(rpc_param)),
            }
        }

        method_ids::ShopCancelOrder => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1984969964 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopCancelOrder(rpc_param)),
            }
        }

        method_ids::ShopEditProductInfo => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1420581989 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopEditProductInfo(rpc_param)),
            }
        }

        method_ids::ShopEditProductPrice => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 735302162 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopEditProductPrice(rpc_param)),
            }
        }

        method_ids::ShopEditProductInventory => {
            let rpc_param: pb::Param = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 395598602 as i64,
                rpc_service: RPC_Shop(RPC_Shop_MethodData::ShopEditProductInventory(rpc_param)),
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
        method_ids::UserRegisterUser => {
            let rpc_param: pb::UserRegisterUserParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1322893071 as i64,
                rpc_service: RPC_User(RPC_User_MethodData::UserRegisterUser(rpc_param)),
            }
        }

        method_ids::UserEditUser => {
            let rpc_param: pb::UserEditUserParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1645828453 as i64,
                rpc_service: RPC_User(RPC_User_MethodData::UserEditUser(rpc_param)),
            }
        }

        method_ids::UserDeleteSendCode => {
            let rpc_param: pb::UserDeleteSendCodeParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1007116723 as i64,
                rpc_service: RPC_User(RPC_User_MethodData::UserDeleteSendCode(rpc_param)),
            }
        }

        method_ids::UserDeleteConfirmCode => {
            let rpc_param: pb::UserDeleteConfirmCodeParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2120919452 as i64,
                rpc_service: RPC_User(RPC_User_MethodData::UserDeleteConfirmCode(rpc_param)),
            }
        }

        method_ids::UserDeleteUser => {
            let rpc_param: pb::UserDeleteUserParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1206281254 as i64,
                rpc_service: RPC_User(RPC_User_MethodData::UserDeleteUser(rpc_param)),
            }
        }

        method_ids::UserChangePhoneNumber => {
            let rpc_param: pb::UserChangePhoneNumberParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 51450414 as i64,
                rpc_service: RPC_User(RPC_User_MethodData::UserChangePhoneNumber(rpc_param)),
            }
        }

        method_ids::UserRemoveSession => {
            let rpc_param: pb::UserRemoveSessionParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1173893234 as i64,
                rpc_service: RPC_User(RPC_User_MethodData::UserRemoveSession(rpc_param)),
            }
        }

        method_ids::UserRemoveOtherSessions => {
            let rpc_param: pb::UserRemoveOtherParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 2042311148 as i64,
                rpc_service: RPC_User(RPC_User_MethodData::UserRemoveOtherSessions(rpc_param)),
            }
        }

        method_ids::UserGetMe => {
            let rpc_param: pb::UserGetMeParam = prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 1362817625 as i64,
                rpc_service: RPC_User(RPC_User_MethodData::UserGetMe(rpc_param)),
            }
        }

        method_ids::UserGetActiveSessions => {
            let rpc_param: pb::UserGetActiveSessionsParam =
                prost::Message::decode(invoke.rpc_data.as_slice())?;
            RpcInvoke {
                method_id: 214259267 as i64,
                rpc_service: RPC_User(RPC_User_MethodData::UserGetActiveSessions(rpc_param)),
            }
        }

        _ => {
            panic!("sdf")
        }
    };
    Ok(rpc)
}

pub async fn server_rpc(act: RpcInvoke, reg: &RPC_Registry) -> Result<Vec<u8>, GenErr> {
    let res_v8 = match act.rpc_service {
        RpcServiceData::IPC_CMaster(method) => match method {
            IPC_CMaster_MethodData::GetNextId(param) => {
                let handler = eror(&reg.IPC_CMaster)?;
                let response = handler.GetNextId(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Auth(method) => match method {
            RPC_Auth_MethodData::AuthSendCode(param) => {
                let handler = eror(&reg.RPC_Auth)?;
                let response = handler.AuthSendCode(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Auth_MethodData::AuthLogIn(param) => {
                let handler = eror(&reg.RPC_Auth)?;
                let response = handler.AuthLogIn(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Auth_MethodData::AuthLogOut(param) => {
                let handler = eror(&reg.RPC_Auth)?;
                let response = handler.AuthLogOut(param).await?;
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

            RPC_Channel_MethodData::ChannelBlockProfile(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelBlockProfile(param).await?;
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

            RPC_Channel_MethodData::ChannelDeleteMessages(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelDeleteMessages(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelLikeMessage(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelLikeMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelUnLikeMessage(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelUnLikeMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelReShareMessage(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelReShareMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelUnReShareMessage(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelUnReShareMessage(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelAddComment(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelAddComment(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelDeleteComment(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelDeleteComment(param).await?;
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

            RPC_Channel_MethodData::ChannelAvatarAdd(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelAvatarAdd(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelAvatarDelete(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelAvatarDelete(param).await?;
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

            RPC_Channel_MethodData::ChannelAvatarGetList(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelAvatarGetList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetInbox(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelGetInbox(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Channel_MethodData::ChannelGetFollowings(param) => {
                let handler = eror(&reg.RPC_Channel)?;
                let response = handler.ChannelGetFollowings(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Chat(method) => match method {
            RPC_Chat_MethodData::ChatDeleteChat(param) => {
                let handler = eror(&reg.RPC_Chat)?;
                let response = handler.ChatDeleteChat(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

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

            RPC_Chat_MethodData::ChatSetNotification(param) => {
                let handler = eror(&reg.RPC_Chat)?;
                let response = handler.ChatSetNotification(param).await?;
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

            RPC_Chat_MethodData::ChatGetInbox(param) => {
                let handler = eror(&reg.RPC_Chat)?;
                let response = handler.ChatGetInbox(param).await?;
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

            RPC_Group_MethodData::GroupBanMember(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupBanMember(param).await?;
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

            RPC_Group_MethodData::GroupAddMember(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupAddMember(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupSetNotification(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupSetNotification(param).await?;
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

            RPC_Group_MethodData::GroupAvatarAdd(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupAvatarAdd(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupAvatarDelete(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupAvatarDelete(param).await?;
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

            RPC_Group_MethodData::GroupAvatarGetList(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupAvatarGetList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Group_MethodData::GroupGetInbox(param) => {
                let handler = eror(&reg.RPC_Group)?;
                let response = handler.GroupGetInbox(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Profile(method) => match method {
            RPC_Profile_MethodData::ProfileSetSettings(param) => {
                let handler = eror(&reg.RPC_Profile)?;
                let response = handler.ProfileSetSettings(param).await?;
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
            RPC_Shared_MethodData::SharedEcho(param) => {
                let handler = eror(&reg.RPC_Shared)?;
                let response = handler.SharedEcho(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shared_MethodData::SharedCheckUserName(param) => {
                let handler = eror(&reg.RPC_Shared)?;
                let response = handler.SharedCheckUserName(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },

        RpcServiceData::RPC_Shop(method) => match method {
            RPC_Shop_MethodData::ShopEEE(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopEEE(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopCreateShop(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopCreateShop(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopEditShop(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopEditShop(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopPauseShop(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopPauseShop(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopTerminateShop(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopTerminateShop(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopAddProduct(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopAddProduct(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopEditProduct(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopEditProduct(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopDeleteProduct(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopDeleteProduct(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopAddProductToBasket(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopAddProductToBasket(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopRemoveProductFromBasket(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopRemoveProductFromBasket(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopCheckoutBasket(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopCheckoutBasket(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopLikeProduct(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopLikeProduct(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopUnLikeProduct(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopUnLikeProduct(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopLikeShop(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopLikeShop(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopUnLikeShop(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopUnLikeShop(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopGetFull(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopGetFull(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopProductList(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopProductList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopGetBasketList(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopGetBasketList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopGetLikedProductsList(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopGetLikedProductsList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopGetLikedShopsList(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopGetLikedShopsList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopGetOrderList(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopGetOrderList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopGetRefundList(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopGetRefundList(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopCancelOrder(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopCancelOrder(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopEditProductInfo(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopEditProductInfo(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopEditProductPrice(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopEditProductPrice(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_Shop_MethodData::ShopEditProductInventory(param) => {
                let handler = eror(&reg.RPC_Shop)?;
                let response = handler.ShopEditProductInventory(param).await?;
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
            RPC_User_MethodData::UserRegisterUser(param) => {
                let handler = eror(&reg.RPC_User)?;
                let response = handler.UserRegisterUser(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_User_MethodData::UserEditUser(param) => {
                let handler = eror(&reg.RPC_User)?;
                let response = handler.UserEditUser(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_User_MethodData::UserDeleteSendCode(param) => {
                let handler = eror(&reg.RPC_User)?;
                let response = handler.UserDeleteSendCode(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_User_MethodData::UserDeleteConfirmCode(param) => {
                let handler = eror(&reg.RPC_User)?;
                let response = handler.UserDeleteConfirmCode(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_User_MethodData::UserDeleteUser(param) => {
                let handler = eror(&reg.RPC_User)?;
                let response = handler.UserDeleteUser(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_User_MethodData::UserChangePhoneNumber(param) => {
                let handler = eror(&reg.RPC_User)?;
                let response = handler.UserChangePhoneNumber(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_User_MethodData::UserRemoveSession(param) => {
                let handler = eror(&reg.RPC_User)?;
                let response = handler.UserRemoveSession(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_User_MethodData::UserRemoveOtherSessions(param) => {
                let handler = eror(&reg.RPC_User)?;
                let response = handler.UserRemoveOtherSessions(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_User_MethodData::UserGetMe(param) => {
                let handler = eror(&reg.RPC_User)?;
                let response = handler.UserGetMe(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }

            RPC_User_MethodData::UserGetActiveSessions(param) => {
                let handler = eror(&reg.RPC_User)?;
                let response = handler.UserGetActiveSessions(param).await?;
                let v8 = to_vev8(&response)?;
                v8
            }
        },
    };

    Ok(res_v8)
}

#[derive(Default)]
pub struct RPC_Registry {
    pub IPC_CMaster: Option<Box<Arc<dyn IPC_CMaster_Handler2>>>,
    pub RPC_Auth: Option<Box<Arc<dyn RPC_Auth_Handler2>>>,
    pub RPC_Channel: Option<Box<Arc<dyn RPC_Channel_Handler2>>>,
    pub RPC_Chat: Option<Box<Arc<dyn RPC_Chat_Handler2>>>,
    pub RPC_Group: Option<Box<Arc<dyn RPC_Group_Handler2>>>,
    pub RPC_Profile: Option<Box<Arc<dyn RPC_Profile_Handler2>>>,
    pub RPC_Sample: Option<Box<Arc<dyn RPC_Sample_Handler2>>>,
    pub RPC_Shared: Option<Box<Arc<dyn RPC_Shared_Handler2>>>,
    pub RPC_Shop: Option<Box<Arc<dyn RPC_Shop_Handler2>>>,
    pub RPC_Upload: Option<Box<Arc<dyn RPC_Upload_Handler2>>>,
    pub RPC_User: Option<Box<Arc<dyn RPC_User_Handler2>>>,
}

impl IPC_CMaster_Handler for RPC_Registry {}
impl IPC_CMaster_Handler2 for RPC_Registry {}
impl RPC_Auth_Handler for RPC_Registry {}
impl RPC_Auth_Handler2 for RPC_Registry {}
impl RPC_Channel_Handler for RPC_Registry {}
impl RPC_Channel_Handler2 for RPC_Registry {}
impl RPC_Chat_Handler for RPC_Registry {}
impl RPC_Chat_Handler2 for RPC_Registry {}
impl RPC_Group_Handler for RPC_Registry {}
impl RPC_Group_Handler2 for RPC_Registry {}
impl RPC_Profile_Handler for RPC_Registry {}
impl RPC_Profile_Handler2 for RPC_Registry {}
impl RPC_Sample_Handler for RPC_Registry {}
impl RPC_Sample_Handler2 for RPC_Registry {}
impl RPC_Shared_Handler for RPC_Registry {}
impl RPC_Shared_Handler2 for RPC_Registry {}
impl RPC_Shop_Handler for RPC_Registry {}
impl RPC_Shop_Handler2 for RPC_Registry {}
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
    // service: IPC_CMaster
    pub async fn GetNextId(
        &self,
        param: pb::GetNextIdParam,
    ) -> Result<pb::GetNextIdResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GetNextId).await?;
        Ok(pb_res)
    }

    // service: RPC_Auth
    pub async fn AuthSendCode(
        &self,
        param: pb::AuthSendCodeParam,
    ) -> Result<pb::AuthSendCodeResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::AuthSendCode).await?;
        Ok(pb_res)
    }

    pub async fn AuthLogIn(
        &self,
        param: pb::AuthLogInParam,
    ) -> Result<pb::AuthLogInResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::AuthLogIn).await?;
        Ok(pb_res)
    }

    pub async fn AuthLogOut(
        &self,
        param: pb::AuthLogOutParam,
    ) -> Result<pb::AuthLogOutResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::AuthLogOut).await?;
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

    pub async fn ChannelBlockProfile(
        &self,
        param: pb::ChannelBlockProfileParam,
    ) -> Result<pb::ChannelBlockProfileResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelBlockProfile)
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

    pub async fn ChannelDeleteMessages(
        &self,
        param: pb::ChannelDeleteMessagesParam,
    ) -> Result<pb::ChannelDeleteMessagesResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelDeleteMessages)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelLikeMessage(
        &self,
        param: pb::ChannelLikeMessageParam,
    ) -> Result<pb::ChannelLikeMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelLikeMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelUnLikeMessage(
        &self,
        param: pb::ChannelUnLikeMessageParam,
    ) -> Result<pb::ChannelUnLikeMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelUnLikeMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelReShareMessage(
        &self,
        param: pb::ChannelReShareMessageParam,
    ) -> Result<pb::ChannelReShareMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelReShareMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelUnReShareMessage(
        &self,
        param: pb::ChannelUnReShareMessageParam,
    ) -> Result<pb::ChannelUnReShareMessageResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelUnReShareMessage)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelAddComment(
        &self,
        param: pb::ChannelAddCommentParam,
    ) -> Result<pb::ChannelAddCommentResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelAddComment)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelDeleteComment(
        &self,
        param: pb::ChannelDeleteCommentParam,
    ) -> Result<pb::ChannelDeleteCommentResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelDeleteComment)
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

    pub async fn ChannelAvatarAdd(
        &self,
        param: pb::ChannelAvatarAddParam,
    ) -> Result<pb::ChannelAvatarAddResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelAvatarAdd)
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

    pub async fn ChannelAvatarGetList(
        &self,
        param: pb::ChannelAvatarGetListParam,
    ) -> Result<pb::ChannelAvatarGetListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChannelAvatarGetList)
            .await?;
        Ok(pb_res)
    }

    pub async fn ChannelGetInbox(
        &self,
        param: pb::ChannelGetInboxParam,
    ) -> Result<pb::ChannelGetInboxResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ChannelGetInbox).await?;
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

    // service: RPC_Chat
    pub async fn ChatDeleteChat(
        &self,
        param: pb::ChatDeleteChatParam,
    ) -> Result<pb::ChatDeleteChatResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ChatDeleteChat).await?;
        Ok(pb_res)
    }

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

    pub async fn ChatSetNotification(
        &self,
        param: pb::ChatSetNotificationParam,
    ) -> Result<pb::ChatSetNotificationResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ChatSetNotification)
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

    pub async fn ChatGetInbox(
        &self,
        param: pb::ChatGetInboxParam,
    ) -> Result<pb::ChatGetInboxResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ChatGetInbox).await?;
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

    pub async fn GroupBanMember(
        &self,
        param: pb::GroupBanMemberParam,
    ) -> Result<pb::GroupBanMemberResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupBanMember).await?;
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

    pub async fn GroupAddMember(
        &self,
        param: pb::GroupAddMemberParam,
    ) -> Result<pb::GroupAddMemberResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupAddMember).await?;
        Ok(pb_res)
    }

    pub async fn GroupSetNotification(
        &self,
        param: pb::GroupSetNotificationParam,
    ) -> Result<pb::GroupSetNotificationResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupSetNotification)
            .await?;
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

    pub async fn GroupAvatarAdd(
        &self,
        param: pb::GroupAvatarAddParam,
    ) -> Result<pb::GroupAvatarAddResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupAvatarAdd).await?;
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
        param: pb::GroupGetFullParam,
    ) -> Result<pb::GroupGetFullResponse, GenErr> {
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

    pub async fn GroupAvatarGetList(
        &self,
        param: pb::GroupAvatarGetListParam,
    ) -> Result<pb::GroupAvatarGetListResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::GroupAvatarGetList)
            .await?;
        Ok(pb_res)
    }

    pub async fn GroupGetInbox(
        &self,
        param: pb::GroupGetInboxParam,
    ) -> Result<pb::GroupGetInboxResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::GroupGetInbox).await?;
        Ok(pb_res)
    }

    // service: RPC_Profile
    pub async fn ProfileSetSettings(
        &self,
        param: pb::ProfileSetSettingsParam,
    ) -> Result<pb::ProfileSetSettingsResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ProfileSetSettings)
            .await?;
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
    pub async fn SharedEcho(
        &self,
        param: pb::SharedEchoParam,
    ) -> Result<pb::SharedEchoResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::SharedEcho).await?;
        Ok(pb_res)
    }

    pub async fn SharedCheckUserName(
        &self,
        param: pb::SharedCheckUserNameParam,
    ) -> Result<pb::SharedCheckUserNameResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::SharedCheckUserName)
            .await?;
        Ok(pb_res)
    }

    // service: RPC_Shop
    pub async fn ShopEEE(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ShopEEE).await?;
        Ok(pb_res)
    }

    pub async fn ShopCreateShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ShopCreateShop).await?;
        Ok(pb_res)
    }

    pub async fn ShopEditShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ShopEditShop).await?;
        Ok(pb_res)
    }

    pub async fn ShopPauseShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ShopPauseShop).await?;
        Ok(pb_res)
    }

    pub async fn ShopTerminateShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopTerminateShop)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopAddProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ShopAddProduct).await?;
        Ok(pb_res)
    }

    pub async fn ShopEditProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ShopEditProduct).await?;
        Ok(pb_res)
    }

    pub async fn ShopDeleteProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopDeleteProduct)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopAddProductToBasket(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopAddProductToBasket)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopRemoveProductFromBasket(
        &self,
        param: pb::Param,
    ) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopRemoveProductFromBasket)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopCheckoutBasket(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopCheckoutBasket)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopLikeProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ShopLikeProduct).await?;
        Ok(pb_res)
    }

    pub async fn ShopUnLikeProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopUnLikeProduct)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopLikeShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ShopLikeShop).await?;
        Ok(pb_res)
    }

    pub async fn ShopUnLikeShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ShopUnLikeShop).await?;
        Ok(pb_res)
    }

    pub async fn ShopGetFull(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ShopGetFull).await?;
        Ok(pb_res)
    }

    pub async fn ShopProductList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ShopProductList).await?;
        Ok(pb_res)
    }

    pub async fn ShopGetBasketList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopGetBasketList)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopGetLikedProductsList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopGetLikedProductsList)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopGetLikedShopsList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopGetLikedShopsList)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopGetOrderList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopGetOrderList)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopGetRefundList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopGetRefundList)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopCancelOrder(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::ShopCancelOrder).await?;
        Ok(pb_res)
    }

    pub async fn ShopEditProductInfo(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopEditProductInfo)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopEditProductPrice(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopEditProductPrice)
            .await?;
        Ok(pb_res)
    }

    pub async fn ShopEditProductInventory(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::ShopEditProductInventory)
            .await?;
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
    pub async fn UserRegisterUser(
        &self,
        param: pb::UserRegisterUserParam,
    ) -> Result<pb::UserRegisterUserResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::UserRegisterUser)
            .await?;
        Ok(pb_res)
    }

    pub async fn UserEditUser(
        &self,
        param: pb::UserEditUserParam,
    ) -> Result<pb::UserEditUserResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::UserEditUser).await?;
        Ok(pb_res)
    }

    pub async fn UserDeleteSendCode(
        &self,
        param: pb::UserDeleteSendCodeParam,
    ) -> Result<pb::UserDeleteSendCodeResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::UserDeleteSendCode)
            .await?;
        Ok(pb_res)
    }

    pub async fn UserDeleteConfirmCode(
        &self,
        param: pb::UserDeleteConfirmCodeParam,
    ) -> Result<pb::UserDeleteConfirmCodeResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::UserDeleteConfirmCode)
            .await?;
        Ok(pb_res)
    }

    pub async fn UserDeleteUser(
        &self,
        param: pb::UserDeleteUserParam,
    ) -> Result<pb::UserDeleteUserResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::UserDeleteUser).await?;
        Ok(pb_res)
    }

    pub async fn UserChangePhoneNumber(
        &self,
        param: pb::UserChangePhoneNumberParam,
    ) -> Result<pb::UserChangePhoneNumberResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::UserChangePhoneNumber)
            .await?;
        Ok(pb_res)
    }

    pub async fn UserRemoveSession(
        &self,
        param: pb::UserRemoveSessionParam,
    ) -> Result<pb::UserRemoveSessionResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::UserRemoveSession)
            .await?;
        Ok(pb_res)
    }

    pub async fn UserRemoveOtherSessions(
        &self,
        param: pb::UserRemoveOtherParam,
    ) -> Result<pb::UserRemoveOtherResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::UserRemoveOtherSessions)
            .await?;
        Ok(pb_res)
    }

    pub async fn UserGetMe(
        &self,
        param: pb::UserGetMeParam,
    ) -> Result<pb::UserGetMeResponse, GenErr> {
        let pb_res = self.rpc_invoke(&param, method_ids::UserGetMe).await?;
        Ok(pb_res)
    }

    pub async fn UserGetActiveSessions(
        &self,
        param: pb::UserGetActiveSessionsParam,
    ) -> Result<pb::UserGetActiveSessionsResponse, GenErr> {
        let pb_res = self
            .rpc_invoke(&param, method_ids::UserGetActiveSessions)
            .await?;
        Ok(pb_res)
    }
}

/////////////////////// Sample codes for act modules ///////
/*
#[rustfmt::skip]
mod IPC_CMasterAct{
    use crate::{act,pb,errors::GenErr};
    pub async fn get_next_id (ret: &act::Ret, param: pb::GetNextIdParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

}
#[rustfmt::skip]
mod AuthAct{
    use crate::{act,pb,errors::GenErr};
    pub async fn auth_send_code (ret: &act::Ret, param: pb::AuthSendCodeParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn auth_log_in (ret: &act::Ret, param: pb::AuthLogInParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn auth_log_out (ret: &act::Ret, param: pb::AuthLogOutParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

}
#[rustfmt::skip]
mod ChannelAct{
    use crate::{act,pb,errors::GenErr};
    pub async fn channel_create_channel (ret: &act::Ret, param: pb::ChannelCreateChannelParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_edit_channel (ret: &act::Ret, param: pb::ChannelEditChannelParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_delete_channel (ret: &act::Ret, param: pb::ChannelDeleteChannelParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_add_author (ret: &act::Ret, param: pb::ChannelAddAuthorParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_change_author_permission (ret: &act::Ret, param: pb::ChannelChangeAuthorPermissionParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_remove_author (ret: &act::Ret, param: pb::ChannelRemoveAuthorParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_follow_channel (ret: &act::Ret, param: pb::ChannelFollowChannelParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_un_follow_channel (ret: &act::Ret, param: pb::ChannelUnFollowChannelParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_remove_followers (ret: &act::Ret, param: pb::ChannelRemoveFollowersParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_subscribe (ret: &act::Ret, param: pb::ChannelSubscribeParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_un_subscribe (ret: &act::Ret, param: pb::ChannelUnSubscribeParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_remove_subscribers (ret: &act::Ret, param: pb::ChannelRemoveSubscribersParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_change_privacy (ret: &act::Ret, param: pb::ChannelChangePrivacyParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_change_default_permission (ret: &act::Ret, param: pb::ChannelChangeDefaultPermissionParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_revoke_link (ret: &act::Ret, param: pb::ChannelRevokeLinkParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_change_username (ret: &act::Ret, param: pb::ChannelChangeUsernameParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_block_profile (ret: &act::Ret, param: pb::ChannelBlockProfileParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_send_message (ret: &act::Ret, param: pb::ChannelSendMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_edit_message (ret: &act::Ret, param: pb::ChannelEditMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_delete_messages (ret: &act::Ret, param: pb::ChannelDeleteMessagesParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_like_message (ret: &act::Ret, param: pb::ChannelLikeMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_un_like_message (ret: &act::Ret, param: pb::ChannelUnLikeMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_re_share_message (ret: &act::Ret, param: pb::ChannelReShareMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_un_re_share_message (ret: &act::Ret, param: pb::ChannelUnReShareMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_add_comment (ret: &act::Ret, param: pb::ChannelAddCommentParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_delete_comment (ret: &act::Ret, param: pb::ChannelDeleteCommentParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_pin_message (ret: &act::Ret, param: pb::ChannelPinMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_un_pin_message (ret: &act::Ret, param: pb::ChannelUnPinMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_avatar_add (ret: &act::Ret, param: pb::ChannelAvatarAddParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_avatar_delete (ret: &act::Ret, param: pb::ChannelAvatarDeleteParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_send_doing_action (ret: &act::Ret, param: pb::ChannelSendDoingActionParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_report_channel (ret: &act::Ret, param: pb::ChannelReportChannelParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_report_message (ret: &act::Ret, param: pb::ChannelReportMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_get_full (ret: &act::Ret, param: pb::ChannelGetFullParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_get_messages_list (ret: &act::Ret, param: pb::ChannelGetMessagesListParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_get_media_list (ret: &act::Ret, param: pb::ChannelGetMediaListParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_get_authors (ret: &act::Ret, param: pb::ChannelGetAuthorsParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_get_followers (ret: &act::Ret, param: pb::ChannelGetFollowersParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_get_subscribers (ret: &act::Ret, param: pb::ChannelGetSubscribersParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_blocked (ret: &act::Ret, param: pb::ChannelBlockedParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_avatar_get_list (ret: &act::Ret, param: pb::ChannelAvatarGetListParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_get_inbox (ret: &act::Ret, param: pb::ChannelGetInboxParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn channel_get_followings (ret: &act::Ret, param: pb::ChannelGetFollowingsParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

}
#[rustfmt::skip]
mod ChatAct{
    use crate::{act,pb,errors::GenErr};
    pub async fn chat_delete_chat (ret: &act::Ret, param: pb::ChatDeleteChatParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn chat_send_message (ret: &act::Ret, param: pb::ChatSendMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn chat_edit_message (ret: &act::Ret, param: pb::ChatEditMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn chat_delete_messages (ret: &act::Ret, param: pb::ChatDeleteMessagesParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn chat_delete_history (ret: &act::Ret, param: pb::ChatDeleteHistoryParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn chat_set_notification (ret: &act::Ret, param: pb::ChatSetNotificationParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn chat_send_doing_action (ret: &act::Ret, param: pb::ChatSendDoingActionParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn chat_report_chat (ret: &act::Ret, param: pb::ChatReportChatParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn chat_get_full (ret: &act::Ret, param: pb::ChatGetFullMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn chat_get_messages_list (ret: &act::Ret, param: pb::ChatGetMessagesListParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn chat_get_media_list (ret: &act::Ret, param: pb::ChatGetMediaListParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn chat_get_inbox (ret: &act::Ret, param: pb::ChatGetInboxParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

}
#[rustfmt::skip]
mod GroupAct{
    use crate::{act,pb,errors::GenErr};
    pub async fn group_create_group (ret: &act::Ret, param: pb::GroupCreateGroupParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_edit_group (ret: &act::Ret, param: pb::GroupEditGroupParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_delete_group (ret: &act::Ret, param: pb::GroupDeleteGroupParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_add_admin (ret: &act::Ret, param: pb::GroupAddAdminParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_remove_member (ret: &act::Ret, param: pb::GroupRemoveMemberParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_change_member_level (ret: &act::Ret, param: pb::GroupChangeMemberLevelParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_change_member_permission (ret: &act::Ret, param: pb::GroupChangeMemberPermissionParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_ban_member (ret: &act::Ret, param: pb::GroupBanMemberParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_join_group (ret: &act::Ret, param: pb::JoinGroupParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_leave_group (ret: &act::Ret, param: pb::GroupLeaveGroupParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_add_member (ret: &act::Ret, param: pb::GroupAddMemberParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_set_notification (ret: &act::Ret, param: pb::GroupSetNotificationParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_change_privacy (ret: &act::Ret, param: pb::GroupChangePrivacyParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_change_default_permission (ret: &act::Ret, param: pb::GroupChangeDefaultPermissionParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_revoke_link (ret: &act::Ret, param: pb::GroupRevokeLinkParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_change_username (ret: &act::Ret, param: pb::GroupChangeUsernameParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_send_message (ret: &act::Ret, param: pb::GroupSendMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_edit_message (ret: &act::Ret, param: pb::GroupEditMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_delete_messages (ret: &act::Ret, param: pb::GroupDeleteMessagesParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_delete_history (ret: &act::Ret, param: pb::GroupDeleteHistoryParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_pin_message (ret: &act::Ret, param: pb::GroupPinMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_un_pin_message (ret: &act::Ret, param: pb::GroupUnPinMessageParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_avatar_add (ret: &act::Ret, param: pb::GroupAvatarAddParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_avatar_delete (ret: &act::Ret, param: pb::GroupAvatarDeleteParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_send_doing_action (ret: &act::Ret, param: pb::GroupSendDoingActionParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_report_group (ret: &act::Ret, param: pb::GroupReportGroupParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_get_full (ret: &act::Ret, param: pb::GroupGetFullParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_get_messages_list (ret: &act::Ret, param: pb::GroupGetMessagesListParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_get_media_list (ret: &act::Ret, param: pb::GroupGetMediaListParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_get_members_list (ret: &act::Ret, param: pb::GroupGetMembersListParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_get_admins_list (ret: &act::Ret, param: pb::GroupGetAdminsListParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_avatar_get_list (ret: &act::Ret, param: pb::GroupAvatarGetListParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn group_get_inbox (ret: &act::Ret, param: pb::GroupGetInboxParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

}
#[rustfmt::skip]
mod ProfileAct{
    use crate::{act,pb,errors::GenErr};
    pub async fn profile_set_settings (ret: &act::Ret, param: pb::ProfileSetSettingsParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

}
#[rustfmt::skip]
mod SampleAct{
    use crate::{act,pb,errors::GenErr};
    pub async fn get_users1 (ret: &act::Ret, param: pb::GetUsers1Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn get_profiles (ret: &act::Ret, param: pb::GetProfilesParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn get_channels (ret: &act::Ret, param: pb::GetChannelsParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn get_directs (ret: &act::Ret, param: pb::GetDirectsParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn get_messages (ret: &act::Ret, param: pb::GetMessagesParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

}
#[rustfmt::skip]
mod SharedAct{
    use crate::{act,pb,errors::GenErr};
    pub async fn shared_echo (ret: &act::Ret, param: pb::SharedEchoParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shared_check_user_name (ret: &act::Ret, param: pb::SharedCheckUserNameParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

}
#[rustfmt::skip]
mod ShopAct{
    use crate::{act,pb,errors::GenErr};
    pub async fn shop_eee (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_create_shop (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_edit_shop (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_pause_shop (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_terminate_shop (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_add_product (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_edit_product (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_delete_product (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_add_product_to_basket (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_remove_product_from_basket (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_checkout_basket (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_like_product (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_un_like_product (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_like_shop (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_un_like_shop (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_get_full (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_product_list (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_get_basket_list (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_get_liked_products_list (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_get_liked_shops_list (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_get_order_list (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_get_refund_list (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_cancel_order (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_edit_product_info (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_edit_product_price (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn shop_edit_product_inventory (ret: &act::Ret, param: pb::Param) -> Result<(),GenErr> {
        unimplemented!("");
    }

}
#[rustfmt::skip]
mod UploadAct{
    use crate::{act,pb,errors::GenErr};
    pub async fn upload_file (ret: &act::Ret, param: pb::UploadFileParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

}
#[rustfmt::skip]
mod UserAct{
    use crate::{act,pb,errors::GenErr};
    pub async fn user_register_user (ret: &act::Ret, param: pb::UserRegisterUserParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn user_edit_user (ret: &act::Ret, param: pb::UserEditUserParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn user_delete_send_code (ret: &act::Ret, param: pb::UserDeleteSendCodeParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn user_delete_confirm_code (ret: &act::Ret, param: pb::UserDeleteConfirmCodeParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn user_delete_user (ret: &act::Ret, param: pb::UserDeleteUserParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn user_change_phone_number (ret: &act::Ret, param: pb::UserChangePhoneNumberParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn user_remove_session (ret: &act::Ret, param: pb::UserRemoveSessionParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn user_remove_other_sessions (ret: &act::Ret, param: pb::UserRemoveOtherParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn user_get_me (ret: &act::Ret, param: pb::UserGetMeParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

    pub async fn user_get_active_sessions (ret: &act::Ret, param: pb::UserGetActiveSessionsParam) -> Result<(),GenErr> {
        unimplemented!("");
    }

}
*/

/////////////////////// Code gen for def rpc //////////////
struct _RRR_ {}

#[async_trait]
impl IPC_CMaster_Handler2 for _RRR_ {
    async fn GetNextId(&self, param: pb::GetNextIdParam) -> Result<pb::GetNextIdResponse, GenErr> {
        println!("called GetNextId in the impl code.");
        Ok(pb::GetNextIdResponse::default())
    }
}
#[async_trait]
impl RPC_Auth_Handler2 for _RRR_ {
    async fn AuthSendCode(
        &self,
        param: pb::AuthSendCodeParam,
    ) -> Result<pb::AuthSendCodeResponse, GenErr> {
        println!("called AuthSendCode in the impl code.");
        Ok(pb::AuthSendCodeResponse::default())
    }
    async fn AuthLogIn(&self, param: pb::AuthLogInParam) -> Result<pb::AuthLogInResponse, GenErr> {
        println!("called AuthLogIn in the impl code.");
        Ok(pb::AuthLogInResponse::default())
    }
    async fn AuthLogOut(
        &self,
        param: pb::AuthLogOutParam,
    ) -> Result<pb::AuthLogOutResponse, GenErr> {
        println!("called AuthLogOut in the impl code.");
        Ok(pb::AuthLogOutResponse::default())
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
    async fn ChannelBlockProfile(
        &self,
        param: pb::ChannelBlockProfileParam,
    ) -> Result<pb::ChannelBlockProfileResponse, GenErr> {
        println!("called ChannelBlockProfile in the impl code.");
        Ok(pb::ChannelBlockProfileResponse::default())
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
    async fn ChannelDeleteMessages(
        &self,
        param: pb::ChannelDeleteMessagesParam,
    ) -> Result<pb::ChannelDeleteMessagesResponse, GenErr> {
        println!("called ChannelDeleteMessages in the impl code.");
        Ok(pb::ChannelDeleteMessagesResponse::default())
    }
    async fn ChannelLikeMessage(
        &self,
        param: pb::ChannelLikeMessageParam,
    ) -> Result<pb::ChannelLikeMessageResponse, GenErr> {
        println!("called ChannelLikeMessage in the impl code.");
        Ok(pb::ChannelLikeMessageResponse::default())
    }
    async fn ChannelUnLikeMessage(
        &self,
        param: pb::ChannelUnLikeMessageParam,
    ) -> Result<pb::ChannelUnLikeMessageResponse, GenErr> {
        println!("called ChannelUnLikeMessage in the impl code.");
        Ok(pb::ChannelUnLikeMessageResponse::default())
    }
    async fn ChannelReShareMessage(
        &self,
        param: pb::ChannelReShareMessageParam,
    ) -> Result<pb::ChannelReShareMessageResponse, GenErr> {
        println!("called ChannelReShareMessage in the impl code.");
        Ok(pb::ChannelReShareMessageResponse::default())
    }
    async fn ChannelUnReShareMessage(
        &self,
        param: pb::ChannelUnReShareMessageParam,
    ) -> Result<pb::ChannelUnReShareMessageResponse, GenErr> {
        println!("called ChannelUnReShareMessage in the impl code.");
        Ok(pb::ChannelUnReShareMessageResponse::default())
    }
    async fn ChannelAddComment(
        &self,
        param: pb::ChannelAddCommentParam,
    ) -> Result<pb::ChannelAddCommentResponse, GenErr> {
        println!("called ChannelAddComment in the impl code.");
        Ok(pb::ChannelAddCommentResponse::default())
    }
    async fn ChannelDeleteComment(
        &self,
        param: pb::ChannelDeleteCommentParam,
    ) -> Result<pb::ChannelDeleteCommentResponse, GenErr> {
        println!("called ChannelDeleteComment in the impl code.");
        Ok(pb::ChannelDeleteCommentResponse::default())
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
    async fn ChannelAvatarAdd(
        &self,
        param: pb::ChannelAvatarAddParam,
    ) -> Result<pb::ChannelAvatarAddResponse, GenErr> {
        println!("called ChannelAvatarAdd in the impl code.");
        Ok(pb::ChannelAvatarAddResponse::default())
    }
    async fn ChannelAvatarDelete(
        &self,
        param: pb::ChannelAvatarDeleteParam,
    ) -> Result<pb::ChannelAvatarDeleteResponse, GenErr> {
        println!("called ChannelAvatarDelete in the impl code.");
        Ok(pb::ChannelAvatarDeleteResponse::default())
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
    async fn ChannelAvatarGetList(
        &self,
        param: pb::ChannelAvatarGetListParam,
    ) -> Result<pb::ChannelAvatarGetListResponse, GenErr> {
        println!("called ChannelAvatarGetList in the impl code.");
        Ok(pb::ChannelAvatarGetListResponse::default())
    }
    async fn ChannelGetInbox(
        &self,
        param: pb::ChannelGetInboxParam,
    ) -> Result<pb::ChannelGetInboxResponse, GenErr> {
        println!("called ChannelGetInbox in the impl code.");
        Ok(pb::ChannelGetInboxResponse::default())
    }
    async fn ChannelGetFollowings(
        &self,
        param: pb::ChannelGetFollowingsParam,
    ) -> Result<pb::ChannelGetFollowingsResponse, GenErr> {
        println!("called ChannelGetFollowings in the impl code.");
        Ok(pb::ChannelGetFollowingsResponse::default())
    }
}
#[async_trait]
impl RPC_Chat_Handler2 for _RRR_ {
    async fn ChatDeleteChat(
        &self,
        param: pb::ChatDeleteChatParam,
    ) -> Result<pb::ChatDeleteChatResponse, GenErr> {
        println!("called ChatDeleteChat in the impl code.");
        Ok(pb::ChatDeleteChatResponse::default())
    }
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
    async fn ChatSetNotification(
        &self,
        param: pb::ChatSetNotificationParam,
    ) -> Result<pb::ChatSetNotificationResponse, GenErr> {
        println!("called ChatSetNotification in the impl code.");
        Ok(pb::ChatSetNotificationResponse::default())
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
    async fn ChatGetInbox(
        &self,
        param: pb::ChatGetInboxParam,
    ) -> Result<pb::ChatGetInboxResponse, GenErr> {
        println!("called ChatGetInbox in the impl code.");
        Ok(pb::ChatGetInboxResponse::default())
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
    async fn GroupBanMember(
        &self,
        param: pb::GroupBanMemberParam,
    ) -> Result<pb::GroupBanMemberResponse, GenErr> {
        println!("called GroupBanMember in the impl code.");
        Ok(pb::GroupBanMemberResponse::default())
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
    async fn GroupAddMember(
        &self,
        param: pb::GroupAddMemberParam,
    ) -> Result<pb::GroupAddMemberResponse, GenErr> {
        println!("called GroupAddMember in the impl code.");
        Ok(pb::GroupAddMemberResponse::default())
    }
    async fn GroupSetNotification(
        &self,
        param: pb::GroupSetNotificationParam,
    ) -> Result<pb::GroupSetNotificationResponse, GenErr> {
        println!("called GroupSetNotification in the impl code.");
        Ok(pb::GroupSetNotificationResponse::default())
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
    async fn GroupAvatarAdd(
        &self,
        param: pb::GroupAvatarAddParam,
    ) -> Result<pb::GroupAvatarAddResponse, GenErr> {
        println!("called GroupAvatarAdd in the impl code.");
        Ok(pb::GroupAvatarAddResponse::default())
    }
    async fn GroupAvatarDelete(
        &self,
        param: pb::GroupAvatarDeleteParam,
    ) -> Result<pb::GroupAvatarDeleteResponse, GenErr> {
        println!("called GroupAvatarDelete in the impl code.");
        Ok(pb::GroupAvatarDeleteResponse::default())
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
        param: pb::GroupGetFullParam,
    ) -> Result<pb::GroupGetFullResponse, GenErr> {
        println!("called GroupGetFull in the impl code.");
        Ok(pb::GroupGetFullResponse::default())
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
    async fn GroupAvatarGetList(
        &self,
        param: pb::GroupAvatarGetListParam,
    ) -> Result<pb::GroupAvatarGetListResponse, GenErr> {
        println!("called GroupAvatarGetList in the impl code.");
        Ok(pb::GroupAvatarGetListResponse::default())
    }
    async fn GroupGetInbox(
        &self,
        param: pb::GroupGetInboxParam,
    ) -> Result<pb::GroupGetInboxResponse, GenErr> {
        println!("called GroupGetInbox in the impl code.");
        Ok(pb::GroupGetInboxResponse::default())
    }
}
#[async_trait]
impl RPC_Profile_Handler2 for _RRR_ {
    async fn ProfileSetSettings(
        &self,
        param: pb::ProfileSetSettingsParam,
    ) -> Result<pb::ProfileSetSettingsResponse, GenErr> {
        println!("called ProfileSetSettings in the impl code.");
        Ok(pb::ProfileSetSettingsResponse::default())
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
    async fn SharedEcho(
        &self,
        param: pb::SharedEchoParam,
    ) -> Result<pb::SharedEchoResponse, GenErr> {
        println!("called SharedEcho in the impl code.");
        Ok(pb::SharedEchoResponse::default())
    }
    async fn SharedCheckUserName(
        &self,
        param: pb::SharedCheckUserNameParam,
    ) -> Result<pb::SharedCheckUserNameResponse, GenErr> {
        println!("called SharedCheckUserName in the impl code.");
        Ok(pb::SharedCheckUserNameResponse::default())
    }
}
#[async_trait]
impl RPC_Shop_Handler2 for _RRR_ {
    async fn ShopEEE(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopEEE in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopCreateShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopCreateShop in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopEditShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopEditShop in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopPauseShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopPauseShop in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopTerminateShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopTerminateShop in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopAddProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopAddProduct in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopEditProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopEditProduct in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopDeleteProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopDeleteProduct in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopAddProductToBasket(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopAddProductToBasket in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopRemoveProductFromBasket(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopRemoveProductFromBasket in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopCheckoutBasket(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopCheckoutBasket in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopLikeProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopLikeProduct in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopUnLikeProduct(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopUnLikeProduct in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopLikeShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopLikeShop in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopUnLikeShop(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopUnLikeShop in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopGetFull(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopGetFull in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopProductList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopProductList in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopGetBasketList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopGetBasketList in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopGetLikedProductsList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopGetLikedProductsList in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopGetLikedShopsList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopGetLikedShopsList in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopGetOrderList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopGetOrderList in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopGetRefundList(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopGetRefundList in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopCancelOrder(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopCancelOrder in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopEditProductInfo(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopEditProductInfo in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopEditProductPrice(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopEditProductPrice in the impl code.");
        Ok(pb::Response::default())
    }
    async fn ShopEditProductInventory(&self, param: pb::Param) -> Result<pb::Response, GenErr> {
        println!("called ShopEditProductInventory in the impl code.");
        Ok(pb::Response::default())
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
    async fn UserRegisterUser(
        &self,
        param: pb::UserRegisterUserParam,
    ) -> Result<pb::UserRegisterUserResponse, GenErr> {
        println!("called UserRegisterUser in the impl code.");
        Ok(pb::UserRegisterUserResponse::default())
    }
    async fn UserEditUser(
        &self,
        param: pb::UserEditUserParam,
    ) -> Result<pb::UserEditUserResponse, GenErr> {
        println!("called UserEditUser in the impl code.");
        Ok(pb::UserEditUserResponse::default())
    }
    async fn UserDeleteSendCode(
        &self,
        param: pb::UserDeleteSendCodeParam,
    ) -> Result<pb::UserDeleteSendCodeResponse, GenErr> {
        println!("called UserDeleteSendCode in the impl code.");
        Ok(pb::UserDeleteSendCodeResponse::default())
    }
    async fn UserDeleteConfirmCode(
        &self,
        param: pb::UserDeleteConfirmCodeParam,
    ) -> Result<pb::UserDeleteConfirmCodeResponse, GenErr> {
        println!("called UserDeleteConfirmCode in the impl code.");
        Ok(pb::UserDeleteConfirmCodeResponse::default())
    }
    async fn UserDeleteUser(
        &self,
        param: pb::UserDeleteUserParam,
    ) -> Result<pb::UserDeleteUserResponse, GenErr> {
        println!("called UserDeleteUser in the impl code.");
        Ok(pb::UserDeleteUserResponse::default())
    }
    async fn UserChangePhoneNumber(
        &self,
        param: pb::UserChangePhoneNumberParam,
    ) -> Result<pb::UserChangePhoneNumberResponse, GenErr> {
        println!("called UserChangePhoneNumber in the impl code.");
        Ok(pb::UserChangePhoneNumberResponse::default())
    }
    async fn UserRemoveSession(
        &self,
        param: pb::UserRemoveSessionParam,
    ) -> Result<pb::UserRemoveSessionResponse, GenErr> {
        println!("called UserRemoveSession in the impl code.");
        Ok(pb::UserRemoveSessionResponse::default())
    }
    async fn UserRemoveOtherSessions(
        &self,
        param: pb::UserRemoveOtherParam,
    ) -> Result<pb::UserRemoveOtherResponse, GenErr> {
        println!("called UserRemoveOtherSessions in the impl code.");
        Ok(pb::UserRemoveOtherResponse::default())
    }
    async fn UserGetMe(&self, param: pb::UserGetMeParam) -> Result<pb::UserGetMeResponse, GenErr> {
        println!("called UserGetMe in the impl code.");
        Ok(pb::UserGetMeResponse::default())
    }
    async fn UserGetActiveSessions(
        &self,
        param: pb::UserGetActiveSessionsParam,
    ) -> Result<pb::UserGetActiveSessionsResponse, GenErr> {
        println!("called UserGetActiveSessions in the impl code.");
        Ok(pb::UserGetActiveSessionsResponse::default())
    }
}
