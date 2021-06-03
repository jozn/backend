use shared;

#[tokio::main]
async fn main() {
    println!("Running db related");
    // shared::act::db_mysql::tests::play_channel1().await;
    shared::act::chat_act::tests::play1().await;
    // shared::checker::telegram::tests::run();
    // shared::checker::instagram::main1();
}
