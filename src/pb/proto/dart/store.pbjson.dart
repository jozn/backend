///
//  Generated code. Do not modify.
//  source: store.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

const MessageType$json = const {
  '1': 'MessageType',
  '2': const [
    const {'1': 'TEXT', '2': 0},
    const {'1': 'IMAGE', '2': 1},
    const {'1': 'VIDEO', '2': 3},
    const {'1': 'AUDIO', '2': 5},
    const {'1': 'VOICE', '2': 7},
    const {'1': 'GIF', '2': 8},
    const {'1': 'FILE', '2': 9},
    const {'1': 'POLL', '2': 10},
    const {'1': 'LOCATION', '2': 11},
    const {'1': 'LOG', '2': 12},
    const {'1': 'CONTACT', '2': 13},
    const {'1': 'WALLET', '2': 15},
    const {'1': 'PRODUCT', '2': 16},
    const {'1': 'FORWARD', '2': 17},
    const {'1': 'POST_MEDIA', '2': 100},
  ],
};

const MessageDeliveryStatues$json = const {
  '1': 'MessageDeliveryStatues',
  '2': const [
    const {'1': 'UNKNOWN_MD', '2': 0},
    const {'1': 'NOT_ABLE', '2': 6},
    const {'1': 'FAILED', '2': 7},
    const {'1': 'SENDING', '2': 1},
    const {'1': 'SENT', '2': 2},
    const {'1': 'DELIVERED', '2': 3},
    const {'1': 'SEEN', '2': 4},
    const {'1': 'CONSUMED', '2': 5},
  ],
};

const MessageLogType$json = const {
  '1': 'MessageLogType',
  '2': const [
    const {'1': 'USER_JOINED', '2': 0},
    const {'1': 'USER_DELETED', '2': 1},
    const {'1': 'ROOM_CREATED', '2': 2},
    const {'1': 'MEMBER_ADDED', '2': 3},
    const {'1': 'MEMBER_KICKED', '2': 4},
    const {'1': 'MEMBER_LEFT', '2': 5},
    const {'1': 'ROOM_CONVERTED_TO_PUBLIC', '2': 6},
    const {'1': 'ROOM_CONVERTED_TO_PRIVATE', '2': 7},
    const {'1': 'MEMBER_JOINED_BY_INVITE_LINK', '2': 8},
    const {'1': 'ROOM_DELETED', '2': 9},
    const {'1': 'MISSED_VOICE_CALL', '2': 10},
    const {'1': 'MISSED_VIDEO_CALL', '2': 11},
    const {'1': 'MISSED_SCREEN_SHARE', '2': 12},
    const {'1': 'MISSED_SECRET_CHAT', '2': 13},
    const {'1': 'PINNED_MESSAGE', '2': 14},
  ],
};

const ChannelType$json = const {
  '1': 'ChannelType',
  '2': const [
    const {'1': 'CHANNEL_UNKNOWN', '2': 0},
    const {'1': 'CHANNEL_PRIMARY', '2': 1},
    const {'1': 'CHANNEL_NORMAL', '2': 2},
    const {'1': 'CHANNEL_SAVES', '2': 3},
    const {'1': 'CHANNEL_STORE', '2': 4},
  ],
};

const ChannelPrivacy$json = const {
  '1': 'ChannelPrivacy',
  '2': const [
    const {'1': 'CHANNEL_UNKNOWN_AB', '2': 0},
    const {'1': 'CHANNEL_OPEN', '2': 1},
    const {'1': 'CHANNEL_PRIVATE_LINK', '2': 2},
    const {'1': 'CHANNEL_CREATOR', '2': 3},
  ],
};

const Cast$json = const {
  '1': 'Cast',
  '2': const [
    const {'1': 'UNKNOWN', '2': 0},
    const {'1': 'USER', '2': 1},
    const {'1': 'CHANNEL', '2': 2},
    const {'1': 'GROUP', '2': 3},
    const {'1': 'BOT', '2': 4},
    const {'1': 'STORE', '2': 6},
    const {'1': 'TAG', '2': 7},
  ],
};

const ReactionType$json = const {
  '1': 'ReactionType',
  '2': const [
    const {'1': 'NONE', '2': 0},
    const {'1': 'LIKE', '2': 1},
  ],
};

const SessionPlatform$json = const {
  '1': 'SessionPlatform',
  '2': const [
    const {'1': 'UNKNOWN_PLATFORM', '2': 0},
    const {'1': 'ANDROID', '2': 1},
    const {'1': 'IOS', '2': 2},
    const {'1': 'MAC_OS', '2': 3},
    const {'1': 'WINDOWS', '2': 4},
    const {'1': 'LINUX', '2': 5},
    const {'1': 'BLACK_BERRY', '2': 6},
    const {'1': 'Web', '2': 7},
  ],
};

const GroupPrivacy$json = const {
  '1': 'GroupPrivacy',
  '2': const [
    const {'1': 'GROUP_UNKNOWN_GP', '2': 0},
    const {'1': 'GROUP_OPEN', '2': 1},
    const {'1': 'GROUP_PRIVATE_LINK', '2': 2},
  ],
};

