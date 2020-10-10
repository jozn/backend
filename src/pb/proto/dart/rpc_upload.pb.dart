///
//  Generated code. Do not modify.
//  source: rpc_upload.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class UploadFileParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UploadFileParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  UploadFileParam._() : super();
  factory UploadFileParam() => create();
  factory UploadFileParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UploadFileParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UploadFileParam clone() => UploadFileParam()..mergeFromMessage(this);
  UploadFileParam copyWith(void Function(UploadFileParam) updates) => super.copyWith((message) => updates(message as UploadFileParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UploadFileParam create() => UploadFileParam._();
  UploadFileParam createEmptyInstance() => create();
  static $pb.PbList<UploadFileParam> createRepeated() => $pb.PbList<UploadFileParam>();
  @$core.pragma('dart2js:noInline')
  static UploadFileParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UploadFileParam>(create);
  static UploadFileParam _defaultInstance;
}

class UploadFileResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UploadFileResponse', createEmptyInstance: create)
    ..aOB(1, 'done')
    ..aOS(2, 'text')
    ..hasRequiredFields = false
  ;

  UploadFileResponse._() : super();
  factory UploadFileResponse() => create();
  factory UploadFileResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UploadFileResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UploadFileResponse clone() => UploadFileResponse()..mergeFromMessage(this);
  UploadFileResponse copyWith(void Function(UploadFileResponse) updates) => super.copyWith((message) => updates(message as UploadFileResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UploadFileResponse create() => UploadFileResponse._();
  UploadFileResponse createEmptyInstance() => create();
  static $pb.PbList<UploadFileResponse> createRepeated() => $pb.PbList<UploadFileResponse>();
  @$core.pragma('dart2js:noInline')
  static UploadFileResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UploadFileResponse>(create);
  static UploadFileResponse _defaultInstance;

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

class RPC_UploadApi {
  $pb.RpcClient _client;
  RPC_UploadApi(this._client);

  $async.Future<UploadFileResponse> uploadFile($pb.ClientContext ctx, UploadFileParam request) {
    var emptyResponse = UploadFileResponse();
    return _client.invoke<UploadFileResponse>(ctx, 'RPC_Upload', 'UploadFile', request, emptyResponse);
  }
}

