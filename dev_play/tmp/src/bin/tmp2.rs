extern crate tmp;

use mysql_async::prelude::*;
use mysql_async::{FromRowError, OptsBuilder, Params, Row};
use mysql_common::row::ColumnIndex;

use tmp::models;

#[tokio::main]
async fn main() {
    let t = models::Tweet_DEP {
        tweet_id: 1987,
        created_time: 444,
        text_body: "sfds".to_string(),
    };

    let db_url = OptsBuilder::default()
        .user(Some("root"))
        .pass(Some("123456"))
        .db_name(Some("twitter"))
        .ip_or_hostname("37.152.187.1");

    let pool = mysql_async::Pool::new(db_url);

    for _ in 1..=10 {
        let o = t.insert(&pool).await;

        println!(">>> {:?}", o);

        let mut o = o.unwrap();

        if o.tweet_id % 2 == 0 {
            o.text_body = "New Sting".to_string();
            o.created_time = 555555;

            o.update(&pool).await.unwrap();
            //  o.delete(&pool).await;
        }

        if o.tweet_id % 5 == 0 {
            o.delete(&pool).await;
        }
    }

    //Selector
    for _ in 1..=10 {
        let mut s = models::TweetSelector::new();
        s.select_all()
            .tweet_id_eq(7)
            .and_created_time_lt(7)
            .limit(8)
            .offset(45)
            .order_by_tweet_id_desc()
            .get_rows(&pool);
    }
}
