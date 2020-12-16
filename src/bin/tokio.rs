use tokio::stream::StreamExt;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
// use std::thread::sleep;

#[tokio::main]
async fn main() {
    let (mut txd, mut rx) = mpsc::channel(10);

    let tm = txd.clone();
    let tx = txd.clone();
    tokio::spawn(async move {
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        tx.send(3).await.unwrap();
        for i in 0..1000 {
            sleep(Duration::from_millis(100)).await;
            tx.send(i).await;
        }
    });

    for i in 0..1000 {
        let m = i;
        let tz = txd.clone();
        tokio::spawn(async move {
            for i in 0..100_000 {
                sleep(Duration::from_millis(1)).await;
                tz.send(m).await.unwrap();
                if i % 50 == 0 {
                    // println!("t {} {}", m, i);
                }
            }
        });
    }

    let mut c = 0;
    while let Some(v) = rx.next().await {
        if v == 300 {
            c += 1;
            println!("GOT = {:?}", c);
        }
    }
}