const GroupMemberRole$json = const {
  '1': 'GroupMemberRole',
  '2': const [
    const {'1': 'MEMBER_Unknown', '2': 0},
    const {'1': 'MEMBER_CREATOR', '2': 1},
    const {'1': 'MEMBER_MODERATOR', '2': 3},
    const {'1': 'MEMBER_NORMAL_USER', '2': 2},
  ],
};

const Message$json = const {
  '1': 'Message',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 6, '10': 'gid'},
    const {'1': 'by_user_cid', '3': 2, '4': 1, '5': 13, '10': 'byUserCid'},
    const {'1': 'by_channel_cid', '3': 100, '4': 1, '5': 13, '10': 'byChannelCid'},
    const {'1': 'post_type', '3': 3, '4': 1, '5': 13, '10': 'postType'},
    const {'1': 'media_id', '3': 4, '4': 1, '5': 3, '10': 'mediaId'},
    const {'1': 'file_ref_id', '3': 5, '4': 1, '5': 3, '10': 'fileRefId'},
    const {'1': 'post_key', '3': 6, '4': 1, '5': 9, '10': 'postKey'},
    const {'1': 'text', '3': 7, '4': 1, '5': 9, '10': 'text'},
    const {'1': 'rich_text', '3': 8, '4': 1, '5': 9, '10': 'richText'},
    const {'1': 'title', '3': 109, '4': 1, '5': 9, '10': 'title'},
    const {'1': 'shared_to', '3': 10, '4': 1, '5': 13, '10': 'sharedTo'},
    const {'1': 'via', '3': 12, '4': 1, '5': 13, '10': 'via'},
    const {'1': 'seq', '3': 13, '4': 1, '5': 13, '10': 'seq'},
    const {'1': 'version_time', '3': 108, '4': 1, '5': 4, '10': 'versionTime'},
    const {'1': 'edited_time', '3': 17, '4': 1, '5': 13, '10': 'editedTime'},
    const {'1': 'created_time', '3': 18, '4': 1, '5': 13, '10': 'createdTime'},
    const {'1': 'verified', '3': 111, '4': 1, '5': 8, '10': 'verified'},
    const {'1': 'delivery_status', '3': 105, '4': 1, '5': 14, '6': '.MessageDeliveryStatues', '10': 'deliveryStatus'},
    const {'1': 'delivery_time', '3': 106, '4': 1, '5': 13, '10': 'deliveryTime'},
    const {'1': 'previous_message_id', '3': 180, '4': 1, '5': 4, '10': 'previousMessageId'},
    const {'1': 'deleted', '3': 15, '4': 1, '5': 8, '10': 'deleted'},
    const {'1': 'flags', '3': 112, '4': 1, '5': 13, '10': 'flags'},
    const {'1': 'forward', '3': 16, '4': 1, '5': 11, '6': '.Message', '10': 'forward'},
    const {'1': 'reply_to', '3': 50, '4': 1, '5': 11, '6': '.Message', '10': 'replyTo'},
    const {'1': 'counts', '3': 101, '4': 1, '5': 11, '6': '.MessageCount', '10': 'counts'},
    const {'1': 'setting', '3': 102, '4': 1, '5': 11, '6': '.MessageSetting', '10': 'setting'},
    const {'1': 'Product', '3': 110, '4': 1, '5': 11, '6': '.Product', '10': 'Product'},
    const {'1': 'files', '3': 103, '4': 3, '5': 11, '6': '.FileMsg', '10': 'files'},
  ],
};

const MessageCount$json = const {
  '1': 'MessageCount',
  '2': const [
    const {'1': 'message_gid', '3': 1, '4': 1, '5': 6, '10': 'messageGid'},
    const {'1': 'comments_count', '3': 2, '4': 1, '5': 13, '10': 'commentsCount'},
    const {'1': 'likes_count', '3': 3, '4': 1, '5': 13, '10': 'likesCount'},
    const {'1': 'views_count', '3': 4, '4': 1, '5': 3, '10': 'viewsCount'},
    const {'1': 're_shared_count', '3': 5, '4': 1, '5': 13, '10': 'reSharedCount'},
    const {'1': 'chat_shared_count', '3': 6, '4': 1, '5': 13, '10': 'chatSharedCount'},
  ],
};

const MessageSetting$json = const {
  '1': 'MessageSetting',
  '2': const [
    const {'1': 'disable_comment', '3': 11, '4': 1, '5': 13, '10': 'disableComment'},
  ],
};

const MessageLog$json = const {
  '1': 'MessageLog',
  '2': const [
    const {'1': 'log_type', '3': 10, '4': 1, '5': 14, '6': '.MessageLogType', '10': 'logType'},
    const {'1': 'by_user_cid', '3': 2, '4': 1, '5': 13, '10': 'byUserCid'},
    const {'1': 'by_channel_cid', '3': 50, '4': 1, '5': 13, '10': 'byChannelCid'},
    const {'1': 'target_user_cid', '3': 3, '4': 1, '5': 13, '10': 'targetUserCid'},
    const {'1': 'target_channel_cid', '3': 4, '4': 1, '5': 13, '10': 'targetChannelCid'},
    const {'1': 'target_channel_view', '3': 11, '4': 1, '5': 11, '6': '.Channel', '10': 'targetChannelView'},
  ],
};

