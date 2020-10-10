///
//  Generated code. Do not modify.
//  source: rpc_general.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;

import 'package:protobuf/protobuf.dart' as $pb;

import 'dart:core' as $core;
import 'rpc_general.pb.dart' as $7;
import 'rpc_general.pbjson.dart';

export 'rpc_general.pb.dart';

abstract class RPC_GeneralServiceBase extends $pb.GeneratedService {
  $async.Future<$7.EchoResponse> echo($pb.ServerContext ctx, $7.EchoParam request);
  $async.Future<$7.CheckUserNameResponse> checkUserName($pb.ServerContext ctx, $7.CheckUserNameParam request);

  $pb.GeneratedMessage createRequest($core.String method) {
    switch (method) {
      case 'Echo': return $7.EchoParam();
      case 'CheckUserName': return $7.CheckUserNameParam();
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $async.Future<$pb.GeneratedMessage> handleCall($pb.ServerContext ctx, $core.String method, $pb.GeneratedMessage request) {
    switch (method) {
      case 'Echo': return this.echo(ctx, request);
      case 'CheckUserName': return this.checkUserName(ctx, request);
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $core.Map<$core.String, $core.dynamic> get $json => RPC_GeneralServiceBase$json;
  $core.Map<$core.String, $core.Map<$core.String, $core.dynamic>> get $messageJson => RPC_GeneralServiceBase$messageJson;
}

