///
//  Generated code. Do not modify.
//  source: rpc_social.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;

import 'package:protobuf/protobuf.dart' as $pb;

import 'dart:core' as $core;
import 'rpc_social.pb.dart' as $9;
import 'rpc_social.pbjson.dart';

export 'rpc_social.pb.dart';

abstract class RPC_SocialServiceBase extends $pb.GeneratedService {
  $async.Future<$9.AddCommentResponse> addComment($pb.ServerContext ctx, $9.AddCommentParam request);
  $async.Future<$9.DeleteCommentResponse> deleteComment($pb.ServerContext ctx, $9.DeleteCommentParam request);
  $async.Future<$9.EditCommentResponse> editComment($pb.ServerContext ctx, $9.EditCommentParam request);
  $async.Future<$9.LikeCommentResponse> likeComment($pb.ServerContext ctx, $9.LikeCommentParam request);
  $async.Future<$9.AddSeenPostsResponse> addSeenPosts($pb.ServerContext ctx, $9.AddSeenPostsParam request);
  $async.Future<$9.LikePostResponse> likePost($pb.ServerContext ctx, $9.LikePostParam request);
  $async.Future<$9.UnLikePostResponse> unLikePost($pb.ServerContext ctx, $9.UnLikePostParam request);
  $async.Future<$9.FollowChannelResponse> followChannel($pb.ServerContext ctx, $9.FollowChannelParam request);
  $async.Future<$9.UnFollowChannelResponse> unFollowChannel($pb.ServerContext ctx, $9.UnFollowChannelParam request);
  $async.Future<$9.PinChannelResponse> pinChannel($pb.ServerContext ctx, $9.PinChannelParam request);
  $async.Future<$9.UnPinChannelResponse> unPinChannel($pb.ServerContext ctx, $9.UnPinChannelParam request);
  $async.Future<$9.BlockChannelResponse> blockChannel($pb.ServerContext ctx, $9.BlockChannelParam request);
  $async.Future<$9.UnBlockChannelResponse> unBlockChannel($pb.ServerContext ctx, $9.UnBlockChannelParam request);

  $pb.GeneratedMessage createRequest($core.String method) {
    switch (method) {
      case 'AddComment': return $9.AddCommentParam();
      case 'DeleteComment': return $9.DeleteCommentParam();
      case 'EditComment': return $9.EditCommentParam();
      case 'LikeComment': return $9.LikeCommentParam();
      case 'AddSeenPosts': return $9.AddSeenPostsParam();
      case 'LikePost': return $9.LikePostParam();
      case 'UnLikePost': return $9.UnLikePostParam();
      case 'FollowChannel': return $9.FollowChannelParam();
      case 'UnFollowChannel': return $9.UnFollowChannelParam();
      case 'PinChannel': return $9.PinChannelParam();
      case 'UnPinChannel': return $9.UnPinChannelParam();
      case 'BlockChannel': return $9.BlockChannelParam();
      case 'UnBlockChannel': return $9.UnBlockChannelParam();
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $async.Future<$pb.GeneratedMessage> handleCall($pb.ServerContext ctx, $core.String method, $pb.GeneratedMessage request) {
    switch (method) {
      case 'AddComment': return this.addComment(ctx, request);
      case 'DeleteComment': return this.deleteComment(ctx, request);
      case 'EditComment': return this.editComment(ctx, request);
      case 'LikeComment': return this.likeComment(ctx, request);
      case 'AddSeenPosts': return this.addSeenPosts(ctx, request);
      case 'LikePost': return this.likePost(ctx, request);
      case 'UnLikePost': return this.unLikePost(ctx, request);
      case 'FollowChannel': return this.followChannel(ctx, request);
      case 'UnFollowChannel': return this.unFollowChannel(ctx, request);
      case 'PinChannel': return this.pinChannel(ctx, request);
      case 'UnPinChannel': return this.unPinChannel(ctx, request);
      case 'BlockChannel': return this.blockChannel(ctx, request);
      case 'UnBlockChannel': return this.unBlockChannel(ctx, request);
      default: throw $core.ArgumentError('Unknown method: $method');
    }
  }

  $core.Map<$core.String, $core.dynamic> get $json => RPC_SocialServiceBase$json;
  $core.Map<$core.String, $core.Map<$core.String, $core.dynamic>> get $messageJson => RPC_SocialServiceBase$messageJson;
}

