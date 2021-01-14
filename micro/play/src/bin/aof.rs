extern crate play;
use prost::Message;
use std::io::Write;
use byteorder::{BE, ByteOrder};

use play::aof;

fn main(){

    let af = aof::AofFile::new();
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


    for i in 1..=100000 {
        let mut buf = vec![];
        msg.encode(&mut buf);

        let fe = aof::AofRow {
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

