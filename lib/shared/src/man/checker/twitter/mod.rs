use std::fs::File;
use std::thread;
use std::thread::Thread;

use serde_json;
use std::collections::HashMap;

use std::io::Write;
use std::ops::Add;

use crate::man::checker::twitter::twtypes::Tweet;
use serde_json::Error;
use tokio::task;

mod twtypes;

// Notes:
// + This module is a copy of old project and it does more than username checking
// + Twitter returns normal html with http 200 for not existed usernames

#[derive(Debug)]
pub enum TwitterError {
    SerdeJson(serde_json::Error),
    Reqwest(reqwest::Error),
    NotFound, // page 404
}

impl From<serde_json::Error> for TwitterError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

impl From<reqwest::Error> for TwitterError {
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
pub struct TwitterClient {
    _reqwest_client: reqwest::blocking::Client,
}

impl TwitterClient {
    pub fn new() -> Self {
        TwitterClient::default()
    }

    pub fn check_username(&self, username: &str) -> Result<UsernameAvailability, TwitterError> {
        let res = self.get_tweets_by_username(username);
        // println!("+++++++>>>> is_username_free >>> {:#?}", res);
        match res {
            Ok(tweets) => {
                let mut txt = "".to_string();
                let mut followers_count = 0;
                let mut description = "".to_string();
                let mut fullname = "".to_string();

                let user = self.get_user_by_username(username);
                if user.is_ok() {
                    let user = user.unwrap();
                    fullname = user.name;
                    description = user.description.unwrap_or("".to_string());
                    followers_count = user.followers_count;
                }

                for t in tweets {
                    txt.push_str(&t.full_text);
                }

                Ok(UsernameAvailability {
                    is_registered: true,
                    followers_count: followers_count,
                    fullname,
                    about: description,
                    texts: txt,
                })
            }
            Err(e) => match e {
                TwitterError::NotFound => Ok(UsernameAvailability {
                    is_registered: false, // Explicit
                    ..Default::default()
                }),
                _ => Err(e),
            },
        }
    }

    pub fn get_tweets(&self, user_id: u64) -> Result<Vec<twtypes::Tweet>, TwitterError> {
        let url = api_url::get_timeline_tweets_user_id(user_id);
        self._get_tweets_action(url.as_str())
    }

    pub fn get_tweets_by_username(
        &self,
        username: &str,
    ) -> Result<Vec<twtypes::Tweet>, TwitterError> {
        let url = api_url::get_timeline_tweets_username(username);
        self._get_tweets_action(url.as_str())
    }

    fn _get_tweets_action(&self, url: &str) -> Result<Vec<twtypes::Tweet>, TwitterError> {
        let body_str = self._get_body(url)?;
        let tweets_res = serde_json::from_str::<Vec<twtypes::Tweet>>(body_str.as_str());
        if tweets_res.is_err() {
            let err_res = serde_json::from_str::<twtypes::Errors>(body_str.as_str())?;
            // Simplified logic: if body string return is Errors then consider it "Not Found" username > buggy but enough
            Err(TwitterError::NotFound)
        } else {
            Ok(tweets_res.unwrap())
        }
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<twtypes::User, TwitterError> {
        let url = format!(
            "https://api.twitter.com/1.1/users/show.json?screen_name={}",
            username
        );
        let body_str = self._get_body(url.as_str())?;
        let user = serde_json::from_str::<twtypes::User>(&body_str)?;
        Ok(user)
    }

    pub fn get_followers(&self, user_id: u64) -> Result<Vec<u64>, TwitterError> {
        let url = format!(
            "https://api.twitter.com/1.1/followers/ids.json?user_id={}&count=250",
            user_id
        );
        let body_str = self._get_body(url.as_str())?;
        let followers = serde_json::from_str::<twtypes::Followers>(&body_str)?;
        Ok(followers.ids.clone())
    }

    pub fn get_users(&self, user_ids: &Vec<u64>) -> Result<Vec<twtypes::User>, TwitterError> {
        let mut arr_str = String::from("");
        let mut cnt = 0;
        for i in user_ids {
            if cnt < 100 {
                // limits of api
                arr_str = arr_str.add(format!("{},", i).as_str());
            }
            cnt += 1;
        }
        let arr_trimed = arr_str.trim_end_matches(",");

        let url_req = format!(
            "https://api.twitter.com/1.1/users/lookup.json?user_id={}",
            arr_trimed
        );
        let body_str = self._get_body(url_req.as_str())?;
        let users = serde_json::from_str::<Vec<twtypes::User>>(body_str.as_str())?;
        Ok(users)
    }

    // This func is commented in integration from old repo into Flip.
    pub fn get_media(&mut self, tweet: twtypes::Tweet) {}

    fn _get_body(&self, url: &str) -> Result<String, TwitterError> {
        let client = &self._reqwest_client;
        let res = client.get(url)
            .header("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAAIu%2FDQEAAAAAaYGRj89RCMH7kC9qN2xTzEHHUaQ%3D7QgRJxwUHILsAeX3dvistI0K5BrdKQUs7t1CNFkbsldJrtPYla")
            .send()?;

        Ok(res.text().unwrap_or_default())
    }
}

mod api_url {
    pub fn get_timeline_tweets_user_id(user_id: u64) -> String {
        format!("https://api.twitter.com/1.1/statuses/user_timeline.json?user_id={}&count=10&tweet_mode=extended&exclude_replies=true&include_rts=false"
                ,user_id )
    }

