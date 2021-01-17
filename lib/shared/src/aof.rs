use byteorder::{ByteOrder, BE};
use prost::Message;
use std::io::Write;

// AOF = Append Only File

const DEFAULT_DATA_DIR: &str = "/opt/flip/";
const DEFAULT_DATA_NAMESPACE: &str = "def";

#[derive(Clone)]
pub struct AofFile {
    pub namespace: String,
    pub shared: u32,
    pub file_name: String,
    pub directory: String,
}

impl AofFile {
    pub fn new() -> Self {
        Self::new_with(DEFAULT_DATA_NAMESPACE, 1)
    }

    pub fn new_with(namespace: &str, shared: u32) -> Self {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("{}_{}_{}.aof", namespace, shared, time);

        // std::fs::create_dir(DEFAULT_DATA_DIR).unwrap();
        std::fs::create_dir(DEFAULT_DATA_DIR); //todo fix this

        AofFile {
            namespace: namespace.to_string(),
            shared: shared,
            file_name: filename,
            directory: DEFAULT_DATA_DIR.to_string(),
        }
    }

    pub fn get_file_handler(&self) -> AofFileHandler {
        // todo add a box to set flag if we already had run
        AofFileHandler::new_with(self)
    }

    fn full_path(&self) -> String {
        format!("{}{}", self.directory, self.file_name)
    }
}

pub struct AofFileHandler {
    pub aof_file: AofFile,
    pub sender: std::sync::mpsc::SyncSender<HandlerCommand>,
    pub join: std::thread::JoinHandle<()>,
}

pub enum HandlerCommand {
    Append(AofRow),
}

impl AofFileHandler {
    pub fn new_with(aof_file: &AofFile) -> Self {
        let mut file_handler = std::fs::OpenOptions::new()
            .create_new(true) // New file in each run (for now)
            .write(true)
            .read(true)
            .open(aof_file.full_path())
            .unwrap();

        // unbufferd channel is considerably faster as it just pushes data to buffer and thread
        // continues to work: this technique could be a good things for future behaviours when
        // we need to 'split' or upgrade log files
        let (sender, receiver) = std::sync::mpsc::sync_channel::<HandlerCommand>(5000);

        let s = std::thread::spawn(move || {
            for cmd in receiver {
                match cmd {
                    HandlerCommand::Append(aof_row) => {
                        file_handler.write(&aof_row.get_file_output());
                    }
                }
            }
        });

        AofFileHandler {
            aof_file: aof_file.clone(),
            sender: sender,
            join: s,
        }
    }

    pub fn append_row(&self, row: AofRow) {
        let cmd = HandlerCommand::Append(row);
        self.sender.send(cmd);
    }
}

pub struct AofRow {
    pub id: u64,
    pub data: Vec<u8>,
}

impl AofRow {
    pub fn get_file_output(&self) -> Vec<u8> {
        let buff_size = 3 + 8 + self.data.len() + 3;

        let mut out = [0u8; 11].to_vec();
        byteorder::BE::write_u24(&mut out, self.data.len() as u32);
        byteorder::BE::write_u64(&mut out, self.id);
        out.write(&self.data);

        // Magic bytes for: 1) easier debugging 2)can look at the end of file to verify end of file
        // is not corrupted so we can append again instead of oppening a new file
        out.write(b"\n>\n");

        assert!(out.len() == buff_size);

        out
    }
}
