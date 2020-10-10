///
//  Generated code. Do not modify.
//  source: enums.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

// ignore_for_file: UNDEFINED_SHOWN_NAME,UNUSED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class FollowingEnum extends $pb.ProtobufEnum {
  static const FollowingEnum FOLLOWING_NONE = FollowingEnum._(0, 'FOLLOWING_NONE');
  static const FollowingEnum FOLLOWING = FollowingEnum._(1, 'FOLLOWING');
  static const FollowingEnum REQUESTED = FollowingEnum._(2, 'REQUESTED');
  static const FollowingEnum BLOCKED = FollowingEnum._(3, 'BLOCKED');

  static const $core.List<FollowingEnum> values = <FollowingEnum> [
    FOLLOWING_NONE,
    FOLLOWING,
    REQUESTED,
    BLOCKED,
  ];

  static final $core.Map<$core.int, FollowingEnum> _byValue = $pb.ProtobufEnum.initByValue(values);
  static FollowingEnum valueOf($core.int value) => _byValue[value];

  const FollowingEnum._($core.int v, $core.String n) : super(v, n);
}

class UserLevelEnum extends $pb.ProtobufEnum {
  static const UserLevelEnum LEVEL_NORMAL = UserLevelEnum._(0, 'LEVEL_NORMAL');
  static const UserLevelEnum APP_ADMIN = UserLevelEnum._(1, 'APP_ADMIN');
  static const UserLevelEnum SUSPONDED = UserLevelEnum._(2, 'SUSPONDED');
  static const UserLevelEnum DELETED_BY_OWENER = UserLevelEnum._(3, 'DELETED_BY_OWENER');
  static const UserLevelEnum DELETED_IRAN = UserLevelEnum._(4, 'DELETED_IRAN');
  static const UserLevelEnum SUSPONDED_IRAN = UserLevelEnum._(5, 'SUSPONDED_IRAN');

  static const $core.List<UserLevelEnum> values = <UserLevelEnum> [
    LEVEL_NORMAL,
    APP_ADMIN,
    SUSPONDED,
    DELETED_BY_OWENER,
    DELETED_IRAN,
    SUSPONDED_IRAN,
  ];

  static final $core.Map<$core.int, UserLevelEnum> _byValue = $pb.ProtobufEnum.initByValue(values);
  static UserLevelEnum valueOf($core.int value) => _byValue[value];

  const UserLevelEnum._($core.int v, $core.String n) : super(v, n);
}

class GeneralPrivacyEnum extends $pb.ProtobufEnum {
  static const GeneralPrivacyEnum UNKNOWN_GENERAL_PRIVACY = GeneralPrivacyEnum._(0, 'UNKNOWN_GENERAL_PRIVACY');
  static const GeneralPrivacyEnum ALL_PEOPLE_PRIVACY = GeneralPrivacyEnum._(1, 'ALL_PEOPLE_PRIVACY');
  static const GeneralPrivacyEnum NOBODY_PRIVACY = GeneralPrivacyEnum._(2, 'NOBODY_PRIVACY');
  static const GeneralPrivacyEnum CONTACTS_ONLY_PRIVACY = GeneralPrivacyEnum._(3, 'CONTACTS_ONLY_PRIVACY');
  static const GeneralPrivacyEnum FOLLOWED_ONLY_PRIVACY = GeneralPrivacyEnum._(4, 'FOLLOWED_ONLY_PRIVACY');
  static const GeneralPrivacyEnum CONTACTS_AND_FOLLOWD_PRIVACY = GeneralPrivacyEnum._(5, 'CONTACTS_AND_FOLLOWD_PRIVACY');

  static const $core.List<GeneralPrivacyEnum> values = <GeneralPrivacyEnum> [
    UNKNOWN_GENERAL_PRIVACY,
    ALL_PEOPLE_PRIVACY,
    NOBODY_PRIVACY,
    CONTACTS_ONLY_PRIVACY,
    FOLLOWED_ONLY_PRIVACY,
    CONTACTS_AND_FOLLOWD_PRIVACY,
  ];

