use serde::{Deserialize, Serialize};
use reqwest::header::USER_AGENT;

#[tokio::main]
async fn main()  {
    let res = sample2().await;
    println!("ewa: {:#?}", res);

}

async fn sample2() -> Result<(), reqwest::Error> {
    let sms =  SendSms{
        Messages: vec!["testing for sms.ir".to_string()],
        MobileNumbers:vec!["09015132328".to_string()],
        LineNumber: "3000123456789".to_string(),
        SendDateTime: "".to_string(),
        CanContinueInCaseOfError: "true".to_string(),
    };

    let new_post = reqwest::Client::new()
        .post("https://RestfulSms.com/api/MessageSend")
        .header("x-sms-ir-secure-token","6cc7b4409e568e77acba03b")
        .json(&sms);
        // .send();
        // .await?;
        // .json()
        // .await?;


    println!("Done ");
    println!("{:#?}", new_post);
    let v =  new_post.send().await?;
    println!("{:#?}", v);
    // Post {
    //     id: Some(
    //         101
    //     ),
    //     title: "Reqwest.rs",
    //     body: "https://docs.rs/reqwest",
    //     user_id: 1
    // }
    Ok(())
}


#[derive(Debug, Serialize, Deserialize)]
struct SendSms {
    Messages: Vec<String>,
    MobileNumbers: Vec<String>,
    LineNumber: String,
    SendDateTime: String,
    CanContinueInCaseOfError: String,
}


//////////// Del



