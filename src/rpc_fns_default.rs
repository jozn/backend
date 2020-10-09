use crate::{com, com::*, pb};


// Service: RPC_Account
pub mod RPC_Account {
    use super::*;
    
    pub fn ChangePhoneNumber(up: &UserParam, param: pb::ChangePhoneNumberParam) -> Result<pb::ChangePhoneNumberResponse, GenErr> {
        Ok(pb::ChangePhoneNumberResponse::default())
    }
}

// Service: RPC_Auth
pub mod RPC_Auth {
    use super::*;
    
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
}

// Service: RPC_Channel
pub mod RPC_Channel {
    use super::*;
    
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
}

// Service: RPC_Chat
pub mod RPC_Chat {
    use super::*;
    
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
}

// Service: RPC_Direct
pub mod RPC_Direct {
    use super::*;
    
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
}

// Service: RPC_General
pub mod RPC_General {
    use super::*;
    
    pub fn Echo(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
    pub fn CheckUserName(up: &UserParam, param: pb::CheckUserNameParam) -> Result<pb::CheckUserNameResponse, GenErr> {
        Ok(pb::CheckUserNameResponse::default())
    }
}

// Service: RPC_Group
pub mod RPC_Group {
    use super::*;
    
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
}

// Service: RPC_Social
pub mod RPC_Social {
    use super::*;
    
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
}

// Service: RPC_Upload
pub mod RPC_Upload {
    use super::*;
    
    pub fn UploadFile(up: &UserParam, param: pb::UploadFileParam) -> Result<pb::UploadFileResponse, GenErr> {
        Ok(pb::UploadFileResponse::default())
    }
}


/*
pub use def::RPC_Account::*;
pub use def::RPC_Auth::*;
pub use def::RPC_Channel::*;
pub use def::RPC_Chat::*;
pub use def::RPC_Direct::*;
pub use def::RPC_General::*;
pub use def::RPC_Group::*;
pub use def::RPC_Social::*;
pub use def::RPC_Upload::*;

 */