const Channel$json = const {
  '1': 'Channel',
  '2': const [
    const {'1': 'cid', '3': 1, '4': 1, '5': 13, '10': 'cid'},
    const {'1': 'user_name', '3': 2, '4': 1, '5': 9, '10': 'userName'},
    const {'1': 'first_name', '3': 4, '4': 1, '5': 9, '10': 'firstName'},
    const {'1': 'last_name', '3': 5, '4': 1, '5': 9, '10': 'lastName'},
    const {'1': 'creator_user_cid', '3': 7, '4': 1, '5': 13, '10': 'creatorUserCid'},
    const {'1': 'is_verified', '3': 6, '4': 1, '5': 13, '10': 'isVerified'},
    const {'1': 'avatar_count', '3': 40, '4': 1, '5': 3, '10': 'avatarCount'},
    const {'1': 'about', '3': 16, '4': 1, '5': 9, '10': 'about'},
    const {'1': 'invite_link_hash', '3': 17, '4': 1, '5': 9, '10': 'inviteLinkHash'},
    const {'1': 'post_seq', '3': 19, '4': 1, '5': 13, '10': 'postSeq'},
    const {'1': 'sort_time', '3': 20, '4': 1, '5': 6, '10': 'sortTime'},
    const {'1': 'sync_time', '3': 21, '4': 1, '5': 6, '10': 'syncTime'},
    const {'1': 'created_time', '3': 36, '4': 1, '5': 13, '10': 'createdTime'},
    const {'1': 'is_deleted', '3': 38, '4': 1, '5': 13, '10': 'isDeleted'},
    const {'1': 'is_banned', '3': 39, '4': 1, '5': 13, '10': 'isBanned'},
    const {'1': 'notification_setting', '3': 90, '4': 1, '5': 11, '6': '.ChannelNotificationSetting', '10': 'notificationSetting'},
    const {'1': 'privacy', '3': 9, '4': 1, '5': 14, '6': '.ChannelPrivacy', '10': 'privacy'},
    const {'1': 'channel_type', '3': 42, '4': 1, '5': 14, '6': '.ChannelType', '10': 'channelType'},
    const {'1': 'counts', '3': 41, '4': 1, '5': 11, '6': '.ChannelCounts', '10': 'counts'},
    const {'1': 'last_message', '3': 25, '4': 1, '5': 11, '6': '.Message', '10': 'lastMessage'},
    const {'1': 'pinned_message', '3': 26, '4': 1, '5': 11, '6': '.Message', '10': 'pinnedMessage'},
    const {'1': 'avatar', '3': 100, '4': 1, '5': 11, '6': '.Message', '10': 'avatar'},
  ],
};

const ChannelPrivacySetting$json = const {
  '1': 'ChannelPrivacySetting',
  '2': const [
    const {'1': 'online_privacy', '3': 10, '4': 1, '5': 13, '10': 'onlinePrivacy'},
    const {'1': 'call_privacy', '3': 11, '4': 1, '5': 13, '10': 'callPrivacy'},
    const {'1': 'add_to_group_privacy', '3': 12, '4': 1, '5': 13, '10': 'addToGroupPrivacy'},
    const {'1': 'seen_message_privacy', '3': 13, '4': 1, '5': 13, '10': 'seenMessagePrivacy'},
  ],
};

const ChannelNotificationSetting$json = const {
  '1': 'ChannelNotificationSetting',
  '2': const [
    const {'1': 'social_led_on', '3': 2, '4': 1, '5': 8, '10': 'socialLedOn'},
    const {'1': 'social_led_color', '3': 3, '4': 1, '5': 9, '10': 'socialLedColor'},
    const {'1': 'request_to_follow_you', '3': 4, '4': 1, '5': 8, '10': 'requestToFollowYou'},
    const {'1': 'followed_channel', '3': 5, '4': 1, '5': 8, '10': 'followedChannel'},
    const {'1': 'accepted_channel_follow_request', '3': 6, '4': 1, '5': 8, '10': 'acceptedChannelFollowRequest'},
    const {'1': 'channel_message_liked', '3': 7, '4': 1, '5': 8, '10': 'channelMessageLiked'},
    const {'1': 'channel_message_commented', '3': 8, '4': 1, '5': 8, '10': 'channelMessageCommented'},
    const {'1': 'mentioned_channel_in_message', '3': 9, '4': 1, '5': 8, '10': 'mentionedChannelInMessage'},
    const {'1': 'mentioned_channel_in_comment', '3': 10, '4': 1, '5': 8, '10': 'mentionedChannelInComment'},
    const {'1': 'contacts_joined', '3': 11, '4': 1, '5': 8, '10': 'contactsJoined'},
    const {'1': 'direct_message', '3': 12, '4': 1, '5': 8, '10': 'directMessage'},
    const {'1': 'direct_alert', '3': 13, '4': 1, '5': 8, '10': 'directAlert'},
    const {'1': 'direct_preview', '3': 14, '4': 1, '5': 8, '10': 'directPreview'},
    const {'1': 'direct_led_on', '3': 15, '4': 1, '5': 8, '10': 'directLedOn'},
    const {'1': 'direct_led_color', '3': 16, '4': 1, '5': 8, '10': 'directLedColor'},
    const {'1': 'direct_vibrate', '3': 17, '4': 1, '5': 8, '10': 'directVibrate'},
    const {'1': 'direct_popup', '3': 18, '4': 1, '5': 8, '10': 'directPopup'},
    const {'1': 'direct_sound', '3': 19, '4': 1, '5': 8, '10': 'directSound'},
    const {'1': 'direct_priority', '3': 20, '4': 1, '5': 8, '10': 'directPriority'},
  ],
};

