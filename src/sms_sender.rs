
use crate::com::GenErr;
use rand::prelude::*;
use std::borrow::Borrow;

const MSG2: &str =
r#"<#> کد تایید: :code:
Flip
این کد را در اختیار بقیه قرار ندهید.
rqwerljk"#;

const MSG: &str =
r#"<#> کد تایید: :code:
Flip
rqwerljk"#;

fn get_msg (code :u32) -> String {
    format!(
r#"<#> کد تایید:  {}
Flip-ir
rqwerljk"#,
code )
}
pub async fn send_confirm_sms(to_phone: &str, code_len :u32) -> Result<(), GenErr> {

    let code: u32 = rand::thread_rng().gen_range( 10_u32.pow(code_len-1) , 10_u32.pow(code_len));
    // let s2 = "{} {}";
    // let msg = MSG.replace(":code:", code.to_string().borrow());
    let msg = get_msg(code);
    // let msg = format!(s2,MSG, code );
    send_sms(to_phone, &msg).await
}

async fn send_sms(to_phone: &str, message :&str) -> Result<(), GenErr> {
    ghasedak::send(to_phone, message).await?;
    Ok(())
}

mod ghasedak {
    use reqwest::header::USER_AGENT;
    use serde::{Deserialize, Serialize};
    use serde_json;

    pub async fn send(to_phone: &str, message :&str) -> Result<(), reqwest::Error> {
/*        let params = [
            ("message", "myسیبmsg"),
            ("receptor", "09015132328"),
            ("linenumber", "10008566"),
        ];*/

        let params = [
            ("message", message),
            ("receptor", to_phone),
            ("linenumber", "10008566"),
        ];

        let req = reqwest::Client::new()
            .post("http://api.ghasedak.io/v2/sms/send/simple")
            .header(
                "apikey",
                "57d606af03970b8713840cdef028fff46fe2f677243a8547d1a3ebfbe4c3ab23",
            )
            .header("Accept", "application/json")
            .header("Charset", "UTF-8")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params);

        println!("{:#?}", req);

        let new_post = req.send().await?.bytes().await;

        println!("Done ");

        println!("{:#?}", new_post);
        let val: SmsResult = serde_json::from_slice(&new_post.unwrap()).unwrap();

        println!(" val {:#?}", val);

        Ok(())
    }

    #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SmsResult {
        pub result: ResultOut,
        pub items: Option<Vec<i64>>,
    }

    #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ResultOut {
        pub code: i64,
        pub message: String,
    }

}
