extern crate chunk_proto;

pub mod types;
pub mod cli;
pub mod serving;
pub mod rpc_chunk;
pub mod spb;
pub mod sutil;
pub mod bucket_act;

pub use chunk_proto::storage;

// macro use should be at root
#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;
