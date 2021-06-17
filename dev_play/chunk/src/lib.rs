pub mod types;
pub mod cli;
pub mod serving;
pub mod proto_gen;
pub mod rpc_chunk;
mod spb;
mod sutil;

pub use proto_gen::{storage};

// macro use should be at root
#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;

pub struct GG{}