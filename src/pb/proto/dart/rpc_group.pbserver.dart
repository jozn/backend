///
//  Generated code. Do not modify.
//  source: rpc_group.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;

import 'package:protobuf/protobuf.dart' as $pb;

import 'dart:core' as $core;
import 'rpc_group.pb.dart' as $8;
import 'rpc_group.pbjson.dart';

export 'rpc_group.pb.dart';

abstract class RPC_GroupServiceBase extends $pb.GeneratedService {
  $async.Future<$8.GroupCreateGroupResponse> groupCreateGroup($pb.ServerContext ctx, $8.GroupCreateGroupParam request);
  $async.Future<$8.GroupEditGroupResponse> groupEditGroup($pb.ServerContext ctx, $8.GroupEditGroupParam request);
  $async.Future<$8.GroupDeleteGroupResponse> groupDeleteGroup($pb.ServerContext ctx, $8.GroupDeleteGroupParam request);
  $async.Future<$8.GroupAddAdminResponse> groupAddAdmin($pb.ServerContext ctx, $8.GroupAddAdminParam request);
  $async.Future<$8.GroupAddMemberResponse> groupAddMember($pb.ServerContext ctx, $8.GroupAddMemberParam request);
  $async.Future<$8.GroupRemoveMemberResponse> groupRemoveMember($pb.ServerContext ctx, $8.GroupRemoveMemberParam request);
  $async.Future<$8.GroupChangeMemberLevelResponse> groupChangeMemberLevel($pb.ServerContext ctx, $8.GroupChangeMemberLevelParam request);
  $async.Future<$8.GroupChangeMemberPermissionResponse> groupChangeMemberPermission($pb.ServerContext ctx, $8.GroupChangeMemberPermissionParam request);
  $async.Future<$8.JoinGroupResponse> groupJoinGroup($pb.ServerContext ctx, $8.JoinGroupParam request);
  $async.Future<$8.GroupLeaveGroupResponse> groupLeaveGroup($pb.ServerContext ctx, $8.GroupLeaveGroupParam request);
  $async.Future<$8.GroupBanMemberResponse> groupBanMember($pb.ServerContext ctx, $8.GroupBanMemberParam request);
  $async.Future<$8.GroupChangePrivacyResponse> groupChangePrivacy($pb.ServerContext ctx, $8.GroupChangePrivacyParam request);
  $async.Future<$8.GroupChangeDefaultPermissionResponse> groupChangeDefaultPermission($pb.ServerContext ctx, $8.GroupChangeDefaultPermissionParam request);
  $async.Future<$8.GroupRevokeLinkResponse> groupRevokeLink($pb.ServerContext ctx, $8.GroupRevokeLinkParam request);
  $async.Future<$8.GroupChangeUsernameResponse> groupChangeUsername($pb.ServerContext ctx, $8.GroupChangeUsernameParam request);
  $async.Future<$8.GroupSendMessageResponse> groupSendMessage($pb.ServerContext ctx, $8.GroupSendMessageParam request);
  $async.Future<$8.GroupEditMessageResponse> groupEditMessage($pb.ServerContext ctx, $8.GroupEditMessageParam request);
  $async.Future<$8.GroupPinMessageResponse> groupPinMessage($pb.ServerContext ctx, $8.GroupPinMessageParam request);
  $async.Future<$8.GroupUnPinMessageResponse> groupUnPinMessage($pb.ServerContext ctx, $8.GroupUnPinMessageParam request);
  $async.Future<$8.GroupDeleteMessageResponse> groupDeleteMessage($pb.ServerContext ctx, $8.GroupDeleteMessageParam request);
  $async.Future<$8.GroupDeleteMessagesResponse> groupDeleteMessages($pb.ServerContext ctx, $8.GroupDeleteMessagesParam request);
  $async.Future<$8.GroupDeleteHistoryResponse> groupDeleteHistory($pb.ServerContext ctx, $8.GroupDeleteHistoryParam request);
  $async.Future<$8.GroupClearHistoryResponse> groupClearHistory($pb.ServerContext ctx, $8.GroupClearHistoryParam request);
  $async.Future<$8.GroupAvatarAddResponse> groupAvatarAdd($pb.ServerContext ctx, $8.GroupAvatarAddParam request);
  $async.Future<$8.GroupAvatarChangeResponse> groupAvatarChange($pb.ServerContext ctx, $8.GroupAvatarChangeParam request);
  $async.Future<$8.GroupAvatarDeleteResponse> groupAvatarDelete($pb.ServerContext ctx, $8.GroupAvatarDeleteParam request);
  $async.Future<$8.GroupAvatarGetListResponse> groupAvatarGetList($pb.ServerContext ctx, $8.GroupAvatarGetListParam request);
  $async.Future<$8.GroupSendDoingActionResponse> groupSendDoingAction($pb.ServerContext ctx, $8.GroupSendDoingActionParam request);
  $async.Future<$8.GroupReportGroupResponse> groupReportGroup($pb.ServerContext ctx, $8.GroupReportGroupParam request);
  $async.Future<$8.GroupGetFullMessageResponse> groupGetFull($pb.ServerContext ctx, $8.GroupGetFullMessageParam request);
  $async.Future<$8.GroupGetMessagesListResponse> groupGetMessagesList($pb.ServerContext ctx, $8.GroupGetMessagesListParam request);
  $async.Future<$8.GroupGetMediaListResponse> groupGetMediaList($pb.ServerContext ctx, $8.GroupGetMediaListParam request);
  $async.Future<$8.GroupGetMembersListResponse> groupGetMembersList($pb.ServerContext ctx, $8.GroupGetMembersListParam request);
  $async.Future<$8.GroupGetAdminsListResponse> groupGetAdminsList($pb.ServerContext ctx, $8.GroupGetAdminsListParam request);
  $async.Future<$8.GroupSetDraftResponse> groupSetDraft($pb.ServerContext ctx, $8.GroupSetDraftParam request);

