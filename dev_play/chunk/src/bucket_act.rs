use crate::sutil;
use tokio::io::AsyncWriteExt;

pub async fn create_bucket(bucket_id: u32, intent: &str) -> bool {
    // Not used
    let intent = if intent.len() == 0 {
        "def"
    } else {
        intent
    };

    let folder = sutil::bucket_to_folder(bucket_id);

    let path = format!("./primary/{}/{}",folder, bucket_id);
    tokio::fs::create_dir_all(path).await;

    true
}

pub async fn save_file_inot_bucket(bucket_id: u32, file_id: u64, blob: &[u8]) -> bool {
    let s = bucket_id_to_path(bucket_id);
    if !std::path::Path::new(&s).exists() {
       // return Err(Status::new(Code::NotFound, "bucket does not exist."))
        return false
    }

    let pp = file_id_to_dir(bucket_id, file_id);
    tokio::fs::create_dir_all(&pp).await;

    let file_path = format!("{}/{}",pp, file_id);
    let mut s = tokio::fs::File::create(file_path).await.unwrap();
    s.write_all(blob).await.unwrap();

    true
}

pub fn bucket_id_to_path(bucket_id: u32) -> String {
    let folder = sutil::bucket_to_folder(bucket_id);
    format!("./primary/{}/{}",folder, bucket_id)
}

pub fn file_id_to_dir(bucket_id: u32, file_id: u64) -> String {
    let folder = bucket_id_to_path(bucket_id);
    let file_sub_folder = sutil::file_id_to_folder(file_id);

    format!("{}/{}",folder, file_sub_folder)
}

pub fn file_id_to_file_path(bucket_id: u32, file_id: u64) -> String {
    let folder = sutil::bucket_to_folder(bucket_id);
    let file_sub_folder = sutil::file_id_to_folder(file_id);
    format!("./primary/{}/{}/{}/{}",folder,bucket_id ,file_sub_folder ,file_id)
}