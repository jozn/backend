// Automatically generated rust module for 'enums.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{BytesReader, Result, MessageRead, MessageWrite};
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FollowingEnum {
    FOLLOWING_NONE = 0,
    FOLLOWING = 1,
    REQUESTED = 2,
    BLOCKED = 3,
}

impl Default for FollowingEnum {
    fn default() -> Self {
        FollowingEnum::FOLLOWING_NONE
    }
}

impl From<i32> for FollowingEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => FollowingEnum::FOLLOWING_NONE,
            1 => FollowingEnum::FOLLOWING,
            2 => FollowingEnum::REQUESTED,
            3 => FollowingEnum::BLOCKED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for FollowingEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "FOLLOWING_NONE" => FollowingEnum::FOLLOWING_NONE,
            "FOLLOWING" => FollowingEnum::FOLLOWING,
            "REQUESTED" => FollowingEnum::REQUESTED,
            "BLOCKED" => FollowingEnum::BLOCKED,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UserLevelEnum {
    LEVEL_NORMAL = 0,
    APP_ADMIN = 1,
    SUSPONDED = 2,
    DELETED_BY_OWENER = 3,
    DELETED_IRAN = 4,
    SUSPONDED_IRAN = 5,
}

impl Default for UserLevelEnum {
    fn default() -> Self {
        UserLevelEnum::LEVEL_NORMAL
    }
}

impl From<i32> for UserLevelEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => UserLevelEnum::LEVEL_NORMAL,
            1 => UserLevelEnum::APP_ADMIN,
            2 => UserLevelEnum::SUSPONDED,
            3 => UserLevelEnum::DELETED_BY_OWENER,
            4 => UserLevelEnum::DELETED_IRAN,
            5 => UserLevelEnum::SUSPONDED_IRAN,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for UserLevelEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "LEVEL_NORMAL" => UserLevelEnum::LEVEL_NORMAL,
            "APP_ADMIN" => UserLevelEnum::APP_ADMIN,
            "SUSPONDED" => UserLevelEnum::SUSPONDED,
            "DELETED_BY_OWENER" => UserLevelEnum::DELETED_BY_OWENER,
            "DELETED_IRAN" => UserLevelEnum::DELETED_IRAN,
            "SUSPONDED_IRAN" => UserLevelEnum::SUSPONDED_IRAN,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GeneralPrivacyEnum {
    UNKNOWN_GENERAL_PRIVACY = 0,
    ALL_PEOPLE_PRIVACY = 1,
    NOBODY_PRIVACY = 2,
    CONTACTS_ONLY_PRIVACY = 3,
    FOLLOWED_ONLY_PRIVACY = 4,
    CONTACTS_AND_FOLLOWD_PRIVACY = 5,
}

impl Default for GeneralPrivacyEnum {
    fn default() -> Self {
        GeneralPrivacyEnum::UNKNOWN_GENERAL_PRIVACY
    }
}

impl From<i32> for GeneralPrivacyEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => GeneralPrivacyEnum::UNKNOWN_GENERAL_PRIVACY,
            1 => GeneralPrivacyEnum::ALL_PEOPLE_PRIVACY,
            2 => GeneralPrivacyEnum::NOBODY_PRIVACY,
            3 => GeneralPrivacyEnum::CONTACTS_ONLY_PRIVACY,
            4 => GeneralPrivacyEnum::FOLLOWED_ONLY_PRIVACY,
            5 => GeneralPrivacyEnum::CONTACTS_AND_FOLLOWD_PRIVACY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for GeneralPrivacyEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN_GENERAL_PRIVACY" => GeneralPrivacyEnum::UNKNOWN_GENERAL_PRIVACY,
            "ALL_PEOPLE_PRIVACY" => GeneralPrivacyEnum::ALL_PEOPLE_PRIVACY,
            "NOBODY_PRIVACY" => GeneralPrivacyEnum::NOBODY_PRIVACY,
            "CONTACTS_ONLY_PRIVACY" => GeneralPrivacyEnum::CONTACTS_ONLY_PRIVACY,
            "FOLLOWED_ONLY_PRIVACY" => GeneralPrivacyEnum::FOLLOWED_ONLY_PRIVACY,
            "CONTACTS_AND_FOLLOWD_PRIVACY" => GeneralPrivacyEnum::CONTACTS_AND_FOLLOWD_PRIVACY,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UserOnlineStatusEnum {
    EXACTLY = 0,
    ONLINE = 1,
    CONNECTED = 2,
    FEW_DAYS_AGO = 3,
    RECENTLY = 4,
    LAST_WEEK = 5,
    LAST_MONTH = 6,
    LONG_TIME_AGO = 7,
    HIDE = 8,
}

