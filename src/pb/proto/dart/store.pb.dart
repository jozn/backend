///
//  Generated code. Do not modify.
//  source: store.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'store.pbenum.dart';

export 'store.pbenum.dart';

class Message extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Message', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'gid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(2, 'byUserCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, 'postType', $pb.PbFieldType.OU3)
    ..aInt64(4, 'mediaId')
    ..aInt64(5, 'fileRefId')
    ..aOS(6, 'postKey')
    ..aOS(7, 'text')
    ..aOS(8, 'richText')
    ..a<$core.int>(10, 'sharedTo', $pb.PbFieldType.OU3)
    ..a<$core.int>(12, 'via', $pb.PbFieldType.OU3)
    ..a<$core.int>(13, 'seq', $pb.PbFieldType.OU3)
    ..aOB(15, 'deleted')
    ..aOM<Message>(16, 'forward', subBuilder: Message.create)
    ..a<$core.int>(17, 'editedTime', $pb.PbFieldType.OU3)
    ..a<$core.int>(18, 'createdTime', $pb.PbFieldType.OU3)
    ..aOM<Message>(50, 'replyTo', subBuilder: Message.create)
    ..a<$core.int>(100, 'byChannelCid', $pb.PbFieldType.OU3)
    ..aOM<MessageCount>(101, 'counts', subBuilder: MessageCount.create)
    ..aOM<MessageSetting>(102, 'setting', subBuilder: MessageSetting.create)
    ..pc<FileMsg>(103, 'files', $pb.PbFieldType.PM, subBuilder: FileMsg.create)
    ..e<MessageDeliveryStatues>(105, 'deliveryStatus', $pb.PbFieldType.OE, defaultOrMaker: MessageDeliveryStatues.UNKNOWN_MD, valueOf: MessageDeliveryStatues.valueOf, enumValues: MessageDeliveryStatues.values)
    ..a<$core.int>(106, 'deliveryTime', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(108, 'versionTime', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(109, 'title')
    ..aOM<Product>(110, 'Product', protoName: 'Product', subBuilder: Product.create)
    ..aOB(111, 'verified')
    ..a<$core.int>(112, 'flags', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(180, 'previousMessageId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  Message._() : super();
  factory Message() => create();
  factory Message.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Message.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Message clone() => Message()..mergeFromMessage(this);
  Message copyWith(void Function(Message) updates) => super.copyWith((message) => updates(message as Message));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Message create() => Message._();
  Message createEmptyInstance() => create();
  static $pb.PbList<Message> createRepeated() => $pb.PbList<Message>();
  @$core.pragma('dart2js:noInline')
  static Message getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Message>(create);
  static Message _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get byUserCid => $_getIZ(1);
  @$pb.TagNumber(2)
  set byUserCid($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasByUserCid() => $_has(1);
  @$pb.TagNumber(2)
  void clearByUserCid() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get postType => $_getIZ(2);
  @$pb.TagNumber(3)
  set postType($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasPostType() => $_has(2);
  @$pb.TagNumber(3)
  void clearPostType() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get mediaId => $_getI64(3);
  @$pb.TagNumber(4)
  set mediaId($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasMediaId() => $_has(3);
  @$pb.TagNumber(4)
  void clearMediaId() => clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get fileRefId => $_getI64(4);
  @$pb.TagNumber(5)
  set fileRefId($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasFileRefId() => $_has(4);
  @$pb.TagNumber(5)
  void clearFileRefId() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get postKey => $_getSZ(5);
  @$pb.TagNumber(6)
  set postKey($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasPostKey() => $_has(5);
  @$pb.TagNumber(6)
  void clearPostKey() => clearField(6);

  @$pb.TagNumber(7)
  $core.String get text => $_getSZ(6);
  @$pb.TagNumber(7)
  set text($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasText() => $_has(6);
  @$pb.TagNumber(7)
  void clearText() => clearField(7);

  @$pb.TagNumber(8)
  $core.String get richText => $_getSZ(7);
  @$pb.TagNumber(8)
  set richText($core.String v) { $_setString(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasRichText() => $_has(7);
  @$pb.TagNumber(8)
  void clearRichText() => clearField(8);

  @$pb.TagNumber(10)
  $core.int get sharedTo => $_getIZ(8);
  @$pb.TagNumber(10)
  set sharedTo($core.int v) { $_setUnsignedInt32(8, v); }
  @$pb.TagNumber(10)
  $core.bool hasSharedTo() => $_has(8);
  @$pb.TagNumber(10)
  void clearSharedTo() => clearField(10);

  @$pb.TagNumber(12)
  $core.int get via => $_getIZ(9);
  @$pb.TagNumber(12)
  set via($core.int v) { $_setUnsignedInt32(9, v); }
  @$pb.TagNumber(12)
  $core.bool hasVia() => $_has(9);
  @$pb.TagNumber(12)
  void clearVia() => clearField(12);

  @$pb.TagNumber(13)
  $core.int get seq => $_getIZ(10);
  @$pb.TagNumber(13)
  set seq($core.int v) { $_setUnsignedInt32(10, v); }
  @$pb.TagNumber(13)
  $core.bool hasSeq() => $_has(10);
  @$pb.TagNumber(13)
  void clearSeq() => clearField(13);

  @$pb.TagNumber(15)
  $core.bool get deleted => $_getBF(11);
  @$pb.TagNumber(15)
  set deleted($core.bool v) { $_setBool(11, v); }
  @$pb.TagNumber(15)
  $core.bool hasDeleted() => $_has(11);
  @$pb.TagNumber(15)
  void clearDeleted() => clearField(15);

  @$pb.TagNumber(16)
  Message get forward => $_getN(12);
  @$pb.TagNumber(16)
  set forward(Message v) { setField(16, v); }
  @$pb.TagNumber(16)
  $core.bool hasForward() => $_has(12);
  @$pb.TagNumber(16)
  void clearForward() => clearField(16);
  @$pb.TagNumber(16)
  Message ensureForward() => $_ensure(12);

  @$pb.TagNumber(17)
  $core.int get editedTime => $_getIZ(13);
  @$pb.TagNumber(17)
  set editedTime($core.int v) { $_setUnsignedInt32(13, v); }
  @$pb.TagNumber(17)
  $core.bool hasEditedTime() => $_has(13);
  @$pb.TagNumber(17)
  void clearEditedTime() => clearField(17);

  @$pb.TagNumber(18)
  $core.int get createdTime => $_getIZ(14);
  @$pb.TagNumber(18)
  set createdTime($core.int v) { $_setUnsignedInt32(14, v); }
  @$pb.TagNumber(18)
  $core.bool hasCreatedTime() => $_has(14);
  @$pb.TagNumber(18)
  void clearCreatedTime() => clearField(18);

  @$pb.TagNumber(50)
  Message get replyTo => $_getN(15);
  @$pb.TagNumber(50)
  set replyTo(Message v) { setField(50, v); }
  @$pb.TagNumber(50)
  $core.bool hasReplyTo() => $_has(15);
  @$pb.TagNumber(50)
  void clearReplyTo() => clearField(50);
  @$pb.TagNumber(50)
  Message ensureReplyTo() => $_ensure(15);

  @$pb.TagNumber(100)
  $core.int get byChannelCid => $_getIZ(16);
  @$pb.TagNumber(100)
  set byChannelCid($core.int v) { $_setUnsignedInt32(16, v); }
  @$pb.TagNumber(100)
  $core.bool hasByChannelCid() => $_has(16);
  @$pb.TagNumber(100)
  void clearByChannelCid() => clearField(100);

  @$pb.TagNumber(101)
  MessageCount get counts => $_getN(17);
  @$pb.TagNumber(101)
  set counts(MessageCount v) { setField(101, v); }
  @$pb.TagNumber(101)
  $core.bool hasCounts() => $_has(17);
  @$pb.TagNumber(101)
  void clearCounts() => clearField(101);
  @$pb.TagNumber(101)
  MessageCount ensureCounts() => $_ensure(17);

  @$pb.TagNumber(102)
  MessageSetting get setting => $_getN(18);
  @$pb.TagNumber(102)
  set setting(MessageSetting v) { setField(102, v); }
  @$pb.TagNumber(102)
  $core.bool hasSetting() => $_has(18);
  @$pb.TagNumber(102)
  void clearSetting() => clearField(102);
  @$pb.TagNumber(102)
  MessageSetting ensureSetting() => $_ensure(18);

  @$pb.TagNumber(103)
  $core.List<FileMsg> get files => $_getList(19);

  @$pb.TagNumber(105)
  MessageDeliveryStatues get deliveryStatus => $_getN(20);
  @$pb.TagNumber(105)
  set deliveryStatus(MessageDeliveryStatues v) { setField(105, v); }
  @$pb.TagNumber(105)
  $core.bool hasDeliveryStatus() => $_has(20);
  @$pb.TagNumber(105)
  void clearDeliveryStatus() => clearField(105);

  @$pb.TagNumber(106)
  $core.int get deliveryTime => $_getIZ(21);
  @$pb.TagNumber(106)
  set deliveryTime($core.int v) { $_setUnsignedInt32(21, v); }
  @$pb.TagNumber(106)
  $core.bool hasDeliveryTime() => $_has(21);
  @$pb.TagNumber(106)
  void clearDeliveryTime() => clearField(106);

  @$pb.TagNumber(108)
  $fixnum.Int64 get versionTime => $_getI64(22);
  @$pb.TagNumber(108)
  set versionTime($fixnum.Int64 v) { $_setInt64(22, v); }
  @$pb.TagNumber(108)
  $core.bool hasVersionTime() => $_has(22);
  @$pb.TagNumber(108)
  void clearVersionTime() => clearField(108);

  @$pb.TagNumber(109)
  $core.String get title => $_getSZ(23);
  @$pb.TagNumber(109)
  set title($core.String v) { $_setString(23, v); }
  @$pb.TagNumber(109)
  $core.bool hasTitle() => $_has(23);
  @$pb.TagNumber(109)
  void clearTitle() => clearField(109);

  @$pb.TagNumber(110)
  Product get product => $_getN(24);
  @$pb.TagNumber(110)
  set product(Product v) { setField(110, v); }
  @$pb.TagNumber(110)
  $core.bool hasProduct() => $_has(24);
  @$pb.TagNumber(110)
  void clearProduct() => clearField(110);
  @$pb.TagNumber(110)
  Product ensureProduct() => $_ensure(24);

  @$pb.TagNumber(111)
  $core.bool get verified => $_getBF(25);
  @$pb.TagNumber(111)
  set verified($core.bool v) { $_setBool(25, v); }
  @$pb.TagNumber(111)
  $core.bool hasVerified() => $_has(25);
  @$pb.TagNumber(111)
  void clearVerified() => clearField(111);

  @$pb.TagNumber(112)
  $core.int get flags => $_getIZ(26);
  @$pb.TagNumber(112)
  set flags($core.int v) { $_setUnsignedInt32(26, v); }
  @$pb.TagNumber(112)
  $core.bool hasFlags() => $_has(26);
  @$pb.TagNumber(112)
  void clearFlags() => clearField(112);

  @$pb.TagNumber(180)
  $fixnum.Int64 get previousMessageId => $_getI64(27);
  @$pb.TagNumber(180)
  set previousMessageId($fixnum.Int64 v) { $_setInt64(27, v); }
  @$pb.TagNumber(180)
  $core.bool hasPreviousMessageId() => $_has(27);
  @$pb.TagNumber(180)
  void clearPreviousMessageId() => clearField(180);
}

class MessageCount extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('MessageCount', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'messageGid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(2, 'commentsCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, 'likesCount', $pb.PbFieldType.OU3)
    ..aInt64(4, 'viewsCount')
    ..a<$core.int>(5, 'reSharedCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(6, 'chatSharedCount', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  MessageCount._() : super();
  factory MessageCount() => create();
  factory MessageCount.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MessageCount.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  MessageCount clone() => MessageCount()..mergeFromMessage(this);
  MessageCount copyWith(void Function(MessageCount) updates) => super.copyWith((message) => updates(message as MessageCount));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MessageCount create() => MessageCount._();
  MessageCount createEmptyInstance() => create();
  static $pb.PbList<MessageCount> createRepeated() => $pb.PbList<MessageCount>();
  @$core.pragma('dart2js:noInline')
  static MessageCount getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MessageCount>(create);
  static MessageCount _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get messageGid => $_getI64(0);
  @$pb.TagNumber(1)
  set messageGid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasMessageGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessageGid() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get commentsCount => $_getIZ(1);
  @$pb.TagNumber(2)
  set commentsCount($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasCommentsCount() => $_has(1);
  @$pb.TagNumber(2)
  void clearCommentsCount() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get likesCount => $_getIZ(2);
  @$pb.TagNumber(3)
  set likesCount($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasLikesCount() => $_has(2);
  @$pb.TagNumber(3)
  void clearLikesCount() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get viewsCount => $_getI64(3);
  @$pb.TagNumber(4)
  set viewsCount($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasViewsCount() => $_has(3);
  @$pb.TagNumber(4)
  void clearViewsCount() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get reSharedCount => $_getIZ(4);
  @$pb.TagNumber(5)
  set reSharedCount($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasReSharedCount() => $_has(4);
  @$pb.TagNumber(5)
  void clearReSharedCount() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get chatSharedCount => $_getIZ(5);
  @$pb.TagNumber(6)
  set chatSharedCount($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasChatSharedCount() => $_has(5);
  @$pb.TagNumber(6)
  void clearChatSharedCount() => clearField(6);
}

class MessageSetting extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('MessageSetting', createEmptyInstance: create)
    ..a<$core.int>(11, 'disableComment', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  MessageSetting._() : super();
  factory MessageSetting() => create();
  factory MessageSetting.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MessageSetting.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  MessageSetting clone() => MessageSetting()..mergeFromMessage(this);
  MessageSetting copyWith(void Function(MessageSetting) updates) => super.copyWith((message) => updates(message as MessageSetting));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MessageSetting create() => MessageSetting._();
  MessageSetting createEmptyInstance() => create();
  static $pb.PbList<MessageSetting> createRepeated() => $pb.PbList<MessageSetting>();
  @$core.pragma('dart2js:noInline')
  static MessageSetting getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MessageSetting>(create);
  static MessageSetting _defaultInstance;

  @$pb.TagNumber(11)
  $core.int get disableComment => $_getIZ(0);
  @$pb.TagNumber(11)
  set disableComment($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(11)
  $core.bool hasDisableComment() => $_has(0);
  @$pb.TagNumber(11)
  void clearDisableComment() => clearField(11);
}

class MessageLog extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('MessageLog', createEmptyInstance: create)
    ..a<$core.int>(2, 'byUserCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, 'targetUserCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(4, 'targetChannelCid', $pb.PbFieldType.OU3)
    ..e<MessageLogType>(10, 'logType', $pb.PbFieldType.OE, defaultOrMaker: MessageLogType.USER_JOINED, valueOf: MessageLogType.valueOf, enumValues: MessageLogType.values)
    ..aOM<Channel>(11, 'targetChannelView', subBuilder: Channel.create)
    ..a<$core.int>(50, 'byChannelCid', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  MessageLog._() : super();
  factory MessageLog() => create();
  factory MessageLog.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MessageLog.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  MessageLog clone() => MessageLog()..mergeFromMessage(this);
  MessageLog copyWith(void Function(MessageLog) updates) => super.copyWith((message) => updates(message as MessageLog));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MessageLog create() => MessageLog._();
  MessageLog createEmptyInstance() => create();
  static $pb.PbList<MessageLog> createRepeated() => $pb.PbList<MessageLog>();
  @$core.pragma('dart2js:noInline')
  static MessageLog getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MessageLog>(create);
  static MessageLog _defaultInstance;

  @$pb.TagNumber(2)
  $core.int get byUserCid => $_getIZ(0);
  @$pb.TagNumber(2)
  set byUserCid($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(2)
  $core.bool hasByUserCid() => $_has(0);
  @$pb.TagNumber(2)
  void clearByUserCid() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get targetUserCid => $_getIZ(1);
  @$pb.TagNumber(3)
  set targetUserCid($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasTargetUserCid() => $_has(1);
  @$pb.TagNumber(3)
  void clearTargetUserCid() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get targetChannelCid => $_getIZ(2);
  @$pb.TagNumber(4)
  set targetChannelCid($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(4)
  $core.bool hasTargetChannelCid() => $_has(2);
  @$pb.TagNumber(4)
  void clearTargetChannelCid() => clearField(4);

  @$pb.TagNumber(10)
  MessageLogType get logType => $_getN(3);
  @$pb.TagNumber(10)
  set logType(MessageLogType v) { setField(10, v); }
  @$pb.TagNumber(10)
  $core.bool hasLogType() => $_has(3);
  @$pb.TagNumber(10)
  void clearLogType() => clearField(10);

  @$pb.TagNumber(11)
  Channel get targetChannelView => $_getN(4);
  @$pb.TagNumber(11)
  set targetChannelView(Channel v) { setField(11, v); }
  @$pb.TagNumber(11)
  $core.bool hasTargetChannelView() => $_has(4);
  @$pb.TagNumber(11)
  void clearTargetChannelView() => clearField(11);
  @$pb.TagNumber(11)
  Channel ensureTargetChannelView() => $_ensure(4);

  @$pb.TagNumber(50)
  $core.int get byChannelCid => $_getIZ(5);
  @$pb.TagNumber(50)
  set byChannelCid($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(50)
  $core.bool hasByChannelCid() => $_has(5);
  @$pb.TagNumber(50)
  void clearByChannelCid() => clearField(50);
}

class Channel extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Channel', createEmptyInstance: create)
    ..a<$core.int>(1, 'cid', $pb.PbFieldType.OU3)
    ..aOS(2, 'userName')
    ..aOS(4, 'firstName')
    ..aOS(5, 'lastName')
    ..a<$core.int>(6, 'isVerified', $pb.PbFieldType.OU3)
    ..a<$core.int>(7, 'creatorUserCid', $pb.PbFieldType.OU3)
    ..e<ChannelPrivacy>(9, 'privacy', $pb.PbFieldType.OE, defaultOrMaker: ChannelPrivacy.CHANNEL_UNKNOWN_AB, valueOf: ChannelPrivacy.valueOf, enumValues: ChannelPrivacy.values)
    ..aOS(16, 'about')
    ..aOS(17, 'inviteLinkHash')
    ..a<$core.int>(19, 'postSeq', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(20, 'sortTime', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(21, 'syncTime', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<Message>(25, 'lastMessage', subBuilder: Message.create)
    ..aOM<Message>(26, 'pinnedMessage', subBuilder: Message.create)
    ..a<$core.int>(36, 'createdTime', $pb.PbFieldType.OU3)
    ..a<$core.int>(38, 'isDeleted', $pb.PbFieldType.OU3)
    ..a<$core.int>(39, 'isBanned', $pb.PbFieldType.OU3)
    ..aInt64(40, 'avatarCount')
    ..aOM<ChannelCounts>(41, 'counts', subBuilder: ChannelCounts.create)
    ..e<ChannelType>(42, 'channelType', $pb.PbFieldType.OE, defaultOrMaker: ChannelType.CHANNEL_UNKNOWN, valueOf: ChannelType.valueOf, enumValues: ChannelType.values)
    ..aOM<ChannelNotificationSetting>(90, 'notificationSetting', subBuilder: ChannelNotificationSetting.create)
    ..aOM<Message>(100, 'avatar', subBuilder: Message.create)
    ..hasRequiredFields = false
  ;

  Channel._() : super();
  factory Channel() => create();
  factory Channel.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Channel.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Channel clone() => Channel()..mergeFromMessage(this);
  Channel copyWith(void Function(Channel) updates) => super.copyWith((message) => updates(message as Channel));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Channel create() => Channel._();
  Channel createEmptyInstance() => create();
  static $pb.PbList<Channel> createRepeated() => $pb.PbList<Channel>();
  @$core.pragma('dart2js:noInline')
  static Channel getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Channel>(create);
  static Channel _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get cid => $_getIZ(0);
  @$pb.TagNumber(1)
  set cid($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCid() => $_has(0);
  @$pb.TagNumber(1)
  void clearCid() => clearField(1);

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

  @$pb.TagNumber(6)
  $core.int get isVerified => $_getIZ(4);
  @$pb.TagNumber(6)
  set isVerified($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(6)
  $core.bool hasIsVerified() => $_has(4);
  @$pb.TagNumber(6)
  void clearIsVerified() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get creatorUserCid => $_getIZ(5);
  @$pb.TagNumber(7)
  set creatorUserCid($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(7)
  $core.bool hasCreatorUserCid() => $_has(5);
  @$pb.TagNumber(7)
  void clearCreatorUserCid() => clearField(7);

  @$pb.TagNumber(9)
  ChannelPrivacy get privacy => $_getN(6);
  @$pb.TagNumber(9)
  set privacy(ChannelPrivacy v) { setField(9, v); }
  @$pb.TagNumber(9)
  $core.bool hasPrivacy() => $_has(6);
  @$pb.TagNumber(9)
  void clearPrivacy() => clearField(9);

  @$pb.TagNumber(16)
  $core.String get about => $_getSZ(7);
  @$pb.TagNumber(16)
  set about($core.String v) { $_setString(7, v); }
  @$pb.TagNumber(16)
  $core.bool hasAbout() => $_has(7);
  @$pb.TagNumber(16)
  void clearAbout() => clearField(16);

  @$pb.TagNumber(17)
  $core.String get inviteLinkHash => $_getSZ(8);
  @$pb.TagNumber(17)
  set inviteLinkHash($core.String v) { $_setString(8, v); }
  @$pb.TagNumber(17)
  $core.bool hasInviteLinkHash() => $_has(8);
  @$pb.TagNumber(17)
  void clearInviteLinkHash() => clearField(17);

  @$pb.TagNumber(19)
  $core.int get postSeq => $_getIZ(9);
  @$pb.TagNumber(19)
  set postSeq($core.int v) { $_setUnsignedInt32(9, v); }
  @$pb.TagNumber(19)
  $core.bool hasPostSeq() => $_has(9);
  @$pb.TagNumber(19)
  void clearPostSeq() => clearField(19);

  @$pb.TagNumber(20)
  $fixnum.Int64 get sortTime => $_getI64(10);
  @$pb.TagNumber(20)
  set sortTime($fixnum.Int64 v) { $_setInt64(10, v); }
  @$pb.TagNumber(20)
  $core.bool hasSortTime() => $_has(10);
  @$pb.TagNumber(20)
  void clearSortTime() => clearField(20);

  @$pb.TagNumber(21)
  $fixnum.Int64 get syncTime => $_getI64(11);
  @$pb.TagNumber(21)
  set syncTime($fixnum.Int64 v) { $_setInt64(11, v); }
  @$pb.TagNumber(21)
  $core.bool hasSyncTime() => $_has(11);
  @$pb.TagNumber(21)
  void clearSyncTime() => clearField(21);

  @$pb.TagNumber(25)
  Message get lastMessage => $_getN(12);
  @$pb.TagNumber(25)
  set lastMessage(Message v) { setField(25, v); }
  @$pb.TagNumber(25)
  $core.bool hasLastMessage() => $_has(12);
  @$pb.TagNumber(25)
  void clearLastMessage() => clearField(25);
  @$pb.TagNumber(25)
  Message ensureLastMessage() => $_ensure(12);

  @$pb.TagNumber(26)
  Message get pinnedMessage => $_getN(13);
  @$pb.TagNumber(26)
  set pinnedMessage(Message v) { setField(26, v); }
  @$pb.TagNumber(26)
  $core.bool hasPinnedMessage() => $_has(13);
  @$pb.TagNumber(26)
  void clearPinnedMessage() => clearField(26);
  @$pb.TagNumber(26)
  Message ensurePinnedMessage() => $_ensure(13);

  @$pb.TagNumber(36)
  $core.int get createdTime => $_getIZ(14);
  @$pb.TagNumber(36)
  set createdTime($core.int v) { $_setUnsignedInt32(14, v); }
  @$pb.TagNumber(36)
  $core.bool hasCreatedTime() => $_has(14);
  @$pb.TagNumber(36)
  void clearCreatedTime() => clearField(36);

  @$pb.TagNumber(38)
  $core.int get isDeleted => $_getIZ(15);
  @$pb.TagNumber(38)
  set isDeleted($core.int v) { $_setUnsignedInt32(15, v); }
  @$pb.TagNumber(38)
  $core.bool hasIsDeleted() => $_has(15);
  @$pb.TagNumber(38)
  void clearIsDeleted() => clearField(38);

  @$pb.TagNumber(39)
  $core.int get isBanned => $_getIZ(16);
  @$pb.TagNumber(39)
  set isBanned($core.int v) { $_setUnsignedInt32(16, v); }
  @$pb.TagNumber(39)
  $core.bool hasIsBanned() => $_has(16);
  @$pb.TagNumber(39)
  void clearIsBanned() => clearField(39);

  @$pb.TagNumber(40)
  $fixnum.Int64 get avatarCount => $_getI64(17);
  @$pb.TagNumber(40)
  set avatarCount($fixnum.Int64 v) { $_setInt64(17, v); }
  @$pb.TagNumber(40)
  $core.bool hasAvatarCount() => $_has(17);
  @$pb.TagNumber(40)
  void clearAvatarCount() => clearField(40);

  @$pb.TagNumber(41)
  ChannelCounts get counts => $_getN(18);
  @$pb.TagNumber(41)
  set counts(ChannelCounts v) { setField(41, v); }
  @$pb.TagNumber(41)
  $core.bool hasCounts() => $_has(18);
  @$pb.TagNumber(41)
  void clearCounts() => clearField(41);
  @$pb.TagNumber(41)
  ChannelCounts ensureCounts() => $_ensure(18);

  @$pb.TagNumber(42)
  ChannelType get channelType => $_getN(19);
  @$pb.TagNumber(42)
  set channelType(ChannelType v) { setField(42, v); }
  @$pb.TagNumber(42)
  $core.bool hasChannelType() => $_has(19);
  @$pb.TagNumber(42)
  void clearChannelType() => clearField(42);

  @$pb.TagNumber(90)
  ChannelNotificationSetting get notificationSetting => $_getN(20);
  @$pb.TagNumber(90)
  set notificationSetting(ChannelNotificationSetting v) { setField(90, v); }
  @$pb.TagNumber(90)
  $core.bool hasNotificationSetting() => $_has(20);
  @$pb.TagNumber(90)
  void clearNotificationSetting() => clearField(90);
  @$pb.TagNumber(90)
  ChannelNotificationSetting ensureNotificationSetting() => $_ensure(20);

  @$pb.TagNumber(100)
  Message get avatar => $_getN(21);
  @$pb.TagNumber(100)
  set avatar(Message v) { setField(100, v); }
  @$pb.TagNumber(100)
  $core.bool hasAvatar() => $_has(21);
  @$pb.TagNumber(100)
  void clearAvatar() => clearField(100);
  @$pb.TagNumber(100)
  Message ensureAvatar() => $_ensure(21);
}

class ChannelPrivacySetting extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelPrivacySetting', createEmptyInstance: create)
    ..a<$core.int>(10, 'onlinePrivacy', $pb.PbFieldType.OU3)
    ..a<$core.int>(11, 'callPrivacy', $pb.PbFieldType.OU3)
    ..a<$core.int>(12, 'addToGroupPrivacy', $pb.PbFieldType.OU3)
    ..a<$core.int>(13, 'seenMessagePrivacy', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelPrivacySetting._() : super();
  factory ChannelPrivacySetting() => create();
  factory ChannelPrivacySetting.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelPrivacySetting.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelPrivacySetting clone() => ChannelPrivacySetting()..mergeFromMessage(this);
  ChannelPrivacySetting copyWith(void Function(ChannelPrivacySetting) updates) => super.copyWith((message) => updates(message as ChannelPrivacySetting));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelPrivacySetting create() => ChannelPrivacySetting._();
  ChannelPrivacySetting createEmptyInstance() => create();
  static $pb.PbList<ChannelPrivacySetting> createRepeated() => $pb.PbList<ChannelPrivacySetting>();
  @$core.pragma('dart2js:noInline')
  static ChannelPrivacySetting getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelPrivacySetting>(create);
  static ChannelPrivacySetting _defaultInstance;

  @$pb.TagNumber(10)
  $core.int get onlinePrivacy => $_getIZ(0);
  @$pb.TagNumber(10)
  set onlinePrivacy($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(10)
  $core.bool hasOnlinePrivacy() => $_has(0);
  @$pb.TagNumber(10)
  void clearOnlinePrivacy() => clearField(10);

  @$pb.TagNumber(11)
  $core.int get callPrivacy => $_getIZ(1);
  @$pb.TagNumber(11)
  set callPrivacy($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(11)
  $core.bool hasCallPrivacy() => $_has(1);
  @$pb.TagNumber(11)
  void clearCallPrivacy() => clearField(11);

  @$pb.TagNumber(12)
  $core.int get addToGroupPrivacy => $_getIZ(2);
  @$pb.TagNumber(12)
  set addToGroupPrivacy($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(12)
  $core.bool hasAddToGroupPrivacy() => $_has(2);
  @$pb.TagNumber(12)
  void clearAddToGroupPrivacy() => clearField(12);

  @$pb.TagNumber(13)
  $core.int get seenMessagePrivacy => $_getIZ(3);
  @$pb.TagNumber(13)
  set seenMessagePrivacy($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(13)
  $core.bool hasSeenMessagePrivacy() => $_has(3);
  @$pb.TagNumber(13)
  void clearSeenMessagePrivacy() => clearField(13);
}

class ChannelNotificationSetting extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelNotificationSetting', createEmptyInstance: create)
    ..aOB(2, 'socialLedOn')
    ..aOS(3, 'socialLedColor')
    ..aOB(4, 'requestToFollowYou')
    ..aOB(5, 'followedChannel')
    ..aOB(6, 'acceptedChannelFollowRequest')
    ..aOB(7, 'channelMessageLiked')
    ..aOB(8, 'channelMessageCommented')
    ..aOB(9, 'mentionedChannelInMessage')
    ..aOB(10, 'mentionedChannelInComment')
    ..aOB(11, 'contactsJoined')
    ..aOB(12, 'directMessage')
    ..aOB(13, 'directAlert')
    ..aOB(14, 'directPreview')
    ..aOB(15, 'directLedOn')
    ..aOB(16, 'directLedColor')
    ..aOB(17, 'directVibrate')
    ..aOB(18, 'directPopup')
    ..aOB(19, 'directSound')
    ..aOB(20, 'directPriority')
    ..hasRequiredFields = false
  ;

  ChannelNotificationSetting._() : super();
  factory ChannelNotificationSetting() => create();
  factory ChannelNotificationSetting.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelNotificationSetting.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelNotificationSetting clone() => ChannelNotificationSetting()..mergeFromMessage(this);
  ChannelNotificationSetting copyWith(void Function(ChannelNotificationSetting) updates) => super.copyWith((message) => updates(message as ChannelNotificationSetting));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelNotificationSetting create() => ChannelNotificationSetting._();
  ChannelNotificationSetting createEmptyInstance() => create();
  static $pb.PbList<ChannelNotificationSetting> createRepeated() => $pb.PbList<ChannelNotificationSetting>();
  @$core.pragma('dart2js:noInline')
  static ChannelNotificationSetting getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelNotificationSetting>(create);
  static ChannelNotificationSetting _defaultInstance;

  @$pb.TagNumber(2)
  $core.bool get socialLedOn => $_getBF(0);
  @$pb.TagNumber(2)
  set socialLedOn($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(2)
  $core.bool hasSocialLedOn() => $_has(0);
  @$pb.TagNumber(2)
  void clearSocialLedOn() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get socialLedColor => $_getSZ(1);
  @$pb.TagNumber(3)
  set socialLedColor($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasSocialLedColor() => $_has(1);
  @$pb.TagNumber(3)
  void clearSocialLedColor() => clearField(3);

  @$pb.TagNumber(4)
  $core.bool get requestToFollowYou => $_getBF(2);
  @$pb.TagNumber(4)
  set requestToFollowYou($core.bool v) { $_setBool(2, v); }
  @$pb.TagNumber(4)
  $core.bool hasRequestToFollowYou() => $_has(2);
  @$pb.TagNumber(4)
  void clearRequestToFollowYou() => clearField(4);

  @$pb.TagNumber(5)
  $core.bool get followedChannel => $_getBF(3);
  @$pb.TagNumber(5)
  set followedChannel($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasFollowedChannel() => $_has(3);
  @$pb.TagNumber(5)
  void clearFollowedChannel() => clearField(5);

  @$pb.TagNumber(6)
  $core.bool get acceptedChannelFollowRequest => $_getBF(4);
  @$pb.TagNumber(6)
  set acceptedChannelFollowRequest($core.bool v) { $_setBool(4, v); }
  @$pb.TagNumber(6)
  $core.bool hasAcceptedChannelFollowRequest() => $_has(4);
  @$pb.TagNumber(6)
  void clearAcceptedChannelFollowRequest() => clearField(6);

  @$pb.TagNumber(7)
  $core.bool get channelMessageLiked => $_getBF(5);
  @$pb.TagNumber(7)
  set channelMessageLiked($core.bool v) { $_setBool(5, v); }
  @$pb.TagNumber(7)
  $core.bool hasChannelMessageLiked() => $_has(5);
  @$pb.TagNumber(7)
  void clearChannelMessageLiked() => clearField(7);

  @$pb.TagNumber(8)
  $core.bool get channelMessageCommented => $_getBF(6);
  @$pb.TagNumber(8)
  set channelMessageCommented($core.bool v) { $_setBool(6, v); }
  @$pb.TagNumber(8)
  $core.bool hasChannelMessageCommented() => $_has(6);
  @$pb.TagNumber(8)
  void clearChannelMessageCommented() => clearField(8);

  @$pb.TagNumber(9)
  $core.bool get mentionedChannelInMessage => $_getBF(7);
  @$pb.TagNumber(9)
  set mentionedChannelInMessage($core.bool v) { $_setBool(7, v); }
  @$pb.TagNumber(9)
  $core.bool hasMentionedChannelInMessage() => $_has(7);
  @$pb.TagNumber(9)
  void clearMentionedChannelInMessage() => clearField(9);

  @$pb.TagNumber(10)
  $core.bool get mentionedChannelInComment => $_getBF(8);
  @$pb.TagNumber(10)
  set mentionedChannelInComment($core.bool v) { $_setBool(8, v); }
  @$pb.TagNumber(10)
  $core.bool hasMentionedChannelInComment() => $_has(8);
  @$pb.TagNumber(10)
  void clearMentionedChannelInComment() => clearField(10);

  @$pb.TagNumber(11)
  $core.bool get contactsJoined => $_getBF(9);
  @$pb.TagNumber(11)
  set contactsJoined($core.bool v) { $_setBool(9, v); }
  @$pb.TagNumber(11)
  $core.bool hasContactsJoined() => $_has(9);
  @$pb.TagNumber(11)
  void clearContactsJoined() => clearField(11);

  @$pb.TagNumber(12)
  $core.bool get directMessage => $_getBF(10);
  @$pb.TagNumber(12)
  set directMessage($core.bool v) { $_setBool(10, v); }
  @$pb.TagNumber(12)
  $core.bool hasDirectMessage() => $_has(10);
  @$pb.TagNumber(12)
  void clearDirectMessage() => clearField(12);

  @$pb.TagNumber(13)
  $core.bool get directAlert => $_getBF(11);
  @$pb.TagNumber(13)
  set directAlert($core.bool v) { $_setBool(11, v); }
  @$pb.TagNumber(13)
  $core.bool hasDirectAlert() => $_has(11);
  @$pb.TagNumber(13)
  void clearDirectAlert() => clearField(13);

  @$pb.TagNumber(14)
  $core.bool get directPreview => $_getBF(12);
  @$pb.TagNumber(14)
  set directPreview($core.bool v) { $_setBool(12, v); }
  @$pb.TagNumber(14)
  $core.bool hasDirectPreview() => $_has(12);
  @$pb.TagNumber(14)
  void clearDirectPreview() => clearField(14);

  @$pb.TagNumber(15)
  $core.bool get directLedOn => $_getBF(13);
  @$pb.TagNumber(15)
  set directLedOn($core.bool v) { $_setBool(13, v); }
  @$pb.TagNumber(15)
  $core.bool hasDirectLedOn() => $_has(13);
  @$pb.TagNumber(15)
  void clearDirectLedOn() => clearField(15);

  @$pb.TagNumber(16)
  $core.bool get directLedColor => $_getBF(14);
  @$pb.TagNumber(16)
  set directLedColor($core.bool v) { $_setBool(14, v); }
  @$pb.TagNumber(16)
  $core.bool hasDirectLedColor() => $_has(14);
  @$pb.TagNumber(16)
  void clearDirectLedColor() => clearField(16);

  @$pb.TagNumber(17)
  $core.bool get directVibrate => $_getBF(15);
  @$pb.TagNumber(17)
  set directVibrate($core.bool v) { $_setBool(15, v); }
  @$pb.TagNumber(17)
  $core.bool hasDirectVibrate() => $_has(15);
  @$pb.TagNumber(17)
  void clearDirectVibrate() => clearField(17);

  @$pb.TagNumber(18)
  $core.bool get directPopup => $_getBF(16);
  @$pb.TagNumber(18)
  set directPopup($core.bool v) { $_setBool(16, v); }
  @$pb.TagNumber(18)
  $core.bool hasDirectPopup() => $_has(16);
  @$pb.TagNumber(18)
  void clearDirectPopup() => clearField(18);

  @$pb.TagNumber(19)
  $core.bool get directSound => $_getBF(17);
  @$pb.TagNumber(19)
  set directSound($core.bool v) { $_setBool(17, v); }
  @$pb.TagNumber(19)
  $core.bool hasDirectSound() => $_has(17);
  @$pb.TagNumber(19)
  void clearDirectSound() => clearField(19);

  @$pb.TagNumber(20)
  $core.bool get directPriority => $_getBF(18);
  @$pb.TagNumber(20)
  set directPriority($core.bool v) { $_setBool(18, v); }
  @$pb.TagNumber(20)
  $core.bool hasDirectPriority() => $_has(18);
  @$pb.TagNumber(20)
  void clearDirectPriority() => clearField(20);
}

class ChannelCounts extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChannelCounts', createEmptyInstance: create)
    ..a<$core.int>(20, 'followersCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(21, 'followingCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(22, 'postsCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(23, 'mediaCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(24, 'photoCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(25, 'videoCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(26, 'gifCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(27, 'audioCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(28, 'voiceCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(29, 'fileCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(30, 'linkCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(31, 'boardCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(32, 'pinedCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(33, 'likesCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(34, 'resharedCount', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ChannelCounts._() : super();
  factory ChannelCounts() => create();
  factory ChannelCounts.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelCounts.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChannelCounts clone() => ChannelCounts()..mergeFromMessage(this);
  ChannelCounts copyWith(void Function(ChannelCounts) updates) => super.copyWith((message) => updates(message as ChannelCounts));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChannelCounts create() => ChannelCounts._();
  ChannelCounts createEmptyInstance() => create();
  static $pb.PbList<ChannelCounts> createRepeated() => $pb.PbList<ChannelCounts>();
  @$core.pragma('dart2js:noInline')
  static ChannelCounts getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelCounts>(create);
  static ChannelCounts _defaultInstance;

  @$pb.TagNumber(20)
  $core.int get followersCount => $_getIZ(0);
  @$pb.TagNumber(20)
  set followersCount($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(20)
  $core.bool hasFollowersCount() => $_has(0);
  @$pb.TagNumber(20)
  void clearFollowersCount() => clearField(20);

  @$pb.TagNumber(21)
  $core.int get followingCount => $_getIZ(1);
  @$pb.TagNumber(21)
  set followingCount($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(21)
  $core.bool hasFollowingCount() => $_has(1);
  @$pb.TagNumber(21)
  void clearFollowingCount() => clearField(21);

  @$pb.TagNumber(22)
  $core.int get postsCount => $_getIZ(2);
  @$pb.TagNumber(22)
  set postsCount($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(22)
  $core.bool hasPostsCount() => $_has(2);
  @$pb.TagNumber(22)
  void clearPostsCount() => clearField(22);

  @$pb.TagNumber(23)
  $core.int get mediaCount => $_getIZ(3);
  @$pb.TagNumber(23)
  set mediaCount($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(23)
  $core.bool hasMediaCount() => $_has(3);
  @$pb.TagNumber(23)
  void clearMediaCount() => clearField(23);

  @$pb.TagNumber(24)
  $core.int get photoCount => $_getIZ(4);
  @$pb.TagNumber(24)
  set photoCount($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(24)
  $core.bool hasPhotoCount() => $_has(4);
  @$pb.TagNumber(24)
  void clearPhotoCount() => clearField(24);

  @$pb.TagNumber(25)
  $core.int get videoCount => $_getIZ(5);
  @$pb.TagNumber(25)
  set videoCount($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(25)
  $core.bool hasVideoCount() => $_has(5);
  @$pb.TagNumber(25)
  void clearVideoCount() => clearField(25);

  @$pb.TagNumber(26)
  $core.int get gifCount => $_getIZ(6);
  @$pb.TagNumber(26)
  set gifCount($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(26)
  $core.bool hasGifCount() => $_has(6);
  @$pb.TagNumber(26)
  void clearGifCount() => clearField(26);

  @$pb.TagNumber(27)
  $core.int get audioCount => $_getIZ(7);
  @$pb.TagNumber(27)
  set audioCount($core.int v) { $_setUnsignedInt32(7, v); }
  @$pb.TagNumber(27)
  $core.bool hasAudioCount() => $_has(7);
  @$pb.TagNumber(27)
  void clearAudioCount() => clearField(27);

  @$pb.TagNumber(28)
  $core.int get voiceCount => $_getIZ(8);
  @$pb.TagNumber(28)
  set voiceCount($core.int v) { $_setUnsignedInt32(8, v); }
  @$pb.TagNumber(28)
  $core.bool hasVoiceCount() => $_has(8);
  @$pb.TagNumber(28)
  void clearVoiceCount() => clearField(28);

  @$pb.TagNumber(29)
  $core.int get fileCount => $_getIZ(9);
  @$pb.TagNumber(29)
  set fileCount($core.int v) { $_setUnsignedInt32(9, v); }
  @$pb.TagNumber(29)
  $core.bool hasFileCount() => $_has(9);
  @$pb.TagNumber(29)
  void clearFileCount() => clearField(29);

  @$pb.TagNumber(30)
  $core.int get linkCount => $_getIZ(10);
  @$pb.TagNumber(30)
  set linkCount($core.int v) { $_setUnsignedInt32(10, v); }
  @$pb.TagNumber(30)
  $core.bool hasLinkCount() => $_has(10);
  @$pb.TagNumber(30)
  void clearLinkCount() => clearField(30);

  @$pb.TagNumber(31)
  $core.int get boardCount => $_getIZ(11);
  @$pb.TagNumber(31)
  set boardCount($core.int v) { $_setUnsignedInt32(11, v); }
  @$pb.TagNumber(31)
  $core.bool hasBoardCount() => $_has(11);
  @$pb.TagNumber(31)
  void clearBoardCount() => clearField(31);

  @$pb.TagNumber(32)
  $core.int get pinedCount => $_getIZ(12);
  @$pb.TagNumber(32)
  set pinedCount($core.int v) { $_setUnsignedInt32(12, v); }
  @$pb.TagNumber(32)
  $core.bool hasPinedCount() => $_has(12);
  @$pb.TagNumber(32)
  void clearPinedCount() => clearField(32);

  @$pb.TagNumber(33)
  $core.int get likesCount => $_getIZ(13);
  @$pb.TagNumber(33)
  set likesCount($core.int v) { $_setUnsignedInt32(13, v); }
  @$pb.TagNumber(33)
  $core.bool hasLikesCount() => $_has(13);
  @$pb.TagNumber(33)
  void clearLikesCount() => clearField(33);

  @$pb.TagNumber(34)
  $core.int get resharedCount => $_getIZ(14);
  @$pb.TagNumber(34)
  set resharedCount($core.int v) { $_setUnsignedInt32(14, v); }
  @$pb.TagNumber(34)
  $core.bool hasResharedCount() => $_has(14);
  @$pb.TagNumber(34)
  void clearResharedCount() => clearField(34);
}

class Store extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Store', createEmptyInstance: create)
    ..aOS(1, 'address')
    ..aOS(2, 'phone')
    ..hasRequiredFields = false
  ;

  Store._() : super();
  factory Store() => create();
  factory Store.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Store.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Store clone() => Store()..mergeFromMessage(this);
  Store copyWith(void Function(Store) updates) => super.copyWith((message) => updates(message as Store));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Store create() => Store._();
  Store createEmptyInstance() => create();
  static $pb.PbList<Store> createRepeated() => $pb.PbList<Store>();
  @$core.pragma('dart2js:noInline')
  static Store getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Store>(create);
  static Store _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get address => $_getSZ(0);
  @$pb.TagNumber(1)
  set address($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAddress() => $_has(0);
  @$pb.TagNumber(1)
  void clearAddress() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get phone => $_getSZ(1);
  @$pb.TagNumber(2)
  set phone($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasPhone() => $_has(1);
  @$pb.TagNumber(2)
  void clearPhone() => clearField(2);
}

class Product extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Product', createEmptyInstance: create)
    ..a<$core.int>(1, 'productId', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, 'feeRate', $pb.PbFieldType.OU3)
    ..a<$core.int>(5, 'salesCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(6, 'price', $pb.PbFieldType.OU3)
    ..aOS(50, 'category')
    ..aOS(51, 'brand')
    ..a<$core.int>(53, 'categoryId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  Product._() : super();
  factory Product() => create();
  factory Product.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Product.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Product clone() => Product()..mergeFromMessage(this);
  Product copyWith(void Function(Product) updates) => super.copyWith((message) => updates(message as Product));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Product create() => Product._();
  Product createEmptyInstance() => create();
  static $pb.PbList<Product> createRepeated() => $pb.PbList<Product>();
  @$core.pragma('dart2js:noInline')
  static Product getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Product>(create);
  static Product _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get productId => $_getIZ(0);
  @$pb.TagNumber(1)
  set productId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasProductId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProductId() => clearField(1);

  @$pb.TagNumber(3)
  $core.int get feeRate => $_getIZ(1);
  @$pb.TagNumber(3)
  set feeRate($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasFeeRate() => $_has(1);
  @$pb.TagNumber(3)
  void clearFeeRate() => clearField(3);

  @$pb.TagNumber(5)
  $core.int get salesCount => $_getIZ(2);
  @$pb.TagNumber(5)
  set salesCount($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(5)
  $core.bool hasSalesCount() => $_has(2);
  @$pb.TagNumber(5)
  void clearSalesCount() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get price => $_getIZ(3);
  @$pb.TagNumber(6)
  set price($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(6)
  $core.bool hasPrice() => $_has(3);
  @$pb.TagNumber(6)
  void clearPrice() => clearField(6);

  @$pb.TagNumber(50)
  $core.String get category => $_getSZ(4);
  @$pb.TagNumber(50)
  set category($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(50)
  $core.bool hasCategory() => $_has(4);
  @$pb.TagNumber(50)
  void clearCategory() => clearField(50);

  @$pb.TagNumber(51)
  $core.String get brand => $_getSZ(5);
  @$pb.TagNumber(51)
  set brand($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(51)
  $core.bool hasBrand() => $_has(5);
  @$pb.TagNumber(51)
  void clearBrand() => clearField(51);

  @$pb.TagNumber(53)
  $core.int get categoryId => $_getIZ(6);
  @$pb.TagNumber(53)
  set categoryId($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(53)
  $core.bool hasCategoryId() => $_has(6);
  @$pb.TagNumber(53)
  void clearCategoryId() => clearField(53);
}

class ProductPriceInfo extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ProductPriceInfo', createEmptyInstance: create)
    ..a<$core.int>(1, 'price', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, 'rate', $pb.PbFieldType.OU3)
    ..a<$core.int>(6, 'discountPrice', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  ProductPriceInfo._() : super();
  factory ProductPriceInfo() => create();
  factory ProductPriceInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ProductPriceInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ProductPriceInfo clone() => ProductPriceInfo()..mergeFromMessage(this);
  ProductPriceInfo copyWith(void Function(ProductPriceInfo) updates) => super.copyWith((message) => updates(message as ProductPriceInfo));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ProductPriceInfo create() => ProductPriceInfo._();
  ProductPriceInfo createEmptyInstance() => create();
  static $pb.PbList<ProductPriceInfo> createRepeated() => $pb.PbList<ProductPriceInfo>();
  @$core.pragma('dart2js:noInline')
  static ProductPriceInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ProductPriceInfo>(create);
  static ProductPriceInfo _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get price => $_getIZ(0);
  @$pb.TagNumber(1)
  set price($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasPrice() => $_has(0);
  @$pb.TagNumber(1)
  void clearPrice() => clearField(1);

  @$pb.TagNumber(3)
  $core.int get rate => $_getIZ(1);
  @$pb.TagNumber(3)
  set rate($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasRate() => $_has(1);
  @$pb.TagNumber(3)
  void clearRate() => clearField(3);

  @$pb.TagNumber(6)
  $core.int get discountPrice => $_getIZ(2);
  @$pb.TagNumber(6)
  set discountPrice($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(6)
  $core.bool hasDiscountPrice() => $_has(2);
  @$pb.TagNumber(6)
  void clearDiscountPrice() => clearField(6);
}

class Action extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Action', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'gid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(2, 'actorUserCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, 'actionType', $pb.PbFieldType.OU3)
    ..a<$core.int>(4, 'onUserCid', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(5, 'messageGid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(6, 'commentGid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aInt64(7, 'hashMurm64')
    ..a<$core.int>(8, 'createdTime', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(9, 'onChannelCid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(50, 'actorChannelCid', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  Action._() : super();
  factory Action() => create();
  factory Action.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Action.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Action clone() => Action()..mergeFromMessage(this);
  Action copyWith(void Function(Action) updates) => super.copyWith((message) => updates(message as Action));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Action create() => Action._();
  Action createEmptyInstance() => create();
  static $pb.PbList<Action> createRepeated() => $pb.PbList<Action>();
  @$core.pragma('dart2js:noInline')
  static Action getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Action>(create);
  static Action _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get actorUserCid => $_getIZ(1);
  @$pb.TagNumber(2)
  set actorUserCid($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasActorUserCid() => $_has(1);
  @$pb.TagNumber(2)
  void clearActorUserCid() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get actionType => $_getIZ(2);
  @$pb.TagNumber(3)
  set actionType($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasActionType() => $_has(2);
  @$pb.TagNumber(3)
  void clearActionType() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get onUserCid => $_getIZ(3);
  @$pb.TagNumber(4)
  set onUserCid($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasOnUserCid() => $_has(3);
  @$pb.TagNumber(4)
  void clearOnUserCid() => clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get messageGid => $_getI64(4);
  @$pb.TagNumber(5)
  set messageGid($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasMessageGid() => $_has(4);
  @$pb.TagNumber(5)
  void clearMessageGid() => clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get commentGid => $_getI64(5);
  @$pb.TagNumber(6)
  set commentGid($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasCommentGid() => $_has(5);
  @$pb.TagNumber(6)
  void clearCommentGid() => clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get hashMurm64 => $_getI64(6);
  @$pb.TagNumber(7)
  set hashMurm64($fixnum.Int64 v) { $_setInt64(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasHashMurm64() => $_has(6);
  @$pb.TagNumber(7)
  void clearHashMurm64() => clearField(7);

  @$pb.TagNumber(8)
  $core.int get createdTime => $_getIZ(7);
  @$pb.TagNumber(8)
  set createdTime($core.int v) { $_setUnsignedInt32(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasCreatedTime() => $_has(7);
  @$pb.TagNumber(8)
  void clearCreatedTime() => clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get onChannelCid => $_getI64(8);
  @$pb.TagNumber(9)
  set onChannelCid($fixnum.Int64 v) { $_setInt64(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasOnChannelCid() => $_has(8);
  @$pb.TagNumber(9)
  void clearOnChannelCid() => clearField(9);

  @$pb.TagNumber(50)
  $core.int get actorChannelCid => $_getIZ(9);
  @$pb.TagNumber(50)
  set actorChannelCid($core.int v) { $_setUnsignedInt32(9, v); }
  @$pb.TagNumber(50)
  $core.bool hasActorChannelCid() => $_has(9);
  @$pb.TagNumber(50)
  void clearActorChannelCid() => clearField(50);
}

class Blocked extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Blocked', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'gid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(2, 'userCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(5, 'createdTime', $pb.PbFieldType.OU3)
    ..a<$core.int>(6, 'blockedUserCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(7, 'blockedChannelCid', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  Blocked._() : super();
  factory Blocked() => create();
  factory Blocked.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Blocked.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Blocked clone() => Blocked()..mergeFromMessage(this);
  Blocked copyWith(void Function(Blocked) updates) => super.copyWith((message) => updates(message as Blocked));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Blocked create() => Blocked._();
  Blocked createEmptyInstance() => create();
  static $pb.PbList<Blocked> createRepeated() => $pb.PbList<Blocked>();
  @$core.pragma('dart2js:noInline')
  static Blocked getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Blocked>(create);
  static Blocked _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get userCid => $_getIZ(1);
  @$pb.TagNumber(2)
  set userCid($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserCid() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserCid() => clearField(2);

  @$pb.TagNumber(5)
  $core.int get createdTime => $_getIZ(2);
  @$pb.TagNumber(5)
  set createdTime($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(5)
  $core.bool hasCreatedTime() => $_has(2);
  @$pb.TagNumber(5)
  void clearCreatedTime() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get blockedUserCid => $_getIZ(3);
  @$pb.TagNumber(6)
  set blockedUserCid($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(6)
  $core.bool hasBlockedUserCid() => $_has(3);
  @$pb.TagNumber(6)
  void clearBlockedUserCid() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get blockedChannelCid => $_getIZ(4);
  @$pb.TagNumber(7)
  set blockedChannelCid($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(7)
  $core.bool hasBlockedChannelCid() => $_has(4);
  @$pb.TagNumber(7)
  void clearBlockedChannelCid() => clearField(7);
}

class Comment extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Comment', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'gid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(2, 'byCastCid', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(3, 'messageGid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(4, 'text')
    ..a<$core.int>(5, 'likesCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(6, 'editTime', $pb.PbFieldType.OU3)
    ..a<$core.int>(7, 'createdTime', $pb.PbFieldType.OU3)
    ..a<$core.int>(9, 'channelCid', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  Comment._() : super();
  factory Comment() => create();
  factory Comment.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Comment.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Comment clone() => Comment()..mergeFromMessage(this);
  Comment copyWith(void Function(Comment) updates) => super.copyWith((message) => updates(message as Comment));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Comment create() => Comment._();
  Comment createEmptyInstance() => create();
  static $pb.PbList<Comment> createRepeated() => $pb.PbList<Comment>();
  @$core.pragma('dart2js:noInline')
  static Comment getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Comment>(create);
  static Comment _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get byCastCid => $_getIZ(1);
  @$pb.TagNumber(2)
  set byCastCid($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasByCastCid() => $_has(1);
  @$pb.TagNumber(2)
  void clearByCastCid() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get messageGid => $_getI64(2);
  @$pb.TagNumber(3)
  set messageGid($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMessageGid() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessageGid() => clearField(3);

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
  set likesCount($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasLikesCount() => $_has(4);
  @$pb.TagNumber(5)
  void clearLikesCount() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get editTime => $_getIZ(5);
  @$pb.TagNumber(6)
  set editTime($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasEditTime() => $_has(5);
  @$pb.TagNumber(6)
  void clearEditTime() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get createdTime => $_getIZ(6);
  @$pb.TagNumber(7)
  set createdTime($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasCreatedTime() => $_has(6);
  @$pb.TagNumber(7)
  void clearCreatedTime() => clearField(7);

  @$pb.TagNumber(9)
  $core.int get channelCid => $_getIZ(7);
  @$pb.TagNumber(9)
  set channelCid($core.int v) { $_setUnsignedInt32(7, v); }
  @$pb.TagNumber(9)
  $core.bool hasChannelCid() => $_has(7);
  @$pb.TagNumber(9)
  void clearChannelCid() => clearField(9);
}

class Followed extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Followed', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'gid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(2, 'byUserCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, 'targetCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(4, 'createdTime', $pb.PbFieldType.OU3)
    ..a<$core.int>(10, 'byChannelCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(11, 'targetChannelId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  Followed._() : super();
  factory Followed() => create();
  factory Followed.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Followed.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Followed clone() => Followed()..mergeFromMessage(this);
  Followed copyWith(void Function(Followed) updates) => super.copyWith((message) => updates(message as Followed));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Followed create() => Followed._();
  Followed createEmptyInstance() => create();
  static $pb.PbList<Followed> createRepeated() => $pb.PbList<Followed>();
  @$core.pragma('dart2js:noInline')
  static Followed getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Followed>(create);
  static Followed _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get byUserCid => $_getIZ(1);
  @$pb.TagNumber(2)
  set byUserCid($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasByUserCid() => $_has(1);
  @$pb.TagNumber(2)
  void clearByUserCid() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get targetCid => $_getIZ(2);
  @$pb.TagNumber(3)
  set targetCid($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasTargetCid() => $_has(2);
  @$pb.TagNumber(3)
  void clearTargetCid() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get createdTime => $_getIZ(3);
  @$pb.TagNumber(4)
  set createdTime($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasCreatedTime() => $_has(3);
  @$pb.TagNumber(4)
  void clearCreatedTime() => clearField(4);

  @$pb.TagNumber(10)
  $core.int get byChannelCid => $_getIZ(4);
  @$pb.TagNumber(10)
  set byChannelCid($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(10)
  $core.bool hasByChannelCid() => $_has(4);
  @$pb.TagNumber(10)
  void clearByChannelCid() => clearField(10);

  @$pb.TagNumber(11)
  $core.int get targetChannelId => $_getIZ(5);
  @$pb.TagNumber(11)
  set targetChannelId($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(11)
  $core.bool hasTargetChannelId() => $_has(5);
  @$pb.TagNumber(11)
  void clearTargetChannelId() => clearField(11);
}

class Reaction extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Reaction', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'gid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aInt64(2, 'forMessageCid')
    ..a<$core.int>(3, 'byUserCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(5, 'createdTime', $pb.PbFieldType.OU3)
    ..a<$core.int>(10, 'byChannelCid', $pb.PbFieldType.OU3)
    ..aInt64(11, 'forChannelCid')
    ..hasRequiredFields = false
  ;

  Reaction._() : super();
  factory Reaction() => create();
  factory Reaction.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Reaction.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Reaction clone() => Reaction()..mergeFromMessage(this);
  Reaction copyWith(void Function(Reaction) updates) => super.copyWith((message) => updates(message as Reaction));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Reaction create() => Reaction._();
  Reaction createEmptyInstance() => create();
  static $pb.PbList<Reaction> createRepeated() => $pb.PbList<Reaction>();
  @$core.pragma('dart2js:noInline')
  static Reaction getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Reaction>(create);
  static Reaction _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get forMessageCid => $_getI64(1);
  @$pb.TagNumber(2)
  set forMessageCid($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasForMessageCid() => $_has(1);
  @$pb.TagNumber(2)
  void clearForMessageCid() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get byUserCid => $_getIZ(2);
  @$pb.TagNumber(3)
  set byUserCid($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasByUserCid() => $_has(2);
  @$pb.TagNumber(3)
  void clearByUserCid() => clearField(3);

  @$pb.TagNumber(5)
  $core.int get createdTime => $_getIZ(3);
  @$pb.TagNumber(5)
  set createdTime($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasCreatedTime() => $_has(3);
  @$pb.TagNumber(5)
  void clearCreatedTime() => clearField(5);

  @$pb.TagNumber(10)
  $core.int get byChannelCid => $_getIZ(4);
  @$pb.TagNumber(10)
  set byChannelCid($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(10)
  $core.bool hasByChannelCid() => $_has(4);
  @$pb.TagNumber(10)
  void clearByChannelCid() => clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get forChannelCid => $_getI64(5);
  @$pb.TagNumber(11)
  set forChannelCid($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(11)
  $core.bool hasForChannelCid() => $_has(5);
  @$pb.TagNumber(11)
  void clearForChannelCid() => clearField(11);
}

class Contact extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Contact', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'gid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(2, 'userCid', $pb.PbFieldType.OU3)
    ..aInt64(3, 'clientId')
    ..aOS(4, 'phone')
    ..aOS(5, 'firstName')
    ..aOS(6, 'lastName')
    ..a<$core.int>(10, 'channelCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(12, 'targetUserCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(15, 'targetChannelCid', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  Contact._() : super();
  factory Contact() => create();
  factory Contact.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Contact.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Contact clone() => Contact()..mergeFromMessage(this);
  Contact copyWith(void Function(Contact) updates) => super.copyWith((message) => updates(message as Contact));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Contact create() => Contact._();
  Contact createEmptyInstance() => create();
  static $pb.PbList<Contact> createRepeated() => $pb.PbList<Contact>();
  @$core.pragma('dart2js:noInline')
  static Contact getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Contact>(create);
  static Contact _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get userCid => $_getIZ(1);
  @$pb.TagNumber(2)
  set userCid($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserCid() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserCid() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get clientId => $_getI64(2);
  @$pb.TagNumber(3)
  set clientId($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasClientId() => $_has(2);
  @$pb.TagNumber(3)
  void clearClientId() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get phone => $_getSZ(3);
  @$pb.TagNumber(4)
  set phone($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasPhone() => $_has(3);
  @$pb.TagNumber(4)
  void clearPhone() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get firstName => $_getSZ(4);
  @$pb.TagNumber(5)
  set firstName($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasFirstName() => $_has(4);
  @$pb.TagNumber(5)
  void clearFirstName() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get lastName => $_getSZ(5);
  @$pb.TagNumber(6)
  set lastName($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasLastName() => $_has(5);
  @$pb.TagNumber(6)
  void clearLastName() => clearField(6);

  @$pb.TagNumber(10)
  $core.int get channelCid => $_getIZ(6);
  @$pb.TagNumber(10)
  set channelCid($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(10)
  $core.bool hasChannelCid() => $_has(6);
  @$pb.TagNumber(10)
  void clearChannelCid() => clearField(10);

  @$pb.TagNumber(12)
  $core.int get targetUserCid => $_getIZ(7);
  @$pb.TagNumber(12)
  set targetUserCid($core.int v) { $_setUnsignedInt32(7, v); }
  @$pb.TagNumber(12)
  $core.bool hasTargetUserCid() => $_has(7);
  @$pb.TagNumber(12)
  void clearTargetUserCid() => clearField(12);

  @$pb.TagNumber(15)
  $core.int get targetChannelCid => $_getIZ(8);
  @$pb.TagNumber(15)
  set targetChannelCid($core.int v) { $_setUnsignedInt32(8, v); }
  @$pb.TagNumber(15)
  $core.bool hasTargetChannelCid() => $_has(8);
  @$pb.TagNumber(15)
  void clearTargetChannelCid() => clearField(15);
}

class Session extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Session', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'gid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(2, 'sessionUuid')
    ..a<$core.int>(3, 'userCid', $pb.PbFieldType.OU3)
    ..aOS(4, 'lastIpAddress')
    ..a<$core.int>(5, 'appVersion', $pb.PbFieldType.OU3)
    ..a<$core.int>(6, 'activeTime', $pb.PbFieldType.OU3)
    ..a<$core.int>(7, 'createdTime', $pb.PbFieldType.OU3)
    ..aOS(8, 'userAgent')
    ..e<SessionPlatform>(9, 'platform', $pb.PbFieldType.OE, defaultOrMaker: SessionPlatform.UNKNOWN_PLATFORM, valueOf: SessionPlatform.valueOf, enumValues: SessionPlatform.values)
    ..hasRequiredFields = false
  ;

  Session._() : super();
  factory Session() => create();
  factory Session.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Session.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Session clone() => Session()..mergeFromMessage(this);
  Session copyWith(void Function(Session) updates) => super.copyWith((message) => updates(message as Session));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Session create() => Session._();
  Session createEmptyInstance() => create();
  static $pb.PbList<Session> createRepeated() => $pb.PbList<Session>();
  @$core.pragma('dart2js:noInline')
  static Session getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Session>(create);
  static Session _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get sessionUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set sessionUuid($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSessionUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearSessionUuid() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get userCid => $_getIZ(2);
  @$pb.TagNumber(3)
  set userCid($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasUserCid() => $_has(2);
  @$pb.TagNumber(3)
  void clearUserCid() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get lastIpAddress => $_getSZ(3);
  @$pb.TagNumber(4)
  set lastIpAddress($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasLastIpAddress() => $_has(3);
  @$pb.TagNumber(4)
  void clearLastIpAddress() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get appVersion => $_getIZ(4);
  @$pb.TagNumber(5)
  set appVersion($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasAppVersion() => $_has(4);
  @$pb.TagNumber(5)
  void clearAppVersion() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get activeTime => $_getIZ(5);
  @$pb.TagNumber(6)
  set activeTime($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasActiveTime() => $_has(5);
  @$pb.TagNumber(6)
  void clearActiveTime() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get createdTime => $_getIZ(6);
  @$pb.TagNumber(7)
  set createdTime($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasCreatedTime() => $_has(6);
  @$pb.TagNumber(7)
  void clearCreatedTime() => clearField(7);

  @$pb.TagNumber(8)
  $core.String get userAgent => $_getSZ(7);
  @$pb.TagNumber(8)
  set userAgent($core.String v) { $_setString(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasUserAgent() => $_has(7);
  @$pb.TagNumber(8)
  void clearUserAgent() => clearField(8);

  @$pb.TagNumber(9)
  SessionPlatform get platform => $_getN(8);
  @$pb.TagNumber(9)
  set platform(SessionPlatform v) { setField(9, v); }
  @$pb.TagNumber(9)
  $core.bool hasPlatform() => $_has(8);
  @$pb.TagNumber(9)
  void clearPlatform() => clearField(9);
}

class Sms extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Sms', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'gid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(2, 'hash')
    ..aOS(3, 'appUuid')
    ..aOS(4, 'clientPhone')
    ..a<$core.int>(5, 'genratedCode', $pb.PbFieldType.OU3)
    ..aOS(6, 'smsSenderNumber')
    ..aOS(7, 'smsSendStatues')
    ..aOS(8, 'smsHttpBody')
    ..aOS(9, 'err')
    ..aOS(10, 'carrier')
    ..a<$core.List<$core.int>>(11, 'country', $pb.PbFieldType.OY)
    ..a<$core.int>(12, 'isValidPhone', $pb.PbFieldType.OU3)
    ..a<$core.int>(13, 'isConfirmed', $pb.PbFieldType.OU3)
    ..a<$core.int>(14, 'isLogin', $pb.PbFieldType.OU3)
    ..a<$core.int>(15, 'isRegister', $pb.PbFieldType.OU3)
    ..a<$core.int>(16, 'retriedCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(17, 'ttl', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  Sms._() : super();
  factory Sms() => create();
  factory Sms.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Sms.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Sms clone() => Sms()..mergeFromMessage(this);
  Sms copyWith(void Function(Sms) updates) => super.copyWith((message) => updates(message as Sms));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Sms create() => Sms._();
  Sms createEmptyInstance() => create();
  static $pb.PbList<Sms> createRepeated() => $pb.PbList<Sms>();
  @$core.pragma('dart2js:noInline')
  static Sms getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Sms>(create);
  static Sms _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get hash => $_getSZ(1);
  @$pb.TagNumber(2)
  set hash($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasHash() => $_has(1);
  @$pb.TagNumber(2)
  void clearHash() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get appUuid => $_getSZ(2);
  @$pb.TagNumber(3)
  set appUuid($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasAppUuid() => $_has(2);
  @$pb.TagNumber(3)
  void clearAppUuid() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get clientPhone => $_getSZ(3);
  @$pb.TagNumber(4)
  set clientPhone($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasClientPhone() => $_has(3);
  @$pb.TagNumber(4)
  void clearClientPhone() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get genratedCode => $_getIZ(4);
  @$pb.TagNumber(5)
  set genratedCode($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasGenratedCode() => $_has(4);
  @$pb.TagNumber(5)
  void clearGenratedCode() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get smsSenderNumber => $_getSZ(5);
  @$pb.TagNumber(6)
  set smsSenderNumber($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasSmsSenderNumber() => $_has(5);
  @$pb.TagNumber(6)
  void clearSmsSenderNumber() => clearField(6);

  @$pb.TagNumber(7)
  $core.String get smsSendStatues => $_getSZ(6);
  @$pb.TagNumber(7)
  set smsSendStatues($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasSmsSendStatues() => $_has(6);
  @$pb.TagNumber(7)
  void clearSmsSendStatues() => clearField(7);

  @$pb.TagNumber(8)
  $core.String get smsHttpBody => $_getSZ(7);
  @$pb.TagNumber(8)
  set smsHttpBody($core.String v) { $_setString(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasSmsHttpBody() => $_has(7);
  @$pb.TagNumber(8)
  void clearSmsHttpBody() => clearField(8);

  @$pb.TagNumber(9)
  $core.String get err => $_getSZ(8);
  @$pb.TagNumber(9)
  set err($core.String v) { $_setString(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasErr() => $_has(8);
  @$pb.TagNumber(9)
  void clearErr() => clearField(9);

  @$pb.TagNumber(10)
  $core.String get carrier => $_getSZ(9);
  @$pb.TagNumber(10)
  set carrier($core.String v) { $_setString(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasCarrier() => $_has(9);
  @$pb.TagNumber(10)
  void clearCarrier() => clearField(10);

  @$pb.TagNumber(11)
  $core.List<$core.int> get country => $_getN(10);
  @$pb.TagNumber(11)
  set country($core.List<$core.int> v) { $_setBytes(10, v); }
  @$pb.TagNumber(11)
  $core.bool hasCountry() => $_has(10);
  @$pb.TagNumber(11)
  void clearCountry() => clearField(11);

  @$pb.TagNumber(12)
  $core.int get isValidPhone => $_getIZ(11);
  @$pb.TagNumber(12)
  set isValidPhone($core.int v) { $_setUnsignedInt32(11, v); }
  @$pb.TagNumber(12)
  $core.bool hasIsValidPhone() => $_has(11);
  @$pb.TagNumber(12)
  void clearIsValidPhone() => clearField(12);

  @$pb.TagNumber(13)
  $core.int get isConfirmed => $_getIZ(12);
  @$pb.TagNumber(13)
  set isConfirmed($core.int v) { $_setUnsignedInt32(12, v); }
  @$pb.TagNumber(13)
  $core.bool hasIsConfirmed() => $_has(12);
  @$pb.TagNumber(13)
  void clearIsConfirmed() => clearField(13);

  @$pb.TagNumber(14)
  $core.int get isLogin => $_getIZ(13);
  @$pb.TagNumber(14)
  set isLogin($core.int v) { $_setUnsignedInt32(13, v); }
  @$pb.TagNumber(14)
  $core.bool hasIsLogin() => $_has(13);
  @$pb.TagNumber(14)
  void clearIsLogin() => clearField(14);

  @$pb.TagNumber(15)
  $core.int get isRegister => $_getIZ(14);
  @$pb.TagNumber(15)
  set isRegister($core.int v) { $_setUnsignedInt32(14, v); }
  @$pb.TagNumber(15)
  $core.bool hasIsRegister() => $_has(14);
  @$pb.TagNumber(15)
  void clearIsRegister() => clearField(15);

  @$pb.TagNumber(16)
  $core.int get retriedCount => $_getIZ(15);
  @$pb.TagNumber(16)
  set retriedCount($core.int v) { $_setUnsignedInt32(15, v); }
  @$pb.TagNumber(16)
  $core.bool hasRetriedCount() => $_has(15);
  @$pb.TagNumber(16)
  void clearRetriedCount() => clearField(16);

  @$pb.TagNumber(17)
  $core.int get ttl => $_getIZ(16);
  @$pb.TagNumber(17)
  set ttl($core.int v) { $_setUnsignedInt32(16, v); }
  @$pb.TagNumber(17)
  $core.bool hasTtl() => $_has(16);
  @$pb.TagNumber(17)
  void clearTtl() => clearField(17);
}

class Tag extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Tag', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'gid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(2, 'name')
    ..a<$core.int>(3, 'count', $pb.PbFieldType.OU3)
    ..aOB(5, 'isBlocked')
    ..a<$core.int>(6, 'groupCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(7, 'createdTime', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  Tag._() : super();
  factory Tag() => create();
  factory Tag.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Tag.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Tag clone() => Tag()..mergeFromMessage(this);
  Tag copyWith(void Function(Tag) updates) => super.copyWith((message) => updates(message as Tag));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Tag create() => Tag._();
  Tag createEmptyInstance() => create();
  static $pb.PbList<Tag> createRepeated() => $pb.PbList<Tag>();
  @$core.pragma('dart2js:noInline')
  static Tag getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Tag>(create);
  static Tag _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

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
  set count($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCount() => $_has(2);
  @$pb.TagNumber(3)
  void clearCount() => clearField(3);

  @$pb.TagNumber(5)
  $core.bool get isBlocked => $_getBF(3);
  @$pb.TagNumber(5)
  set isBlocked($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasIsBlocked() => $_has(3);
  @$pb.TagNumber(5)
  void clearIsBlocked() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get groupCid => $_getIZ(4);
  @$pb.TagNumber(6)
  set groupCid($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(6)
  $core.bool hasGroupCid() => $_has(4);
  @$pb.TagNumber(6)
  void clearGroupCid() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get createdTime => $_getIZ(5);
  @$pb.TagNumber(7)
  set createdTime($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(7)
  $core.bool hasCreatedTime() => $_has(5);
  @$pb.TagNumber(7)
  void clearCreatedTime() => clearField(7);
}

class User extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('User', createEmptyInstance: create)
    ..a<$core.int>(1, 'cid', $pb.PbFieldType.OU3)
    ..aOS(14, 'phone')
    ..aOS(15, 'email')
    ..aOS(17, 'passwordHash')
    ..aOS(18, 'passwordSalt')
    ..a<$core.int>(36, 'createdTime', $pb.PbFieldType.OU3)
    ..a<$core.int>(37, 'versionTime', $pb.PbFieldType.OU3)
    ..a<$core.int>(38, 'isDeleted', $pb.PbFieldType.OU3)
    ..a<$core.int>(39, 'isBanned', $pb.PbFieldType.OU3)
    ..a<$core.int>(50, 'primaryChannelChangedTime', $pb.PbFieldType.OU3)
    ..aOM<Channel>(80, 'UserCounts', protoName: 'UserCounts', subBuilder: Channel.create)
    ..aOM<Channel>(100, 'primaryChannel', subBuilder: Channel.create)
    ..pc<Channel>(101, 'channels', $pb.PbFieldType.PM, subBuilder: Channel.create)
    ..pc<Session>(102, 'sessions', $pb.PbFieldType.PM, subBuilder: Session.create)
    ..hasRequiredFields = false
  ;

  User._() : super();
  factory User() => create();
  factory User.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory User.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  User clone() => User()..mergeFromMessage(this);
  User copyWith(void Function(User) updates) => super.copyWith((message) => updates(message as User));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static User create() => User._();
  User createEmptyInstance() => create();
  static $pb.PbList<User> createRepeated() => $pb.PbList<User>();
  @$core.pragma('dart2js:noInline')
  static User getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<User>(create);
  static User _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get cid => $_getIZ(0);
  @$pb.TagNumber(1)
  set cid($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCid() => $_has(0);
  @$pb.TagNumber(1)
  void clearCid() => clearField(1);

  @$pb.TagNumber(14)
  $core.String get phone => $_getSZ(1);
  @$pb.TagNumber(14)
  set phone($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(14)
  $core.bool hasPhone() => $_has(1);
  @$pb.TagNumber(14)
  void clearPhone() => clearField(14);

  @$pb.TagNumber(15)
  $core.String get email => $_getSZ(2);
  @$pb.TagNumber(15)
  set email($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(15)
  $core.bool hasEmail() => $_has(2);
  @$pb.TagNumber(15)
  void clearEmail() => clearField(15);

  @$pb.TagNumber(17)
  $core.String get passwordHash => $_getSZ(3);
  @$pb.TagNumber(17)
  set passwordHash($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(17)
  $core.bool hasPasswordHash() => $_has(3);
  @$pb.TagNumber(17)
  void clearPasswordHash() => clearField(17);

  @$pb.TagNumber(18)
  $core.String get passwordSalt => $_getSZ(4);
  @$pb.TagNumber(18)
  set passwordSalt($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(18)
  $core.bool hasPasswordSalt() => $_has(4);
  @$pb.TagNumber(18)
  void clearPasswordSalt() => clearField(18);

  @$pb.TagNumber(36)
  $core.int get createdTime => $_getIZ(5);
  @$pb.TagNumber(36)
  set createdTime($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(36)
  $core.bool hasCreatedTime() => $_has(5);
  @$pb.TagNumber(36)
  void clearCreatedTime() => clearField(36);

  @$pb.TagNumber(37)
  $core.int get versionTime => $_getIZ(6);
  @$pb.TagNumber(37)
  set versionTime($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(37)
  $core.bool hasVersionTime() => $_has(6);
  @$pb.TagNumber(37)
  void clearVersionTime() => clearField(37);

  @$pb.TagNumber(38)
  $core.int get isDeleted => $_getIZ(7);
  @$pb.TagNumber(38)
  set isDeleted($core.int v) { $_setUnsignedInt32(7, v); }
  @$pb.TagNumber(38)
  $core.bool hasIsDeleted() => $_has(7);
  @$pb.TagNumber(38)
  void clearIsDeleted() => clearField(38);

  @$pb.TagNumber(39)
  $core.int get isBanned => $_getIZ(8);
  @$pb.TagNumber(39)
  set isBanned($core.int v) { $_setUnsignedInt32(8, v); }
  @$pb.TagNumber(39)
  $core.bool hasIsBanned() => $_has(8);
  @$pb.TagNumber(39)
  void clearIsBanned() => clearField(39);

  @$pb.TagNumber(50)
  $core.int get primaryChannelChangedTime => $_getIZ(9);
  @$pb.TagNumber(50)
  set primaryChannelChangedTime($core.int v) { $_setUnsignedInt32(9, v); }
  @$pb.TagNumber(50)
  $core.bool hasPrimaryChannelChangedTime() => $_has(9);
  @$pb.TagNumber(50)
  void clearPrimaryChannelChangedTime() => clearField(50);

  @$pb.TagNumber(80)
  Channel get userCounts => $_getN(10);
  @$pb.TagNumber(80)
  set userCounts(Channel v) { setField(80, v); }
  @$pb.TagNumber(80)
  $core.bool hasUserCounts() => $_has(10);
  @$pb.TagNumber(80)
  void clearUserCounts() => clearField(80);
  @$pb.TagNumber(80)
  Channel ensureUserCounts() => $_ensure(10);

  @$pb.TagNumber(100)
  Channel get primaryChannel => $_getN(11);
  @$pb.TagNumber(100)
  set primaryChannel(Channel v) { setField(100, v); }
  @$pb.TagNumber(100)
  $core.bool hasPrimaryChannel() => $_has(11);
  @$pb.TagNumber(100)
  void clearPrimaryChannel() => clearField(100);
  @$pb.TagNumber(100)
  Channel ensurePrimaryChannel() => $_ensure(11);

  @$pb.TagNumber(101)
  $core.List<Channel> get channels => $_getList(12);

  @$pb.TagNumber(102)
  $core.List<Session> get sessions => $_getList(13);
}

class UserCounts extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UserCounts', createEmptyInstance: create)
    ..a<$core.int>(1, 'createdChannels', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  UserCounts._() : super();
  factory UserCounts() => create();
  factory UserCounts.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserCounts.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UserCounts clone() => UserCounts()..mergeFromMessage(this);
  UserCounts copyWith(void Function(UserCounts) updates) => super.copyWith((message) => updates(message as UserCounts));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UserCounts create() => UserCounts._();
  UserCounts createEmptyInstance() => create();
  static $pb.PbList<UserCounts> createRepeated() => $pb.PbList<UserCounts>();
  @$core.pragma('dart2js:noInline')
  static UserCounts getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserCounts>(create);
  static UserCounts _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get createdChannels => $_getIZ(0);
  @$pb.TagNumber(1)
  set createdChannels($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCreatedChannels() => $_has(0);
  @$pb.TagNumber(1)
  void clearCreatedChannels() => clearField(1);
}

class UserRelation extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('UserRelation', createEmptyInstance: create)
    ..aInt64(1, 'relNanoId')
    ..a<$core.int>(2, 'userCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, 'peerUserId', $pb.PbFieldType.OU3)
    ..a<$core.int>(4, 'follwing', $pb.PbFieldType.OU3)
    ..a<$core.int>(5, 'followed', $pb.PbFieldType.OU3)
    ..a<$core.int>(6, 'inContacts', $pb.PbFieldType.OU3)
    ..a<$core.int>(7, 'mutualContact', $pb.PbFieldType.OU3)
    ..a<$core.int>(8, 'isFavorite', $pb.PbFieldType.OU3)
    ..a<$core.int>(9, 'notify', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  UserRelation._() : super();
  factory UserRelation() => create();
  factory UserRelation.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserRelation.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  UserRelation clone() => UserRelation()..mergeFromMessage(this);
  UserRelation copyWith(void Function(UserRelation) updates) => super.copyWith((message) => updates(message as UserRelation));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UserRelation create() => UserRelation._();
  UserRelation createEmptyInstance() => create();
  static $pb.PbList<UserRelation> createRepeated() => $pb.PbList<UserRelation>();
  @$core.pragma('dart2js:noInline')
  static UserRelation getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserRelation>(create);
  static UserRelation _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get relNanoId => $_getI64(0);
  @$pb.TagNumber(1)
  set relNanoId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasRelNanoId() => $_has(0);
  @$pb.TagNumber(1)
  void clearRelNanoId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get userCid => $_getIZ(1);
  @$pb.TagNumber(2)
  set userCid($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserCid() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserCid() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get peerUserId => $_getIZ(2);
  @$pb.TagNumber(3)
  set peerUserId($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasPeerUserId() => $_has(2);
  @$pb.TagNumber(3)
  void clearPeerUserId() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get follwing => $_getIZ(3);
  @$pb.TagNumber(4)
  set follwing($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasFollwing() => $_has(3);
  @$pb.TagNumber(4)
  void clearFollwing() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get followed => $_getIZ(4);
  @$pb.TagNumber(5)
  set followed($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasFollowed() => $_has(4);
  @$pb.TagNumber(5)
  void clearFollowed() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get inContacts => $_getIZ(5);
  @$pb.TagNumber(6)
  set inContacts($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasInContacts() => $_has(5);
  @$pb.TagNumber(6)
  void clearInContacts() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get mutualContact => $_getIZ(6);
  @$pb.TagNumber(7)
  set mutualContact($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasMutualContact() => $_has(6);
  @$pb.TagNumber(7)
  void clearMutualContact() => clearField(7);

  @$pb.TagNumber(8)
  $core.int get isFavorite => $_getIZ(7);
  @$pb.TagNumber(8)
  set isFavorite($core.int v) { $_setUnsignedInt32(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasIsFavorite() => $_has(7);
  @$pb.TagNumber(8)
  void clearIsFavorite() => clearField(8);

  @$pb.TagNumber(9)
  $core.int get notify => $_getIZ(8);
  @$pb.TagNumber(9)
  set notify($core.int v) { $_setUnsignedInt32(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasNotify() => $_has(8);
  @$pb.TagNumber(9)
  void clearNotify() => clearField(9);
}

class Chat extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Chat', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'gid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(5, 'userCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(6, 'peerUserCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(7, 'groupCid', $pb.PbFieldType.OU3)
    ..aOS(9, 'customTitle')
    ..a<$fixnum.Int64>(10, 'pinTimeMs', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aInt64(11, 'fromMsgGid')
    ..a<$core.int>(12, 'unseenCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(13, 'seq', $pb.PbFieldType.OU3)
    ..a<$core.int>(16, 'myLastSeenSeq', $pb.PbFieldType.OU3)
    ..aInt64(17, 'myLastSeenMsgId')
    ..aInt64(18, 'peerLastSeenMsgId')
    ..a<$core.int>(19, 'myLastDeliveredSeq', $pb.PbFieldType.OU3)
    ..aInt64(20, 'myLastDeliveredMsgId')
    ..aInt64(21, 'peerLastDeliveredMsgId')
    ..aOB(22, 'isActive')
    ..aOM<Message>(25, 'lastMessage', subBuilder: Message.create)
    ..aOM<Message>(26, 'pinnedMessage', subBuilder: Message.create)
    ..a<$core.int>(29, 'muteUntil', $pb.PbFieldType.OU3)
    ..aInt64(30, 'sortTimeMs')
    ..a<$core.int>(33, 'createdTime', $pb.PbFieldType.OU3)
    ..a<$core.int>(40, 'channelCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(41, 'peerChannelCid', $pb.PbFieldType.OU3)
    ..aOM<GroupMember>(43, 'groupMember', subBuilder: GroupMember.create)
    ..aOM<FileMsg>(44, 'avatar', subBuilder: FileMsg.create)
    ..a<$fixnum.Int64>(45, 'sortTime', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<MessageDraft>(46, 'draft', subBuilder: MessageDraft.create)
    ..aOM<ChatCustomNotification>(47, 'customNotification', subBuilder: ChatCustomNotification.create)
    ..aOM<Channel>(48, 'channel', subBuilder: Channel.create)
    ..aOM<Contact>(49, 'contact', subBuilder: Contact.create)
    ..aOM<Group>(50, 'group', subBuilder: Group.create)
    ..hasRequiredFields = false
  ;

  Chat._() : super();
  factory Chat() => create();
  factory Chat.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Chat.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Chat clone() => Chat()..mergeFromMessage(this);
  Chat copyWith(void Function(Chat) updates) => super.copyWith((message) => updates(message as Chat));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Chat create() => Chat._();
  Chat createEmptyInstance() => create();
  static $pb.PbList<Chat> createRepeated() => $pb.PbList<Chat>();
  @$core.pragma('dart2js:noInline')
  static Chat getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Chat>(create);
  static Chat _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

  @$pb.TagNumber(5)
  $core.int get userCid => $_getIZ(1);
  @$pb.TagNumber(5)
  set userCid($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(5)
  $core.bool hasUserCid() => $_has(1);
  @$pb.TagNumber(5)
  void clearUserCid() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get peerUserCid => $_getIZ(2);
  @$pb.TagNumber(6)
  set peerUserCid($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(6)
  $core.bool hasPeerUserCid() => $_has(2);
  @$pb.TagNumber(6)
  void clearPeerUserCid() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get groupCid => $_getIZ(3);
  @$pb.TagNumber(7)
  set groupCid($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(7)
  $core.bool hasGroupCid() => $_has(3);
  @$pb.TagNumber(7)
  void clearGroupCid() => clearField(7);

  @$pb.TagNumber(9)
  $core.String get customTitle => $_getSZ(4);
  @$pb.TagNumber(9)
  set customTitle($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(9)
  $core.bool hasCustomTitle() => $_has(4);
  @$pb.TagNumber(9)
  void clearCustomTitle() => clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get pinTimeMs => $_getI64(5);
  @$pb.TagNumber(10)
  set pinTimeMs($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(10)
  $core.bool hasPinTimeMs() => $_has(5);
  @$pb.TagNumber(10)
  void clearPinTimeMs() => clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get fromMsgGid => $_getI64(6);
  @$pb.TagNumber(11)
  set fromMsgGid($fixnum.Int64 v) { $_setInt64(6, v); }
  @$pb.TagNumber(11)
  $core.bool hasFromMsgGid() => $_has(6);
  @$pb.TagNumber(11)
  void clearFromMsgGid() => clearField(11);

  @$pb.TagNumber(12)
  $core.int get unseenCount => $_getIZ(7);
  @$pb.TagNumber(12)
  set unseenCount($core.int v) { $_setUnsignedInt32(7, v); }
  @$pb.TagNumber(12)
  $core.bool hasUnseenCount() => $_has(7);
  @$pb.TagNumber(12)
  void clearUnseenCount() => clearField(12);

  @$pb.TagNumber(13)
  $core.int get seq => $_getIZ(8);
  @$pb.TagNumber(13)
  set seq($core.int v) { $_setUnsignedInt32(8, v); }
  @$pb.TagNumber(13)
  $core.bool hasSeq() => $_has(8);
  @$pb.TagNumber(13)
  void clearSeq() => clearField(13);

  @$pb.TagNumber(16)
  $core.int get myLastSeenSeq => $_getIZ(9);
  @$pb.TagNumber(16)
  set myLastSeenSeq($core.int v) { $_setUnsignedInt32(9, v); }
  @$pb.TagNumber(16)
  $core.bool hasMyLastSeenSeq() => $_has(9);
  @$pb.TagNumber(16)
  void clearMyLastSeenSeq() => clearField(16);

  @$pb.TagNumber(17)
  $fixnum.Int64 get myLastSeenMsgId => $_getI64(10);
  @$pb.TagNumber(17)
  set myLastSeenMsgId($fixnum.Int64 v) { $_setInt64(10, v); }
  @$pb.TagNumber(17)
  $core.bool hasMyLastSeenMsgId() => $_has(10);
  @$pb.TagNumber(17)
  void clearMyLastSeenMsgId() => clearField(17);

  @$pb.TagNumber(18)
  $fixnum.Int64 get peerLastSeenMsgId => $_getI64(11);
  @$pb.TagNumber(18)
  set peerLastSeenMsgId($fixnum.Int64 v) { $_setInt64(11, v); }
  @$pb.TagNumber(18)
  $core.bool hasPeerLastSeenMsgId() => $_has(11);
  @$pb.TagNumber(18)
  void clearPeerLastSeenMsgId() => clearField(18);

  @$pb.TagNumber(19)
  $core.int get myLastDeliveredSeq => $_getIZ(12);
  @$pb.TagNumber(19)
  set myLastDeliveredSeq($core.int v) { $_setUnsignedInt32(12, v); }
  @$pb.TagNumber(19)
  $core.bool hasMyLastDeliveredSeq() => $_has(12);
  @$pb.TagNumber(19)
  void clearMyLastDeliveredSeq() => clearField(19);

  @$pb.TagNumber(20)
  $fixnum.Int64 get myLastDeliveredMsgId => $_getI64(13);
  @$pb.TagNumber(20)
  set myLastDeliveredMsgId($fixnum.Int64 v) { $_setInt64(13, v); }
  @$pb.TagNumber(20)
  $core.bool hasMyLastDeliveredMsgId() => $_has(13);
  @$pb.TagNumber(20)
  void clearMyLastDeliveredMsgId() => clearField(20);

  @$pb.TagNumber(21)
  $fixnum.Int64 get peerLastDeliveredMsgId => $_getI64(14);
  @$pb.TagNumber(21)
  set peerLastDeliveredMsgId($fixnum.Int64 v) { $_setInt64(14, v); }
  @$pb.TagNumber(21)
  $core.bool hasPeerLastDeliveredMsgId() => $_has(14);
  @$pb.TagNumber(21)
  void clearPeerLastDeliveredMsgId() => clearField(21);

  @$pb.TagNumber(22)
  $core.bool get isActive => $_getBF(15);
  @$pb.TagNumber(22)
  set isActive($core.bool v) { $_setBool(15, v); }
  @$pb.TagNumber(22)
  $core.bool hasIsActive() => $_has(15);
  @$pb.TagNumber(22)
  void clearIsActive() => clearField(22);

  @$pb.TagNumber(25)
  Message get lastMessage => $_getN(16);
  @$pb.TagNumber(25)
  set lastMessage(Message v) { setField(25, v); }
  @$pb.TagNumber(25)
  $core.bool hasLastMessage() => $_has(16);
  @$pb.TagNumber(25)
  void clearLastMessage() => clearField(25);
  @$pb.TagNumber(25)
  Message ensureLastMessage() => $_ensure(16);

  @$pb.TagNumber(26)
  Message get pinnedMessage => $_getN(17);
  @$pb.TagNumber(26)
  set pinnedMessage(Message v) { setField(26, v); }
  @$pb.TagNumber(26)
  $core.bool hasPinnedMessage() => $_has(17);
  @$pb.TagNumber(26)
  void clearPinnedMessage() => clearField(26);
  @$pb.TagNumber(26)
  Message ensurePinnedMessage() => $_ensure(17);

  @$pb.TagNumber(29)
  $core.int get muteUntil => $_getIZ(18);
  @$pb.TagNumber(29)
  set muteUntil($core.int v) { $_setUnsignedInt32(18, v); }
  @$pb.TagNumber(29)
  $core.bool hasMuteUntil() => $_has(18);
  @$pb.TagNumber(29)
  void clearMuteUntil() => clearField(29);

  @$pb.TagNumber(30)
  $fixnum.Int64 get sortTimeMs => $_getI64(19);
  @$pb.TagNumber(30)
  set sortTimeMs($fixnum.Int64 v) { $_setInt64(19, v); }
  @$pb.TagNumber(30)
  $core.bool hasSortTimeMs() => $_has(19);
  @$pb.TagNumber(30)
  void clearSortTimeMs() => clearField(30);

  @$pb.TagNumber(33)
  $core.int get createdTime => $_getIZ(20);
  @$pb.TagNumber(33)
  set createdTime($core.int v) { $_setUnsignedInt32(20, v); }
  @$pb.TagNumber(33)
  $core.bool hasCreatedTime() => $_has(20);
  @$pb.TagNumber(33)
  void clearCreatedTime() => clearField(33);

  @$pb.TagNumber(40)
  $core.int get channelCid => $_getIZ(21);
  @$pb.TagNumber(40)
  set channelCid($core.int v) { $_setUnsignedInt32(21, v); }
  @$pb.TagNumber(40)
  $core.bool hasChannelCid() => $_has(21);
  @$pb.TagNumber(40)
  void clearChannelCid() => clearField(40);

  @$pb.TagNumber(41)
  $core.int get peerChannelCid => $_getIZ(22);
  @$pb.TagNumber(41)
  set peerChannelCid($core.int v) { $_setUnsignedInt32(22, v); }
  @$pb.TagNumber(41)
  $core.bool hasPeerChannelCid() => $_has(22);
  @$pb.TagNumber(41)
  void clearPeerChannelCid() => clearField(41);

  @$pb.TagNumber(43)
  GroupMember get groupMember => $_getN(23);
  @$pb.TagNumber(43)
  set groupMember(GroupMember v) { setField(43, v); }
  @$pb.TagNumber(43)
  $core.bool hasGroupMember() => $_has(23);
  @$pb.TagNumber(43)
  void clearGroupMember() => clearField(43);
  @$pb.TagNumber(43)
  GroupMember ensureGroupMember() => $_ensure(23);

  @$pb.TagNumber(44)
  FileMsg get avatar => $_getN(24);
  @$pb.TagNumber(44)
  set avatar(FileMsg v) { setField(44, v); }
  @$pb.TagNumber(44)
  $core.bool hasAvatar() => $_has(24);
  @$pb.TagNumber(44)
  void clearAvatar() => clearField(44);
  @$pb.TagNumber(44)
  FileMsg ensureAvatar() => $_ensure(24);

  @$pb.TagNumber(45)
  $fixnum.Int64 get sortTime => $_getI64(25);
  @$pb.TagNumber(45)
  set sortTime($fixnum.Int64 v) { $_setInt64(25, v); }
  @$pb.TagNumber(45)
  $core.bool hasSortTime() => $_has(25);
  @$pb.TagNumber(45)
  void clearSortTime() => clearField(45);

  @$pb.TagNumber(46)
  MessageDraft get draft => $_getN(26);
  @$pb.TagNumber(46)
  set draft(MessageDraft v) { setField(46, v); }
  @$pb.TagNumber(46)
  $core.bool hasDraft() => $_has(26);
  @$pb.TagNumber(46)
  void clearDraft() => clearField(46);
  @$pb.TagNumber(46)
  MessageDraft ensureDraft() => $_ensure(26);

  @$pb.TagNumber(47)
  ChatCustomNotification get customNotification => $_getN(27);
  @$pb.TagNumber(47)
  set customNotification(ChatCustomNotification v) { setField(47, v); }
  @$pb.TagNumber(47)
  $core.bool hasCustomNotification() => $_has(27);
  @$pb.TagNumber(47)
  void clearCustomNotification() => clearField(47);
  @$pb.TagNumber(47)
  ChatCustomNotification ensureCustomNotification() => $_ensure(27);

  @$pb.TagNumber(48)
  Channel get channel => $_getN(28);
  @$pb.TagNumber(48)
  set channel(Channel v) { setField(48, v); }
  @$pb.TagNumber(48)
  $core.bool hasChannel() => $_has(28);
  @$pb.TagNumber(48)
  void clearChannel() => clearField(48);
  @$pb.TagNumber(48)
  Channel ensureChannel() => $_ensure(28);

  @$pb.TagNumber(49)
  Contact get contact => $_getN(29);
  @$pb.TagNumber(49)
  set contact(Contact v) { setField(49, v); }
  @$pb.TagNumber(49)
  $core.bool hasContact() => $_has(29);
  @$pb.TagNumber(49)
  void clearContact() => clearField(49);
  @$pb.TagNumber(49)
  Contact ensureContact() => $_ensure(29);

  @$pb.TagNumber(50)
  Group get group => $_getN(30);
  @$pb.TagNumber(50)
  set group(Group v) { setField(50, v); }
  @$pb.TagNumber(50)
  $core.bool hasGroup() => $_has(30);
  @$pb.TagNumber(50)
  void clearGroup() => clearField(50);
  @$pb.TagNumber(50)
  Group ensureGroup() => $_ensure(30);
}

class MessageDraft extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('MessageDraft', createEmptyInstance: create)
    ..aOS(34, 'draftText')
    ..aInt64(35, 'dratReplyToMsgId')
    ..hasRequiredFields = false
  ;

  MessageDraft._() : super();
  factory MessageDraft() => create();
  factory MessageDraft.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MessageDraft.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  MessageDraft clone() => MessageDraft()..mergeFromMessage(this);
  MessageDraft copyWith(void Function(MessageDraft) updates) => super.copyWith((message) => updates(message as MessageDraft));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MessageDraft create() => MessageDraft._();
  MessageDraft createEmptyInstance() => create();
  static $pb.PbList<MessageDraft> createRepeated() => $pb.PbList<MessageDraft>();
  @$core.pragma('dart2js:noInline')
  static MessageDraft getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MessageDraft>(create);
  static MessageDraft _defaultInstance;

  @$pb.TagNumber(34)
  $core.String get draftText => $_getSZ(0);
  @$pb.TagNumber(34)
  set draftText($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(34)
  $core.bool hasDraftText() => $_has(0);
  @$pb.TagNumber(34)
  void clearDraftText() => clearField(34);

  @$pb.TagNumber(35)
  $fixnum.Int64 get dratReplyToMsgId => $_getI64(1);
  @$pb.TagNumber(35)
  set dratReplyToMsgId($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(35)
  $core.bool hasDratReplyToMsgId() => $_has(1);
  @$pb.TagNumber(35)
  void clearDratReplyToMsgId() => clearField(35);
}

class ChatCustomNotification extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ChatCustomNotification', createEmptyInstance: create)
    ..aOB(13, 'alert')
    ..aOB(14, 'preview')
    ..aOB(15, 'ledOn')
    ..aOB(16, 'ledColor')
    ..aOB(17, 'vibrate')
    ..aOB(18, 'popup')
    ..aOB(19, 'sound')
    ..aOB(20, 'priority')
    ..hasRequiredFields = false
  ;

  ChatCustomNotification._() : super();
  factory ChatCustomNotification() => create();
  factory ChatCustomNotification.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChatCustomNotification.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ChatCustomNotification clone() => ChatCustomNotification()..mergeFromMessage(this);
  ChatCustomNotification copyWith(void Function(ChatCustomNotification) updates) => super.copyWith((message) => updates(message as ChatCustomNotification));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChatCustomNotification create() => ChatCustomNotification._();
  ChatCustomNotification createEmptyInstance() => create();
  static $pb.PbList<ChatCustomNotification> createRepeated() => $pb.PbList<ChatCustomNotification>();
  @$core.pragma('dart2js:noInline')
  static ChatCustomNotification getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatCustomNotification>(create);
  static ChatCustomNotification _defaultInstance;

  @$pb.TagNumber(13)
  $core.bool get alert => $_getBF(0);
  @$pb.TagNumber(13)
  set alert($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(13)
  $core.bool hasAlert() => $_has(0);
  @$pb.TagNumber(13)
  void clearAlert() => clearField(13);

  @$pb.TagNumber(14)
  $core.bool get preview => $_getBF(1);
  @$pb.TagNumber(14)
  set preview($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(14)
  $core.bool hasPreview() => $_has(1);
  @$pb.TagNumber(14)
  void clearPreview() => clearField(14);

  @$pb.TagNumber(15)
  $core.bool get ledOn => $_getBF(2);
  @$pb.TagNumber(15)
  set ledOn($core.bool v) { $_setBool(2, v); }
  @$pb.TagNumber(15)
  $core.bool hasLedOn() => $_has(2);
  @$pb.TagNumber(15)
  void clearLedOn() => clearField(15);

  @$pb.TagNumber(16)
  $core.bool get ledColor => $_getBF(3);
  @$pb.TagNumber(16)
  set ledColor($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(16)
  $core.bool hasLedColor() => $_has(3);
  @$pb.TagNumber(16)
  void clearLedColor() => clearField(16);

  @$pb.TagNumber(17)
  $core.bool get vibrate => $_getBF(4);
  @$pb.TagNumber(17)
  set vibrate($core.bool v) { $_setBool(4, v); }
  @$pb.TagNumber(17)
  $core.bool hasVibrate() => $_has(4);
  @$pb.TagNumber(17)
  void clearVibrate() => clearField(17);

  @$pb.TagNumber(18)
  $core.bool get popup => $_getBF(5);
  @$pb.TagNumber(18)
  set popup($core.bool v) { $_setBool(5, v); }
  @$pb.TagNumber(18)
  $core.bool hasPopup() => $_has(5);
  @$pb.TagNumber(18)
  void clearPopup() => clearField(18);

  @$pb.TagNumber(19)
  $core.bool get sound => $_getBF(6);
  @$pb.TagNumber(19)
  set sound($core.bool v) { $_setBool(6, v); }
  @$pb.TagNumber(19)
  $core.bool hasSound() => $_has(6);
  @$pb.TagNumber(19)
  void clearSound() => clearField(19);

  @$pb.TagNumber(20)
  $core.bool get priority => $_getBF(7);
  @$pb.TagNumber(20)
  set priority($core.bool v) { $_setBool(7, v); }
  @$pb.TagNumber(20)
  $core.bool hasPriority() => $_has(7);
  @$pb.TagNumber(20)
  void clearPriority() => clearField(20);
}

class Group extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Group', createEmptyInstance: create)
    ..a<$core.int>(1, 'cid', $pb.PbFieldType.OU3)
    ..aOS(3, 'groupTitle')
    ..aOS(4, 'userName')
    ..a<$core.int>(7, 'creatorUserCid', $pb.PbFieldType.OU3)
    ..e<GroupPrivacy>(8, 'groupPrivacy', $pb.PbFieldType.OE, defaultOrMaker: GroupPrivacy.GROUP_UNKNOWN_GP, valueOf: GroupPrivacy.valueOf, enumValues: GroupPrivacy.values)
    ..aOB(9, 'historyViewable')
    ..a<$core.int>(10, 'seq', $pb.PbFieldType.OU3)
    ..a<$core.int>(14, 'avatarCount', $pb.PbFieldType.OU3)
    ..aOS(15, 'about')
    ..aOS(16, 'inviteLinkHash')
    ..a<$core.int>(17, 'membersCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(18, 'adminsCount', $pb.PbFieldType.OU3)
    ..a<$core.int>(19, 'moderatorCounts', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(20, 'sortTime', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(21, 'createdTime', $pb.PbFieldType.OU3)
    ..aOB(23, 'isDeleted')
    ..aOB(24, 'isBanned')
    ..aOM<Message>(25, 'lastMessage', subBuilder: Message.create)
    ..aOM<Message>(26, 'pinnedMessage', subBuilder: Message.create)
    ..aOM<FileMsg>(27, 'avatar', subBuilder: FileMsg.create)
    ..a<$core.int>(31, 'creatorChannelCid', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(40, 'syncTime', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  Group._() : super();
  factory Group() => create();
  factory Group.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Group.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Group clone() => Group()..mergeFromMessage(this);
  Group copyWith(void Function(Group) updates) => super.copyWith((message) => updates(message as Group));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Group create() => Group._();
  Group createEmptyInstance() => create();
  static $pb.PbList<Group> createRepeated() => $pb.PbList<Group>();
  @$core.pragma('dart2js:noInline')
  static Group getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Group>(create);
  static Group _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get cid => $_getIZ(0);
  @$pb.TagNumber(1)
  set cid($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCid() => $_has(0);
  @$pb.TagNumber(1)
  void clearCid() => clearField(1);

  @$pb.TagNumber(3)
  $core.String get groupTitle => $_getSZ(1);
  @$pb.TagNumber(3)
  set groupTitle($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasGroupTitle() => $_has(1);
  @$pb.TagNumber(3)
  void clearGroupTitle() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get userName => $_getSZ(2);
  @$pb.TagNumber(4)
  set userName($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(4)
  $core.bool hasUserName() => $_has(2);
  @$pb.TagNumber(4)
  void clearUserName() => clearField(4);

  @$pb.TagNumber(7)
  $core.int get creatorUserCid => $_getIZ(3);
  @$pb.TagNumber(7)
  set creatorUserCid($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(7)
  $core.bool hasCreatorUserCid() => $_has(3);
  @$pb.TagNumber(7)
  void clearCreatorUserCid() => clearField(7);

  @$pb.TagNumber(8)
  GroupPrivacy get groupPrivacy => $_getN(4);
  @$pb.TagNumber(8)
  set groupPrivacy(GroupPrivacy v) { setField(8, v); }
  @$pb.TagNumber(8)
  $core.bool hasGroupPrivacy() => $_has(4);
  @$pb.TagNumber(8)
  void clearGroupPrivacy() => clearField(8);

  @$pb.TagNumber(9)
  $core.bool get historyViewable => $_getBF(5);
  @$pb.TagNumber(9)
  set historyViewable($core.bool v) { $_setBool(5, v); }
  @$pb.TagNumber(9)
  $core.bool hasHistoryViewable() => $_has(5);
  @$pb.TagNumber(9)
  void clearHistoryViewable() => clearField(9);

  @$pb.TagNumber(10)
  $core.int get seq => $_getIZ(6);
  @$pb.TagNumber(10)
  set seq($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(10)
  $core.bool hasSeq() => $_has(6);
  @$pb.TagNumber(10)
  void clearSeq() => clearField(10);

  @$pb.TagNumber(14)
  $core.int get avatarCount => $_getIZ(7);
  @$pb.TagNumber(14)
  set avatarCount($core.int v) { $_setUnsignedInt32(7, v); }
  @$pb.TagNumber(14)
  $core.bool hasAvatarCount() => $_has(7);
  @$pb.TagNumber(14)
  void clearAvatarCount() => clearField(14);

  @$pb.TagNumber(15)
  $core.String get about => $_getSZ(8);
  @$pb.TagNumber(15)
  set about($core.String v) { $_setString(8, v); }
  @$pb.TagNumber(15)
  $core.bool hasAbout() => $_has(8);
  @$pb.TagNumber(15)
  void clearAbout() => clearField(15);

  @$pb.TagNumber(16)
  $core.String get inviteLinkHash => $_getSZ(9);
  @$pb.TagNumber(16)
  set inviteLinkHash($core.String v) { $_setString(9, v); }
  @$pb.TagNumber(16)
  $core.bool hasInviteLinkHash() => $_has(9);
  @$pb.TagNumber(16)
  void clearInviteLinkHash() => clearField(16);

  @$pb.TagNumber(17)
  $core.int get membersCount => $_getIZ(10);
  @$pb.TagNumber(17)
  set membersCount($core.int v) { $_setUnsignedInt32(10, v); }
  @$pb.TagNumber(17)
  $core.bool hasMembersCount() => $_has(10);
  @$pb.TagNumber(17)
  void clearMembersCount() => clearField(17);

  @$pb.TagNumber(18)
  $core.int get adminsCount => $_getIZ(11);
  @$pb.TagNumber(18)
  set adminsCount($core.int v) { $_setUnsignedInt32(11, v); }
  @$pb.TagNumber(18)
  $core.bool hasAdminsCount() => $_has(11);
  @$pb.TagNumber(18)
  void clearAdminsCount() => clearField(18);

  @$pb.TagNumber(19)
  $core.int get moderatorCounts => $_getIZ(12);
  @$pb.TagNumber(19)
  set moderatorCounts($core.int v) { $_setUnsignedInt32(12, v); }
  @$pb.TagNumber(19)
  $core.bool hasModeratorCounts() => $_has(12);
  @$pb.TagNumber(19)
  void clearModeratorCounts() => clearField(19);

  @$pb.TagNumber(20)
  $fixnum.Int64 get sortTime => $_getI64(13);
  @$pb.TagNumber(20)
  set sortTime($fixnum.Int64 v) { $_setInt64(13, v); }
  @$pb.TagNumber(20)
  $core.bool hasSortTime() => $_has(13);
  @$pb.TagNumber(20)
  void clearSortTime() => clearField(20);

  @$pb.TagNumber(21)
  $core.int get createdTime => $_getIZ(14);
  @$pb.TagNumber(21)
  set createdTime($core.int v) { $_setUnsignedInt32(14, v); }
  @$pb.TagNumber(21)
  $core.bool hasCreatedTime() => $_has(14);
  @$pb.TagNumber(21)
  void clearCreatedTime() => clearField(21);

  @$pb.TagNumber(23)
  $core.bool get isDeleted => $_getBF(15);
  @$pb.TagNumber(23)
  set isDeleted($core.bool v) { $_setBool(15, v); }
  @$pb.TagNumber(23)
  $core.bool hasIsDeleted() => $_has(15);
  @$pb.TagNumber(23)
  void clearIsDeleted() => clearField(23);

  @$pb.TagNumber(24)
  $core.bool get isBanned => $_getBF(16);
  @$pb.TagNumber(24)
  set isBanned($core.bool v) { $_setBool(16, v); }
  @$pb.TagNumber(24)
  $core.bool hasIsBanned() => $_has(16);
  @$pb.TagNumber(24)
  void clearIsBanned() => clearField(24);

  @$pb.TagNumber(25)
  Message get lastMessage => $_getN(17);
  @$pb.TagNumber(25)
  set lastMessage(Message v) { setField(25, v); }
  @$pb.TagNumber(25)
  $core.bool hasLastMessage() => $_has(17);
  @$pb.TagNumber(25)
  void clearLastMessage() => clearField(25);
  @$pb.TagNumber(25)
  Message ensureLastMessage() => $_ensure(17);

  @$pb.TagNumber(26)
  Message get pinnedMessage => $_getN(18);
  @$pb.TagNumber(26)
  set pinnedMessage(Message v) { setField(26, v); }
  @$pb.TagNumber(26)
  $core.bool hasPinnedMessage() => $_has(18);
  @$pb.TagNumber(26)
  void clearPinnedMessage() => clearField(26);
  @$pb.TagNumber(26)
  Message ensurePinnedMessage() => $_ensure(18);

  @$pb.TagNumber(27)
  FileMsg get avatar => $_getN(19);
  @$pb.TagNumber(27)
  set avatar(FileMsg v) { setField(27, v); }
  @$pb.TagNumber(27)
  $core.bool hasAvatar() => $_has(19);
  @$pb.TagNumber(27)
  void clearAvatar() => clearField(27);
  @$pb.TagNumber(27)
  FileMsg ensureAvatar() => $_ensure(19);

  @$pb.TagNumber(31)
  $core.int get creatorChannelCid => $_getIZ(20);
  @$pb.TagNumber(31)
  set creatorChannelCid($core.int v) { $_setUnsignedInt32(20, v); }
  @$pb.TagNumber(31)
  $core.bool hasCreatorChannelCid() => $_has(20);
  @$pb.TagNumber(31)
  void clearCreatorChannelCid() => clearField(31);

  @$pb.TagNumber(40)
  $fixnum.Int64 get syncTime => $_getI64(21);
  @$pb.TagNumber(40)
  set syncTime($fixnum.Int64 v) { $_setInt64(21, v); }
  @$pb.TagNumber(40)
  $core.bool hasSyncTime() => $_has(21);
  @$pb.TagNumber(40)
  void clearSyncTime() => clearField(40);
}

class GroupMember extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('GroupMember', createEmptyInstance: create)
    ..aInt64(1, 'gid')
    ..aInt64(2, 'groupCid')
    ..a<$core.int>(3, 'userCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(4, 'byUserCid', $pb.PbFieldType.OU3)
    ..e<GroupMemberRole>(5, 'groupRole', $pb.PbFieldType.OE, defaultOrMaker: GroupMemberRole.MEMBER_Unknown, valueOf: GroupMemberRole.valueOf, enumValues: GroupMemberRole.values)
    ..a<$core.int>(6, 'createdTime', $pb.PbFieldType.OU3)
    ..a<$core.int>(7, 'byChannelCid', $pb.PbFieldType.OU3)
    ..a<$core.int>(8, 'channelCid', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GroupMember._() : super();
  factory GroupMember() => create();
  factory GroupMember.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupMember.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  GroupMember clone() => GroupMember()..mergeFromMessage(this);
  GroupMember copyWith(void Function(GroupMember) updates) => super.copyWith((message) => updates(message as GroupMember));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupMember create() => GroupMember._();
  GroupMember createEmptyInstance() => create();
  static $pb.PbList<GroupMember> createRepeated() => $pb.PbList<GroupMember>();
  @$core.pragma('dart2js:noInline')
  static GroupMember getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupMember>(create);
  static GroupMember _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get groupCid => $_getI64(1);
  @$pb.TagNumber(2)
  set groupCid($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasGroupCid() => $_has(1);
  @$pb.TagNumber(2)
  void clearGroupCid() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get userCid => $_getIZ(2);
  @$pb.TagNumber(3)
  set userCid($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasUserCid() => $_has(2);
  @$pb.TagNumber(3)
  void clearUserCid() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get byUserCid => $_getIZ(3);
  @$pb.TagNumber(4)
  set byUserCid($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasByUserCid() => $_has(3);
  @$pb.TagNumber(4)
  void clearByUserCid() => clearField(4);

  @$pb.TagNumber(5)
  GroupMemberRole get groupRole => $_getN(4);
  @$pb.TagNumber(5)
  set groupRole(GroupMemberRole v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasGroupRole() => $_has(4);
  @$pb.TagNumber(5)
  void clearGroupRole() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get createdTime => $_getIZ(5);
  @$pb.TagNumber(6)
  set createdTime($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasCreatedTime() => $_has(5);
  @$pb.TagNumber(6)
  void clearCreatedTime() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get byChannelCid => $_getIZ(6);
  @$pb.TagNumber(7)
  set byChannelCid($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasByChannelCid() => $_has(6);
  @$pb.TagNumber(7)
  void clearByChannelCid() => clearField(7);

  @$pb.TagNumber(8)
  $core.int get channelCid => $_getIZ(7);
  @$pb.TagNumber(8)
  set channelCid($core.int v) { $_setUnsignedInt32(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasChannelCid() => $_has(7);
  @$pb.TagNumber(8)
  void clearChannelCid() => clearField(8);
}

class FileMsg extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('FileMsg', createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, 'gid', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(2, 'accessHash', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, 'fileType', $pb.PbFieldType.OU3)
    ..a<$core.int>(4, 'width', $pb.PbFieldType.OU3)
    ..a<$core.int>(5, 'height', $pb.PbFieldType.OU3)
    ..aOS(6, 'extension')
    ..a<$core.int>(7, 'userCid', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(8, 'dataThumb', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(9, 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  FileMsg._() : super();
  factory FileMsg() => create();
  factory FileMsg.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FileMsg.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  FileMsg clone() => FileMsg()..mergeFromMessage(this);
  FileMsg copyWith(void Function(FileMsg) updates) => super.copyWith((message) => updates(message as FileMsg));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FileMsg create() => FileMsg._();
  FileMsg createEmptyInstance() => create();
  static $pb.PbList<FileMsg> createRepeated() => $pb.PbList<FileMsg>();
  @$core.pragma('dart2js:noInline')
  static FileMsg getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileMsg>(create);
  static FileMsg _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get gid => $_getI64(0);
  @$pb.TagNumber(1)
  set gid($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasGid() => $_has(0);
  @$pb.TagNumber(1)
  void clearGid() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get accessHash => $_getIZ(1);
  @$pb.TagNumber(2)
  set accessHash($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasAccessHash() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccessHash() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get fileType => $_getIZ(2);
  @$pb.TagNumber(3)
  set fileType($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFileType() => $_has(2);
  @$pb.TagNumber(3)
  void clearFileType() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get width => $_getIZ(3);
  @$pb.TagNumber(4)
  set width($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasWidth() => $_has(3);
  @$pb.TagNumber(4)
  void clearWidth() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get height => $_getIZ(4);
  @$pb.TagNumber(5)
  set height($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasHeight() => $_has(4);
  @$pb.TagNumber(5)
  void clearHeight() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get extension_6 => $_getSZ(5);
  @$pb.TagNumber(6)
  set extension_6($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasExtension_6() => $_has(5);
  @$pb.TagNumber(6)
  void clearExtension_6() => clearField(6);

  @$pb.TagNumber(7)
  $core.int get userCid => $_getIZ(6);
  @$pb.TagNumber(7)
  set userCid($core.int v) { $_setUnsignedInt32(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasUserCid() => $_has(6);
  @$pb.TagNumber(7)
  void clearUserCid() => clearField(7);

  @$pb.TagNumber(8)
  $core.List<$core.int> get dataThumb => $_getN(7);
  @$pb.TagNumber(8)
  set dataThumb($core.List<$core.int> v) { $_setBytes(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasDataThumb() => $_has(7);
  @$pb.TagNumber(8)
  void clearDataThumb() => clearField(8);

  @$pb.TagNumber(9)
  $core.List<$core.int> get data => $_getN(8);
  @$pb.TagNumber(9)
  set data($core.List<$core.int> v) { $_setBytes(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasData() => $_has(8);
  @$pb.TagNumber(9)
  void clearData() => clearField(9);
}

