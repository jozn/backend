extern crate shared;

use shared::{pb, rpc2};

#[tokio::main]
async fn main() {
    // client_sample_loop().await;
    client_sample().await;
}

async fn client_sample() {
    let m = shared::my::Sms {
        sms_id: 0,
        result_code: 23,
        pd_data: vec![],
        json_data: "ffsdf".to_string(),
    };

    // let database_url = "flipper:12345678@tcp(192.168.43.116:3306)/flip_my?charset=utf8mb4";
    let database_url = "mysql://flipper:12345678@192.168.43.116:3306/flip_my";

    let pool = mysql_async::Pool::new(database_url);
    // let pool = mysql::Pool::new(database_url).unwrap();
    //let mut conn = pool.get_conn().await.unwrap();

    let r = m.insert(&pool).await;
    println!("{:?}", r);
}
