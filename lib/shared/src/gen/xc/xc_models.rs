use cdrs::query::*;

use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;



/*#[derive(Clone, Debug, PartialEq)]
struct RowStruct {
    key: i32,
    user: User,
    map: HashMap<String, User>,
    list: Vec<User>,
}
*/
#[derive(Clone, Debug, PartialEq)]
struct Channel {
	pub channel_cid: i64,   // channel_cid    partition_key  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.Channel {
	channel_cid: 0i64,
	pb_data: vec![],
*/
#[derive(Clone, Debug, PartialEq)]
struct ChannelFollower {
	pub follow_gid: i64,   // follow_gid    clustering  1
	pub channel_cid: i64,   // channel_cid    partition_key  0
	pub profile_cid: i64,   // profile_cid    clustering  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ChannelFollower {
	follow_gid: 0i64,
	channel_cid: 0i64,
	profile_cid: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct ChannelFollowerBk {
	pub channel_cid: i64,   // channel_cid    partition_key  0
	pub profile_cid: i64,   // profile_cid    clustering  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ChannelFollowerBk {
	channel_cid: 0i64,
	profile_cid: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct ChannelFollowerSort {
	pub profile_cid: i64,   // profile_cid    clustering  1
	pub channel_cid: i64,   // channel_cid    partition_key  0
	pub follow_gid: i64,   // follow_gid    clustering  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ChannelFollowerSort {
	profile_cid: 0i64,
	channel_cid: 0i64,
	follow_gid: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct ChannelMsg {
	pub channel_cid: i64,   // channel_cid    partition_key  0
	pub msg_gid: i64,   // msg_gid    clustering  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ChannelMsg {
	channel_cid: 0i64,
	msg_gid: 0i64,
	pb_data: vec![],
*/
#[derive(Clone, Debug, PartialEq)]
struct ChannelMsgMedia {
	pub msg_gid: i64,   // msg_gid    clustering  1
	pub channel_cid: i64,   // channel_cid    partition_key  0
	pub media_type_id: i64,   // media_type_id    clustering  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ChannelMsgMedia {
	msg_gid: 0i64,
	channel_cid: 0i64,
	media_type_id: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct ChannelSubscriber {
	pub subscriber_gid: i64,   // subscriber_gid    clustering  1
	pub channel_cid: i64,   // channel_cid    partition_key  0
	pub profile_cid: i64,   // profile_cid    clustering  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ChannelSubscriber {
	subscriber_gid: 0i64,
	channel_cid: 0i64,
	profile_cid: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct ChannelSubscriberBk {
	pub channel_cid: i64,   // channel_cid    partition_key  0
	pub profile_cid: i64,   // profile_cid    clustering  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ChannelSubscriberBk {
	channel_cid: 0i64,
	profile_cid: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct ChannelSubscriberSort {
	pub profile_cid: i64,   // profile_cid    clustering  1
	pub channel_cid: i64,   // channel_cid    partition_key  0
	pub subscriber_gid: i64,   // subscriber_gid    clustering  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ChannelSubscriberSort {
	profile_cid: 0i64,
	channel_cid: 0i64,
	subscriber_gid: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct Chat {
	pub chat_gid: i64,   // chat_gid    clustering  0
	pub profile_cid: i64,   // profile_cid    partition_key  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.Chat {
	chat_gid: 0i64,
	profile_cid: 0i64,
	pb_data: vec![],
*/
#[derive(Clone, Debug, PartialEq)]
struct ChatMsg {
	pub chat_gid: i64,   // chat_gid    partition_key  1
	pub msg_gid: i64,   // msg_gid    clustering  0
	pub profile_cid: i64,   // profile_cid    partition_key  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ChatMsg {
	chat_gid: 0i64,
	msg_gid: 0i64,
	profile_cid: 0i64,
	pb_data: vec![],
*/
#[derive(Clone, Debug, PartialEq)]
struct Direct {
	pub direct_gid: i64,   // direct_gid    clustering  0
	pub profile_cid: i64,   // profile_cid    partition_key  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.Direct {
	direct_gid: 0i64,
	profile_cid: 0i64,
	pb_data: vec![],
*/
#[derive(Clone, Debug, PartialEq)]
struct File {
	pub file_gid: i64,   // file_gid    partition_key  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.File {
	file_gid: 0i64,
	pb_data: vec![],
*/
#[derive(Clone, Debug, PartialEq)]
struct FileRef {
	pub file_gid: i64,   // file_gid    partition_key  0
	pub ref_id: i64,   // ref_id    clustering  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.FileRef {
	file_gid: 0i64,
	ref_id: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct MsgComment {
	pub comment_gid: i64,   // comment_gid    clustering  0
	pub message_gid: i64,   // message_gid    partition_key  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.MsgComment {
	comment_gid: 0i64,
	message_gid: 0i64,
	pb_data: vec![],
*/
#[derive(Clone, Debug, PartialEq)]
struct MsgLike {
	pub message_gid: i64,   // message_gid    partition_key  0
	pub profile_cid: i64,   // profile_cid    clustering  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.MsgLike {
	message_gid: 0i64,
	profile_cid: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct MsgReshare {
	pub message_gid: i64,   // message_gid    partition_key  0
	pub profile_cid: i64,   // profile_cid    clustering  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.MsgReshare {
	message_gid: 0i64,
	profile_cid: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct Profile {
	pub profile_cid: i64,   // profile_cid    partition_key  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.Profile {
	profile_cid: 0i64,
	pb_data: vec![],
*/
#[derive(Clone, Debug, PartialEq)]
struct ProfileFollower {
	pub follow_gid: i64,   // follow_gid    clustering  1
	pub channel_cid: i64,   // channel_cid    clustering  0
	pub profile_cid: i64,   // profile_cid    partition_key  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ProfileFollower {
	follow_gid: 0i64,
	channel_cid: 0i64,
	profile_cid: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct ProfileFollowerSort {
	pub channel_cid: i64,   // channel_cid    clustering  1
	pub follow_gid: i64,   // follow_gid    clustering  0
	pub profile_cid: i64,   // profile_cid    partition_key  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ProfileFollowerSort {
	channel_cid: 0i64,
	follow_gid: 0i64,
	profile_cid: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct ProfileSubscriber {
	pub subscriber_gid: i64,   // subscriber_gid    clustering  1
	pub channel_cid: i64,   // channel_cid    clustering  0
	pub profile_cid: i64,   // profile_cid    partition_key  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ProfileSubscriber {
	subscriber_gid: 0i64,
	channel_cid: 0i64,
	profile_cid: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct ProfileSubscriberSort {
	pub channel_cid: i64,   // channel_cid    clustering  1
	pub profile_cid: i64,   // profile_cid    partition_key  0
	pub subscriber_gid: i64,   // subscriber_gid    clustering  0
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ProfileSubscriberSort {
	channel_cid: 0i64,
	profile_cid: 0i64,
	subscriber_gid: 0i64,
*/
#[derive(Clone, Debug, PartialEq)]
struct User {
	pub user_cid: i64,   // user_cid    partition_key  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.User {
	user_cid: 0i64,
	pb_data: vec![],
*/


/*
// logs tables
type LogTableCql struct{
    
    Channel bool
    ChannelFollower bool
    ChannelFollowerBk bool
    ChannelFollowerSort bool
    ChannelMsg bool
    ChannelMsgMedia bool
    ChannelSubscriber bool
    ChannelSubscriberBk bool
    ChannelSubscriberSort bool
    Chat bool
    ChatMsg bool
    Direct bool
    File bool
    FileRef bool
    MsgComment bool
    MsgLike bool
    MsgReshare bool
    Profile bool
    ProfileFollower bool
    ProfileFollowerSort bool
    ProfileSubscriber bool
    ProfileSubscriberSort bool
    User bool
}

var LogTableCqlReq = LogTableCql{
    Channel: true ,
    ChannelFollower: true ,
    ChannelFollowerBk: true ,
    ChannelFollowerSort: true ,
    ChannelMsg: true ,
    ChannelMsgMedia: true ,
    ChannelSubscriber: true ,
    ChannelSubscriberBk: true ,
    ChannelSubscriberSort: true ,
    Chat: true ,
    ChatMsg: true ,
    Direct: true ,
    File: true ,
    FileRef: true ,
    MsgComment: true ,
    MsgLike: true ,
    MsgReshare: true ,
    Profile: true ,
    ProfileFollower: true ,
    ProfileFollowerSort: true ,
    ProfileSubscriber: true ,
    ProfileSubscriberSort: true ,
    User: true ,
}

*/
