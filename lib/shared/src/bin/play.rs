// extern crate shared;

use shared;

#[tokio::main]
async fn main() {
    println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");
    println!("Runned Play !");

    let mut res = shared::rpc_impl::rpc_sample::MemDb::default();

    res.build();
    println!("{:#?}", res);
}