const ChannelCounts$json = const {
  '1': 'ChannelCounts',
  '2': const [
    const {'1': 'followers_count', '3': 20, '4': 1, '5': 13, '10': 'followersCount'},
    const {'1': 'following_count', '3': 21, '4': 1, '5': 13, '10': 'followingCount'},
    const {'1': 'posts_count', '3': 22, '4': 1, '5': 13, '10': 'postsCount'},
    const {'1': 'media_count', '3': 23, '4': 1, '5': 13, '10': 'mediaCount'},
    const {'1': 'photo_count', '3': 24, '4': 1, '5': 13, '10': 'photoCount'},
    const {'1': 'video_count', '3': 25, '4': 1, '5': 13, '10': 'videoCount'},
    const {'1': 'gif_count', '3': 26, '4': 1, '5': 13, '10': 'gifCount'},
    const {'1': 'audio_count', '3': 27, '4': 1, '5': 13, '10': 'audioCount'},
    const {'1': 'voice_count', '3': 28, '4': 1, '5': 13, '10': 'voiceCount'},
    const {'1': 'file_count', '3': 29, '4': 1, '5': 13, '10': 'fileCount'},
    const {'1': 'link_count', '3': 30, '4': 1, '5': 13, '10': 'linkCount'},
    const {'1': 'board_count', '3': 31, '4': 1, '5': 13, '10': 'boardCount'},
    const {'1': 'pined_count', '3': 32, '4': 1, '5': 13, '10': 'pinedCount'},
    const {'1': 'likes_count', '3': 33, '4': 1, '5': 13, '10': 'likesCount'},
    const {'1': 'reshared_count', '3': 34, '4': 1, '5': 13, '10': 'resharedCount'},
  ],
};

const Store$json = const {
  '1': 'Store',
  '2': const [
    const {'1': 'address', '3': 1, '4': 1, '5': 9, '10': 'address'},
    const {'1': 'phone', '3': 2, '4': 1, '5': 9, '10': 'phone'},
  ],
};

const Product$json = const {
  '1': 'Product',
  '2': const [
    const {'1': 'product_id', '3': 1, '4': 1, '5': 13, '10': 'productId'},
    const {'1': 'category_id', '3': 53, '4': 1, '5': 13, '10': 'categoryId'},
    const {'1': 'category', '3': 50, '4': 1, '5': 9, '10': 'category'},
    const {'1': 'brand', '3': 51, '4': 1, '5': 9, '10': 'brand'},
    const {'1': 'fee_rate', '3': 3, '4': 1, '5': 13, '10': 'feeRate'},
    const {'1': 'sales_count', '3': 5, '4': 1, '5': 13, '10': 'salesCount'},
    const {'1': 'price', '3': 6, '4': 1, '5': 13, '10': 'price'},
  ],
};

const ProductPriceInfo$json = const {
  '1': 'ProductPriceInfo',
  '2': const [
    const {'1': 'price', '3': 1, '4': 1, '5': 13, '10': 'price'},
    const {'1': 'discount_price', '3': 6, '4': 1, '5': 13, '10': 'discountPrice'},
    const {'1': 'rate', '3': 3, '4': 1, '5': 13, '10': 'rate'},
  ],
};

const Action$json = const {
  '1': 'Action',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 6, '10': 'gid'},
    const {'1': 'actor_user_cid', '3': 2, '4': 1, '5': 13, '10': 'actorUserCid'},
    const {'1': 'actor_channel_cid', '3': 50, '4': 1, '5': 13, '10': 'actorChannelCid'},
    const {'1': 'action_type', '3': 3, '4': 1, '5': 13, '10': 'actionType'},
    const {'1': 'on_user_cid', '3': 4, '4': 1, '5': 13, '10': 'onUserCid'},
    const {'1': 'on_channel_cid', '3': 9, '4': 1, '5': 6, '10': 'onChannelCid'},
    const {'1': 'message_gid', '3': 5, '4': 1, '5': 6, '10': 'messageGid'},
    const {'1': 'comment_gid', '3': 6, '4': 1, '5': 6, '10': 'commentGid'},
    const {'1': 'hash_murm64', '3': 7, '4': 1, '5': 3, '10': 'hashMurm64'},
    const {'1': 'created_time', '3': 8, '4': 1, '5': 13, '10': 'createdTime'},
  ],
  '4': const [Action_ActionType$json],
};