  static final $core.Map<$core.int, GeneralPrivacyEnum> _byValue = $pb.ProtobufEnum.initByValue(values);
  static GeneralPrivacyEnum valueOf($core.int value) => _byValue[value];

  const GeneralPrivacyEnum._($core.int v, $core.String n) : super(v, n);
}

class UserOnlineStatusEnum extends $pb.ProtobufEnum {
  static const UserOnlineStatusEnum EXACTLY = UserOnlineStatusEnum._(0, 'EXACTLY');
  static const UserOnlineStatusEnum ONLINE = UserOnlineStatusEnum._(1, 'ONLINE');
  static const UserOnlineStatusEnum CONNECTED = UserOnlineStatusEnum._(2, 'CONNECTED');
  static const UserOnlineStatusEnum FEW_DAYS_AGO = UserOnlineStatusEnum._(3, 'FEW_DAYS_AGO');
  static const UserOnlineStatusEnum RECENTLY = UserOnlineStatusEnum._(4, 'RECENTLY');
  static const UserOnlineStatusEnum LAST_WEEK = UserOnlineStatusEnum._(5, 'LAST_WEEK');
  static const UserOnlineStatusEnum LAST_MONTH = UserOnlineStatusEnum._(6, 'LAST_MONTH');
  static const UserOnlineStatusEnum LONG_TIME_AGO = UserOnlineStatusEnum._(7, 'LONG_TIME_AGO');
  static const UserOnlineStatusEnum HIDE = UserOnlineStatusEnum._(8, 'HIDE');

  static const $core.List<UserOnlineStatusEnum> values = <UserOnlineStatusEnum> [
    EXACTLY,
    ONLINE,
    CONNECTED,
    FEW_DAYS_AGO,
    RECENTLY,
    LAST_WEEK,
    LAST_MONTH,
    LONG_TIME_AGO,
    HIDE,
  ];

  static final $core.Map<$core.int, UserOnlineStatusEnum> _byValue = $pb.ProtobufEnum.initByValue(values);
  static UserOnlineStatusEnum valueOf($core.int value) => _byValue[value];

  const UserOnlineStatusEnum._($core.int v, $core.String n) : super(v, n);
}

class PostCategoryEnum extends $pb.ProtobufEnum {
  static const PostCategoryEnum PostCat_Text = PostCategoryEnum._(0, 'PostCat_Text');
  static const PostCategoryEnum PostCat_Media = PostCategoryEnum._(1, 'PostCat_Media');
  static const PostCategoryEnum PostCat_File = PostCategoryEnum._(2, 'PostCat_File');

  static const $core.List<PostCategoryEnum> values = <PostCategoryEnum> [
    PostCat_Text,
    PostCat_Media,
    PostCat_File,
  ];

  static final $core.Map<$core.int, PostCategoryEnum> _byValue = $pb.ProtobufEnum.initByValue(values);
  static PostCategoryEnum valueOf($core.int value) => _byValue[value];

  const PostCategoryEnum._($core.int v, $core.String n) : super(v, n);
}

class NotifyEnum extends $pb.ProtobufEnum {
  static const NotifyEnum NOTIFY_POST_LIKED = NotifyEnum._(0, 'NOTIFY_POST_LIKED');
  static const NotifyEnum NOTIFY_POST_COMMENTED = NotifyEnum._(1, 'NOTIFY_POST_COMMENTED');
  static const NotifyEnum NOTIFY_FOLLOWED_YOU = NotifyEnum._(2, 'NOTIFY_FOLLOWED_YOU');

  static const $core.List<NotifyEnum> values = <NotifyEnum> [
    NOTIFY_POST_LIKED,
    NOTIFY_POST_COMMENTED,
    NOTIFY_FOLLOWED_YOU,
  ];

  static final $core.Map<$core.int, NotifyEnum> _byValue = $pb.ProtobufEnum.initByValue(values);
  static NotifyEnum valueOf($core.int value) => _byValue[value];

  const NotifyEnum._($core.int v, $core.String n) : super(v, n);
}

