use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::{common, my, pb, rpc2};

use crate::UserSpace;

#[async_trait]
impl rpc2::RPC_Auth_Handler2 for UserSpace {
    async fn AuthSendCode(
        &self,
        param: pb::AuthSendCodeParam,
    ) -> Result<pb::AuthSendCodeResponse, GenErr> {
        let phone = format!("0{}", &param.phone_number);

        let sms_code = shared::sms_sender::send_login_code_sms(&phone, 4)
            .await
            .unwrap();

        let hash_code = shared::utils::rand::rand_string(15);

        let sms_pb = pb::Sms {
            install_uuid: "".to_string(),
            phone_number: phone.clone(),
            country_code: param.country_code,
            for_login: true,
            hash_code: hash_code.clone(),
            confirm_code: sms_code,
            gateway_number: "".to_string(),
            text_body: "".to_string(),
            created_time: shared::utils::time::get_time_now_sec(),
            gateway_error: "".to_string(), //todo
            intent: "login".to_string(),
            result: "ok".to_string(),
        };

        let buff = shared::common::prost_encode(&sms_pb)?;
        let json_pb = format!("{:#?}", sms_pb);

        let sms_row = shared::my::Sms {
            sms_id: 0,
            phone_number: phone.clone(),
            hash_code: hash_code.clone(),
            result_code: 200,
            pd_data: buff,
            debug_data: json_pb,
        };
        sms_row.insert(&self.mysql_pool).await;

        let res = pb::AuthSendCodeResponse {
            hash_code: hash_code.clone(),
            phone_registered: false,
        };

        Ok(res)
    }

    async fn AuthLogIn(&self, param: pb::AuthLogInParam) -> Result<pb::AuthLogInResponse, GenErr> {
        let s = my::SmsSelector::new()
            .hash_code_eq(&param.hash_code)
            .get_row(&self.mysql_pool)
            .await?;

        let phone = format!("98{}", param.phone_number);

        if s.phone_number != format!("0{}", param.phone_number) {
            return Err(GenErr::NotFound);
        }

        let sms_pb: pb::Sms = common::prost_decode(&s.pd_data)?;
        if sms_pb.confirm_code != param.confirm_code {
            return Err(GenErr::WrongParam);
        }

        // todo extract this
        let user_row = my::UserSelector::new()
            .phone_number_eq(&phone)
            .get_row(&self.mysql_pool)
            .await?;

        let user_pb: pb::User = common::prost_decode(&user_row.pb_data)?;

        // todo session after code extraction

        Ok(pb::AuthLogInResponse {
            user: Some(user_pb),
            session: None, // todo
        })
    }

    // Not now > maybe never
    async fn AuthLogOut(
        &self,
        param: pb::AuthLogOutParam,
    ) -> Result<pb::AuthLogOutResponse, GenErr> {
        Ok(pb::AuthLogOutResponse::default())
    }
}
