use shared;

#[tokio::main]
async fn main() {
    println!("Hi there!");
    shared::checker::twitter::main1();
}
