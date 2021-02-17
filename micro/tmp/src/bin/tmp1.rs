use mysql_async::prelude::*;
use mysql_async::{FromRowError, OptsBuilder, Params, Row};
use mysql_common::row::ColumnIndex;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Pay {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

impl FromRow for Payment {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        Ok(Payment {
            customer_id: row.get(1).unwrap(),
            amount: row.get(1).unwrap(),
            account_name: row.get(2).unwrap(),
        })
    }
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let payments = vec![
        Payment {
            customer_id: 1,
            amount: 2,
            account_name: None,
        },
        Payment {
            customer_id: 3,
            amount: 4,
            account_name: Some("foo".into()),
        },
        Payment {
            customer_id: 5,
            amount: 6,
            account_name: None,
        },
        Payment {
            customer_id: 7,
            amount: 8,
            account_name: None,
        },
        Payment {
            customer_id: 9,
            amount: 10,
            account_name: Some("bar".into()),
        },
    ];

    let mut database_url = OptsBuilder::default();

    let db_url = database_url
        .user(Some("root"))
        .pass(Some("123456"))
        .db_name(Some("twitter"))
        .ip_or_hostname("37.152.187.1");

    let pool = mysql_async::Pool::new(db_url);
    // let pool = mysql_async::Pool::from_url("root:123456@tcp(37.152.187.1:3306)/twitter?charset=utf8mb4").unwrap();
    let mut conn = pool.get_conn().await.unwrap();

    // Create temporary table
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS payment (
        customer_id int not null,
        amount int not null,
        account_name text
    )",
    )
    .await
    .unwrap();

    // Save payments
    let params = payments.clone().into_iter().map(|payment| {
        params! {
            "customer_id" => payment.customer_id,
            "amount" => payment.amount,
            "account_name" => payment.account_name,
        }
    });

    let p = Params::Positional(vec![52.into(), 44.into(), "for your".into()]);

    conn.exec_drop(
        r"insert INTO payment (customer_id, amount, account_name) values(?,?,?)",
        p,
    )
    .await
    .unwrap();

    conn.exec_batch(
        r"INSERT INTO payment (customer_id, amount, account_name)
      VALUES (:customer_id, :amount, :account_name)",
        params,
    )
    .await
    .unwrap();

    // Load payments from database. Type inference will work here.
    let loaded_payments = conn
        .exec_map(
            "SELECT customer_id, amount, account_name FROM payment",
            (),
            |(customer_id, amount, account_name)| Payment {
                customer_id,
                amount,
                account_name,
            },
        )
        .await
        .unwrap();

    /*    let loaded_payments2 : Vec<Payment> = conn.exec_map(
        "SELECT * FROM payment",
        (),
        |(f)| { println!("{:?}",f); Payment{
            customer_id: 0,
            amount: 0,
            account_name: None
        }},
    ).await.unwrap();*/
    // Dropped connection will go to the pool
    // conn;

    // Pool must be disconnected explicitly because
    // it's an asynchronous operation.
    // pool.disconnect().await.unwrap();

    // assert_eq!(loaded_payments, payments);

    let loaded_payments = conn
        .exec_map(
            "SELECT customer_id, amount, account_name FROM payment",
            (),
            |f: Payment| f,
        )
        .await;

    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>..");
    println!("{:?}", loaded_payments);

    // the async fn returns Result, so
}
