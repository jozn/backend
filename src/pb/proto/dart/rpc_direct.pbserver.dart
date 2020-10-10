///
//  Generated code. Do not modify.
//  source: rpc_direct.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;

import 'package:protobuf/protobuf.dart' as $pb;

import 'dart:core' as $core;
import 'rpc_direct.pb.dart' as $6;
import 'rpc_direct.pbjson.dart';

export 'rpc_direct.pb.dart';

abstract class RPC_DirectServiceBase extends $pb.GeneratedService {
  $async.Future<$6.DirectDeleteDirectResponse> directDeleteDirect($pb.ServerContext ctx, $6.DirectDeleteDirectParam request);
  $async.Future<$6.DirectChangeTitleResponse> directChangeTitle($pb.ServerContext ctx, $6.DirectChangeTitleParam request);
  $async.Future<$6.DirectSetCustomNotificationResponse> directSetCustomNotification($pb.ServerContext ctx, $6.DirectSetCustomNotificationParam request);
  $async.Future<$6.DirectSendActionDoingResponse> directSendActionDoing($pb.ServerContext ctx, $6.DirectSendActionDoingParam request);
  $async.Future<$6.DirectSetDraftResponse> directSetDraft($pb.ServerContext ctx, $6.DirectSetDraftParam request);
  $async.Future<$6.DirectDeleteDirectsResponse> directDeleteDirects($pb.ServerContext ctx, $6.DirectDeleteDirectsParam request);
  $async.Future<$6.DirectMarkAsReadResponse> directMarkAsRead($pb.ServerContext ctx, $6.DirectMarkAsReadParam request);
  $async.Future<$6.DirectMarkAsUnReadResponse> directMarkAsUnRead($pb.ServerContext ctx, $6.DirectMarkAsUnReadParam request);
  $async.Future<$6.DirectPinDirectsResponse> directPinDirects($pb.ServerContext ctx, $6.DirectPinDirectsParam request);
  $async.Future<$6.DirectUnPinDirectsResponse> directUnPinDirects($pb.ServerContext ctx, $6.DirectUnPinDirectsParam request);
  $async.Future<$6.DirectArchiveDirectsResponse> directArchiveDirects($pb.ServerContext ctx, $6.DirectArchiveDirectsParam request);
  $async.Future<$6.DirectUnArchiveDirectsResponse> directUnArchiveDirects($pb.ServerContext ctx, $6.DirectUnArchiveDirectsParam request);
  $async.Future<$6.DirectClearHistoriesResponse> directClearHistories($pb.ServerContext ctx, $6.DirectClearHistoriesParam request);
  $async.Future<$6.DirectMuteDirectsResponse> directMuteDirects($pb.ServerContext ctx, $6.DirectMuteDirectsParam request);
  $async.Future<$6.DirectUnMuteDirectsResponse> directUnMuteDirects($pb.ServerContext ctx, $6.DirectUnMuteDirectsParam request);
  $async.Future<$6.DirectCreateFolderResponse> directCreateFolder($pb.ServerContext ctx, $6.DirectCreateFolderParam request);
  $async.Future<$6.DirectChangeFolderResponse> directChangeFolder($pb.ServerContext ctx, $6.DirectChangeFolderParam request);
  $async.Future<$6.DirectRemoveFromFolderResponse> directRemoveFromFolder($pb.ServerContext ctx, $6.DirectRemoveFromFolderParam request);
  $async.Future<$6.DirectReordersFolderResponse> directReordersFolder($pb.ServerContext ctx, $6.DirectReordersFolderParam request);
  $async.Future<$6.DirectDeleteFolderResponse> directDeleteFolder($pb.ServerContext ctx, $6.DirectDeleteFolderParam request);
  $async.Future<$6.DirectGetChatsListResponse> directGetChatsList($pb.ServerContext ctx, $6.DirectGetChatsListParam request);
  $async.Future<$6.DirectGetGroupsListResponse> directGetGroupsList($pb.ServerContext ctx, $6.DirectGetGroupsListParam request);
  $async.Future<$6.DirectGetChannelsListResponse> directGetChannelsList($pb.ServerContext ctx, $6.DirectGetChannelsListParam request);
  $async.Future<$6.DirectGetFoldersListResponse> directGetFoldersList($pb.ServerContext ctx, $6.DirectGetFoldersListParam request);
  $async.Future<$6.DirectGetFoldersFullListResponse> directGetFoldersFullList($pb.ServerContext ctx, $6.DirectGetFoldersFullListParam request);

