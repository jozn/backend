use grammers_client::{Client, Config};
// use grammers_mtproto::errors::RpcError;
use grammers_mtsender::InvocationError;
use rusqlite;
// use serde::export::Formatter;
use grammers_mtsender::AuthorizationError;
use serde::__private::Formatter;
use shared::gen::my::models::MyError;
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
    /*    pub fn is_tg_not_found(&self) -> bool {
        match self {
            GenErr::TgRPC(rpc) => {
                if rpc.code == 400 {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    */
    pub fn is_tg_username_free(&self) -> bool {
        use TelegramGenErr::*;
        match &self {
            TgInvocation(invocation) => match invocation {
                InvocationError::Rpc(rpc_err) => {
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
// impl Display for GenErr {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         use GenErr::*;
//         write!(f, "Gen err: {:?}", self)
//     }
// }
/*impl Display for GenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use GenErr::*;
        // match *self {
        //
        // };
        // let s = format!("{}")
        write!(f, "Gen err: {:?}", self)
    }
}*/

// impl From<AuthorizationError> for GenErr {
//     fn from(tg_err: AuthorizationError) -> Self {
//         use AuthorizationError::*;
//         match &tg_err {
//             Invocation(inv) => match inv {
//                 InvocationError::RPC(rpc) => GenErr::TgRPC(rpc.clone()),
//                 _ => GenErr::TgConnection,
//             },
//             IO(io) => GenErr::TgAuth(tg_err),
//             Gen(gen) => GenErr::TgAuth(tg_err),
//         }
//     }
// }

impl From<InvocationError> for TelegramGenErr {
    fn from(inv: InvocationError) -> TelegramGenErr {
        TelegramGenErr::TgInvocation(inv)
        /* match inv {
            // InvocationError::RPC => GenErr::TgRPC,//(rpc.clone()),
            _ => TelegramGenErr::TgConnection,
        }*/
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
