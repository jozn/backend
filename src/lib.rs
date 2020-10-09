#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use quick_protobuf::{BytesReader, BytesWriter};
use quick_protobuf::{MessageRead, MessageWrite, Writer};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::net::SocketAddr;

pub mod com;
pub mod pb;
pub mod rpc;
pub mod rpc_fns;
pub mod rpc_fns_default;

use pb::pb::sys::*;
use prost::Message;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Inv {
    pub method: u32,
    pub action_id: u64,
    pub is_response: bool,
    pub rpc_data: Vec<u8>,
}

pub struct GenErr {}
pub struct UserParam {}
