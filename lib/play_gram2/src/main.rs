#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

mod con_mgr;
mod crawler;
mod db_dep;
mod errors;
mod tg;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    crawler::crawl_run().await;
}