impl Default for UserOnlineStatusEnum {
    fn default() -> Self {
        UserOnlineStatusEnum::EXACTLY
    }
}

impl From<i32> for UserOnlineStatusEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => UserOnlineStatusEnum::EXACTLY,
            1 => UserOnlineStatusEnum::ONLINE,
            2 => UserOnlineStatusEnum::CONNECTED,
            3 => UserOnlineStatusEnum::FEW_DAYS_AGO,
            4 => UserOnlineStatusEnum::RECENTLY,
            5 => UserOnlineStatusEnum::LAST_WEEK,
            6 => UserOnlineStatusEnum::LAST_MONTH,
            7 => UserOnlineStatusEnum::LONG_TIME_AGO,
            8 => UserOnlineStatusEnum::HIDE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for UserOnlineStatusEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "EXACTLY" => UserOnlineStatusEnum::EXACTLY,
            "ONLINE" => UserOnlineStatusEnum::ONLINE,
            "CONNECTED" => UserOnlineStatusEnum::CONNECTED,
            "FEW_DAYS_AGO" => UserOnlineStatusEnum::FEW_DAYS_AGO,
            "RECENTLY" => UserOnlineStatusEnum::RECENTLY,
            "LAST_WEEK" => UserOnlineStatusEnum::LAST_WEEK,
            "LAST_MONTH" => UserOnlineStatusEnum::LAST_MONTH,
            "LONG_TIME_AGO" => UserOnlineStatusEnum::LONG_TIME_AGO,
            "HIDE" => UserOnlineStatusEnum::HIDE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PostCategoryEnum {
    PostCat_Text = 0,
    PostCat_Media = 1,
    PostCat_File = 2,
}

impl Default for PostCategoryEnum {
    fn default() -> Self {
        PostCategoryEnum::PostCat_Text
    }
}

impl From<i32> for PostCategoryEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => PostCategoryEnum::PostCat_Text,
            1 => PostCategoryEnum::PostCat_Media,
            2 => PostCategoryEnum::PostCat_File,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PostCategoryEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "PostCat_Text" => PostCategoryEnum::PostCat_Text,
            "PostCat_Media" => PostCategoryEnum::PostCat_Media,
            "PostCat_File" => PostCategoryEnum::PostCat_File,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NotifyEnum {
    NOTIFY_POST_LIKED = 0,
    NOTIFY_POST_COMMENTED = 1,
    NOTIFY_FOLLOWED_YOU = 2,
}

impl Default for NotifyEnum {
    fn default() -> Self {
        NotifyEnum::NOTIFY_POST_LIKED
    }
}

impl From<i32> for NotifyEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => NotifyEnum::NOTIFY_POST_LIKED,
            1 => NotifyEnum::NOTIFY_POST_COMMENTED,
            2 => NotifyEnum::NOTIFY_FOLLOWED_YOU,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for NotifyEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "NOTIFY_POST_LIKED" => NotifyEnum::NOTIFY_POST_LIKED,
            "NOTIFY_POST_COMMENTED" => NotifyEnum::NOTIFY_POST_COMMENTED,
            "NOTIFY_FOLLOWED_YOU" => NotifyEnum::NOTIFY_FOLLOWED_YOU,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ActionEnum {
    ACTION_POST_LIKED = 0,
    ACTION_POST_COMMENTED = 1,
    ACTION_FOLLOWED_USER = 2,
}

impl Default for ActionEnum {
    fn default() -> Self {
        ActionEnum::ACTION_POST_LIKED
    }
}

