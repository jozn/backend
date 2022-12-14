use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Act {
    method: u32,
    data: Vec<u8>,
    act_id: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckUsernameParam {
    id: u64,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    me1().await
}

async fn me1() -> Result<(), reqwest::Error> {
    let act = Act {
        method: 45,
        // data: "{som:'sdf'}".as_bytes().to_owned(),
        data: bincode::serialize(&CheckUsernameParam{id:1894}).unwrap(),
        act_id: 15
    };
    let bts = bincode::serialize(&act).unwrap();

    let new_post = reqwest::Client::new()
        .post("http://127.0.0.1:3000/rpc")
        // .json(&act)
        .body(bts)
        .send()
        .await?;

    println!("{:#?}", new_post);
    println!("body {:#?}", new_post.bytes().await);
    Ok(())
}

async fn sample1() -> Result<(), reqwest::Error> {
    let new_post = Post {
        id: None,
        title: "Reqwest.rs".into(),
        body: "https://docs.rs/reqwest".into(),
        user_id: 1,
    };
    let new_post: Post = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_post);
    // Post {
    //     id: Some(
    //         101
    //     ),
    //     title: "Reqwest.rs",
    //     body: "https://docs.rs/reqwest",
    //     user_id: 1
    // }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}
