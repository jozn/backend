use crate::{com, com::*, pb};


pub fn SendConfirmCode(
    up: &UserParam,
    param: pb::SendConfirmCodeParam,
) -> Result<pb::SendConfirmCodeResponse, GenErr> {
    println!("called sms sender");
    Ok(pb::SendConfirmCodeResponse::default())
}
