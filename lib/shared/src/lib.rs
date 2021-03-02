#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

// pub mod errors;
// pub mod new_rpc;
// // pub mod pb;
// // pub mod rpc2;
// // pub mod rpc;
// // mod rpc_fns_default;
// // mod play;
// // pub mod rpc;
// pub mod common;
// pub mod sms_sender;
// pub mod utils;
//
// pub mod aof;
pub mod gen;
pub mod man;

pub use gen::{pb, rpc2, xc};
pub use man::*;

pub struct UserParam {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
