use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::{pb, rpc2};

use crate::UserSpace;

#[async_trait]
impl rpc2::RPC_Auth_Handler2 for UserSpace {
    async fn AuthSendConfirmCode(
        &self,
        param: pb::AuthSendConfirmCodeParam,
    ) -> Result<pb::AuthSendConfirmCodeResponse, GenErr> {
        let phone = format!("0{}", &param.phone);
        /*        shared::sms_sender::send_confirm_sms(&phone, 4)
        .await
        .unwrap();*/
        let r = pb::AuthSendConfirmCodeResponse {
            done: true,
            error_message: "".to_string(),
            just_email_register: false,
            sms_numbers: vec![],
            is_login: false,
        };
        let m = shared::my::Sms {
            sms_id: 0,
            result_code: 23,
            pd_data: vec![],
            json_data: "ffsdf".to_string(),
        };

        let database_url = "flipper:12345678@tcp(192.168.43.116:3306)/flip_my?charset=utf8mb4";

        let pool = mysql_async::Pool::new(database_url);
        //let mut conn = pool.get_conn().await.unwrap();

        m.insert(&pool);
        Ok(r)
    }
    async fn AuthConfirmCode(
        &self,
        param: pb::AuthConfirmCodeParam,
    ) -> Result<pb::AuthConfirmCodeResponse, GenErr> {
        Ok(pb::AuthConfirmCodeResponse::default())
    }
    async fn AuthSingUp(
        &self,
        param: pb::AuthSingUpParam,
    ) -> Result<pb::AuthSingUpResponse, GenErr> {
        Ok(pb::AuthSingUpResponse::default())
    }
    async fn AuthSingIn(
        &self,
        param: pb::AuthSingInParam,
    ) -> Result<pb::AuthSingInResponse, GenErr> {
        Ok(pb::AuthSingInResponse::default())
    }
    async fn AuthLogOut(
        &self,
        param: pb::AuthLogOutParam,
    ) -> Result<pb::AuthLogOutResponse, GenErr> {
        Ok(pb::AuthLogOutResponse::default())
    }
}
