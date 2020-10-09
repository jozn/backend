

use crate::rpc_fns_default as def;

use crate::{com, com::*, pb};

pub use def::RPC_Account::*;
pub use def::RPC_Auth::*;
pub use def::RPC_Channel::*;
pub use def::RPC_Chat::*;
pub use def::RPC_Direct::*;
pub use def::RPC_General::*;
pub use def::RPC_Group::*;
pub use def::RPC_Social::*;
pub use def::RPC_Upload::*;

pub fn ChangePhoneNumber(up: &UserParam, param: pb::ChangePhoneNumberParam) -> Result<pb::ChangePhoneNumberResponse, GenErr> {
    Ok(pb::ChangePhoneNumberResponse::default())
}

