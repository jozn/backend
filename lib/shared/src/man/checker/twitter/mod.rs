use std::fs::File;
use std::thread;
use std::thread::Thread;
use time;
use ureq;

use serde_json;
use std::collections::HashMap;
use std::time::Instant;

extern crate chrono;
use chrono::format::ParseError;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use std::io::Write;
use std::ops::Add;

use async_std::task;
use std::borrow::Borrow;
use surf;
use time::NumericalStdDurationShort;

mod twtypes;

pub fn main1() {
    run();
    // twitter_crawler::main1();
}

fn run() {
    let mut api = API::default();
    // let res = api.GetTweets(790728);
    let res = api.GetTweetsByUsername("assassinscreed"); // bbcpersian assassinscreed

    println!("{:?}", res);

    println!("getusers ============");
    // let tweet = res.unwrap().get(0).clone().unwrap();
    let tweets = res.unwrap();
    let tweet = tweets.get(0).unwrap().clone();
    let folloers = api.GetFollowers(tweet.user.id);
    println!("followers {:?} ", folloers);

    let users = api.GetUsers(&folloers.unwrap());
    println!("getusers {:#?}", users);

    println!("Get Media");

    for t in tweets {
        //  api.GetMedia(t);
    }

    let g = Grabber {
        api: api,
        db: Mem {},
        harvesting: false,
        stopping: false,
    };
    // g.db.save_tweet(&g);
    // g.db.save_tweet(&g);
    g.db.get_tweet();

    // Grabber::sa\\

    thread::sleep(10000.seconds())
}

struct Grabber {
    api: API,
    db: Mem,
    harvesting: bool, // if we run in harvest mod
    stopping: bool,   // signal stop harvesting
}

impl Grabber {
    fn run_production_harvest(&self) {}

    fn try_username(&self, username: &str) {}

    fn is_username_free(&self, username: &str) -> bool {
        false
    }

    fn run_sample_harvest(&self) {}
}

struct Mem {}
struct UserState {
    user_id: u64,
    username: String,
    last_fetched: u32,
    lang: &'static str,
    blocked: bool,
}

// maybe better not to have this
struct GlobalState {
    total_users: u64,
}

impl DB for Mem {
    fn save_tweet(&self, t: twtypes::Tweet) {
        println!("y9999");
    }
}

trait DB {
    fn save_tweet(&self, t: twtypes::Tweet) {
        println!("yes");
    }

    fn get_tweet(&self) -> Option<twtypes::Tweet> {
        println!("1");
        None
    }
    fn save_user_state(&self, m: &UserState) {
        println!("ok");
    }

    fn get_user_state(&self, uid: u64) -> Option<UserState> {
        println!("ok");
        None
    }

    fn get_user_state_by_username(&self, username: &str) -> Option<UserState> {
        println!("ok");
        None
    }
    fn save_global_state(&self, gs: &GlobalState) {
        println!("ok");
    }

    fn get_global_state(&self) -> Option<GlobalState> {
        println!("ok");
        None
    }
    // use this for debuging -- maybe not suited for realworld app
    fn on_grabbed_tweets(&self, _: Vec<twtypes::Tweet>) {}
    fn on_grabbed_users(&self, _: Vec<twtypes::User>) {}
    fn on_grabbed_media(&self, _: Vec<twtypes::Media>) {}
}

//type Error8 = std::io::Error;
#[derive(Default, Debug)]
struct Error8 {}

type TweetsResponse = Result<Vec<twtypes::Tweet>, Error8>;

#[derive(Default)]
struct API {}

impl API {
    pub fn GetTweets(&self, user_id: u64) -> TweetsResponse {
        let url = API_URl::getTimelineTweetsUserId(user_id);
        self.GetTweetsAction(url.as_str())
    }

    pub fn GetTweetsByUsername(&self, username: &str) -> Result<Vec<twtypes::Tweet>, Error8> {
        let url = API_URl::getTimelineTweetsUserName(username);
        self.GetTweetsAction(url.as_str())
    }

    fn GetTweetsAction(&self, url: &str) -> Result<Vec<twtypes::Tweet>, Error8> {
        let body = getBody(url);
        match body {
            Ok(body_str) => {
                let tweets_resu = serde_json::from_str::<Vec<twtypes::Tweet>>(body_str.as_str());
                match tweets_resu {
                    Ok(tweets) => Ok(tweets),
                    Err(err) => Err(Error8::default()),
                }
            }
            Err(err) => Err(Error8::default()),
        }
    }

