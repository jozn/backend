use std::fs::File;
use std::thread;
use std::thread::Thread;

use serde_json;
use std::collections::HashMap;

use std::io::Write;
use std::ops::Add;

use serde_json::Error;
use tokio::task;

#[derive(Debug)]
pub enum TelgError {
    Reqwest(reqwest::Error),
    NotFound, // page 404
}

impl From<reqwest::Error> for TelgError {
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

#[derive(Debug, Default)]
struct TelgHtmResult {
    is_supergroup: bool,
    followers_count: i64,
    fullname: String,
    about: String,
    texts: String,
}

#[derive(Default)]
struct TelgClient {
    _reqwest_client: reqwest::blocking::Client,
}

impl TelgClient {
    pub fn new() -> Self {
        TelgClient::default()
    }

    pub fn check_username(&self, username: &str) -> Result<UsernameAvailability, TelgError> {
        let res = self.get_user_by_username(username);
        // println!("+++++++>>>> check_username >>> {:#?}", res);
        /*        match res {
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
        }*/
        Err(TelgError::NotFound)
    }

    fn get_user_by_username(&self, username: &str) -> Result<bool, TelgError> {
        // def of needed param to extract
        let mut html_extra_text = "";
        let mut about = "";
        let mut mem_num = -1_i32;

        let url = format!("https://t.me/{}/", username);
        let body_str = self._get_http_body(url.as_str())?;

        let html_doc = scraper::Html::parse_document(&body_str);

        // This html div contains the number of members of a channel; if this class is present
        //  it means this is channels/supergroup page otherwise it's not. In the second case we
        //  simply return "it's not registered".
        // selecting: <div class="tgme_page_extra">1 035 427 members</div>
        let selector = scraper::Selector::parse("div.tgme_page_extra").unwrap();
        let select = html_doc.select(&selector);

        for el in select {
            html_extra_text = el.text().next().unwrap_or_default();
        }

        println!(">> html txt: {:?}", html_extra_text);


        if html_extra_text.contains("members") { // if true means this is channel/supergroup page
            println!(">> xxxxxxxxxxx html txt: {:?}", html_extra_text);
            let mem_str = html_extra_text.replace("members", "").replace(" ", "");
            println!(">> bbbb: {:?}", mem_str);
            mem_num = mem_str.parse().unwrap_or(-1);
            if mem_num > 1 {
                // select about
                let s = scraper::Selector::parse(r#"meta[name="twitter:description"]"#).unwrap();
                let re = html_doc.select(&s);

                for el in re {
                    about = &el.value().attr("content").unwrap_or_default();
                }
            }
        }

        println!("{:?}", body_str);
        println!("about >> {:?}", about);
        println!("follower >> {:?}", mem_num);

        // If no such username exists body will be "{}"; for safeguarding we consider anything
        // than 100 to be not registered as correct Json body is bigger than that
        if body_str.len() < 200 {
            return Err(TelgError::NotFound);
        }

        Ok(true)
    }

    // Notes:
    // + With our code right now Instagram only requires cookie header > other header is just set optionally
    fn _get_http_body(&self, url: &str) -> Result<String, TelgError> {
        let client = &self._reqwest_client; //reqwest::blocking::Client::new();
        let res = client.get(url).send()?;

        Ok(res.text().unwrap_or_default())
    }
}

// #[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test1() {
        println!("running instagram username checker tests ...")
    }

    #[test]
    pub fn play_main() {
        run();
    }

    pub fn run() {
        let api = TelgClient::default();
        let t = api.get_user_by_username("farsna");
        // println!("-------------------------------  {:#?}", t);
        // let t = api.get_user_by_username("pugloulou");
        // let t = api.check_username("instagram_459034759");
        // let t = api.check_username("raika.com._");
        // let t = api.check_username("mailproxy30");
        println!("-------------------------------  {:#?}", t);
    }
}
