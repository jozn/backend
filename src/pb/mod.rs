pub mod ps;

/// This module is a convienet mod for exporting all protos in one place; so they can be more easily
/// accessible while still respecting diffrent proto files and sub-modules.

/// gen in ./proto dir: pb-rs --dont_use_cow --custom_struct_derive "Serialize, Deserialize" -d ../pb *.proto

pub use ps::*;
// pub use pb::*;

// pub use pb::global::*;
// pub use pb::rpc_general::*;

// legacy ones
/*pub use pb::pb_changes::*;
pub use pb::pb_enum::*;
pub use pb::pb_global::*;
pub use pb::pb_rpc_auth2::*;
pub use pb::pb_rpc_chat::*;
pub use pb::pb_rpc_general::*;
pub use pb::pb_rpc_page::*;
pub use pb::pb_rpc_social::*;
pub use pb::pb_rpc_user::*;
pub use pb::pb_tables::*;
pub use pb::pb_views::*;
pub use pb::proto::*;

pub use mod_RPC_Page_Types::*;
pub use mod_RPC_Auth_Types::*;
pub use mod_RPC_General_Types::*;
pub use mod_RPC_Social_Types::*;
pub use mod_RPC_User_Types::*;
pub use mod_PB_RPC_Chat_Types::*;*/