    fn GetFollowers(&self, user_id: u64) -> Result<Vec<u64>, Error8> {
        let url = format!(
            "https://api.twitter.com/1.1/followers/ids.json?user_id={}&count=250",
            user_id
        );
        let body_resu = getBody(url.as_str());

        match body_resu {
            Ok(body_str) => {
                let follow_res = serde_json::from_str::<twtypes::Followers>(&body_str);
                match follow_res {
                    Ok(followers) => Ok(followers.ids.clone()),
                    Err(err) => Err(Error8::default()),
                }
            }
            Err(err) => Err(Error8::default()),
        }
    }

    pub fn GetUsers(&self, user_ids: &Vec<u64>) -> Result<Vec<twtypes::User>, Error8> {
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
        let body_resu = getBody(url_req.as_str());

        match body_resu {
            Ok(body_str) => {
                let users_resu = serde_json::from_str::<Vec<twtypes::User>>(body_str.as_str());
                match users_resu {
                    Ok(users) => Ok(users),
                    Err(err) => Err(Error8::default()),
                }
            }
            Err(err) => Err(Error8::default()),
        }
    }

    fn GetMedia(&mut self, tweet: twtypes::Tweet) {
        let hanlder = thread::spawn(move || {
            match &tweet.extended_entities {
                Some(et) => {
                    for m in et.media.clone() {
                        let media = m.clone();
                        let t = task::block_on(async {
                            print!("dl: {}", m.media_url);
                            let name = format!("./out/{}.jpg", m.id);
                            let mut file = File::create(name).unwrap();
                            let mut res = surf::get(&m.media_url).await;
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
                        });

                        if let Some(vi) = m.video_info.clone() {
                            let t = task::block_on(async {
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
                            });
                        }
                    }
                }
                None => {}
            }
        });
    }
}

const TWITTER_API_BASE: &str = "https://api.twitter.com/1.1/";
const TWITTER_API_TIMELINE_USERNAME: &str = "https://api.twitter.com/1.1/user_timeline.json?screen_name=sugandiiii&count=1000&tweet_mode=extended&exclude_replies=true&include_rts=false";
const TWITTER_API_TIMELINE_USER_ID: &'static str = "https://api.twitter.com/1.1/user_timeline.json?user_id={}&count=1000&tweet_mode=extended&exclude_replies=true&include_rts=false";

fn getBody(url: &str) -> std::io::Result<String> {
    // let agent = getAgent();

    let mut agent = ureq::agent();
    agent.set("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAAIu%2FDQEAAAAAaYGRj89RCMH7kC9qN2xTzEHHUaQ%3D7QgRJxwUHILsAeX3dvistI0K5BrdKQUs7t1CNFkbsldJrtPYla");

    let mut req = agent.get(url);
    let body = req.call().into_string();
    // println!("{} \n{:?}",url,body);

    body
}

fn getAgent() -> ureq::Agent {
    let mut agent = ureq::agent();
    agent.set("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAAIu%2FDQEAAAAAaYGRj89RCMH7kC9qN2xTzEHHUaQ%3D7QgRJxwUHILsAeX3dvistI0K5BrdKQUs7t1CNFkbsldJrtPYla");
    agent
}

mod API_URl {
    pub fn getTimelineTweetsUserId(user_id: u64) -> String {
        format!("https://api.twitter.com/1.1/statuses/user_timeline.json?user_id={}&count=1000&tweet_mode=extended&exclude_replies=true&include_rts=false"
                ,user_id )
    }

    pub fn getTimelineTweetsUserName(username: &str) -> String {
        format!("https://api.twitter.com/1.1/statuses/user_timeline.json?screen_name={}&count=1000&tweet_mode=extended&exclude_replies=true&include_rts=false"
                ,username )
    }
}

fn getSampleTweet() -> twtypes::Tweet {
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
/*
fn GetTweets(self, user_id: u64) -> Result<twtypes::Tweet, Error8> {
        let url = "https://api.twitter.com/1.1/statuses/user_timeline\
        .json?screen_name=ladygaga&count=1000&tweet_mode=extended&exclude_replies=true&include_rts\
        =false";

        task::block_on(async {
            let res = surf::get(url).await;
            match res {
                Ok(res2) => {


                    let t = twtypes::Tweet{
                        created_at: "".to_string(),
                        id: 0,
                        id_str: "".to_string(),
                        full_text: "".to_string(),
                        truncated: false,
                        entities: twtypes::Entities { urls: vec![], media: None },
                        in_reply_to_status_id: None,
                        in_reply_to_user_id: None,
                        user: twtypes::User{
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
                            default_profile_image: false
                        },
                        is_quote_status: false,
                        retweet_count: 0,
                        favorite_count: 0,
                        favorited: false,
                        retweeted: false,
                        lang: "".to_string(),
                        extended_entities: None,
                        possibly_sensitive: None
                    };

                    Ok(t);
                },
                Err(res2) => {
                    Result::Err(Error8::default());
                },
            }
        })


*/