  $pb.GeneratedMessage createRequest($core.String method) {
    switch (method) {
      case 'GroupCreateGroup': return $8.GroupCreateGroupParam();
      case 'GroupEditGroup': return $8.GroupEditGroupParam();
      case 'GroupDeleteGroup': return $8.GroupDeleteGroupParam();
      case 'GroupAddAdmin': return $8.GroupAddAdminParam();
      case 'GroupAddMember': return $8.GroupAddMemberParam();
      case 'GroupRemoveMember': return $8.GroupRemoveMemberParam();
      case 'GroupChangeMemberLevel': return $8.GroupChangeMemberLevelParam();
      case 'GroupChangeMemberPermission': return $8.GroupChangeMemberPermissionParam();
      case 'GroupJoinGroup': return $8.JoinGroupParam();
      case 'GroupLeaveGroup': return $8.GroupLeaveGroupParam();
      case 'GroupBanMember': return $8.GroupBanMemberParam();
      case 'GroupChangePrivacy': return $8.GroupChangePrivacyParam();
      case 'GroupChangeDefaultPermission': return $8.GroupChangeDefaultPermissionParam();
      case 'GroupRevokeLink': return $8.GroupRevokeLinkParam();
      case 'GroupChangeUsername': return $8.GroupChangeUsernameParam();
      case 'GroupSendMessage': return $8.GroupSendMessageParam();
      case 'GroupEditMessage': return $8.GroupEditMessageParam();
      case 'GroupPinMessage': return $8.GroupPinMessageParam();
      case 'GroupUnPinMessage': return $8.GroupUnPinMessageParam();
      case 'GroupDeleteMessage': return $8.GroupDeleteMessageParam();
      case 'GroupDeleteMessages': return $8.GroupDeleteMessagesParam();
      case 'GroupDeleteHistory': return $8.GroupDeleteHistoryParam();
      case 'GroupClearHistory': return $8.GroupClearHistoryParam();
      case 'GroupAvatarAdd': return $8.GroupAvatarAddParam();
      case 'GroupAvatarChange': return $8.GroupAvatarChangeParam();
      case 'GroupAvatarDelete': return $8.GroupAvatarDeleteParam();
      case 'GroupAvatarGetList': return $8.GroupAvatarGetListParam();
      case 'GroupSendDoingAction': return $8.GroupSendDoingActionParam();
      case 'GroupReportGroup': return $8.GroupReportGroupParam();
      case 'GroupGetFull': return $8.GroupGetFullMessageParam();
      case 'GroupGetMessagesList': return $8.GroupGetMessagesListParam();
      case 'GroupGetMediaList': return $8.GroupGetMediaListParam();
      case 'GroupGetMembersList': return $8.GroupGetMembersListParam();
      case 'GroupGetAdminsList': return $8.GroupGetAdminsListParam();
      case 'GroupSetDraft': return $8.GroupSetDraftParam();
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $async.Future<$pb.GeneratedMessage> handleCall($pb.ServerContext ctx, $core.String method, $pb.GeneratedMessage request) {
    switch (method) {
      case 'GroupCreateGroup': return this.groupCreateGroup(ctx, request);
      case 'GroupEditGroup': return this.groupEditGroup(ctx, request);
      case 'GroupDeleteGroup': return this.groupDeleteGroup(ctx, request);
      case 'GroupAddAdmin': return this.groupAddAdmin(ctx, request);
      case 'GroupAddMember': return this.groupAddMember(ctx, request);
      case 'GroupRemoveMember': return this.groupRemoveMember(ctx, request);
      case 'GroupChangeMemberLevel': return this.groupChangeMemberLevel(ctx, request);
      case 'GroupChangeMemberPermission': return this.groupChangeMemberPermission(ctx, request);
      case 'GroupJoinGroup': return this.groupJoinGroup(ctx, request);
      case 'GroupLeaveGroup': return this.groupLeaveGroup(ctx, request);
      case 'GroupBanMember': return this.groupBanMember(ctx, request);
      case 'GroupChangePrivacy': return this.groupChangePrivacy(ctx, request);
      case 'GroupChangeDefaultPermission': return this.groupChangeDefaultPermission(ctx, request);
      case 'GroupRevokeLink': return this.groupRevokeLink(ctx, request);
      case 'GroupChangeUsername': return this.groupChangeUsername(ctx, request);
      case 'GroupSendMessage': return this.groupSendMessage(ctx, request);
      case 'GroupEditMessage': return this.groupEditMessage(ctx, request);
      case 'GroupPinMessage': return this.groupPinMessage(ctx, request);
      case 'GroupUnPinMessage': return this.groupUnPinMessage(ctx, request);
      case 'GroupDeleteMessage': return this.groupDeleteMessage(ctx, request);
      case 'GroupDeleteMessages': return this.groupDeleteMessages(ctx, request);
      case 'GroupDeleteHistory': return this.groupDeleteHistory(ctx, request);
      case 'GroupClearHistory': return this.groupClearHistory(ctx, request);
      case 'GroupAvatarAdd': return this.groupAvatarAdd(ctx, request);
      case 'GroupAvatarChange': return this.groupAvatarChange(ctx, request);
      case 'GroupAvatarDelete': return this.groupAvatarDelete(ctx, request);
      case 'GroupAvatarGetList': return this.groupAvatarGetList(ctx, request);
      case 'GroupSendDoingAction': return this.groupSendDoingAction(ctx, request);
      case 'GroupReportGroup': return this.groupReportGroup(ctx, request);
      case 'GroupGetFull': return this.groupGetFull(ctx, request);
      case 'GroupGetMessagesList': return this.groupGetMessagesList(ctx, request);
      case 'GroupGetMediaList': return this.groupGetMediaList(ctx, request);
      case 'GroupGetMembersList': return this.groupGetMembersList(ctx, request);
      case 'GroupGetAdminsList': return this.groupGetAdminsList(ctx, request);
      case 'GroupSetDraft': return this.groupSetDraft(ctx, request);
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $core.Map<$core.String, $core.dynamic> get $json => RPC_GroupServiceBase$json;
  $core.Map<$core.String, $core.Map<$core.String, $core.dynamic>> get $messageJson => RPC_GroupServiceBase$messageJson;
}

