use crate::{com, com::*, pb, sms_sender};


pub async fn GetUsers1(up: &UserParam, param: pb::GetUsers1Param) -> Result<pb::GetUsers1Response, GenErr> {
    Ok(pb::GetUsers1Response::default())
    /*Ok(pb::GetUsers1Response{
        users
    })*/
}