const Action_ActionType$json = const {
  '1': 'ActionType',
  '2': const [
    const {'1': 'UNKNOWN_AT', '2': 0},
    const {'1': 'LIKED', '2': 1},
    const {'1': 'FOLLOWED', '2': 2},
  ],
};

const Blocked$json = const {
  '1': 'Blocked',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 6, '10': 'gid'},
    const {'1': 'user_cid', '3': 2, '4': 1, '5': 13, '10': 'userCid'},
    const {'1': 'blocked_user_cid', '3': 6, '4': 1, '5': 13, '10': 'blockedUserCid'},
    const {'1': 'blocked_channel_cid', '3': 7, '4': 1, '5': 13, '10': 'blockedChannelCid'},
    const {'1': 'created_time', '3': 5, '4': 1, '5': 13, '10': 'createdTime'},
  ],
};

const Comment$json = const {
  '1': 'Comment',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 6, '10': 'gid'},
    const {'1': 'channel_cid', '3': 9, '4': 1, '5': 13, '10': 'channelCid'},
    const {'1': 'by_cast_cid', '3': 2, '4': 1, '5': 13, '10': 'byCastCid'},
    const {'1': 'message_gid', '3': 3, '4': 1, '5': 6, '10': 'messageGid'},
    const {'1': 'text', '3': 4, '4': 1, '5': 9, '10': 'text'},
    const {'1': 'likes_count', '3': 5, '4': 1, '5': 13, '10': 'likesCount'},
    const {'1': 'edit_time', '3': 6, '4': 1, '5': 13, '10': 'editTime'},
    const {'1': 'created_time', '3': 7, '4': 1, '5': 13, '10': 'createdTime'},
  ],
};

const Followed$json = const {
  '1': 'Followed',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 6, '10': 'gid'},
    const {'1': 'by_user_cid', '3': 2, '4': 1, '5': 13, '10': 'byUserCid'},
    const {'1': 'by_channel_cid', '3': 10, '4': 1, '5': 13, '10': 'byChannelCid'},
    const {'1': 'target_cid', '3': 3, '4': 1, '5': 13, '10': 'targetCid'},
    const {'1': 'target_channel_id', '3': 11, '4': 1, '5': 13, '10': 'targetChannelId'},
    const {'1': 'created_time', '3': 4, '4': 1, '5': 13, '10': 'createdTime'},
  ],
};

const Reaction$json = const {
  '1': 'Reaction',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 6, '10': 'gid'},
    const {'1': 'for_message_cid', '3': 2, '4': 1, '5': 3, '10': 'forMessageCid'},
    const {'1': 'for_channel_cid', '3': 11, '4': 1, '5': 3, '10': 'forChannelCid'},
    const {'1': 'by_user_cid', '3': 3, '4': 1, '5': 13, '10': 'byUserCid'},
    const {'1': 'by_channel_cid', '3': 10, '4': 1, '5': 13, '10': 'byChannelCid'},
    const {'1': 'created_time', '3': 5, '4': 1, '5': 13, '10': 'createdTime'},
  ],
};

const Contact$json = const {
  '1': 'Contact',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 6, '10': 'gid'},
    const {'1': 'user_cid', '3': 2, '4': 1, '5': 13, '10': 'userCid'},
    const {'1': 'channel_cid', '3': 10, '4': 1, '5': 13, '10': 'channelCid'},
    const {'1': 'client_id', '3': 3, '4': 1, '5': 3, '10': 'clientId'},
    const {'1': 'phone', '3': 4, '4': 1, '5': 9, '10': 'phone'},
    const {'1': 'first_name', '3': 5, '4': 1, '5': 9, '10': 'firstName'},
    const {'1': 'last_name', '3': 6, '4': 1, '5': 9, '10': 'lastName'},
    const {'1': 'target_user_cid', '3': 12, '4': 1, '5': 13, '10': 'targetUserCid'},
    const {'1': 'target_channel_cid', '3': 15, '4': 1, '5': 13, '10': 'targetChannelCid'},
  ],
};

const Session$json = const {
  '1': 'Session',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 6, '10': 'gid'},
    const {'1': 'session_uuid', '3': 2, '4': 1, '5': 9, '10': 'sessionUuid'},
    const {'1': 'user_cid', '3': 3, '4': 1, '5': 13, '10': 'userCid'},
    const {'1': 'last_ip_address', '3': 4, '4': 1, '5': 9, '10': 'lastIpAddress'},
    const {'1': 'user_agent', '3': 8, '4': 1, '5': 9, '10': 'userAgent'},
    const {'1': 'platform', '3': 9, '4': 1, '5': 14, '6': '.SessionPlatform', '10': 'platform'},
    const {'1': 'app_version', '3': 5, '4': 1, '5': 13, '10': 'appVersion'},
    const {'1': 'active_time', '3': 6, '4': 1, '5': 13, '10': 'activeTime'},
    const {'1': 'created_time', '3': 7, '4': 1, '5': 13, '10': 'createdTime'},
  ],
};