class ActionEnum extends $pb.ProtobufEnum {
  static const ActionEnum ACTION_POST_LIKED = ActionEnum._(0, 'ACTION_POST_LIKED');
  static const ActionEnum ACTION_POST_COMMENTED = ActionEnum._(1, 'ACTION_POST_COMMENTED');
  static const ActionEnum ACTION_FOLLOWED_USER = ActionEnum._(2, 'ACTION_FOLLOWED_USER');

  static const $core.List<ActionEnum> values = <ActionEnum> [
    ACTION_POST_LIKED,
    ACTION_POST_COMMENTED,
    ACTION_FOLLOWED_USER,
  ];

  static final $core.Map<$core.int, ActionEnum> _byValue = $pb.ProtobufEnum.initByValue(values);
  static ActionEnum valueOf($core.int value) => _byValue[value];

  const ActionEnum._($core.int v, $core.String n) : super(v, n);
}

class RoomActionDoingEnum extends $pb.ProtobufEnum {
  static const RoomActionDoingEnum UNKNOWN_ROOM_ACTION_DOING = RoomActionDoingEnum._(0, 'UNKNOWN_ROOM_ACTION_DOING');
  static const RoomActionDoingEnum CANCEL = RoomActionDoingEnum._(1, 'CANCEL');
  static const RoomActionDoingEnum TYPING = RoomActionDoingEnum._(2, 'TYPING');
  static const RoomActionDoingEnum SENDING_IMAGE = RoomActionDoingEnum._(3, 'SENDING_IMAGE');
  static const RoomActionDoingEnum CAPTURING_IMAGE = RoomActionDoingEnum._(4, 'CAPTURING_IMAGE');
  static const RoomActionDoingEnum SENDING_VIDEO = RoomActionDoingEnum._(5, 'SENDING_VIDEO');
  static const RoomActionDoingEnum CAPTURING_VIDEO = RoomActionDoingEnum._(6, 'CAPTURING_VIDEO');
  static const RoomActionDoingEnum SENDING_AUDIO = RoomActionDoingEnum._(7, 'SENDING_AUDIO');
  static const RoomActionDoingEnum RECORDING_VOICE = RoomActionDoingEnum._(8, 'RECORDING_VOICE');
  static const RoomActionDoingEnum SENDING_VOICE = RoomActionDoingEnum._(9, 'SENDING_VOICE');
  static const RoomActionDoingEnum SENDING_DOCUMENT = RoomActionDoingEnum._(11, 'SENDING_DOCUMENT');
  static const RoomActionDoingEnum SENDING_GIF = RoomActionDoingEnum._(12, 'SENDING_GIF');
  static const RoomActionDoingEnum SENDING_FILE = RoomActionDoingEnum._(13, 'SENDING_FILE');
  static const RoomActionDoingEnum SENDING_LOCATION = RoomActionDoingEnum._(14, 'SENDING_LOCATION');
  static const RoomActionDoingEnum CHOOSING_CONTACT = RoomActionDoingEnum._(15, 'CHOOSING_CONTACT');
  static const RoomActionDoingEnum PAINTING = RoomActionDoingEnum._(16, 'PAINTING');

  static const $core.List<RoomActionDoingEnum> values = <RoomActionDoingEnum> [
    UNKNOWN_ROOM_ACTION_DOING,
    CANCEL,
    TYPING,
    SENDING_IMAGE,
    CAPTURING_IMAGE,
    SENDING_VIDEO,
    CAPTURING_VIDEO,
    SENDING_AUDIO,
    RECORDING_VOICE,
    SENDING_VOICE,
    SENDING_DOCUMENT,
    SENDING_GIF,
    SENDING_FILE,
    SENDING_LOCATION,
    CHOOSING_CONTACT,
    PAINTING,
  ];

  static final $core.Map<$core.int, RoomActionDoingEnum> _byValue = $pb.ProtobufEnum.initByValue(values);
  static RoomActionDoingEnum valueOf($core.int value) => _byValue[value];

  const RoomActionDoingEnum._($core.int v, $core.String n) : super(v, n);
}

