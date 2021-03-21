use std::fs::File;
use std::thread;
use std::thread::Thread;

use serde_json;
use std::collections::HashMap;

use std::io::Write;
use std::ops::Add;

use serde_json::Error;
use tokio::task;

mod twtypes;

pub fn main1() {
    run();
}

fn run() {
    let mut api = TwitterClient::default();
    let followers = api.get_followers(1373700793874911241);
    println!("{:#?}", followers);
    // let res = api.GetTweets(790728);
    let res = api.get_tweets_by_username("assassinscreed"); // bbcpersian assassinscreed

    println!("{:#?}", res);

    /*
    println!("getusers ============");
    // let tweet = res.unwrap().get(0).clone().unwrap();
    let tweets = res.unwrap();
    let tweet = tweets.get(0).unwrap().clone();
    let folloers = api.get_followers(tweet.user.id);
    println!("followers {:?} ", folloers);

    let users = api.get_users(&folloers.unwrap());
    println!("getusers {:#?}", users);

    println!("Get Media");

    for t in tweets {
        //  api.GetMedia(t);
    }

    thread::sleep(std::time::Duration::from_secs(10))
    */
}

#[derive(Debug)]
enum TwitterError {
    SerdeJson(serde_json::Error),
}

impl From<serde_json::Error> for TwitterError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

#[derive(Default)]
struct TwitterClient {
    _reqwest_client: reqwest::blocking::Client,
}

impl TwitterClient {
    pub fn new() -> Self {
        TwitterClient::default()
    }

    // todo
    fn is_username_free(&self, username: &str) -> bool {
        false
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
        let tweets = serde_json::from_str::<Vec<twtypes::Tweet>>(body_str.as_str())?;
        Ok(tweets)
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
        // });
    }

    fn _get_body(&self, url: &str) -> Result<String, TwitterError> {
        let client = &self._reqwest_client; //reqwest::blocking::Client::new();
        let res = client.get(url)
            .header("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAAIu%2FDQEAAAAAaYGRj89RCMH7kC9qN2xTzEHHUaQ%3D7QgRJxwUHILsAeX3dvistI0K5BrdKQUs7t1CNFkbsldJrtPYla")
            .send();

        // todo
        Ok(res.unwrap().text().unwrap())
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
    use super::twtypes;

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
}
