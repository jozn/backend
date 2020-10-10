///
//  Generated code. Do not modify.
//  source: rpc_channel.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;

import 'package:protobuf/protobuf.dart' as $pb;

import 'dart:core' as $core;
import 'rpc_channel.pb.dart' as $4;
import 'rpc_channel.pbjson.dart';

export 'rpc_channel.pb.dart';

abstract class RPC_ChannelServiceBase extends $pb.GeneratedService {
  $async.Future<$4.ChannelCreateChannelResponse> channelCreateChannel($pb.ServerContext ctx, $4.ChannelCreateChannelParam request);
  $async.Future<$4.ChannelEditChannelResponse> channelEditChannel($pb.ServerContext ctx, $4.ChannelEditChannelParam request);
  $async.Future<$4.ChannelDeleteChannelResponse> channelDeleteChannel($pb.ServerContext ctx, $4.ChannelDeleteChannelParam request);
  $async.Future<$4.ChannelAddAuthorResponse> channelAddAuthor($pb.ServerContext ctx, $4.ChannelAddAuthorParam request);
  $async.Future<$4.ChannelChangeAuthorPermissionResponse> channelChangeAuthorPermission($pb.ServerContext ctx, $4.ChannelChangeAuthorPermissionParam request);
  $async.Future<$4.ChannelRemoveAuthorResponse> channelRemoveAuthor($pb.ServerContext ctx, $4.ChannelRemoveAuthorParam request);
  $async.Future<$4.ChannelFollowChannelResponse> channelFollowChannel($pb.ServerContext ctx, $4.ChannelFollowChannelParam request);
  $async.Future<$4.ChannelUnFollowChannelResponse> channelUnFollowChannel($pb.ServerContext ctx, $4.ChannelUnFollowChannelParam request);
  $async.Future<$4.ChannelRemoveFollowersResponse> channelRemoveFollowers($pb.ServerContext ctx, $4.ChannelRemoveFollowersParam request);
  $async.Future<$4.ChannelSubscribeResponse> channelSubscribe($pb.ServerContext ctx, $4.ChannelSubscribeParam request);
  $async.Future<$4.ChannelUnSubscribeResponse> channelUnSubscribe($pb.ServerContext ctx, $4.ChannelUnSubscribeParam request);
  $async.Future<$4.ChannelRemoveSubscribersResponse> channelRemoveSubscribers($pb.ServerContext ctx, $4.ChannelRemoveSubscribersParam request);
  $async.Future<$4.ChannelChangePrivacyResponse> channelChangePrivacy($pb.ServerContext ctx, $4.ChannelChangePrivacyParam request);
  $async.Future<$4.ChannelChangeDefaultPermissionResponse> channelChangeDefaultPermission($pb.ServerContext ctx, $4.ChannelChangeDefaultPermissionParam request);
  $async.Future<$4.ChannelRevokeLinkResponse> channelRevokeLink($pb.ServerContext ctx, $4.ChannelRevokeLinkParam request);
  $async.Future<$4.ChannelChangeUsernameResponse> channelChangeUsername($pb.ServerContext ctx, $4.ChannelChangeUsernameParam request);
  $async.Future<$4.ChannelBlockChannelResponse> channelBlockChannel($pb.ServerContext ctx, $4.ChannelBlockChannelParam request);
  $async.Future<$4.ChannelSendMessageResponse> channelSendMessage($pb.ServerContext ctx, $4.ChannelSendMessageParam request);
  $async.Future<$4.ChannelEditMessageResponse> channelEditMessage($pb.ServerContext ctx, $4.ChannelEditMessageParam request);
  $async.Future<$4.ChannelPinMessageResponse> channelPinMessage($pb.ServerContext ctx, $4.ChannelPinMessageParam request);
  $async.Future<$4.ChannelUnPinMessageResponse> channelUnPinMessage($pb.ServerContext ctx, $4.ChannelUnPinMessageParam request);
  $async.Future<$4.ChannelDeleteMessageResponse> channelDeleteMessage($pb.ServerContext ctx, $4.ChannelDeleteMessageParam request);
  $async.Future<$4.ChannelDeleteMessagesResponse> channelDeleteMessages($pb.ServerContext ctx, $4.ChannelDeleteMessagesParam request);
  $async.Future<$4.ChannelClearHistoryResponse> channelClearHistory($pb.ServerContext ctx, $4.ChannelClearHistoryParam request);
  $async.Future<$4.ChannelAvatarAddResponse> channelAvatarAdd($pb.ServerContext ctx, $4.ChannelAvatarAddParam request);
  $async.Future<$4.ChannelAvatarChangeResponse> channelAvatarChange($pb.ServerContext ctx, $4.ChannelAvatarChangeParam request);
  $async.Future<$4.ChannelAvatarDeleteResponse> channelAvatarDelete($pb.ServerContext ctx, $4.ChannelAvatarDeleteParam request);
  $async.Future<$4.ChannelAvatarGetListResponse> channelAvatarGetList($pb.ServerContext ctx, $4.ChannelAvatarGetListParam request);
  $async.Future<$4.ChannelSendDoingActionResponse> channelSendDoingAction($pb.ServerContext ctx, $4.ChannelSendDoingActionParam request);
  $async.Future<$4.ChannelReportChannelResponse> channelReportChannel($pb.ServerContext ctx, $4.ChannelReportChannelParam request);
  $async.Future<$4.ChannelReportMessageResponse> channelReportMessage($pb.ServerContext ctx, $4.ChannelReportMessageParam request);
  $async.Future<$4.ChannelGetFullResponse> channelGetFull($pb.ServerContext ctx, $4.ChannelGetFullParam request);
  $async.Future<$4.ChannelGetMessagesListResponse> channelGetMessagesList($pb.ServerContext ctx, $4.ChannelGetMessagesListParam request);
  $async.Future<$4.ChannelGetMediaListResponse> channelGetMediaList($pb.ServerContext ctx, $4.ChannelGetMediaListParam request);
  $async.Future<$4.ChannelGetAuthorsResponse> channelGetAuthors($pb.ServerContext ctx, $4.ChannelGetAuthorsParam request);
  $async.Future<$4.ChannelGetFollowersResponse> channelGetFollowers($pb.ServerContext ctx, $4.ChannelGetFollowersParam request);
  $async.Future<$4.ChannelGetFollowingsResponse> channelGetFollowings($pb.ServerContext ctx, $4.ChannelGetFollowingsParam request);
  $async.Future<$4.ChannelGetSubscribersResponse> channelGetSubscribers($pb.ServerContext ctx, $4.ChannelGetSubscribersParam request);
  $async.Future<$4.ChannelBlockedResponse> channelBlocked($pb.ServerContext ctx, $4.ChannelBlockedParam request);
  $async.Future<$4.ChannelSetDraftResponse> channelSetDraft($pb.ServerContext ctx, $4.ChannelSetDraftParam request);

