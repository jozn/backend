
use prost::Message;
use std::io::Write;
use byteorder::{BE, ByteOrder};

fn main(){

    let af = AofFile::new();
    let hand = af.get_file_handler();


    let msg = shared::pb::Message{
        gid: 234,
        by_profile_cid: 23,
        message_type: 54,
        text: "aaaaaaaaaaaaaaaaaaa \n dfjlaskdf slskfj lsdk fasdlfjsd fsd aaaaabbbbbbbbbbbbbbbbbbbb".to_string(),
        via_app_id: 3,
        seq: 2,
        edited_time: 2345345234,
        created_time: 34543534,
        verified: true,
        delivery_status: 22,
        delivery_time: 423,
        deleted: true,
        flags: 234,
        forward: None,
        reply_to: None,
        title: "ioioudf fjsd".to_string(),
        counts: None,
        setting: None,
        product: None,
        files: vec![]
    };

    // for print
    let mut buf = vec![];
    msg.encode(&mut buf);
    println!("{:?}", buf);


    for i in 1..=1000000 {
        let mut buf = vec![];
        msg.encode(&mut buf);

        let fe = AofRow {
            id: i as u64,
            data: buf,
        };

        hand.append_row(fe);

        if i%10000 == 0 {
            println!("{}",i);
        }
    }

    // If we do not call this, program will shutdown and will not give background thread the chance to wirte it's data
    hand.join.join();

}

const DEFAULT_DATA_DIR:&str = "/opt/flip/";
const DEFAULT_DATA_NAMESPACE:&str = "def";

#[derive(Clone)]
struct AofFile {
    namespace: String,
    shared: u32,
    file_name: String,
    directory: String,
}

impl AofFile {
    fn new() -> Self {
        Self::new_with(DEFAULT_DATA_NAMESPACE,1)
    }

    fn new_with(namespace: &str, shared: u32) -> Self {
        let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let filename = format!("{}_{}_{}.aof",namespace, shared,time );

        // std::fs::create_dir(DEFAULT_DATA_DIR).unwrap();
        std::fs::create_dir(DEFAULT_DATA_DIR); //todo fix this

        AofFile{
            namespace: namespace.to_string(),
            shared: shared,
            file_name: filename,
            directory: DEFAULT_DATA_DIR.to_string(),
        }
    }

    fn get_file_handler(&self) -> AofFileHandler {
        // todo add a box to set flag if we already had run
        AofFileHandler::new_with(self)
    }

    fn full_path(&self) -> String{
        format!("{}{}",self.directory,self.file_name)
    }
}

struct AofFileHandler {
    aof_file: AofFile,
    sender: std::sync::mpsc::SyncSender<HandlerCommand>,
    join: std::thread::JoinHandle<()>,
}

enum HandlerCommand {
    Append(AofRow)
}

impl AofFileHandler {

    fn new_with(aof_file: &AofFile) -> Self {

        let mut file_handler = std::fs::OpenOptions::new()
            .create_new(true)// New file in each run (for now)
            .write(true)
            .read(true).open(aof_file.full_path()).unwrap();

        // unbufferd channel is considerably faster as it just pushes data to buffer and thread
        // continues to work: this technique could be a good things for future behaviours when
        // we need to split or upgrade log files
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

        AofFileHandler{
            aof_file: aof_file.clone(),
            sender: sender,
            join: s,
        }
    }

    fn append_row(&self, row: AofRow) {
        let cmd = HandlerCommand::Append(row);
        self.sender.send(cmd);
    }
}

struct AofRow {
    id: u64,
    data: Vec<u8>,
}

impl AofRow {
    fn get_file_output(&self) -> Vec<u8> {
        let buff_size = 3 + 8 + self.data.len() + 3;

        let mut out = [0u8;11].to_vec();
        byteorder::BE::write_u24(&mut out, self.data.len() as u32);
        byteorder::BE::write_u64(&mut out, self.id);
        out.write(&self.data);

        // Magic bytes for: 1) easier debugging 2)can look at the end of file to verify end of file
        // is not corrupted
        out.write(b"\n>\n");

        assert!(out.len() == buff_size);

        out
    }
}