const Sms$json = const {
  '1': 'Sms',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 6, '10': 'gid'},
    const {'1': 'hash', '3': 2, '4': 1, '5': 9, '10': 'hash'},
    const {'1': 'app_uuid', '3': 3, '4': 1, '5': 9, '10': 'appUuid'},
    const {'1': 'client_phone', '3': 4, '4': 1, '5': 9, '10': 'clientPhone'},
    const {'1': 'genrated_code', '3': 5, '4': 1, '5': 13, '10': 'genratedCode'},
    const {'1': 'sms_sender_number', '3': 6, '4': 1, '5': 9, '10': 'smsSenderNumber'},
    const {'1': 'sms_send_statues', '3': 7, '4': 1, '5': 9, '10': 'smsSendStatues'},
    const {'1': 'sms_http_body', '3': 8, '4': 1, '5': 9, '10': 'smsHttpBody'},
    const {'1': 'err', '3': 9, '4': 1, '5': 9, '10': 'err'},
    const {'1': 'carrier', '3': 10, '4': 1, '5': 9, '10': 'carrier'},
    const {'1': 'country', '3': 11, '4': 1, '5': 12, '10': 'country'},
    const {'1': 'is_valid_phone', '3': 12, '4': 1, '5': 13, '10': 'isValidPhone'},
    const {'1': 'is_confirmed', '3': 13, '4': 1, '5': 13, '10': 'isConfirmed'},
    const {'1': 'is_login', '3': 14, '4': 1, '5': 13, '10': 'isLogin'},
    const {'1': 'is_register', '3': 15, '4': 1, '5': 13, '10': 'isRegister'},
    const {'1': 'retried_count', '3': 16, '4': 1, '5': 13, '10': 'retriedCount'},
    const {'1': 'ttl', '3': 17, '4': 1, '5': 13, '10': 'ttl'},
  ],
};

const Tag$json = const {
  '1': 'Tag',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 6, '10': 'gid'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'count', '3': 3, '4': 1, '5': 13, '10': 'count'},
    const {'1': 'is_blocked', '3': 5, '4': 1, '5': 8, '10': 'isBlocked'},
    const {'1': 'group_cid', '3': 6, '4': 1, '5': 13, '10': 'groupCid'},
    const {'1': 'created_time', '3': 7, '4': 1, '5': 13, '10': 'createdTime'},
  ],
};

const User$json = const {
  '1': 'User',
  '2': const [
    const {'1': 'cid', '3': 1, '4': 1, '5': 13, '10': 'cid'},
    const {'1': 'phone', '3': 14, '4': 1, '5': 9, '10': 'phone'},
    const {'1': 'email', '3': 15, '4': 1, '5': 9, '10': 'email'},
    const {'1': 'password_hash', '3': 17, '4': 1, '5': 9, '10': 'passwordHash'},
    const {'1': 'password_salt', '3': 18, '4': 1, '5': 9, '10': 'passwordSalt'},
    const {'1': 'created_time', '3': 36, '4': 1, '5': 13, '10': 'createdTime'},
    const {'1': 'version_time', '3': 37, '4': 1, '5': 13, '10': 'versionTime'},
    const {'1': 'is_deleted', '3': 38, '4': 1, '5': 13, '10': 'isDeleted'},
    const {'1': 'is_banned', '3': 39, '4': 1, '5': 13, '10': 'isBanned'},
    const {'1': 'primary_channel_changed_time', '3': 50, '4': 1, '5': 13, '10': 'primaryChannelChangedTime'},
    const {'1': 'UserCounts', '3': 80, '4': 1, '5': 11, '6': '.Channel', '10': 'UserCounts'},
    const {'1': 'primary_channel', '3': 100, '4': 1, '5': 11, '6': '.Channel', '10': 'primaryChannel'},
    const {'1': 'channels', '3': 101, '4': 3, '5': 11, '6': '.Channel', '10': 'channels'},
    const {'1': 'sessions', '3': 102, '4': 3, '5': 11, '6': '.Session', '10': 'sessions'},
  ],
};

const UserCounts$json = const {
  '1': 'UserCounts',
  '2': const [
    const {'1': 'created_channels', '3': 1, '4': 1, '5': 13, '10': 'createdChannels'},
  ],
};

const UserRelation$json = const {
  '1': 'UserRelation',
  '2': const [
    const {'1': 'rel_nano_id', '3': 1, '4': 1, '5': 3, '10': 'relNanoId'},
    const {'1': 'user_cid', '3': 2, '4': 1, '5': 13, '10': 'userCid'},
    const {'1': 'peer_user_id', '3': 3, '4': 1, '5': 13, '10': 'peerUserId'},
    const {'1': 'follwing', '3': 4, '4': 1, '5': 13, '10': 'follwing'},
    const {'1': 'followed', '3': 5, '4': 1, '5': 13, '10': 'followed'},
    const {'1': 'in_contacts', '3': 6, '4': 1, '5': 13, '10': 'inContacts'},
    const {'1': 'mutual_contact', '3': 7, '4': 1, '5': 13, '10': 'mutualContact'},
    const {'1': 'is_favorite', '3': 8, '4': 1, '5': 13, '10': 'isFavorite'},
    const {'1': 'notify', '3': 9, '4': 1, '5': 13, '10': 'notify'},
  ],
};

