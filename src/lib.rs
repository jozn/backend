#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

pub mod com;
pub mod pb;
pub mod rpc;
pub mod rpc_fns;
pub mod rpc_fns_default;
pub mod rpc_impl;
pub mod sms_sender;
pub mod mock;

pub mod errors;

use errors::GenErr;

pub mod utils;

pub use utils::id_gen;
