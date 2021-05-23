use async_trait::async_trait;
use shared;
use shared::errors::GenErr;
use shared::{pb, rpc2};

use crate::UserSpace;

#[async_trait]
impl rpc2::RPC_Shared_Handler2 for UserSpace {
    async fn SharedEcho(
        &self,
        param: pb::SharedEchoParam,
    ) -> Result<pb::SharedEchoResponse, GenErr> {
        Ok(pb::SharedEchoResponse {
            done: true,
            text: format!("Echoing:: version 2 >>> {}", param.text),
        })
    }
}
