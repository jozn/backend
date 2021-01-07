use shared::{pb,errors::GenErr};
use async_trait::async_trait;


pub async fn get_chat() -> Result<pb::Channel,GenErr>{

    GenErr(GenErr::NoRpcMatch)
}

#[async_trait]
pub trait DBStore {
    async fn get(key: &str) -> Result<Vec<u8>, GenErr>;
    async fn delete(key: &str) -> Result<Vec<u8>, GenErr>;
    async fn put(key: &str, value: Vec<u8>) -> Result<Vec<u8>, GenErr>;
    //range
    async fn get_range(keys_prefix: &str) -> Result<Vec<Vec<u8>>, GenErr>;
    async fn delete_range(keys_prefix: &str) -> Result<Vec<Vec<u8>>, GenErr>;

    async fn put_many(pairs: (&str,Vec<u8>)) -> Result<Vec<u8>, GenErr>;
}

mod sdf {
    const chat: &str ="user/12/chats/234";
    const chat2: &str ="user/12/chat_msg/234/1543221";
    const chat3: &str ="user/12/chat_msg/234/1543221";
}