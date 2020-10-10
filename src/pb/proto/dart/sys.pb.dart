///
//  Generated code. Do not modify.
//  source: sys.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

class Event extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Event', createEmptyInstance: create)
    ..aInt64(1, 'eventId')
    ..a<$core.int>(2, 'eventType', $pb.PbFieldType.O3)
    ..aInt64(3, 'byUserId')
    ..aInt64(4, 'peerUserId')
    ..aInt64(5, 'postId')
    ..aInt64(6, 'commentId')
    ..aInt64(7, 'hashTagId')
    ..aInt64(8, 'groupId')
    ..aInt64(9, 'actionId')
    ..aInt64(10, 'chatId')
    ..aOS(11, 'chatKey')
    ..aInt64(12, 'messageId')
    ..aInt64(13, 'reSharedId')
    ..aInt64(14, 'murmur64Hash')
    ..hasRequiredFields = false
  ;

  Event._() : super();
  factory Event() => create();
  factory Event.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Event.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Event clone() => Event()..mergeFromMessage(this);
  Event copyWith(void Function(Event) updates) => super.copyWith((message) => updates(message as Event));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Event create() => Event._();
  Event createEmptyInstance() => create();
  static $pb.PbList<Event> createRepeated() => $pb.PbList<Event>();
  @$core.pragma('dart2js:noInline')
  static Event getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Event>(create);
  static Event _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get eventId => $_getI64(0);
  @$pb.TagNumber(1)
  set eventId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasEventId() => $_has(0);
  @$pb.TagNumber(1)
  void clearEventId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get eventType => $_getIZ(1);
  @$pb.TagNumber(2)
  set eventType($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasEventType() => $_has(1);
  @$pb.TagNumber(2)
  void clearEventType() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get byUserId => $_getI64(2);
  @$pb.TagNumber(3)
  set byUserId($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasByUserId() => $_has(2);
  @$pb.TagNumber(3)
  void clearByUserId() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get peerUserId => $_getI64(3);
  @$pb.TagNumber(4)
  set peerUserId($fixnum.Int64 v) { $_setInt64(3, v); }
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
  $fixnum.Int64 get hashTagId => $_getI64(6);
  @$pb.TagNumber(7)
  set hashTagId($fixnum.Int64 v) { $_setInt64(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasHashTagId() => $_has(6);
  @$pb.TagNumber(7)
  void clearHashTagId() => clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get groupId => $_getI64(7);
  @$pb.TagNumber(8)
  set groupId($fixnum.Int64 v) { $_setInt64(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasGroupId() => $_has(7);
  @$pb.TagNumber(8)
  void clearGroupId() => clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get actionId => $_getI64(8);
  @$pb.TagNumber(9)
  set actionId($fixnum.Int64 v) { $_setInt64(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasActionId() => $_has(8);
  @$pb.TagNumber(9)
  void clearActionId() => clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get chatId => $_getI64(9);
  @$pb.TagNumber(10)
  set chatId($fixnum.Int64 v) { $_setInt64(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasChatId() => $_has(9);
  @$pb.TagNumber(10)
  void clearChatId() => clearField(10);

  @$pb.TagNumber(11)
  $core.String get chatKey => $_getSZ(10);
  @$pb.TagNumber(11)
  set chatKey($core.String v) { $_setString(10, v); }
  @$pb.TagNumber(11)
  $core.bool hasChatKey() => $_has(10);
  @$pb.TagNumber(11)
  void clearChatKey() => clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get messageId => $_getI64(11);
  @$pb.TagNumber(12)
  set messageId($fixnum.Int64 v) { $_setInt64(11, v); }
  @$pb.TagNumber(12)
  $core.bool hasMessageId() => $_has(11);
  @$pb.TagNumber(12)
  void clearMessageId() => clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get reSharedId => $_getI64(12);
  @$pb.TagNumber(13)
  set reSharedId($fixnum.Int64 v) { $_setInt64(12, v); }
  @$pb.TagNumber(13)
  $core.bool hasReSharedId() => $_has(12);
  @$pb.TagNumber(13)
  void clearReSharedId() => clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get murmur64Hash => $_getI64(13);
  @$pb.TagNumber(14)
  set murmur64Hash($fixnum.Int64 v) { $_setInt64(13, v); }
  @$pb.TagNumber(14)
  $core.bool hasMurmur64Hash() => $_has(13);
  @$pb.TagNumber(14)
  void clearMurmur64Hash() => clearField(14);
}

class PB_Notify extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('PB_Notify', createEmptyInstance: create)
    ..aInt64(1, 'notifyId')
    ..a<$core.int>(2, 'forUserId', $pb.PbFieldType.O3)
    ..a<$core.int>(3, 'actorUserId', $pb.PbFieldType.O3)
    ..a<$core.int>(4, 'notifyType', $pb.PbFieldType.O3)
    ..aInt64(5, 'postId')
    ..aInt64(6, 'commentId')
    ..a<$core.int>(7, 'peerUserId', $pb.PbFieldType.O3)
    ..aInt64(8, 'murmur64Hash')
    ..a<$core.int>(9, 'seenStatus', $pb.PbFieldType.O3)
    ..a<$core.int>(10, 'createdTime', $pb.PbFieldType.O3)
    ..hasRequiredFields = false
  ;

  PB_Notify._() : super();
  factory PB_Notify() => create();
  factory PB_Notify.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PB_Notify.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  PB_Notify clone() => PB_Notify()..mergeFromMessage(this);
  PB_Notify copyWith(void Function(PB_Notify) updates) => super.copyWith((message) => updates(message as PB_Notify));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static PB_Notify create() => PB_Notify._();
  PB_Notify createEmptyInstance() => create();
  static $pb.PbList<PB_Notify> createRepeated() => $pb.PbList<PB_Notify>();
  @$core.pragma('dart2js:noInline')
  static PB_Notify getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PB_Notify>(create);
  static PB_Notify _defaultInstance;

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
  $core.int get notifyType => $_getIZ(3);
  @$pb.TagNumber(4)
  set notifyType($core.int v) { $_setSignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasNotifyType() => $_has(3);
  @$pb.TagNumber(4)
  void clearNotifyType() => clearField(4);

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
}

class Invoke extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Invoke', createEmptyInstance: create)
    ..a<$core.int>(1, 'method', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(2, 'actionId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOB(3, 'isResponse')
    ..a<$core.List<$core.int>>(4, 'rpcData', $pb.PbFieldType.OY)
    ..a<$core.int>(6, 'namespace', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  Invoke._() : super();
  factory Invoke() => create();
  factory Invoke.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Invoke.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Invoke clone() => Invoke()..mergeFromMessage(this);
  Invoke copyWith(void Function(Invoke) updates) => super.copyWith((message) => updates(message as Invoke));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Invoke create() => Invoke._();
  Invoke createEmptyInstance() => create();
  static $pb.PbList<Invoke> createRepeated() => $pb.PbList<Invoke>();
  @$core.pragma('dart2js:noInline')
  static Invoke getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Invoke>(create);
  static Invoke _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get method => $_getIZ(0);
  @$pb.TagNumber(1)
  set method($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasMethod() => $_has(0);
  @$pb.TagNumber(1)
  void clearMethod() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get actionId => $_getI64(1);
  @$pb.TagNumber(2)
  set actionId($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasActionId() => $_has(1);
  @$pb.TagNumber(2)
  void clearActionId() => clearField(2);

  @$pb.TagNumber(3)
  $core.bool get isResponse => $_getBF(2);
  @$pb.TagNumber(3)
  set isResponse($core.bool v) { $_setBool(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasIsResponse() => $_has(2);
  @$pb.TagNumber(3)
  void clearIsResponse() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get rpcData => $_getN(3);
  @$pb.TagNumber(4)
  set rpcData($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasRpcData() => $_has(3);
  @$pb.TagNumber(4)
  void clearRpcData() => clearField(4);

  @$pb.TagNumber(6)
  $core.int get namespace => $_getIZ(4);
  @$pb.TagNumber(6)
  set namespace($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(6)
  $core.bool hasNamespace() => $_has(4);
  @$pb.TagNumber(6)
  void clearNamespace() => clearField(6);
}

