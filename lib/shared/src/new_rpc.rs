use crate::pb;
use crate::pb::{EchoParam, EchoResponse};
use crate::{errors::GenErr, UserParam};
use async_trait::async_trait;

#[async_trait]
trait RPC_Shared {
    async fn Echo(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
}

#[async_trait]
trait RPC_Chat {
    async fn ChatSendMsg(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
}

#[async_trait]
trait RPC_Chat2_Trait {
    async fn ChatSendMsg(
        &self,
        up: &UserParam,
        param: pb::EchoParam,
    ) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
}

struct Rpc {
    RPC_Chat2: Box<dyn RPC_Chat2_Trait>,
    RPC_Chat3: Option<Box<dyn RPC_Chat2_Trait>>,
}

impl Rpc {
    pub fn new(RPC_Chat2: Box<dyn RPC_Chat2_Trait>) -> Self {
        Rpc { RPC_Chat2: RPC_Chat2 , RPC_Chat3: None }
    }
}

struct Ut {
    s: String,
}

impl Ut {
    pub fn serve(data: Vec<u8>) {

    }
}

impl RPC_Shared for Ut{
    async fn Echo(up: &UserParam, param: EchoParam) -> Result<EchoResponse, GenErr> {
        unimplemented!()
    }
}

#[async_trait]
impl RPC_Chat2_Trait for Ut {
    async fn ChatSendMsg(&self, up: &UserParam, param: EchoParam) -> Result<EchoResponse, GenErr> {

    }
}
