use crate::man::errors::GenErr;

// static

pub fn get_channel() {}

pub fn get_next_cid_(intent: &str) -> Result<u64, GenErr> {
    let user_cid_row = shared::my::GenCid {
        cid: 0,
        intent: intent.to_string(),
    };
    let user_cid = user_cid_row.insert(&self.mysql_pool).await?.cid;
}
