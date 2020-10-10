///
//  Generated code. Do not modify.
//  source: views.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'enums.pbenum.dart' as $1;
import 'views.pbenum.dart';

export 'views.pbenum.dart';

class MediaView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('MediaView', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  MediaView._() : super();
  factory MediaView() => create();
  factory MediaView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MediaView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  MediaView clone() => MediaView()..mergeFromMessage(this);
  MediaView copyWith(void Function(MediaView) updates) => super.copyWith((message) => updates(message as MediaView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MediaView create() => MediaView._();
  MediaView createEmptyInstance() => create();
  static $pb.PbList<MediaView> createRepeated() => $pb.PbList<MediaView>();
  @$core.pragma('dart2js:noInline')
  static MediaView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MediaView>(create);
  static MediaView _defaultInstance;
}

class ActionView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ActionView', createEmptyInstance: create)
    ..aInt64(1, 'ActionId', protoName: 'ActionId')
    ..a<$core.int>(2, 'ActorUserId', $pb.PbFieldType.O3, protoName: 'ActorUserId')
    ..a<$core.int>(3, 'ActionTypeEnum', $pb.PbFieldType.O3, protoName: 'ActionTypeEnum')
    ..a<$core.int>(4, 'PeerUserId', $pb.PbFieldType.O3, protoName: 'PeerUserId')
    ..aInt64(5, 'PostId', protoName: 'PostId')
    ..aInt64(6, 'CommentId', protoName: 'CommentId')
    ..aInt64(7, 'Murmur64Hash', protoName: 'Murmur64Hash')
    ..a<$core.int>(8, 'CreatedTime', $pb.PbFieldType.O3, protoName: 'CreatedTime')
    ..aOM<UserView>(100, 'ActorUserView', protoName: 'ActorUserView', subBuilder: UserView.create)
    ..aOM<PostView>(101, 'PostView', protoName: 'PostView', subBuilder: PostView.create)
    ..aOM<CommentView>(102, 'CommentView', protoName: 'CommentView', subBuilder: CommentView.create)
    ..aOM<UserView>(103, 'FollowedUserView', protoName: 'FollowedUserView', subBuilder: UserView.create)
    ..aOM<UserView>(104, 'ContentOwenerUserView', protoName: 'ContentOwenerUserView', subBuilder: UserView.create)
    ..hasRequiredFields = false
  ;

  ActionView._() : super();
  factory ActionView() => create();
  factory ActionView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ActionView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ActionView clone() => ActionView()..mergeFromMessage(this);
  ActionView copyWith(void Function(ActionView) updates) => super.copyWith((message) => updates(message as ActionView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ActionView create() => ActionView._();
  ActionView createEmptyInstance() => create();
  static $pb.PbList<ActionView> createRepeated() => $pb.PbList<ActionView>();
  @$core.pragma('dart2js:noInline')
  static ActionView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ActionView>(create);
  static ActionView _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get actionId => $_getI64(0);
  @$pb.TagNumber(1)
  set actionId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasActionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearActionId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get actorUserId => $_getIZ(1);
  @$pb.TagNumber(2)
  set actorUserId($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasActorUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearActorUserId() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get actionTypeEnum => $_getIZ(2);
  @$pb.TagNumber(3)
  set actionTypeEnum($core.int v) { $_setSignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasActionTypeEnum() => $_has(2);
  @$pb.TagNumber(3)
  void clearActionTypeEnum() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get peerUserId => $_getIZ(3);
  @$pb.TagNumber(4)
  set peerUserId($core.int v) { $_setSignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasPeerUserId() => $_has(3);
  @$pb.TagNumber(4)
  void clearPeerUserId() => clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get postId => $_getI64(4);
  @$pb.TagNumber(5)
  set postId($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasPostId() => $_has(4);
  @$pb.TagNumber(5)
  void clearPostId() => clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get commentId => $_getI64(5);
  @$pb.TagNumber(6)
  set commentId($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasCommentId() => $_has(5);
  @$pb.TagNumber(6)
  void clearCommentId() => clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get murmur64Hash => $_getI64(6);
  @$pb.TagNumber(7)
  set murmur64Hash($fixnum.Int64 v) { $_setInt64(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasMurmur64Hash() => $_has(6);
  @$pb.TagNumber(7)
  void clearMurmur64Hash() => clearField(7);

  @$pb.TagNumber(8)
  $core.int get createdTime => $_getIZ(7);
  @$pb.TagNumber(8)
  set createdTime($core.int v) { $_setSignedInt32(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasCreatedTime() => $_has(7);
  @$pb.TagNumber(8)
  void clearCreatedTime() => clearField(8);

  @$pb.TagNumber(100)
  UserView get actorUserView => $_getN(8);
  @$pb.TagNumber(100)
  set actorUserView(UserView v) { setField(100, v); }
  @$pb.TagNumber(100)
  $core.bool hasActorUserView() => $_has(8);
  @$pb.TagNumber(100)
  void clearActorUserView() => clearField(100);
  @$pb.TagNumber(100)
  UserView ensureActorUserView() => $_ensure(8);

  @$pb.TagNumber(101)
  PostView get postView => $_getN(9);
  @$pb.TagNumber(101)
  set postView(PostView v) { setField(101, v); }
  @$pb.TagNumber(101)
  $core.bool hasPostView() => $_has(9);
  @$pb.TagNumber(101)
  void clearPostView() => clearField(101);
  @$pb.TagNumber(101)
  PostView ensurePostView() => $_ensure(9);

  @$pb.TagNumber(102)
  CommentView get commentView => $_getN(10);
  @$pb.TagNumber(102)
  set commentView(CommentView v) { setField(102, v); }
  @$pb.TagNumber(102)
  $core.bool hasCommentView() => $_has(10);
  @$pb.TagNumber(102)
  void clearCommentView() => clearField(102);
  @$pb.TagNumber(102)
  CommentView ensureCommentView() => $_ensure(10);

  @$pb.TagNumber(103)
  UserView get followedUserView => $_getN(11);
  @$pb.TagNumber(103)
  set followedUserView(UserView v) { setField(103, v); }
  @$pb.TagNumber(103)
  $core.bool hasFollowedUserView() => $_has(11);
  @$pb.TagNumber(103)
  void clearFollowedUserView() => clearField(103);
  @$pb.TagNumber(103)
  UserView ensureFollowedUserView() => $_ensure(11);

  @$pb.TagNumber(104)
  UserView get contentOwenerUserView => $_getN(12);
  @$pb.TagNumber(104)
  set contentOwenerUserView(UserView v) { setField(104, v); }
  @$pb.TagNumber(104)
  $core.bool hasContentOwenerUserView() => $_has(12);
  @$pb.TagNumber(104)
  void clearContentOwenerUserView() => clearField(104);
  @$pb.TagNumber(104)
  UserView ensureContentOwenerUserView() => $_ensure(12);
}

class NotifyView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('NotifyView', createEmptyInstance: create)
    ..aInt64(1, 'NotifyId', protoName: 'NotifyId')
    ..a<$core.int>(2, 'ForUserId', $pb.PbFieldType.O3, protoName: 'ForUserId')
    ..a<$core.int>(3, 'ActorUserId', $pb.PbFieldType.O3, protoName: 'ActorUserId')
    ..a<$core.int>(4, 'NotiyTypeEnum', $pb.PbFieldType.O3, protoName: 'NotiyTypeEnum')
    ..aInt64(5, 'PostId', protoName: 'PostId')
    ..aInt64(6, 'CommentId', protoName: 'CommentId')
    ..a<$core.int>(7, 'PeerUserId', $pb.PbFieldType.O3, protoName: 'PeerUserId')
    ..aInt64(8, 'Murmur64Hash', protoName: 'Murmur64Hash')
    ..a<$core.int>(9, 'SeenStatus', $pb.PbFieldType.O3, protoName: 'SeenStatus')
    ..a<$core.int>(10, 'CreatedTime', $pb.PbFieldType.O3, protoName: 'CreatedTime')
    ..aOM<UserView>(100, 'ActorUserView', protoName: 'ActorUserView', subBuilder: UserView.create)
    ..aOM<PostView>(101, 'PostView', protoName: 'PostView', subBuilder: PostView.create)
    ..aOM<CommentView>(102, 'CommentView', protoName: 'CommentView', subBuilder: CommentView.create)
    ..hasRequiredFields = false
  ;

  NotifyView._() : super();
  factory NotifyView() => create();
  factory NotifyView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NotifyView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  NotifyView clone() => NotifyView()..mergeFromMessage(this);
  NotifyView copyWith(void Function(NotifyView) updates) => super.copyWith((message) => updates(message as NotifyView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NotifyView create() => NotifyView._();
  NotifyView createEmptyInstance() => create();
  static $pb.PbList<NotifyView> createRepeated() => $pb.PbList<NotifyView>();
  @$core.pragma('dart2js:noInline')
  static NotifyView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NotifyView>(create);
  static NotifyView _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get notifyId => $_getI64(0);
  @$pb.TagNumber(1)
  set notifyId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasNotifyId() => $_has(0);
  @$pb.TagNumber(1)
  void clearNotifyId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get forUserId => $_getIZ(1);
  @$pb.TagNumber(2)
  set forUserId($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasForUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearForUserId() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get actorUserId => $_getIZ(2);
  @$pb.TagNumber(3)
  set actorUserId($core.int v) { $_setSignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasActorUserId() => $_has(2);
  @$pb.TagNumber(3)
  void clearActorUserId() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get notiyTypeEnum => $_getIZ(3);
  @$pb.TagNumber(4)
  set notiyTypeEnum($core.int v) { $_setSignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasNotiyTypeEnum() => $_has(3);
  @$pb.TagNumber(4)
  void clearNotiyTypeEnum() => clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get postId => $_getI64(4);
  @$pb.TagNumber(5)
  set postId($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasPostId() => $_has(4);
  @$pb.TagNumber(5)
  void clearPostId() => clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get commentId => $_getI64(5);
  @$pb.TagNumber(6)
  set commentId($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasCommentId() => $_has(5);
  @$pb.TagNumber(6)
  void clearCommentId() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get peerUserId => $_getIZ(6);
  @$pb.TagNumber(7)
  set peerUserId($core.int v) { $_setSignedInt32(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasPeerUserId() => $_has(6);
  @$pb.TagNumber(7)
  void clearPeerUserId() => clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get murmur64Hash => $_getI64(7);
  @$pb.TagNumber(8)
  set murmur64Hash($fixnum.Int64 v) { $_setInt64(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasMurmur64Hash() => $_has(7);
  @$pb.TagNumber(8)
  void clearMurmur64Hash() => clearField(8);

  @$pb.TagNumber(9)
  $core.int get seenStatus => $_getIZ(8);
  @$pb.TagNumber(9)
  set seenStatus($core.int v) { $_setSignedInt32(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasSeenStatus() => $_has(8);
  @$pb.TagNumber(9)
  void clearSeenStatus() => clearField(9);

  @$pb.TagNumber(10)
  $core.int get createdTime => $_getIZ(9);
  @$pb.TagNumber(10)
  set createdTime($core.int v) { $_setSignedInt32(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasCreatedTime() => $_has(9);
  @$pb.TagNumber(10)
  void clearCreatedTime() => clearField(10);

  @$pb.TagNumber(100)
  UserView get actorUserView => $_getN(10);
  @$pb.TagNumber(100)
  set actorUserView(UserView v) { setField(100, v); }
  @$pb.TagNumber(100)
  $core.bool hasActorUserView() => $_has(10);
  @$pb.TagNumber(100)
  void clearActorUserView() => clearField(100);
  @$pb.TagNumber(100)
  UserView ensureActorUserView() => $_ensure(10);

  @$pb.TagNumber(101)
  PostView get postView => $_getN(11);
  @$pb.TagNumber(101)
  set postView(PostView v) { setField(101, v); }
  @$pb.TagNumber(101)
  $core.bool hasPostView() => $_has(11);
  @$pb.TagNumber(101)
  void clearPostView() => clearField(101);
  @$pb.TagNumber(101)
  PostView ensurePostView() => $_ensure(11);

  @$pb.TagNumber(102)
  CommentView get commentView => $_getN(12);
  @$pb.TagNumber(102)
  set commentView(CommentView v) { setField(102, v); }
  @$pb.TagNumber(102)
  $core.bool hasCommentView() => $_has(12);
  @$pb.TagNumber(102)
  void clearCommentView() => clearField(102);
  @$pb.TagNumber(102)
  CommentView ensureCommentView() => $_ensure(12);
}

class CommentView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('CommentView', createEmptyInstance: create)
    ..aInt64(1, 'CommentId', protoName: 'CommentId')
    ..a<$core.int>(2, 'UserId', $pb.PbFieldType.O3, protoName: 'UserId')
    ..aInt64(3, 'PostId', protoName: 'PostId')
    ..aOS(4, 'Text', protoName: 'Text')
    ..a<$core.int>(5, 'LikesCount', $pb.PbFieldType.O3, protoName: 'LikesCount')
    ..a<$core.int>(6, 'CreatedTime', $pb.PbFieldType.O3, protoName: 'CreatedTime')
    ..aOM<UserView>(15, 'SenderUserView', protoName: 'SenderUserView', subBuilder: UserView.create)
    ..hasRequiredFields = false
  ;

  CommentView._() : super();
  factory CommentView() => create();
  factory CommentView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CommentView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  CommentView clone() => CommentView()..mergeFromMessage(this);
  CommentView copyWith(void Function(CommentView) updates) => super.copyWith((message) => updates(message as CommentView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static CommentView create() => CommentView._();
  CommentView createEmptyInstance() => create();
  static $pb.PbList<CommentView> createRepeated() => $pb.PbList<CommentView>();
  @$core.pragma('dart2js:noInline')
  static CommentView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CommentView>(create);
  static CommentView _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get commentId => $_getI64(0);
  @$pb.TagNumber(1)
  set commentId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCommentId() => $_has(0);
  @$pb.TagNumber(1)
  void clearCommentId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get userId => $_getIZ(1);
  @$pb.TagNumber(2)
  set userId($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get postId => $_getI64(2);
  @$pb.TagNumber(3)
  set postId($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasPostId() => $_has(2);
  @$pb.TagNumber(3)
  void clearPostId() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get text => $_getSZ(3);
  @$pb.TagNumber(4)
  set text($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasText() => $_has(3);
  @$pb.TagNumber(4)
  void clearText() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get likesCount => $_getIZ(4);
  @$pb.TagNumber(5)
  set likesCount($core.int v) { $_setSignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasLikesCount() => $_has(4);
  @$pb.TagNumber(5)
  void clearLikesCount() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get createdTime => $_getIZ(5);
  @$pb.TagNumber(6)
  set createdTime($core.int v) { $_setSignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasCreatedTime() => $_has(5);
  @$pb.TagNumber(6)
  void clearCreatedTime() => clearField(6);

  @$pb.TagNumber(15)
  UserView get senderUserView => $_getN(6);
  @$pb.TagNumber(15)
  set senderUserView(UserView v) { setField(15, v); }
  @$pb.TagNumber(15)
  $core.bool hasSenderUserView() => $_has(6);
  @$pb.TagNumber(15)
  void clearSenderUserView() => clearField(15);
  @$pb.TagNumber(15)
  UserView ensureSenderUserView() => $_ensure(6);
}

class PostView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('PostView', createEmptyInstance: create)
    ..aInt64(1, 'PostId', protoName: 'PostId')
    ..a<$core.int>(2, 'UserId', $pb.PbFieldType.O3, protoName: 'UserId')
    ..aOS(4, 'Text', protoName: 'Text')
    ..aOS(5, 'RichText', protoName: 'RichText')
    ..a<$core.int>(6, 'MediaCount', $pb.PbFieldType.O3, protoName: 'MediaCount')
    ..a<$core.int>(7, 'SharedTo', $pb.PbFieldType.O3, protoName: 'SharedTo')
    ..a<$core.int>(8, 'DisableComment', $pb.PbFieldType.O3, protoName: 'DisableComment')
    ..a<$core.int>(9, 'HasTag', $pb.PbFieldType.O3, protoName: 'HasTag')
    ..a<$core.int>(10, 'CommentsCount', $pb.PbFieldType.O3, protoName: 'CommentsCount')
    ..a<$core.int>(11, 'LikesCount', $pb.PbFieldType.O3, protoName: 'LikesCount')
    ..a<$core.int>(12, 'ViewsCount', $pb.PbFieldType.O3, protoName: 'ViewsCount')
    ..a<$core.int>(13, 'EditedTime', $pb.PbFieldType.O3, protoName: 'EditedTime')
    ..a<$core.int>(14, 'CreatedTime', $pb.PbFieldType.O3, protoName: 'CreatedTime')
    ..aInt64(15, 'ReSharedPostId', protoName: 'ReSharedPostId')
    ..aOB(50, 'DidILiked', protoName: 'DidILiked')
    ..aOB(51, 'DidIReShared', protoName: 'DidIReShared')
    ..aOM<UserView>(100, 'SenderUserView', protoName: 'SenderUserView', subBuilder: UserView.create)
    ..aOM<UserView>(101, 'ReSharedUserView', protoName: 'ReSharedUserView', subBuilder: UserView.create)
    ..aOM<MediaView>(102, 'MediaView', protoName: 'MediaView', subBuilder: MediaView.create)
    ..pc<MediaView>(103, 'MediaViewList', $pb.PbFieldType.PM, protoName: 'MediaViewList', subBuilder: MediaView.create)
    ..hasRequiredFields = false
  ;

  PostView._() : super();
  factory PostView() => create();
  factory PostView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PostView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  PostView clone() => PostView()..mergeFromMessage(this);
  PostView copyWith(void Function(PostView) updates) => super.copyWith((message) => updates(message as PostView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static PostView create() => PostView._();
  PostView createEmptyInstance() => create();
  static $pb.PbList<PostView> createRepeated() => $pb.PbList<PostView>();
  @$core.pragma('dart2js:noInline')
  static PostView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PostView>(create);
  static PostView _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get postId => $_getI64(0);
  @$pb.TagNumber(1)
  set postId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasPostId() => $_has(0);
  @$pb.TagNumber(1)
  void clearPostId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get userId => $_getIZ(1);
  @$pb.TagNumber(2)
  set userId($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => clearField(2);

  @$pb.TagNumber(4)
  $core.String get text => $_getSZ(2);
  @$pb.TagNumber(4)
  set text($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(4)
  $core.bool hasText() => $_has(2);
  @$pb.TagNumber(4)
  void clearText() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get richText => $_getSZ(3);
  @$pb.TagNumber(5)
  set richText($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasRichText() => $_has(3);
  @$pb.TagNumber(5)
  void clearRichText() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get mediaCount => $_getIZ(4);
  @$pb.TagNumber(6)
  set mediaCount($core.int v) { $_setSignedInt32(4, v); }
  @$pb.TagNumber(6)
  $core.bool hasMediaCount() => $_has(4);
  @$pb.TagNumber(6)
  void clearMediaCount() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get sharedTo => $_getIZ(5);
  @$pb.TagNumber(7)
  set sharedTo($core.int v) { $_setSignedInt32(5, v); }
  @$pb.TagNumber(7)
  $core.bool hasSharedTo() => $_has(5);
  @$pb.TagNumber(7)
  void clearSharedTo() => clearField(7);

  @$pb.TagNumber(8)
  $core.int get disableComment => $_getIZ(6);
  @$pb.TagNumber(8)
  set disableComment($core.int v) { $_setSignedInt32(6, v); }
  @$pb.TagNumber(8)
  $core.bool hasDisableComment() => $_has(6);
  @$pb.TagNumber(8)
  void clearDisableComment() => clearField(8);

  @$pb.TagNumber(9)
  $core.int get hasTag => $_getIZ(7);
  @$pb.TagNumber(9)
  set hasTag($core.int v) { $_setSignedInt32(7, v); }
  @$pb.TagNumber(9)
  $core.bool hasHasTag() => $_has(7);
  @$pb.TagNumber(9)
  void clearHasTag() => clearField(9);

  @$pb.TagNumber(10)
  $core.int get commentsCount => $_getIZ(8);
  @$pb.TagNumber(10)
  set commentsCount($core.int v) { $_setSignedInt32(8, v); }
  @$pb.TagNumber(10)
  $core.bool hasCommentsCount() => $_has(8);
  @$pb.TagNumber(10)
  void clearCommentsCount() => clearField(10);

  @$pb.TagNumber(11)
  $core.int get likesCount => $_getIZ(9);
  @$pb.TagNumber(11)
  set likesCount($core.int v) { $_setSignedInt32(9, v); }
  @$pb.TagNumber(11)
  $core.bool hasLikesCount() => $_has(9);
  @$pb.TagNumber(11)
  void clearLikesCount() => clearField(11);

  @$pb.TagNumber(12)
  $core.int get viewsCount => $_getIZ(10);
  @$pb.TagNumber(12)
  set viewsCount($core.int v) { $_setSignedInt32(10, v); }
  @$pb.TagNumber(12)
  $core.bool hasViewsCount() => $_has(10);
  @$pb.TagNumber(12)
  void clearViewsCount() => clearField(12);

  @$pb.TagNumber(13)
  $core.int get editedTime => $_getIZ(11);
  @$pb.TagNumber(13)
  set editedTime($core.int v) { $_setSignedInt32(11, v); }
  @$pb.TagNumber(13)
  $core.bool hasEditedTime() => $_has(11);
  @$pb.TagNumber(13)
  void clearEditedTime() => clearField(13);

  @$pb.TagNumber(14)
  $core.int get createdTime => $_getIZ(12);
  @$pb.TagNumber(14)
  set createdTime($core.int v) { $_setSignedInt32(12, v); }
  @$pb.TagNumber(14)
  $core.bool hasCreatedTime() => $_has(12);
  @$pb.TagNumber(14)
  void clearCreatedTime() => clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get reSharedPostId => $_getI64(13);
  @$pb.TagNumber(15)
  set reSharedPostId($fixnum.Int64 v) { $_setInt64(13, v); }
  @$pb.TagNumber(15)
  $core.bool hasReSharedPostId() => $_has(13);
  @$pb.TagNumber(15)
  void clearReSharedPostId() => clearField(15);

  @$pb.TagNumber(50)
  $core.bool get didILiked => $_getBF(14);
  @$pb.TagNumber(50)
  set didILiked($core.bool v) { $_setBool(14, v); }
  @$pb.TagNumber(50)
  $core.bool hasDidILiked() => $_has(14);
  @$pb.TagNumber(50)
  void clearDidILiked() => clearField(50);

  @$pb.TagNumber(51)
  $core.bool get didIReShared => $_getBF(15);
  @$pb.TagNumber(51)
  set didIReShared($core.bool v) { $_setBool(15, v); }
  @$pb.TagNumber(51)
  $core.bool hasDidIReShared() => $_has(15);
  @$pb.TagNumber(51)
  void clearDidIReShared() => clearField(51);

  @$pb.TagNumber(100)
  UserView get senderUserView => $_getN(16);
  @$pb.TagNumber(100)
  set senderUserView(UserView v) { setField(100, v); }
  @$pb.TagNumber(100)
  $core.bool hasSenderUserView() => $_has(16);
  @$pb.TagNumber(100)
  void clearSenderUserView() => clearField(100);
  @$pb.TagNumber(100)
  UserView ensureSenderUserView() => $_ensure(16);

  @$pb.TagNumber(101)
  UserView get reSharedUserView => $_getN(17);
  @$pb.TagNumber(101)
  set reSharedUserView(UserView v) { setField(101, v); }
  @$pb.TagNumber(101)
  $core.bool hasReSharedUserView() => $_has(17);
  @$pb.TagNumber(101)
  void clearReSharedUserView() => clearField(101);
  @$pb.TagNumber(101)
  UserView ensureReSharedUserView() => $_ensure(17);

  @$pb.TagNumber(102)
  MediaView get mediaView => $_getN(18);
  @$pb.TagNumber(102)
  set mediaView(MediaView v) { setField(102, v); }
  @$pb.TagNumber(102)
  $core.bool hasMediaView() => $_has(18);
  @$pb.TagNumber(102)
  void clearMediaView() => clearField(102);
  @$pb.TagNumber(102)
  MediaView ensureMediaView() => $_ensure(18);

  @$pb.TagNumber(103)
  $core.List<MediaView> get mediaViewList => $_getList(19);
}

class ChatView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatView', createEmptyInstance: create)
    ..aInt64(1, 'ChatId', protoName: 'ChatId')
    ..aOS(2, 'ChatKey', protoName: 'ChatKey')
    ..aOS(3, 'RoomKey', protoName: 'RoomKey')
    ..a<$core.int>(4, 'RoomType', $pb.PbFieldType.O3, protoName: 'RoomType')
    ..a<$core.int>(5, 'UserId', $pb.PbFieldType.O3, protoName: 'UserId')
    ..a<$core.int>(6, 'PeerUserId', $pb.PbFieldType.O3, protoName: 'PeerUserId')
    ..aInt64(7, 'GroupId', protoName: 'GroupId')
    ..aInt64(8, 'HashTagId', protoName: 'HashTagId')
    ..a<$core.int>(9, 'StartedByMe', $pb.PbFieldType.O3, protoName: 'StartedByMe')
    ..aOS(10, 'Title', protoName: 'Title')
    ..aInt64(11, 'PinTime', protoName: 'PinTime')
    ..aInt64(12, 'FromMsgId', protoName: 'FromMsgId')
    ..a<$core.int>(13, 'Seq', $pb.PbFieldType.O3, protoName: 'Seq')
    ..aInt64(14, 'LastMsgId', protoName: 'LastMsgId')
    ..a<$core.int>(15, 'LastMsgStatus', $pb.PbFieldType.O3, protoName: 'LastMsgStatus')
    ..a<$core.int>(16, 'SeenSeq', $pb.PbFieldType.O3, protoName: 'SeenSeq')
    ..aInt64(17, 'SeenMsgId', protoName: 'SeenMsgId')
    ..a<$core.int>(18, 'Left', $pb.PbFieldType.O3, protoName: 'Left')
    ..a<$core.int>(19, 'Creator', $pb.PbFieldType.O3, protoName: 'Creator')
    ..a<$core.int>(20, 'Kicked', $pb.PbFieldType.O3, protoName: 'Kicked')
    ..a<$core.int>(21, 'Admin', $pb.PbFieldType.O3, protoName: 'Admin')
    ..a<$core.int>(22, 'Deactivated', $pb.PbFieldType.O3, protoName: 'Deactivated')
    ..a<$core.int>(23, 'VersionTime', $pb.PbFieldType.O3, protoName: 'VersionTime')
    ..a<$core.int>(24, 'SortTime', $pb.PbFieldType.O3, protoName: 'SortTime')
    ..a<$core.int>(25, 'CreatedTime', $pb.PbFieldType.O3, protoName: 'CreatedTime')
    ..aOS(26, 'DraftText', protoName: 'DraftText')
    ..aInt64(27, 'DratReplyToMsgId', protoName: 'DratReplyToMsgId')
    ..a<$core.int>(28, 'IsMute', $pb.PbFieldType.O3, protoName: 'IsMute')
    ..aOM<UserView>(100, 'UserView', protoName: 'UserView', subBuilder: UserView.create)
    ..aOM<GroupView>(101, 'GroupView', protoName: 'GroupView', subBuilder: GroupView.create)
    ..aOM<MessageView>(200, 'FirstUnreadMessage', protoName: 'FirstUnreadMessage', subBuilder: MessageView.create)
    ..aOM<MessageView>(201, 'LastMessage', protoName: 'LastMessage', subBuilder: MessageView.create)
    ..hasRequiredFields = false
  ;

  ChatView._() : super();
  factory ChatView() => create();
  factory ChatView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatView clone() => ChatView()..mergeFromMessage(this);
  ChatView copyWith(void Function(ChatView) updates) => super.copyWith((message) => updates(message as ChatView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatView create() => ChatView._();
  ChatView createEmptyInstance() => create();
  static $pb.PbList<ChatView> createRepeated() => $pb.PbList<ChatView>();
  @$core.pragma('dart2js:noInline')
  static ChatView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatView>(create);
  static ChatView _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get chatId => $_getI64(0);
  @$pb.TagNumber(1)
  set chatId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasChatId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChatId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get chatKey => $_getSZ(1);
  @$pb.TagNumber(2)
  set chatKey($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasChatKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearChatKey() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get roomKey => $_getSZ(2);
  @$pb.TagNumber(3)
  set roomKey($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasRoomKey() => $_has(2);
  @$pb.TagNumber(3)
  void clearRoomKey() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get roomType => $_getIZ(3);
  @$pb.TagNumber(4)
  set roomType($core.int v) { $_setSignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasRoomType() => $_has(3);
  @$pb.TagNumber(4)
  void clearRoomType() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get userId => $_getIZ(4);
  @$pb.TagNumber(5)
  set userId($core.int v) { $_setSignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasUserId() => $_has(4);
  @$pb.TagNumber(5)
  void clearUserId() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get peerUserId => $_getIZ(5);
  @$pb.TagNumber(6)
  set peerUserId($core.int v) { $_setSignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasPeerUserId() => $_has(5);
  @$pb.TagNumber(6)
  void clearPeerUserId() => clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get groupId => $_getI64(6);
  @$pb.TagNumber(7)
  set groupId($fixnum.Int64 v) { $_setInt64(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasGroupId() => $_has(6);
  @$pb.TagNumber(7)
  void clearGroupId() => clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get hashTagId => $_getI64(7);
  @$pb.TagNumber(8)
  set hashTagId($fixnum.Int64 v) { $_setInt64(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasHashTagId() => $_has(7);
  @$pb.TagNumber(8)
  void clearHashTagId() => clearField(8);

  @$pb.TagNumber(9)
  $core.int get startedByMe => $_getIZ(8);
  @$pb.TagNumber(9)
  set startedByMe($core.int v) { $_setSignedInt32(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasStartedByMe() => $_has(8);
  @$pb.TagNumber(9)
  void clearStartedByMe() => clearField(9);

  @$pb.TagNumber(10)
  $core.String get title => $_getSZ(9);
  @$pb.TagNumber(10)
  set title($core.String v) { $_setString(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasTitle() => $_has(9);
  @$pb.TagNumber(10)
  void clearTitle() => clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get pinTime => $_getI64(10);
  @$pb.TagNumber(11)
  set pinTime($fixnum.Int64 v) { $_setInt64(10, v); }
  @$pb.TagNumber(11)
  $core.bool hasPinTime() => $_has(10);
  @$pb.TagNumber(11)
  void clearPinTime() => clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get fromMsgId => $_getI64(11);
  @$pb.TagNumber(12)
  set fromMsgId($fixnum.Int64 v) { $_setInt64(11, v); }
  @$pb.TagNumber(12)
  $core.bool hasFromMsgId() => $_has(11);
  @$pb.TagNumber(12)
  void clearFromMsgId() => clearField(12);

  @$pb.TagNumber(13)
  $core.int get seq => $_getIZ(12);
  @$pb.TagNumber(13)
  set seq($core.int v) { $_setSignedInt32(12, v); }
  @$pb.TagNumber(13)
  $core.bool hasSeq() => $_has(12);
  @$pb.TagNumber(13)
  void clearSeq() => clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get lastMsgId => $_getI64(13);
  @$pb.TagNumber(14)
  set lastMsgId($fixnum.Int64 v) { $_setInt64(13, v); }
  @$pb.TagNumber(14)
  $core.bool hasLastMsgId() => $_has(13);
  @$pb.TagNumber(14)
  void clearLastMsgId() => clearField(14);

  @$pb.TagNumber(15)
  $core.int get lastMsgStatus => $_getIZ(14);
  @$pb.TagNumber(15)
  set lastMsgStatus($core.int v) { $_setSignedInt32(14, v); }
  @$pb.TagNumber(15)
  $core.bool hasLastMsgStatus() => $_has(14);
  @$pb.TagNumber(15)
  void clearLastMsgStatus() => clearField(15);

  @$pb.TagNumber(16)
  $core.int get seenSeq => $_getIZ(15);
  @$pb.TagNumber(16)
  set seenSeq($core.int v) { $_setSignedInt32(15, v); }
  @$pb.TagNumber(16)
  $core.bool hasSeenSeq() => $_has(15);
  @$pb.TagNumber(16)
  void clearSeenSeq() => clearField(16);

  @$pb.TagNumber(17)
  $fixnum.Int64 get seenMsgId => $_getI64(16);
  @$pb.TagNumber(17)
  set seenMsgId($fixnum.Int64 v) { $_setInt64(16, v); }
  @$pb.TagNumber(17)
  $core.bool hasSeenMsgId() => $_has(16);
  @$pb.TagNumber(17)
  void clearSeenMsgId() => clearField(17);

  @$pb.TagNumber(18)
  $core.int get left => $_getIZ(17);
  @$pb.TagNumber(18)
  set left($core.int v) { $_setSignedInt32(17, v); }
  @$pb.TagNumber(18)
  $core.bool hasLeft() => $_has(17);
  @$pb.TagNumber(18)
  void clearLeft() => clearField(18);

  @$pb.TagNumber(19)
  $core.int get creator => $_getIZ(18);
  @$pb.TagNumber(19)
  set creator($core.int v) { $_setSignedInt32(18, v); }
  @$pb.TagNumber(19)
  $core.bool hasCreator() => $_has(18);
  @$pb.TagNumber(19)
  void clearCreator() => clearField(19);

  @$pb.TagNumber(20)
  $core.int get kicked => $_getIZ(19);
  @$pb.TagNumber(20)
  set kicked($core.int v) { $_setSignedInt32(19, v); }
  @$pb.TagNumber(20)
  $core.bool hasKicked() => $_has(19);
  @$pb.TagNumber(20)
  void clearKicked() => clearField(20);

  @$pb.TagNumber(21)
  $core.int get admin => $_getIZ(20);
  @$pb.TagNumber(21)
  set admin($core.int v) { $_setSignedInt32(20, v); }
  @$pb.TagNumber(21)
  $core.bool hasAdmin() => $_has(20);
  @$pb.TagNumber(21)
  void clearAdmin() => clearField(21);

  @$pb.TagNumber(22)
  $core.int get deactivated => $_getIZ(21);
  @$pb.TagNumber(22)
  set deactivated($core.int v) { $_setSignedInt32(21, v); }
  @$pb.TagNumber(22)
  $core.bool hasDeactivated() => $_has(21);
  @$pb.TagNumber(22)
  void clearDeactivated() => clearField(22);

  @$pb.TagNumber(23)
  $core.int get versionTime => $_getIZ(22);
  @$pb.TagNumber(23)
  set versionTime($core.int v) { $_setSignedInt32(22, v); }
  @$pb.TagNumber(23)
  $core.bool hasVersionTime() => $_has(22);
  @$pb.TagNumber(23)
  void clearVersionTime() => clearField(23);

  @$pb.TagNumber(24)
  $core.int get sortTime => $_getIZ(23);
  @$pb.TagNumber(24)
  set sortTime($core.int v) { $_setSignedInt32(23, v); }
  @$pb.TagNumber(24)
  $core.bool hasSortTime() => $_has(23);
  @$pb.TagNumber(24)
  void clearSortTime() => clearField(24);

  @$pb.TagNumber(25)
  $core.int get createdTime => $_getIZ(24);
  @$pb.TagNumber(25)
  set createdTime($core.int v) { $_setSignedInt32(24, v); }
  @$pb.TagNumber(25)
  $core.bool hasCreatedTime() => $_has(24);
  @$pb.TagNumber(25)
  void clearCreatedTime() => clearField(25);

  @$pb.TagNumber(26)
  $core.String get draftText => $_getSZ(25);
  @$pb.TagNumber(26)
  set draftText($core.String v) { $_setString(25, v); }
  @$pb.TagNumber(26)
  $core.bool hasDraftText() => $_has(25);
  @$pb.TagNumber(26)
  void clearDraftText() => clearField(26);

  @$pb.TagNumber(27)
  $fixnum.Int64 get dratReplyToMsgId => $_getI64(26);
  @$pb.TagNumber(27)
  set dratReplyToMsgId($fixnum.Int64 v) { $_setInt64(26, v); }
  @$pb.TagNumber(27)
  $core.bool hasDratReplyToMsgId() => $_has(26);
  @$pb.TagNumber(27)
  void clearDratReplyToMsgId() => clearField(27);

  @$pb.TagNumber(28)
  $core.int get isMute => $_getIZ(27);
  @$pb.TagNumber(28)
  set isMute($core.int v) { $_setSignedInt32(27, v); }
  @$pb.TagNumber(28)
  $core.bool hasIsMute() => $_has(27);
  @$pb.TagNumber(28)
  void clearIsMute() => clearField(28);

  @$pb.TagNumber(100)
  UserView get userView => $_getN(28);
  @$pb.TagNumber(100)
  set userView(UserView v) { setField(100, v); }
  @$pb.TagNumber(100)
  $core.bool hasUserView() => $_has(28);
  @$pb.TagNumber(100)
  void clearUserView() => clearField(100);
  @$pb.TagNumber(100)
  UserView ensureUserView() => $_ensure(28);

  @$pb.TagNumber(101)
  GroupView get groupView => $_getN(29);
  @$pb.TagNumber(101)
  set groupView(GroupView v) { setField(101, v); }
  @$pb.TagNumber(101)
  $core.bool hasGroupView() => $_has(29);
  @$pb.TagNumber(101)
  void clearGroupView() => clearField(101);
  @$pb.TagNumber(101)
  GroupView ensureGroupView() => $_ensure(29);

  @$pb.TagNumber(200)
  MessageView get firstUnreadMessage => $_getN(30);
  @$pb.TagNumber(200)
  set firstUnreadMessage(MessageView v) { setField(200, v); }
  @$pb.TagNumber(200)
  $core.bool hasFirstUnreadMessage() => $_has(30);
  @$pb.TagNumber(200)
  void clearFirstUnreadMessage() => clearField(200);
  @$pb.TagNumber(200)
  MessageView ensureFirstUnreadMessage() => $_ensure(30);

  @$pb.TagNumber(201)
  MessageView get lastMessage => $_getN(31);
  @$pb.TagNumber(201)
  set lastMessage(MessageView v) { setField(201, v); }
  @$pb.TagNumber(201)
  $core.bool hasLastMessage() => $_has(31);
  @$pb.TagNumber(201)
  void clearLastMessage() => clearField(201);
  @$pb.TagNumber(201)
  MessageView ensureLastMessage() => $_ensure(31);
}

class GroupView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupView', createEmptyInstance: create)
    ..aInt64(1, 'GroupId', protoName: 'GroupId')
    ..aOS(2, 'GroupKey', protoName: 'GroupKey')
    ..aOS(3, 'GroupName', protoName: 'GroupName')
    ..aOS(4, 'UserName', protoName: 'UserName')
    ..a<$core.int>(5, 'IsSuperGroup', $pb.PbFieldType.O3, protoName: 'IsSuperGroup')
    ..aInt64(6, 'HashTagId', protoName: 'HashTagId')
    ..a<$core.int>(7, 'CreatorUserId', $pb.PbFieldType.O3, protoName: 'CreatorUserId')
    ..a<$core.int>(8, 'GroupPrivacy', $pb.PbFieldType.O3, protoName: 'GroupPrivacy')
    ..a<$core.int>(9, 'HistoryViewAble', $pb.PbFieldType.O3, protoName: 'HistoryViewAble')
    ..aInt64(10, 'Seq', protoName: 'Seq')
    ..aInt64(11, 'LastMsgId', protoName: 'LastMsgId')
    ..aInt64(12, 'PinedMsgId', protoName: 'PinedMsgId')
    ..aInt64(13, 'AvatarRefId', protoName: 'AvatarRefId')
    ..a<$core.int>(14, 'AvatarCount', $pb.PbFieldType.O3, protoName: 'AvatarCount')
    ..aOS(15, 'About', protoName: 'About')
    ..aOS(16, 'InviteLink', protoName: 'InviteLink')
    ..a<$core.int>(17, 'MembersCount', $pb.PbFieldType.O3, protoName: 'MembersCount')
    ..a<$core.int>(18, 'SortTime', $pb.PbFieldType.O3, protoName: 'SortTime')
    ..a<$core.int>(19, 'CreatedTime', $pb.PbFieldType.O3, protoName: 'CreatedTime')
    ..hasRequiredFields = false
  ;

  GroupView._() : super();
  factory GroupView() => create();
  factory GroupView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupView clone() => GroupView()..mergeFromMessage(this);
  GroupView copyWith(void Function(GroupView) updates) => super.copyWith((message) => updates(message as GroupView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupView create() => GroupView._();
  GroupView createEmptyInstance() => create();
  static $pb.PbList<GroupView> createRepeated() => $pb.PbList<GroupView>();
  @$core.pragma('dart2js:noInline')
  static GroupView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupView>(create);
  static GroupView _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get groupId => $_getI64(0);
  @$pb.TagNumber(1)
  set groupId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGroupId() => $_has(0);
  @$pb.TagNumber(1)
  void clearGroupId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get groupKey => $_getSZ(1);
  @$pb.TagNumber(2)
  set groupKey($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupKey() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get groupName => $_getSZ(2);
  @$pb.TagNumber(3)
  set groupName($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasGroupName() => $_has(2);
  @$pb.TagNumber(3)
  void clearGroupName() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get userName => $_getSZ(3);
  @$pb.TagNumber(4)
  set userName($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasUserName() => $_has(3);
  @$pb.TagNumber(4)
  void clearUserName() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get isSuperGroup => $_getIZ(4);
  @$pb.TagNumber(5)
  set isSuperGroup($core.int v) { $_setSignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasIsSuperGroup() => $_has(4);
  @$pb.TagNumber(5)
  void clearIsSuperGroup() => clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get hashTagId => $_getI64(5);
  @$pb.TagNumber(6)
  set hashTagId($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasHashTagId() => $_has(5);
  @$pb.TagNumber(6)
  void clearHashTagId() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get creatorUserId => $_getIZ(6);
  @$pb.TagNumber(7)
  set creatorUserId($core.int v) { $_setSignedInt32(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasCreatorUserId() => $_has(6);
  @$pb.TagNumber(7)
  void clearCreatorUserId() => clearField(7);

  @$pb.TagNumber(8)
  $core.int get groupPrivacy => $_getIZ(7);
  @$pb.TagNumber(8)
  set groupPrivacy($core.int v) { $_setSignedInt32(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasGroupPrivacy() => $_has(7);
  @$pb.TagNumber(8)
  void clearGroupPrivacy() => clearField(8);

  @$pb.TagNumber(9)
  $core.int get historyViewAble => $_getIZ(8);
  @$pb.TagNumber(9)
  set historyViewAble($core.int v) { $_setSignedInt32(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasHistoryViewAble() => $_has(8);
  @$pb.TagNumber(9)
  void clearHistoryViewAble() => clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get seq => $_getI64(9);
  @$pb.TagNumber(10)
  set seq($fixnum.Int64 v) { $_setInt64(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasSeq() => $_has(9);
  @$pb.TagNumber(10)
  void clearSeq() => clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get lastMsgId => $_getI64(10);
  @$pb.TagNumber(11)
  set lastMsgId($fixnum.Int64 v) { $_setInt64(10, v); }
  @$pb.TagNumber(11)
  $core.bool hasLastMsgId() => $_has(10);
  @$pb.TagNumber(11)
  void clearLastMsgId() => clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get pinedMsgId => $_getI64(11);
  @$pb.TagNumber(12)
  set pinedMsgId($fixnum.Int64 v) { $_setInt64(11, v); }
  @$pb.TagNumber(12)
  $core.bool hasPinedMsgId() => $_has(11);
  @$pb.TagNumber(12)
  void clearPinedMsgId() => clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get avatarRefId => $_getI64(12);
  @$pb.TagNumber(13)
  set avatarRefId($fixnum.Int64 v) { $_setInt64(12, v); }
  @$pb.TagNumber(13)
  $core.bool hasAvatarRefId() => $_has(12);
  @$pb.TagNumber(13)
  void clearAvatarRefId() => clearField(13);

  @$pb.TagNumber(14)
  $core.int get avatarCount => $_getIZ(13);
  @$pb.TagNumber(14)
  set avatarCount($core.int v) { $_setSignedInt32(13, v); }
  @$pb.TagNumber(14)
  $core.bool hasAvatarCount() => $_has(13);
  @$pb.TagNumber(14)
  void clearAvatarCount() => clearField(14);

  @$pb.TagNumber(15)
  $core.String get about => $_getSZ(14);
  @$pb.TagNumber(15)
  set about($core.String v) { $_setString(14, v); }
  @$pb.TagNumber(15)
  $core.bool hasAbout() => $_has(14);
  @$pb.TagNumber(15)
  void clearAbout() => clearField(15);

  @$pb.TagNumber(16)
  $core.String get inviteLink => $_getSZ(15);
  @$pb.TagNumber(16)
  set inviteLink($core.String v) { $_setString(15, v); }
  @$pb.TagNumber(16)
  $core.bool hasInviteLink() => $_has(15);
  @$pb.TagNumber(16)
  void clearInviteLink() => clearField(16);

  @$pb.TagNumber(17)
  $core.int get membersCount => $_getIZ(16);
  @$pb.TagNumber(17)
  set membersCount($core.int v) { $_setSignedInt32(16, v); }
  @$pb.TagNumber(17)
  $core.bool hasMembersCount() => $_has(16);
  @$pb.TagNumber(17)
  void clearMembersCount() => clearField(17);

  @$pb.TagNumber(18)
  $core.int get sortTime => $_getIZ(17);
  @$pb.TagNumber(18)
  set sortTime($core.int v) { $_setSignedInt32(17, v); }
  @$pb.TagNumber(18)
  $core.bool hasSortTime() => $_has(17);
  @$pb.TagNumber(18)
  void clearSortTime() => clearField(18);

  @$pb.TagNumber(19)
  $core.int get createdTime => $_getIZ(18);
  @$pb.TagNumber(19)
  set createdTime($core.int v) { $_setSignedInt32(18, v); }
  @$pb.TagNumber(19)
  $core.bool hasCreatedTime() => $_has(18);
  @$pb.TagNumber(19)
  void clearCreatedTime() => clearField(19);
}

class GroupMemberView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupMemberView', createEmptyInstance: create)
    ..aInt64(1, 'OrderId', protoName: 'OrderId')
    ..aInt64(2, 'GroupId', protoName: 'GroupId')
    ..a<$core.int>(3, 'UserId', $pb.PbFieldType.O3, protoName: 'UserId')
    ..a<$core.int>(4, 'ByUserId', $pb.PbFieldType.O3, protoName: 'ByUserId')
    ..a<$core.int>(5, 'GroupRole', $pb.PbFieldType.O3, protoName: 'GroupRole')
    ..a<$core.int>(6, 'CreatedTime', $pb.PbFieldType.O3, protoName: 'CreatedTime')
    ..hasRequiredFields = false
  ;

  GroupMemberView._() : super();
  factory GroupMemberView() => create();
  factory GroupMemberView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupMemberView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupMemberView clone() => GroupMemberView()..mergeFromMessage(this);
  GroupMemberView copyWith(void Function(GroupMemberView) updates) => super.copyWith((message) => updates(message as GroupMemberView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupMemberView create() => GroupMemberView._();
  GroupMemberView createEmptyInstance() => create();
  static $pb.PbList<GroupMemberView> createRepeated() => $pb.PbList<GroupMemberView>();
  @$core.pragma('dart2js:noInline')
  static GroupMemberView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupMemberView>(create);
  static GroupMemberView _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get orderId => $_getI64(0);
  @$pb.TagNumber(1)
  set orderId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOrderId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOrderId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get groupId => $_getI64(1);
  @$pb.TagNumber(2)
  set groupId($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupId() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupId() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get userId => $_getIZ(2);
  @$pb.TagNumber(3)
  set userId($core.int v) { $_setSignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasUserId() => $_has(2);
  @$pb.TagNumber(3)
  void clearUserId() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get byUserId => $_getIZ(3);
  @$pb.TagNumber(4)
  set byUserId($core.int v) { $_setSignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasByUserId() => $_has(3);
  @$pb.TagNumber(4)
  void clearByUserId() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get groupRole => $_getIZ(4);
  @$pb.TagNumber(5)
  set groupRole($core.int v) { $_setSignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasGroupRole() => $_has(4);
  @$pb.TagNumber(5)
  void clearGroupRole() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get createdTime => $_getIZ(5);
  @$pb.TagNumber(6)
  set createdTime($core.int v) { $_setSignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasCreatedTime() => $_has(5);
  @$pb.TagNumber(6)
  void clearCreatedTime() => clearField(6);
}

class MessageView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('MessageView', createEmptyInstance: create)
    ..aOS(1, 'RoomKey', protoName: 'RoomKey')
    ..aInt64(2, 'MessageId', protoName: 'MessageId')
    ..a<$core.int>(3, 'UserId', $pb.PbFieldType.O3, protoName: 'UserId')
    ..aInt64(4, 'FileRefId', protoName: 'FileRefId')
    ..a<$core.int>(5, 'MessageType', $pb.PbFieldType.O3, protoName: 'MessageType')
    ..aOS(6, 'Text', protoName: 'Text')
    ..a<$core.int>(7, 'Hiden', $pb.PbFieldType.O3, protoName: 'Hiden')
    ..a<$core.int>(8, 'Seq', $pb.PbFieldType.O3, protoName: 'Seq')
    ..aInt64(9, 'ForwardedMsgId', protoName: 'ForwardedMsgId')
    ..aInt64(10, 'PostId', protoName: 'PostId')
    ..aInt64(11, 'StickerId', protoName: 'StickerId')
    ..a<$core.int>(12, 'CreatedTime', $pb.PbFieldType.O3, protoName: 'CreatedTime')
    ..a<$core.int>(13, 'DeliveredTime', $pb.PbFieldType.O3, protoName: 'DeliveredTime')
    ..a<$core.int>(14, 'SeenTime', $pb.PbFieldType.O3, protoName: 'SeenTime')
    ..a<$core.int>(15, 'DeliviryStatus', $pb.PbFieldType.O3, protoName: 'DeliviryStatus')
    ..aInt64(16, 'ReplyToMessageId', protoName: 'ReplyToMessageId')
    ..aInt64(17, 'ViewsCount', protoName: 'ViewsCount')
    ..a<$core.int>(18, 'EditTime', $pb.PbFieldType.O3, protoName: 'EditTime')
    ..a<$core.int>(19, 'Ttl', $pb.PbFieldType.O3, protoName: 'Ttl')
    ..aOM<FileRedView>(50, 'FileRedView', protoName: 'FileRedView', subBuilder: FileRedView.create)
    ..hasRequiredFields = false
  ;

  MessageView._() : super();
  factory MessageView() => create();
  factory MessageView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MessageView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  MessageView clone() => MessageView()..mergeFromMessage(this);
  MessageView copyWith(void Function(MessageView) updates) => super.copyWith((message) => updates(message as MessageView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MessageView create() => MessageView._();
  MessageView createEmptyInstance() => create();
  static $pb.PbList<MessageView> createRepeated() => $pb.PbList<MessageView>();
  @$core.pragma('dart2js:noInline')
  static MessageView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MessageView>(create);
  static MessageView _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get roomKey => $_getSZ(0);
  @$pb.TagNumber(1)
  set roomKey($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasRoomKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearRoomKey() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get messageId => $_getI64(1);
  @$pb.TagNumber(2)
  set messageId($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMessageId() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessageId() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get userId => $_getIZ(2);
  @$pb.TagNumber(3)
  set userId($core.int v) { $_setSignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasUserId() => $_has(2);
  @$pb.TagNumber(3)
  void clearUserId() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get fileRefId => $_getI64(3);
  @$pb.TagNumber(4)
  set fileRefId($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasFileRefId() => $_has(3);
  @$pb.TagNumber(4)
  void clearFileRefId() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get messageType => $_getIZ(4);
  @$pb.TagNumber(5)
  set messageType($core.int v) { $_setSignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasMessageType() => $_has(4);
  @$pb.TagNumber(5)
  void clearMessageType() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get text => $_getSZ(5);
  @$pb.TagNumber(6)
  set text($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasText() => $_has(5);
  @$pb.TagNumber(6)
  void clearText() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get hiden => $_getIZ(6);
  @$pb.TagNumber(7)
  set hiden($core.int v) { $_setSignedInt32(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasHiden() => $_has(6);
  @$pb.TagNumber(7)
  void clearHiden() => clearField(7);

  @$pb.TagNumber(8)
  $core.int get seq => $_getIZ(7);
  @$pb.TagNumber(8)
  set seq($core.int v) { $_setSignedInt32(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasSeq() => $_has(7);
  @$pb.TagNumber(8)
  void clearSeq() => clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get forwardedMsgId => $_getI64(8);
  @$pb.TagNumber(9)
  set forwardedMsgId($fixnum.Int64 v) { $_setInt64(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasForwardedMsgId() => $_has(8);
  @$pb.TagNumber(9)
  void clearForwardedMsgId() => clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get postId => $_getI64(9);
  @$pb.TagNumber(10)
  set postId($fixnum.Int64 v) { $_setInt64(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasPostId() => $_has(9);
  @$pb.TagNumber(10)
  void clearPostId() => clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get stickerId => $_getI64(10);
  @$pb.TagNumber(11)
  set stickerId($fixnum.Int64 v) { $_setInt64(10, v); }
  @$pb.TagNumber(11)
  $core.bool hasStickerId() => $_has(10);
  @$pb.TagNumber(11)
  void clearStickerId() => clearField(11);

  @$pb.TagNumber(12)
  $core.int get createdTime => $_getIZ(11);
  @$pb.TagNumber(12)
  set createdTime($core.int v) { $_setSignedInt32(11, v); }
  @$pb.TagNumber(12)
  $core.bool hasCreatedTime() => $_has(11);
  @$pb.TagNumber(12)
  void clearCreatedTime() => clearField(12);

  @$pb.TagNumber(13)
  $core.int get deliveredTime => $_getIZ(12);
  @$pb.TagNumber(13)
  set deliveredTime($core.int v) { $_setSignedInt32(12, v); }
  @$pb.TagNumber(13)
  $core.bool hasDeliveredTime() => $_has(12);
  @$pb.TagNumber(13)
  void clearDeliveredTime() => clearField(13);

  @$pb.TagNumber(14)
  $core.int get seenTime => $_getIZ(13);
  @$pb.TagNumber(14)
  set seenTime($core.int v) { $_setSignedInt32(13, v); }
  @$pb.TagNumber(14)
  $core.bool hasSeenTime() => $_has(13);
  @$pb.TagNumber(14)
  void clearSeenTime() => clearField(14);

  @$pb.TagNumber(15)
  $core.int get deliviryStatus => $_getIZ(14);
  @$pb.TagNumber(15)
  set deliviryStatus($core.int v) { $_setSignedInt32(14, v); }
  @$pb.TagNumber(15)
  $core.bool hasDeliviryStatus() => $_has(14);
  @$pb.TagNumber(15)
  void clearDeliviryStatus() => clearField(15);

  @$pb.TagNumber(16)
  $fixnum.Int64 get replyToMessageId => $_getI64(15);
  @$pb.TagNumber(16)
  set replyToMessageId($fixnum.Int64 v) { $_setInt64(15, v); }
  @$pb.TagNumber(16)
  $core.bool hasReplyToMessageId() => $_has(15);
  @$pb.TagNumber(16)
  void clearReplyToMessageId() => clearField(16);

  @$pb.TagNumber(17)
  $fixnum.Int64 get viewsCount => $_getI64(16);
  @$pb.TagNumber(17)
  set viewsCount($fixnum.Int64 v) { $_setInt64(16, v); }
  @$pb.TagNumber(17)
  $core.bool hasViewsCount() => $_has(16);
  @$pb.TagNumber(17)
  void clearViewsCount() => clearField(17);

  @$pb.TagNumber(18)
  $core.int get editTime => $_getIZ(17);
  @$pb.TagNumber(18)
  set editTime($core.int v) { $_setSignedInt32(17, v); }
  @$pb.TagNumber(18)
  $core.bool hasEditTime() => $_has(17);
  @$pb.TagNumber(18)
  void clearEditTime() => clearField(18);

  @$pb.TagNumber(19)
  $core.int get ttl => $_getIZ(18);
  @$pb.TagNumber(19)
  set ttl($core.int v) { $_setSignedInt32(18, v); }
  @$pb.TagNumber(19)
  $core.bool hasTtl() => $_has(18);
  @$pb.TagNumber(19)
  void clearTtl() => clearField(19);

  @$pb.TagNumber(50)
  FileRedView get fileRedView => $_getN(19);
  @$pb.TagNumber(50)
  set fileRedView(FileRedView v) { setField(50, v); }
  @$pb.TagNumber(50)
  $core.bool hasFileRedView() => $_has(19);
  @$pb.TagNumber(50)
  void clearFileRedView() => clearField(50);
  @$pb.TagNumber(50)
  FileRedView ensureFileRedView() => $_ensure(19);
}

class FileRedView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('FileRedView', createEmptyInstance: create)
    ..aInt64(1, 'FileRefId', protoName: 'FileRefId')
    ..aInt64(2, 'UserId', protoName: 'UserId')
    ..aOS(3, 'Name', protoName: 'Name')
    ..a<$core.int>(4, 'Width', $pb.PbFieldType.O3, protoName: 'Width')
    ..a<$core.int>(5, 'Height', $pb.PbFieldType.O3, protoName: 'Height')
    ..a<$core.int>(6, 'Duration', $pb.PbFieldType.O3, protoName: 'Duration')
    ..aOS(7, 'Extension', protoName: 'Extension')
    ..aOS(8, 'UrlSource', protoName: 'UrlSource')
    ..hasRequiredFields = false
  ;

  FileRedView._() : super();
  factory FileRedView() => create();
  factory FileRedView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileRedView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  FileRedView clone() => FileRedView()..mergeFromMessage(this);
  FileRedView copyWith(void Function(FileRedView) updates) => super.copyWith((message) => updates(message as FileRedView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileRedView create() => FileRedView._();
  FileRedView createEmptyInstance() => create();
  static $pb.PbList<FileRedView> createRepeated() => $pb.PbList<FileRedView>();
  @$core.pragma('dart2js:noInline')
  static FileRedView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileRedView>(create);
  static FileRedView _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get fileRefId => $_getI64(0);
  @$pb.TagNumber(1)
  set fileRefId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFileRefId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileRefId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get userId => $_getI64(1);
  @$pb.TagNumber(2)
  set userId($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get width => $_getIZ(3);
  @$pb.TagNumber(4)
  set width($core.int v) { $_setSignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasWidth() => $_has(3);
  @$pb.TagNumber(4)
  void clearWidth() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get height => $_getIZ(4);
  @$pb.TagNumber(5)
  set height($core.int v) { $_setSignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasHeight() => $_has(4);
  @$pb.TagNumber(5)
  void clearHeight() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get duration => $_getIZ(5);
  @$pb.TagNumber(6)
  set duration($core.int v) { $_setSignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasDuration() => $_has(5);
  @$pb.TagNumber(6)
  void clearDuration() => clearField(6);

  @$pb.TagNumber(7)
  $core.String get extension_7 => $_getSZ(6);
  @$pb.TagNumber(7)
  set extension_7($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasExtension_7() => $_has(6);
  @$pb.TagNumber(7)
  void clearExtension_7() => clearField(7);

  @$pb.TagNumber(8)
  $core.String get urlSource => $_getSZ(7);
  @$pb.TagNumber(8)
  set urlSource($core.String v) { $_setString(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasUrlSource() => $_has(7);
  @$pb.TagNumber(8)
  void clearUrlSource() => clearField(8);
}

class UserView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UserView', createEmptyInstance: create)
    ..a<$core.int>(1, 'UserId', $pb.PbFieldType.O3, protoName: 'UserId')
    ..aOS(2, 'UserName', protoName: 'UserName')
    ..aOS(4, 'FirstName', protoName: 'FirstName')
    ..aOS(5, 'LastName', protoName: 'LastName')
    ..aInt64(8, 'AvatarRefId', protoName: 'AvatarRefId')
    ..a<$core.int>(9, 'ProfilePrivacy', $pb.PbFieldType.O3, protoName: 'ProfilePrivacy')
    ..aInt64(10, 'Phone', protoName: 'Phone')
    ..aOS(11, 'About', protoName: 'About')
    ..a<$core.int>(100, 'FollowersCount', $pb.PbFieldType.O3, protoName: 'FollowersCount')
    ..a<$core.int>(101, 'FollowingCount', $pb.PbFieldType.O3, protoName: 'FollowingCount')
    ..a<$core.int>(102, 'PostsCount', $pb.PbFieldType.O3, protoName: 'PostsCount')
    ..a<$core.int>(103, 'MediaCount', $pb.PbFieldType.O3, protoName: 'MediaCount')
    ..e<$1.UserOnlineStatusEnum>(200, 'UserOnlineStatusEnum', $pb.PbFieldType.OE, protoName: 'UserOnlineStatusEnum', defaultOrMaker: $1.UserOnlineStatusEnum.EXACTLY, valueOf: $1.UserOnlineStatusEnum.valueOf, enumValues: $1.UserOnlineStatusEnum.values)
    ..a<$core.int>(201, 'LastActiveTime', $pb.PbFieldType.O3, protoName: 'LastActiveTime')
    ..aOS(202, 'LastActiveTimeShow', protoName: 'LastActiveTimeShow')
    ..e<$1.FollowingEnum>(300, 'MyFollwing', $pb.PbFieldType.OE, protoName: 'MyFollwing', defaultOrMaker: $1.FollowingEnum.FOLLOWING_NONE, valueOf: $1.FollowingEnum.valueOf, enumValues: $1.FollowingEnum.values)
    ..hasRequiredFields = false
  ;

  UserView._() : super();
  factory UserView() => create();
  factory UserView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UserView clone() => UserView()..mergeFromMessage(this);
  UserView copyWith(void Function(UserView) updates) => super.copyWith((message) => updates(message as UserView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UserView create() => UserView._();
  UserView createEmptyInstance() => create();
  static $pb.PbList<UserView> createRepeated() => $pb.PbList<UserView>();
  @$core.pragma('dart2js:noInline')
  static UserView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserView>(create);
  static UserView _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get userId => $_getIZ(0);
  @$pb.TagNumber(1)
  set userId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get userName => $_getSZ(1);
  @$pb.TagNumber(2)
  set userName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserName() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserName() => clearField(2);

  @$pb.TagNumber(4)
  $core.String get firstName => $_getSZ(2);
  @$pb.TagNumber(4)
  set firstName($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(4)
  $core.bool hasFirstName() => $_has(2);
  @$pb.TagNumber(4)
  void clearFirstName() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get lastName => $_getSZ(3);
  @$pb.TagNumber(5)
  set lastName($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasLastName() => $_has(3);
  @$pb.TagNumber(5)
  void clearLastName() => clearField(5);

  @$pb.TagNumber(8)
  $fixnum.Int64 get avatarRefId => $_getI64(4);
  @$pb.TagNumber(8)
  set avatarRefId($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(8)
  $core.bool hasAvatarRefId() => $_has(4);
  @$pb.TagNumber(8)
  void clearAvatarRefId() => clearField(8);

  @$pb.TagNumber(9)
  $core.int get profilePrivacy => $_getIZ(5);
  @$pb.TagNumber(9)
  set profilePrivacy($core.int v) { $_setSignedInt32(5, v); }
  @$pb.TagNumber(9)
  $core.bool hasProfilePrivacy() => $_has(5);
  @$pb.TagNumber(9)
  void clearProfilePrivacy() => clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get phone => $_getI64(6);
  @$pb.TagNumber(10)
  set phone($fixnum.Int64 v) { $_setInt64(6, v); }
  @$pb.TagNumber(10)
  $core.bool hasPhone() => $_has(6);
  @$pb.TagNumber(10)
  void clearPhone() => clearField(10);

  @$pb.TagNumber(11)
  $core.String get about => $_getSZ(7);
  @$pb.TagNumber(11)
  set about($core.String v) { $_setString(7, v); }
  @$pb.TagNumber(11)
  $core.bool hasAbout() => $_has(7);
  @$pb.TagNumber(11)
  void clearAbout() => clearField(11);

  @$pb.TagNumber(100)
  $core.int get followersCount => $_getIZ(8);
  @$pb.TagNumber(100)
  set followersCount($core.int v) { $_setSignedInt32(8, v); }
  @$pb.TagNumber(100)
  $core.bool hasFollowersCount() => $_has(8);
  @$pb.TagNumber(100)
  void clearFollowersCount() => clearField(100);

  @$pb.TagNumber(101)
  $core.int get followingCount => $_getIZ(9);
  @$pb.TagNumber(101)
  set followingCount($core.int v) { $_setSignedInt32(9, v); }
  @$pb.TagNumber(101)
  $core.bool hasFollowingCount() => $_has(9);
  @$pb.TagNumber(101)
  void clearFollowingCount() => clearField(101);

  @$pb.TagNumber(102)
  $core.int get postsCount => $_getIZ(10);
  @$pb.TagNumber(102)
  set postsCount($core.int v) { $_setSignedInt32(10, v); }
  @$pb.TagNumber(102)
  $core.bool hasPostsCount() => $_has(10);
  @$pb.TagNumber(102)
  void clearPostsCount() => clearField(102);

  @$pb.TagNumber(103)
  $core.int get mediaCount => $_getIZ(11);
  @$pb.TagNumber(103)
  set mediaCount($core.int v) { $_setSignedInt32(11, v); }
  @$pb.TagNumber(103)
  $core.bool hasMediaCount() => $_has(11);
  @$pb.TagNumber(103)
  void clearMediaCount() => clearField(103);

  @$pb.TagNumber(200)
  $1.UserOnlineStatusEnum get userOnlineStatusEnum => $_getN(12);
  @$pb.TagNumber(200)
  set userOnlineStatusEnum($1.UserOnlineStatusEnum v) { setField(200, v); }
  @$pb.TagNumber(200)
  $core.bool hasUserOnlineStatusEnum() => $_has(12);
  @$pb.TagNumber(200)
  void clearUserOnlineStatusEnum() => clearField(200);

  @$pb.TagNumber(201)
  $core.int get lastActiveTime => $_getIZ(13);
  @$pb.TagNumber(201)
  set lastActiveTime($core.int v) { $_setSignedInt32(13, v); }
  @$pb.TagNumber(201)
  $core.bool hasLastActiveTime() => $_has(13);
  @$pb.TagNumber(201)
  void clearLastActiveTime() => clearField(201);

  @$pb.TagNumber(202)
  $core.String get lastActiveTimeShow => $_getSZ(14);
  @$pb.TagNumber(202)
  set lastActiveTimeShow($core.String v) { $_setString(14, v); }
  @$pb.TagNumber(202)
  $core.bool hasLastActiveTimeShow() => $_has(14);
  @$pb.TagNumber(202)
  void clearLastActiveTimeShow() => clearField(202);

  @$pb.TagNumber(300)
  $1.FollowingEnum get myFollwing => $_getN(15);
  @$pb.TagNumber(300)
  set myFollwing($1.FollowingEnum v) { setField(300, v); }
  @$pb.TagNumber(300)
  $core.bool hasMyFollwing() => $_has(15);
  @$pb.TagNumber(300)
  void clearMyFollwing() => clearField(300);
}

class SettingNotificationView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('SettingNotificationView', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  SettingNotificationView._() : super();
  factory SettingNotificationView() => create();
  factory SettingNotificationView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SettingNotificationView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  SettingNotificationView clone() => SettingNotificationView()..mergeFromMessage(this);
  SettingNotificationView copyWith(void Function(SettingNotificationView) updates) => super.copyWith((message) => updates(message as SettingNotificationView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SettingNotificationView create() => SettingNotificationView._();
  SettingNotificationView createEmptyInstance() => create();
  static $pb.PbList<SettingNotificationView> createRepeated() => $pb.PbList<SettingNotificationView>();
  @$core.pragma('dart2js:noInline')
  static SettingNotificationView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SettingNotificationView>(create);
  static SettingNotificationView _defaultInstance;
}

class AppConfig extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('AppConfig', createEmptyInstance: create)
    ..aOB(1, 'DeprecatedClient', protoName: 'DeprecatedClient')
    ..aOB(2, 'HasNewUpdate', protoName: 'HasNewUpdate')
    ..hasRequiredFields = false
  ;

  AppConfig._() : super();
  factory AppConfig() => create();
  factory AppConfig.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AppConfig.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  AppConfig clone() => AppConfig()..mergeFromMessage(this);
  AppConfig copyWith(void Function(AppConfig) updates) => super.copyWith((message) => updates(message as AppConfig));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AppConfig create() => AppConfig._();
  AppConfig createEmptyInstance() => create();
  static $pb.PbList<AppConfig> createRepeated() => $pb.PbList<AppConfig>();
  @$core.pragma('dart2js:noInline')
  static AppConfig getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AppConfig>(create);
  static AppConfig _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get deprecatedClient => $_getBF(0);
  @$pb.TagNumber(1)
  set deprecatedClient($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDeprecatedClient() => $_has(0);
  @$pb.TagNumber(1)
  void clearDeprecatedClient() => clearField(1);

  @$pb.TagNumber(2)
  $core.bool get hasNewUpdate => $_getBF(1);
  @$pb.TagNumber(2)
  set hasNewUpdate($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasHasNewUpdate() => $_has(1);
  @$pb.TagNumber(2)
  void clearHasNewUpdate() => clearField(2);
}

class UserProfileView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UserProfileView', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  UserProfileView._() : super();
  factory UserProfileView() => create();
  factory UserProfileView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserProfileView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UserProfileView clone() => UserProfileView()..mergeFromMessage(this);
  UserProfileView copyWith(void Function(UserProfileView) updates) => super.copyWith((message) => updates(message as UserProfileView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UserProfileView create() => UserProfileView._();
  UserProfileView createEmptyInstance() => create();
  static $pb.PbList<UserProfileView> createRepeated() => $pb.PbList<UserProfileView>();
  @$core.pragma('dart2js:noInline')
  static UserProfileView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserProfileView>(create);
  static UserProfileView _defaultInstance;
}

class UserViewRowify extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UserViewRowify', createEmptyInstance: create)
    ..aInt64(1, 'Id', protoName: 'Id')
    ..a<$core.int>(2, 'CreatedTime', $pb.PbFieldType.O3, protoName: 'CreatedTime')
    ..aOM<UserView>(10, 'UserView', protoName: 'UserView', subBuilder: UserView.create)
    ..hasRequiredFields = false
  ;

  UserViewRowify._() : super();
  factory UserViewRowify() => create();
  factory UserViewRowify.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserViewRowify.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UserViewRowify clone() => UserViewRowify()..mergeFromMessage(this);
  UserViewRowify copyWith(void Function(UserViewRowify) updates) => super.copyWith((message) => updates(message as UserViewRowify));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UserViewRowify create() => UserViewRowify._();
  UserViewRowify createEmptyInstance() => create();
  static $pb.PbList<UserViewRowify> createRepeated() => $pb.PbList<UserViewRowify>();
  @$core.pragma('dart2js:noInline')
  static UserViewRowify getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserViewRowify>(create);
  static UserViewRowify _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get createdTime => $_getIZ(1);
  @$pb.TagNumber(2)
  set createdTime($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasCreatedTime() => $_has(1);
  @$pb.TagNumber(2)
  void clearCreatedTime() => clearField(2);

  @$pb.TagNumber(10)
  UserView get userView => $_getN(2);
  @$pb.TagNumber(10)
  set userView(UserView v) { setField(10, v); }
  @$pb.TagNumber(10)
  $core.bool hasUserView() => $_has(2);
  @$pb.TagNumber(10)
  void clearUserView() => clearField(10);
  @$pb.TagNumber(10)
  UserView ensureUserView() => $_ensure(2);
}

class PostViewRowify extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('PostViewRowify', createEmptyInstance: create)
    ..aInt64(1, 'Id', protoName: 'Id')
    ..aOM<PostView>(10, 'PostView', protoName: 'PostView', subBuilder: PostView.create)
    ..hasRequiredFields = false
  ;

  PostViewRowify._() : super();
  factory PostViewRowify() => create();
  factory PostViewRowify.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PostViewRowify.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  PostViewRowify clone() => PostViewRowify()..mergeFromMessage(this);
  PostViewRowify copyWith(void Function(PostViewRowify) updates) => super.copyWith((message) => updates(message as PostViewRowify));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static PostViewRowify create() => PostViewRowify._();
  PostViewRowify createEmptyInstance() => create();
  static $pb.PbList<PostViewRowify> createRepeated() => $pb.PbList<PostViewRowify>();
  @$core.pragma('dart2js:noInline')
  static PostViewRowify getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PostViewRowify>(create);
  static PostViewRowify _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(10)
  PostView get postView => $_getN(1);
  @$pb.TagNumber(10)
  set postView(PostView v) { setField(10, v); }
  @$pb.TagNumber(10)
  $core.bool hasPostView() => $_has(1);
  @$pb.TagNumber(10)
  void clearPostView() => clearField(10);
  @$pb.TagNumber(10)
  PostView ensurePostView() => $_ensure(1);
}

class TagView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('TagView', createEmptyInstance: create)
    ..aInt64(1, 'TagId', protoName: 'TagId')
    ..aOS(2, 'Name', protoName: 'Name')
    ..a<$core.int>(3, 'Count', $pb.PbFieldType.O3, protoName: 'Count')
    ..a<$core.int>(4, 'TagStatusEnum', $pb.PbFieldType.O3, protoName: 'TagStatusEnum')
    ..a<$core.int>(5, 'CreatedTime', $pb.PbFieldType.O3, protoName: 'CreatedTime')
    ..hasRequiredFields = false
  ;

  TagView._() : super();
  factory TagView() => create();
  factory TagView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TagView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  TagView clone() => TagView()..mergeFromMessage(this);
  TagView copyWith(void Function(TagView) updates) => super.copyWith((message) => updates(message as TagView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static TagView create() => TagView._();
  TagView createEmptyInstance() => create();
  static $pb.PbList<TagView> createRepeated() => $pb.PbList<TagView>();
  @$core.pragma('dart2js:noInline')
  static TagView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TagView>(create);
  static TagView _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get tagId => $_getI64(0);
  @$pb.TagNumber(1)
  set tagId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTagId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTagId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get count => $_getIZ(2);
  @$pb.TagNumber(3)
  set count($core.int v) { $_setSignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCount() => $_has(2);
  @$pb.TagNumber(3)
  void clearCount() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get tagStatusEnum => $_getIZ(3);
  @$pb.TagNumber(4)
  set tagStatusEnum($core.int v) { $_setSignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasTagStatusEnum() => $_has(3);
  @$pb.TagNumber(4)
  void clearTagStatusEnum() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get createdTime => $_getIZ(4);
  @$pb.TagNumber(5)
  set createdTime($core.int v) { $_setSignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasCreatedTime() => $_has(4);
  @$pb.TagNumber(5)
  void clearCreatedTime() => clearField(5);
}

class TopTagWithSamplePosts extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('TopTagWithSamplePosts', createEmptyInstance: create)
    ..aOM<TagView>(1, 'TagView', protoName: 'TagView', subBuilder: TagView.create)
    ..pc<PostView>(2, 'PostViewList', $pb.PbFieldType.PM, protoName: 'PostViewList', subBuilder: PostView.create)
    ..hasRequiredFields = false
  ;

  TopTagWithSamplePosts._() : super();
  factory TopTagWithSamplePosts() => create();
  factory TopTagWithSamplePosts.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TopTagWithSamplePosts.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  TopTagWithSamplePosts clone() => TopTagWithSamplePosts()..mergeFromMessage(this);
  TopTagWithSamplePosts copyWith(void Function(TopTagWithSamplePosts) updates) => super.copyWith((message) => updates(message as TopTagWithSamplePosts));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static TopTagWithSamplePosts create() => TopTagWithSamplePosts._();
  TopTagWithSamplePosts createEmptyInstance() => create();
  static $pb.PbList<TopTagWithSamplePosts> createRepeated() => $pb.PbList<TopTagWithSamplePosts>();
  @$core.pragma('dart2js:noInline')
  static TopTagWithSamplePosts getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TopTagWithSamplePosts>(create);
  static TopTagWithSamplePosts _defaultInstance;

  @$pb.TagNumber(1)
  TagView get tagView => $_getN(0);
  @$pb.TagNumber(1)
  set tagView(TagView v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasTagView() => $_has(0);
  @$pb.TagNumber(1)
  void clearTagView() => clearField(1);
  @$pb.TagNumber(1)
  TagView ensureTagView() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.List<PostView> get postViewList => $_getList(1);
}

class SelfUserView extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('SelfUserView', createEmptyInstance: create)
    ..aOM<UserView>(1, 'UserView', protoName: 'UserView', subBuilder: UserView.create)
    ..a<$core.int>(30, 'ProfilePrivacy', $pb.PbFieldType.O3, protoName: 'ProfilePrivacy')
    ..a<$core.int>(32, 'OnlinePrivacy', $pb.PbFieldType.O3, protoName: 'OnlinePrivacy')
    ..a<$core.int>(33, 'CallPrivacy', $pb.PbFieldType.O3, protoName: 'CallPrivacy')
    ..a<$core.int>(34, 'AddToGroupPrivacy', $pb.PbFieldType.O3, protoName: 'AddToGroupPrivacy')
    ..a<$core.int>(35, 'SeenMessagePrivacy', $pb.PbFieldType.O3, protoName: 'SeenMessagePrivacy')
    ..aOM<SettingNotificationView>(100, 'SettingNotification', protoName: 'SettingNotification', subBuilder: SettingNotificationView.create)
    ..hasRequiredFields = false
  ;

  SelfUserView._() : super();
  factory SelfUserView() => create();
  factory SelfUserView.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SelfUserView.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  SelfUserView clone() => SelfUserView()..mergeFromMessage(this);
  SelfUserView copyWith(void Function(SelfUserView) updates) => super.copyWith((message) => updates(message as SelfUserView));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SelfUserView create() => SelfUserView._();
  SelfUserView createEmptyInstance() => create();
  static $pb.PbList<SelfUserView> createRepeated() => $pb.PbList<SelfUserView>();
  @$core.pragma('dart2js:noInline')
  static SelfUserView getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SelfUserView>(create);
  static SelfUserView _defaultInstance;

  @$pb.TagNumber(1)
  UserView get userView => $_getN(0);
  @$pb.TagNumber(1)
  set userView(UserView v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserView() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserView() => clearField(1);
  @$pb.TagNumber(1)
  UserView ensureUserView() => $_ensure(0);

  @$pb.TagNumber(30)
  $core.int get profilePrivacy => $_getIZ(1);
  @$pb.TagNumber(30)
  set profilePrivacy($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(30)
  $core.bool hasProfilePrivacy() => $_has(1);
  @$pb.TagNumber(30)
  void clearProfilePrivacy() => clearField(30);

  @$pb.TagNumber(32)
  $core.int get onlinePrivacy => $_getIZ(2);
  @$pb.TagNumber(32)
  set onlinePrivacy($core.int v) { $_setSignedInt32(2, v); }
  @$pb.TagNumber(32)
  $core.bool hasOnlinePrivacy() => $_has(2);
  @$pb.TagNumber(32)
  void clearOnlinePrivacy() => clearField(32);

  @$pb.TagNumber(33)
  $core.int get callPrivacy => $_getIZ(3);
  @$pb.TagNumber(33)
  set callPrivacy($core.int v) { $_setSignedInt32(3, v); }
  @$pb.TagNumber(33)
  $core.bool hasCallPrivacy() => $_has(3);
  @$pb.TagNumber(33)
  void clearCallPrivacy() => clearField(33);

  @$pb.TagNumber(34)
  $core.int get addToGroupPrivacy => $_getIZ(4);
  @$pb.TagNumber(34)
  set addToGroupPrivacy($core.int v) { $_setSignedInt32(4, v); }
  @$pb.TagNumber(34)
  $core.bool hasAddToGroupPrivacy() => $_has(4);
  @$pb.TagNumber(34)
  void clearAddToGroupPrivacy() => clearField(34);

  @$pb.TagNumber(35)
  $core.int get seenMessagePrivacy => $_getIZ(5);
  @$pb.TagNumber(35)
  set seenMessagePrivacy($core.int v) { $_setSignedInt32(5, v); }
  @$pb.TagNumber(35)
  $core.bool hasSeenMessagePrivacy() => $_has(5);
  @$pb.TagNumber(35)
  void clearSeenMessagePrivacy() => clearField(35);

  @$pb.TagNumber(100)
  SettingNotificationView get settingNotification => $_getN(6);
  @$pb.TagNumber(100)
  set settingNotification(SettingNotificationView v) { setField(100, v); }
  @$pb.TagNumber(100)
  $core.bool hasSettingNotification() => $_has(6);
  @$pb.TagNumber(100)
  void clearSettingNotification() => clearField(100);
  @$pb.TagNumber(100)
  SettingNotificationView ensureSettingNotification() => $_ensure(6);
}

class Error extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Error', createEmptyInstance: create)
    ..e<ServerErrors>(1, 'Error', $pb.PbFieldType.OE, protoName: 'Error', defaultOrMaker: ServerErrors.UNKNOWN_ERR, valueOf: ServerErrors.valueOf, enumValues: ServerErrors.values)
    ..aOB(2, 'ShowError', protoName: 'ShowError')
    ..aOS(3, 'ErrorMessage', protoName: 'ErrorMessage')
    ..hasRequiredFields = false
  ;

  Error._() : super();
  factory Error() => create();
  factory Error.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Error.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Error clone() => Error()..mergeFromMessage(this);
  Error copyWith(void Function(Error) updates) => super.copyWith((message) => updates(message as Error));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Error create() => Error._();
  Error createEmptyInstance() => create();
  static $pb.PbList<Error> createRepeated() => $pb.PbList<Error>();
  @$core.pragma('dart2js:noInline')
  static Error getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Error>(create);
  static Error _defaultInstance;

  @$pb.TagNumber(1)
  ServerErrors get error => $_getN(0);
  @$pb.TagNumber(1)
  set error(ServerErrors v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasError() => $_has(0);
  @$pb.TagNumber(1)
  void clearError() => clearField(1);

  @$pb.TagNumber(2)
  $core.bool get showError => $_getBF(1);
  @$pb.TagNumber(2)
  set showError($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasShowError() => $_has(1);
  @$pb.TagNumber(2)
  void clearShowError() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get errorMessage => $_getSZ(2);
  @$pb.TagNumber(3)
  set errorMessage($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasErrorMessage() => $_has(2);
  @$pb.TagNumber(3)
  void clearErrorMessage() => clearField(3);
}