    pub fn get_timeline_tweets_username(username: &str) -> String {
        format!("https://api.twitter.com/1.1/statuses/user_timeline.json?screen_name={}&count=10&tweet_mode=extended&exclude_replies=true&include_rts=false"
                ,username )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This function is quick showcase for how a Tweet looks like. It's not used anywhere.
    fn sample_tweet() -> twtypes::Tweet {
        let t = twtypes::Tweet {
            created_at: "".to_string(),
            id: 0,
            id_str: "".to_string(),
            full_text: "".to_string(),
            truncated: false,
            entities: twtypes::Entities {
                urls: vec![],
                media: None,
            },
            in_reply_to_status_id: None,
            in_reply_to_user_id: None,
            user: twtypes::User {
                id: 0,
                name: "".to_string(),
                screen_name: "".to_string(),
                description: None,
                url: None,
                protected: false,
                followers_count: 0,
                friends_count: 0,
                listed_count: 0,
                created_at: "".to_string(),
                favourites_count: 0,
                verified: false,
                statuses_count: 0,
                profile_image_url: None,
                profile_image_url_https: None,
                profile_banner_url: None,
                default_profile_image: false,
            },
            is_quote_status: false,
            retweet_count: 0,
            favorite_count: 0,
            favorited: false,
            retweeted: false,
            lang: "".to_string(),
            extended_entities: None,
            possibly_sensitive: None,
        };
        t
    }

    #[test]
    fn test1() {
        println!("running twitter tests ...")
    }

    // #[test]
    pub fn play_main() {
        run()
    }

    fn run() {
        let api = TwitterClient::default();
        let t = api.check_username("assassinscreed");
        println!("-------------------------------  {:#?}", t);
        let t = api.check_username("assassinscreed55");
        println!("-------------------------------  {:#?}", t);
    }
}

// Archive
// Old dl media codes
/*
// This func is commented in integration from old repo into Flip.
    pub fn get_media(&mut self, tweet: twtypes::Tweet) {
        // let hanlder = thread::spawn(move || {
        match &tweet.extended_entities {
            Some(et) => {
                /*for m in et.media.clone() {
                    let media = m.clone();
                    // let t = task::block_on(async {
                    let m2 = m.clone();
                    let t = tokio::task::spawn(async {
                        print!("dl: {}", m.media_url);
                        let name = format!("./out/{}.jpg", m2.id);
                        let mut file = File::create(name).unwrap();
                        let mut res = surf::get(&m2.media_url).await;
                        match res {
                            Ok(mut res) => {
                                let body = res.body_bytes().await.unwrap();
                                println!(" == size : {}", body.len());
                                file.write_all(&body);
                            }
                            Err(err) => {
                                println!("Err in Res: {}", err);
                            }
                        }
                    }).await;

                    if let Some(vi) = m.video_info.clone() {
                        // let t = task::block_on(async {
                        let t = tokio::task::spawn(async {
                            for vid in &vi.variants {
                                if vid.content_type == "video/mp4" {
                                    // print!("dl: {}", m.video_info);
                                    let name = format!("./out/{}.mp4", m.id);
                                    let mut file = File::create(name).unwrap();
                                    let mut res = surf::get(&vid.url).await;
                                    match res {
                                        Ok(mut res) => {
                                            let body = res.body_bytes().await.unwrap();
                                            println!(" MP4 == size : {}", body.len());
                                            file.write_all(&body);
                                        }
                                        Err(err) => {
                                            println!("Err in MP4 Res: {}", err);
                                        }
                                    }
                                }
                            }
                        }).await;
                    }
                }*/
            }
            None => {}
        };
*/
