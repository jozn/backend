use shared;

/*#[tokio::main]
async fn main() {
    println!("Hi there!");
    shared::checker::twitter::main1();
}*/

fn main() {
    println!("Hi there!");
    shared::checker::tests::run();
    // shared::checker::telegram::tests::run();
    // shared::checker::instagram::main1();
}
