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

pub fn main1() {
    run();
}

fn run() {
    let api = InstaClient::default();
    // let t = api.check_username("assassinscreed");
    // println!("-------------------------------  {:#?}", t);
    let t = api.get_user_by_username("pugloulou");
    println!("-------------------------------  {:#?}", t);
}

fn run_loop() {
    let api = InstaClient::default();
    for i in  0..1000 {
        let t = api.get_user_by_username("pugloulou");
        println!("{:} -> {:#?}", i, t.is_ok());
    }

}


#[derive(Debug)]
enum InstaError {
    SerdeJson(serde_json::Error),
    NotFound, // page 404
}

impl From<serde_json::Error> for InstaError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

#[derive(Debug)]
pub struct UsernameAvailability {
    pub is_registered: bool,
    pub followers_count: i64,
    pub texts: String,
    pub lang: String,
}

#[derive(Default)]
struct InstaClient {
    _reqwest_client: reqwest::blocking::Client,
}

impl InstaClient {
    pub fn new() -> Self {
        InstaClient::default()
    }

    pub fn check_username(&self, username: &str) -> Result<UsernameAvailability, InstaError> {
/*        let res = self.get_tweets_by_username(username);
        // println!("+++++++>>>> is_username_free >>> {:#?}", res);
        match res {
            Ok(tweets) => {
                let mut txt = "".to_string();
                let mut followers_count = 0;

                let user = self.get_user_by_username(username);
                if user.is_ok() {
                    let user = user.unwrap();
                    txt.push_str(&user.description.unwrap_or("".to_string()));
                    followers_count = user.followers_count;
                }

                for t in tweets {
                    txt.push_str(&t.full_text);
                }

                Ok(UsernameAvailability {
                    is_registered: true,
                    followers_count: followers_count,
                    texts: txt,
                    lang: "".to_string(), // todo
                })
            }
            Err(e) => match e {
                InstaError::NotFound => Ok(UsernameAvailability {
                    is_registered: false,
                    followers_count: -1,
                    texts: "".to_string(),
                    lang: "".to_string(),
                }),
                _ => Err(e),
            },
        }*/
        Err(InstaError::NotFound)
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<insta_types::Root, InstaError> {
        let url = format!(
            "https://www.instagram.com/{}/?__a=1",
            username
        );
        let body_str = self._get_body_insta(url.as_str())?;
        println!("+++++++ body +++++>>>> {} ", &body_str);
        let user = serde_json::from_str::<insta_types::Root>(&body_str)?;
        Ok(user)
    }

    // Notes:
    // + With our code right now Instagram only requires cookie header > other header is just set optionally
    fn _get_body_insta(&self, url: &str) -> Result<String, InstaError> {
        let client = &self._reqwest_client; //reqwest::blocking::Client::new();
        let res = client.get(url)
            // .header("accept","*/*")
            // .header("accept-language", "en-US")
            // .header("sec-ch-ua","\"Google Chrome\";v=\"89\", \"Chromium\";v=\"89\", \";Not A Brand\";v=\"99\"")
            // .header("x-requested-with", "XMLHttpRequest")
            .header("cookie", "ig_cb=2; mid=YF-QdQAEAAH6ypdrPzu-y9ILcHcP; ig_did=222692C9-F1FE-4730-AF13-3F1905F54FEA; csrftoken=13eu4bWHUSOIR5gxx4WHebDkn7fXkmqv; ds_user_id=37578497393; sessionid=37578497393%3ACF2Rwzcp35M4Qv%3A29; rur=FTW")
            .send();

        // todo fix
        Ok(res.unwrap().text().unwrap())
    }

    fn _get_body(&self, url: &str) -> Result<String, InstaError> {
        let client = &self._reqwest_client; //reqwest::blocking::Client::new();
        let res = client.get(url)
            .header("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAAIu%2FDQEAAAAAaYGRj89RCMH7kC9qN2xTzEHHUaQ%3D7QgRJxwUHILsAeX3dvistI0K5BrdKQUs7t1CNFkbsldJrtPYla")
            .send();

        // todo fix
        Ok(res.unwrap().text().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::insta_types;

    #[test]
    fn test1() {
        println!("running instagram tests ...")
    }
}
