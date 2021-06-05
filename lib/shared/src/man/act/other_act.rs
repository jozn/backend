use crate::{
    act,
    db_helper,
    errors::GenErr,
    pb, sms_sender,
    utils::{rand, time},
};

pub struct OtherAct {
    db: db_helper::DBMySql,
}

#[rustfmt::skip]
impl OtherAct {

    // phone: 98901...
    pub async fn send_sms_code(&self, phone_number: &str) -> Result<pb::Sms,GenErr> {
        // our sms sender just accepts Iran format: "0901513..."
        let phone_ir = phone_to_iran_format(phone_number)?;
        let phone_inter = phone_to_international_format(phone_number)?;

        // A session like code just for sms verfication process
        let hash_code = rand::rand_string(15);

        let sms_code = sms_sender::send_login_code_sms(&phone_ir, 4)
            .await
            .unwrap();

        let sms_pb = pb::Sms {
            install_uuid: "".to_string(),
            phone_number: phone_inter.clone(),
            country_code: "98".to_string(),
            for_login: true,
            hash_code: hash_code.clone(),
            confirm_code: sms_code,
            gateway_number: "".to_string(),
            text_body: "".to_string(),
            created_time: time::get_time_now_sec(),
            gateway_error: "".to_string(), //todo
            intent: "login".to_string(),
            result: "ok".to_string(),
        };

        self.db.save_sms(&sms_pb).await?;

        Ok(sms_pb)
    }

    pub async fn verify_sms_code(&self, phone_number: &str , hash_code: &str, confirm_code: &str) -> Result<bool,GenErr> {
        let phone_inter = phone_to_international_format(phone_number)?;
        let sms = self.db.get_sms(&phone_inter,hash_code).await?;

        if sms.hash_code == confirm_code {
            Ok(true)
        } else {
            Ok(false)
        }
    }

}

fn phone_to_iran_format(number: &str) -> Result<String, GenErr> {
    let number_ir = if number.starts_with("98") {
        // international format
        format!("0{}", &number[2..])
    } else if number.starts_with("0") {
        // Iran format
        format!("{}", &number)
    } else if number.starts_with("9") {
        // Iran mobile without 0
        format!("0{}", &number)
    } else {
        // Unknown format
        return Err(GenErr::WrongParam);
    };

    Ok(number_ir)
}

fn phone_to_international_format(number: &str) -> Result<String, GenErr> {
    let number_inter = if number.starts_with("98") {
        // international format
        format!("{}", &number)
    } else if number.starts_with("0") {
        // Iran format with 0
        format!("98{}", &number[1..])
    } else if number.starts_with("9") {
        // Iran mobile without 0
        format!("98{}", &number)
    } else {
        // Unknown format
        return Err(GenErr::WrongParam);
    };

    Ok(number_inter)
}

pub mod param {

    #[derive(Clone, Default, Debug)]
    pub struct XXSample {}
}
