use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::body;
use serde::{Deserialize, Serialize};
use quick_protobuf::{BytesReader, BytesWriter};
use quick_protobuf::{MessageRead,MessageWrite,Writer};

use crate::{pb,com, pb::sys::Invoke,com::*};

pub fn server_rpc(act : Invoke) -> Result<Vec<u8>,GenErr> {
    let up = UserParam{};

    match act.method {
    
    // service: RPC_Auth
        939965206 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::SendConfirmCodeParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::SendConfirmCode(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1740258084 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::ConfirmCodeParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::ConfirmCode(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        291193302 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::SingUpParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::SingUp(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1017957090 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::SingInParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::SingIn(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1283119009 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::LogOutParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::LogOut(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
    
    // service: RPC_General
        101973561 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::EchoParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::Echo(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
        1897027349 => {
            let vec = "funk ".as_bytes().to_owned();

            let mut reader = BytesReader::from_bytes(&act.rpc_data);
            let rpc_param= pb::CheckUserNameParam::from_reader(&mut reader, &act.rpc_data);

            if let Ok(param) = rpc_param {
            println!("param {:?}", param);
            let result = rpc::CheckUserName(&up,param)?;

            let mut out_bytes = Vec::new();
            let mut writer = Writer::new(&mut out_bytes);
            let out = writer.write_message(&result);
            return Ok(out_bytes)
            } else {
            }
            Ok(vec)
        },
    
        _ => {
           Err(GenErr{})
        }
    }
}

pub mod rpc {
    use super::*;

    
    // service: RPC_Auth
    pub fn SendConfirmCode(up: &UserParam, param: pb::SendConfirmCodeParam) -> Result<pb::SendConfirmCodeResponse, GenErr> {
        Ok(pb::SendConfirmCodeResponse::default())
    }
    pub fn ConfirmCode(up: &UserParam, param: pb::ConfirmCodeParam) -> Result<pb::ConfirmCodeResponse, GenErr> {
        Ok(pb::ConfirmCodeResponse::default())
    }
    pub fn SingUp(up: &UserParam, param: pb::SingUpParam) -> Result<pb::SingUpResponse, GenErr> {
        Ok(pb::SingUpResponse::default())
    }
    pub fn SingIn(up: &UserParam, param: pb::SingInParam) -> Result<pb::SingInResponse, GenErr> {
        Ok(pb::SingInResponse::default())
    }
    pub fn LogOut(up: &UserParam, param: pb::LogOutParam) -> Result<pb::LogOutResponse, GenErr> {
        Ok(pb::LogOutResponse::default())
    }
    
    // service: RPC_General
    pub fn Echo(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
    pub fn CheckUserName(up: &UserParam, param: pb::CheckUserNameParam) -> Result<pb::CheckUserNameResponse, GenErr> {
        Ok(pb::CheckUserNameResponse::default())
    }
    

    pub fn check_username(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
}
