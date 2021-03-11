use grammers_client::Client;
use grammers_tl_types::Serializable;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use rand;
use rand::Rng;
use std::collections::HashMap;
use std::fs;
use std::fs::{File, read};
use std::ops::Index;
use std::sync::{Mutex, Arc};
use std::time::Duration;
use tokio::time::sleep;

use crate::{db_dep, errors::TelegramGenErr, tg, types, utils};
use crate::types::ChannelByUsernameResult;

struct Crawler {
    caller: types::TgPool,
}

impl Crawler {
    async fn new() -> Self {
        let ch = crate::con_mgr::get_new_session().await.unwrap();
        let handle = ch.handle();
        let network_handle = tokio::spawn(async move { ch.run_until_disconnected().await });
        Crawler{
            caller: types::TgPool {
                // client: Arc::new(Mutex::new(ch.handle()))
                client: handle,
            }
        }
    }

    async fn crawl_username(&self, username: &str) -> Result<(), TelegramGenErr> {

        let rpc_res = tg::get_channel_by_username(&self.caller, &username).await;

        println!("res >> {:#?}", rpc_res);

        match rpc_res {
            Ok(_) => {}
            Err(e) => {
                println!("is free username: {}", e.is_tg_username_free())
            }
        }

        Ok(())
    }

}

pub async fn crawl_next_username() -> Result<(), TelegramGenErr> {

    let crawler = Crawler::new().await;
    crawler.crawl_username("flip_group").await; // group
    crawler.crawl_username("Flip_net").await; // user
    crawler.crawl_username("jozn132523789492378").await; // user > free
    crawler.crawl_username("p0rnhub_videos").await; // channel: porn > restricted
    crawler.crawl_username("thezoomit").await; // channel

    Ok(())
}











////////////////////////////// Depreccated ////////////////

pub async fn crawl_next_username_old() -> Result<(), TelegramGenErr> {

    crawl_username("flip_group").await; // group
    crawl_username("Flip_net").await; // user
    crawl_username("jozn132523789492378").await; // user > free
    crawl_username("p0rnhub_videos").await; // channel: porn > restricted
    crawl_username("thezoomit").await; // channel

    Ok(())
}

async fn crawl_username(username: &str) -> Result<(), TelegramGenErr> {
    let mut caller = get_caller().await;

    let rpc_res = tg::get_channel_by_username_old(&mut caller, &username).await;

    println!("res >> {:#?}", rpc_res);

    match rpc_res {
        Ok(_) => {}
        Err(e) => {
            println!("is free username: {}", e.is_tg_username_free())
        }
    }


    Ok(())
}


pub async fn get_caller() -> types::Caller {
    let con = crate::con_mgr::get_new_session().await.unwrap();
    let caller = types::Caller { client: con };
    caller
}
