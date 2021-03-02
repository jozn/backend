pub mod pb;
// pub mod pb;
pub mod common;
pub mod errors;
pub mod rpc2;
pub mod xc;

pub struct UserParam {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
