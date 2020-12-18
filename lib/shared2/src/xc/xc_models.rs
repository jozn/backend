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
struct Tweet {
	pub tweet_id: String,   // tweet_id    clustering  0
	pub user_id: i64,   // user_id    partition_key  0
	pub body: String,   // body    regular  -1
	pub create_time: i32,   // create_time    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.Tweet {
	tweet_id: "".to_string(),
	user_id: 0i64,
	body: "".to_string(),
	create_time: 0i32,
*/
#[derive(Clone, Debug, PartialEq)]
struct User {
	pub user_id: i32,   // user_id    partition_key  0
	pub created_time: i64,   // created_time    regular  -1
	pub full_name: String,   // full_name    regular  -1
	pub user_name: String,   // user_name    regular  -1
	
	_exists: bool,
	_deleted: bool,
}
/*
:= &xc.User {
	user_id: 0i32,
	created_time: 0i64,
	full_name: "".to_string(),
	user_name: "".to_string(),
*/


/*
// logs tables
type LogTableCql struct{
    
    Tweet bool
    User bool
}

var LogTableCqlReq = LogTableCql{
    Tweet: true ,
    User: true ,
}

*/
