///
//  Generated code. Do not modify.
//  source: rpc_chat.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

const ChatSendMessageParam$json = const {
  '1': 'ChatSendMessageParam',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 13, '10': 'groupId'},
  ],
};

const ChatSendMessageResponse$json = const {
  '1': 'ChatSendMessageResponse',
};

const ChatEditMessageParam$json = const {
  '1': 'ChatEditMessageParam',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 13, '10': 'groupId'},
  ],
};

const ChatEditMessageResponse$json = const {
  '1': 'ChatEditMessageResponse',
};

const ChatDeleteMessagesParam$json = const {
  '1': 'ChatDeleteMessagesParam',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 13, '10': 'groupId'},
  ],
};

const ChatDeleteMessagesResponse$json = const {
  '1': 'ChatDeleteMessagesResponse',
};

const ChatDeleteHistoryParam$json = const {
  '1': 'ChatDeleteHistoryParam',
};

const ChatDeleteHistoryResponse$json = const {
  '1': 'ChatDeleteHistoryResponse',
};

const ChatAvatarChangeParam$json = const {
  '1': 'ChatAvatarChangeParam',
};

const ChatAvatarChangeResponse$json = const {
  '1': 'ChatAvatarChangeResponse',
};

const ChatSendDoingActionParam$json = const {
  '1': 'ChatSendDoingActionParam',
};

const ChatSendDoingActionResponse$json = const {
  '1': 'ChatSendDoingActionResponse',
};

const ChatReportChatParam$json = const {
  '1': 'ChatReportChatParam',
};

const ChatReportChatResponse$json = const {
  '1': 'ChatReportChatResponse',
};

const ChatGetFullMessageParam$json = const {
  '1': 'ChatGetFullMessageParam',
  '2': const [
    const {'1': 'group_id', '3': 1, '4': 1, '5': 13, '10': 'groupId'},
  ],
};

const ChatGetFullMessageResponse$json = const {
  '1': 'ChatGetFullMessageResponse',
};

const ChatGetMessagesListParam$json = const {
  '1': 'ChatGetMessagesListParam',
};

const ChatGetMessagesListResponse$json = const {
  '1': 'ChatGetMessagesListResponse',
};

const ChatGetMediaListParam$json = const {
  '1': 'ChatGetMediaListParam',
};

const ChatGetMediaListResponse$json = const {
  '1': 'ChatGetMediaListResponse',
};

const RPC_ChatServiceBase$json = const {
  '1': 'RPC_Chat',
  '2': const [
    const {'1': 'ChatSendMessage', '2': '.ChatSendMessageParam', '3': '.ChatSendMessageResponse'},
    const {'1': 'ChatEditMessage', '2': '.ChatEditMessageParam', '3': '.ChatEditMessageResponse'},
    const {'1': 'ChatDeleteMessages', '2': '.ChatDeleteMessagesParam', '3': '.ChatDeleteMessagesResponse'},
    const {'1': 'ChatDeleteHistory', '2': '.ChatDeleteHistoryParam', '3': '.ChatDeleteHistoryResponse'},
    const {'1': 'ChatSendDoingAction', '2': '.ChatSendDoingActionParam', '3': '.ChatSendDoingActionResponse'},
    const {'1': 'ChatReportChat', '2': '.ChatReportChatParam', '3': '.ChatReportChatResponse'},
    const {'1': 'ChatGetFull', '2': '.ChatGetFullMessageParam', '3': '.ChatGetFullMessageResponse'},
    const {'1': 'ChatGetMessagesList', '2': '.ChatGetMessagesListParam', '3': '.ChatGetMessagesListResponse'},
    const {'1': 'ChatGetMediaList', '2': '.ChatGetMediaListParam', '3': '.ChatGetMediaListResponse'},
  ],
};

const RPC_ChatServiceBase$messageJson = const {
  '.ChatSendMessageParam': ChatSendMessageParam$json,
  '.ChatSendMessageResponse': ChatSendMessageResponse$json,
  '.ChatEditMessageParam': ChatEditMessageParam$json,
  '.ChatEditMessageResponse': ChatEditMessageResponse$json,
  '.ChatDeleteMessagesParam': ChatDeleteMessagesParam$json,
  '.ChatDeleteMessagesResponse': ChatDeleteMessagesResponse$json,
  '.ChatDeleteHistoryParam': ChatDeleteHistoryParam$json,
  '.ChatDeleteHistoryResponse': ChatDeleteHistoryResponse$json,
  '.ChatSendDoingActionParam': ChatSendDoingActionParam$json,
  '.ChatSendDoingActionResponse': ChatSendDoingActionResponse$json,
  '.ChatReportChatParam': ChatReportChatParam$json,
  '.ChatReportChatResponse': ChatReportChatResponse$json,
  '.ChatGetFullMessageParam': ChatGetFullMessageParam$json,
  '.ChatGetFullMessageResponse': ChatGetFullMessageResponse$json,
  '.ChatGetMessagesListParam': ChatGetMessagesListParam$json,
  '.ChatGetMessagesListResponse': ChatGetMessagesListResponse$json,
  '.ChatGetMediaListParam': ChatGetMediaListParam$json,
  '.ChatGetMediaListResponse': ChatGetMediaListResponse$json,
};

