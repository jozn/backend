///
//  Generated code. Do not modify.
//  source: rpc_upload.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;

import 'package:protobuf/protobuf.dart' as $pb;

import 'dart:core' as $core;
import 'rpc_upload.pb.dart' as $10;
import 'rpc_upload.pbjson.dart';

export 'rpc_upload.pb.dart';

abstract class RPC_UploadServiceBase extends $pb.GeneratedService {
  $async.Future<$10.UploadFileResponse> uploadFile($pb.ServerContext ctx, $10.UploadFileParam request);

  $pb.GeneratedMessage createRequest($core.String method) {
    switch (method) {
      case 'UploadFile': return $10.UploadFileParam();
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $async.Future<$pb.GeneratedMessage> handleCall($pb.ServerContext ctx, $core.String method, $pb.GeneratedMessage request) {
    switch (method) {
      case 'UploadFile': return this.uploadFile(ctx, request);
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $core.Map<$core.String, $core.dynamic> get $json => RPC_UploadServiceBase$json;
  $core.Map<$core.String, $core.Map<$core.String, $core.dynamic>> get $messageJson => RPC_UploadServiceBase$messageJson;
}

