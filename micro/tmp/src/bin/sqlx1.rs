use sqlx::mysql::MySqlPool;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let pool = MySqlPool::connect("root:123456@tcp(37.152.187.1:3306)/twitter?charset=utf8mb4")
        .await
        .unwrap();

    sqlx::query!("insert into twitter.payment (customer_id,amount) values (44,44)")
        .execute(pool)
        .await;

    println!("> {}", pool.size())
}
