///
//  Generated code. Do not modify.
//  source: views.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

const ServerErrors$json = const {
  '1': 'ServerErrors',
  '2': const [
    const {'1': 'UNKNOWN_ERR', '2': 0},
    const {'1': 'ERR_CLIENT_IS_DEPRECATED', '2': 1},
    const {'1': 'ERR_SERVER_UPGRADING', '2': 2},
  ],
};

const MediaView$json = const {
  '1': 'MediaView',
};

const ActionView$json = const {
  '1': 'ActionView',
  '2': const [
    const {'1': 'ActionId', '3': 1, '4': 1, '5': 3, '10': 'ActionId'},
    const {'1': 'ActorUserId', '3': 2, '4': 1, '5': 5, '10': 'ActorUserId'},
    const {'1': 'ActionTypeEnum', '3': 3, '4': 1, '5': 5, '10': 'ActionTypeEnum'},
    const {'1': 'PeerUserId', '3': 4, '4': 1, '5': 5, '10': 'PeerUserId'},
    const {'1': 'PostId', '3': 5, '4': 1, '5': 3, '10': 'PostId'},
    const {'1': 'CommentId', '3': 6, '4': 1, '5': 3, '10': 'CommentId'},
    const {'1': 'Murmur64Hash', '3': 7, '4': 1, '5': 3, '10': 'Murmur64Hash'},
    const {'1': 'CreatedTime', '3': 8, '4': 1, '5': 5, '10': 'CreatedTime'},
    const {'1': 'ActorUserView', '3': 100, '4': 1, '5': 11, '6': '.UserView', '10': 'ActorUserView'},
    const {'1': 'PostView', '3': 101, '4': 1, '5': 11, '6': '.PostView', '10': 'PostView'},
    const {'1': 'CommentView', '3': 102, '4': 1, '5': 11, '6': '.CommentView', '10': 'CommentView'},
    const {'1': 'FollowedUserView', '3': 103, '4': 1, '5': 11, '6': '.UserView', '10': 'FollowedUserView'},
    const {'1': 'ContentOwenerUserView', '3': 104, '4': 1, '5': 11, '6': '.UserView', '10': 'ContentOwenerUserView'},
  ],
};

const NotifyView$json = const {
  '1': 'NotifyView',
  '2': const [
    const {'1': 'NotifyId', '3': 1, '4': 1, '5': 3, '10': 'NotifyId'},
    const {'1': 'ForUserId', '3': 2, '4': 1, '5': 5, '10': 'ForUserId'},
    const {'1': 'ActorUserId', '3': 3, '4': 1, '5': 5, '10': 'ActorUserId'},
    const {'1': 'NotiyTypeEnum', '3': 4, '4': 1, '5': 5, '10': 'NotiyTypeEnum'},
    const {'1': 'PostId', '3': 5, '4': 1, '5': 3, '10': 'PostId'},
    const {'1': 'CommentId', '3': 6, '4': 1, '5': 3, '10': 'CommentId'},
    const {'1': 'PeerUserId', '3': 7, '4': 1, '5': 5, '10': 'PeerUserId'},
    const {'1': 'Murmur64Hash', '3': 8, '4': 1, '5': 3, '10': 'Murmur64Hash'},
    const {'1': 'SeenStatus', '3': 9, '4': 1, '5': 5, '10': 'SeenStatus'},
    const {'1': 'CreatedTime', '3': 10, '4': 1, '5': 5, '10': 'CreatedTime'},
    const {'1': 'ActorUserView', '3': 100, '4': 1, '5': 11, '6': '.UserView', '10': 'ActorUserView'},
    const {'1': 'PostView', '3': 101, '4': 1, '5': 11, '6': '.PostView', '10': 'PostView'},
    const {'1': 'CommentView', '3': 102, '4': 1, '5': 11, '6': '.CommentView', '10': 'CommentView'},
  ],
};

const CommentView$json = const {
  '1': 'CommentView',
  '2': const [
    const {'1': 'CommentId', '3': 1, '4': 1, '5': 3, '10': 'CommentId'},
    const {'1': 'UserId', '3': 2, '4': 1, '5': 5, '10': 'UserId'},
    const {'1': 'PostId', '3': 3, '4': 1, '5': 3, '10': 'PostId'},
    const {'1': 'Text', '3': 4, '4': 1, '5': 9, '10': 'Text'},
    const {'1': 'LikesCount', '3': 5, '4': 1, '5': 5, '10': 'LikesCount'},
    const {'1': 'CreatedTime', '3': 6, '4': 1, '5': 5, '10': 'CreatedTime'},
    const {'1': 'SenderUserView', '3': 15, '4': 1, '5': 11, '6': '.UserView', '10': 'SenderUserView'},
  ],
};

