// extern crate shared2;

use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::new_rpc::{FHttpRequest, FHttpResponse, FIMicroService};
use shared::pb::{
    ConfirmCodeParam, ConfirmCodeResponse, EchoParam, EchoResponse, SendConfirmCodeParam,
    SendConfirmCodeResponse,
};
use shared::{pb, rpc2};
use std::sync::{atomic, Arc};
use once_cell::sync::OnceCell;
use std::sync::atomic::Ordering;
use rocksdb;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

static HANDLER_INSTANCE: OnceCell<Arc<DB>> = OnceCell::new();


#[derive(Debug)]
struct DB {
    next: atomic::AtomicU64,
    rocks: rocksdb::DB,
}

#[derive(Debug)]
struct CMasterHandler {
}

#[async_trait]
impl rpc2::IPC_CMaster_Handler2 for CMasterHandler {
    async fn GetNextId(&self, param: pb::GetNextIdParam) -> Result<pb::GetNextIdResponse, GenErr> {
        let db = HANDLER_INSTANCE.get().unwrap();
        let id = db.next.fetch_add(1,Ordering::SeqCst);

        let mut wtr = vec![];
        wtr.write_u64::<byteorder::BigEndian>(id);
        // let m = db.rocks.put(param.key.clone(), wtr);

        if id % 1000 == 0  {
            println!("called GetNextId {}", id);
        }

        let m = db.rocks.get(param.key.clone());
        // println!("{:#?}", m);
        let b = bytes::Bytes::from(m.unwrap().unwrap());
        let mut cur = std::io::Cursor::new(b.to_vec());
        cur.read_u64::<byteorder::BigEndian>();

        Ok(pb::GetNextIdResponse{
            next_id: id,// shared::common::get_random_u64(),
            error: false
        })
    }
}

struct CMaster {
    //handler: Arc<DB>,
}

#[async_trait]
impl FIMicroService for CMaster {
    fn port(&self) -> u16 {
        4004
    }

    async fn serve_request(req: FHttpRequest) -> Result<FHttpResponse, GenErr> {
        let h = HANDLER_INSTANCE.get().unwrap().clone();
        let handler = CMasterHandler {
        };
        let reg = shared::rpc2::RPC_Registry {
            IPC_CMaster: Some(Box::new(handler)),
            ..Default::default()
        };

        shared::common::rpc_handle_registry(&reg, req).await
    }
}

#[tokio::main]
async fn main() {
    println!("Hi cmaster1 !");

    let handler = DB {
        next: atomic::AtomicU64::new(5000000),
        rocks: rocksdb::DB::open_default("./rocks1.db").unwrap()
    };

    HANDLER_INSTANCE.set(Arc::new(handler));

    let c = CMaster {
        //handler: Arc::new(handler),
    };
    c.listen_http_requests().await;
}
