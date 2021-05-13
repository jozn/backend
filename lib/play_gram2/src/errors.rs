use grammers_mtsender::AuthorizationError;
use grammers_mtsender::InvocationError;
use rusqlite; //todo remove
use serde::__private::Formatter;
use shared::gen::my::mysql_models::MyError;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum TelegramGenErr {
    DB(rusqlite::Error),
    Io,
    Download,
    // TgRPC(RpcError),
    TgRPC,
    TgInvocation(InvocationError),
    TgConnection,
    TgAuth(AuthorizationError),
    TgConverter,
    JSON(serde_json::Error),
    MySql(MyError),

    BadParam,
}

// impl Error for GenErr {}

impl TelegramGenErr {
    pub fn is_tg_username_free(&self) -> bool {
        use TelegramGenErr::*;
        match &self {
            TgInvocation(invocation) => match invocation {
                InvocationError::Rpc(rpc_err) => {
                    // code 400 seems to be returned for many things.
                    if rpc_err.code == 400 && rpc_err.name == "USERNAME_NOT_OCCUPIED" {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            _ => false,
        }
    }
}

impl From<InvocationError> for TelegramGenErr {
    fn from(inv: InvocationError) -> TelegramGenErr {
        TelegramGenErr::TgInvocation(inv)
    }
}

impl From<rusqlite::Error> for TelegramGenErr {
    fn from(e: rusqlite::Error) -> TelegramGenErr {
        TelegramGenErr::DB(e)
    }
}

impl From<serde_json::Error> for TelegramGenErr {
    fn from(e: serde_json::Error) -> TelegramGenErr {
        TelegramGenErr::JSON(e)
    }
}

impl From<MyError> for TelegramGenErr {
    fn from(e: MyError) -> TelegramGenErr {
        TelegramGenErr::MySql(e)
    }
}
