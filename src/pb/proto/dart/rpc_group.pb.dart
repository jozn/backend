///
//  Generated code. Do not modify.
//  source: rpc_group.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class GroupAvatarAddParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupAvatarAddParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupAvatarAddParam._() : super();
  factory GroupAvatarAddParam() => create();
  factory GroupAvatarAddParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupAvatarAddParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupAvatarAddParam clone() => GroupAvatarAddParam()..mergeFromMessage(this);
  GroupAvatarAddParam copyWith(void Function(GroupAvatarAddParam) updates) => super.copyWith((message) => updates(message as GroupAvatarAddParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupAvatarAddParam create() => GroupAvatarAddParam._();
  GroupAvatarAddParam createEmptyInstance() => create();
  static $pb.PbList<GroupAvatarAddParam> createRepeated() => $pb.PbList<GroupAvatarAddParam>();
  @$core.pragma('dart2js:noInline')
  static GroupAvatarAddParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupAvatarAddParam>(create);
  static GroupAvatarAddParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupAvatarAddResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupAvatarAddResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupAvatarAddResponse._() : super();
  factory GroupAvatarAddResponse() => create();
  factory GroupAvatarAddResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupAvatarAddResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupAvatarAddResponse clone() => GroupAvatarAddResponse()..mergeFromMessage(this);
  GroupAvatarAddResponse copyWith(void Function(GroupAvatarAddResponse) updates) => super.copyWith((message) => updates(message as GroupAvatarAddResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupAvatarAddResponse create() => GroupAvatarAddResponse._();
  GroupAvatarAddResponse createEmptyInstance() => create();
  static $pb.PbList<GroupAvatarAddResponse> createRepeated() => $pb.PbList<GroupAvatarAddResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupAvatarAddResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupAvatarAddResponse>(create);
  static GroupAvatarAddResponse _defaultInstance;
}

class GroupAvatarDeleteParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupAvatarDeleteParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupAvatarDeleteParam._() : super();
  factory GroupAvatarDeleteParam() => create();
  factory GroupAvatarDeleteParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupAvatarDeleteParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupAvatarDeleteParam clone() => GroupAvatarDeleteParam()..mergeFromMessage(this);
  GroupAvatarDeleteParam copyWith(void Function(GroupAvatarDeleteParam) updates) => super.copyWith((message) => updates(message as GroupAvatarDeleteParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupAvatarDeleteParam create() => GroupAvatarDeleteParam._();
  GroupAvatarDeleteParam createEmptyInstance() => create();
  static $pb.PbList<GroupAvatarDeleteParam> createRepeated() => $pb.PbList<GroupAvatarDeleteParam>();
  @$core.pragma('dart2js:noInline')
  static GroupAvatarDeleteParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupAvatarDeleteParam>(create);
  static GroupAvatarDeleteParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupAvatarDeleteResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupAvatarDeleteResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupAvatarDeleteResponse._() : super();
  factory GroupAvatarDeleteResponse() => create();
  factory GroupAvatarDeleteResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupAvatarDeleteResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupAvatarDeleteResponse clone() => GroupAvatarDeleteResponse()..mergeFromMessage(this);
  GroupAvatarDeleteResponse copyWith(void Function(GroupAvatarDeleteResponse) updates) => super.copyWith((message) => updates(message as GroupAvatarDeleteResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupAvatarDeleteResponse create() => GroupAvatarDeleteResponse._();
  GroupAvatarDeleteResponse createEmptyInstance() => create();
  static $pb.PbList<GroupAvatarDeleteResponse> createRepeated() => $pb.PbList<GroupAvatarDeleteResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupAvatarDeleteResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupAvatarDeleteResponse>(create);
  static GroupAvatarDeleteResponse _defaultInstance;
}

class GroupAvatarGetListParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupAvatarGetListParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupAvatarGetListParam._() : super();
  factory GroupAvatarGetListParam() => create();
  factory GroupAvatarGetListParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupAvatarGetListParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupAvatarGetListParam clone() => GroupAvatarGetListParam()..mergeFromMessage(this);
  GroupAvatarGetListParam copyWith(void Function(GroupAvatarGetListParam) updates) => super.copyWith((message) => updates(message as GroupAvatarGetListParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupAvatarGetListParam create() => GroupAvatarGetListParam._();
  GroupAvatarGetListParam createEmptyInstance() => create();
  static $pb.PbList<GroupAvatarGetListParam> createRepeated() => $pb.PbList<GroupAvatarGetListParam>();
  @$core.pragma('dart2js:noInline')
  static GroupAvatarGetListParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupAvatarGetListParam>(create);
  static GroupAvatarGetListParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupAvatarGetListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupAvatarGetListResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupAvatarGetListResponse._() : super();
  factory GroupAvatarGetListResponse() => create();
  factory GroupAvatarGetListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupAvatarGetListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupAvatarGetListResponse clone() => GroupAvatarGetListResponse()..mergeFromMessage(this);
  GroupAvatarGetListResponse copyWith(void Function(GroupAvatarGetListResponse) updates) => super.copyWith((message) => updates(message as GroupAvatarGetListResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupAvatarGetListResponse create() => GroupAvatarGetListResponse._();
  GroupAvatarGetListResponse createEmptyInstance() => create();
  static $pb.PbList<GroupAvatarGetListResponse> createRepeated() => $pb.PbList<GroupAvatarGetListResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupAvatarGetListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupAvatarGetListResponse>(create);
  static GroupAvatarGetListResponse _defaultInstance;
}

class GroupSendMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupSendMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupSendMessageParam._() : super();
  factory GroupSendMessageParam() => create();
  factory GroupSendMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupSendMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupSendMessageParam clone() => GroupSendMessageParam()..mergeFromMessage(this);
  GroupSendMessageParam copyWith(void Function(GroupSendMessageParam) updates) => super.copyWith((message) => updates(message as GroupSendMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupSendMessageParam create() => GroupSendMessageParam._();
  GroupSendMessageParam createEmptyInstance() => create();
  static $pb.PbList<GroupSendMessageParam> createRepeated() => $pb.PbList<GroupSendMessageParam>();
  @$core.pragma('dart2js:noInline')
  static GroupSendMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupSendMessageParam>(create);
  static GroupSendMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupSendMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupSendMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupSendMessageResponse._() : super();
  factory GroupSendMessageResponse() => create();
  factory GroupSendMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupSendMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupSendMessageResponse clone() => GroupSendMessageResponse()..mergeFromMessage(this);
  GroupSendMessageResponse copyWith(void Function(GroupSendMessageResponse) updates) => super.copyWith((message) => updates(message as GroupSendMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupSendMessageResponse create() => GroupSendMessageResponse._();
  GroupSendMessageResponse createEmptyInstance() => create();
  static $pb.PbList<GroupSendMessageResponse> createRepeated() => $pb.PbList<GroupSendMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupSendMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupSendMessageResponse>(create);
  static GroupSendMessageResponse _defaultInstance;
}

class GroupEditMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupEditMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupEditMessageParam._() : super();
  factory GroupEditMessageParam() => create();
  factory GroupEditMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupEditMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupEditMessageParam clone() => GroupEditMessageParam()..mergeFromMessage(this);
  GroupEditMessageParam copyWith(void Function(GroupEditMessageParam) updates) => super.copyWith((message) => updates(message as GroupEditMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupEditMessageParam create() => GroupEditMessageParam._();
  GroupEditMessageParam createEmptyInstance() => create();
  static $pb.PbList<GroupEditMessageParam> createRepeated() => $pb.PbList<GroupEditMessageParam>();
  @$core.pragma('dart2js:noInline')
  static GroupEditMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupEditMessageParam>(create);
  static GroupEditMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupEditMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupEditMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupEditMessageResponse._() : super();
  factory GroupEditMessageResponse() => create();
  factory GroupEditMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupEditMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupEditMessageResponse clone() => GroupEditMessageResponse()..mergeFromMessage(this);
  GroupEditMessageResponse copyWith(void Function(GroupEditMessageResponse) updates) => super.copyWith((message) => updates(message as GroupEditMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupEditMessageResponse create() => GroupEditMessageResponse._();
  GroupEditMessageResponse createEmptyInstance() => create();
  static $pb.PbList<GroupEditMessageResponse> createRepeated() => $pb.PbList<GroupEditMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupEditMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupEditMessageResponse>(create);
  static GroupEditMessageResponse _defaultInstance;
}

class GroupPinMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupPinMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupPinMessageParam._() : super();
  factory GroupPinMessageParam() => create();
  factory GroupPinMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupPinMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupPinMessageParam clone() => GroupPinMessageParam()..mergeFromMessage(this);
  GroupPinMessageParam copyWith(void Function(GroupPinMessageParam) updates) => super.copyWith((message) => updates(message as GroupPinMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupPinMessageParam create() => GroupPinMessageParam._();
  GroupPinMessageParam createEmptyInstance() => create();
  static $pb.PbList<GroupPinMessageParam> createRepeated() => $pb.PbList<GroupPinMessageParam>();
  @$core.pragma('dart2js:noInline')
  static GroupPinMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupPinMessageParam>(create);
  static GroupPinMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupPinMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupPinMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupPinMessageResponse._() : super();
  factory GroupPinMessageResponse() => create();
  factory GroupPinMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupPinMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupPinMessageResponse clone() => GroupPinMessageResponse()..mergeFromMessage(this);
  GroupPinMessageResponse copyWith(void Function(GroupPinMessageResponse) updates) => super.copyWith((message) => updates(message as GroupPinMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupPinMessageResponse create() => GroupPinMessageResponse._();
  GroupPinMessageResponse createEmptyInstance() => create();
  static $pb.PbList<GroupPinMessageResponse> createRepeated() => $pb.PbList<GroupPinMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupPinMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupPinMessageResponse>(create);
  static GroupPinMessageResponse _defaultInstance;
}

class GroupUnPinMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupUnPinMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupUnPinMessageParam._() : super();
  factory GroupUnPinMessageParam() => create();
  factory GroupUnPinMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupUnPinMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupUnPinMessageParam clone() => GroupUnPinMessageParam()..mergeFromMessage(this);
  GroupUnPinMessageParam copyWith(void Function(GroupUnPinMessageParam) updates) => super.copyWith((message) => updates(message as GroupUnPinMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupUnPinMessageParam create() => GroupUnPinMessageParam._();
  GroupUnPinMessageParam createEmptyInstance() => create();
  static $pb.PbList<GroupUnPinMessageParam> createRepeated() => $pb.PbList<GroupUnPinMessageParam>();
  @$core.pragma('dart2js:noInline')
  static GroupUnPinMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupUnPinMessageParam>(create);
  static GroupUnPinMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupUnPinMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupUnPinMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupUnPinMessageResponse._() : super();
  factory GroupUnPinMessageResponse() => create();
  factory GroupUnPinMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupUnPinMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupUnPinMessageResponse clone() => GroupUnPinMessageResponse()..mergeFromMessage(this);
  GroupUnPinMessageResponse copyWith(void Function(GroupUnPinMessageResponse) updates) => super.copyWith((message) => updates(message as GroupUnPinMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupUnPinMessageResponse create() => GroupUnPinMessageResponse._();
  GroupUnPinMessageResponse createEmptyInstance() => create();
  static $pb.PbList<GroupUnPinMessageResponse> createRepeated() => $pb.PbList<GroupUnPinMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupUnPinMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupUnPinMessageResponse>(create);
  static GroupUnPinMessageResponse _defaultInstance;
}

class GroupDeleteMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupDeleteMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupDeleteMessageParam._() : super();
  factory GroupDeleteMessageParam() => create();
  factory GroupDeleteMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupDeleteMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupDeleteMessageParam clone() => GroupDeleteMessageParam()..mergeFromMessage(this);
  GroupDeleteMessageParam copyWith(void Function(GroupDeleteMessageParam) updates) => super.copyWith((message) => updates(message as GroupDeleteMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupDeleteMessageParam create() => GroupDeleteMessageParam._();
  GroupDeleteMessageParam createEmptyInstance() => create();
  static $pb.PbList<GroupDeleteMessageParam> createRepeated() => $pb.PbList<GroupDeleteMessageParam>();
  @$core.pragma('dart2js:noInline')
  static GroupDeleteMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupDeleteMessageParam>(create);
  static GroupDeleteMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupDeleteMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupDeleteMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupDeleteMessageResponse._() : super();
  factory GroupDeleteMessageResponse() => create();
  factory GroupDeleteMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupDeleteMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupDeleteMessageResponse clone() => GroupDeleteMessageResponse()..mergeFromMessage(this);
  GroupDeleteMessageResponse copyWith(void Function(GroupDeleteMessageResponse) updates) => super.copyWith((message) => updates(message as GroupDeleteMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupDeleteMessageResponse create() => GroupDeleteMessageResponse._();
  GroupDeleteMessageResponse createEmptyInstance() => create();
  static $pb.PbList<GroupDeleteMessageResponse> createRepeated() => $pb.PbList<GroupDeleteMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupDeleteMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupDeleteMessageResponse>(create);
  static GroupDeleteMessageResponse _defaultInstance;
}

class GroupSetDraftParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupSetDraftParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupSetDraftParam._() : super();
  factory GroupSetDraftParam() => create();
  factory GroupSetDraftParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupSetDraftParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupSetDraftParam clone() => GroupSetDraftParam()..mergeFromMessage(this);
  GroupSetDraftParam copyWith(void Function(GroupSetDraftParam) updates) => super.copyWith((message) => updates(message as GroupSetDraftParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupSetDraftParam create() => GroupSetDraftParam._();
  GroupSetDraftParam createEmptyInstance() => create();
  static $pb.PbList<GroupSetDraftParam> createRepeated() => $pb.PbList<GroupSetDraftParam>();
  @$core.pragma('dart2js:noInline')
  static GroupSetDraftParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupSetDraftParam>(create);
  static GroupSetDraftParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupSetDraftResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupSetDraftResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupSetDraftResponse._() : super();
  factory GroupSetDraftResponse() => create();
  factory GroupSetDraftResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupSetDraftResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupSetDraftResponse clone() => GroupSetDraftResponse()..mergeFromMessage(this);
  GroupSetDraftResponse copyWith(void Function(GroupSetDraftResponse) updates) => super.copyWith((message) => updates(message as GroupSetDraftResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupSetDraftResponse create() => GroupSetDraftResponse._();
  GroupSetDraftResponse createEmptyInstance() => create();
  static $pb.PbList<GroupSetDraftResponse> createRepeated() => $pb.PbList<GroupSetDraftResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupSetDraftResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupSetDraftResponse>(create);
  static GroupSetDraftResponse _defaultInstance;
}

class GroupGetFullMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupGetFullMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupGetFullMessageParam._() : super();
  factory GroupGetFullMessageParam() => create();
  factory GroupGetFullMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupGetFullMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupGetFullMessageParam clone() => GroupGetFullMessageParam()..mergeFromMessage(this);
  GroupGetFullMessageParam copyWith(void Function(GroupGetFullMessageParam) updates) => super.copyWith((message) => updates(message as GroupGetFullMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupGetFullMessageParam create() => GroupGetFullMessageParam._();
  GroupGetFullMessageParam createEmptyInstance() => create();
  static $pb.PbList<GroupGetFullMessageParam> createRepeated() => $pb.PbList<GroupGetFullMessageParam>();
  @$core.pragma('dart2js:noInline')
  static GroupGetFullMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupGetFullMessageParam>(create);
  static GroupGetFullMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupGetFullMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupGetFullMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupGetFullMessageResponse._() : super();
  factory GroupGetFullMessageResponse() => create();
  factory GroupGetFullMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupGetFullMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupGetFullMessageResponse clone() => GroupGetFullMessageResponse()..mergeFromMessage(this);
  GroupGetFullMessageResponse copyWith(void Function(GroupGetFullMessageResponse) updates) => super.copyWith((message) => updates(message as GroupGetFullMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupGetFullMessageResponse create() => GroupGetFullMessageResponse._();
  GroupGetFullMessageResponse createEmptyInstance() => create();
  static $pb.PbList<GroupGetFullMessageResponse> createRepeated() => $pb.PbList<GroupGetFullMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupGetFullMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupGetFullMessageResponse>(create);
  static GroupGetFullMessageResponse _defaultInstance;
}

class GroupCreateGroupParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupCreateGroupParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupCreateGroupParam._() : super();
  factory GroupCreateGroupParam() => create();
  factory GroupCreateGroupParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupCreateGroupParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupCreateGroupParam clone() => GroupCreateGroupParam()..mergeFromMessage(this);
  GroupCreateGroupParam copyWith(void Function(GroupCreateGroupParam) updates) => super.copyWith((message) => updates(message as GroupCreateGroupParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupCreateGroupParam create() => GroupCreateGroupParam._();
  GroupCreateGroupParam createEmptyInstance() => create();
  static $pb.PbList<GroupCreateGroupParam> createRepeated() => $pb.PbList<GroupCreateGroupParam>();
  @$core.pragma('dart2js:noInline')
  static GroupCreateGroupParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupCreateGroupParam>(create);
  static GroupCreateGroupParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupCreateGroupResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupCreateGroupResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupCreateGroupResponse._() : super();
  factory GroupCreateGroupResponse() => create();
  factory GroupCreateGroupResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupCreateGroupResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupCreateGroupResponse clone() => GroupCreateGroupResponse()..mergeFromMessage(this);
  GroupCreateGroupResponse copyWith(void Function(GroupCreateGroupResponse) updates) => super.copyWith((message) => updates(message as GroupCreateGroupResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupCreateGroupResponse create() => GroupCreateGroupResponse._();
  GroupCreateGroupResponse createEmptyInstance() => create();
  static $pb.PbList<GroupCreateGroupResponse> createRepeated() => $pb.PbList<GroupCreateGroupResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupCreateGroupResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupCreateGroupResponse>(create);
  static GroupCreateGroupResponse _defaultInstance;
}

class GroupEditGroupParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupEditGroupParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupEditGroupParam._() : super();
  factory GroupEditGroupParam() => create();
  factory GroupEditGroupParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupEditGroupParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupEditGroupParam clone() => GroupEditGroupParam()..mergeFromMessage(this);
  GroupEditGroupParam copyWith(void Function(GroupEditGroupParam) updates) => super.copyWith((message) => updates(message as GroupEditGroupParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupEditGroupParam create() => GroupEditGroupParam._();
  GroupEditGroupParam createEmptyInstance() => create();
  static $pb.PbList<GroupEditGroupParam> createRepeated() => $pb.PbList<GroupEditGroupParam>();
  @$core.pragma('dart2js:noInline')
  static GroupEditGroupParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupEditGroupParam>(create);
  static GroupEditGroupParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupEditGroupResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupEditGroupResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupEditGroupResponse._() : super();
  factory GroupEditGroupResponse() => create();
  factory GroupEditGroupResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupEditGroupResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupEditGroupResponse clone() => GroupEditGroupResponse()..mergeFromMessage(this);
  GroupEditGroupResponse copyWith(void Function(GroupEditGroupResponse) updates) => super.copyWith((message) => updates(message as GroupEditGroupResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupEditGroupResponse create() => GroupEditGroupResponse._();
  GroupEditGroupResponse createEmptyInstance() => create();
  static $pb.PbList<GroupEditGroupResponse> createRepeated() => $pb.PbList<GroupEditGroupResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupEditGroupResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupEditGroupResponse>(create);
  static GroupEditGroupResponse _defaultInstance;
}

class GroupDeleteGroupParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupDeleteGroupParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupDeleteGroupParam._() : super();
  factory GroupDeleteGroupParam() => create();
  factory GroupDeleteGroupParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupDeleteGroupParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupDeleteGroupParam clone() => GroupDeleteGroupParam()..mergeFromMessage(this);
  GroupDeleteGroupParam copyWith(void Function(GroupDeleteGroupParam) updates) => super.copyWith((message) => updates(message as GroupDeleteGroupParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupDeleteGroupParam create() => GroupDeleteGroupParam._();
  GroupDeleteGroupParam createEmptyInstance() => create();
  static $pb.PbList<GroupDeleteGroupParam> createRepeated() => $pb.PbList<GroupDeleteGroupParam>();
  @$core.pragma('dart2js:noInline')
  static GroupDeleteGroupParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupDeleteGroupParam>(create);
  static GroupDeleteGroupParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupDeleteGroupResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupDeleteGroupResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupDeleteGroupResponse._() : super();
  factory GroupDeleteGroupResponse() => create();
  factory GroupDeleteGroupResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupDeleteGroupResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupDeleteGroupResponse clone() => GroupDeleteGroupResponse()..mergeFromMessage(this);
  GroupDeleteGroupResponse copyWith(void Function(GroupDeleteGroupResponse) updates) => super.copyWith((message) => updates(message as GroupDeleteGroupResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupDeleteGroupResponse create() => GroupDeleteGroupResponse._();
  GroupDeleteGroupResponse createEmptyInstance() => create();
  static $pb.PbList<GroupDeleteGroupResponse> createRepeated() => $pb.PbList<GroupDeleteGroupResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupDeleteGroupResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupDeleteGroupResponse>(create);
  static GroupDeleteGroupResponse _defaultInstance;
}

class GroupAddAdminParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupAddAdminParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupAddAdminParam._() : super();
  factory GroupAddAdminParam() => create();
  factory GroupAddAdminParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupAddAdminParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupAddAdminParam clone() => GroupAddAdminParam()..mergeFromMessage(this);
  GroupAddAdminParam copyWith(void Function(GroupAddAdminParam) updates) => super.copyWith((message) => updates(message as GroupAddAdminParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupAddAdminParam create() => GroupAddAdminParam._();
  GroupAddAdminParam createEmptyInstance() => create();
  static $pb.PbList<GroupAddAdminParam> createRepeated() => $pb.PbList<GroupAddAdminParam>();
  @$core.pragma('dart2js:noInline')
  static GroupAddAdminParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupAddAdminParam>(create);
  static GroupAddAdminParam _defaultInstance;
}

class GroupAddAdminResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupAddAdminResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupAddAdminResponse._() : super();
  factory GroupAddAdminResponse() => create();
  factory GroupAddAdminResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupAddAdminResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupAddAdminResponse clone() => GroupAddAdminResponse()..mergeFromMessage(this);
  GroupAddAdminResponse copyWith(void Function(GroupAddAdminResponse) updates) => super.copyWith((message) => updates(message as GroupAddAdminResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupAddAdminResponse create() => GroupAddAdminResponse._();
  GroupAddAdminResponse createEmptyInstance() => create();
  static $pb.PbList<GroupAddAdminResponse> createRepeated() => $pb.PbList<GroupAddAdminResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupAddAdminResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupAddAdminResponse>(create);
  static GroupAddAdminResponse _defaultInstance;
}

class GroupAddMemberParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupAddMemberParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupAddMemberParam._() : super();
  factory GroupAddMemberParam() => create();
  factory GroupAddMemberParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupAddMemberParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupAddMemberParam clone() => GroupAddMemberParam()..mergeFromMessage(this);
  GroupAddMemberParam copyWith(void Function(GroupAddMemberParam) updates) => super.copyWith((message) => updates(message as GroupAddMemberParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupAddMemberParam create() => GroupAddMemberParam._();
  GroupAddMemberParam createEmptyInstance() => create();
  static $pb.PbList<GroupAddMemberParam> createRepeated() => $pb.PbList<GroupAddMemberParam>();
  @$core.pragma('dart2js:noInline')
  static GroupAddMemberParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupAddMemberParam>(create);
  static GroupAddMemberParam _defaultInstance;
}

class GroupAddMemberResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupAddMemberResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupAddMemberResponse._() : super();
  factory GroupAddMemberResponse() => create();
  factory GroupAddMemberResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupAddMemberResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupAddMemberResponse clone() => GroupAddMemberResponse()..mergeFromMessage(this);
  GroupAddMemberResponse copyWith(void Function(GroupAddMemberResponse) updates) => super.copyWith((message) => updates(message as GroupAddMemberResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupAddMemberResponse create() => GroupAddMemberResponse._();
  GroupAddMemberResponse createEmptyInstance() => create();
  static $pb.PbList<GroupAddMemberResponse> createRepeated() => $pb.PbList<GroupAddMemberResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupAddMemberResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupAddMemberResponse>(create);
  static GroupAddMemberResponse _defaultInstance;
}

class GroupRemoveMemberParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupRemoveMemberParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupRemoveMemberParam._() : super();
  factory GroupRemoveMemberParam() => create();
  factory GroupRemoveMemberParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupRemoveMemberParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupRemoveMemberParam clone() => GroupRemoveMemberParam()..mergeFromMessage(this);
  GroupRemoveMemberParam copyWith(void Function(GroupRemoveMemberParam) updates) => super.copyWith((message) => updates(message as GroupRemoveMemberParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupRemoveMemberParam create() => GroupRemoveMemberParam._();
  GroupRemoveMemberParam createEmptyInstance() => create();
  static $pb.PbList<GroupRemoveMemberParam> createRepeated() => $pb.PbList<GroupRemoveMemberParam>();
  @$core.pragma('dart2js:noInline')
  static GroupRemoveMemberParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupRemoveMemberParam>(create);
  static GroupRemoveMemberParam _defaultInstance;
}

class GroupRemoveMemberResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupRemoveMemberResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupRemoveMemberResponse._() : super();
  factory GroupRemoveMemberResponse() => create();
  factory GroupRemoveMemberResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupRemoveMemberResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupRemoveMemberResponse clone() => GroupRemoveMemberResponse()..mergeFromMessage(this);
  GroupRemoveMemberResponse copyWith(void Function(GroupRemoveMemberResponse) updates) => super.copyWith((message) => updates(message as GroupRemoveMemberResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupRemoveMemberResponse create() => GroupRemoveMemberResponse._();
  GroupRemoveMemberResponse createEmptyInstance() => create();
  static $pb.PbList<GroupRemoveMemberResponse> createRepeated() => $pb.PbList<GroupRemoveMemberResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupRemoveMemberResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupRemoveMemberResponse>(create);
  static GroupRemoveMemberResponse _defaultInstance;
}

class GroupChangeMemberLevelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupChangeMemberLevelParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupChangeMemberLevelParam._() : super();
  factory GroupChangeMemberLevelParam() => create();
  factory GroupChangeMemberLevelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupChangeMemberLevelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupChangeMemberLevelParam clone() => GroupChangeMemberLevelParam()..mergeFromMessage(this);
  GroupChangeMemberLevelParam copyWith(void Function(GroupChangeMemberLevelParam) updates) => super.copyWith((message) => updates(message as GroupChangeMemberLevelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupChangeMemberLevelParam create() => GroupChangeMemberLevelParam._();
  GroupChangeMemberLevelParam createEmptyInstance() => create();
  static $pb.PbList<GroupChangeMemberLevelParam> createRepeated() => $pb.PbList<GroupChangeMemberLevelParam>();
  @$core.pragma('dart2js:noInline')
  static GroupChangeMemberLevelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupChangeMemberLevelParam>(create);
  static GroupChangeMemberLevelParam _defaultInstance;
}

class GroupChangeMemberLevelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupChangeMemberLevelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupChangeMemberLevelResponse._() : super();
  factory GroupChangeMemberLevelResponse() => create();
  factory GroupChangeMemberLevelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupChangeMemberLevelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupChangeMemberLevelResponse clone() => GroupChangeMemberLevelResponse()..mergeFromMessage(this);
  GroupChangeMemberLevelResponse copyWith(void Function(GroupChangeMemberLevelResponse) updates) => super.copyWith((message) => updates(message as GroupChangeMemberLevelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupChangeMemberLevelResponse create() => GroupChangeMemberLevelResponse._();
  GroupChangeMemberLevelResponse createEmptyInstance() => create();
  static $pb.PbList<GroupChangeMemberLevelResponse> createRepeated() => $pb.PbList<GroupChangeMemberLevelResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupChangeMemberLevelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupChangeMemberLevelResponse>(create);
  static GroupChangeMemberLevelResponse _defaultInstance;
}

class GroupChangeMemberPermissionParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupChangeMemberPermissionParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupChangeMemberPermissionParam._() : super();
  factory GroupChangeMemberPermissionParam() => create();
  factory GroupChangeMemberPermissionParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupChangeMemberPermissionParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupChangeMemberPermissionParam clone() => GroupChangeMemberPermissionParam()..mergeFromMessage(this);
  GroupChangeMemberPermissionParam copyWith(void Function(GroupChangeMemberPermissionParam) updates) => super.copyWith((message) => updates(message as GroupChangeMemberPermissionParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupChangeMemberPermissionParam create() => GroupChangeMemberPermissionParam._();
  GroupChangeMemberPermissionParam createEmptyInstance() => create();
  static $pb.PbList<GroupChangeMemberPermissionParam> createRepeated() => $pb.PbList<GroupChangeMemberPermissionParam>();
  @$core.pragma('dart2js:noInline')
  static GroupChangeMemberPermissionParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupChangeMemberPermissionParam>(create);
  static GroupChangeMemberPermissionParam _defaultInstance;
}

class GroupChangeMemberPermissionResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupChangeMemberPermissionResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupChangeMemberPermissionResponse._() : super();
  factory GroupChangeMemberPermissionResponse() => create();
  factory GroupChangeMemberPermissionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupChangeMemberPermissionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupChangeMemberPermissionResponse clone() => GroupChangeMemberPermissionResponse()..mergeFromMessage(this);
  GroupChangeMemberPermissionResponse copyWith(void Function(GroupChangeMemberPermissionResponse) updates) => super.copyWith((message) => updates(message as GroupChangeMemberPermissionResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupChangeMemberPermissionResponse create() => GroupChangeMemberPermissionResponse._();
  GroupChangeMemberPermissionResponse createEmptyInstance() => create();
  static $pb.PbList<GroupChangeMemberPermissionResponse> createRepeated() => $pb.PbList<GroupChangeMemberPermissionResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupChangeMemberPermissionResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupChangeMemberPermissionResponse>(create);
  static GroupChangeMemberPermissionResponse _defaultInstance;
}

class JoinGroupParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('JoinGroupParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  JoinGroupParam._() : super();
  factory JoinGroupParam() => create();
  factory JoinGroupParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory JoinGroupParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  JoinGroupParam clone() => JoinGroupParam()..mergeFromMessage(this);
  JoinGroupParam copyWith(void Function(JoinGroupParam) updates) => super.copyWith((message) => updates(message as JoinGroupParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static JoinGroupParam create() => JoinGroupParam._();
  JoinGroupParam createEmptyInstance() => create();
  static $pb.PbList<JoinGroupParam> createRepeated() => $pb.PbList<JoinGroupParam>();
  @$core.pragma('dart2js:noInline')
  static JoinGroupParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<JoinGroupParam>(create);
  static JoinGroupParam _defaultInstance;
}

class JoinGroupResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('JoinGroupResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  JoinGroupResponse._() : super();
  factory JoinGroupResponse() => create();
  factory JoinGroupResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory JoinGroupResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  JoinGroupResponse clone() => JoinGroupResponse()..mergeFromMessage(this);
  JoinGroupResponse copyWith(void Function(JoinGroupResponse) updates) => super.copyWith((message) => updates(message as JoinGroupResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static JoinGroupResponse create() => JoinGroupResponse._();
  JoinGroupResponse createEmptyInstance() => create();
  static $pb.PbList<JoinGroupResponse> createRepeated() => $pb.PbList<JoinGroupResponse>();
  @$core.pragma('dart2js:noInline')
  static JoinGroupResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<JoinGroupResponse>(create);
  static JoinGroupResponse _defaultInstance;
}

class GroupLeaveGroupParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupLeaveGroupParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupLeaveGroupParam._() : super();
  factory GroupLeaveGroupParam() => create();
  factory GroupLeaveGroupParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupLeaveGroupParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupLeaveGroupParam clone() => GroupLeaveGroupParam()..mergeFromMessage(this);
  GroupLeaveGroupParam copyWith(void Function(GroupLeaveGroupParam) updates) => super.copyWith((message) => updates(message as GroupLeaveGroupParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupLeaveGroupParam create() => GroupLeaveGroupParam._();
  GroupLeaveGroupParam createEmptyInstance() => create();
  static $pb.PbList<GroupLeaveGroupParam> createRepeated() => $pb.PbList<GroupLeaveGroupParam>();
  @$core.pragma('dart2js:noInline')
  static GroupLeaveGroupParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupLeaveGroupParam>(create);
  static GroupLeaveGroupParam _defaultInstance;
}

class GroupLeaveGroupResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupLeaveGroupResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupLeaveGroupResponse._() : super();
  factory GroupLeaveGroupResponse() => create();
  factory GroupLeaveGroupResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupLeaveGroupResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupLeaveGroupResponse clone() => GroupLeaveGroupResponse()..mergeFromMessage(this);
  GroupLeaveGroupResponse copyWith(void Function(GroupLeaveGroupResponse) updates) => super.copyWith((message) => updates(message as GroupLeaveGroupResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupLeaveGroupResponse create() => GroupLeaveGroupResponse._();
  GroupLeaveGroupResponse createEmptyInstance() => create();
  static $pb.PbList<GroupLeaveGroupResponse> createRepeated() => $pb.PbList<GroupLeaveGroupResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupLeaveGroupResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupLeaveGroupResponse>(create);
  static GroupLeaveGroupResponse _defaultInstance;
}

class GroupBanMemberParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupBanMemberParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupBanMemberParam._() : super();
  factory GroupBanMemberParam() => create();
  factory GroupBanMemberParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupBanMemberParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupBanMemberParam clone() => GroupBanMemberParam()..mergeFromMessage(this);
  GroupBanMemberParam copyWith(void Function(GroupBanMemberParam) updates) => super.copyWith((message) => updates(message as GroupBanMemberParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupBanMemberParam create() => GroupBanMemberParam._();
  GroupBanMemberParam createEmptyInstance() => create();
  static $pb.PbList<GroupBanMemberParam> createRepeated() => $pb.PbList<GroupBanMemberParam>();
  @$core.pragma('dart2js:noInline')
  static GroupBanMemberParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupBanMemberParam>(create);
  static GroupBanMemberParam _defaultInstance;
}

class GroupBanMemberResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupBanMemberResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupBanMemberResponse._() : super();
  factory GroupBanMemberResponse() => create();
  factory GroupBanMemberResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupBanMemberResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupBanMemberResponse clone() => GroupBanMemberResponse()..mergeFromMessage(this);
  GroupBanMemberResponse copyWith(void Function(GroupBanMemberResponse) updates) => super.copyWith((message) => updates(message as GroupBanMemberResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupBanMemberResponse create() => GroupBanMemberResponse._();
  GroupBanMemberResponse createEmptyInstance() => create();
  static $pb.PbList<GroupBanMemberResponse> createRepeated() => $pb.PbList<GroupBanMemberResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupBanMemberResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupBanMemberResponse>(create);
  static GroupBanMemberResponse _defaultInstance;
}

class GroupChangePrivacyParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupChangePrivacyParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupChangePrivacyParam._() : super();
  factory GroupChangePrivacyParam() => create();
  factory GroupChangePrivacyParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupChangePrivacyParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupChangePrivacyParam clone() => GroupChangePrivacyParam()..mergeFromMessage(this);
  GroupChangePrivacyParam copyWith(void Function(GroupChangePrivacyParam) updates) => super.copyWith((message) => updates(message as GroupChangePrivacyParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupChangePrivacyParam create() => GroupChangePrivacyParam._();
  GroupChangePrivacyParam createEmptyInstance() => create();
  static $pb.PbList<GroupChangePrivacyParam> createRepeated() => $pb.PbList<GroupChangePrivacyParam>();
  @$core.pragma('dart2js:noInline')
  static GroupChangePrivacyParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupChangePrivacyParam>(create);
  static GroupChangePrivacyParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupChangePrivacyResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupChangePrivacyResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupChangePrivacyResponse._() : super();
  factory GroupChangePrivacyResponse() => create();
  factory GroupChangePrivacyResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupChangePrivacyResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupChangePrivacyResponse clone() => GroupChangePrivacyResponse()..mergeFromMessage(this);
  GroupChangePrivacyResponse copyWith(void Function(GroupChangePrivacyResponse) updates) => super.copyWith((message) => updates(message as GroupChangePrivacyResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupChangePrivacyResponse create() => GroupChangePrivacyResponse._();
  GroupChangePrivacyResponse createEmptyInstance() => create();
  static $pb.PbList<GroupChangePrivacyResponse> createRepeated() => $pb.PbList<GroupChangePrivacyResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupChangePrivacyResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupChangePrivacyResponse>(create);
  static GroupChangePrivacyResponse _defaultInstance;
}

class GroupChangeDefaultPermissionParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupChangeDefaultPermissionParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupChangeDefaultPermissionParam._() : super();
  factory GroupChangeDefaultPermissionParam() => create();
  factory GroupChangeDefaultPermissionParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupChangeDefaultPermissionParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupChangeDefaultPermissionParam clone() => GroupChangeDefaultPermissionParam()..mergeFromMessage(this);
  GroupChangeDefaultPermissionParam copyWith(void Function(GroupChangeDefaultPermissionParam) updates) => super.copyWith((message) => updates(message as GroupChangeDefaultPermissionParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupChangeDefaultPermissionParam create() => GroupChangeDefaultPermissionParam._();
  GroupChangeDefaultPermissionParam createEmptyInstance() => create();
  static $pb.PbList<GroupChangeDefaultPermissionParam> createRepeated() => $pb.PbList<GroupChangeDefaultPermissionParam>();
  @$core.pragma('dart2js:noInline')
  static GroupChangeDefaultPermissionParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupChangeDefaultPermissionParam>(create);
  static GroupChangeDefaultPermissionParam _defaultInstance;
}

class GroupChangeDefaultPermissionResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupChangeDefaultPermissionResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupChangeDefaultPermissionResponse._() : super();
  factory GroupChangeDefaultPermissionResponse() => create();
  factory GroupChangeDefaultPermissionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupChangeDefaultPermissionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupChangeDefaultPermissionResponse clone() => GroupChangeDefaultPermissionResponse()..mergeFromMessage(this);
  GroupChangeDefaultPermissionResponse copyWith(void Function(GroupChangeDefaultPermissionResponse) updates) => super.copyWith((message) => updates(message as GroupChangeDefaultPermissionResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupChangeDefaultPermissionResponse create() => GroupChangeDefaultPermissionResponse._();
  GroupChangeDefaultPermissionResponse createEmptyInstance() => create();
  static $pb.PbList<GroupChangeDefaultPermissionResponse> createRepeated() => $pb.PbList<GroupChangeDefaultPermissionResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupChangeDefaultPermissionResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupChangeDefaultPermissionResponse>(create);
  static GroupChangeDefaultPermissionResponse _defaultInstance;
}

class GroupRevokeLinkParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupRevokeLinkParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupRevokeLinkParam._() : super();
  factory GroupRevokeLinkParam() => create();
  factory GroupRevokeLinkParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupRevokeLinkParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupRevokeLinkParam clone() => GroupRevokeLinkParam()..mergeFromMessage(this);
  GroupRevokeLinkParam copyWith(void Function(GroupRevokeLinkParam) updates) => super.copyWith((message) => updates(message as GroupRevokeLinkParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupRevokeLinkParam create() => GroupRevokeLinkParam._();
  GroupRevokeLinkParam createEmptyInstance() => create();
  static $pb.PbList<GroupRevokeLinkParam> createRepeated() => $pb.PbList<GroupRevokeLinkParam>();
  @$core.pragma('dart2js:noInline')
  static GroupRevokeLinkParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupRevokeLinkParam>(create);
  static GroupRevokeLinkParam _defaultInstance;
}

class GroupRevokeLinkResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupRevokeLinkResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupRevokeLinkResponse._() : super();
  factory GroupRevokeLinkResponse() => create();
  factory GroupRevokeLinkResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupRevokeLinkResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupRevokeLinkResponse clone() => GroupRevokeLinkResponse()..mergeFromMessage(this);
  GroupRevokeLinkResponse copyWith(void Function(GroupRevokeLinkResponse) updates) => super.copyWith((message) => updates(message as GroupRevokeLinkResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupRevokeLinkResponse create() => GroupRevokeLinkResponse._();
  GroupRevokeLinkResponse createEmptyInstance() => create();
  static $pb.PbList<GroupRevokeLinkResponse> createRepeated() => $pb.PbList<GroupRevokeLinkResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupRevokeLinkResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupRevokeLinkResponse>(create);
  static GroupRevokeLinkResponse _defaultInstance;
}

class GroupChangeUsernameParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupChangeUsernameParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupChangeUsernameParam._() : super();
  factory GroupChangeUsernameParam() => create();
  factory GroupChangeUsernameParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupChangeUsernameParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupChangeUsernameParam clone() => GroupChangeUsernameParam()..mergeFromMessage(this);
  GroupChangeUsernameParam copyWith(void Function(GroupChangeUsernameParam) updates) => super.copyWith((message) => updates(message as GroupChangeUsernameParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupChangeUsernameParam create() => GroupChangeUsernameParam._();
  GroupChangeUsernameParam createEmptyInstance() => create();
  static $pb.PbList<GroupChangeUsernameParam> createRepeated() => $pb.PbList<GroupChangeUsernameParam>();
  @$core.pragma('dart2js:noInline')
  static GroupChangeUsernameParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupChangeUsernameParam>(create);
  static GroupChangeUsernameParam _defaultInstance;
}

class GroupChangeUsernameResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupChangeUsernameResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupChangeUsernameResponse._() : super();
  factory GroupChangeUsernameResponse() => create();
  factory GroupChangeUsernameResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupChangeUsernameResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupChangeUsernameResponse clone() => GroupChangeUsernameResponse()..mergeFromMessage(this);
  GroupChangeUsernameResponse copyWith(void Function(GroupChangeUsernameResponse) updates) => super.copyWith((message) => updates(message as GroupChangeUsernameResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupChangeUsernameResponse create() => GroupChangeUsernameResponse._();
  GroupChangeUsernameResponse createEmptyInstance() => create();
  static $pb.PbList<GroupChangeUsernameResponse> createRepeated() => $pb.PbList<GroupChangeUsernameResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupChangeUsernameResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupChangeUsernameResponse>(create);
  static GroupChangeUsernameResponse _defaultInstance;
}

class GroupDeleteMessagesParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupDeleteMessagesParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupDeleteMessagesParam._() : super();
  factory GroupDeleteMessagesParam() => create();
  factory GroupDeleteMessagesParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupDeleteMessagesParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupDeleteMessagesParam clone() => GroupDeleteMessagesParam()..mergeFromMessage(this);
  GroupDeleteMessagesParam copyWith(void Function(GroupDeleteMessagesParam) updates) => super.copyWith((message) => updates(message as GroupDeleteMessagesParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupDeleteMessagesParam create() => GroupDeleteMessagesParam._();
  GroupDeleteMessagesParam createEmptyInstance() => create();
  static $pb.PbList<GroupDeleteMessagesParam> createRepeated() => $pb.PbList<GroupDeleteMessagesParam>();
  @$core.pragma('dart2js:noInline')
  static GroupDeleteMessagesParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupDeleteMessagesParam>(create);
  static GroupDeleteMessagesParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class GroupDeleteMessagesResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupDeleteMessagesResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupDeleteMessagesResponse._() : super();
  factory GroupDeleteMessagesResponse() => create();
  factory GroupDeleteMessagesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupDeleteMessagesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupDeleteMessagesResponse clone() => GroupDeleteMessagesResponse()..mergeFromMessage(this);
  GroupDeleteMessagesResponse copyWith(void Function(GroupDeleteMessagesResponse) updates) => super.copyWith((message) => updates(message as GroupDeleteMessagesResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupDeleteMessagesResponse create() => GroupDeleteMessagesResponse._();
  GroupDeleteMessagesResponse createEmptyInstance() => create();
  static $pb.PbList<GroupDeleteMessagesResponse> createRepeated() => $pb.PbList<GroupDeleteMessagesResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupDeleteMessagesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupDeleteMessagesResponse>(create);
  static GroupDeleteMessagesResponse _defaultInstance;
}

class GroupDeleteHistoryParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupDeleteHistoryParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupDeleteHistoryParam._() : super();
  factory GroupDeleteHistoryParam() => create();
  factory GroupDeleteHistoryParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupDeleteHistoryParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupDeleteHistoryParam clone() => GroupDeleteHistoryParam()..mergeFromMessage(this);
  GroupDeleteHistoryParam copyWith(void Function(GroupDeleteHistoryParam) updates) => super.copyWith((message) => updates(message as GroupDeleteHistoryParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupDeleteHistoryParam create() => GroupDeleteHistoryParam._();
  GroupDeleteHistoryParam createEmptyInstance() => create();
  static $pb.PbList<GroupDeleteHistoryParam> createRepeated() => $pb.PbList<GroupDeleteHistoryParam>();
  @$core.pragma('dart2js:noInline')
  static GroupDeleteHistoryParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupDeleteHistoryParam>(create);
  static GroupDeleteHistoryParam _defaultInstance;
}

class GroupDeleteHistoryResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupDeleteHistoryResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupDeleteHistoryResponse._() : super();
  factory GroupDeleteHistoryResponse() => create();
  factory GroupDeleteHistoryResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupDeleteHistoryResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupDeleteHistoryResponse clone() => GroupDeleteHistoryResponse()..mergeFromMessage(this);
  GroupDeleteHistoryResponse copyWith(void Function(GroupDeleteHistoryResponse) updates) => super.copyWith((message) => updates(message as GroupDeleteHistoryResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupDeleteHistoryResponse create() => GroupDeleteHistoryResponse._();
  GroupDeleteHistoryResponse createEmptyInstance() => create();
  static $pb.PbList<GroupDeleteHistoryResponse> createRepeated() => $pb.PbList<GroupDeleteHistoryResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupDeleteHistoryResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupDeleteHistoryResponse>(create);
  static GroupDeleteHistoryResponse _defaultInstance;
}

class GroupClearHistoryParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupClearHistoryParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupClearHistoryParam._() : super();
  factory GroupClearHistoryParam() => create();
  factory GroupClearHistoryParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupClearHistoryParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupClearHistoryParam clone() => GroupClearHistoryParam()..mergeFromMessage(this);
  GroupClearHistoryParam copyWith(void Function(GroupClearHistoryParam) updates) => super.copyWith((message) => updates(message as GroupClearHistoryParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupClearHistoryParam create() => GroupClearHistoryParam._();
  GroupClearHistoryParam createEmptyInstance() => create();
  static $pb.PbList<GroupClearHistoryParam> createRepeated() => $pb.PbList<GroupClearHistoryParam>();
  @$core.pragma('dart2js:noInline')
  static GroupClearHistoryParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupClearHistoryParam>(create);
  static GroupClearHistoryParam _defaultInstance;
}

class GroupClearHistoryResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupClearHistoryResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupClearHistoryResponse._() : super();
  factory GroupClearHistoryResponse() => create();
  factory GroupClearHistoryResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupClearHistoryResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupClearHistoryResponse clone() => GroupClearHistoryResponse()..mergeFromMessage(this);
  GroupClearHistoryResponse copyWith(void Function(GroupClearHistoryResponse) updates) => super.copyWith((message) => updates(message as GroupClearHistoryResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupClearHistoryResponse create() => GroupClearHistoryResponse._();
  GroupClearHistoryResponse createEmptyInstance() => create();
  static $pb.PbList<GroupClearHistoryResponse> createRepeated() => $pb.PbList<GroupClearHistoryResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupClearHistoryResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupClearHistoryResponse>(create);
  static GroupClearHistoryResponse _defaultInstance;
}

class GroupAvatarChangeParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupAvatarChangeParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupAvatarChangeParam._() : super();
  factory GroupAvatarChangeParam() => create();
  factory GroupAvatarChangeParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupAvatarChangeParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupAvatarChangeParam clone() => GroupAvatarChangeParam()..mergeFromMessage(this);
  GroupAvatarChangeParam copyWith(void Function(GroupAvatarChangeParam) updates) => super.copyWith((message) => updates(message as GroupAvatarChangeParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupAvatarChangeParam create() => GroupAvatarChangeParam._();
  GroupAvatarChangeParam createEmptyInstance() => create();
  static $pb.PbList<GroupAvatarChangeParam> createRepeated() => $pb.PbList<GroupAvatarChangeParam>();
  @$core.pragma('dart2js:noInline')
  static GroupAvatarChangeParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupAvatarChangeParam>(create);
  static GroupAvatarChangeParam _defaultInstance;
}

class GroupAvatarChangeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupAvatarChangeResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupAvatarChangeResponse._() : super();
  factory GroupAvatarChangeResponse() => create();
  factory GroupAvatarChangeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupAvatarChangeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupAvatarChangeResponse clone() => GroupAvatarChangeResponse()..mergeFromMessage(this);
  GroupAvatarChangeResponse copyWith(void Function(GroupAvatarChangeResponse) updates) => super.copyWith((message) => updates(message as GroupAvatarChangeResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupAvatarChangeResponse create() => GroupAvatarChangeResponse._();
  GroupAvatarChangeResponse createEmptyInstance() => create();
  static $pb.PbList<GroupAvatarChangeResponse> createRepeated() => $pb.PbList<GroupAvatarChangeResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupAvatarChangeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupAvatarChangeResponse>(create);
  static GroupAvatarChangeResponse _defaultInstance;
}

class GroupSendDoingActionParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupSendDoingActionParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupSendDoingActionParam._() : super();
  factory GroupSendDoingActionParam() => create();
  factory GroupSendDoingActionParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupSendDoingActionParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupSendDoingActionParam clone() => GroupSendDoingActionParam()..mergeFromMessage(this);
  GroupSendDoingActionParam copyWith(void Function(GroupSendDoingActionParam) updates) => super.copyWith((message) => updates(message as GroupSendDoingActionParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupSendDoingActionParam create() => GroupSendDoingActionParam._();
  GroupSendDoingActionParam createEmptyInstance() => create();
  static $pb.PbList<GroupSendDoingActionParam> createRepeated() => $pb.PbList<GroupSendDoingActionParam>();
  @$core.pragma('dart2js:noInline')
  static GroupSendDoingActionParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupSendDoingActionParam>(create);
  static GroupSendDoingActionParam _defaultInstance;
}

class GroupSendDoingActionResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupSendDoingActionResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupSendDoingActionResponse._() : super();
  factory GroupSendDoingActionResponse() => create();
  factory GroupSendDoingActionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupSendDoingActionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupSendDoingActionResponse clone() => GroupSendDoingActionResponse()..mergeFromMessage(this);
  GroupSendDoingActionResponse copyWith(void Function(GroupSendDoingActionResponse) updates) => super.copyWith((message) => updates(message as GroupSendDoingActionResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupSendDoingActionResponse create() => GroupSendDoingActionResponse._();
  GroupSendDoingActionResponse createEmptyInstance() => create();
  static $pb.PbList<GroupSendDoingActionResponse> createRepeated() => $pb.PbList<GroupSendDoingActionResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupSendDoingActionResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupSendDoingActionResponse>(create);
  static GroupSendDoingActionResponse _defaultInstance;
}

class GroupReportGroupParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupReportGroupParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupReportGroupParam._() : super();
  factory GroupReportGroupParam() => create();
  factory GroupReportGroupParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupReportGroupParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupReportGroupParam clone() => GroupReportGroupParam()..mergeFromMessage(this);
  GroupReportGroupParam copyWith(void Function(GroupReportGroupParam) updates) => super.copyWith((message) => updates(message as GroupReportGroupParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupReportGroupParam create() => GroupReportGroupParam._();
  GroupReportGroupParam createEmptyInstance() => create();
  static $pb.PbList<GroupReportGroupParam> createRepeated() => $pb.PbList<GroupReportGroupParam>();
  @$core.pragma('dart2js:noInline')
  static GroupReportGroupParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupReportGroupParam>(create);
  static GroupReportGroupParam _defaultInstance;
}

class GroupReportGroupResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupReportGroupResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupReportGroupResponse._() : super();
  factory GroupReportGroupResponse() => create();
  factory GroupReportGroupResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupReportGroupResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupReportGroupResponse clone() => GroupReportGroupResponse()..mergeFromMessage(this);
  GroupReportGroupResponse copyWith(void Function(GroupReportGroupResponse) updates) => super.copyWith((message) => updates(message as GroupReportGroupResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupReportGroupResponse create() => GroupReportGroupResponse._();
  GroupReportGroupResponse createEmptyInstance() => create();
  static $pb.PbList<GroupReportGroupResponse> createRepeated() => $pb.PbList<GroupReportGroupResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupReportGroupResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupReportGroupResponse>(create);
  static GroupReportGroupResponse _defaultInstance;
}

class GroupGetMessagesListParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupGetMessagesListParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupGetMessagesListParam._() : super();
  factory GroupGetMessagesListParam() => create();
  factory GroupGetMessagesListParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupGetMessagesListParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupGetMessagesListParam clone() => GroupGetMessagesListParam()..mergeFromMessage(this);
  GroupGetMessagesListParam copyWith(void Function(GroupGetMessagesListParam) updates) => super.copyWith((message) => updates(message as GroupGetMessagesListParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupGetMessagesListParam create() => GroupGetMessagesListParam._();
  GroupGetMessagesListParam createEmptyInstance() => create();
  static $pb.PbList<GroupGetMessagesListParam> createRepeated() => $pb.PbList<GroupGetMessagesListParam>();
  @$core.pragma('dart2js:noInline')
  static GroupGetMessagesListParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupGetMessagesListParam>(create);
  static GroupGetMessagesListParam _defaultInstance;
}

class GroupGetMessagesListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupGetMessagesListResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupGetMessagesListResponse._() : super();
  factory GroupGetMessagesListResponse() => create();
  factory GroupGetMessagesListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupGetMessagesListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupGetMessagesListResponse clone() => GroupGetMessagesListResponse()..mergeFromMessage(this);
  GroupGetMessagesListResponse copyWith(void Function(GroupGetMessagesListResponse) updates) => super.copyWith((message) => updates(message as GroupGetMessagesListResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupGetMessagesListResponse create() => GroupGetMessagesListResponse._();
  GroupGetMessagesListResponse createEmptyInstance() => create();
  static $pb.PbList<GroupGetMessagesListResponse> createRepeated() => $pb.PbList<GroupGetMessagesListResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupGetMessagesListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupGetMessagesListResponse>(create);
  static GroupGetMessagesListResponse _defaultInstance;
}

class GroupGetMediaListParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupGetMediaListParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupGetMediaListParam._() : super();
  factory GroupGetMediaListParam() => create();
  factory GroupGetMediaListParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupGetMediaListParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupGetMediaListParam clone() => GroupGetMediaListParam()..mergeFromMessage(this);
  GroupGetMediaListParam copyWith(void Function(GroupGetMediaListParam) updates) => super.copyWith((message) => updates(message as GroupGetMediaListParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupGetMediaListParam create() => GroupGetMediaListParam._();
  GroupGetMediaListParam createEmptyInstance() => create();
  static $pb.PbList<GroupGetMediaListParam> createRepeated() => $pb.PbList<GroupGetMediaListParam>();
  @$core.pragma('dart2js:noInline')
  static GroupGetMediaListParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupGetMediaListParam>(create);
  static GroupGetMediaListParam _defaultInstance;
}

class GroupGetMediaListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupGetMediaListResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupGetMediaListResponse._() : super();
  factory GroupGetMediaListResponse() => create();
  factory GroupGetMediaListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupGetMediaListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupGetMediaListResponse clone() => GroupGetMediaListResponse()..mergeFromMessage(this);
  GroupGetMediaListResponse copyWith(void Function(GroupGetMediaListResponse) updates) => super.copyWith((message) => updates(message as GroupGetMediaListResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupGetMediaListResponse create() => GroupGetMediaListResponse._();
  GroupGetMediaListResponse createEmptyInstance() => create();
  static $pb.PbList<GroupGetMediaListResponse> createRepeated() => $pb.PbList<GroupGetMediaListResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupGetMediaListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupGetMediaListResponse>(create);
  static GroupGetMediaListResponse _defaultInstance;
}

class GroupGetMembersListParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupGetMembersListParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupGetMembersListParam._() : super();
  factory GroupGetMembersListParam() => create();
  factory GroupGetMembersListParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupGetMembersListParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupGetMembersListParam clone() => GroupGetMembersListParam()..mergeFromMessage(this);
  GroupGetMembersListParam copyWith(void Function(GroupGetMembersListParam) updates) => super.copyWith((message) => updates(message as GroupGetMembersListParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupGetMembersListParam create() => GroupGetMembersListParam._();
  GroupGetMembersListParam createEmptyInstance() => create();
  static $pb.PbList<GroupGetMembersListParam> createRepeated() => $pb.PbList<GroupGetMembersListParam>();
  @$core.pragma('dart2js:noInline')
  static GroupGetMembersListParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupGetMembersListParam>(create);
  static GroupGetMembersListParam _defaultInstance;
}

class GroupGetMembersListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupGetMembersListResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupGetMembersListResponse._() : super();
  factory GroupGetMembersListResponse() => create();
  factory GroupGetMembersListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupGetMembersListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupGetMembersListResponse clone() => GroupGetMembersListResponse()..mergeFromMessage(this);
  GroupGetMembersListResponse copyWith(void Function(GroupGetMembersListResponse) updates) => super.copyWith((message) => updates(message as GroupGetMembersListResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupGetMembersListResponse create() => GroupGetMembersListResponse._();
  GroupGetMembersListResponse createEmptyInstance() => create();
  static $pb.PbList<GroupGetMembersListResponse> createRepeated() => $pb.PbList<GroupGetMembersListResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupGetMembersListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupGetMembersListResponse>(create);
  static GroupGetMembersListResponse _defaultInstance;
}

class GroupGetAdminsListParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupGetAdminsListParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupGetAdminsListParam._() : super();
  factory GroupGetAdminsListParam() => create();
  factory GroupGetAdminsListParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupGetAdminsListParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupGetAdminsListParam clone() => GroupGetAdminsListParam()..mergeFromMessage(this);
  GroupGetAdminsListParam copyWith(void Function(GroupGetAdminsListParam) updates) => super.copyWith((message) => updates(message as GroupGetAdminsListParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupGetAdminsListParam create() => GroupGetAdminsListParam._();
  GroupGetAdminsListParam createEmptyInstance() => create();
  static $pb.PbList<GroupGetAdminsListParam> createRepeated() => $pb.PbList<GroupGetAdminsListParam>();
  @$core.pragma('dart2js:noInline')
  static GroupGetAdminsListParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupGetAdminsListParam>(create);
  static GroupGetAdminsListParam _defaultInstance;
}

class GroupGetAdminsListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupGetAdminsListResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GroupGetAdminsListResponse._() : super();
  factory GroupGetAdminsListResponse() => create();
  factory GroupGetAdminsListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupGetAdminsListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupGetAdminsListResponse clone() => GroupGetAdminsListResponse()..mergeFromMessage(this);
  GroupGetAdminsListResponse copyWith(void Function(GroupGetAdminsListResponse) updates) => super.copyWith((message) => updates(message as GroupGetAdminsListResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupGetAdminsListResponse create() => GroupGetAdminsListResponse._();
  GroupGetAdminsListResponse createEmptyInstance() => create();
  static $pb.PbList<GroupGetAdminsListResponse> createRepeated() => $pb.PbList<GroupGetAdminsListResponse>();
  @$core.pragma('dart2js:noInline')
  static GroupGetAdminsListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupGetAdminsListResponse>(create);
  static GroupGetAdminsListResponse _defaultInstance;
}

class RPC_GroupApi {
  $pb.RpcClient _client;
  RPC_GroupApi(this._client);

  $async.Future<GroupCreateGroupResponse> groupCreateGroup($pb.ClientContext ctx, GroupCreateGroupParam request) {
    var emptyResponse = GroupCreateGroupResponse();
    return _client.invoke<GroupCreateGroupResponse>(ctx, 'RPC_Group', 'GroupCreateGroup', request, emptyResponse);
  }
  $async.Future<GroupEditGroupResponse> groupEditGroup($pb.ClientContext ctx, GroupEditGroupParam request) {
    var emptyResponse = GroupEditGroupResponse();
    return _client.invoke<GroupEditGroupResponse>(ctx, 'RPC_Group', 'GroupEditGroup', request, emptyResponse);
  }
  $async.Future<GroupDeleteGroupResponse> groupDeleteGroup($pb.ClientContext ctx, GroupDeleteGroupParam request) {
    var emptyResponse = GroupDeleteGroupResponse();
    return _client.invoke<GroupDeleteGroupResponse>(ctx, 'RPC_Group', 'GroupDeleteGroup', request, emptyResponse);
  }
  $async.Future<GroupAddAdminResponse> groupAddAdmin($pb.ClientContext ctx, GroupAddAdminParam request) {
    var emptyResponse = GroupAddAdminResponse();
    return _client.invoke<GroupAddAdminResponse>(ctx, 'RPC_Group', 'GroupAddAdmin', request, emptyResponse);
  }
  $async.Future<GroupAddMemberResponse> groupAddMember($pb.ClientContext ctx, GroupAddMemberParam request) {
    var emptyResponse = GroupAddMemberResponse();
    return _client.invoke<GroupAddMemberResponse>(ctx, 'RPC_Group', 'GroupAddMember', request, emptyResponse);
  }
  $async.Future<GroupRemoveMemberResponse> groupRemoveMember($pb.ClientContext ctx, GroupRemoveMemberParam request) {
    var emptyResponse = GroupRemoveMemberResponse();
    return _client.invoke<GroupRemoveMemberResponse>(ctx, 'RPC_Group', 'GroupRemoveMember', request, emptyResponse);
  }
  $async.Future<GroupChangeMemberLevelResponse> groupChangeMemberLevel($pb.ClientContext ctx, GroupChangeMemberLevelParam request) {
    var emptyResponse = GroupChangeMemberLevelResponse();
    return _client.invoke<GroupChangeMemberLevelResponse>(ctx, 'RPC_Group', 'GroupChangeMemberLevel', request, emptyResponse);
  }
  $async.Future<GroupChangeMemberPermissionResponse> groupChangeMemberPermission($pb.ClientContext ctx, GroupChangeMemberPermissionParam request) {
    var emptyResponse = GroupChangeMemberPermissionResponse();
    return _client.invoke<GroupChangeMemberPermissionResponse>(ctx, 'RPC_Group', 'GroupChangeMemberPermission', request, emptyResponse);
  }
  $async.Future<JoinGroupResponse> groupJoinGroup($pb.ClientContext ctx, JoinGroupParam request) {
    var emptyResponse = JoinGroupResponse();
    return _client.invoke<JoinGroupResponse>(ctx, 'RPC_Group', 'GroupJoinGroup', request, emptyResponse);
  }
  $async.Future<GroupLeaveGroupResponse> groupLeaveGroup($pb.ClientContext ctx, GroupLeaveGroupParam request) {
    var emptyResponse = GroupLeaveGroupResponse();
    return _client.invoke<GroupLeaveGroupResponse>(ctx, 'RPC_Group', 'GroupLeaveGroup', request, emptyResponse);
  }
  $async.Future<GroupBanMemberResponse> groupBanMember($pb.ClientContext ctx, GroupBanMemberParam request) {
    var emptyResponse = GroupBanMemberResponse();
    return _client.invoke<GroupBanMemberResponse>(ctx, 'RPC_Group', 'GroupBanMember', request, emptyResponse);
  }
  $async.Future<GroupChangePrivacyResponse> groupChangePrivacy($pb.ClientContext ctx, GroupChangePrivacyParam request) {
    var emptyResponse = GroupChangePrivacyResponse();
    return _client.invoke<GroupChangePrivacyResponse>(ctx, 'RPC_Group', 'GroupChangePrivacy', request, emptyResponse);
  }
  $async.Future<GroupChangeDefaultPermissionResponse> groupChangeDefaultPermission($pb.ClientContext ctx, GroupChangeDefaultPermissionParam request) {
    var emptyResponse = GroupChangeDefaultPermissionResponse();
    return _client.invoke<GroupChangeDefaultPermissionResponse>(ctx, 'RPC_Group', 'GroupChangeDefaultPermission', request, emptyResponse);
  }
  $async.Future<GroupRevokeLinkResponse> groupRevokeLink($pb.ClientContext ctx, GroupRevokeLinkParam request) {
    var emptyResponse = GroupRevokeLinkResponse();
    return _client.invoke<GroupRevokeLinkResponse>(ctx, 'RPC_Group', 'GroupRevokeLink', request, emptyResponse);
  }
  $async.Future<GroupChangeUsernameResponse> groupChangeUsername($pb.ClientContext ctx, GroupChangeUsernameParam request) {
    var emptyResponse = GroupChangeUsernameResponse();
    return _client.invoke<GroupChangeUsernameResponse>(ctx, 'RPC_Group', 'GroupChangeUsername', request, emptyResponse);
  }
  $async.Future<GroupSendMessageResponse> groupSendMessage($pb.ClientContext ctx, GroupSendMessageParam request) {
    var emptyResponse = GroupSendMessageResponse();
    return _client.invoke<GroupSendMessageResponse>(ctx, 'RPC_Group', 'GroupSendMessage', request, emptyResponse);
  }
  $async.Future<GroupEditMessageResponse> groupEditMessage($pb.ClientContext ctx, GroupEditMessageParam request) {
    var emptyResponse = GroupEditMessageResponse();
    return _client.invoke<GroupEditMessageResponse>(ctx, 'RPC_Group', 'GroupEditMessage', request, emptyResponse);
  }
  $async.Future<GroupPinMessageResponse> groupPinMessage($pb.ClientContext ctx, GroupPinMessageParam request) {
    var emptyResponse = GroupPinMessageResponse();
    return _client.invoke<GroupPinMessageResponse>(ctx, 'RPC_Group', 'GroupPinMessage', request, emptyResponse);
  }
  $async.Future<GroupUnPinMessageResponse> groupUnPinMessage($pb.ClientContext ctx, GroupUnPinMessageParam request) {
    var emptyResponse = GroupUnPinMessageResponse();
    return _client.invoke<GroupUnPinMessageResponse>(ctx, 'RPC_Group', 'GroupUnPinMessage', request, emptyResponse);
  }
  $async.Future<GroupDeleteMessageResponse> groupDeleteMessage($pb.ClientContext ctx, GroupDeleteMessageParam request) {
    var emptyResponse = GroupDeleteMessageResponse();
    return _client.invoke<GroupDeleteMessageResponse>(ctx, 'RPC_Group', 'GroupDeleteMessage', request, emptyResponse);
  }
  $async.Future<GroupDeleteMessagesResponse> groupDeleteMessages($pb.ClientContext ctx, GroupDeleteMessagesParam request) {
    var emptyResponse = GroupDeleteMessagesResponse();
    return _client.invoke<GroupDeleteMessagesResponse>(ctx, 'RPC_Group', 'GroupDeleteMessages', request, emptyResponse);
  }
  $async.Future<GroupDeleteHistoryResponse> groupDeleteHistory($pb.ClientContext ctx, GroupDeleteHistoryParam request) {
    var emptyResponse = GroupDeleteHistoryResponse();
    return _client.invoke<GroupDeleteHistoryResponse>(ctx, 'RPC_Group', 'GroupDeleteHistory', request, emptyResponse);
  }
  $async.Future<GroupClearHistoryResponse> groupClearHistory($pb.ClientContext ctx, GroupClearHistoryParam request) {
    var emptyResponse = GroupClearHistoryResponse();
    return _client.invoke<GroupClearHistoryResponse>(ctx, 'RPC_Group', 'GroupClearHistory', request, emptyResponse);
  }
  $async.Future<GroupAvatarAddResponse> groupAvatarAdd($pb.ClientContext ctx, GroupAvatarAddParam request) {
    var emptyResponse = GroupAvatarAddResponse();
    return _client.invoke<GroupAvatarAddResponse>(ctx, 'RPC_Group', 'GroupAvatarAdd', request, emptyResponse);
  }
  $async.Future<GroupAvatarChangeResponse> groupAvatarChange($pb.ClientContext ctx, GroupAvatarChangeParam request) {
    var emptyResponse = GroupAvatarChangeResponse();
    return _client.invoke<GroupAvatarChangeResponse>(ctx, 'RPC_Group', 'GroupAvatarChange', request, emptyResponse);
  }
  $async.Future<GroupAvatarDeleteResponse> groupAvatarDelete($pb.ClientContext ctx, GroupAvatarDeleteParam request) {
    var emptyResponse = GroupAvatarDeleteResponse();
    return _client.invoke<GroupAvatarDeleteResponse>(ctx, 'RPC_Group', 'GroupAvatarDelete', request, emptyResponse);
  }
  $async.Future<GroupAvatarGetListResponse> groupAvatarGetList($pb.ClientContext ctx, GroupAvatarGetListParam request) {
    var emptyResponse = GroupAvatarGetListResponse();
    return _client.invoke<GroupAvatarGetListResponse>(ctx, 'RPC_Group', 'GroupAvatarGetList', request, emptyResponse);
  }
  $async.Future<GroupSendDoingActionResponse> groupSendDoingAction($pb.ClientContext ctx, GroupSendDoingActionParam request) {
    var emptyResponse = GroupSendDoingActionResponse();
    return _client.invoke<GroupSendDoingActionResponse>(ctx, 'RPC_Group', 'GroupSendDoingAction', request, emptyResponse);
  }
  $async.Future<GroupReportGroupResponse> groupReportGroup($pb.ClientContext ctx, GroupReportGroupParam request) {
    var emptyResponse = GroupReportGroupResponse();
    return _client.invoke<GroupReportGroupResponse>(ctx, 'RPC_Group', 'GroupReportGroup', request, emptyResponse);
  }
  $async.Future<GroupGetFullMessageResponse> groupGetFull($pb.ClientContext ctx, GroupGetFullMessageParam request) {
    var emptyResponse = GroupGetFullMessageResponse();
    return _client.invoke<GroupGetFullMessageResponse>(ctx, 'RPC_Group', 'GroupGetFull', request, emptyResponse);
  }
  $async.Future<GroupGetMessagesListResponse> groupGetMessagesList($pb.ClientContext ctx, GroupGetMessagesListParam request) {
    var emptyResponse = GroupGetMessagesListResponse();
    return _client.invoke<GroupGetMessagesListResponse>(ctx, 'RPC_Group', 'GroupGetMessagesList', request, emptyResponse);
  }
  $async.Future<GroupGetMediaListResponse> groupGetMediaList($pb.ClientContext ctx, GroupGetMediaListParam request) {
    var emptyResponse = GroupGetMediaListResponse();
    return _client.invoke<GroupGetMediaListResponse>(ctx, 'RPC_Group', 'GroupGetMediaList', request, emptyResponse);
  }
  $async.Future<GroupGetMembersListResponse> groupGetMembersList($pb.ClientContext ctx, GroupGetMembersListParam request) {
    var emptyResponse = GroupGetMembersListResponse();
    return _client.invoke<GroupGetMembersListResponse>(ctx, 'RPC_Group', 'GroupGetMembersList', request, emptyResponse);
  }
  $async.Future<GroupGetAdminsListResponse> groupGetAdminsList($pb.ClientContext ctx, GroupGetAdminsListParam request) {
    var emptyResponse = GroupGetAdminsListResponse();
    return _client.invoke<GroupGetAdminsListResponse>(ctx, 'RPC_Group', 'GroupGetAdminsList', request, emptyResponse);
  }
  $async.Future<GroupSetDraftResponse> groupSetDraft($pb.ClientContext ctx, GroupSetDraftParam request) {
    var emptyResponse = GroupSetDraftResponse();
    return _client.invoke<GroupSetDraftResponse>(ctx, 'RPC_Group', 'GroupSetDraft', request, emptyResponse);
  }
}