const PostView$json = const {
  '1': 'PostView',
  '2': const [
    const {'1': 'PostId', '3': 1, '4': 1, '5': 3, '10': 'PostId'},
    const {'1': 'UserId', '3': 2, '4': 1, '5': 5, '10': 'UserId'},
    const {'1': 'Text', '3': 4, '4': 1, '5': 9, '10': 'Text'},
    const {'1': 'RichText', '3': 5, '4': 1, '5': 9, '10': 'RichText'},
    const {'1': 'MediaCount', '3': 6, '4': 1, '5': 5, '10': 'MediaCount'},
    const {'1': 'SharedTo', '3': 7, '4': 1, '5': 5, '10': 'SharedTo'},
    const {'1': 'DisableComment', '3': 8, '4': 1, '5': 5, '10': 'DisableComment'},
    const {'1': 'HasTag', '3': 9, '4': 1, '5': 5, '10': 'HasTag'},
    const {'1': 'CommentsCount', '3': 10, '4': 1, '5': 5, '10': 'CommentsCount'},
    const {'1': 'LikesCount', '3': 11, '4': 1, '5': 5, '10': 'LikesCount'},
    const {'1': 'ViewsCount', '3': 12, '4': 1, '5': 5, '10': 'ViewsCount'},
    const {'1': 'EditedTime', '3': 13, '4': 1, '5': 5, '10': 'EditedTime'},
    const {'1': 'CreatedTime', '3': 14, '4': 1, '5': 5, '10': 'CreatedTime'},
    const {'1': 'ReSharedPostId', '3': 15, '4': 1, '5': 3, '10': 'ReSharedPostId'},
    const {'1': 'DidILiked', '3': 50, '4': 1, '5': 8, '10': 'DidILiked'},
    const {'1': 'DidIReShared', '3': 51, '4': 1, '5': 8, '10': 'DidIReShared'},
    const {'1': 'SenderUserView', '3': 100, '4': 1, '5': 11, '6': '.UserView', '10': 'SenderUserView'},
    const {'1': 'ReSharedUserView', '3': 101, '4': 1, '5': 11, '6': '.UserView', '10': 'ReSharedUserView'},
    const {'1': 'MediaView', '3': 102, '4': 1, '5': 11, '6': '.MediaView', '10': 'MediaView'},
    const {'1': 'MediaViewList', '3': 103, '4': 3, '5': 11, '6': '.MediaView', '10': 'MediaViewList'},
  ],
};

const ChatView$json = const {
  '1': 'ChatView',
  '2': const [
    const {'1': 'ChatId', '3': 1, '4': 1, '5': 3, '10': 'ChatId'},
    const {'1': 'ChatKey', '3': 2, '4': 1, '5': 9, '10': 'ChatKey'},
    const {'1': 'RoomKey', '3': 3, '4': 1, '5': 9, '10': 'RoomKey'},
    const {'1': 'RoomType', '3': 4, '4': 1, '5': 5, '10': 'RoomType'},
    const {'1': 'UserId', '3': 5, '4': 1, '5': 5, '10': 'UserId'},
    const {'1': 'PeerUserId', '3': 6, '4': 1, '5': 5, '10': 'PeerUserId'},
    const {'1': 'GroupId', '3': 7, '4': 1, '5': 3, '10': 'GroupId'},
    const {'1': 'HashTagId', '3': 8, '4': 1, '5': 3, '10': 'HashTagId'},
    const {'1': 'StartedByMe', '3': 9, '4': 1, '5': 5, '10': 'StartedByMe'},
    const {'1': 'Title', '3': 10, '4': 1, '5': 9, '10': 'Title'},
    const {'1': 'PinTime', '3': 11, '4': 1, '5': 3, '10': 'PinTime'},
    const {'1': 'FromMsgId', '3': 12, '4': 1, '5': 3, '10': 'FromMsgId'},
    const {'1': 'Seq', '3': 13, '4': 1, '5': 5, '10': 'Seq'},
    const {'1': 'LastMsgId', '3': 14, '4': 1, '5': 3, '10': 'LastMsgId'},
    const {'1': 'LastMsgStatus', '3': 15, '4': 1, '5': 5, '10': 'LastMsgStatus'},
    const {'1': 'SeenSeq', '3': 16, '4': 1, '5': 5, '10': 'SeenSeq'},
    const {'1': 'SeenMsgId', '3': 17, '4': 1, '5': 3, '10': 'SeenMsgId'},
    const {'1': 'Left', '3': 18, '4': 1, '5': 5, '10': 'Left'},
    const {'1': 'Creator', '3': 19, '4': 1, '5': 5, '10': 'Creator'},
    const {'1': 'Kicked', '3': 20, '4': 1, '5': 5, '10': 'Kicked'},
    const {'1': 'Admin', '3': 21, '4': 1, '5': 5, '10': 'Admin'},
    const {'1': 'Deactivated', '3': 22, '4': 1, '5': 5, '10': 'Deactivated'},
    const {'1': 'VersionTime', '3': 23, '4': 1, '5': 5, '10': 'VersionTime'},
    const {'1': 'SortTime', '3': 24, '4': 1, '5': 5, '10': 'SortTime'},
    const {'1': 'CreatedTime', '3': 25, '4': 1, '5': 5, '10': 'CreatedTime'},
    const {'1': 'DraftText', '3': 26, '4': 1, '5': 9, '10': 'DraftText'},
    const {'1': 'DratReplyToMsgId', '3': 27, '4': 1, '5': 3, '10': 'DratReplyToMsgId'},
    const {'1': 'IsMute', '3': 28, '4': 1, '5': 5, '10': 'IsMute'},
    const {'1': 'UserView', '3': 100, '4': 1, '5': 11, '6': '.UserView', '10': 'UserView'},
    const {'1': 'GroupView', '3': 101, '4': 1, '5': 11, '6': '.GroupView', '10': 'GroupView'},
    const {'1': 'FirstUnreadMessage', '3': 200, '4': 1, '5': 11, '6': '.MessageView', '10': 'FirstUnreadMessage'},
    const {'1': 'LastMessage', '3': 201, '4': 1, '5': 11, '6': '.MessageView', '10': 'LastMessage'},
  ],
};

