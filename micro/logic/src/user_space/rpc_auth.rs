use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::{pb, rpc2};

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

        let hash_code = shared::utils::rand::rand_string(20);

        let sms_pb = pb::Sms {
            gid: 0,
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
        let json_pb = format!("{:?}", sms_pb);

        let sms_row = shared::my::Sms {
            sms_id: 0,
            phone_number: phone.clone(),
            hash_code: hash_code.clone(),
            result_code: 200,
            pd_data: buff,
            json_data: json_pb,
        };
        sms_row.insert(&self.mysql_pool).await;

        let res = pb::AuthSendCodeResponse {
            hash_code: hash_code.clone(),
            phone_registered: false,
        };

        Ok(res)
    }

    async fn AuthLogIn(&self, param: pb::AuthLogInParam) -> Result<pb::AuthLogInResponse, GenErr> {
        Ok(pb::AuthLogInResponse::default())
    }

    async fn AuthLogOut(
        &self,
        param: pb::AuthLogOutParam,
    ) -> Result<pb::AuthLogOutResponse, GenErr> {
        Ok(pb::AuthLogOutResponse::default())
    }
}
