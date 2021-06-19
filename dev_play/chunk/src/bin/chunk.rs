/*mod types;
mod cli;
mod serving;
pub mod proto_gen;

// macro use should be at root
#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;*/

use chunk::{cli, serving, rpc_chunk, types};
use chunk::types::LogEvent::CreateBucket;

#[tokio::main]
async fn main() {
    let cfg = cli::get_cli_args();
    println!(">> Config: {:#?}", cfg);
    // pretty_env_logger::init();

    let grpc_port = cfg.grpc_port;
    tokio::spawn( async move {
        rpc_chunk::server_chunk(grpc_port).await;
    });

    std::thread::spawn(|| async {
        // rpc_chunk::server_chunk().await;
    });

    play();

    serving::listen_http(&cfg).await;
}

fn play(){
    use types::*;
    let s = types::LogEvent::CreateBucket(LogCreateBucket{
        bucket_id: 23,
        intent: "23".to_string(),
        date: "23".to_string()
    });

    let d = serde_json::to_string(&s).unwrap();
    println!("//> {}", d);
}