const GroupView$json = const {
  '1': 'GroupView',
  '2': const [
    const {'1': 'GroupId', '3': 1, '4': 1, '5': 3, '10': 'GroupId'},
    const {'1': 'GroupKey', '3': 2, '4': 1, '5': 9, '10': 'GroupKey'},
    const {'1': 'GroupName', '3': 3, '4': 1, '5': 9, '10': 'GroupName'},
    const {'1': 'UserName', '3': 4, '4': 1, '5': 9, '10': 'UserName'},
    const {'1': 'IsSuperGroup', '3': 5, '4': 1, '5': 5, '10': 'IsSuperGroup'},
    const {'1': 'HashTagId', '3': 6, '4': 1, '5': 3, '10': 'HashTagId'},
    const {'1': 'CreatorUserId', '3': 7, '4': 1, '5': 5, '10': 'CreatorUserId'},
    const {'1': 'GroupPrivacy', '3': 8, '4': 1, '5': 5, '10': 'GroupPrivacy'},
    const {'1': 'HistoryViewAble', '3': 9, '4': 1, '5': 5, '10': 'HistoryViewAble'},
    const {'1': 'Seq', '3': 10, '4': 1, '5': 3, '10': 'Seq'},
    const {'1': 'LastMsgId', '3': 11, '4': 1, '5': 3, '10': 'LastMsgId'},
    const {'1': 'PinedMsgId', '3': 12, '4': 1, '5': 3, '10': 'PinedMsgId'},
    const {'1': 'AvatarRefId', '3': 13, '4': 1, '5': 3, '10': 'AvatarRefId'},
    const {'1': 'AvatarCount', '3': 14, '4': 1, '5': 5, '10': 'AvatarCount'},
    const {'1': 'About', '3': 15, '4': 1, '5': 9, '10': 'About'},
    const {'1': 'InviteLink', '3': 16, '4': 1, '5': 9, '10': 'InviteLink'},
    const {'1': 'MembersCount', '3': 17, '4': 1, '5': 5, '10': 'MembersCount'},
    const {'1': 'SortTime', '3': 18, '4': 1, '5': 5, '10': 'SortTime'},
    const {'1': 'CreatedTime', '3': 19, '4': 1, '5': 5, '10': 'CreatedTime'},
  ],
};

const GroupMemberView$json = const {
  '1': 'GroupMemberView',
  '2': const [
    const {'1': 'OrderId', '3': 1, '4': 1, '5': 3, '10': 'OrderId'},
    const {'1': 'GroupId', '3': 2, '4': 1, '5': 3, '10': 'GroupId'},
    const {'1': 'UserId', '3': 3, '4': 1, '5': 5, '10': 'UserId'},
    const {'1': 'ByUserId', '3': 4, '4': 1, '5': 5, '10': 'ByUserId'},
    const {'1': 'GroupRole', '3': 5, '4': 1, '5': 5, '10': 'GroupRole'},
    const {'1': 'CreatedTime', '3': 6, '4': 1, '5': 5, '10': 'CreatedTime'},
  ],
};

