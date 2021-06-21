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

    serving::listen_http(&cfg).await;
}



