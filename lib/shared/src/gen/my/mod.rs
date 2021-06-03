pub mod ch2;
pub mod channel;
pub mod channel_follower;
pub mod channel_msg;
pub mod channel_msg_like;
pub mod channel_msg_media;
pub mod channel_subscriber;
pub mod chat;
pub mod chat_msg;
pub mod direct;
pub mod file;
pub mod file_ref;
pub mod gen_cid;
pub mod msg_comment;
pub mod msg_like2;
pub mod msg_reshare;
pub mod profile;
pub mod profile_follower;
pub mod profile_subscriber;
pub mod session;
pub mod sms;
pub mod sms_bk;
pub mod tweet;
pub mod user;


pub use ch2::*;
pub use channel::*;
pub use channel_follower::*;
pub use channel_msg::*;
pub use channel_msg_like::*;
pub use channel_msg_media::*;
pub use channel_subscriber::*;
pub use chat::*;
pub use chat_msg::*;
pub use direct::*;
pub use file::*;
pub use file_ref::*;
pub use gen_cid::*;
pub use msg_comment::*;
pub use msg_like2::*;
pub use msg_reshare::*;
pub use profile::*;
pub use profile_follower::*;
pub use profile_subscriber::*;
pub use session::*;
pub use sms::*;
pub use sms_bk::*;
pub use tweet::*;
pub use user::*;

// Every Table Must Have Primary Keys to Be Included In This Output
// Primiay Keys must be one column (no compostion types yet)
// Primiay Keys can be 1) Auto Increment 2) Other self Inserted

// Implemention is simple NOT many features is suported in Rust version:
// Keep mysql data types in int, bigint, text, varchar, bool, blob
// No signed integer is supported
// For now Primary key should only be numbers
// Not fully ORM is supported: limited to CRUD on rows + Indexes querys