  $pb.GeneratedMessage createRequest($core.String method) {
    switch (method) {
      case 'DirectDeleteDirect': return $6.DirectDeleteDirectParam();
      case 'DirectChangeTitle': return $6.DirectChangeTitleParam();
      case 'DirectSetCustomNotification': return $6.DirectSetCustomNotificationParam();
      case 'DirectSendActionDoing': return $6.DirectSendActionDoingParam();
      case 'DirectSetDraft': return $6.DirectSetDraftParam();
      case 'DirectDeleteDirects': return $6.DirectDeleteDirectsParam();
      case 'DirectMarkAsRead': return $6.DirectMarkAsReadParam();
      case 'DirectMarkAsUnRead': return $6.DirectMarkAsUnReadParam();
      case 'DirectPinDirects': return $6.DirectPinDirectsParam();
      case 'DirectUnPinDirects': return $6.DirectUnPinDirectsParam();
      case 'DirectArchiveDirects': return $6.DirectArchiveDirectsParam();
      case 'DirectUnArchiveDirects': return $6.DirectUnArchiveDirectsParam();
      case 'DirectClearHistories': return $6.DirectClearHistoriesParam();
      case 'DirectMuteDirects': return $6.DirectMuteDirectsParam();
      case 'DirectUnMuteDirects': return $6.DirectUnMuteDirectsParam();
      case 'DirectCreateFolder': return $6.DirectCreateFolderParam();
      case 'DirectChangeFolder': return $6.DirectChangeFolderParam();
      case 'DirectRemoveFromFolder': return $6.DirectRemoveFromFolderParam();
      case 'DirectReordersFolder': return $6.DirectReordersFolderParam();
      case 'DirectDeleteFolder': return $6.DirectDeleteFolderParam();
      case 'DirectGetChatsList': return $6.DirectGetChatsListParam();
      case 'DirectGetGroupsList': return $6.DirectGetGroupsListParam();
      case 'DirectGetChannelsList': return $6.DirectGetChannelsListParam();
      case 'DirectGetFoldersList': return $6.DirectGetFoldersListParam();
      case 'DirectGetFoldersFullList': return $6.DirectGetFoldersFullListParam();
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $async.Future<$pb.GeneratedMessage> handleCall($pb.ServerContext ctx, $core.String method, $pb.GeneratedMessage request) {
    switch (method) {
      case 'DirectDeleteDirect': return this.directDeleteDirect(ctx, request);
      case 'DirectChangeTitle': return this.directChangeTitle(ctx, request);
      case 'DirectSetCustomNotification': return this.directSetCustomNotification(ctx, request);
      case 'DirectSendActionDoing': return this.directSendActionDoing(ctx, request);
      case 'DirectSetDraft': return this.directSetDraft(ctx, request);
      case 'DirectDeleteDirects': return this.directDeleteDirects(ctx, request);
      case 'DirectMarkAsRead': return this.directMarkAsRead(ctx, request);
      case 'DirectMarkAsUnRead': return this.directMarkAsUnRead(ctx, request);
      case 'DirectPinDirects': return this.directPinDirects(ctx, request);
      case 'DirectUnPinDirects': return this.directUnPinDirects(ctx, request);
      case 'DirectArchiveDirects': return this.directArchiveDirects(ctx, request);
      case 'DirectUnArchiveDirects': return this.directUnArchiveDirects(ctx, request);
      case 'DirectClearHistories': return this.directClearHistories(ctx, request);
      case 'DirectMuteDirects': return this.directMuteDirects(ctx, request);
      case 'DirectUnMuteDirects': return this.directUnMuteDirects(ctx, request);
      case 'DirectCreateFolder': return this.directCreateFolder(ctx, request);
      case 'DirectChangeFolder': return this.directChangeFolder(ctx, request);
      case 'DirectRemoveFromFolder': return this.directRemoveFromFolder(ctx, request);
      case 'DirectReordersFolder': return this.directReordersFolder(ctx, request);
      case 'DirectDeleteFolder': return this.directDeleteFolder(ctx, request);
      case 'DirectGetChatsList': return this.directGetChatsList(ctx, request);
      case 'DirectGetGroupsList': return this.directGetGroupsList(ctx, request);
      case 'DirectGetChannelsList': return this.directGetChannelsList(ctx, request);
      case 'DirectGetFoldersList': return this.directGetFoldersList(ctx, request);
      case 'DirectGetFoldersFullList': return this.directGetFoldersFullList(ctx, request);
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $core.Map<$core.String, $core.dynamic> get $json => RPC_DirectServiceBase$json;
  $core.Map<$core.String, $core.Map<$core.String, $core.dynamic>> get $messageJson => RPC_DirectServiceBase$messageJson;
}

