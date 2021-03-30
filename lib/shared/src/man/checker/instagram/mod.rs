use std::fs::File;
use std::thread;
use std::thread::Thread;

use serde_json;
use std::collections::HashMap;

use std::io::Write;
use std::ops::Add;

use serde_json::Error;
use tokio::task;

mod insta_types;

// Notes:
// + Some observation about requesting "https://www.instagram.com/{username}/?__a=1":
//  + Instagram will redirect us to an html page if session is invalid.
//  + After working fine for some periods, Insta returned html "Fill Your Birthday".
//  + Having some mechanism about monitoring if api working correctly is unavoidable.

#[derive(Debug)]
pub enum InstaError {
    SerdeJson(serde_json::Error),
    Reqwest(reqwest::Error),
    NotFound, // page 404
    ApiMisBehave,
}

impl From<serde_json::Error> for InstaError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

impl From<reqwest::Error> for InstaError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

#[derive(Debug, Default)]
pub struct UsernameAvailability {
    pub is_registered: bool,
    pub followers_count: i64,
    pub fullname: String,
    pub about: String,
    pub texts: String,
}

#[derive(Default)]
pub struct InstaClient {
    _reqwest_client: reqwest::Client,
}

impl InstaClient {
    pub fn new() -> Self {
        InstaClient::default()
    }

    pub async fn check_username(&self, username: &str) -> Result<UsernameAvailability, InstaError> {
        let res = self.get_user_by_username(username).await;
        // println!("+++++++>>>> check_username >>> {:#?}", res);
        match res {
            Ok(root) => {
                let mut txt = "".to_string();

                let followers_count = root.graphql.user.edge_followed_by.count;
                let about = root.graphql.user.biography;
                let fullname = root.graphql.user.full_name;

                for ed in root.graphql.user.edge_owner_to_timeline_media.edges {
                    for ed2 in ed.node.edge_media_to_caption.edges {
                        txt.push_str(&ed2.node.text);
                    }
                }

                Ok(UsernameAvailability {
                    is_registered: true,
                    followers_count: followers_count,
                    fullname,
                    about,
                    texts: txt,
                })
            }
            Err(e) => match e {
                InstaError::NotFound => Ok(UsernameAvailability {
                    is_registered: false,
                    followers_count: -1,
                    ..Default::default()
                }),
                _ => Err(e),
            },
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Result<insta_types::Root, InstaError> {
        let url = format!("https://www.instagram.com/{}/?__a=1", username);
        let body_str = self._get_insta_body(url.as_str()).await?;

        println!("+++++++ body +++++>>>> {} ", &body_str);

        // If no such username exists body will be "{}"; for safeguarding we consider anything
        // than 100 to be not registered as correct Json body is bigger than that
        if body_str.len() < 200 {
            return Err(InstaError::NotFound);
        }

        // Note: not found usernames has already returned (body "{}")
        if body_str.starts_with(r#"{""#) { // If is json string
            let user = serde_json::from_str::<insta_types::Root>(&body_str)?;
            Ok(user)
        } else {
            // Checking whether Insta is sending us an html page
            if body_str.starts_with("<!DOCTYPE html>") {
                // todo: some alerting Admins
            }

            Err(InstaError::ApiMisBehave)
        }
    }

    // Notes:
    // + With our code right now Instagram only requires cookie header > other header is just set optionally
    async fn _get_insta_body(&self, url: &str) -> Result<String, InstaError> {
        let client = &self._reqwest_client; //reqwest::blocking::Client::new();
        let res = client.get(url)
            .header("accept","*/*")
            .header("accept-language", "en-US")
            .header("sec-ch-ua","\"Google Chrome\";v=\"89\", \"Chromium\";v=\"89\", \";Not A Brand\";v=\"99\"")
            .header("x-requested-with", "XMLHttpRequest")
            .header("cookie", "ig_cb=2; mid=YGCk_QAEAAGfPE6uNlrJ3qQXneEU; ig_did=C8F0F2FD-9331-48BF-94F0-C6D9A3FD166C; csrftoken=LkJq1a2Zc7XwFCUWSOECMxnIBNkG1Wst; ds_user_id=9211030766; sessionid=9211030766%3ATiAq00e1RHkUZ0%3A24; rur=RVA")
            .send().await?;

        Ok(res.text().await.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        println!("running instagram username checker tests ...")
    }

    #[test]
    pub async fn play_main() {
        run().await;
    }

    pub async fn run() {
        let api = InstaClient::default();
        let t = api.check_username("assassinscreed").await;
        // println!("-------------------------------  {:#?}", t);
        // let t = api.get_user_by_username("pugloulou");
        // let t = api.check_username("instagram_459034759");
        // let t = api.check_username("raika.com._");
        // let t = api.check_username("mailproxy30");
        println!("-------------------------------  {:#?}", t);
    }
}