  $pb.GeneratedMessage createRequest($core.String method) {
    switch (method) {
      case 'ChannelCreateChannel': return $4.ChannelCreateChannelParam();
      case 'ChannelEditChannel': return $4.ChannelEditChannelParam();
      case 'ChannelDeleteChannel': return $4.ChannelDeleteChannelParam();
      case 'ChannelAddAuthor': return $4.ChannelAddAuthorParam();
      case 'ChannelChangeAuthorPermission': return $4.ChannelChangeAuthorPermissionParam();
      case 'ChannelRemoveAuthor': return $4.ChannelRemoveAuthorParam();
      case 'ChannelFollowChannel': return $4.ChannelFollowChannelParam();
      case 'ChannelUnFollowChannel': return $4.ChannelUnFollowChannelParam();
      case 'ChannelRemoveFollowers': return $4.ChannelRemoveFollowersParam();
      case 'ChannelSubscribe': return $4.ChannelSubscribeParam();
      case 'ChannelUnSubscribe': return $4.ChannelUnSubscribeParam();
      case 'ChannelRemoveSubscribers': return $4.ChannelRemoveSubscribersParam();
      case 'ChannelChangePrivacy': return $4.ChannelChangePrivacyParam();
      case 'ChannelChangeDefaultPermission': return $4.ChannelChangeDefaultPermissionParam();
      case 'ChannelRevokeLink': return $4.ChannelRevokeLinkParam();
      case 'ChannelChangeUsername': return $4.ChannelChangeUsernameParam();
      case 'ChannelBlockChannel': return $4.ChannelBlockChannelParam();
      case 'ChannelSendMessage': return $4.ChannelSendMessageParam();
      case 'ChannelEditMessage': return $4.ChannelEditMessageParam();
      case 'ChannelPinMessage': return $4.ChannelPinMessageParam();
      case 'ChannelUnPinMessage': return $4.ChannelUnPinMessageParam();
      case 'ChannelDeleteMessage': return $4.ChannelDeleteMessageParam();
      case 'ChannelDeleteMessages': return $4.ChannelDeleteMessagesParam();
      case 'ChannelClearHistory': return $4.ChannelClearHistoryParam();
      case 'ChannelAvatarAdd': return $4.ChannelAvatarAddParam();
      case 'ChannelAvatarChange': return $4.ChannelAvatarChangeParam();
      case 'ChannelAvatarDelete': return $4.ChannelAvatarDeleteParam();
      case 'ChannelAvatarGetList': return $4.ChannelAvatarGetListParam();
      case 'ChannelSendDoingAction': return $4.ChannelSendDoingActionParam();
      case 'ChannelReportChannel': return $4.ChannelReportChannelParam();
      case 'ChannelReportMessage': return $4.ChannelReportMessageParam();
      case 'ChannelGetFull': return $4.ChannelGetFullParam();
      case 'ChannelGetMessagesList': return $4.ChannelGetMessagesListParam();
      case 'ChannelGetMediaList': return $4.ChannelGetMediaListParam();
      case 'ChannelGetAuthors': return $4.ChannelGetAuthorsParam();
      case 'ChannelGetFollowers': return $4.ChannelGetFollowersParam();
      case 'ChannelGetFollowings': return $4.ChannelGetFollowingsParam();
      case 'ChannelGetSubscribers': return $4.ChannelGetSubscribersParam();
      case 'ChannelBlocked': return $4.ChannelBlockedParam();
      case 'ChannelSetDraft': return $4.ChannelSetDraftParam();
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $async.Future<$pb.GeneratedMessage> handleCall($pb.ServerContext ctx, $core.String method, $pb.GeneratedMessage request) {
    switch (method) {
      case 'ChannelCreateChannel': return this.channelCreateChannel(ctx, request);
      case 'ChannelEditChannel': return this.channelEditChannel(ctx, request);
      case 'ChannelDeleteChannel': return this.channelDeleteChannel(ctx, request);
      case 'ChannelAddAuthor': return this.channelAddAuthor(ctx, request);
      case 'ChannelChangeAuthorPermission': return this.channelChangeAuthorPermission(ctx, request);
      case 'ChannelRemoveAuthor': return this.channelRemoveAuthor(ctx, request);
      case 'ChannelFollowChannel': return this.channelFollowChannel(ctx, request);
      case 'ChannelUnFollowChannel': return this.channelUnFollowChannel(ctx, request);
      case 'ChannelRemoveFollowers': return this.channelRemoveFollowers(ctx, request);
      case 'ChannelSubscribe': return this.channelSubscribe(ctx, request);
      case 'ChannelUnSubscribe': return this.channelUnSubscribe(ctx, request);
      case 'ChannelRemoveSubscribers': return this.channelRemoveSubscribers(ctx, request);
      case 'ChannelChangePrivacy': return this.channelChangePrivacy(ctx, request);
      case 'ChannelChangeDefaultPermission': return this.channelChangeDefaultPermission(ctx, request);
      case 'ChannelRevokeLink': return this.channelRevokeLink(ctx, request);
      case 'ChannelChangeUsername': return this.channelChangeUsername(ctx, request);
      case 'ChannelBlockChannel': return this.channelBlockChannel(ctx, request);
      case 'ChannelSendMessage': return this.channelSendMessage(ctx, request);
      case 'ChannelEditMessage': return this.channelEditMessage(ctx, request);
      case 'ChannelPinMessage': return this.channelPinMessage(ctx, request);
      case 'ChannelUnPinMessage': return this.channelUnPinMessage(ctx, request);
      case 'ChannelDeleteMessage': return this.channelDeleteMessage(ctx, request);
      case 'ChannelDeleteMessages': return this.channelDeleteMessages(ctx, request);
      case 'ChannelClearHistory': return this.channelClearHistory(ctx, request);
      case 'ChannelAvatarAdd': return this.channelAvatarAdd(ctx, request);
      case 'ChannelAvatarChange': return this.channelAvatarChange(ctx, request);
      case 'ChannelAvatarDelete': return this.channelAvatarDelete(ctx, request);
      case 'ChannelAvatarGetList': return this.channelAvatarGetList(ctx, request);
      case 'ChannelSendDoingAction': return this.channelSendDoingAction(ctx, request);
      case 'ChannelReportChannel': return this.channelReportChannel(ctx, request);
      case 'ChannelReportMessage': return this.channelReportMessage(ctx, request);
      case 'ChannelGetFull': return this.channelGetFull(ctx, request);
      case 'ChannelGetMessagesList': return this.channelGetMessagesList(ctx, request);
      case 'ChannelGetMediaList': return this.channelGetMediaList(ctx, request);
      case 'ChannelGetAuthors': return this.channelGetAuthors(ctx, request);
      case 'ChannelGetFollowers': return this.channelGetFollowers(ctx, request);
      case 'ChannelGetFollowings': return this.channelGetFollowings(ctx, request);
      case 'ChannelGetSubscribers': return this.channelGetSubscribers(ctx, request);
      case 'ChannelBlocked': return this.channelBlocked(ctx, request);
      case 'ChannelSetDraft': return this.channelSetDraft(ctx, request);
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $core.Map<$core.String, $core.dynamic> get $json => RPC_ChannelServiceBase$json;
  $core.Map<$core.String, $core.Map<$core.String, $core.dynamic>> get $messageJson => RPC_ChannelServiceBase$messageJson;
}

