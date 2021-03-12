#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

#[macro_use]
extern crate lazy_static;

use async_std::task;
use grammers_client::{Client, Config};
use grammers_session as session;
use grammers_tl_types as tl;
use grammers_tl_types::enums::messages::Messages;
use grammers_tl_types::enums::{Message, MessageEntity};

// mod client_pool;
mod types;

use std::cell::*;

mod con_mgr;
// mod crawl;
mod db_dep;
mod errors;
// mod tg_old;
mod tg;
mod utils;
mod crawler_new;

//tasks:
// flooding tests and policy

#[tokio::main]
async fn main() {

    // db_dep::play_my().await;
    crawler_new::crawl_next_username().await;



    // crawl::crawl_next_username().await;
    // crawl::crawl_config().await;
    // crawl::crawl_next_channel().await;

    // tg::get_file()
/*    for i in 0..1 {
        println!("{}", 1);
        // crawl::crawl_next_username().await;
        let r = crawl::crawl_next_channel_messages().await;
        println!("{} {:?}", i, r);
    }*/
}
