use grammers_client::Client;
use grammers_tl_types::Serializable;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use rand;
use rand::Rng;
use std::collections::HashMap;
use std::fs;
use std::fs::{read, File};
use std::ops::Index;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

use crate::types::ChannelInfoCompact;
use crate::{db_dep, errors::TelegramGenErr, tg, types, utils};

struct Crawler {
    caller: types::TgPool,
    mysql_pool: mysql_async::Pool,
}

impl Crawler {
    async fn new() -> Self {
        let ch = crate::con_mgr::get_new_session().await.unwrap();
        let handle = ch.handle();
        let network_handle = tokio::spawn(async move { ch.run_until_disconnected().await });

        // Mysql
        let path = "mysql://root:123456@37.152.187.1:3306/flip_tg";
        let pool = mysql_async::Pool::from_url(path).unwrap();

        Crawler {
            caller: types::TgPool {
                // client: Arc::new(Mutex::new(ch.handle()))
                client: handle,
            },
            mysql_pool: pool,
        }
    }

    async fn crawl_config(&self) -> Result<(), TelegramGenErr> {
        println!("Getting config ... ");
        let res = tg::get_config(&self.caller).await;
        println!("res Config >> {:#?}", res);

        Ok(())
    }

    async fn crawl_username(&self, username: &str) -> Result<(), TelegramGenErr> {
        let rpc_res = tg::get_channel_by_username(&self.caller, &username).await;

        // println!("res >> {:#?}", rpc_res);

        match rpc_res {
            Ok(c) => {
                let i = tg::get_channel_info(&self.caller, c.id, c.access_hash).await?;

                // todo: eighter extract this to retain CahnnelData already saved or make a new table

                let channel_data = types::ChannelData {
                    channel_info: i.clone(),
                    last_checked: 1, //todo
                };

                let data = serde_json::to_string(&channel_data).unwrap();

                let channel_row = shared::my::models::TgChannel {
                    channel_id: i.id as u32,
                    username: i.username.clone(),
                    data: data,
                };

                channel_row.replace(&self.mysql_pool).await?;
                println!("+> crawl_username '@{}' was successful.", &username);
            }
            Err(e) => {
                println!("is free username: {}", e.is_tg_username_free());
                return Err(e);
            }
        }

        Ok(())
    }

    async fn crawl_next_channel_messages(&self) -> Result<(), TelegramGenErr> {
        //todo
        let rows = shared::my::models::TgChannelSelector::new()
            .get_rows(&self.mysql_pool)
            .await?;

        for row in rows {
            let chan: types::ChannelData = serde_json::from_str(&row.data).unwrap();
            let i = chan.channel_info;
            let req = tg::ReqGetMessages {
                channel_id: i.id,
                access_hash: i.access_hash,
                offset_id: 0,
                offset_date: 0,
                add_offset: 0,
                limit: 5,
                max_id: 0,
                min_id: 0,
                hash: 0,
            };

            let rpc_res = tg::get_messages(&self.caller, req).await;
            println!("channel: {:#?} \n\n  res >> {:#?}", i, rpc_res);
        }

        Ok(())
    }
}

pub async fn crawl_run() -> Result<(), TelegramGenErr> {
    let crawler = Crawler::new().await;
    // crawler.crawl_username("thezoomit").await;
    // crawler.crawl_username("boursecampaign").await;
    // crawler.crawl_username("flip_app").await;
    crawler.crawl_username("flip_info").await;
    crawler.crawl_next_channel_messages().await; // channel: porn > restricted
                                                 // println!("zoomit {:?}",res);
    Ok(())
}

pub async fn crawl_next_username2() -> Result<(), TelegramGenErr> {
    let crawler = Crawler::new().await;
    // crawler.crawl_username("flip_group").await; // group
    // crawler.crawl_username("Flip_net").await; // user
    // crawler.crawl_username("jozn132523789492378").await; // user > free
    crawler.crawl_username("p0rnhub_videos").await; // channel: porn > restricted
    let res = crawler.crawl_username("thezoomit").await; // channel
    println!("zoomit {:?}", res);

    // crawler.crawl_config().await;

    Ok(())
}
