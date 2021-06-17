/*mod types;
mod cli;
mod serving;
pub mod proto_gen;

// macro use should be at root
#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;*/

use chunk::{cli, serving, rpc_chunk};

#[tokio::main]
async fn main() {
    let cfg = cli::get_cli_args();
    println!(">> Config: {:#?}", cfg);
    // pretty_env_logger::init();

    tokio::spawn( async {
        rpc_chunk::server_chunk().await;

    });

    std::thread::spawn(|| async {
        // rpc_chunk::server_chunk().await;
    });

    serving::listen_http(&cfg).await;
}




