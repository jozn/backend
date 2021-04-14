use crate::man::checker::telegram::{TelgError, UsernameAvailability};
use crate::man::utils;

pub mod instagram;
pub mod telegram;
pub mod twitter;

//====== Types
#[derive(Default, Debug)]
pub struct CheckResult {
    registered: Vec<PlatformRes>,
}

#[derive(Debug)]
pub struct PlatformRes {
    platform: Platform,
    followers: u32,
    farsi: bool,
    fullname: String,
}

#[derive(Debug)]
pub enum Platform {
    Telegram,
    Twitter,
    Instagram,
}

//====== Checker Functionality
#[derive(Default)]
pub struct Checker {
    telegram: telegram::TelgClient,
    instagram: instagram::InstaClient,
    twitter: twitter::TwitterClient,
}

impl Checker {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn check_username(&self, username: &str) -> CheckResult {
        let mut check_out_res = CheckResult::default();

        use utils::lang::is_farsi_text;

        // Check Telegram
        let res = self.telegram.check_username(username).await;
        match res {
            Ok(ua) => {
                if ua.is_registered && ua.followers_count > 100 {
                    let is_fa = is_farsi_text(&ua.fullname)
                        || is_farsi_text(&ua.about)
                        || is_farsi_text(&ua.texts);

                    if is_fa || ua.followers_count > 2000 {
                        let pr = PlatformRes {
                            platform: Platform::Telegram,
                            followers: ua.followers_count as u32,
                            farsi: is_fa,
                            fullname: ua.fullname,
                        };
                        check_out_res.registered.push(pr);
                    }
                }
            }
            Err(_) => {
                // Nothing to do with errors; it's not the worst disaster!
            }
        }

        // Check Instagram
        let res = self.instagram.check_username(username).await;
        // println!("inst >> {:?}", &res);
        match res {
            Ok(ua) => {
                if ua.is_registered && ua.followers_count > 100 {
                    let is_fa = is_farsi_text(&ua.fullname)
                        || is_farsi_text(&ua.about)
                        || is_farsi_text(&ua.texts);

                    if is_fa || ua.followers_count > 2000 {
                        let pr = PlatformRes {
                            platform: Platform::Instagram,
                            followers: ua.followers_count as u32,
                            farsi: is_fa,
                            fullname: ua.fullname,
                        };
                        check_out_res.registered.push(pr);
                    }
                }
            }
            Err(_) => {
                // Nothing to do with errors; it's not the worst disaster!
            }
        }

        // Check Twitter
        let res = self.twitter.check_username(username).await;
        match res {
            Ok(ua) => {
                if ua.is_registered && ua.followers_count > 100 {
                    let is_fa = is_farsi_text(&ua.fullname)
                        || is_farsi_text(&ua.about)
                        || is_farsi_text(&ua.texts);

                    if is_fa || ua.followers_count > 2000 {
                        let pr = PlatformRes {
                            platform: Platform::Twitter,
                            followers: ua.followers_count as u32,
                            farsi: is_fa,
                            fullname: ua.fullname,
                        };
                        check_out_res.registered.push(pr);
                    }
                }
            }
            Err(_) => {
                // Nothing to do with errors; it's not the worst disaster!
            }
        }

        check_out_res
    }
}

// #[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test1() {
        println!("running instagram username checker tests ...")
    }

    pub async fn play_main() {
        run_few().await;
    }

    async fn run() {
        let api = Checker::default();
        let us = vec![
            "farsna",
            "youtube",
            "mardomsara",
            "flip",
            "telegram",
            "tabnak",
            "iran_news",
            "aparat",
            "digikala",
            "snapp",
            "tapsi",
            "komodaa",
            "khamenei_ir",
            "raisi_org",
            "iran_shop",
            "varzesh3",
            "bbcpersian",
            "blogfa",
            "tehran",
            "iran",
            "tvplus",
            "digikala_lifestyle",
            "digikala.fresh",
            "digikalacom",
            "anten.ir",
        ];
        for u in us {
            let t = api.check_username(u).await;
            println!("{:} => {:#?} \n", u, t);
        }
    }

    async fn run_few() {
        let api = Checker::default();
        let us = vec!["farsna", "telegram", "oisdlj9753490534590_not_found"];
        for u in us {
            let t = api.check_username(u).await;
            println!("{:} => {:#?} \n", u, t);
        }
    }
}
