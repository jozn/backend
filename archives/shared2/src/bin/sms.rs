use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use serde_json;

#[tokio::main]
async fn main() {
    let res = sample2().await;
    println!("ewa: {:#?}", res);

    let o = std::path::Path::new("like/tes.te");
    println!("{:?}", o);
}

async fn sample2() -> Result<(), reqwest::Error> {
    let params = [
        ("message", "myسیبmsg"),
        ("receptor", "09015132328"),
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

    println!("+++++++++++++++++++++++++++++++++++++++");
    dbg!(&req);

    println!("{:#?}", req);

    println!("+++++++++++++++++++++++++++++++++++++++");

    let mm = req.send().await?;

    println!("+++++++++++++++++++++++++++++++++++++++");
    dbg!(&mm);
    let new_post = dbg!(mm.bytes().await);

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
