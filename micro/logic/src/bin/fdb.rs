extern crate logic;

use async_trait::async_trait;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use once_cell::sync::OnceCell;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};
use shared::pb::{
    ConfirmCodeParam, ConfirmCodeResponse, EchoParam, EchoResponse, SendConfirmCodeParam,
    SendConfirmCodeResponse,
};
use shared::{pb, rpc2};
use std::collections::{HashMap, HashSet};
use std::iter::Map;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::Ordering;
use std::sync::{atomic, Arc, Mutex};
use tokio::sync::mpsc::{channel, Receiver};

// use logic::UserSpaceOld;
use foundationdb;
use tokio::time::Duration;

struct LogicMicro {}

#[async_trait]
impl FIMicroService for LogicMicro {
    fn port(&self) -> u16 {
        4001
    }

    async fn serve_request(req: FHttpRequest) -> Result<FHttpResponse, GenErr> {
        println!("thread {:#?}", std::thread::current().id());
        tokio::spawn(async {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            println!(">>>>>thread {:#?}", std::thread::current());
        });
        Ok((200, b"sdfsd".to_vec()))
    }
}

#[tokio::main]
async fn main() {
    println!("Hi fdb !");

    let network = unsafe { foundationdb::boot() };

    // println!("net {:?}", networ);

    let res = async_main().await;
    println!("res {:?}", res);
    for i in 1..=10 {
        let i = i;
        tokio::spawn(async move {
            fdb_heavi_wiret(i).await
        });
    }

    tokio::time::sleep(Duration::from_secs(1000)).await;
/*    for i in 0..100_000_000 {
    // for i in 0..1{
        let key = format!("key-{}", i);

        let tx = db.create_trx().unwrap();
        tx.set(key.as_bytes(), b"for some other reason than that");
        let res = tx.commit().await.unwrap();
        if i%1000 == 0 {
            println!(" > {}", i);
        }
    }*/

}

async fn fdb_heavi_wiret(cnt: i32) {
    let db = foundationdb::Database::default().unwrap();

    for i in 0..100_000_000 {

        let tx = db.create_trx().unwrap();
        for j in 0..10 {
            let key = format!("key5-{}-{}-{}", cnt, i,j);
            tx.set(key.as_bytes(), b"for some other reason than that");
        }
        let res = tx.commit().await.unwrap();

        if i%1000 == 0 {
            println!("- {} > {} - {}",cnt, i, i*100);
        }
    }
}

async fn fdb1(cnt: i32) {
    let db = foundationdb::Database::default().unwrap();

    for i in 0..100_000_000 {
        // for i in 0..1{
        let key = format!("key4-{}-{}", cnt, i);

        let tx = db.create_trx().unwrap();
        tx.set(key.as_bytes(), b"for some other reason than that");
        let res = tx.commit().await.unwrap();
        if i%1000 == 0 {
            println!("- {} > {} - {}",cnt, i, i*cnt);
        }
    }
}

async fn async_main() -> foundationdb::FdbResult<()> {
    let db = foundationdb::Database::default()?;

    // write a value
    let trx = db.create_trx()?;
    trx.set(b"hello", b"world"); // errors will be returned in the future result
    trx.commit().await?;

    // read a value
    let trx = db.create_trx()?;
    let maybe_value = trx.get(b"hello", false).await?;
    let value = maybe_value.unwrap(); // unwrap the option

    assert_eq!(b"world", &value.as_ref());

    Ok(())
}