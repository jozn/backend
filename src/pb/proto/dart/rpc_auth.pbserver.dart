///
//  Generated code. Do not modify.
//  source: rpc_auth.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;

import 'package:protobuf/protobuf.dart' as $pb;

import 'dart:core' as $core;
import 'rpc_auth.pb.dart' as $3;
import 'rpc_auth.pbjson.dart';

export 'rpc_auth.pb.dart';

abstract class RPC_AuthServiceBase extends $pb.GeneratedService {
  $async.Future<$3.SendConfirmCodeResponse> sendConfirmCode($pb.ServerContext ctx, $3.SendConfirmCodeParam request);
  $async.Future<$3.ConfirmCodeResponse> confirmCode($pb.ServerContext ctx, $3.ConfirmCodeParam request);
  $async.Future<$3.SingUpResponse> singUp($pb.ServerContext ctx, $3.SingUpParam request);
  $async.Future<$3.SingInResponse> singIn($pb.ServerContext ctx, $3.SingInParam request);
  $async.Future<$3.LogOutResponse> logOut($pb.ServerContext ctx, $3.LogOutParam request);

  $pb.GeneratedMessage createRequest($core.String method) {
    switch (method) {
      case 'SendConfirmCode': return $3.SendConfirmCodeParam();
      case 'ConfirmCode': return $3.ConfirmCodeParam();
      case 'SingUp': return $3.SingUpParam();
      case 'SingIn': return $3.SingInParam();
      case 'LogOut': return $3.LogOutParam();
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $async.Future<$pb.GeneratedMessage> handleCall($pb.ServerContext ctx, $core.String method, $pb.GeneratedMessage request) {
    switch (method) {
      case 'SendConfirmCode': return this.sendConfirmCode(ctx, request);
      case 'ConfirmCode': return this.confirmCode(ctx, request);
      case 'SingUp': return this.singUp(ctx, request);
      case 'SingIn': return this.singIn(ctx, request);
      case 'LogOut': return this.logOut(ctx, request);
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $core.Map<$core.String, $core.dynamic> get $json => RPC_AuthServiceBase$json;
  $core.Map<$core.String, $core.Map<$core.String, $core.dynamic>> get $messageJson => RPC_AuthServiceBase$messageJson;
}