const MessageView$json = const {
  '1': 'MessageView',
  '2': const [
    const {'1': 'RoomKey', '3': 1, '4': 1, '5': 9, '10': 'RoomKey'},
    const {'1': 'MessageId', '3': 2, '4': 1, '5': 3, '10': 'MessageId'},
    const {'1': 'UserId', '3': 3, '4': 1, '5': 5, '10': 'UserId'},
    const {'1': 'FileRefId', '3': 4, '4': 1, '5': 3, '10': 'FileRefId'},
    const {'1': 'MessageType', '3': 5, '4': 1, '5': 5, '10': 'MessageType'},
    const {'1': 'Text', '3': 6, '4': 1, '5': 9, '10': 'Text'},
    const {'1': 'Hiden', '3': 7, '4': 1, '5': 5, '10': 'Hiden'},
    const {'1': 'Seq', '3': 8, '4': 1, '5': 5, '10': 'Seq'},
    const {'1': 'ForwardedMsgId', '3': 9, '4': 1, '5': 3, '10': 'ForwardedMsgId'},
    const {'1': 'PostId', '3': 10, '4': 1, '5': 3, '10': 'PostId'},
    const {'1': 'StickerId', '3': 11, '4': 1, '5': 3, '10': 'StickerId'},
    const {'1': 'CreatedTime', '3': 12, '4': 1, '5': 5, '10': 'CreatedTime'},
    const {'1': 'DeliveredTime', '3': 13, '4': 1, '5': 5, '10': 'DeliveredTime'},
    const {'1': 'SeenTime', '3': 14, '4': 1, '5': 5, '10': 'SeenTime'},
    const {'1': 'DeliviryStatus', '3': 15, '4': 1, '5': 5, '10': 'DeliviryStatus'},
    const {'1': 'ReplyToMessageId', '3': 16, '4': 1, '5': 3, '10': 'ReplyToMessageId'},
    const {'1': 'ViewsCount', '3': 17, '4': 1, '5': 3, '10': 'ViewsCount'},
    const {'1': 'EditTime', '3': 18, '4': 1, '5': 5, '10': 'EditTime'},
    const {'1': 'Ttl', '3': 19, '4': 1, '5': 5, '10': 'Ttl'},
    const {'1': 'FileRedView', '3': 50, '4': 1, '5': 11, '6': '.FileRedView', '10': 'FileRedView'},
  ],
};

const FileRedView$json = const {
  '1': 'FileRedView',
  '2': const [
    const {'1': 'FileRefId', '3': 1, '4': 1, '5': 3, '10': 'FileRefId'},
    const {'1': 'UserId', '3': 2, '4': 1, '5': 3, '10': 'UserId'},
    const {'1': 'Name', '3': 3, '4': 1, '5': 9, '10': 'Name'},
    const {'1': 'Width', '3': 4, '4': 1, '5': 5, '10': 'Width'},
    const {'1': 'Height', '3': 5, '4': 1, '5': 5, '10': 'Height'},
    const {'1': 'Duration', '3': 6, '4': 1, '5': 5, '10': 'Duration'},
    const {'1': 'Extension', '3': 7, '4': 1, '5': 9, '10': 'Extension'},
    const {'1': 'UrlSource', '3': 8, '4': 1, '5': 9, '10': 'UrlSource'},
  ],
};

const UserView$json = const {
  '1': 'UserView',
  '2': const [
    const {'1': 'UserId', '3': 1, '4': 1, '5': 5, '10': 'UserId'},
    const {'1': 'UserName', '3': 2, '4': 1, '5': 9, '10': 'UserName'},
    const {'1': 'FirstName', '3': 4, '4': 1, '5': 9, '10': 'FirstName'},
    const {'1': 'LastName', '3': 5, '4': 1, '5': 9, '10': 'LastName'},
    const {'1': 'AvatarRefId', '3': 8, '4': 1, '5': 3, '10': 'AvatarRefId'},
    const {'1': 'ProfilePrivacy', '3': 9, '4': 1, '5': 5, '10': 'ProfilePrivacy'},
    const {'1': 'Phone', '3': 10, '4': 1, '5': 3, '10': 'Phone'},
    const {'1': 'About', '3': 11, '4': 1, '5': 9, '10': 'About'},
    const {'1': 'FollowersCount', '3': 100, '4': 1, '5': 5, '10': 'FollowersCount'},
    const {'1': 'FollowingCount', '3': 101, '4': 1, '5': 5, '10': 'FollowingCount'},
    const {'1': 'PostsCount', '3': 102, '4': 1, '5': 5, '10': 'PostsCount'},
    const {'1': 'MediaCount', '3': 103, '4': 1, '5': 5, '10': 'MediaCount'},
    const {'1': 'UserOnlineStatusEnum', '3': 200, '4': 1, '5': 14, '6': '.UserOnlineStatusEnum', '10': 'UserOnlineStatusEnum'},
    const {'1': 'LastActiveTime', '3': 201, '4': 1, '5': 5, '10': 'LastActiveTime'},
    const {'1': 'LastActiveTimeShow', '3': 202, '4': 1, '5': 9, '10': 'LastActiveTimeShow'},
    const {'1': 'MyFollwing', '3': 300, '4': 1, '5': 14, '6': '.FollowingEnum', '10': 'MyFollwing'},
  ],
};

