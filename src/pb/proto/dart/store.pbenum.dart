///
//  Generated code. Do not modify.
//  source: store.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

// ignore_for_file: UNDEFINED_SHOWN_NAME,UNUSED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class MessageType extends $pb.ProtobufEnum {
  static const MessageType TEXT = MessageType._(0, 'TEXT');
  static const MessageType IMAGE = MessageType._(1, 'IMAGE');
  static const MessageType VIDEO = MessageType._(3, 'VIDEO');
  static const MessageType AUDIO = MessageType._(5, 'AUDIO');
  static const MessageType VOICE = MessageType._(7, 'VOICE');
  static const MessageType GIF = MessageType._(8, 'GIF');
  static const MessageType FILE = MessageType._(9, 'FILE');
  static const MessageType POLL = MessageType._(10, 'POLL');
  static const MessageType LOCATION = MessageType._(11, 'LOCATION');
  static const MessageType LOG = MessageType._(12, 'LOG');
  static const MessageType CONTACT = MessageType._(13, 'CONTACT');
  static const MessageType WALLET = MessageType._(15, 'WALLET');
  static const MessageType PRODUCT = MessageType._(16, 'PRODUCT');
  static const MessageType FORWARD = MessageType._(17, 'FORWARD');
  static const MessageType POST_MEDIA = MessageType._(100, 'POST_MEDIA');

  static const $core.List<MessageType> values = <MessageType> [
    TEXT,
    IMAGE,
    VIDEO,
    AUDIO,
    VOICE,
    GIF,
    FILE,
    POLL,
    LOCATION,
    LOG,
    CONTACT,
    WALLET,
    PRODUCT,
    FORWARD,
    POST_MEDIA,
  ];

  static final $core.Map<$core.int, MessageType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static MessageType valueOf($core.int value) => _byValue[value];

  const MessageType._($core.int v, $core.String n) : super(v, n);
}

class MessageDeliveryStatues extends $pb.ProtobufEnum {
  static const MessageDeliveryStatues UNKNOWN_MD = MessageDeliveryStatues._(0, 'UNKNOWN_MD');
  static const MessageDeliveryStatues NOT_ABLE = MessageDeliveryStatues._(6, 'NOT_ABLE');
  static const MessageDeliveryStatues FAILED = MessageDeliveryStatues._(7, 'FAILED');
  static const MessageDeliveryStatues SENDING = MessageDeliveryStatues._(1, 'SENDING');
  static const MessageDeliveryStatues SENT = MessageDeliveryStatues._(2, 'SENT');
  static const MessageDeliveryStatues DELIVERED = MessageDeliveryStatues._(3, 'DELIVERED');
  static const MessageDeliveryStatues SEEN = MessageDeliveryStatues._(4, 'SEEN');
  static const MessageDeliveryStatues CONSUMED = MessageDeliveryStatues._(5, 'CONSUMED');

  static const $core.List<MessageDeliveryStatues> values = <MessageDeliveryStatues> [
    UNKNOWN_MD,
    NOT_ABLE,
    FAILED,
    SENDING,
    SENT,
    DELIVERED,
    SEEN,
    CONSUMED,
  ];

  static final $core.Map<$core.int, MessageDeliveryStatues> _byValue = $pb.ProtobufEnum.initByValue(values);
  static MessageDeliveryStatues valueOf($core.int value) => _byValue[value];

  const MessageDeliveryStatues._($core.int v, $core.String n) : super(v, n);
}

class MessageLogType extends $pb.ProtobufEnum {
  static const MessageLogType USER_JOINED = MessageLogType._(0, 'USER_JOINED');
  static const MessageLogType USER_DELETED = MessageLogType._(1, 'USER_DELETED');
  static const MessageLogType ROOM_CREATED = MessageLogType._(2, 'ROOM_CREATED');
  static const MessageLogType MEMBER_ADDED = MessageLogType._(3, 'MEMBER_ADDED');
  static const MessageLogType MEMBER_KICKED = MessageLogType._(4, 'MEMBER_KICKED');
  static const MessageLogType MEMBER_LEFT = MessageLogType._(5, 'MEMBER_LEFT');
  static const MessageLogType ROOM_CONVERTED_TO_PUBLIC = MessageLogType._(6, 'ROOM_CONVERTED_TO_PUBLIC');
  static const MessageLogType ROOM_CONVERTED_TO_PRIVATE = MessageLogType._(7, 'ROOM_CONVERTED_TO_PRIVATE');
  static const MessageLogType MEMBER_JOINED_BY_INVITE_LINK = MessageLogType._(8, 'MEMBER_JOINED_BY_INVITE_LINK');
  static const MessageLogType ROOM_DELETED = MessageLogType._(9, 'ROOM_DELETED');
  static const MessageLogType MISSED_VOICE_CALL = MessageLogType._(10, 'MISSED_VOICE_CALL');
  static const MessageLogType MISSED_VIDEO_CALL = MessageLogType._(11, 'MISSED_VIDEO_CALL');
  static const MessageLogType MISSED_SCREEN_SHARE = MessageLogType._(12, 'MISSED_SCREEN_SHARE');
  static const MessageLogType MISSED_SECRET_CHAT = MessageLogType._(13, 'MISSED_SECRET_CHAT');
  static const MessageLogType PINNED_MESSAGE = MessageLogType._(14, 'PINNED_MESSAGE');

