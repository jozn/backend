// extern crate backbone;

use backbone;

#[tokio::main]
async fn main() {
    println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");
    println!("Runned Play !");

    let mut res = backbone::rpc_impl::rpc_sample::MemDb::default();

    res.build();
    println!("{:#?}", res);

}

