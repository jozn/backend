///
//  Generated code. Do not modify.
//  source: rpc_social.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class AddCommentParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('AddCommentParam', createEmptyInstance: create)
    ..aOS(1, 'text')
    ..hasRequiredFields = false
  ;

  AddCommentParam._() : super();
  factory AddCommentParam() => create();
  factory AddCommentParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddCommentParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  AddCommentParam clone() => AddCommentParam()..mergeFromMessage(this);
  AddCommentParam copyWith(void Function(AddCommentParam) updates) => super.copyWith((message) => updates(message as AddCommentParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddCommentParam create() => AddCommentParam._();
  AddCommentParam createEmptyInstance() => create();
  static $pb.PbList<AddCommentParam> createRepeated() => $pb.PbList<AddCommentParam>();
  @$core.pragma('dart2js:noInline')
  static AddCommentParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddCommentParam>(create);
  static AddCommentParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get text => $_getSZ(0);
  @$pb.TagNumber(1)
  set text($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasText() => $_has(0);
  @$pb.TagNumber(1)
  void clearText() => clearField(1);
}

class AddCommentResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('AddCommentResponse', createEmptyInstance: create)
    ..aOB(1, 'done')
    ..aOS(2, 'text')
    ..hasRequiredFields = false
  ;

  AddCommentResponse._() : super();
  factory AddCommentResponse() => create();
  factory AddCommentResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddCommentResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  AddCommentResponse clone() => AddCommentResponse()..mergeFromMessage(this);
  AddCommentResponse copyWith(void Function(AddCommentResponse) updates) => super.copyWith((message) => updates(message as AddCommentResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddCommentResponse create() => AddCommentResponse._();
  AddCommentResponse createEmptyInstance() => create();
  static $pb.PbList<AddCommentResponse> createRepeated() => $pb.PbList<AddCommentResponse>();
  @$core.pragma('dart2js:noInline')
  static AddCommentResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddCommentResponse>(create);
  static AddCommentResponse _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get done => $_getBF(0);
  @$pb.TagNumber(1)
  set done($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDone() => $_has(0);
  @$pb.TagNumber(1)
  void clearDone() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get text => $_getSZ(1);
  @$pb.TagNumber(2)
  set text($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasText() => $_has(1);
  @$pb.TagNumber(2)
  void clearText() => clearField(2);
}

class DeleteCommentParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('DeleteCommentParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  DeleteCommentParam._() : super();
  factory DeleteCommentParam() => create();
  factory DeleteCommentParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DeleteCommentParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  DeleteCommentParam clone() => DeleteCommentParam()..mergeFromMessage(this);
  DeleteCommentParam copyWith(void Function(DeleteCommentParam) updates) => super.copyWith((message) => updates(message as DeleteCommentParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DeleteCommentParam create() => DeleteCommentParam._();
  DeleteCommentParam createEmptyInstance() => create();
  static $pb.PbList<DeleteCommentParam> createRepeated() => $pb.PbList<DeleteCommentParam>();
  @$core.pragma('dart2js:noInline')
  static DeleteCommentParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DeleteCommentParam>(create);
  static DeleteCommentParam _defaultInstance;
}

class DeleteCommentResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('DeleteCommentResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  DeleteCommentResponse._() : super();
  factory DeleteCommentResponse() => create();
  factory DeleteCommentResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DeleteCommentResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  DeleteCommentResponse clone() => DeleteCommentResponse()..mergeFromMessage(this);
  DeleteCommentResponse copyWith(void Function(DeleteCommentResponse) updates) => super.copyWith((message) => updates(message as DeleteCommentResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DeleteCommentResponse create() => DeleteCommentResponse._();
  DeleteCommentResponse createEmptyInstance() => create();
  static $pb.PbList<DeleteCommentResponse> createRepeated() => $pb.PbList<DeleteCommentResponse>();
  @$core.pragma('dart2js:noInline')
  static DeleteCommentResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DeleteCommentResponse>(create);
  static DeleteCommentResponse _defaultInstance;
}

class EditCommentParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('EditCommentParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  EditCommentParam._() : super();
  factory EditCommentParam() => create();
  factory EditCommentParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditCommentParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  EditCommentParam clone() => EditCommentParam()..mergeFromMessage(this);
  EditCommentParam copyWith(void Function(EditCommentParam) updates) => super.copyWith((message) => updates(message as EditCommentParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EditCommentParam create() => EditCommentParam._();
  EditCommentParam createEmptyInstance() => create();
  static $pb.PbList<EditCommentParam> createRepeated() => $pb.PbList<EditCommentParam>();
  @$core.pragma('dart2js:noInline')
  static EditCommentParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditCommentParam>(create);
  static EditCommentParam _defaultInstance;
}

class EditCommentResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('EditCommentResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  EditCommentResponse._() : super();
  factory EditCommentResponse() => create();
  factory EditCommentResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EditCommentResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  EditCommentResponse clone() => EditCommentResponse()..mergeFromMessage(this);
  EditCommentResponse copyWith(void Function(EditCommentResponse) updates) => super.copyWith((message) => updates(message as EditCommentResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EditCommentResponse create() => EditCommentResponse._();
  EditCommentResponse createEmptyInstance() => create();
  static $pb.PbList<EditCommentResponse> createRepeated() => $pb.PbList<EditCommentResponse>();
  @$core.pragma('dart2js:noInline')
  static EditCommentResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EditCommentResponse>(create);
  static EditCommentResponse _defaultInstance;
}

class LikeCommentParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('LikeCommentParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  LikeCommentParam._() : super();
  factory LikeCommentParam() => create();
  factory LikeCommentParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory LikeCommentParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  LikeCommentParam clone() => LikeCommentParam()..mergeFromMessage(this);
  LikeCommentParam copyWith(void Function(LikeCommentParam) updates) => super.copyWith((message) => updates(message as LikeCommentParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static LikeCommentParam create() => LikeCommentParam._();
  LikeCommentParam createEmptyInstance() => create();
  static $pb.PbList<LikeCommentParam> createRepeated() => $pb.PbList<LikeCommentParam>();
  @$core.pragma('dart2js:noInline')
  static LikeCommentParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LikeCommentParam>(create);
  static LikeCommentParam _defaultInstance;
}

class LikeCommentResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('LikeCommentResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  LikeCommentResponse._() : super();
  factory LikeCommentResponse() => create();
  factory LikeCommentResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory LikeCommentResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  LikeCommentResponse clone() => LikeCommentResponse()..mergeFromMessage(this);
  LikeCommentResponse copyWith(void Function(LikeCommentResponse) updates) => super.copyWith((message) => updates(message as LikeCommentResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static LikeCommentResponse create() => LikeCommentResponse._();
  LikeCommentResponse createEmptyInstance() => create();
  static $pb.PbList<LikeCommentResponse> createRepeated() => $pb.PbList<LikeCommentResponse>();
  @$core.pragma('dart2js:noInline')
  static LikeCommentResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LikeCommentResponse>(create);
  static LikeCommentResponse _defaultInstance;
}

class AddSeenPostsParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('AddSeenPostsParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  AddSeenPostsParam._() : super();
  factory AddSeenPostsParam() => create();
  factory AddSeenPostsParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddSeenPostsParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  AddSeenPostsParam clone() => AddSeenPostsParam()..mergeFromMessage(this);
  AddSeenPostsParam copyWith(void Function(AddSeenPostsParam) updates) => super.copyWith((message) => updates(message as AddSeenPostsParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddSeenPostsParam create() => AddSeenPostsParam._();
  AddSeenPostsParam createEmptyInstance() => create();
  static $pb.PbList<AddSeenPostsParam> createRepeated() => $pb.PbList<AddSeenPostsParam>();
  @$core.pragma('dart2js:noInline')
  static AddSeenPostsParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddSeenPostsParam>(create);
  static AddSeenPostsParam _defaultInstance;
}

class AddSeenPostsResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('AddSeenPostsResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  AddSeenPostsResponse._() : super();
  factory AddSeenPostsResponse() => create();
  factory AddSeenPostsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddSeenPostsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  AddSeenPostsResponse clone() => AddSeenPostsResponse()..mergeFromMessage(this);
  AddSeenPostsResponse copyWith(void Function(AddSeenPostsResponse) updates) => super.copyWith((message) => updates(message as AddSeenPostsResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddSeenPostsResponse create() => AddSeenPostsResponse._();
  AddSeenPostsResponse createEmptyInstance() => create();
  static $pb.PbList<AddSeenPostsResponse> createRepeated() => $pb.PbList<AddSeenPostsResponse>();
  @$core.pragma('dart2js:noInline')
  static AddSeenPostsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddSeenPostsResponse>(create);
  static AddSeenPostsResponse _defaultInstance;
}

class LikePostParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('LikePostParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  LikePostParam._() : super();
  factory LikePostParam() => create();
  factory LikePostParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory LikePostParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  LikePostParam clone() => LikePostParam()..mergeFromMessage(this);
  LikePostParam copyWith(void Function(LikePostParam) updates) => super.copyWith((message) => updates(message as LikePostParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static LikePostParam create() => LikePostParam._();
  LikePostParam createEmptyInstance() => create();
  static $pb.PbList<LikePostParam> createRepeated() => $pb.PbList<LikePostParam>();
  @$core.pragma('dart2js:noInline')
  static LikePostParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LikePostParam>(create);
  static LikePostParam _defaultInstance;
}

class LikePostResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('LikePostResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  LikePostResponse._() : super();
  factory LikePostResponse() => create();
  factory LikePostResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory LikePostResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  LikePostResponse clone() => LikePostResponse()..mergeFromMessage(this);
  LikePostResponse copyWith(void Function(LikePostResponse) updates) => super.copyWith((message) => updates(message as LikePostResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static LikePostResponse create() => LikePostResponse._();
  LikePostResponse createEmptyInstance() => create();
  static $pb.PbList<LikePostResponse> createRepeated() => $pb.PbList<LikePostResponse>();
  @$core.pragma('dart2js:noInline')
  static LikePostResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LikePostResponse>(create);
  static LikePostResponse _defaultInstance;
}

class UnLikePostParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UnLikePostParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  UnLikePostParam._() : super();
  factory UnLikePostParam() => create();
  factory UnLikePostParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UnLikePostParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UnLikePostParam clone() => UnLikePostParam()..mergeFromMessage(this);
  UnLikePostParam copyWith(void Function(UnLikePostParam) updates) => super.copyWith((message) => updates(message as UnLikePostParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UnLikePostParam create() => UnLikePostParam._();
  UnLikePostParam createEmptyInstance() => create();
  static $pb.PbList<UnLikePostParam> createRepeated() => $pb.PbList<UnLikePostParam>();
  @$core.pragma('dart2js:noInline')
  static UnLikePostParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UnLikePostParam>(create);
  static UnLikePostParam _defaultInstance;
}

class UnLikePostResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UnLikePostResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  UnLikePostResponse._() : super();
  factory UnLikePostResponse() => create();
  factory UnLikePostResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UnLikePostResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UnLikePostResponse clone() => UnLikePostResponse()..mergeFromMessage(this);
  UnLikePostResponse copyWith(void Function(UnLikePostResponse) updates) => super.copyWith((message) => updates(message as UnLikePostResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UnLikePostResponse create() => UnLikePostResponse._();
  UnLikePostResponse createEmptyInstance() => create();
  static $pb.PbList<UnLikePostResponse> createRepeated() => $pb.PbList<UnLikePostResponse>();
  @$core.pragma('dart2js:noInline')
  static UnLikePostResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UnLikePostResponse>(create);
  static UnLikePostResponse _defaultInstance;
}

class FollowChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('FollowChannelParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  FollowChannelParam._() : super();
  factory FollowChannelParam() => create();
  factory FollowChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FollowChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  FollowChannelParam clone() => FollowChannelParam()..mergeFromMessage(this);
  FollowChannelParam copyWith(void Function(FollowChannelParam) updates) => super.copyWith((message) => updates(message as FollowChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FollowChannelParam create() => FollowChannelParam._();
  FollowChannelParam createEmptyInstance() => create();
  static $pb.PbList<FollowChannelParam> createRepeated() => $pb.PbList<FollowChannelParam>();
  @$core.pragma('dart2js:noInline')
  static FollowChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FollowChannelParam>(create);
  static FollowChannelParam _defaultInstance;
}

class FollowChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('FollowChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  FollowChannelResponse._() : super();
  factory FollowChannelResponse() => create();
  factory FollowChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FollowChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  FollowChannelResponse clone() => FollowChannelResponse()..mergeFromMessage(this);
  FollowChannelResponse copyWith(void Function(FollowChannelResponse) updates) => super.copyWith((message) => updates(message as FollowChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FollowChannelResponse create() => FollowChannelResponse._();
  FollowChannelResponse createEmptyInstance() => create();
  static $pb.PbList<FollowChannelResponse> createRepeated() => $pb.PbList<FollowChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static FollowChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FollowChannelResponse>(create);
  static FollowChannelResponse _defaultInstance;
}

class UnFollowChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UnFollowChannelParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  UnFollowChannelParam._() : super();
  factory UnFollowChannelParam() => create();
  factory UnFollowChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UnFollowChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UnFollowChannelParam clone() => UnFollowChannelParam()..mergeFromMessage(this);
  UnFollowChannelParam copyWith(void Function(UnFollowChannelParam) updates) => super.copyWith((message) => updates(message as UnFollowChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UnFollowChannelParam create() => UnFollowChannelParam._();
  UnFollowChannelParam createEmptyInstance() => create();
  static $pb.PbList<UnFollowChannelParam> createRepeated() => $pb.PbList<UnFollowChannelParam>();
  @$core.pragma('dart2js:noInline')
  static UnFollowChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UnFollowChannelParam>(create);
  static UnFollowChannelParam _defaultInstance;
}

class UnFollowChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UnFollowChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  UnFollowChannelResponse._() : super();
  factory UnFollowChannelResponse() => create();
  factory UnFollowChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UnFollowChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UnFollowChannelResponse clone() => UnFollowChannelResponse()..mergeFromMessage(this);
  UnFollowChannelResponse copyWith(void Function(UnFollowChannelResponse) updates) => super.copyWith((message) => updates(message as UnFollowChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UnFollowChannelResponse create() => UnFollowChannelResponse._();
  UnFollowChannelResponse createEmptyInstance() => create();
  static $pb.PbList<UnFollowChannelResponse> createRepeated() => $pb.PbList<UnFollowChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static UnFollowChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UnFollowChannelResponse>(create);
  static UnFollowChannelResponse _defaultInstance;
}

class PinChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('PinChannelParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  PinChannelParam._() : super();
  factory PinChannelParam() => create();
  factory PinChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PinChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  PinChannelParam clone() => PinChannelParam()..mergeFromMessage(this);
  PinChannelParam copyWith(void Function(PinChannelParam) updates) => super.copyWith((message) => updates(message as PinChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static PinChannelParam create() => PinChannelParam._();
  PinChannelParam createEmptyInstance() => create();
  static $pb.PbList<PinChannelParam> createRepeated() => $pb.PbList<PinChannelParam>();
  @$core.pragma('dart2js:noInline')
  static PinChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PinChannelParam>(create);
  static PinChannelParam _defaultInstance;
}

class PinChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('PinChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  PinChannelResponse._() : super();
  factory PinChannelResponse() => create();
  factory PinChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PinChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  PinChannelResponse clone() => PinChannelResponse()..mergeFromMessage(this);
  PinChannelResponse copyWith(void Function(PinChannelResponse) updates) => super.copyWith((message) => updates(message as PinChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static PinChannelResponse create() => PinChannelResponse._();
  PinChannelResponse createEmptyInstance() => create();
  static $pb.PbList<PinChannelResponse> createRepeated() => $pb.PbList<PinChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static PinChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PinChannelResponse>(create);
  static PinChannelResponse _defaultInstance;
}

class UnPinChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UnPinChannelParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  UnPinChannelParam._() : super();
  factory UnPinChannelParam() => create();
  factory UnPinChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UnPinChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UnPinChannelParam clone() => UnPinChannelParam()..mergeFromMessage(this);
  UnPinChannelParam copyWith(void Function(UnPinChannelParam) updates) => super.copyWith((message) => updates(message as UnPinChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UnPinChannelParam create() => UnPinChannelParam._();
  UnPinChannelParam createEmptyInstance() => create();
  static $pb.PbList<UnPinChannelParam> createRepeated() => $pb.PbList<UnPinChannelParam>();
  @$core.pragma('dart2js:noInline')
  static UnPinChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UnPinChannelParam>(create);
  static UnPinChannelParam _defaultInstance;
}

class UnPinChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UnPinChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  UnPinChannelResponse._() : super();
  factory UnPinChannelResponse() => create();
  factory UnPinChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UnPinChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UnPinChannelResponse clone() => UnPinChannelResponse()..mergeFromMessage(this);
  UnPinChannelResponse copyWith(void Function(UnPinChannelResponse) updates) => super.copyWith((message) => updates(message as UnPinChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UnPinChannelResponse create() => UnPinChannelResponse._();
  UnPinChannelResponse createEmptyInstance() => create();
  static $pb.PbList<UnPinChannelResponse> createRepeated() => $pb.PbList<UnPinChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static UnPinChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UnPinChannelResponse>(create);
  static UnPinChannelResponse _defaultInstance;
}

class BlockChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('BlockChannelParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  BlockChannelParam._() : super();
  factory BlockChannelParam() => create();
  factory BlockChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BlockChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  BlockChannelParam clone() => BlockChannelParam()..mergeFromMessage(this);
  BlockChannelParam copyWith(void Function(BlockChannelParam) updates) => super.copyWith((message) => updates(message as BlockChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BlockChannelParam create() => BlockChannelParam._();
  BlockChannelParam createEmptyInstance() => create();
  static $pb.PbList<BlockChannelParam> createRepeated() => $pb.PbList<BlockChannelParam>();
  @$core.pragma('dart2js:noInline')
  static BlockChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BlockChannelParam>(create);
  static BlockChannelParam _defaultInstance;
}

class BlockChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('BlockChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  BlockChannelResponse._() : super();
  factory BlockChannelResponse() => create();
  factory BlockChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BlockChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  BlockChannelResponse clone() => BlockChannelResponse()..mergeFromMessage(this);
  BlockChannelResponse copyWith(void Function(BlockChannelResponse) updates) => super.copyWith((message) => updates(message as BlockChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BlockChannelResponse create() => BlockChannelResponse._();
  BlockChannelResponse createEmptyInstance() => create();
  static $pb.PbList<BlockChannelResponse> createRepeated() => $pb.PbList<BlockChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static BlockChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BlockChannelResponse>(create);
  static BlockChannelResponse _defaultInstance;
}

class UnBlockChannelParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UnBlockChannelParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  UnBlockChannelParam._() : super();
  factory UnBlockChannelParam() => create();
  factory UnBlockChannelParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UnBlockChannelParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UnBlockChannelParam clone() => UnBlockChannelParam()..mergeFromMessage(this);
  UnBlockChannelParam copyWith(void Function(UnBlockChannelParam) updates) => super.copyWith((message) => updates(message as UnBlockChannelParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UnBlockChannelParam create() => UnBlockChannelParam._();
  UnBlockChannelParam createEmptyInstance() => create();
  static $pb.PbList<UnBlockChannelParam> createRepeated() => $pb.PbList<UnBlockChannelParam>();
  @$core.pragma('dart2js:noInline')
  static UnBlockChannelParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UnBlockChannelParam>(create);
  static UnBlockChannelParam _defaultInstance;
}

class UnBlockChannelResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UnBlockChannelResponse', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  UnBlockChannelResponse._() : super();
  factory UnBlockChannelResponse() => create();
  factory UnBlockChannelResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UnBlockChannelResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UnBlockChannelResponse clone() => UnBlockChannelResponse()..mergeFromMessage(this);
  UnBlockChannelResponse copyWith(void Function(UnBlockChannelResponse) updates) => super.copyWith((message) => updates(message as UnBlockChannelResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UnBlockChannelResponse create() => UnBlockChannelResponse._();
  UnBlockChannelResponse createEmptyInstance() => create();
  static $pb.PbList<UnBlockChannelResponse> createRepeated() => $pb.PbList<UnBlockChannelResponse>();
  @$core.pragma('dart2js:noInline')
  static UnBlockChannelResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UnBlockChannelResponse>(create);
  static UnBlockChannelResponse _defaultInstance;
}

class RPC_SocialApi {
  $pb.RpcClient _client;
  RPC_SocialApi(this._client);

  $async.Future<AddCommentResponse> addComment($pb.ClientContext ctx, AddCommentParam request) {
    var emptyResponse = AddCommentResponse();
    return _client.invoke<AddCommentResponse>(ctx, 'RPC_Social', 'AddComment', request, emptyResponse);
  }
  $async.Future<DeleteCommentResponse> deleteComment($pb.ClientContext ctx, DeleteCommentParam request) {
    var emptyResponse = DeleteCommentResponse();
    return _client.invoke<DeleteCommentResponse>(ctx, 'RPC_Social', 'DeleteComment', request, emptyResponse);
  }
  $async.Future<EditCommentResponse> editComment($pb.ClientContext ctx, EditCommentParam request) {
    var emptyResponse = EditCommentResponse();
    return _client.invoke<EditCommentResponse>(ctx, 'RPC_Social', 'EditComment', request, emptyResponse);
  }
  $async.Future<LikeCommentResponse> likeComment($pb.ClientContext ctx, LikeCommentParam request) {
    var emptyResponse = LikeCommentResponse();
    return _client.invoke<LikeCommentResponse>(ctx, 'RPC_Social', 'LikeComment', request, emptyResponse);
  }
  $async.Future<AddSeenPostsResponse> addSeenPosts($pb.ClientContext ctx, AddSeenPostsParam request) {
    var emptyResponse = AddSeenPostsResponse();
    return _client.invoke<AddSeenPostsResponse>(ctx, 'RPC_Social', 'AddSeenPosts', request, emptyResponse);
  }
  $async.Future<LikePostResponse> likePost($pb.ClientContext ctx, LikePostParam request) {
    var emptyResponse = LikePostResponse();
    return _client.invoke<LikePostResponse>(ctx, 'RPC_Social', 'LikePost', request, emptyResponse);
  }
  $async.Future<UnLikePostResponse> unLikePost($pb.ClientContext ctx, UnLikePostParam request) {
    var emptyResponse = UnLikePostResponse();
    return _client.invoke<UnLikePostResponse>(ctx, 'RPC_Social', 'UnLikePost', request, emptyResponse);
  }
  $async.Future<FollowChannelResponse> followChannel($pb.ClientContext ctx, FollowChannelParam request) {
    var emptyResponse = FollowChannelResponse();
    return _client.invoke<FollowChannelResponse>(ctx, 'RPC_Social', 'FollowChannel', request, emptyResponse);
  }
  $async.Future<UnFollowChannelResponse> unFollowChannel($pb.ClientContext ctx, UnFollowChannelParam request) {
    var emptyResponse = UnFollowChannelResponse();
    return _client.invoke<UnFollowChannelResponse>(ctx, 'RPC_Social', 'UnFollowChannel', request, emptyResponse);
  }
  $async.Future<PinChannelResponse> pinChannel($pb.ClientContext ctx, PinChannelParam request) {
    var emptyResponse = PinChannelResponse();
    return _client.invoke<PinChannelResponse>(ctx, 'RPC_Social', 'PinChannel', request, emptyResponse);
  }
  $async.Future<UnPinChannelResponse> unPinChannel($pb.ClientContext ctx, UnPinChannelParam request) {
    var emptyResponse = UnPinChannelResponse();
    return _client.invoke<UnPinChannelResponse>(ctx, 'RPC_Social', 'UnPinChannel', request, emptyResponse);
  }
  $async.Future<BlockChannelResponse> blockChannel($pb.ClientContext ctx, BlockChannelParam request) {
    var emptyResponse = BlockChannelResponse();
    return _client.invoke<BlockChannelResponse>(ctx, 'RPC_Social', 'BlockChannel', request, emptyResponse);
  }
  $async.Future<UnBlockChannelResponse> unBlockChannel($pb.ClientContext ctx, UnBlockChannelParam request) {
    var emptyResponse = UnBlockChannelResponse();
    return _client.invoke<UnBlockChannelResponse>(ctx, 'RPC_Social', 'UnBlockChannel', request, emptyResponse);
  }
}

