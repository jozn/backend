


mod serving {

}

struct Bucket {
    bucket_id: u32,
}

struct File22 {
    bucket_id: u32,
    file_id: u64,
    ref_id: u64,
    secret: u32,

    created_time: u32,
}

mod args {}
mod file_serving {}
// mod types {}

struct Config3{}
struct FileReq{} // file_id,...
fn sync(){}
fn upload(){}

#[derive(Default, Debug,Clone)]
pub struct Config {
    pub port: u16,
    pub dirs: Vec<String>,
    pub db_path: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Default)]
#[serde(default)]
pub struct FileReqParam {
    pub bucket_id: u32,
    pub file_id: u64,
    pub ref_id: u64,
    pub secret: u32,
    pub created_time: u32,
    pub file: String,
}
