use crate::{com, com::*, pb, sms_sender};


pub async fn SendConfirmCode(
    up: &UserParam,
    param: pb::SendConfirmCodeParam,
) -> Result<pb::SendConfirmCodeResponse, GenErr> {
    println!("called sms sender");
    sms_sender::send().await;
    Ok(pb::SendConfirmCodeResponse::default())
}
