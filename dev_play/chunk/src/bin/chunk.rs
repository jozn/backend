/*mod types;
mod cli;
mod serving;
pub mod proto_gen;

// macro use should be at root
#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;*/

use chunk::{cli,GG,serving};

#[tokio::main]
async fn main() {
    let cfg = cli::get_cli_args();
    println!(">> Config: {:#?}", cfg);
    // pretty_env_logger::init();

    serving::listen_http(&cfg).await;
}




