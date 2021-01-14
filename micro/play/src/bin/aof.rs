
use prost::Message;
use std::io::Write;
use byteorder::{BE, ByteOrder};

fn main(){
    std::fs::create_dir_all("./aof_out").unwrap();
    // let mut f = std::fs::File::create("./aof_out/1.aof").unwrap();
    let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let filename = format!("./aof_out/{}.aof", time);
    let mut f = std::fs::OpenOptions::new().create_new(true).write(true).read(true).open(filename).unwrap();

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


    for i in 1..=100000 {
        let mut buf = vec![];
        msg.encode(&mut buf);

        let fe = FileEntry {
            id: i as u64,
            data: buf,
        };

        f.write(&fe.get_file_output());

        if i%10000 == 0 {
            println!("{}",i);
        }

    }

}

struct FileEntry {
    id: u64,
    data: Vec<u8>,
}

impl FileEntry {
    fn get_file_output(&self) -> Vec<u8> {
        // let mut out = Vec::with_capacity(110);
        let buff_size = 3 + 8 + self.data.len() + 3;
        println!("{}",buff_size);
        let mut out = [0u8;11].to_vec();
        // let mut out = Vec::new();
        // out.resize(buff_size,0);
        byteorder::BE::write_u24(&mut out, self.data.len() as u32);
        byteorder::BE::write_u64(&mut out, self.id);
        println!("out 1 {}",out.len());

        out.write(&self.data);
        out.write(b"\n>\n");
        println!("out {}",out.len());

        assert!(out.len() == buff_size);

        out
    }
}


fn main2_bk(){
    std::fs::create_dir_all("./aof_out").unwrap();
    // let mut f = std::fs::File::create("./aof_out/1.aof").unwrap();
    let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let filename = format!("./aof_out/{}.aof", time);
    let mut f = std::fs::OpenOptions::new().create_new(true).write(true).read(true).open(filename).unwrap();

    let msg = shared::pb::Message{
        gid: 234,
        by_profile_cid: 23,
        message_type: 54,
        text: "aaaaaaaaaaaaaaaaaaa  dfjlaskdf slskfj lsdk fasdlfjsd fsd aaaaabbbbbbbbbbbbbbbbbbbb".to_string(),
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
    let mut buf = vec![];
    msg.encode(&mut buf);
    println!("{:?}", buf);

    f.write(&buf);

    let mut buf2 = vec![];

    for i in 1..=100_000 {
        let mut buf = vec![];
        msg.encode(&mut buf);

        if i%10000 == 0 {
            f.write(&buf2);
            buf2.clear();
            println!("{}",i);
        }
        buf2.write(&buf);
    }

}
/*

for i in 1..=1_000_000 {
        let mut buf = vec![];
        msg.encode(&mut buf);

        if i%10000 == 0 {
            f.write(&buf2);
            buf2.clear();
            println!("{}",i);
        }
        buf2.write(&buf);
    }
*/