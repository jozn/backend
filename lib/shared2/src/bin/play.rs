// extern crate shared2;

use shared2;

#[tokio::main]
async fn main() {
    println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");
    println!("Runned Play !");

    let mut res = shared2::rpc_impl::rpc_sample::MemDb::default();

    res.build();
    println!("{:#?}", res);
}
