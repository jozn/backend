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
	pub channel_id: i64,   // channel_id    partition_key  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.Channel {
	channel_id: 0i64,
	pb_data: vec![],
*/
#[derive(Clone, Debug, PartialEq)]
struct ChannelMsg {
	pub channel_id: i64,   // channel_id    partition_key  0
	pub msg_id: i64,   // msg_id    clustering  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ChannelMsg {
	channel_id: 0i64,
	msg_id: 0i64,
	pb_data: vec![],
*/
#[derive(Clone, Debug, PartialEq)]
struct Chat {
	pub chat_id: i64,   // chat_id    clustering  0
	pub profile_cid: i64,   // profile_cid    partition_key  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.Chat {
	chat_id: 0i64,
	profile_cid: 0i64,
	pb_data: vec![],
*/
#[derive(Clone, Debug, PartialEq)]
struct ChatMsg {
	pub chat_id: i64,   // chat_id    partition_key  0
	pub msg_id: i64,   // msg_id    clustering  0
	pub pb_data: Vec<u8>,   // pb_data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.ChatMsg {
	chat_id: 0i64,
	msg_id: 0i64,
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
struct Messages {
	pub chat_id: i64,   // chat_id    partition_key  0
	pub data: Vec<u8>,   // data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.Messages {
	chat_id: 0i64,
	data: vec![],
*/
#[derive(Clone, Debug, PartialEq)]
struct Messages12 {
	pub chat_id: i64,   // chat_id    partition_key  0
	pub data: Vec<u8>,   // data    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.Messages12 {
	chat_id: 0i64,
	data: vec![],
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
    ChannelMsg bool
    Chat bool
    ChatMsg bool
    File bool
    FileRef bool
    Messages bool
    Messages12 bool
    Profile bool
    User bool
}

var LogTableCqlReq = LogTableCql{
    Channel: true ,
    ChannelMsg: true ,
    Chat: true ,
    ChatMsg: true ,
    File: true ,
    FileRef: true ,
    Messages: true ,
    Messages12: true ,
    Profile: true ,
    User: true ,
}

*/
