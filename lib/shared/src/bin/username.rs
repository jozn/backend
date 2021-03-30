use shared;

/*#[tokio::main]
async fn main() {
    println!("Hi there!");
    shared::checker::twitter::main1();
}*/

#[tokio::main]
async fn main() {
    println!("Hi there!");
    shared::checker::tests::play_main().await;
    // shared::checker::telegram::tests::run();
    // shared::checker::instagram::main1();
}