const Chat$json = const {
  '1': 'Chat',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 6, '10': 'gid'},
    const {'1': 'user_cid', '3': 5, '4': 1, '5': 13, '10': 'userCid'},
    const {'1': 'channel_cid', '3': 40, '4': 1, '5': 13, '10': 'channelCid'},
    const {'1': 'peer_user_cid', '3': 6, '4': 1, '5': 13, '10': 'peerUserCid'},
    const {'1': 'peer_channel_cid', '3': 41, '4': 1, '5': 13, '10': 'peerChannelCid'},
    const {'1': 'group_cid', '3': 7, '4': 1, '5': 13, '10': 'groupCid'},
    const {'1': 'custom_title', '3': 9, '4': 1, '5': 9, '10': 'customTitle'},
    const {'1': 'pin_time_ms', '3': 10, '4': 1, '5': 6, '10': 'pinTimeMs'},
    const {'1': 'from_msg_gid', '3': 11, '4': 1, '5': 3, '10': 'fromMsgGid'},
    const {'1': 'unseen_count', '3': 12, '4': 1, '5': 13, '10': 'unseenCount'},
    const {'1': 'seq', '3': 13, '4': 1, '5': 13, '10': 'seq'},
    const {'1': 'my_last_seen_seq', '3': 16, '4': 1, '5': 13, '10': 'myLastSeenSeq'},
    const {'1': 'my_last_seen_msg_id', '3': 17, '4': 1, '5': 3, '10': 'myLastSeenMsgId'},
    const {'1': 'peer_last_seen_msg_id', '3': 18, '4': 1, '5': 3, '10': 'peerLastSeenMsgId'},
    const {'1': 'my_last_delivered_seq', '3': 19, '4': 1, '5': 13, '10': 'myLastDeliveredSeq'},
    const {'1': 'my_last_delivered_msg_id', '3': 20, '4': 1, '5': 3, '10': 'myLastDeliveredMsgId'},
    const {'1': 'peer_last_delivered_msg_id', '3': 21, '4': 1, '5': 3, '10': 'peerLastDeliveredMsgId'},
    const {'1': 'is_active', '3': 22, '4': 1, '5': 8, '10': 'isActive'},
    const {'1': 'mute_until', '3': 29, '4': 1, '5': 13, '10': 'muteUntil'},
    const {'1': 'sort_time_ms', '3': 30, '4': 1, '5': 3, '10': 'sortTimeMs'},
    const {'1': 'created_time', '3': 33, '4': 1, '5': 13, '10': 'createdTime'},
    const {'1': 'sort_time', '3': 45, '4': 1, '5': 6, '10': 'sortTime'},
    const {'1': 'channel', '3': 48, '4': 1, '5': 11, '6': '.Channel', '10': 'channel'},
    const {'1': 'contact', '3': 49, '4': 1, '5': 11, '6': '.Contact', '10': 'contact'},
    const {'1': 'group', '3': 50, '4': 1, '5': 11, '6': '.Group', '10': 'group'},
    const {'1': 'last_message', '3': 25, '4': 1, '5': 11, '6': '.Message', '10': 'lastMessage'},
    const {'1': 'pinned_message', '3': 26, '4': 1, '5': 11, '6': '.Message', '10': 'pinnedMessage'},
    const {'1': 'avatar', '3': 44, '4': 1, '5': 11, '6': '.FileMsg', '10': 'avatar'},
    const {'1': 'group_member', '3': 43, '4': 1, '5': 11, '6': '.GroupMember', '10': 'groupMember'},
    const {'1': 'draft', '3': 46, '4': 1, '5': 11, '6': '.MessageDraft', '10': 'draft'},
    const {'1': 'custom_notification', '3': 47, '4': 1, '5': 11, '6': '.ChatCustomNotification', '10': 'customNotification'},
  ],
};

const MessageDraft$json = const {
  '1': 'MessageDraft',
  '2': const [
    const {'1': 'draft_text', '3': 34, '4': 1, '5': 9, '10': 'draftText'},
    const {'1': 'drat_reply_to_msg_id', '3': 35, '4': 1, '5': 3, '10': 'dratReplyToMsgId'},
  ],
};