const SettingNotificationView$json = const {
  '1': 'SettingNotificationView',
};

const AppConfig$json = const {
  '1': 'AppConfig',
  '2': const [
    const {'1': 'DeprecatedClient', '3': 1, '4': 1, '5': 8, '10': 'DeprecatedClient'},
    const {'1': 'HasNewUpdate', '3': 2, '4': 1, '5': 8, '10': 'HasNewUpdate'},
  ],
};

const UserProfileView$json = const {
  '1': 'UserProfileView',
};

const UserViewRowify$json = const {
  '1': 'UserViewRowify',
  '2': const [
    const {'1': 'Id', '3': 1, '4': 1, '5': 3, '10': 'Id'},
    const {'1': 'CreatedTime', '3': 2, '4': 1, '5': 5, '10': 'CreatedTime'},
    const {'1': 'UserView', '3': 10, '4': 1, '5': 11, '6': '.UserView', '10': 'UserView'},
  ],
};

const PostViewRowify$json = const {
  '1': 'PostViewRowify',
  '2': const [
    const {'1': 'Id', '3': 1, '4': 1, '5': 3, '10': 'Id'},
    const {'1': 'PostView', '3': 10, '4': 1, '5': 11, '6': '.PostView', '10': 'PostView'},
  ],
};

const TagView$json = const {
  '1': 'TagView',
  '2': const [
    const {'1': 'TagId', '3': 1, '4': 1, '5': 3, '10': 'TagId'},
    const {'1': 'Name', '3': 2, '4': 1, '5': 9, '10': 'Name'},
    const {'1': 'Count', '3': 3, '4': 1, '5': 5, '10': 'Count'},
    const {'1': 'TagStatusEnum', '3': 4, '4': 1, '5': 5, '10': 'TagStatusEnum'},
    const {'1': 'CreatedTime', '3': 5, '4': 1, '5': 5, '10': 'CreatedTime'},
  ],
};

const TopTagWithSamplePosts$json = const {
  '1': 'TopTagWithSamplePosts',
  '2': const [
    const {'1': 'TagView', '3': 1, '4': 1, '5': 11, '6': '.TagView', '10': 'TagView'},
    const {'1': 'PostViewList', '3': 2, '4': 3, '5': 11, '6': '.PostView', '10': 'PostViewList'},
  ],
};

const SelfUserView$json = const {
  '1': 'SelfUserView',
  '2': const [
    const {'1': 'UserView', '3': 1, '4': 1, '5': 11, '6': '.UserView', '10': 'UserView'},
    const {'1': 'ProfilePrivacy', '3': 30, '4': 1, '5': 5, '10': 'ProfilePrivacy'},
    const {'1': 'OnlinePrivacy', '3': 32, '4': 1, '5': 5, '10': 'OnlinePrivacy'},
    const {'1': 'CallPrivacy', '3': 33, '4': 1, '5': 5, '10': 'CallPrivacy'},
    const {'1': 'AddToGroupPrivacy', '3': 34, '4': 1, '5': 5, '10': 'AddToGroupPrivacy'},
    const {'1': 'SeenMessagePrivacy', '3': 35, '4': 1, '5': 5, '10': 'SeenMessagePrivacy'},
    const {'1': 'SettingNotification', '3': 100, '4': 1, '5': 11, '6': '.SettingNotificationView', '10': 'SettingNotification'},
  ],
};

const Error$json = const {
  '1': 'Error',
  '2': const [
    const {'1': 'Error', '3': 1, '4': 1, '5': 14, '6': '.ServerErrors', '10': 'Error'},
    const {'1': 'ShowError', '3': 2, '4': 1, '5': 8, '10': 'ShowError'},
    const {'1': 'ErrorMessage', '3': 3, '4': 1, '5': 9, '10': 'ErrorMessage'},
  ],
};

