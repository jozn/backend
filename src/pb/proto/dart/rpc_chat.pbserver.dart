///
//  Generated code. Do not modify.
//  source: rpc_chat.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;

import 'package:protobuf/protobuf.dart' as $pb;

import 'dart:core' as $core;
import 'rpc_chat.pb.dart' as $5;
import 'rpc_chat.pbjson.dart';

export 'rpc_chat.pb.dart';

abstract class RPC_ChatServiceBase extends $pb.GeneratedService {
  $async.Future<$5.ChatSendMessageResponse> chatSendMessage($pb.ServerContext ctx, $5.ChatSendMessageParam request);
  $async.Future<$5.ChatEditMessageResponse> chatEditMessage($pb.ServerContext ctx, $5.ChatEditMessageParam request);
  $async.Future<$5.ChatDeleteMessagesResponse> chatDeleteMessages($pb.ServerContext ctx, $5.ChatDeleteMessagesParam request);
  $async.Future<$5.ChatDeleteHistoryResponse> chatDeleteHistory($pb.ServerContext ctx, $5.ChatDeleteHistoryParam request);
  $async.Future<$5.ChatSendDoingActionResponse> chatSendDoingAction($pb.ServerContext ctx, $5.ChatSendDoingActionParam request);
  $async.Future<$5.ChatReportChatResponse> chatReportChat($pb.ServerContext ctx, $5.ChatReportChatParam request);
  $async.Future<$5.ChatGetFullMessageResponse> chatGetFull($pb.ServerContext ctx, $5.ChatGetFullMessageParam request);
  $async.Future<$5.ChatGetMessagesListResponse> chatGetMessagesList($pb.ServerContext ctx, $5.ChatGetMessagesListParam request);
  $async.Future<$5.ChatGetMediaListResponse> chatGetMediaList($pb.ServerContext ctx, $5.ChatGetMediaListParam request);

  $pb.GeneratedMessage createRequest($core.String method) {
    switch (method) {
      case 'ChatSendMessage': return $5.ChatSendMessageParam();
      case 'ChatEditMessage': return $5.ChatEditMessageParam();
      case 'ChatDeleteMessages': return $5.ChatDeleteMessagesParam();
      case 'ChatDeleteHistory': return $5.ChatDeleteHistoryParam();
      case 'ChatSendDoingAction': return $5.ChatSendDoingActionParam();
      case 'ChatReportChat': return $5.ChatReportChatParam();
      case 'ChatGetFull': return $5.ChatGetFullMessageParam();
      case 'ChatGetMessagesList': return $5.ChatGetMessagesListParam();
      case 'ChatGetMediaList': return $5.ChatGetMediaListParam();
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $async.Future<$pb.GeneratedMessage> handleCall($pb.ServerContext ctx, $core.String method, $pb.GeneratedMessage request) {
    switch (method) {
      case 'ChatSendMessage': return this.chatSendMessage(ctx, request);
      case 'ChatEditMessage': return this.chatEditMessage(ctx, request);
      case 'ChatDeleteMessages': return this.chatDeleteMessages(ctx, request);
      case 'ChatDeleteHistory': return this.chatDeleteHistory(ctx, request);
      case 'ChatSendDoingAction': return this.chatSendDoingAction(ctx, request);
      case 'ChatReportChat': return this.chatReportChat(ctx, request);
      case 'ChatGetFull': return this.chatGetFull(ctx, request);
      case 'ChatGetMessagesList': return this.chatGetMessagesList(ctx, request);
      case 'ChatGetMediaList': return this.chatGetMediaList(ctx, request);
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $core.Map<$core.String, $core.dynamic> get $json => RPC_ChatServiceBase$json;
  $core.Map<$core.String, $core.Map<$core.String, $core.dynamic>> get $messageJson => RPC_ChatServiceBase$messageJson;
}

