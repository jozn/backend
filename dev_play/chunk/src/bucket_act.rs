use crate::{sutil,types};
use tokio::io::AsyncWriteExt;
use std::io::Write;

pub async fn create_bucket(bucket_id: u32, intent: &str) -> bool {
    // Intent is not processed yet

    let path_builder = StoragePathBuilder::new(bucket_id,0);
    let path = path_builder.bucket_dir();
    if std::path::Path::new(&path).exists() {
        return false
    }
    tokio::fs::create_dir_all(&path).await;

    // Create table.json
    let f = tokio::fs::File::create(&path_builder.json_log_loc()).await.unwrap();

    // Save log create bucket
    use types::*;
    let log = LogEvent::CreateBucket(LogCreateBucket{
        bucket_id: bucket_id,
        intent: intent.to_string(),
        date: "Now Fir 23 2021".to_string()
    });
    log_event(bucket_id,&log).await;

    true
}

pub async fn save_file_into_bucket(bucket_id: u32, file_id: u64, blob: &[u8]) -> bool {
    let path_builder = StoragePathBuilder::new(bucket_id,file_id);

    if !std::path::Path::new(&path_builder.bucket_dir()).exists() {
        return false
    }
    tokio::fs::create_dir_all(&path_builder.file_sub_dir()).await;

    let mut s = tokio::fs::File::create(path_builder.file_loc()).await.unwrap();
    s.write_all(blob).await.unwrap();

    // Save log create bucket
    use types::*;
    let log = LogEvent::UploadFile(LogUploadFile{
        bucket_id,
        file_id,
        date: "Now Fir 23 2021".to_string()
    });
    log_event(bucket_id,&log).await;
    
    true
}

//// Table log Jsom
pub async fn log_event(bucket_id: u32, log: &types::LogEvent) -> bool {
    let table_path = StoragePathBuilder::new(bucket_id,0).json_log_loc();

    let log_str = serde_json::to_string(&log).unwrap();

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&table_path)
        .unwrap();
    file.write(log_str.as_bytes());
    file.write("\n".as_bytes());

    true
}

pub struct StoragePathBuilder {
    bucket_id: u32,
    file_id: u64
}

impl StoragePathBuilder {
    pub fn new(bucket_id: u32, file_id: u64) -> Self {
        StoragePathBuilder {
            bucket_id,
            file_id
        }
    }

    pub fn bucket_dir(&self) -> String {
        let folder = sutil::bucket_to_folder(self.bucket_id);
        format!("./primary/{}/{}",folder, self.bucket_id)
    }

    pub fn file_sub_dir(&self) -> String {
        let folder = self.bucket_dir();
        let file_sub_folder = sutil::file_id_to_folder(self.file_id);

        format!("{}/{}",folder, file_sub_folder)
    }

    pub fn file_loc(&self) -> String {
        let folder = sutil::bucket_to_folder(self.bucket_id);
        let file_sub_folder = sutil::file_id_to_folder(self.file_id);
        format!("./primary/{}/{}/{}/{}",folder,self.bucket_id ,file_sub_folder ,self.file_id)
    }

    pub fn json_log_loc(&self) -> String {
        let folder = sutil::bucket_to_folder(self.bucket_id);
        format!("./primary/{}/{}/log_{}.json",folder,self.bucket_id, self.bucket_id)
    }
}

// DEPRECATED CODES
//// Path builders
pub fn bucket_id_to_path_DEP(bucket_id: u32) -> String {
    let folder = sutil::bucket_to_folder(bucket_id);
    format!("./primary/{}/{}",folder, bucket_id)
}

pub fn file_id_to_dir_DEP(bucket_id: u32, file_id: u64) -> String {
    let folder = bucket_id_to_path_DEP(bucket_id);
    let file_sub_folder = sutil::file_id_to_folder(file_id);

    format!("{}/{}",folder, file_sub_folder)
}

pub fn file_id_to_file_path_DEP(bucket_id: u32, file_id: u64) -> String {
    let folder = sutil::bucket_to_folder(bucket_id);
    let file_sub_folder = sutil::file_id_to_folder(file_id);
    format!("./primary/{}/{}/{}/{}",folder,bucket_id ,file_sub_folder ,file_id)
}

pub fn json_log_file_path_DEP(bucket_id: u32) -> String {
    let folder = sutil::bucket_to_folder(bucket_id);
    format!("./primary/{}/{}/log_{}.json",folder,bucket_id, bucket_id)
}