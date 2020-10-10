///
//  Generated code. Do not modify.
//  source: rpc_channel.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class ChannelAvatarAddParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelAvatarAddParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'channelId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelAvatarAddParam._() : super();
  factory ChannelAvatarAddParam() => create();
  factory ChannelAvatarAddParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelAvatarAddParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelAvatarAddParam clone() => ChannelAvatarAddParam()..mergeFromMessage(this);
  ChannelAvatarAddParam copyWith(void Function(ChannelAvatarAddParam) updates) => super.copyWith((message) => updates(message as ChannelAvatarAddParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarAddParam create() => ChannelAvatarAddParam._();
  ChannelAvatarAddParam createEmptyInstance() => create();
  static $pb.PbList<ChannelAvatarAddParam> createRepeated() => $pb.PbList<ChannelAvatarAddParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarAddParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelAvatarAddParam>(create);
  static ChannelAvatarAddParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get channelId => $_getIZ(0);
  @$pb.TagNumber(1)
  set channelId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class ChannelAvatarAddResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelAvatarAddResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelAvatarAddResponse._() : super();
  factory ChannelAvatarAddResponse() => create();
  factory ChannelAvatarAddResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelAvatarAddResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelAvatarAddResponse clone() => ChannelAvatarAddResponse()..mergeFromMessage(this);
  ChannelAvatarAddResponse copyWith(void Function(ChannelAvatarAddResponse) updates) => super.copyWith((message) => updates(message as ChannelAvatarAddResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarAddResponse create() => ChannelAvatarAddResponse._();
  ChannelAvatarAddResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelAvatarAddResponse> createRepeated() => $pb.PbList<ChannelAvatarAddResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarAddResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelAvatarAddResponse>(create);
  static ChannelAvatarAddResponse _defaultInstance;
}

class ChannelAvatarDeleteParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelAvatarDeleteParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'channelId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelAvatarDeleteParam._() : super();
  factory ChannelAvatarDeleteParam() => create();
  factory ChannelAvatarDeleteParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelAvatarDeleteParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelAvatarDeleteParam clone() => ChannelAvatarDeleteParam()..mergeFromMessage(this);
  ChannelAvatarDeleteParam copyWith(void Function(ChannelAvatarDeleteParam) updates) => super.copyWith((message) => updates(message as ChannelAvatarDeleteParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarDeleteParam create() => ChannelAvatarDeleteParam._();
  ChannelAvatarDeleteParam createEmptyInstance() => create();
  static $pb.PbList<ChannelAvatarDeleteParam> createRepeated() => $pb.PbList<ChannelAvatarDeleteParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarDeleteParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelAvatarDeleteParam>(create);
  static ChannelAvatarDeleteParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get channelId => $_getIZ(0);
  @$pb.TagNumber(1)
  set channelId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class ChannelAvatarDeleteResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelAvatarDeleteResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelAvatarDeleteResponse._() : super();
  factory ChannelAvatarDeleteResponse() => create();
  factory ChannelAvatarDeleteResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelAvatarDeleteResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelAvatarDeleteResponse clone() => ChannelAvatarDeleteResponse()..mergeFromMessage(this);
  ChannelAvatarDeleteResponse copyWith(void Function(ChannelAvatarDeleteResponse) updates) => super.copyWith((message) => updates(message as ChannelAvatarDeleteResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarDeleteResponse create() => ChannelAvatarDeleteResponse._();
  ChannelAvatarDeleteResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelAvatarDeleteResponse> createRepeated() => $pb.PbList<ChannelAvatarDeleteResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarDeleteResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelAvatarDeleteResponse>(create);
  static ChannelAvatarDeleteResponse _defaultInstance;
}

class ChannelAvatarGetListParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelAvatarGetListParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'channelId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelAvatarGetListParam._() : super();
  factory ChannelAvatarGetListParam() => create();
  factory ChannelAvatarGetListParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelAvatarGetListParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelAvatarGetListParam clone() => ChannelAvatarGetListParam()..mergeFromMessage(this);
  ChannelAvatarGetListParam copyWith(void Function(ChannelAvatarGetListParam) updates) => super.copyWith((message) => updates(message as ChannelAvatarGetListParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarGetListParam create() => ChannelAvatarGetListParam._();
  ChannelAvatarGetListParam createEmptyInstance() => create();
  static $pb.PbList<ChannelAvatarGetListParam> createRepeated() => $pb.PbList<ChannelAvatarGetListParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarGetListParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelAvatarGetListParam>(create);
  static ChannelAvatarGetListParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get channelId => $_getIZ(0);
  @$pb.TagNumber(1)
  set channelId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class ChannelAvatarGetListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelAvatarGetListResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelAvatarGetListResponse._() : super();
  factory ChannelAvatarGetListResponse() => create();
  factory ChannelAvatarGetListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelAvatarGetListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelAvatarGetListResponse clone() => ChannelAvatarGetListResponse()..mergeFromMessage(this);
  ChannelAvatarGetListResponse copyWith(void Function(ChannelAvatarGetListResponse) updates) => super.copyWith((message) => updates(message as ChannelAvatarGetListResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarGetListResponse create() => ChannelAvatarGetListResponse._();
  ChannelAvatarGetListResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelAvatarGetListResponse> createRepeated() => $pb.PbList<ChannelAvatarGetListResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarGetListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelAvatarGetListResponse>(create);
  static ChannelAvatarGetListResponse _defaultInstance;
}

class ChannelSendMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelSendMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'channelId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelSendMessageParam._() : super();
  factory ChannelSendMessageParam() => create();
  factory ChannelSendMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelSendMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelSendMessageParam clone() => ChannelSendMessageParam()..mergeFromMessage(this);
  ChannelSendMessageParam copyWith(void Function(ChannelSendMessageParam) updates) => super.copyWith((message) => updates(message as ChannelSendMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelSendMessageParam create() => ChannelSendMessageParam._();
  ChannelSendMessageParam createEmptyInstance() => create();
  static $pb.PbList<ChannelSendMessageParam> createRepeated() => $pb.PbList<ChannelSendMessageParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelSendMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelSendMessageParam>(create);
  static ChannelSendMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get channelId => $_getIZ(0);
  @$pb.TagNumber(1)
  set channelId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class ChannelSendMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelSendMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelSendMessageResponse._() : super();
  factory ChannelSendMessageResponse() => create();
  factory ChannelSendMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelSendMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelSendMessageResponse clone() => ChannelSendMessageResponse()..mergeFromMessage(this);
  ChannelSendMessageResponse copyWith(void Function(ChannelSendMessageResponse) updates) => super.copyWith((message) => updates(message as ChannelSendMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelSendMessageResponse create() => ChannelSendMessageResponse._();
  ChannelSendMessageResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelSendMessageResponse> createRepeated() => $pb.PbList<ChannelSendMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelSendMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelSendMessageResponse>(create);
  static ChannelSendMessageResponse _defaultInstance;
}

class ChannelEditMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelEditMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'channelId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelEditMessageParam._() : super();
  factory ChannelEditMessageParam() => create();
  factory ChannelEditMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelEditMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelEditMessageParam clone() => ChannelEditMessageParam()..mergeFromMessage(this);
  ChannelEditMessageParam copyWith(void Function(ChannelEditMessageParam) updates) => super.copyWith((message) => updates(message as ChannelEditMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelEditMessageParam create() => ChannelEditMessageParam._();
  ChannelEditMessageParam createEmptyInstance() => create();
  static $pb.PbList<ChannelEditMessageParam> createRepeated() => $pb.PbList<ChannelEditMessageParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelEditMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelEditMessageParam>(create);
  static ChannelEditMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get channelId => $_getIZ(0);
  @$pb.TagNumber(1)
  set channelId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class ChannelEditMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelEditMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelEditMessageResponse._() : super();
  factory ChannelEditMessageResponse() => create();
  factory ChannelEditMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelEditMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelEditMessageResponse clone() => ChannelEditMessageResponse()..mergeFromMessage(this);
  ChannelEditMessageResponse copyWith(void Function(ChannelEditMessageResponse) updates) => super.copyWith((message) => updates(message as ChannelEditMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelEditMessageResponse create() => ChannelEditMessageResponse._();
  ChannelEditMessageResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelEditMessageResponse> createRepeated() => $pb.PbList<ChannelEditMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelEditMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelEditMessageResponse>(create);
  static ChannelEditMessageResponse _defaultInstance;
}

class ChannelPinMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelPinMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'channelId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelPinMessageParam._() : super();
  factory ChannelPinMessageParam() => create();
  factory ChannelPinMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelPinMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelPinMessageParam clone() => ChannelPinMessageParam()..mergeFromMessage(this);
  ChannelPinMessageParam copyWith(void Function(ChannelPinMessageParam) updates) => super.copyWith((message) => updates(message as ChannelPinMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelPinMessageParam create() => ChannelPinMessageParam._();
  ChannelPinMessageParam createEmptyInstance() => create();
  static $pb.PbList<ChannelPinMessageParam> createRepeated() => $pb.PbList<ChannelPinMessageParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelPinMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelPinMessageParam>(create);
  static ChannelPinMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get channelId => $_getIZ(0);
  @$pb.TagNumber(1)
  set channelId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class ChannelPinMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelPinMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelPinMessageResponse._() : super();
  factory ChannelPinMessageResponse() => create();
  factory ChannelPinMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelPinMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelPinMessageResponse clone() => ChannelPinMessageResponse()..mergeFromMessage(this);
  ChannelPinMessageResponse copyWith(void Function(ChannelPinMessageResponse) updates) => super.copyWith((message) => updates(message as ChannelPinMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelPinMessageResponse create() => ChannelPinMessageResponse._();
  ChannelPinMessageResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelPinMessageResponse> createRepeated() => $pb.PbList<ChannelPinMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelPinMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelPinMessageResponse>(create);
  static ChannelPinMessageResponse _defaultInstance;
}

class ChannelUnPinMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelUnPinMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'channelId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelUnPinMessageParam._() : super();
  factory ChannelUnPinMessageParam() => create();
  factory ChannelUnPinMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelUnPinMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelUnPinMessageParam clone() => ChannelUnPinMessageParam()..mergeFromMessage(this);
  ChannelUnPinMessageParam copyWith(void Function(ChannelUnPinMessageParam) updates) => super.copyWith((message) => updates(message as ChannelUnPinMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelUnPinMessageParam create() => ChannelUnPinMessageParam._();
  ChannelUnPinMessageParam createEmptyInstance() => create();
  static $pb.PbList<ChannelUnPinMessageParam> createRepeated() => $pb.PbList<ChannelUnPinMessageParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelUnPinMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelUnPinMessageParam>(create);
  static ChannelUnPinMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get channelId => $_getIZ(0);
  @$pb.TagNumber(1)
  set channelId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class ChannelUnPinMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelUnPinMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelUnPinMessageResponse._() : super();
  factory ChannelUnPinMessageResponse() => create();
  factory ChannelUnPinMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelUnPinMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelUnPinMessageResponse clone() => ChannelUnPinMessageResponse()..mergeFromMessage(this);
  ChannelUnPinMessageResponse copyWith(void Function(ChannelUnPinMessageResponse) updates) => super.copyWith((message) => updates(message as ChannelUnPinMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelUnPinMessageResponse create() => ChannelUnPinMessageResponse._();
  ChannelUnPinMessageResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelUnPinMessageResponse> createRepeated() => $pb.PbList<ChannelUnPinMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelUnPinMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelUnPinMessageResponse>(create);
  static ChannelUnPinMessageResponse _defaultInstance;
}

class ChannelDeleteMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelDeleteMessageParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'channelId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelDeleteMessageParam._() : super();
  factory ChannelDeleteMessageParam() => create();
  factory ChannelDeleteMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelDeleteMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelDeleteMessageParam clone() => ChannelDeleteMessageParam()..mergeFromMessage(this);
  ChannelDeleteMessageParam copyWith(void Function(ChannelDeleteMessageParam) updates) => super.copyWith((message) => updates(message as ChannelDeleteMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelDeleteMessageParam create() => ChannelDeleteMessageParam._();
  ChannelDeleteMessageParam createEmptyInstance() => create();
  static $pb.PbList<ChannelDeleteMessageParam> createRepeated() => $pb.PbList<ChannelDeleteMessageParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelDeleteMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelDeleteMessageParam>(create);
  static ChannelDeleteMessageParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get channelId => $_getIZ(0);
  @$pb.TagNumber(1)
  set channelId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class ChannelDeleteMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelDeleteMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelDeleteMessageResponse._() : super();
  factory ChannelDeleteMessageResponse() => create();
  factory ChannelDeleteMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelDeleteMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelDeleteMessageResponse clone() => ChannelDeleteMessageResponse()..mergeFromMessage(this);
  ChannelDeleteMessageResponse copyWith(void Function(ChannelDeleteMessageResponse) updates) => super.copyWith((message) => updates(message as ChannelDeleteMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelDeleteMessageResponse create() => ChannelDeleteMessageResponse._();
  ChannelDeleteMessageResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelDeleteMessageResponse> createRepeated() => $pb.PbList<ChannelDeleteMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelDeleteMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelDeleteMessageResponse>(create);
  static ChannelDeleteMessageResponse _defaultInstance;
}

class ChannelSetDraftParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelSetDraftParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'channelId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelSetDraftParam._() : super();
  factory ChannelSetDraftParam() => create();
  factory ChannelSetDraftParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelSetDraftParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelSetDraftParam clone() => ChannelSetDraftParam()..mergeFromMessage(this);
  ChannelSetDraftParam copyWith(void Function(ChannelSetDraftParam) updates) => super.copyWith((message) => updates(message as ChannelSetDraftParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelSetDraftParam create() => ChannelSetDraftParam._();
  ChannelSetDraftParam createEmptyInstance() => create();
  static $pb.PbList<ChannelSetDraftParam> createRepeated() => $pb.PbList<ChannelSetDraftParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelSetDraftParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelSetDraftParam>(create);
  static ChannelSetDraftParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get channelId => $_getIZ(0);
  @$pb.TagNumber(1)
  set channelId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class ChannelSetDraftResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelSetDraftResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelSetDraftResponse._() : super();
  factory ChannelSetDraftResponse() => create();
  factory ChannelSetDraftResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelSetDraftResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelSetDraftResponse clone() => ChannelSetDraftResponse()..mergeFromMessage(this);
  ChannelSetDraftResponse copyWith(void Function(ChannelSetDraftResponse) updates) => super.copyWith((message) => updates(message as ChannelSetDraftResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelSetDraftResponse create() => ChannelSetDraftResponse._();
  ChannelSetDraftResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelSetDraftResponse> createRepeated() => $pb.PbList<ChannelSetDraftResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelSetDraftResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelSetDraftResponse>(create);
  static ChannelSetDraftResponse _defaultInstance;
}

class ChannelCreateChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelCreateChannelParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelCreateChannelParam._() : super();
  factory ChannelCreateChannelParam() => create();
  factory ChannelCreateChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelCreateChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelCreateChannelParam clone() => ChannelCreateChannelParam()..mergeFromMessage(this);
  ChannelCreateChannelParam copyWith(void Function(ChannelCreateChannelParam) updates) => super.copyWith((message) => updates(message as ChannelCreateChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelCreateChannelParam create() => ChannelCreateChannelParam._();
  ChannelCreateChannelParam createEmptyInstance() => create();
  static $pb.PbList<ChannelCreateChannelParam> createRepeated() => $pb.PbList<ChannelCreateChannelParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelCreateChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelCreateChannelParam>(create);
  static ChannelCreateChannelParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class ChannelCreateChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelCreateChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelCreateChannelResponse._() : super();
  factory ChannelCreateChannelResponse() => create();
  factory ChannelCreateChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelCreateChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelCreateChannelResponse clone() => ChannelCreateChannelResponse()..mergeFromMessage(this);
  ChannelCreateChannelResponse copyWith(void Function(ChannelCreateChannelResponse) updates) => super.copyWith((message) => updates(message as ChannelCreateChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelCreateChannelResponse create() => ChannelCreateChannelResponse._();
  ChannelCreateChannelResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelCreateChannelResponse> createRepeated() => $pb.PbList<ChannelCreateChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelCreateChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelCreateChannelResponse>(create);
  static ChannelCreateChannelResponse _defaultInstance;
}

class ChannelEditChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelEditChannelParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelEditChannelParam._() : super();
  factory ChannelEditChannelParam() => create();
  factory ChannelEditChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelEditChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelEditChannelParam clone() => ChannelEditChannelParam()..mergeFromMessage(this);
  ChannelEditChannelParam copyWith(void Function(ChannelEditChannelParam) updates) => super.copyWith((message) => updates(message as ChannelEditChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelEditChannelParam create() => ChannelEditChannelParam._();
  ChannelEditChannelParam createEmptyInstance() => create();
  static $pb.PbList<ChannelEditChannelParam> createRepeated() => $pb.PbList<ChannelEditChannelParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelEditChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelEditChannelParam>(create);
  static ChannelEditChannelParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class ChannelEditChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelEditChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelEditChannelResponse._() : super();
  factory ChannelEditChannelResponse() => create();
  factory ChannelEditChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelEditChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelEditChannelResponse clone() => ChannelEditChannelResponse()..mergeFromMessage(this);
  ChannelEditChannelResponse copyWith(void Function(ChannelEditChannelResponse) updates) => super.copyWith((message) => updates(message as ChannelEditChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelEditChannelResponse create() => ChannelEditChannelResponse._();
  ChannelEditChannelResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelEditChannelResponse> createRepeated() => $pb.PbList<ChannelEditChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelEditChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelEditChannelResponse>(create);
  static ChannelEditChannelResponse _defaultInstance;
}

class ChannelDeleteChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelDeleteChannelParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelDeleteChannelParam._() : super();
  factory ChannelDeleteChannelParam() => create();
  factory ChannelDeleteChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelDeleteChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelDeleteChannelParam clone() => ChannelDeleteChannelParam()..mergeFromMessage(this);
  ChannelDeleteChannelParam copyWith(void Function(ChannelDeleteChannelParam) updates) => super.copyWith((message) => updates(message as ChannelDeleteChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelDeleteChannelParam create() => ChannelDeleteChannelParam._();
  ChannelDeleteChannelParam createEmptyInstance() => create();
  static $pb.PbList<ChannelDeleteChannelParam> createRepeated() => $pb.PbList<ChannelDeleteChannelParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelDeleteChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelDeleteChannelParam>(create);
  static ChannelDeleteChannelParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class ChannelDeleteChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelDeleteChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelDeleteChannelResponse._() : super();
  factory ChannelDeleteChannelResponse() => create();
  factory ChannelDeleteChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelDeleteChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelDeleteChannelResponse clone() => ChannelDeleteChannelResponse()..mergeFromMessage(this);
  ChannelDeleteChannelResponse copyWith(void Function(ChannelDeleteChannelResponse) updates) => super.copyWith((message) => updates(message as ChannelDeleteChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelDeleteChannelResponse create() => ChannelDeleteChannelResponse._();
  ChannelDeleteChannelResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelDeleteChannelResponse> createRepeated() => $pb.PbList<ChannelDeleteChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelDeleteChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelDeleteChannelResponse>(create);
  static ChannelDeleteChannelResponse _defaultInstance;
}

class ChannelAddAuthorParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelAddAuthorParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelAddAuthorParam._() : super();
  factory ChannelAddAuthorParam() => create();
  factory ChannelAddAuthorParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelAddAuthorParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelAddAuthorParam clone() => ChannelAddAuthorParam()..mergeFromMessage(this);
  ChannelAddAuthorParam copyWith(void Function(ChannelAddAuthorParam) updates) => super.copyWith((message) => updates(message as ChannelAddAuthorParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelAddAuthorParam create() => ChannelAddAuthorParam._();
  ChannelAddAuthorParam createEmptyInstance() => create();
  static $pb.PbList<ChannelAddAuthorParam> createRepeated() => $pb.PbList<ChannelAddAuthorParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelAddAuthorParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelAddAuthorParam>(create);
  static ChannelAddAuthorParam _defaultInstance;
}

class ChannelAddAuthorResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelAddAuthorResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelAddAuthorResponse._() : super();
  factory ChannelAddAuthorResponse() => create();
  factory ChannelAddAuthorResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelAddAuthorResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelAddAuthorResponse clone() => ChannelAddAuthorResponse()..mergeFromMessage(this);
  ChannelAddAuthorResponse copyWith(void Function(ChannelAddAuthorResponse) updates) => super.copyWith((message) => updates(message as ChannelAddAuthorResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelAddAuthorResponse create() => ChannelAddAuthorResponse._();
  ChannelAddAuthorResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelAddAuthorResponse> createRepeated() => $pb.PbList<ChannelAddAuthorResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelAddAuthorResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelAddAuthorResponse>(create);
  static ChannelAddAuthorResponse _defaultInstance;
}

class ChannelChangeAuthorPermissionParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelChangeAuthorPermissionParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelChangeAuthorPermissionParam._() : super();
  factory ChannelChangeAuthorPermissionParam() => create();
  factory ChannelChangeAuthorPermissionParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelChangeAuthorPermissionParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelChangeAuthorPermissionParam clone() => ChannelChangeAuthorPermissionParam()..mergeFromMessage(this);
  ChannelChangeAuthorPermissionParam copyWith(void Function(ChannelChangeAuthorPermissionParam) updates) => super.copyWith((message) => updates(message as ChannelChangeAuthorPermissionParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelChangeAuthorPermissionParam create() => ChannelChangeAuthorPermissionParam._();
  ChannelChangeAuthorPermissionParam createEmptyInstance() => create();
  static $pb.PbList<ChannelChangeAuthorPermissionParam> createRepeated() => $pb.PbList<ChannelChangeAuthorPermissionParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelChangeAuthorPermissionParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelChangeAuthorPermissionParam>(create);
  static ChannelChangeAuthorPermissionParam _defaultInstance;
}

class ChannelChangeAuthorPermissionResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelChangeAuthorPermissionResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelChangeAuthorPermissionResponse._() : super();
  factory ChannelChangeAuthorPermissionResponse() => create();
  factory ChannelChangeAuthorPermissionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelChangeAuthorPermissionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelChangeAuthorPermissionResponse clone() => ChannelChangeAuthorPermissionResponse()..mergeFromMessage(this);
  ChannelChangeAuthorPermissionResponse copyWith(void Function(ChannelChangeAuthorPermissionResponse) updates) => super.copyWith((message) => updates(message as ChannelChangeAuthorPermissionResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelChangeAuthorPermissionResponse create() => ChannelChangeAuthorPermissionResponse._();
  ChannelChangeAuthorPermissionResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelChangeAuthorPermissionResponse> createRepeated() => $pb.PbList<ChannelChangeAuthorPermissionResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelChangeAuthorPermissionResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelChangeAuthorPermissionResponse>(create);
  static ChannelChangeAuthorPermissionResponse _defaultInstance;
}

class ChannelRemoveAuthorParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelRemoveAuthorParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelRemoveAuthorParam._() : super();
  factory ChannelRemoveAuthorParam() => create();
  factory ChannelRemoveAuthorParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelRemoveAuthorParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelRemoveAuthorParam clone() => ChannelRemoveAuthorParam()..mergeFromMessage(this);
  ChannelRemoveAuthorParam copyWith(void Function(ChannelRemoveAuthorParam) updates) => super.copyWith((message) => updates(message as ChannelRemoveAuthorParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelRemoveAuthorParam create() => ChannelRemoveAuthorParam._();
  ChannelRemoveAuthorParam createEmptyInstance() => create();
  static $pb.PbList<ChannelRemoveAuthorParam> createRepeated() => $pb.PbList<ChannelRemoveAuthorParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelRemoveAuthorParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelRemoveAuthorParam>(create);
  static ChannelRemoveAuthorParam _defaultInstance;
}

class ChannelRemoveAuthorResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelRemoveAuthorResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelRemoveAuthorResponse._() : super();
  factory ChannelRemoveAuthorResponse() => create();
  factory ChannelRemoveAuthorResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelRemoveAuthorResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelRemoveAuthorResponse clone() => ChannelRemoveAuthorResponse()..mergeFromMessage(this);
  ChannelRemoveAuthorResponse copyWith(void Function(ChannelRemoveAuthorResponse) updates) => super.copyWith((message) => updates(message as ChannelRemoveAuthorResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelRemoveAuthorResponse create() => ChannelRemoveAuthorResponse._();
  ChannelRemoveAuthorResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelRemoveAuthorResponse> createRepeated() => $pb.PbList<ChannelRemoveAuthorResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelRemoveAuthorResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelRemoveAuthorResponse>(create);
  static ChannelRemoveAuthorResponse _defaultInstance;
}

class ChannelFollowChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelFollowChannelParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelFollowChannelParam._() : super();
  factory ChannelFollowChannelParam() => create();
  factory ChannelFollowChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelFollowChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelFollowChannelParam clone() => ChannelFollowChannelParam()..mergeFromMessage(this);
  ChannelFollowChannelParam copyWith(void Function(ChannelFollowChannelParam) updates) => super.copyWith((message) => updates(message as ChannelFollowChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelFollowChannelParam create() => ChannelFollowChannelParam._();
  ChannelFollowChannelParam createEmptyInstance() => create();
  static $pb.PbList<ChannelFollowChannelParam> createRepeated() => $pb.PbList<ChannelFollowChannelParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelFollowChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelFollowChannelParam>(create);
  static ChannelFollowChannelParam _defaultInstance;
}

class ChannelFollowChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelFollowChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelFollowChannelResponse._() : super();
  factory ChannelFollowChannelResponse() => create();
  factory ChannelFollowChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelFollowChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelFollowChannelResponse clone() => ChannelFollowChannelResponse()..mergeFromMessage(this);
  ChannelFollowChannelResponse copyWith(void Function(ChannelFollowChannelResponse) updates) => super.copyWith((message) => updates(message as ChannelFollowChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelFollowChannelResponse create() => ChannelFollowChannelResponse._();
  ChannelFollowChannelResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelFollowChannelResponse> createRepeated() => $pb.PbList<ChannelFollowChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelFollowChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelFollowChannelResponse>(create);
  static ChannelFollowChannelResponse _defaultInstance;
}

class ChannelUnFollowChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelUnFollowChannelParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelUnFollowChannelParam._() : super();
  factory ChannelUnFollowChannelParam() => create();
  factory ChannelUnFollowChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelUnFollowChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelUnFollowChannelParam clone() => ChannelUnFollowChannelParam()..mergeFromMessage(this);
  ChannelUnFollowChannelParam copyWith(void Function(ChannelUnFollowChannelParam) updates) => super.copyWith((message) => updates(message as ChannelUnFollowChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelUnFollowChannelParam create() => ChannelUnFollowChannelParam._();
  ChannelUnFollowChannelParam createEmptyInstance() => create();
  static $pb.PbList<ChannelUnFollowChannelParam> createRepeated() => $pb.PbList<ChannelUnFollowChannelParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelUnFollowChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelUnFollowChannelParam>(create);
  static ChannelUnFollowChannelParam _defaultInstance;
}

class ChannelUnFollowChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelUnFollowChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelUnFollowChannelResponse._() : super();
  factory ChannelUnFollowChannelResponse() => create();
  factory ChannelUnFollowChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelUnFollowChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelUnFollowChannelResponse clone() => ChannelUnFollowChannelResponse()..mergeFromMessage(this);
  ChannelUnFollowChannelResponse copyWith(void Function(ChannelUnFollowChannelResponse) updates) => super.copyWith((message) => updates(message as ChannelUnFollowChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelUnFollowChannelResponse create() => ChannelUnFollowChannelResponse._();
  ChannelUnFollowChannelResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelUnFollowChannelResponse> createRepeated() => $pb.PbList<ChannelUnFollowChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelUnFollowChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelUnFollowChannelResponse>(create);
  static ChannelUnFollowChannelResponse _defaultInstance;
}

class ChannelRemoveFollowersParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelRemoveFollowersParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelRemoveFollowersParam._() : super();
  factory ChannelRemoveFollowersParam() => create();
  factory ChannelRemoveFollowersParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelRemoveFollowersParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelRemoveFollowersParam clone() => ChannelRemoveFollowersParam()..mergeFromMessage(this);
  ChannelRemoveFollowersParam copyWith(void Function(ChannelRemoveFollowersParam) updates) => super.copyWith((message) => updates(message as ChannelRemoveFollowersParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelRemoveFollowersParam create() => ChannelRemoveFollowersParam._();
  ChannelRemoveFollowersParam createEmptyInstance() => create();
  static $pb.PbList<ChannelRemoveFollowersParam> createRepeated() => $pb.PbList<ChannelRemoveFollowersParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelRemoveFollowersParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelRemoveFollowersParam>(create);
  static ChannelRemoveFollowersParam _defaultInstance;
}

class ChannelRemoveFollowersResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelRemoveFollowersResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelRemoveFollowersResponse._() : super();
  factory ChannelRemoveFollowersResponse() => create();
  factory ChannelRemoveFollowersResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelRemoveFollowersResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelRemoveFollowersResponse clone() => ChannelRemoveFollowersResponse()..mergeFromMessage(this);
  ChannelRemoveFollowersResponse copyWith(void Function(ChannelRemoveFollowersResponse) updates) => super.copyWith((message) => updates(message as ChannelRemoveFollowersResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelRemoveFollowersResponse create() => ChannelRemoveFollowersResponse._();
  ChannelRemoveFollowersResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelRemoveFollowersResponse> createRepeated() => $pb.PbList<ChannelRemoveFollowersResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelRemoveFollowersResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelRemoveFollowersResponse>(create);
  static ChannelRemoveFollowersResponse _defaultInstance;
}

class ChannelSubscribeParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelSubscribeParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelSubscribeParam._() : super();
  factory ChannelSubscribeParam() => create();
  factory ChannelSubscribeParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelSubscribeParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelSubscribeParam clone() => ChannelSubscribeParam()..mergeFromMessage(this);
  ChannelSubscribeParam copyWith(void Function(ChannelSubscribeParam) updates) => super.copyWith((message) => updates(message as ChannelSubscribeParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelSubscribeParam create() => ChannelSubscribeParam._();
  ChannelSubscribeParam createEmptyInstance() => create();
  static $pb.PbList<ChannelSubscribeParam> createRepeated() => $pb.PbList<ChannelSubscribeParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelSubscribeParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelSubscribeParam>(create);
  static ChannelSubscribeParam _defaultInstance;
}

class ChannelSubscribeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelSubscribeResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelSubscribeResponse._() : super();
  factory ChannelSubscribeResponse() => create();
  factory ChannelSubscribeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelSubscribeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelSubscribeResponse clone() => ChannelSubscribeResponse()..mergeFromMessage(this);
  ChannelSubscribeResponse copyWith(void Function(ChannelSubscribeResponse) updates) => super.copyWith((message) => updates(message as ChannelSubscribeResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelSubscribeResponse create() => ChannelSubscribeResponse._();
  ChannelSubscribeResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelSubscribeResponse> createRepeated() => $pb.PbList<ChannelSubscribeResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelSubscribeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelSubscribeResponse>(create);
  static ChannelSubscribeResponse _defaultInstance;
}

class ChannelUnSubscribeParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelUnSubscribeParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelUnSubscribeParam._() : super();
  factory ChannelUnSubscribeParam() => create();
  factory ChannelUnSubscribeParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelUnSubscribeParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelUnSubscribeParam clone() => ChannelUnSubscribeParam()..mergeFromMessage(this);
  ChannelUnSubscribeParam copyWith(void Function(ChannelUnSubscribeParam) updates) => super.copyWith((message) => updates(message as ChannelUnSubscribeParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelUnSubscribeParam create() => ChannelUnSubscribeParam._();
  ChannelUnSubscribeParam createEmptyInstance() => create();
  static $pb.PbList<ChannelUnSubscribeParam> createRepeated() => $pb.PbList<ChannelUnSubscribeParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelUnSubscribeParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelUnSubscribeParam>(create);
  static ChannelUnSubscribeParam _defaultInstance;
}

class ChannelUnSubscribeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelUnSubscribeResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelUnSubscribeResponse._() : super();
  factory ChannelUnSubscribeResponse() => create();
  factory ChannelUnSubscribeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelUnSubscribeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelUnSubscribeResponse clone() => ChannelUnSubscribeResponse()..mergeFromMessage(this);
  ChannelUnSubscribeResponse copyWith(void Function(ChannelUnSubscribeResponse) updates) => super.copyWith((message) => updates(message as ChannelUnSubscribeResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelUnSubscribeResponse create() => ChannelUnSubscribeResponse._();
  ChannelUnSubscribeResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelUnSubscribeResponse> createRepeated() => $pb.PbList<ChannelUnSubscribeResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelUnSubscribeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelUnSubscribeResponse>(create);
  static ChannelUnSubscribeResponse _defaultInstance;
}

class ChannelRemoveSubscribersParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelRemoveSubscribersParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelRemoveSubscribersParam._() : super();
  factory ChannelRemoveSubscribersParam() => create();
  factory ChannelRemoveSubscribersParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelRemoveSubscribersParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelRemoveSubscribersParam clone() => ChannelRemoveSubscribersParam()..mergeFromMessage(this);
  ChannelRemoveSubscribersParam copyWith(void Function(ChannelRemoveSubscribersParam) updates) => super.copyWith((message) => updates(message as ChannelRemoveSubscribersParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelRemoveSubscribersParam create() => ChannelRemoveSubscribersParam._();
  ChannelRemoveSubscribersParam createEmptyInstance() => create();
  static $pb.PbList<ChannelRemoveSubscribersParam> createRepeated() => $pb.PbList<ChannelRemoveSubscribersParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelRemoveSubscribersParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelRemoveSubscribersParam>(create);
  static ChannelRemoveSubscribersParam _defaultInstance;
}

class ChannelRemoveSubscribersResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelRemoveSubscribersResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelRemoveSubscribersResponse._() : super();
  factory ChannelRemoveSubscribersResponse() => create();
  factory ChannelRemoveSubscribersResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelRemoveSubscribersResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelRemoveSubscribersResponse clone() => ChannelRemoveSubscribersResponse()..mergeFromMessage(this);
  ChannelRemoveSubscribersResponse copyWith(void Function(ChannelRemoveSubscribersResponse) updates) => super.copyWith((message) => updates(message as ChannelRemoveSubscribersResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelRemoveSubscribersResponse create() => ChannelRemoveSubscribersResponse._();
  ChannelRemoveSubscribersResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelRemoveSubscribersResponse> createRepeated() => $pb.PbList<ChannelRemoveSubscribersResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelRemoveSubscribersResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelRemoveSubscribersResponse>(create);
  static ChannelRemoveSubscribersResponse _defaultInstance;
}

class ChannelChangePrivacyParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelChangePrivacyParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelChangePrivacyParam._() : super();
  factory ChannelChangePrivacyParam() => create();
  factory ChannelChangePrivacyParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelChangePrivacyParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelChangePrivacyParam clone() => ChannelChangePrivacyParam()..mergeFromMessage(this);
  ChannelChangePrivacyParam copyWith(void Function(ChannelChangePrivacyParam) updates) => super.copyWith((message) => updates(message as ChannelChangePrivacyParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelChangePrivacyParam create() => ChannelChangePrivacyParam._();
  ChannelChangePrivacyParam createEmptyInstance() => create();
  static $pb.PbList<ChannelChangePrivacyParam> createRepeated() => $pb.PbList<ChannelChangePrivacyParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelChangePrivacyParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelChangePrivacyParam>(create);
  static ChannelChangePrivacyParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class ChannelChangePrivacyResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelChangePrivacyResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelChangePrivacyResponse._() : super();
  factory ChannelChangePrivacyResponse() => create();
  factory ChannelChangePrivacyResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelChangePrivacyResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelChangePrivacyResponse clone() => ChannelChangePrivacyResponse()..mergeFromMessage(this);
  ChannelChangePrivacyResponse copyWith(void Function(ChannelChangePrivacyResponse) updates) => super.copyWith((message) => updates(message as ChannelChangePrivacyResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelChangePrivacyResponse create() => ChannelChangePrivacyResponse._();
  ChannelChangePrivacyResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelChangePrivacyResponse> createRepeated() => $pb.PbList<ChannelChangePrivacyResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelChangePrivacyResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelChangePrivacyResponse>(create);
  static ChannelChangePrivacyResponse _defaultInstance;
}

class ChannelChangeDefaultPermissionParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelChangeDefaultPermissionParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelChangeDefaultPermissionParam._() : super();
  factory ChannelChangeDefaultPermissionParam() => create();
  factory ChannelChangeDefaultPermissionParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelChangeDefaultPermissionParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelChangeDefaultPermissionParam clone() => ChannelChangeDefaultPermissionParam()..mergeFromMessage(this);
  ChannelChangeDefaultPermissionParam copyWith(void Function(ChannelChangeDefaultPermissionParam) updates) => super.copyWith((message) => updates(message as ChannelChangeDefaultPermissionParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelChangeDefaultPermissionParam create() => ChannelChangeDefaultPermissionParam._();
  ChannelChangeDefaultPermissionParam createEmptyInstance() => create();
  static $pb.PbList<ChannelChangeDefaultPermissionParam> createRepeated() => $pb.PbList<ChannelChangeDefaultPermissionParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelChangeDefaultPermissionParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelChangeDefaultPermissionParam>(create);
  static ChannelChangeDefaultPermissionParam _defaultInstance;
}

class ChannelChangeDefaultPermissionResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelChangeDefaultPermissionResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelChangeDefaultPermissionResponse._() : super();
  factory ChannelChangeDefaultPermissionResponse() => create();
  factory ChannelChangeDefaultPermissionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelChangeDefaultPermissionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelChangeDefaultPermissionResponse clone() => ChannelChangeDefaultPermissionResponse()..mergeFromMessage(this);
  ChannelChangeDefaultPermissionResponse copyWith(void Function(ChannelChangeDefaultPermissionResponse) updates) => super.copyWith((message) => updates(message as ChannelChangeDefaultPermissionResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelChangeDefaultPermissionResponse create() => ChannelChangeDefaultPermissionResponse._();
  ChannelChangeDefaultPermissionResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelChangeDefaultPermissionResponse> createRepeated() => $pb.PbList<ChannelChangeDefaultPermissionResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelChangeDefaultPermissionResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelChangeDefaultPermissionResponse>(create);
  static ChannelChangeDefaultPermissionResponse _defaultInstance;
}

class ChannelRevokeLinkParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelRevokeLinkParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelRevokeLinkParam._() : super();
  factory ChannelRevokeLinkParam() => create();
  factory ChannelRevokeLinkParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelRevokeLinkParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelRevokeLinkParam clone() => ChannelRevokeLinkParam()..mergeFromMessage(this);
  ChannelRevokeLinkParam copyWith(void Function(ChannelRevokeLinkParam) updates) => super.copyWith((message) => updates(message as ChannelRevokeLinkParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelRevokeLinkParam create() => ChannelRevokeLinkParam._();
  ChannelRevokeLinkParam createEmptyInstance() => create();
  static $pb.PbList<ChannelRevokeLinkParam> createRepeated() => $pb.PbList<ChannelRevokeLinkParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelRevokeLinkParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelRevokeLinkParam>(create);
  static ChannelRevokeLinkParam _defaultInstance;
}

class ChannelRevokeLinkResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelRevokeLinkResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelRevokeLinkResponse._() : super();
  factory ChannelRevokeLinkResponse() => create();
  factory ChannelRevokeLinkResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelRevokeLinkResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelRevokeLinkResponse clone() => ChannelRevokeLinkResponse()..mergeFromMessage(this);
  ChannelRevokeLinkResponse copyWith(void Function(ChannelRevokeLinkResponse) updates) => super.copyWith((message) => updates(message as ChannelRevokeLinkResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelRevokeLinkResponse create() => ChannelRevokeLinkResponse._();
  ChannelRevokeLinkResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelRevokeLinkResponse> createRepeated() => $pb.PbList<ChannelRevokeLinkResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelRevokeLinkResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelRevokeLinkResponse>(create);
  static ChannelRevokeLinkResponse _defaultInstance;
}

class ChannelChangeUsernameParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelChangeUsernameParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelChangeUsernameParam._() : super();
  factory ChannelChangeUsernameParam() => create();
  factory ChannelChangeUsernameParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelChangeUsernameParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelChangeUsernameParam clone() => ChannelChangeUsernameParam()..mergeFromMessage(this);
  ChannelChangeUsernameParam copyWith(void Function(ChannelChangeUsernameParam) updates) => super.copyWith((message) => updates(message as ChannelChangeUsernameParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelChangeUsernameParam create() => ChannelChangeUsernameParam._();
  ChannelChangeUsernameParam createEmptyInstance() => create();
  static $pb.PbList<ChannelChangeUsernameParam> createRepeated() => $pb.PbList<ChannelChangeUsernameParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelChangeUsernameParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelChangeUsernameParam>(create);
  static ChannelChangeUsernameParam _defaultInstance;
}

class ChannelChangeUsernameResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelChangeUsernameResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelChangeUsernameResponse._() : super();
  factory ChannelChangeUsernameResponse() => create();
  factory ChannelChangeUsernameResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelChangeUsernameResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelChangeUsernameResponse clone() => ChannelChangeUsernameResponse()..mergeFromMessage(this);
  ChannelChangeUsernameResponse copyWith(void Function(ChannelChangeUsernameResponse) updates) => super.copyWith((message) => updates(message as ChannelChangeUsernameResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelChangeUsernameResponse create() => ChannelChangeUsernameResponse._();
  ChannelChangeUsernameResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelChangeUsernameResponse> createRepeated() => $pb.PbList<ChannelChangeUsernameResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelChangeUsernameResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelChangeUsernameResponse>(create);
  static ChannelChangeUsernameResponse _defaultInstance;
}

class ChannelBlockChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelBlockChannelParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelBlockChannelParam._() : super();
  factory ChannelBlockChannelParam() => create();
  factory ChannelBlockChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelBlockChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelBlockChannelParam clone() => ChannelBlockChannelParam()..mergeFromMessage(this);
  ChannelBlockChannelParam copyWith(void Function(ChannelBlockChannelParam) updates) => super.copyWith((message) => updates(message as ChannelBlockChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelBlockChannelParam create() => ChannelBlockChannelParam._();
  ChannelBlockChannelParam createEmptyInstance() => create();
  static $pb.PbList<ChannelBlockChannelParam> createRepeated() => $pb.PbList<ChannelBlockChannelParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelBlockChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelBlockChannelParam>(create);
  static ChannelBlockChannelParam _defaultInstance;
}

class ChannelBlockChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelBlockChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelBlockChannelResponse._() : super();
  factory ChannelBlockChannelResponse() => create();
  factory ChannelBlockChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelBlockChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelBlockChannelResponse clone() => ChannelBlockChannelResponse()..mergeFromMessage(this);
  ChannelBlockChannelResponse copyWith(void Function(ChannelBlockChannelResponse) updates) => super.copyWith((message) => updates(message as ChannelBlockChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelBlockChannelResponse create() => ChannelBlockChannelResponse._();
  ChannelBlockChannelResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelBlockChannelResponse> createRepeated() => $pb.PbList<ChannelBlockChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelBlockChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelBlockChannelResponse>(create);
  static ChannelBlockChannelResponse _defaultInstance;
}

class ChannelDeleteMessagesParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelDeleteMessagesParam', createEmptyInstance: create)
    ..a<$core.int>(1, 'groupId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelDeleteMessagesParam._() : super();
  factory ChannelDeleteMessagesParam() => create();
  factory ChannelDeleteMessagesParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelDeleteMessagesParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelDeleteMessagesParam clone() => ChannelDeleteMessagesParam()..mergeFromMessage(this);
  ChannelDeleteMessagesParam copyWith(void Function(ChannelDeleteMessagesParam) updates) => super.copyWith((message) => updates(message as ChannelDeleteMessagesParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelDeleteMessagesParam create() => ChannelDeleteMessagesParam._();
  ChannelDeleteMessagesParam createEmptyInstance() => create();
  static $pb.PbList<ChannelDeleteMessagesParam> createRepeated() => $pb.PbList<ChannelDeleteMessagesParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelDeleteMessagesParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelDeleteMessagesParam>(create);
  static ChannelDeleteMessagesParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get groupId => $_getIZ(0);
  @$pb.TagNumber(1)
  set groupId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);
}

class ChannelDeleteMessagesResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelDeleteMessagesResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelDeleteMessagesResponse._() : super();
  factory ChannelDeleteMessagesResponse() => create();
  factory ChannelDeleteMessagesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelDeleteMessagesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelDeleteMessagesResponse clone() => ChannelDeleteMessagesResponse()..mergeFromMessage(this);
  ChannelDeleteMessagesResponse copyWith(void Function(ChannelDeleteMessagesResponse) updates) => super.copyWith((message) => updates(message as ChannelDeleteMessagesResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelDeleteMessagesResponse create() => ChannelDeleteMessagesResponse._();
  ChannelDeleteMessagesResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelDeleteMessagesResponse> createRepeated() => $pb.PbList<ChannelDeleteMessagesResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelDeleteMessagesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelDeleteMessagesResponse>(create);
  static ChannelDeleteMessagesResponse _defaultInstance;
}

class ChannelClearHistoryParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelClearHistoryParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelClearHistoryParam._() : super();
  factory ChannelClearHistoryParam() => create();
  factory ChannelClearHistoryParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelClearHistoryParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelClearHistoryParam clone() => ChannelClearHistoryParam()..mergeFromMessage(this);
  ChannelClearHistoryParam copyWith(void Function(ChannelClearHistoryParam) updates) => super.copyWith((message) => updates(message as ChannelClearHistoryParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelClearHistoryParam create() => ChannelClearHistoryParam._();
  ChannelClearHistoryParam createEmptyInstance() => create();
  static $pb.PbList<ChannelClearHistoryParam> createRepeated() => $pb.PbList<ChannelClearHistoryParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelClearHistoryParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelClearHistoryParam>(create);
  static ChannelClearHistoryParam _defaultInstance;
}

class ChannelClearHistoryResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelClearHistoryResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelClearHistoryResponse._() : super();
  factory ChannelClearHistoryResponse() => create();
  factory ChannelClearHistoryResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelClearHistoryResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelClearHistoryResponse clone() => ChannelClearHistoryResponse()..mergeFromMessage(this);
  ChannelClearHistoryResponse copyWith(void Function(ChannelClearHistoryResponse) updates) => super.copyWith((message) => updates(message as ChannelClearHistoryResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelClearHistoryResponse create() => ChannelClearHistoryResponse._();
  ChannelClearHistoryResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelClearHistoryResponse> createRepeated() => $pb.PbList<ChannelClearHistoryResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelClearHistoryResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelClearHistoryResponse>(create);
  static ChannelClearHistoryResponse _defaultInstance;
}

class ChannelAvatarChangeParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelAvatarChangeParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelAvatarChangeParam._() : super();
  factory ChannelAvatarChangeParam() => create();
  factory ChannelAvatarChangeParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelAvatarChangeParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelAvatarChangeParam clone() => ChannelAvatarChangeParam()..mergeFromMessage(this);
  ChannelAvatarChangeParam copyWith(void Function(ChannelAvatarChangeParam) updates) => super.copyWith((message) => updates(message as ChannelAvatarChangeParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarChangeParam create() => ChannelAvatarChangeParam._();
  ChannelAvatarChangeParam createEmptyInstance() => create();
  static $pb.PbList<ChannelAvatarChangeParam> createRepeated() => $pb.PbList<ChannelAvatarChangeParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarChangeParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelAvatarChangeParam>(create);
  static ChannelAvatarChangeParam _defaultInstance;
}

class ChannelAvatarChangeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelAvatarChangeResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelAvatarChangeResponse._() : super();
  factory ChannelAvatarChangeResponse() => create();
  factory ChannelAvatarChangeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelAvatarChangeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelAvatarChangeResponse clone() => ChannelAvatarChangeResponse()..mergeFromMessage(this);
  ChannelAvatarChangeResponse copyWith(void Function(ChannelAvatarChangeResponse) updates) => super.copyWith((message) => updates(message as ChannelAvatarChangeResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarChangeResponse create() => ChannelAvatarChangeResponse._();
  ChannelAvatarChangeResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelAvatarChangeResponse> createRepeated() => $pb.PbList<ChannelAvatarChangeResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelAvatarChangeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelAvatarChangeResponse>(create);
  static ChannelAvatarChangeResponse _defaultInstance;
}

class ChannelSendDoingActionParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelSendDoingActionParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelSendDoingActionParam._() : super();
  factory ChannelSendDoingActionParam() => create();
  factory ChannelSendDoingActionParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelSendDoingActionParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelSendDoingActionParam clone() => ChannelSendDoingActionParam()..mergeFromMessage(this);
  ChannelSendDoingActionParam copyWith(void Function(ChannelSendDoingActionParam) updates) => super.copyWith((message) => updates(message as ChannelSendDoingActionParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelSendDoingActionParam create() => ChannelSendDoingActionParam._();
  ChannelSendDoingActionParam createEmptyInstance() => create();
  static $pb.PbList<ChannelSendDoingActionParam> createRepeated() => $pb.PbList<ChannelSendDoingActionParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelSendDoingActionParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelSendDoingActionParam>(create);
  static ChannelSendDoingActionParam _defaultInstance;
}

class ChannelSendDoingActionResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelSendDoingActionResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelSendDoingActionResponse._() : super();
  factory ChannelSendDoingActionResponse() => create();
  factory ChannelSendDoingActionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelSendDoingActionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelSendDoingActionResponse clone() => ChannelSendDoingActionResponse()..mergeFromMessage(this);
  ChannelSendDoingActionResponse copyWith(void Function(ChannelSendDoingActionResponse) updates) => super.copyWith((message) => updates(message as ChannelSendDoingActionResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelSendDoingActionResponse create() => ChannelSendDoingActionResponse._();
  ChannelSendDoingActionResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelSendDoingActionResponse> createRepeated() => $pb.PbList<ChannelSendDoingActionResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelSendDoingActionResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelSendDoingActionResponse>(create);
  static ChannelSendDoingActionResponse _defaultInstance;
}

class ChannelReportChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelReportChannelParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelReportChannelParam._() : super();
  factory ChannelReportChannelParam() => create();
  factory ChannelReportChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelReportChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelReportChannelParam clone() => ChannelReportChannelParam()..mergeFromMessage(this);
  ChannelReportChannelParam copyWith(void Function(ChannelReportChannelParam) updates) => super.copyWith((message) => updates(message as ChannelReportChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelReportChannelParam create() => ChannelReportChannelParam._();
  ChannelReportChannelParam createEmptyInstance() => create();
  static $pb.PbList<ChannelReportChannelParam> createRepeated() => $pb.PbList<ChannelReportChannelParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelReportChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelReportChannelParam>(create);
  static ChannelReportChannelParam _defaultInstance;
}

class ChannelReportChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelReportChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelReportChannelResponse._() : super();
  factory ChannelReportChannelResponse() => create();
  factory ChannelReportChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelReportChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelReportChannelResponse clone() => ChannelReportChannelResponse()..mergeFromMessage(this);
  ChannelReportChannelResponse copyWith(void Function(ChannelReportChannelResponse) updates) => super.copyWith((message) => updates(message as ChannelReportChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelReportChannelResponse create() => ChannelReportChannelResponse._();
  ChannelReportChannelResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelReportChannelResponse> createRepeated() => $pb.PbList<ChannelReportChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelReportChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelReportChannelResponse>(create);
  static ChannelReportChannelResponse _defaultInstance;
}

class ChannelReportMessageParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelReportMessageParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelReportMessageParam._() : super();
  factory ChannelReportMessageParam() => create();
  factory ChannelReportMessageParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelReportMessageParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelReportMessageParam clone() => ChannelReportMessageParam()..mergeFromMessage(this);
  ChannelReportMessageParam copyWith(void Function(ChannelReportMessageParam) updates) => super.copyWith((message) => updates(message as ChannelReportMessageParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelReportMessageParam create() => ChannelReportMessageParam._();
  ChannelReportMessageParam createEmptyInstance() => create();
  static $pb.PbList<ChannelReportMessageParam> createRepeated() => $pb.PbList<ChannelReportMessageParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelReportMessageParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelReportMessageParam>(create);
  static ChannelReportMessageParam _defaultInstance;
}

class ChannelReportMessageResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelReportMessageResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelReportMessageResponse._() : super();
  factory ChannelReportMessageResponse() => create();
  factory ChannelReportMessageResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelReportMessageResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelReportMessageResponse clone() => ChannelReportMessageResponse()..mergeFromMessage(this);
  ChannelReportMessageResponse copyWith(void Function(ChannelReportMessageResponse) updates) => super.copyWith((message) => updates(message as ChannelReportMessageResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelReportMessageResponse create() => ChannelReportMessageResponse._();
  ChannelReportMessageResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelReportMessageResponse> createRepeated() => $pb.PbList<ChannelReportMessageResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelReportMessageResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelReportMessageResponse>(create);
  static ChannelReportMessageResponse _defaultInstance;
}

class ChannelGetFullParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetFullParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetFullParam._() : super();
  factory ChannelGetFullParam() => create();
  factory ChannelGetFullParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetFullParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetFullParam clone() => ChannelGetFullParam()..mergeFromMessage(this);
  ChannelGetFullParam copyWith(void Function(ChannelGetFullParam) updates) => super.copyWith((message) => updates(message as ChannelGetFullParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetFullParam create() => ChannelGetFullParam._();
  ChannelGetFullParam createEmptyInstance() => create();
  static $pb.PbList<ChannelGetFullParam> createRepeated() => $pb.PbList<ChannelGetFullParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetFullParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetFullParam>(create);
  static ChannelGetFullParam _defaultInstance;
}

class ChannelGetFullResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetFullResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetFullResponse._() : super();
  factory ChannelGetFullResponse() => create();
  factory ChannelGetFullResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetFullResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetFullResponse clone() => ChannelGetFullResponse()..mergeFromMessage(this);
  ChannelGetFullResponse copyWith(void Function(ChannelGetFullResponse) updates) => super.copyWith((message) => updates(message as ChannelGetFullResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetFullResponse create() => ChannelGetFullResponse._();
  ChannelGetFullResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelGetFullResponse> createRepeated() => $pb.PbList<ChannelGetFullResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetFullResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetFullResponse>(create);
  static ChannelGetFullResponse _defaultInstance;
}

class ChannelGetMessagesListParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetMessagesListParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetMessagesListParam._() : super();
  factory ChannelGetMessagesListParam() => create();
  factory ChannelGetMessagesListParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetMessagesListParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetMessagesListParam clone() => ChannelGetMessagesListParam()..mergeFromMessage(this);
  ChannelGetMessagesListParam copyWith(void Function(ChannelGetMessagesListParam) updates) => super.copyWith((message) => updates(message as ChannelGetMessagesListParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetMessagesListParam create() => ChannelGetMessagesListParam._();
  ChannelGetMessagesListParam createEmptyInstance() => create();
  static $pb.PbList<ChannelGetMessagesListParam> createRepeated() => $pb.PbList<ChannelGetMessagesListParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetMessagesListParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetMessagesListParam>(create);
  static ChannelGetMessagesListParam _defaultInstance;
}

class ChannelGetMessagesListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetMessagesListResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetMessagesListResponse._() : super();
  factory ChannelGetMessagesListResponse() => create();
  factory ChannelGetMessagesListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetMessagesListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetMessagesListResponse clone() => ChannelGetMessagesListResponse()..mergeFromMessage(this);
  ChannelGetMessagesListResponse copyWith(void Function(ChannelGetMessagesListResponse) updates) => super.copyWith((message) => updates(message as ChannelGetMessagesListResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetMessagesListResponse create() => ChannelGetMessagesListResponse._();
  ChannelGetMessagesListResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelGetMessagesListResponse> createRepeated() => $pb.PbList<ChannelGetMessagesListResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetMessagesListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetMessagesListResponse>(create);
  static ChannelGetMessagesListResponse _defaultInstance;
}

class ChannelGetMediaListParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetMediaListParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetMediaListParam._() : super();
  factory ChannelGetMediaListParam() => create();
  factory ChannelGetMediaListParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetMediaListParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetMediaListParam clone() => ChannelGetMediaListParam()..mergeFromMessage(this);
  ChannelGetMediaListParam copyWith(void Function(ChannelGetMediaListParam) updates) => super.copyWith((message) => updates(message as ChannelGetMediaListParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetMediaListParam create() => ChannelGetMediaListParam._();
  ChannelGetMediaListParam createEmptyInstance() => create();
  static $pb.PbList<ChannelGetMediaListParam> createRepeated() => $pb.PbList<ChannelGetMediaListParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetMediaListParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetMediaListParam>(create);
  static ChannelGetMediaListParam _defaultInstance;
}

class ChannelGetMediaListResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetMediaListResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetMediaListResponse._() : super();
  factory ChannelGetMediaListResponse() => create();
  factory ChannelGetMediaListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetMediaListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetMediaListResponse clone() => ChannelGetMediaListResponse()..mergeFromMessage(this);
  ChannelGetMediaListResponse copyWith(void Function(ChannelGetMediaListResponse) updates) => super.copyWith((message) => updates(message as ChannelGetMediaListResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetMediaListResponse create() => ChannelGetMediaListResponse._();
  ChannelGetMediaListResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelGetMediaListResponse> createRepeated() => $pb.PbList<ChannelGetMediaListResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetMediaListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetMediaListResponse>(create);
  static ChannelGetMediaListResponse _defaultInstance;
}

class ChannelGetAuthorsParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetAuthorsParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetAuthorsParam._() : super();
  factory ChannelGetAuthorsParam() => create();
  factory ChannelGetAuthorsParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetAuthorsParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetAuthorsParam clone() => ChannelGetAuthorsParam()..mergeFromMessage(this);
  ChannelGetAuthorsParam copyWith(void Function(ChannelGetAuthorsParam) updates) => super.copyWith((message) => updates(message as ChannelGetAuthorsParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetAuthorsParam create() => ChannelGetAuthorsParam._();
  ChannelGetAuthorsParam createEmptyInstance() => create();
  static $pb.PbList<ChannelGetAuthorsParam> createRepeated() => $pb.PbList<ChannelGetAuthorsParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetAuthorsParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetAuthorsParam>(create);
  static ChannelGetAuthorsParam _defaultInstance;
}

class ChannelGetAuthorsResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetAuthorsResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetAuthorsResponse._() : super();
  factory ChannelGetAuthorsResponse() => create();
  factory ChannelGetAuthorsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetAuthorsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetAuthorsResponse clone() => ChannelGetAuthorsResponse()..mergeFromMessage(this);
  ChannelGetAuthorsResponse copyWith(void Function(ChannelGetAuthorsResponse) updates) => super.copyWith((message) => updates(message as ChannelGetAuthorsResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetAuthorsResponse create() => ChannelGetAuthorsResponse._();
  ChannelGetAuthorsResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelGetAuthorsResponse> createRepeated() => $pb.PbList<ChannelGetAuthorsResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetAuthorsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetAuthorsResponse>(create);
  static ChannelGetAuthorsResponse _defaultInstance;
}

class ChannelGetFollowersParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetFollowersParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetFollowersParam._() : super();
  factory ChannelGetFollowersParam() => create();
  factory ChannelGetFollowersParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetFollowersParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetFollowersParam clone() => ChannelGetFollowersParam()..mergeFromMessage(this);
  ChannelGetFollowersParam copyWith(void Function(ChannelGetFollowersParam) updates) => super.copyWith((message) => updates(message as ChannelGetFollowersParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetFollowersParam create() => ChannelGetFollowersParam._();
  ChannelGetFollowersParam createEmptyInstance() => create();
  static $pb.PbList<ChannelGetFollowersParam> createRepeated() => $pb.PbList<ChannelGetFollowersParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetFollowersParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetFollowersParam>(create);
  static ChannelGetFollowersParam _defaultInstance;
}

class ChannelGetFollowersResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetFollowersResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetFollowersResponse._() : super();
  factory ChannelGetFollowersResponse() => create();
  factory ChannelGetFollowersResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetFollowersResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetFollowersResponse clone() => ChannelGetFollowersResponse()..mergeFromMessage(this);
  ChannelGetFollowersResponse copyWith(void Function(ChannelGetFollowersResponse) updates) => super.copyWith((message) => updates(message as ChannelGetFollowersResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetFollowersResponse create() => ChannelGetFollowersResponse._();
  ChannelGetFollowersResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelGetFollowersResponse> createRepeated() => $pb.PbList<ChannelGetFollowersResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetFollowersResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetFollowersResponse>(create);
  static ChannelGetFollowersResponse _defaultInstance;
}

class ChannelGetFollowingsParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetFollowingsParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetFollowingsParam._() : super();
  factory ChannelGetFollowingsParam() => create();
  factory ChannelGetFollowingsParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetFollowingsParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetFollowingsParam clone() => ChannelGetFollowingsParam()..mergeFromMessage(this);
  ChannelGetFollowingsParam copyWith(void Function(ChannelGetFollowingsParam) updates) => super.copyWith((message) => updates(message as ChannelGetFollowingsParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetFollowingsParam create() => ChannelGetFollowingsParam._();
  ChannelGetFollowingsParam createEmptyInstance() => create();
  static $pb.PbList<ChannelGetFollowingsParam> createRepeated() => $pb.PbList<ChannelGetFollowingsParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetFollowingsParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetFollowingsParam>(create);
  static ChannelGetFollowingsParam _defaultInstance;
}

class ChannelGetFollowingsResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetFollowingsResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetFollowingsResponse._() : super();
  factory ChannelGetFollowingsResponse() => create();
  factory ChannelGetFollowingsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetFollowingsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetFollowingsResponse clone() => ChannelGetFollowingsResponse()..mergeFromMessage(this);
  ChannelGetFollowingsResponse copyWith(void Function(ChannelGetFollowingsResponse) updates) => super.copyWith((message) => updates(message as ChannelGetFollowingsResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetFollowingsResponse create() => ChannelGetFollowingsResponse._();
  ChannelGetFollowingsResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelGetFollowingsResponse> createRepeated() => $pb.PbList<ChannelGetFollowingsResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetFollowingsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetFollowingsResponse>(create);
  static ChannelGetFollowingsResponse _defaultInstance;
}

class ChannelGetSubscribersParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetSubscribersParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetSubscribersParam._() : super();
  factory ChannelGetSubscribersParam() => create();
  factory ChannelGetSubscribersParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetSubscribersParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetSubscribersParam clone() => ChannelGetSubscribersParam()..mergeFromMessage(this);
  ChannelGetSubscribersParam copyWith(void Function(ChannelGetSubscribersParam) updates) => super.copyWith((message) => updates(message as ChannelGetSubscribersParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetSubscribersParam create() => ChannelGetSubscribersParam._();
  ChannelGetSubscribersParam createEmptyInstance() => create();
  static $pb.PbList<ChannelGetSubscribersParam> createRepeated() => $pb.PbList<ChannelGetSubscribersParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetSubscribersParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetSubscribersParam>(create);
  static ChannelGetSubscribersParam _defaultInstance;
}

class ChannelGetSubscribersResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelGetSubscribersResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelGetSubscribersResponse._() : super();
  factory ChannelGetSubscribersResponse() => create();
  factory ChannelGetSubscribersResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelGetSubscribersResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelGetSubscribersResponse clone() => ChannelGetSubscribersResponse()..mergeFromMessage(this);
  ChannelGetSubscribersResponse copyWith(void Function(ChannelGetSubscribersResponse) updates) => super.copyWith((message) => updates(message as ChannelGetSubscribersResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelGetSubscribersResponse create() => ChannelGetSubscribersResponse._();
  ChannelGetSubscribersResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelGetSubscribersResponse> createRepeated() => $pb.PbList<ChannelGetSubscribersResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelGetSubscribersResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelGetSubscribersResponse>(create);
  static ChannelGetSubscribersResponse _defaultInstance;
}

class ChannelBlockedParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelBlockedParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelBlockedParam._() : super();
  factory ChannelBlockedParam() => create();
  factory ChannelBlockedParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelBlockedParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelBlockedParam clone() => ChannelBlockedParam()..mergeFromMessage(this);
  ChannelBlockedParam copyWith(void Function(ChannelBlockedParam) updates) => super.copyWith((message) => updates(message as ChannelBlockedParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelBlockedParam create() => ChannelBlockedParam._();
  ChannelBlockedParam createEmptyInstance() => create();
  static $pb.PbList<ChannelBlockedParam> createRepeated() => $pb.PbList<ChannelBlockedParam>();
  @$core.pragma('dart2js:noInline')
  static ChannelBlockedParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelBlockedParam>(create);
  static ChannelBlockedParam _defaultInstance;
}

class ChannelBlockedResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelBlockedResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChannelBlockedResponse._() : super();
  factory ChannelBlockedResponse() => create();
  factory ChannelBlockedResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelBlockedResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelBlockedResponse clone() => ChannelBlockedResponse()..mergeFromMessage(this);
  ChannelBlockedResponse copyWith(void Function(ChannelBlockedResponse) updates) => super.copyWith((message) => updates(message as ChannelBlockedResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelBlockedResponse create() => ChannelBlockedResponse._();
  ChannelBlockedResponse createEmptyInstance() => create();
  static $pb.PbList<ChannelBlockedResponse> createRepeated() => $pb.PbList<ChannelBlockedResponse>();
  @$core.pragma('dart2js:noInline')
  static ChannelBlockedResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelBlockedResponse>(create);
  static ChannelBlockedResponse _defaultInstance;
}

class RPC_ChannelApi {
  $pb.RpcClient _client;
  RPC_ChannelApi(this._client);

  $async.Future<ChannelCreateChannelResponse> channelCreateChannel($pb.ClientContext ctx, ChannelCreateChannelParam request) {
    var emptyResponse = ChannelCreateChannelResponse();
    return _client.invoke<ChannelCreateChannelResponse>(ctx, 'RPC_Channel', 'ChannelCreateChannel', request, emptyResponse);
  }
  $async.Future<ChannelEditChannelResponse> channelEditChannel($pb.ClientContext ctx, ChannelEditChannelParam request) {
    var emptyResponse = ChannelEditChannelResponse();
    return _client.invoke<ChannelEditChannelResponse>(ctx, 'RPC_Channel', 'ChannelEditChannel', request, emptyResponse);
  }
  $async.Future<ChannelDeleteChannelResponse> channelDeleteChannel($pb.ClientContext ctx, ChannelDeleteChannelParam request) {
    var emptyResponse = ChannelDeleteChannelResponse();
    return _client.invoke<ChannelDeleteChannelResponse>(ctx, 'RPC_Channel', 'ChannelDeleteChannel', request, emptyResponse);
  }
  $async.Future<ChannelAddAuthorResponse> channelAddAuthor($pb.ClientContext ctx, ChannelAddAuthorParam request) {
    var emptyResponse = ChannelAddAuthorResponse();
    return _client.invoke<ChannelAddAuthorResponse>(ctx, 'RPC_Channel', 'ChannelAddAuthor', request, emptyResponse);
  }
  $async.Future<ChannelChangeAuthorPermissionResponse> channelChangeAuthorPermission($pb.ClientContext ctx, ChannelChangeAuthorPermissionParam request) {
    var emptyResponse = ChannelChangeAuthorPermissionResponse();
    return _client.invoke<ChannelChangeAuthorPermissionResponse>(ctx, 'RPC_Channel', 'ChannelChangeAuthorPermission', request, emptyResponse);
  }
  $async.Future<ChannelRemoveAuthorResponse> channelRemoveAuthor($pb.ClientContext ctx, ChannelRemoveAuthorParam request) {
    var emptyResponse = ChannelRemoveAuthorResponse();
    return _client.invoke<ChannelRemoveAuthorResponse>(ctx, 'RPC_Channel', 'ChannelRemoveAuthor', request, emptyResponse);
  }
  $async.Future<ChannelFollowChannelResponse> channelFollowChannel($pb.ClientContext ctx, ChannelFollowChannelParam request) {
    var emptyResponse = ChannelFollowChannelResponse();
    return _client.invoke<ChannelFollowChannelResponse>(ctx, 'RPC_Channel', 'ChannelFollowChannel', request, emptyResponse);
  }
  $async.Future<ChannelUnFollowChannelResponse> channelUnFollowChannel($pb.ClientContext ctx, ChannelUnFollowChannelParam request) {
    var emptyResponse = ChannelUnFollowChannelResponse();
    return _client.invoke<ChannelUnFollowChannelResponse>(ctx, 'RPC_Channel', 'ChannelUnFollowChannel', request, emptyResponse);
  }
  $async.Future<ChannelRemoveFollowersResponse> channelRemoveFollowers($pb.ClientContext ctx, ChannelRemoveFollowersParam request) {
    var emptyResponse = ChannelRemoveFollowersResponse();
    return _client.invoke<ChannelRemoveFollowersResponse>(ctx, 'RPC_Channel', 'ChannelRemoveFollowers', request, emptyResponse);
  }
  $async.Future<ChannelSubscribeResponse> channelSubscribe($pb.ClientContext ctx, ChannelSubscribeParam request) {
    var emptyResponse = ChannelSubscribeResponse();
    return _client.invoke<ChannelSubscribeResponse>(ctx, 'RPC_Channel', 'ChannelSubscribe', request, emptyResponse);
  }
  $async.Future<ChannelUnSubscribeResponse> channelUnSubscribe($pb.ClientContext ctx, ChannelUnSubscribeParam request) {
    var emptyResponse = ChannelUnSubscribeResponse();
    return _client.invoke<ChannelUnSubscribeResponse>(ctx, 'RPC_Channel', 'ChannelUnSubscribe', request, emptyResponse);
  }
  $async.Future<ChannelRemoveSubscribersResponse> channelRemoveSubscribers($pb.ClientContext ctx, ChannelRemoveSubscribersParam request) {
    var emptyResponse = ChannelRemoveSubscribersResponse();
    return _client.invoke<ChannelRemoveSubscribersResponse>(ctx, 'RPC_Channel', 'ChannelRemoveSubscribers', request, emptyResponse);
  }
  $async.Future<ChannelChangePrivacyResponse> channelChangePrivacy($pb.ClientContext ctx, ChannelChangePrivacyParam request) {
    var emptyResponse = ChannelChangePrivacyResponse();
    return _client.invoke<ChannelChangePrivacyResponse>(ctx, 'RPC_Channel', 'ChannelChangePrivacy', request, emptyResponse);
  }
  $async.Future<ChannelChangeDefaultPermissionResponse> channelChangeDefaultPermission($pb.ClientContext ctx, ChannelChangeDefaultPermissionParam request) {
    var emptyResponse = ChannelChangeDefaultPermissionResponse();
    return _client.invoke<ChannelChangeDefaultPermissionResponse>(ctx, 'RPC_Channel', 'ChannelChangeDefaultPermission', request, emptyResponse);
  }
  $async.Future<ChannelRevokeLinkResponse> channelRevokeLink($pb.ClientContext ctx, ChannelRevokeLinkParam request) {
    var emptyResponse = ChannelRevokeLinkResponse();
    return _client.invoke<ChannelRevokeLinkResponse>(ctx, 'RPC_Channel', 'ChannelRevokeLink', request, emptyResponse);
  }
  $async.Future<ChannelChangeUsernameResponse> channelChangeUsername($pb.ClientContext ctx, ChannelChangeUsernameParam request) {
    var emptyResponse = ChannelChangeUsernameResponse();
    return _client.invoke<ChannelChangeUsernameResponse>(ctx, 'RPC_Channel', 'ChannelChangeUsername', request, emptyResponse);
  }
  $async.Future<ChannelBlockChannelResponse> channelBlockChannel($pb.ClientContext ctx, ChannelBlockChannelParam request) {
    var emptyResponse = ChannelBlockChannelResponse();
    return _client.invoke<ChannelBlockChannelResponse>(ctx, 'RPC_Channel', 'ChannelBlockChannel', request, emptyResponse);
  }
  $async.Future<ChannelSendMessageResponse> channelSendMessage($pb.ClientContext ctx, ChannelSendMessageParam request) {
    var emptyResponse = ChannelSendMessageResponse();
    return _client.invoke<ChannelSendMessageResponse>(ctx, 'RPC_Channel', 'ChannelSendMessage', request, emptyResponse);
  }
  $async.Future<ChannelEditMessageResponse> channelEditMessage($pb.ClientContext ctx, ChannelEditMessageParam request) {
    var emptyResponse = ChannelEditMessageResponse();
    return _client.invoke<ChannelEditMessageResponse>(ctx, 'RPC_Channel', 'ChannelEditMessage', request, emptyResponse);
  }
  $async.Future<ChannelPinMessageResponse> channelPinMessage($pb.ClientContext ctx, ChannelPinMessageParam request) {
    var emptyResponse = ChannelPinMessageResponse();
    return _client.invoke<ChannelPinMessageResponse>(ctx, 'RPC_Channel', 'ChannelPinMessage', request, emptyResponse);
  }
  $async.Future<ChannelUnPinMessageResponse> channelUnPinMessage($pb.ClientContext ctx, ChannelUnPinMessageParam request) {
    var emptyResponse = ChannelUnPinMessageResponse();
    return _client.invoke<ChannelUnPinMessageResponse>(ctx, 'RPC_Channel', 'ChannelUnPinMessage', request, emptyResponse);
  }
  $async.Future<ChannelDeleteMessageResponse> channelDeleteMessage($pb.ClientContext ctx, ChannelDeleteMessageParam request) {
    var emptyResponse = ChannelDeleteMessageResponse();
    return _client.invoke<ChannelDeleteMessageResponse>(ctx, 'RPC_Channel', 'ChannelDeleteMessage', request, emptyResponse);
  }
  $async.Future<ChannelDeleteMessagesResponse> channelDeleteMessages($pb.ClientContext ctx, ChannelDeleteMessagesParam request) {
    var emptyResponse = ChannelDeleteMessagesResponse();
    return _client.invoke<ChannelDeleteMessagesResponse>(ctx, 'RPC_Channel', 'ChannelDeleteMessages', request, emptyResponse);
  }
  $async.Future<ChannelClearHistoryResponse> channelClearHistory($pb.ClientContext ctx, ChannelClearHistoryParam request) {
    var emptyResponse = ChannelClearHistoryResponse();
    return _client.invoke<ChannelClearHistoryResponse>(ctx, 'RPC_Channel', 'ChannelClearHistory', request, emptyResponse);
  }
  $async.Future<ChannelAvatarAddResponse> channelAvatarAdd($pb.ClientContext ctx, ChannelAvatarAddParam request) {
    var emptyResponse = ChannelAvatarAddResponse();
    return _client.invoke<ChannelAvatarAddResponse>(ctx, 'RPC_Channel', 'ChannelAvatarAdd', request, emptyResponse);
  }
  $async.Future<ChannelAvatarChangeResponse> channelAvatarChange($pb.ClientContext ctx, ChannelAvatarChangeParam request) {
    var emptyResponse = ChannelAvatarChangeResponse();
    return _client.invoke<ChannelAvatarChangeResponse>(ctx, 'RPC_Channel', 'ChannelAvatarChange', request, emptyResponse);
  }
  $async.Future<ChannelAvatarDeleteResponse> channelAvatarDelete($pb.ClientContext ctx, ChannelAvatarDeleteParam request) {
    var emptyResponse = ChannelAvatarDeleteResponse();
    return _client.invoke<ChannelAvatarDeleteResponse>(ctx, 'RPC_Channel', 'ChannelAvatarDelete', request, emptyResponse);
  }
  $async.Future<ChannelAvatarGetListResponse> channelAvatarGetList($pb.ClientContext ctx, ChannelAvatarGetListParam request) {
    var emptyResponse = ChannelAvatarGetListResponse();
    return _client.invoke<ChannelAvatarGetListResponse>(ctx, 'RPC_Channel', 'ChannelAvatarGetList', request, emptyResponse);
  }
  $async.Future<ChannelSendDoingActionResponse> channelSendDoingAction($pb.ClientContext ctx, ChannelSendDoingActionParam request) {
    var emptyResponse = ChannelSendDoingActionResponse();
    return _client.invoke<ChannelSendDoingActionResponse>(ctx, 'RPC_Channel', 'ChannelSendDoingAction', request, emptyResponse);
  }
  $async.Future<ChannelReportChannelResponse> channelReportChannel($pb.ClientContext ctx, ChannelReportChannelParam request) {
    var emptyResponse = ChannelReportChannelResponse();
    return _client.invoke<ChannelReportChannelResponse>(ctx, 'RPC_Channel', 'ChannelReportChannel', request, emptyResponse);
  }
  $async.Future<ChannelReportMessageResponse> channelReportMessage($pb.ClientContext ctx, ChannelReportMessageParam request) {
    var emptyResponse = ChannelReportMessageResponse();
    return _client.invoke<ChannelReportMessageResponse>(ctx, 'RPC_Channel', 'ChannelReportMessage', request, emptyResponse);
  }
  $async.Future<ChannelGetFullResponse> channelGetFull($pb.ClientContext ctx, ChannelGetFullParam request) {
    var emptyResponse = ChannelGetFullResponse();
    return _client.invoke<ChannelGetFullResponse>(ctx, 'RPC_Channel', 'ChannelGetFull', request, emptyResponse);
  }
  $async.Future<ChannelGetMessagesListResponse> channelGetMessagesList($pb.ClientContext ctx, ChannelGetMessagesListParam request) {
    var emptyResponse = ChannelGetMessagesListResponse();
    return _client.invoke<ChannelGetMessagesListResponse>(ctx, 'RPC_Channel', 'ChannelGetMessagesList', request, emptyResponse);
  }
  $async.Future<ChannelGetMediaListResponse> channelGetMediaList($pb.ClientContext ctx, ChannelGetMediaListParam request) {
    var emptyResponse = ChannelGetMediaListResponse();
    return _client.invoke<ChannelGetMediaListResponse>(ctx, 'RPC_Channel', 'ChannelGetMediaList', request, emptyResponse);
  }
  $async.Future<ChannelGetAuthorsResponse> channelGetAuthors($pb.ClientContext ctx, ChannelGetAuthorsParam request) {
    var emptyResponse = ChannelGetAuthorsResponse();
    return _client.invoke<ChannelGetAuthorsResponse>(ctx, 'RPC_Channel', 'ChannelGetAuthors', request, emptyResponse);
  }
  $async.Future<ChannelGetFollowersResponse> channelGetFollowers($pb.ClientContext ctx, ChannelGetFollowersParam request) {
    var emptyResponse = ChannelGetFollowersResponse();
    return _client.invoke<ChannelGetFollowersResponse>(ctx, 'RPC_Channel', 'ChannelGetFollowers', request, emptyResponse);
  }
  $async.Future<ChannelGetFollowingsResponse> channelGetFollowings($pb.ClientContext ctx, ChannelGetFollowingsParam request) {
    var emptyResponse = ChannelGetFollowingsResponse();
    return _client.invoke<ChannelGetFollowingsResponse>(ctx, 'RPC_Channel', 'ChannelGetFollowings', request, emptyResponse);
  }
  $async.Future<ChannelGetSubscribersResponse> channelGetSubscribers($pb.ClientContext ctx, ChannelGetSubscribersParam request) {
    var emptyResponse = ChannelGetSubscribersResponse();
    return _client.invoke<ChannelGetSubscribersResponse>(ctx, 'RPC_Channel', 'ChannelGetSubscribers', request, emptyResponse);
  }
  $async.Future<ChannelBlockedResponse> channelBlocked($pb.ClientContext ctx, ChannelBlockedParam request) {
    var emptyResponse = ChannelBlockedResponse();
    return _client.invoke<ChannelBlockedResponse>(ctx, 'RPC_Channel', 'ChannelBlocked', request, emptyResponse);
  }
  $async.Future<ChannelSetDraftResponse> channelSetDraft($pb.ClientContext ctx, ChannelSetDraftParam request) {
    var emptyResponse = ChannelSetDraftResponse();
    return _client.invoke<ChannelSetDraftResponse>(ctx, 'RPC_Channel', 'ChannelSetDraft', request, emptyResponse);
  }
}

