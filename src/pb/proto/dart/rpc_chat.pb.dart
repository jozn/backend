///
//  Generated code. Do not modify.
//  source: rpc_chat.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class ChatSendMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatSendMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChatSendMessageParam._() : super();
  factory ChatSendMessageParam() => create();
  factory ChatSendMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatSendMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatSendMessageParam clone() => ChatSendMessageParam()..mergeFromMessage(this);
  ChatSendMessageParam copyWith(void Function(ChatSendMessageParam) updates) => super.copyWith((message) => updates(message as ChatSendMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatSendMessageParam create() => ChatSendMessageParam._();
  ChatSendMessageParam createEmptyInstance() => create();
  static $pb.PbList<ChatSendMessageParam> createRepeated() => $pb.PbList<ChatSendMessageParam>();
  @$core.pragma('dart2js:noInline')
  static ChatSendMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatSendMessageParam>(create);
  static ChatSendMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class ChatSendMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatSendMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatSendMessageResponse._() : super();
  factory ChatSendMessageResponse() => create();
  factory ChatSendMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatSendMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatSendMessageResponse clone() => ChatSendMessageResponse()..mergeFromMessage(this);
  ChatSendMessageResponse copyWith(void Function(ChatSendMessageResponse) updates) => super.copyWith((message) => updates(message as ChatSendMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatSendMessageResponse create() => ChatSendMessageResponse._();
  ChatSendMessageResponse createEmptyInstance() => create();
  static $pb.PbList<ChatSendMessageResponse> createRepeated() => $pb.PbList<ChatSendMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static ChatSendMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatSendMessageResponse>(create);
  static ChatSendMessageResponse _defaultInstance;
}

class ChatEditMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatEditMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChatEditMessageParam._() : super();
  factory ChatEditMessageParam() => create();
  factory ChatEditMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatEditMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatEditMessageParam clone() => ChatEditMessageParam()..mergeFromMessage(this);
  ChatEditMessageParam copyWith(void Function(ChatEditMessageParam) updates) => super.copyWith((message) => updates(message as ChatEditMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatEditMessageParam create() => ChatEditMessageParam._();
  ChatEditMessageParam createEmptyInstance() => create();
  static $pb.PbList<ChatEditMessageParam> createRepeated() => $pb.PbList<ChatEditMessageParam>();
  @$core.pragma('dart2js:noInline')
  static ChatEditMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatEditMessageParam>(create);
  static ChatEditMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class ChatEditMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatEditMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatEditMessageResponse._() : super();
  factory ChatEditMessageResponse() => create();
  factory ChatEditMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatEditMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatEditMessageResponse clone() => ChatEditMessageResponse()..mergeFromMessage(this);
  ChatEditMessageResponse copyWith(void Function(ChatEditMessageResponse) updates) => super.copyWith((message) => updates(message as ChatEditMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatEditMessageResponse create() => ChatEditMessageResponse._();
  ChatEditMessageResponse createEmptyInstance() => create();
  static $pb.PbList<ChatEditMessageResponse> createRepeated() => $pb.PbList<ChatEditMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static ChatEditMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatEditMessageResponse>(create);
  static ChatEditMessageResponse _defaultInstance;
}

class ChatDeleteMessagesParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatDeleteMessagesParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChatDeleteMessagesParam._() : super();
  factory ChatDeleteMessagesParam() => create();
  factory ChatDeleteMessagesParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatDeleteMessagesParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatDeleteMessagesParam clone() => ChatDeleteMessagesParam()..mergeFromMessage(this);
  ChatDeleteMessagesParam copyWith(void Function(ChatDeleteMessagesParam) updates) => super.copyWith((message) => updates(message as ChatDeleteMessagesParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatDeleteMessagesParam create() => ChatDeleteMessagesParam._();
  ChatDeleteMessagesParam createEmptyInstance() => create();
  static $pb.PbList<ChatDeleteMessagesParam> createRepeated() => $pb.PbList<ChatDeleteMessagesParam>();
  @$core.pragma('dart2js:noInline')
  static ChatDeleteMessagesParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatDeleteMessagesParam>(create);
  static ChatDeleteMessagesParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class ChatDeleteMessagesResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatDeleteMessagesResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatDeleteMessagesResponse._() : super();
  factory ChatDeleteMessagesResponse() => create();
  factory ChatDeleteMessagesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatDeleteMessagesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatDeleteMessagesResponse clone() => ChatDeleteMessagesResponse()..mergeFromMessage(this);
  ChatDeleteMessagesResponse copyWith(void Function(ChatDeleteMessagesResponse) updates) => super.copyWith((message) => updates(message as ChatDeleteMessagesResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatDeleteMessagesResponse create() => ChatDeleteMessagesResponse._();
  ChatDeleteMessagesResponse createEmptyInstance() => create();
  static $pb.PbList<ChatDeleteMessagesResponse> createRepeated() => $pb.PbList<ChatDeleteMessagesResponse>();
  @$core.pragma('dart2js:noInline')
  static ChatDeleteMessagesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatDeleteMessagesResponse>(create);
  static ChatDeleteMessagesResponse _defaultInstance;
}

class ChatDeleteHistoryParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatDeleteHistoryParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatDeleteHistoryParam._() : super();
  factory ChatDeleteHistoryParam() => create();
  factory ChatDeleteHistoryParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatDeleteHistoryParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatDeleteHistoryParam clone() => ChatDeleteHistoryParam()..mergeFromMessage(this);
  ChatDeleteHistoryParam copyWith(void Function(ChatDeleteHistoryParam) updates) => super.copyWith((message) => updates(message as ChatDeleteHistoryParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatDeleteHistoryParam create() => ChatDeleteHistoryParam._();
  ChatDeleteHistoryParam createEmptyInstance() => create();
  static $pb.PbList<ChatDeleteHistoryParam> createRepeated() => $pb.PbList<ChatDeleteHistoryParam>();
  @$core.pragma('dart2js:noInline')
  static ChatDeleteHistoryParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatDeleteHistoryParam>(create);
  static ChatDeleteHistoryParam _defaultInstance;
}

class ChatDeleteHistoryResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatDeleteHistoryResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatDeleteHistoryResponse._() : super();
  factory ChatDeleteHistoryResponse() => create();
  factory ChatDeleteHistoryResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatDeleteHistoryResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatDeleteHistoryResponse clone() => ChatDeleteHistoryResponse()..mergeFromMessage(this);
  ChatDeleteHistoryResponse copyWith(void Function(ChatDeleteHistoryResponse) updates) => super.copyWith((message) => updates(message as ChatDeleteHistoryResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatDeleteHistoryResponse create() => ChatDeleteHistoryResponse._();
  ChatDeleteHistoryResponse createEmptyInstance() => create();
  static $pb.PbList<ChatDeleteHistoryResponse> createRepeated() => $pb.PbList<ChatDeleteHistoryResponse>();
  @$core.pragma('dart2js:noInline')
  static ChatDeleteHistoryResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatDeleteHistoryResponse>(create);
  static ChatDeleteHistoryResponse _defaultInstance;
}

class ChatAvatarChangeParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatAvatarChangeParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatAvatarChangeParam._() : super();
  factory ChatAvatarChangeParam() => create();
  factory ChatAvatarChangeParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatAvatarChangeParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatAvatarChangeParam clone() => ChatAvatarChangeParam()..mergeFromMessage(this);
  ChatAvatarChangeParam copyWith(void Function(ChatAvatarChangeParam) updates) => super.copyWith((message) => updates(message as ChatAvatarChangeParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatAvatarChangeParam create() => ChatAvatarChangeParam._();
  ChatAvatarChangeParam createEmptyInstance() => create();
  static $pb.PbList<ChatAvatarChangeParam> createRepeated() => $pb.PbList<ChatAvatarChangeParam>();
  @$core.pragma('dart2js:noInline')
  static ChatAvatarChangeParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatAvatarChangeParam>(create);
  static ChatAvatarChangeParam _defaultInstance;
}

class ChatAvatarChangeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatAvatarChangeResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatAvatarChangeResponse._() : super();
  factory ChatAvatarChangeResponse() => create();
  factory ChatAvatarChangeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatAvatarChangeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatAvatarChangeResponse clone() => ChatAvatarChangeResponse()..mergeFromMessage(this);
  ChatAvatarChangeResponse copyWith(void Function(ChatAvatarChangeResponse) updates) => super.copyWith((message) => updates(message as ChatAvatarChangeResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatAvatarChangeResponse create() => ChatAvatarChangeResponse._();
  ChatAvatarChangeResponse createEmptyInstance() => create();
  static $pb.PbList<ChatAvatarChangeResponse> createRepeated() => $pb.PbList<ChatAvatarChangeResponse>();
  @$core.pragma('dart2js:noInline')
  static ChatAvatarChangeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatAvatarChangeResponse>(create);
  static ChatAvatarChangeResponse _defaultInstance;
}

class ChatSendDoingActionParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatSendDoingActionParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatSendDoingActionParam._() : super();
  factory ChatSendDoingActionParam() => create();
  factory ChatSendDoingActionParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatSendDoingActionParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatSendDoingActionParam clone() => ChatSendDoingActionParam()..mergeFromMessage(this);
  ChatSendDoingActionParam copyWith(void Function(ChatSendDoingActionParam) updates) => super.copyWith((message) => updates(message as ChatSendDoingActionParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatSendDoingActionParam create() => ChatSendDoingActionParam._();
  ChatSendDoingActionParam createEmptyInstance() => create();
  static $pb.PbList<ChatSendDoingActionParam> createRepeated() => $pb.PbList<ChatSendDoingActionParam>();
  @$core.pragma('dart2js:noInline')
  static ChatSendDoingActionParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatSendDoingActionParam>(create);
  static ChatSendDoingActionParam _defaultInstance;
}

class ChatSendDoingActionResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatSendDoingActionResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatSendDoingActionResponse._() : super();
  factory ChatSendDoingActionResponse() => create();
  factory ChatSendDoingActionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatSendDoingActionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatSendDoingActionResponse clone() => ChatSendDoingActionResponse()..mergeFromMessage(this);
  ChatSendDoingActionResponse copyWith(void Function(ChatSendDoingActionResponse) updates) => super.copyWith((message) => updates(message as ChatSendDoingActionResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatSendDoingActionResponse create() => ChatSendDoingActionResponse._();
  ChatSendDoingActionResponse createEmptyInstance() => create();
  static $pb.PbList<ChatSendDoingActionResponse> createRepeated() => $pb.PbList<ChatSendDoingActionResponse>();
  @$core.pragma('dart2js:noInline')
  static ChatSendDoingActionResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatSendDoingActionResponse>(create);
  static ChatSendDoingActionResponse _defaultInstance;
}

class ChatReportChatParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatReportChatParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatReportChatParam._() : super();
  factory ChatReportChatParam() => create();
  factory ChatReportChatParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatReportChatParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatReportChatParam clone() => ChatReportChatParam()..mergeFromMessage(this);
  ChatReportChatParam copyWith(void Function(ChatReportChatParam) updates) => super.copyWith((message) => updates(message as ChatReportChatParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatReportChatParam create() => ChatReportChatParam._();
  ChatReportChatParam createEmptyInstance() => create();
  static $pb.PbList<ChatReportChatParam> createRepeated() => $pb.PbList<ChatReportChatParam>();
  @$core.pragma('dart2js:noInline')
  static ChatReportChatParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatReportChatParam>(create);
  static ChatReportChatParam _defaultInstance;
}

class ChatReportChatResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatReportChatResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatReportChatResponse._() : super();
  factory ChatReportChatResponse() => create();
  factory ChatReportChatResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatReportChatResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatReportChatResponse clone() => ChatReportChatResponse()..mergeFromMessage(this);
  ChatReportChatResponse copyWith(void Function(ChatReportChatResponse) updates) => super.copyWith((message) => updates(message as ChatReportChatResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatReportChatResponse create() => ChatReportChatResponse._();
  ChatReportChatResponse createEmptyInstance() => create();
  static $pb.PbList<ChatReportChatResponse> createRepeated() => $pb.PbList<ChatReportChatResponse>();
  @$core.pragma('dart2js:noInline')
  static ChatReportChatResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatReportChatResponse>(create);
  static ChatReportChatResponse _defaultInstance;
}

class ChatGetFullMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatGetFullMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChatGetFullMessageParam._() : super();
  factory ChatGetFullMessageParam() => create();
  factory ChatGetFullMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatGetFullMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatGetFullMessageParam clone() => ChatGetFullMessageParam()..mergeFromMessage(this);
  ChatGetFullMessageParam copyWith(void Function(ChatGetFullMessageParam) updates) => super.copyWith((message) => updates(message as ChatGetFullMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatGetFullMessageParam create() => ChatGetFullMessageParam._();
  ChatGetFullMessageParam createEmptyInstance() => create();
  static $pb.PbList<ChatGetFullMessageParam> createRepeated() => $pb.PbList<ChatGetFullMessageParam>();
  @$core.pragma('dart2js:noInline')
  static ChatGetFullMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatGetFullMessageParam>(create);
  static ChatGetFullMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class ChatGetFullMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatGetFullMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatGetFullMessageResponse._() : super();
  factory ChatGetFullMessageResponse() => create();
  factory ChatGetFullMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatGetFullMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatGetFullMessageResponse clone() => ChatGetFullMessageResponse()..mergeFromMessage(this);
  ChatGetFullMessageResponse copyWith(void Function(ChatGetFullMessageResponse) updates) => super.copyWith((message) => updates(message as ChatGetFullMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatGetFullMessageResponse create() => ChatGetFullMessageResponse._();
  ChatGetFullMessageResponse createEmptyInstance() => create();
  static $pb.PbList<ChatGetFullMessageResponse> createRepeated() => $pb.PbList<ChatGetFullMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static ChatGetFullMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatGetFullMessageResponse>(create);
  static ChatGetFullMessageResponse _defaultInstance;
}

class ChatGetMessagesListParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatGetMessagesListParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatGetMessagesListParam._() : super();
  factory ChatGetMessagesListParam() => create();
  factory ChatGetMessagesListParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatGetMessagesListParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatGetMessagesListParam clone() => ChatGetMessagesListParam()..mergeFromMessage(this);
  ChatGetMessagesListParam copyWith(void Function(ChatGetMessagesListParam) updates) => super.copyWith((message) => updates(message as ChatGetMessagesListParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatGetMessagesListParam create() => ChatGetMessagesListParam._();
  ChatGetMessagesListParam createEmptyInstance() => create();
  static $pb.PbList<ChatGetMessagesListParam> createRepeated() => $pb.PbList<ChatGetMessagesListParam>();
  @$core.pragma('dart2js:noInline')
  static ChatGetMessagesListParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatGetMessagesListParam>(create);
  static ChatGetMessagesListParam _defaultInstance;
}

class ChatGetMessagesListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatGetMessagesListResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatGetMessagesListResponse._() : super();
  factory ChatGetMessagesListResponse() => create();
  factory ChatGetMessagesListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatGetMessagesListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatGetMessagesListResponse clone() => ChatGetMessagesListResponse()..mergeFromMessage(this);
  ChatGetMessagesListResponse copyWith(void Function(ChatGetMessagesListResponse) updates) => super.copyWith((message) => updates(message as ChatGetMessagesListResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatGetMessagesListResponse create() => ChatGetMessagesListResponse._();
  ChatGetMessagesListResponse createEmptyInstance() => create();
  static $pb.PbList<ChatGetMessagesListResponse> createRepeated() => $pb.PbList<ChatGetMessagesListResponse>();
  @$core.pragma('dart2js:noInline')
  static ChatGetMessagesListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatGetMessagesListResponse>(create);
  static ChatGetMessagesListResponse _defaultInstance;
}

class ChatGetMediaListParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatGetMediaListParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatGetMediaListParam._() : super();
  factory ChatGetMediaListParam() => create();
  factory ChatGetMediaListParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatGetMediaListParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatGetMediaListParam clone() => ChatGetMediaListParam()..mergeFromMessage(this);
  ChatGetMediaListParam copyWith(void Function(ChatGetMediaListParam) updates) => super.copyWith((message) => updates(message as ChatGetMediaListParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatGetMediaListParam create() => ChatGetMediaListParam._();
  ChatGetMediaListParam createEmptyInstance() => create();
  static $pb.PbList<ChatGetMediaListParam> createRepeated() => $pb.PbList<ChatGetMediaListParam>();
  @$core.pragma('dart2js:noInline')
  static ChatGetMediaListParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatGetMediaListParam>(create);
  static ChatGetMediaListParam _defaultInstance;
}

class ChatGetMediaListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatGetMediaListResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChatGetMediaListResponse._() : super();
  factory ChatGetMediaListResponse() => create();
  factory ChatGetMediaListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatGetMediaListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatGetMediaListResponse clone() => ChatGetMediaListResponse()..mergeFromMessage(this);
  ChatGetMediaListResponse copyWith(void Function(ChatGetMediaListResponse) updates) => super.copyWith((message) => updates(message as ChatGetMediaListResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatGetMediaListResponse create() => ChatGetMediaListResponse._();
  ChatGetMediaListResponse createEmptyInstance() => create();
  static $pb.PbList<ChatGetMediaListResponse> createRepeated() => $pb.PbList<ChatGetMediaListResponse>();
  @$core.pragma('dart2js:noInline')
  static ChatGetMediaListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatGetMediaListResponse>(create);
  static ChatGetMediaListResponse _defaultInstance;
}

class RPC_ChatApi {
  $pb.RpcClient _client;
  RPC_ChatApi(this._client);

  $async.Future<ChatSendMessageResponse> chatSendMessage($pb.ClientContext ctx, ChatSendMessageParam request) {
    var emptyResponse = ChatSendMessageResponse();
    return _client.invoke<ChatSendMessageResponse>(ctx, 'RPC_Chat', 'ChatSendMessage', request, emptyResponse);
  }
  $async.Future<ChatEditMessageResponse> chatEditMessage($pb.ClientContext ctx, ChatEditMessageParam request) {
    var emptyResponse = ChatEditMessageResponse();
    return _client.invoke<ChatEditMessageResponse>(ctx, 'RPC_Chat', 'ChatEditMessage', request, emptyResponse);
  }
  $async.Future<ChatDeleteMessagesResponse> chatDeleteMessages($pb.ClientContext ctx, ChatDeleteMessagesParam request) {
    var emptyResponse = ChatDeleteMessagesResponse();
    return _client.invoke<ChatDeleteMessagesResponse>(ctx, 'RPC_Chat', 'ChatDeleteMessages', request, emptyResponse);
  }
  $async.Future<ChatDeleteHistoryResponse> chatDeleteHistory($pb.ClientContext ctx, ChatDeleteHistoryParam request) {
    var emptyResponse = ChatDeleteHistoryResponse();
    return _client.invoke<ChatDeleteHistoryResponse>(ctx, 'RPC_Chat', 'ChatDeleteHistory', request, emptyResponse);
  }
  $async.Future<ChatSendDoingActionResponse> chatSendDoingAction($pb.ClientContext ctx, ChatSendDoingActionParam request) {
    var emptyResponse = ChatSendDoingActionResponse();
    return _client.invoke<ChatSendDoingActionResponse>(ctx, 'RPC_Chat', 'ChatSendDoingAction', request, emptyResponse);
  }
  $async.Future<ChatReportChatResponse> chatReportChat($pb.ClientContext ctx, ChatReportChatParam request) {
    var emptyResponse = ChatReportChatResponse();
    return _client.invoke<ChatReportChatResponse>(ctx, 'RPC_Chat', 'ChatReportChat', request, emptyResponse);
  }
  $async.Future<ChatGetFullMessageResponse> chatGetFull($pb.ClientContext ctx, ChatGetFullMessageParam request) {
    var emptyResponse = ChatGetFullMessageResponse();
    return _client.invoke<ChatGetFullMessageResponse>(ctx, 'RPC_Chat', 'ChatGetFull', request, emptyResponse);
  }
  $async.Future<ChatGetMessagesListResponse> chatGetMessagesList($pb.ClientContext ctx, ChatGetMessagesListParam request) {
    var emptyResponse = ChatGetMessagesListResponse();
    return _client.invoke<ChatGetMessagesListResponse>(ctx, 'RPC_Chat', 'ChatGetMessagesList', request, emptyResponse);
  }
  $async.Future<ChatGetMediaListResponse> chatGetMediaList($pb.ClientContext ctx, ChatGetMediaListParam request) {
    var emptyResponse = ChatGetMediaListResponse();
    return _client.invoke<ChatGetMediaListResponse>(ctx, 'RPC_Chat', 'ChatGetMediaList', request, emptyResponse);
  }
}