const ChatCustomNotification$json = const {
  '1': 'ChatCustomNotification',
  '2': const [
    const {'1': 'alert', '3': 13, '4': 1, '5': 8, '10': 'alert'},
    const {'1': 'preview', '3': 14, '4': 1, '5': 8, '10': 'preview'},
    const {'1': 'led_on', '3': 15, '4': 1, '5': 8, '10': 'ledOn'},
    const {'1': 'led_color', '3': 16, '4': 1, '5': 8, '10': 'ledColor'},
    const {'1': 'vibrate', '3': 17, '4': 1, '5': 8, '10': 'vibrate'},
    const {'1': 'popup', '3': 18, '4': 1, '5': 8, '10': 'popup'},
    const {'1': 'sound', '3': 19, '4': 1, '5': 8, '10': 'sound'},
    const {'1': 'priority', '3': 20, '4': 1, '5': 8, '10': 'priority'},
  ],
};

const Group$json = const {
  '1': 'Group',
  '2': const [
    const {'1': 'cid', '3': 1, '4': 1, '5': 13, '10': 'cid'},
    const {'1': 'group_title', '3': 3, '4': 1, '5': 9, '10': 'groupTitle'},
    const {'1': 'user_name', '3': 4, '4': 1, '5': 9, '10': 'userName'},
    const {'1': 'creator_user_cid', '3': 7, '4': 1, '5': 13, '10': 'creatorUserCid'},
    const {'1': 'creator_channel_cid', '3': 31, '4': 1, '5': 13, '10': 'creatorChannelCid'},
    const {'1': 'group_privacy', '3': 8, '4': 1, '5': 14, '6': '.GroupPrivacy', '10': 'groupPrivacy'},
    const {'1': 'history_viewable', '3': 9, '4': 1, '5': 8, '10': 'historyViewable'},
    const {'1': 'seq', '3': 10, '4': 1, '5': 13, '10': 'seq'},
    const {'1': 'avatar_count', '3': 14, '4': 1, '5': 13, '10': 'avatarCount'},
    const {'1': 'about', '3': 15, '4': 1, '5': 9, '10': 'about'},
    const {'1': 'invite_link_hash', '3': 16, '4': 1, '5': 9, '10': 'inviteLinkHash'},
    const {'1': 'members_count', '3': 17, '4': 1, '5': 13, '10': 'membersCount'},
    const {'1': 'admins_count', '3': 18, '4': 1, '5': 13, '10': 'adminsCount'},
    const {'1': 'moderator_counts', '3': 19, '4': 1, '5': 13, '10': 'moderatorCounts'},
    const {'1': 'sort_time', '3': 20, '4': 1, '5': 6, '10': 'sortTime'},
    const {'1': 'sync_time', '3': 40, '4': 1, '5': 6, '10': 'syncTime'},
    const {'1': 'created_time', '3': 21, '4': 1, '5': 13, '10': 'createdTime'},
    const {'1': 'is_deleted', '3': 23, '4': 1, '5': 8, '10': 'isDeleted'},
    const {'1': 'is_banned', '3': 24, '4': 1, '5': 8, '10': 'isBanned'},
    const {'1': 'last_message', '3': 25, '4': 1, '5': 11, '6': '.Message', '10': 'lastMessage'},
    const {'1': 'pinned_message', '3': 26, '4': 1, '5': 11, '6': '.Message', '10': 'pinnedMessage'},
    const {'1': 'avatar', '3': 27, '4': 1, '5': 11, '6': '.FileMsg', '10': 'avatar'},
  ],
};

const GroupMember$json = const {
  '1': 'GroupMember',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 3, '10': 'gid'},
    const {'1': 'group_cid', '3': 2, '4': 1, '5': 3, '10': 'groupCid'},
    const {'1': 'user_cid', '3': 3, '4': 1, '5': 13, '10': 'userCid'},
    const {'1': 'channel_cid', '3': 8, '4': 1, '5': 13, '10': 'channelCid'},
    const {'1': 'by_user_cid', '3': 4, '4': 1, '5': 13, '10': 'byUserCid'},
    const {'1': 'by_channel_cid', '3': 7, '4': 1, '5': 13, '10': 'byChannelCid'},
    const {'1': 'group_role', '3': 5, '4': 1, '5': 14, '6': '.GroupMemberRole', '10': 'groupRole'},
    const {'1': 'created_time', '3': 6, '4': 1, '5': 13, '10': 'createdTime'},
  ],
};

const FileMsg$json = const {
  '1': 'FileMsg',
  '2': const [
    const {'1': 'gid', '3': 1, '4': 1, '5': 6, '10': 'gid'},
    const {'1': 'access_hash', '3': 2, '4': 1, '5': 13, '10': 'accessHash'},
    const {'1': 'file_type', '3': 3, '4': 1, '5': 13, '10': 'fileType'},
    const {'1': 'width', '3': 4, '4': 1, '5': 13, '10': 'width'},
    const {'1': 'height', '3': 5, '4': 1, '5': 13, '10': 'height'},
    const {'1': 'extension', '3': 6, '4': 1, '5': 9, '10': 'extension'},
    const {'1': 'user_cid', '3': 7, '4': 1, '5': 13, '10': 'userCid'},
    const {'1': 'data_thumb', '3': 8, '4': 1, '5': 12, '10': 'dataThumb'},
    const {'1': 'data', '3': 9, '4': 1, '5': 12, '10': 'data'},
  ],
};

