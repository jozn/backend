use std::fs::File;
use std::io::Read;
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
    HtmlParsing,
    NotFound, // page 404
}

impl From<reqwest::Error> for TelgError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

/*impl From<cssparser::parser::ParseError<'_, selectors::parser::SelectorParseErrorKind<'_>>> for TelgError{
    fn from(_: cssparser::parser::ParseError<'_, selectors::parser::SelectorParseErrorKind<'_>>) -> Self {
       TelgError::HtmlParsing
    }
}
*/
/*impl From<cssparser::parser::ParseError<E>> for TelgError {
    fn from(_: cssparser::parser::ParseError<E>) -> Self {
        TelgError::HtmlParsing
    }
}*/

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

    pub fn check_username2(&self, username: &str) -> Result<UsernameAvailability, TelgError> {
        let res = self.check_username(username);
        // println!("+++++++>>>> check_username >>> {:#?}", res);
        match res {
            Ok(root) => {
                Ok(root)
            }
            Err(e) => match e {
                TelgError::NotFound => Ok(UsernameAvailability {
                    is_registered: false,
                    ..Default::default()
                }),
                _ => Err(e),
            },
        }
    }

    fn check_username(&self, username: &str) -> Result<UsernameAvailability, TelgError> {
        // def of needed param to extract
        let mut html_extra_text = "";
        let mut fullname = "";
        let mut description = "";
        let mut channel_html_msgs_text = "".to_string(); // Channels messages texts (a simple collection of all html text tags)
        let mut members_count = -1_i64;

        let url = format!("https://t.me/{}/", username);
        let body_str = self._get_http_body(url.as_str())?;

        // Safeguarding
        if body_str.len() < 200 {
            return Ok(UsernameAvailability {
                is_registered: false,
                ..Default::default()
            });
        }

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

        if html_extra_text.contains("members") {
            // if true means this is channel/supergroup page
            let mem_str = html_extra_text.replace("members", "").replace(" ", "");
            members_count = mem_str.parse().unwrap_or(-1);
            if members_count > 1 {
                // Select description
                let selector2 =
                    scraper::Selector::parse(r#"meta[name="twitter:description"]"#).unwrap();
                let select2 = html_doc.select(&selector2);

                for el in select2 {
                    description = &el.value().attr("content").unwrap_or_default();
                }

                // Select channel name
                let selector3 = scraper::Selector::parse(r#"meta[property="og:title"]"#).unwrap();
                let select3 = html_doc.select(&selector3);

                for el in select3 {
                    fullname = &el.value().attr("content").unwrap_or_default();
                }
            }
        }

        if members_count > 5 {
            let url = format!("https://t.me/s/{}/", username);
            let body_str = self._get_http_body(url.as_str());
            if body_str.is_ok() {
                let body_str = body_str.unwrap();
                channel_html_msgs_text = html2text::from_read(body_str.as_bytes(), body_str.len());
            }
        }

        // println!("{:?}", body_str);
        println!("about >> {:?}", description);
        println!("name >> {:?}", fullname);
        println!("follower >> {:?}", members_count);
        println!("msgs >> {:?}", channel_html_msgs_text);

        if members_count > 5 {
            Ok(UsernameAvailability {
                is_registered: true,
                followers_count: members_count,
                fullname: fullname.to_string(),
                about: description.to_string(),
                texts: channel_html_msgs_text,
            })
        } else {
            Ok(UsernameAvailability {
                is_registered: false,
                ..Default::default()
            })
        }
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
        // let t = api.get_user_by_username("farsna");
        // println!("-------------------------------  {:#?}", t);
        // let t = api.get_user_by_username("pugloulou");
        // let t = api.check_username("instagram_459034759");
        // let t = api.check_username("raika.com._");
        let t = api.check_username("mailproxy30");
        let t = api.check_username("flip_net");
        println!("-------------------------------  {:#?}", t);
    }
}