impl From<i32> for ActionEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => ActionEnum::ACTION_POST_LIKED,
            1 => ActionEnum::ACTION_POST_COMMENTED,
            2 => ActionEnum::ACTION_FOLLOWED_USER,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ActionEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "ACTION_POST_LIKED" => ActionEnum::ACTION_POST_LIKED,
            "ACTION_POST_COMMENTED" => ActionEnum::ACTION_POST_COMMENTED,
            "ACTION_FOLLOWED_USER" => ActionEnum::ACTION_FOLLOWED_USER,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RoomActionDoingEnum {
    UNKNOWN_ROOM_ACTION_DOING = 0,
    CANCEL = 1,
    TYPING = 2,
    SENDING_IMAGE = 3,
    CAPTURING_IMAGE = 4,
    SENDING_VIDEO = 5,
    CAPTURING_VIDEO = 6,
    SENDING_AUDIO = 7,
    RECORDING_VOICE = 8,
    SENDING_VOICE = 9,
    SENDING_DOCUMENT = 11,
    SENDING_GIF = 12,
    SENDING_FILE = 13,
    SENDING_LOCATION = 14,
    CHOOSING_CONTACT = 15,
    PAINTING = 16,
}

impl Default for RoomActionDoingEnum {
    fn default() -> Self {
        RoomActionDoingEnum::UNKNOWN_ROOM_ACTION_DOING
    }
}

impl From<i32> for RoomActionDoingEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => RoomActionDoingEnum::UNKNOWN_ROOM_ACTION_DOING,
            1 => RoomActionDoingEnum::CANCEL,
            2 => RoomActionDoingEnum::TYPING,
            3 => RoomActionDoingEnum::SENDING_IMAGE,
            4 => RoomActionDoingEnum::CAPTURING_IMAGE,
            5 => RoomActionDoingEnum::SENDING_VIDEO,
            6 => RoomActionDoingEnum::CAPTURING_VIDEO,
            7 => RoomActionDoingEnum::SENDING_AUDIO,
            8 => RoomActionDoingEnum::RECORDING_VOICE,
            9 => RoomActionDoingEnum::SENDING_VOICE,
            11 => RoomActionDoingEnum::SENDING_DOCUMENT,
            12 => RoomActionDoingEnum::SENDING_GIF,
            13 => RoomActionDoingEnum::SENDING_FILE,
            14 => RoomActionDoingEnum::SENDING_LOCATION,
            15 => RoomActionDoingEnum::CHOOSING_CONTACT,
            16 => RoomActionDoingEnum::PAINTING,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for RoomActionDoingEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN_ROOM_ACTION_DOING" => RoomActionDoingEnum::UNKNOWN_ROOM_ACTION_DOING,
            "CANCEL" => RoomActionDoingEnum::CANCEL,
            "TYPING" => RoomActionDoingEnum::TYPING,
            "SENDING_IMAGE" => RoomActionDoingEnum::SENDING_IMAGE,
            "CAPTURING_IMAGE" => RoomActionDoingEnum::CAPTURING_IMAGE,
            "SENDING_VIDEO" => RoomActionDoingEnum::SENDING_VIDEO,
            "CAPTURING_VIDEO" => RoomActionDoingEnum::CAPTURING_VIDEO,
            "SENDING_AUDIO" => RoomActionDoingEnum::SENDING_AUDIO,
            "RECORDING_VOICE" => RoomActionDoingEnum::RECORDING_VOICE,
            "SENDING_VOICE" => RoomActionDoingEnum::SENDING_VOICE,
            "SENDING_DOCUMENT" => RoomActionDoingEnum::SENDING_DOCUMENT,
            "SENDING_GIF" => RoomActionDoingEnum::SENDING_GIF,
            "SENDING_FILE" => RoomActionDoingEnum::SENDING_FILE,
            "SENDING_LOCATION" => RoomActionDoingEnum::SENDING_LOCATION,
            "CHOOSING_CONTACT" => RoomActionDoingEnum::CHOOSING_CONTACT,
            "PAINTING" => RoomActionDoingEnum::PAINTING,
            _ => Self::default(),
        }
    }
}

