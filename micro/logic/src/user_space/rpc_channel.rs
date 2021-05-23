use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::utils::time;
use shared::{pb, rpc2, utils};

use crate::UserSpace;

#[async_trait]
impl rpc2::RPC_Channel_Handler2 for UserSpace {
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
