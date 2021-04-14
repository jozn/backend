extern crate logic;

use async_trait::async_trait;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use once_cell::sync::OnceCell;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};

use shared::{pb, rpc2};

// use logic::UserSpaceOld;

struct LogicMicro {}

#[async_trait]
impl FIMicroService for LogicMicro {
    fn port(&self) -> u16 {
        4001
    }

    async fn serve_request(req: FHttpRequest) -> Result<FHttpResponse, GenErr> {
        // println!("thread {:#?}", std::thread::current().id());

        let invoke: pb::Invoke = prost::Message::decode(req.body)?;

        let m = INSTANCE.get_or_init(|| logic::UserSpaceMapper::new());
        // let mut m = logic::UserSpaceMapper::new();
        let res_v8 = m.serve_rpc_request(invoke).await?;

        let mut vec = vec![];
        prost::Message::encode(&res_v8, &mut vec)?;

        Ok((200, vec))
    }
}

static INSTANCE: OnceCell<logic::UserSpaceMapper> = OnceCell::new();

#[tokio::main]
async fn main() {
    println!("Starting logic1 ... ");

    let c = LogicMicro {
        //handler: Arc::new(handler),
    };
    c.listen_http_requests().await;
}
