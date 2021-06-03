extern crate tmp;

use mysql_async::prelude::*;
use mysql_async::{FromRowError, OptsBuilder, Params, Row};
use mysql_common::row::ColumnIndex;

use tmp::my_dev;
use tmp::my_dev::tweet::Tweet;
use crate::tmp::mysql_shared;

#[tokio::main]
async fn main() {
    let s = my_dev::tweet::Tweet{
        ..Default::default()
    };

    let database_url = "mysql://flipper:12345678@192.168.92.115:3306/twitter";
    let pool = mysql_async::Pool::new(database_url);
    let spool = mysql_shared::SPool{ pool, database: "twitter".to_string() };

    let mut t = my_dev::tweet::Tweet{
        tweet_id: 1,
        user_id: 12,
        created_time: 234,
        text_body: "sdfsdfdsfsdfsdf".to_string()
    };
    t.insert(&spool).await;


    let arr = vec![Tweet{
        tweet_id: 0,
        user_id: 100,
        created_time: 1,
        text_body: "11".to_string()
    }, Tweet{
        tweet_id: 0,
        user_id: 101,
        created_time: 2,
        text_body: "22".to_string()
    }];

    my_dev::tweet::tweet_mass_insert(&arr,&spool).await.unwrap();

    let mut t = my_dev::tweet::TweetSelector::new();
    t.select_all().tweet_id_eq(234).and_created_time_eq(234).get_rows(&spool).await;

    let mut u = my_dev::tweet::TweetUpdater::new();
    u.set_text_body("werwer").user_id_eq(12).limit(4).order_by_user_id_desc().update(&spool).await;


    let m = my_dev::tweet::tweets_of_user(100,&spool).await;
    println!("{:?}", m)

    // t.select_all().hash_code_eq("sdf").and_sms_id_ge(324).get_rows(&pool).await.unwrap();

    // t.select_all().sms_id_in().sms_id_ge()
}
