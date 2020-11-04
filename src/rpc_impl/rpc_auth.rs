use crate::{com, com::*, pb, sms_sender};


pub async fn SendConfirmCode(
    up: &UserParam,
    param: pb::SendConfirmCodeParam,
) -> Result<pb::SendConfirmCodeResponse, GenErr> {
    println!("called sms sender");
    // sms_sender::send().await;
    sms_sender::send_confirm_sms("09015132328",4).await;
    Ok(pb::SendConfirmCodeResponse::default())
}
