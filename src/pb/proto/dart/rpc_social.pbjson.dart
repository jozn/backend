///
//  Generated code. Do not modify.
//  source: rpc_social.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

const AddCommentParam$json = const {
  '1': 'AddCommentParam',
  '2': const [
    const {'1': 'text', '3': 1, '4': 1, '5': 9, '10': 'text'},
  ],
};

const AddCommentResponse$json = const {
  '1': 'AddCommentResponse',
  '2': const [
    const {'1': 'done', '3': 1, '4': 1, '5': 8, '10': 'done'},
    const {'1': 'text', '3': 2, '4': 1, '5': 9, '10': 'text'},
  ],
};

const DeleteCommentParam$json = const {
  '1': 'DeleteCommentParam',
};

const DeleteCommentResponse$json = const {
  '1': 'DeleteCommentResponse',
};

const EditCommentParam$json = const {
  '1': 'EditCommentParam',
};

const EditCommentResponse$json = const {
  '1': 'EditCommentResponse',
};

const LikeCommentParam$json = const {
  '1': 'LikeCommentParam',
};

const LikeCommentResponse$json = const {
  '1': 'LikeCommentResponse',
};

const AddSeenPostsParam$json = const {
  '1': 'AddSeenPostsParam',
};

const AddSeenPostsResponse$json = const {
  '1': 'AddSeenPostsResponse',
};

const LikePostParam$json = const {
  '1': 'LikePostParam',
};

const LikePostResponse$json = const {
  '1': 'LikePostResponse',
};

const UnLikePostParam$json = const {
  '1': 'UnLikePostParam',
};

const UnLikePostResponse$json = const {
  '1': 'UnLikePostResponse',
};

const FollowChannelParam$json = const {
  '1': 'FollowChannelParam',
};

const FollowChannelResponse$json = const {
  '1': 'FollowChannelResponse',
};

const UnFollowChannelParam$json = const {
  '1': 'UnFollowChannelParam',
};

const UnFollowChannelResponse$json = const {
  '1': 'UnFollowChannelResponse',
};

const PinChannelParam$json = const {
  '1': 'PinChannelParam',
};

const PinChannelResponse$json = const {
  '1': 'PinChannelResponse',
};

const UnPinChannelParam$json = const {
  '1': 'UnPinChannelParam',
};

const UnPinChannelResponse$json = const {
  '1': 'UnPinChannelResponse',
};

const BlockChannelParam$json = const {
  '1': 'BlockChannelParam',
};

const BlockChannelResponse$json = const {
  '1': 'BlockChannelResponse',
};

const UnBlockChannelParam$json = const {
  '1': 'UnBlockChannelParam',
};

const UnBlockChannelResponse$json = const {
  '1': 'UnBlockChannelResponse',
};

const RPC_SocialServiceBase$json = const {
  '1': 'RPC_Social',
  '2': const [
    const {'1': 'AddComment', '2': '.AddCommentParam', '3': '.AddCommentResponse'},
    const {'1': 'DeleteComment', '2': '.DeleteCommentParam', '3': '.DeleteCommentResponse'},
    const {'1': 'EditComment', '2': '.EditCommentParam', '3': '.EditCommentResponse'},
    const {'1': 'LikeComment', '2': '.LikeCommentParam', '3': '.LikeCommentResponse'},
    const {'1': 'AddSeenPosts', '2': '.AddSeenPostsParam', '3': '.AddSeenPostsResponse'},
    const {'1': 'LikePost', '2': '.LikePostParam', '3': '.LikePostResponse'},
    const {'1': 'UnLikePost', '2': '.UnLikePostParam', '3': '.UnLikePostResponse'},
    const {'1': 'FollowChannel', '2': '.FollowChannelParam', '3': '.FollowChannelResponse'},
    const {'1': 'UnFollowChannel', '2': '.UnFollowChannelParam', '3': '.UnFollowChannelResponse'},
    const {'1': 'PinChannel', '2': '.PinChannelParam', '3': '.PinChannelResponse'},
    const {'1': 'UnPinChannel', '2': '.UnPinChannelParam', '3': '.UnPinChannelResponse'},
    const {'1': 'BlockChannel', '2': '.BlockChannelParam', '3': '.BlockChannelResponse'},
    const {'1': 'UnBlockChannel', '2': '.UnBlockChannelParam', '3': '.UnBlockChannelResponse'},
  ],
};

const RPC_SocialServiceBase$messageJson = const {
  '.AddCommentParam': AddCommentParam$json,
  '.AddCommentResponse': AddCommentResponse$json,
  '.DeleteCommentParam': DeleteCommentParam$json,
  '.DeleteCommentResponse': DeleteCommentResponse$json,
  '.EditCommentParam': EditCommentParam$json,
  '.EditCommentResponse': EditCommentResponse$json,
  '.LikeCommentParam': LikeCommentParam$json,
  '.LikeCommentResponse': LikeCommentResponse$json,
  '.AddSeenPostsParam': AddSeenPostsParam$json,
  '.AddSeenPostsResponse': AddSeenPostsResponse$json,
  '.LikePostParam': LikePostParam$json,
  '.LikePostResponse': LikePostResponse$json,
  '.UnLikePostParam': UnLikePostParam$json,
  '.UnLikePostResponse': UnLikePostResponse$json,
  '.FollowChannelParam': FollowChannelParam$json,
  '.FollowChannelResponse': FollowChannelResponse$json,
  '.UnFollowChannelParam': UnFollowChannelParam$json,
  '.UnFollowChannelResponse': UnFollowChannelResponse$json,
  '.PinChannelParam': PinChannelParam$json,
  '.PinChannelResponse': PinChannelResponse$json,
  '.UnPinChannelParam': UnPinChannelParam$json,
  '.UnPinChannelResponse': UnPinChannelResponse$json,
  '.BlockChannelParam': BlockChannelParam$json,
  '.BlockChannelResponse': BlockChannelResponse$json,
  '.UnBlockChannelParam': UnBlockChannelParam$json,
  '.UnBlockChannelResponse': UnBlockChannelResponse$json,
};

