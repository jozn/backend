
struct Bucket {
    bucket_id: u32,
    intent: String,
    create_time_str: String,
}

fn sync(){}
fn upload(){}

#[derive(Default, Debug,Clone)]
pub struct Config {
    pub serving_port: u16,
    pub grpc_port: u16,
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

// logs
#[derive(Deserialize, Serialize, Debug, PartialEq, Default)]
#[serde(default)]
pub struct LogCreateBucket {
    pub bucket_id: u32,
    pub intent: String,
    pub date: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Default)]
#[serde(default)]
pub struct LogUploadFile {
    pub bucket_id: u32,
    pub file_id: u64,
    // pub ref_id: u64,
    // pub secret: u32,
    pub date: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Default)]
#[serde(default)]
pub struct LogRemoveFile {
    pub bucket_id: u32,
    pub file_id: u64,
    pub ref_id: u64,
    pub secret: u32,
    pub date: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum LogEvent {
    CreateBucket(LogCreateBucket),
    UploadFile(LogUploadFile),
    RemoveFile(LogRemoveFile),
}