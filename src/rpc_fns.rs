use crate::rpc_fns_default as def;

use crate::{com, com::*, pb, rpc_impl};

pub use def::RPC_Account::*;
pub use def::RPC_Auth::*;
pub use def::RPC_Channel::*;
pub use def::RPC_Chat::*;
pub use def::RPC_Direct::*;
pub use def::RPC_General::*;
pub use def::RPC_Group::*;
pub use def::RPC_Social::*;
pub use def::RPC_Upload::*;

pub fn AddComment2(
    up: &UserParam,
    param: pb::pb2::AddCommentParam,
) -> Result<pb::pb2::AddCommentResponse, GenErr> {
    println!("inside add comment {:?}", param);
    Ok(pb::pb2::AddCommentResponse::default())
}

pub use rpc_impl::rpc_auth::SendConfirmCode;