  static const $core.List<MessageLogType> values = <MessageLogType> [
    USER_JOINED,
    USER_DELETED,
    ROOM_CREATED,
    MEMBER_ADDED,
    MEMBER_KICKED,
    MEMBER_LEFT,
    ROOM_CONVERTED_TO_PUBLIC,
    ROOM_CONVERTED_TO_PRIVATE,
    MEMBER_JOINED_BY_INVITE_LINK,
    ROOM_DELETED,
    MISSED_VOICE_CALL,
    MISSED_VIDEO_CALL,
    MISSED_SCREEN_SHARE,
    MISSED_SECRET_CHAT,
    PINNED_MESSAGE,
  ];

  static final $core.Map<$core.int, MessageLogType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static MessageLogType valueOf($core.int value) => _byValue[value];

  const MessageLogType._($core.int v, $core.String n) : super(v, n);
}

class ChannelType extends $pb.ProtobufEnum {
  static const ChannelType CHANNEL_UNKNOWN = ChannelType._(0, 'CHANNEL_UNKNOWN');
  static const ChannelType CHANNEL_PRIMARY = ChannelType._(1, 'CHANNEL_PRIMARY');
  static const ChannelType CHANNEL_NORMAL = ChannelType._(2, 'CHANNEL_NORMAL');
  static const ChannelType CHANNEL_SAVES = ChannelType._(3, 'CHANNEL_SAVES');
  static const ChannelType CHANNEL_STORE = ChannelType._(4, 'CHANNEL_STORE');

  static const $core.List<ChannelType> values = <ChannelType> [
    CHANNEL_UNKNOWN,
    CHANNEL_PRIMARY,
    CHANNEL_NORMAL,
    CHANNEL_SAVES,
    CHANNEL_STORE,
  ];

  static final $core.Map<$core.int, ChannelType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static ChannelType valueOf($core.int value) => _byValue[value];

  const ChannelType._($core.int v, $core.String n) : super(v, n);
}

class ChannelPrivacy extends $pb.ProtobufEnum {
  static const ChannelPrivacy CHANNEL_UNKNOWN_AB = ChannelPrivacy._(0, 'CHANNEL_UNKNOWN_AB');
  static const ChannelPrivacy CHANNEL_OPEN = ChannelPrivacy._(1, 'CHANNEL_OPEN');
  static const ChannelPrivacy CHANNEL_PRIVATE_LINK = ChannelPrivacy._(2, 'CHANNEL_PRIVATE_LINK');
  static const ChannelPrivacy CHANNEL_CREATOR = ChannelPrivacy._(3, 'CHANNEL_CREATOR');

  static const $core.List<ChannelPrivacy> values = <ChannelPrivacy> [
    CHANNEL_UNKNOWN_AB,
    CHANNEL_OPEN,
    CHANNEL_PRIVATE_LINK,
    CHANNEL_CREATOR,
  ];

  static final $core.Map<$core.int, ChannelPrivacy> _byValue = $pb.ProtobufEnum.initByValue(values);
  static ChannelPrivacy valueOf($core.int value) => _byValue[value];

  const ChannelPrivacy._($core.int v, $core.String n) : super(v, n);
}

class Cast extends $pb.ProtobufEnum {
  static const Cast UNKNOWN = Cast._(0, 'UNKNOWN');
  static const Cast USER = Cast._(1, 'USER');
  static const Cast CHANNEL = Cast._(2, 'CHANNEL');
  static const Cast GROUP = Cast._(3, 'GROUP');
  static const Cast BOT = Cast._(4, 'BOT');
  static const Cast STORE = Cast._(6, 'STORE');
  static const Cast TAG = Cast._(7, 'TAG');

  static const $core.List<Cast> values = <Cast> [
    UNKNOWN,
    USER,
    CHANNEL,
    GROUP,
    BOT,
    STORE,
    TAG,
  ];

  static final $core.Map<$core.int, Cast> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Cast valueOf($core.int value) => _byValue[value];

