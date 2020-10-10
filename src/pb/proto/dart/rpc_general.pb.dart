///
//  Generated code. Do not modify.
//  source: rpc_general.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class EchoParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('EchoParam', createEmptyInstance: create)
    ..aOS(1, 'text')
    ..hasRequiredFields = false
  ;

  EchoParam._() : super();
  factory EchoParam() => create();
  factory EchoParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EchoParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  EchoParam clone() => EchoParam()..mergeFromMessage(this);
  EchoParam copyWith(void Function(EchoParam) updates) => super.copyWith((message) => updates(message as EchoParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EchoParam create() => EchoParam._();
  EchoParam createEmptyInstance() => create();
  static $pb.PbList<EchoParam> createRepeated() => $pb.PbList<EchoParam>();
  @$core.pragma('dart2js:noInline')
  static EchoParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EchoParam>(create);
  static EchoParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get text => $_getSZ(0);
  @$pb.TagNumber(1)
  set text($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasText() => $_has(0);
  @$pb.TagNumber(1)
  void clearText() => clearField(1);
}

class EchoResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('EchoResponse', createEmptyInstance: create)
    ..aOB(1, 'done')
    ..aOS(2, 'text')
    ..hasRequiredFields = false
  ;

  EchoResponse._() : super();
  factory EchoResponse() => create();
  factory EchoResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory EchoResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  EchoResponse clone() => EchoResponse()..mergeFromMessage(this);
  EchoResponse copyWith(void Function(EchoResponse) updates) => super.copyWith((message) => updates(message as EchoResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EchoResponse create() => EchoResponse._();
  EchoResponse createEmptyInstance() => create();
  static $pb.PbList<EchoResponse> createRepeated() => $pb.PbList<EchoResponse>();
  @$core.pragma('dart2js:noInline')
  static EchoResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EchoResponse>(create);
  static EchoResponse _defaultInstance;

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

class CheckUserNameParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('CheckUserNameParam', createEmptyInstance: create)
    ..aOS(1, 'username')
    ..hasRequiredFields = false
  ;

  CheckUserNameParam._() : super();
  factory CheckUserNameParam() => create();
  factory CheckUserNameParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CheckUserNameParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  CheckUserNameParam clone() => CheckUserNameParam()..mergeFromMessage(this);
  CheckUserNameParam copyWith(void Function(CheckUserNameParam) updates) => super.copyWith((message) => updates(message as CheckUserNameParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static CheckUserNameParam create() => CheckUserNameParam._();
  CheckUserNameParam createEmptyInstance() => create();
  static $pb.PbList<CheckUserNameParam> createRepeated() => $pb.PbList<CheckUserNameParam>();
  @$core.pragma('dart2js:noInline')
  static CheckUserNameParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CheckUserNameParam>(create);
  static CheckUserNameParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get username => $_getSZ(0);
  @$pb.TagNumber(1)
  set username($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUsername() => $_has(0);
  @$pb.TagNumber(1)
  void clearUsername() => clearField(1);
}

class CheckUserNameResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('CheckUserNameResponse', createEmptyInstance: create)
    ..aOB(1, 'isAvailable')
    ..aOS(2, 'username')
    ..aOS(3, 'showMessage')
    ..hasRequiredFields = false
  ;

  CheckUserNameResponse._() : super();
  factory CheckUserNameResponse() => create();
  factory CheckUserNameResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CheckUserNameResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  CheckUserNameResponse clone() => CheckUserNameResponse()..mergeFromMessage(this);
  CheckUserNameResponse copyWith(void Function(CheckUserNameResponse) updates) => super.copyWith((message) => updates(message as CheckUserNameResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static CheckUserNameResponse create() => CheckUserNameResponse._();
  CheckUserNameResponse createEmptyInstance() => create();
  static $pb.PbList<CheckUserNameResponse> createRepeated() => $pb.PbList<CheckUserNameResponse>();
  @$core.pragma('dart2js:noInline')
  static CheckUserNameResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CheckUserNameResponse>(create);
  static CheckUserNameResponse _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get isAvailable => $_getBF(0);
  @$pb.TagNumber(1)
  set isAvailable($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasIsAvailable() => $_has(0);
  @$pb.TagNumber(1)
  void clearIsAvailable() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get username => $_getSZ(1);
  @$pb.TagNumber(2)
  set username($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUsername() => $_has(1);
  @$pb.TagNumber(2)
  void clearUsername() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get showMessage => $_getSZ(2);
  @$pb.TagNumber(3)
  set showMessage($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasShowMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearShowMessage() => clearField(3);
}

class RPC_GeneralApi {
  $pb.RpcClient _client;
  RPC_GeneralApi(this._client);

  $async.Future<EchoResponse> echo($pb.ClientContext ctx, EchoParam request) {
    var emptyResponse = EchoResponse();
    return _client.invoke<EchoResponse>(ctx, 'RPC_General', 'Echo', request, emptyResponse);
  }
  $async.Future<CheckUserNameResponse> checkUserName($pb.ClientContext ctx, CheckUserNameParam request) {
    var emptyResponse = CheckUserNameResponse();
    return _client.invoke<CheckUserNameResponse>(ctx, 'RPC_General', 'CheckUserName', request, emptyResponse);
  }
}

