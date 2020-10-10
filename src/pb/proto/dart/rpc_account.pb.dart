///
//  Generated code. Do not modify.
//  source: rpc_account.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class ChangePhoneNumberParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChangePhoneNumberParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  ChangePhoneNumberParam._() : super();
  factory ChangePhoneNumberParam() => create();
  factory ChangePhoneNumberParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangePhoneNumberParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChangePhoneNumberParam clone() => ChangePhoneNumberParam()..mergeFromMessage(this);
  ChangePhoneNumberParam copyWith(void Function(ChangePhoneNumberParam) updates) => super.copyWith((message) => updates(message as ChangePhoneNumberParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChangePhoneNumberParam create() => ChangePhoneNumberParam._();
  ChangePhoneNumberParam createEmptyInstance() => create();
  static $pb.PbList<ChangePhoneNumberParam> createRepeated() => $pb.PbList<ChangePhoneNumberParam>();
  @$core.pragma('dart2js:noInline')
  static ChangePhoneNumberParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangePhoneNumberParam>(create);
  static ChangePhoneNumberParam _defaultInstance;
}

class ChangePhoneNumberResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChangePhoneNumberResponse', createEmptyInstance: create)
    ..aOB(1, 'done')
    ..aOS(2, 'text')
    ..hasRequiredFields = false
  ;

  ChangePhoneNumberResponse._() : super();
  factory ChangePhoneNumberResponse() => create();
  factory ChangePhoneNumberResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChangePhoneNumberResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChangePhoneNumberResponse clone() => ChangePhoneNumberResponse()..mergeFromMessage(this);
  ChangePhoneNumberResponse copyWith(void Function(ChangePhoneNumberResponse) updates) => super.copyWith((message) => updates(message as ChangePhoneNumberResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChangePhoneNumberResponse create() => ChangePhoneNumberResponse._();
  ChangePhoneNumberResponse createEmptyInstance() => create();
  static $pb.PbList<ChangePhoneNumberResponse> createRepeated() => $pb.PbList<ChangePhoneNumberResponse>();
  @$core.pragma('dart2js:noInline')
  static ChangePhoneNumberResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChangePhoneNumberResponse>(create);
  static ChangePhoneNumberResponse _defaultInstance;

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

class RPC_AccountApi {
  $pb.RpcClient _client;
  RPC_AccountApi(this._client);

  $async.Future<ChangePhoneNumberResponse> changePhoneNumber($pb.ClientContext ctx, ChangePhoneNumberParam request) {
    var emptyResponse = ChangePhoneNumberResponse();
    return _client.invoke<ChangePhoneNumberResponse>(ctx, 'RPC_Account', 'ChangePhoneNumber', request, emptyResponse);
  }
}