  const Cast._($core.int v, $core.String n) : super(v, n);
}

class ReactionType extends $pb.ProtobufEnum {
  static const ReactionType NONE = ReactionType._(0, 'NONE');
  static const ReactionType LIKE = ReactionType._(1, 'LIKE');

  static const $core.List<ReactionType> values = <ReactionType> [
    NONE,
    LIKE,
  ];

  static final $core.Map<$core.int, ReactionType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static ReactionType valueOf($core.int value) => _byValue[value];

  const ReactionType._($core.int v, $core.String n) : super(v, n);
}

class SessionPlatform extends $pb.ProtobufEnum {
  static const SessionPlatform UNKNOWN_PLATFORM = SessionPlatform._(0, 'UNKNOWN_PLATFORM');
  static const SessionPlatform ANDROID = SessionPlatform._(1, 'ANDROID');
  static const SessionPlatform IOS = SessionPlatform._(2, 'IOS');
  static const SessionPlatform MAC_OS = SessionPlatform._(3, 'MAC_OS');
  static const SessionPlatform WINDOWS = SessionPlatform._(4, 'WINDOWS');
  static const SessionPlatform LINUX = SessionPlatform._(5, 'LINUX');
  static const SessionPlatform BLACK_BERRY = SessionPlatform._(6, 'BLACK_BERRY');
  static const SessionPlatform Web = SessionPlatform._(7, 'Web');

  static const $core.List<SessionPlatform> values = <SessionPlatform> [
    UNKNOWN_PLATFORM,
    ANDROID,
    IOS,
    MAC_OS,
    WINDOWS,
    LINUX,
    BLACK_BERRY,
    Web,
  ];

  static final $core.Map<$core.int, SessionPlatform> _byValue = $pb.ProtobufEnum.initByValue(values);
  static SessionPlatform valueOf($core.int value) => _byValue[value];

  const SessionPlatform._($core.int v, $core.String n) : super(v, n);
}

class GroupPrivacy extends $pb.ProtobufEnum {
  static const GroupPrivacy GROUP_UNKNOWN_GP = GroupPrivacy._(0, 'GROUP_UNKNOWN_GP');
  static const GroupPrivacy GROUP_OPEN = GroupPrivacy._(1, 'GROUP_OPEN');
  static const GroupPrivacy GROUP_PRIVATE_LINK = GroupPrivacy._(2, 'GROUP_PRIVATE_LINK');

  static const $core.List<GroupPrivacy> values = <GroupPrivacy> [
    GROUP_UNKNOWN_GP,
    GROUP_OPEN,
    GROUP_PRIVATE_LINK,
  ];

  static final $core.Map<$core.int, GroupPrivacy> _byValue = $pb.ProtobufEnum.initByValue(values);
  static GroupPrivacy valueOf($core.int value) => _byValue[value];

  const GroupPrivacy._($core.int v, $core.String n) : super(v, n);
}

class GroupMemberRole extends $pb.ProtobufEnum {
  static const GroupMemberRole MEMBER_Unknown = GroupMemberRole._(0, 'MEMBER_Unknown');
  static const GroupMemberRole MEMBER_CREATOR = GroupMemberRole._(1, 'MEMBER_CREATOR');
  static const GroupMemberRole MEMBER_MODERATOR = GroupMemberRole._(3, 'MEMBER_MODERATOR');
  static const GroupMemberRole MEMBER_NORMAL_USER = GroupMemberRole._(2, 'MEMBER_NORMAL_USER');

  static const $core.List<GroupMemberRole> values = <GroupMemberRole> [
    MEMBER_Unknown,
    MEMBER_CREATOR,
    MEMBER_MODERATOR,
    MEMBER_NORMAL_USER,
  ];

  static final $core.Map<$core.int, GroupMemberRole> _byValue = $pb.ProtobufEnum.initByValue(values);
  static GroupMemberRole valueOf($core.int value) => _byValue[value];

  const GroupMemberRole._($core.int v, $core.String n) : super(v, n);
}

class Action_ActionType extends $pb.ProtobufEnum {
  static const Action_ActionType UNKNOWN_AT = Action_ActionType._(0, 'UNKNOWN_AT');
  static const Action_ActionType LIKED = Action_ActionType._(1, 'LIKED');
  static const Action_ActionType FOLLOWED = Action_ActionType._(2, 'FOLLOWED');

  static const $core.List<Action_ActionType> values = <Action_ActionType> [
    UNKNOWN_AT,
    LIKED,
    FOLLOWED,
  ];

  static final $core.Map<$core.int, Action_ActionType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Action_ActionType valueOf($core.int value) => _byValue[value];

  const Action_ActionType._($core.int v, $core.String n) : super(v, n);
}

