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

pub async fn remove_file_from_bucket(bucket_id: u32, file_id: u64) -> bool {
    let path_builder = StoragePathBuilder::new(bucket_id, file_id);
    tokio::fs::create_dir_all(path_builder.file_remove_sub_dir()).await;

    tokio::fs::rename(path_builder.file_loc(), path_builder.file_remove_loc()).await;

    // Save log create bucket
    use types::*;
    let log = LogEvent::RemoveFile(LogRemoveFile{
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
        format!("./primary/{}/bucket_{}",folder, self.bucket_id)
    }

    pub fn file_sub_dir(&self) -> String {
        let folder = self.bucket_dir();
        let file_sub_folder = sutil::file_id_to_folder(self.file_id);

        format!("{}/sub_{}",folder, file_sub_folder)
    }

    pub fn file_loc(&self) -> String {
        let sub_dir = self.file_sub_dir();
        format!("{}/{}", sub_dir ,self.file_id)
    }

    pub fn json_log_loc(&self) -> String {
        let bucket_dir = self.bucket_dir();
        format!("{}/log_{}.json", bucket_dir, self.bucket_id)
    }

    // Remove

    pub fn bucket_remove_dir(&self) -> String {
        let folder = sutil::bucket_to_folder(self.bucket_id);
        format!("./primary/_remove/{}/bucket_{}",folder, self.bucket_id)
    }

    pub fn file_remove_sub_dir(&self) -> String {
        let folder = self.bucket_remove_dir();
        let file_sub_folder = sutil::file_id_to_folder(self.file_id);

        format!("{}/sub_{}",folder, file_sub_folder)
    }

    pub fn file_remove_loc(&self) -> String {
        let sub_dir = self.file_remove_sub_dir();
        format!("{}/{}", sub_dir ,self.file_id)
    }
}
