extern crate play;
use byteorder::{ByteOrder, BE};
use prost::Message;
use shared::aof;
use shared::pb;
use std::io::Write;
use std::ops::Sub;

fn main() {
    let s = "".to_string();
    println!("empty string cap {} ", s.capacity());

    cal_time(vec_alloc);
    cal_time(pb_alloc2);
    cal_time(incoke_alloc);
    cal_time(message_alloc);
    cal_time(incoke2_alloc);
    cal_time(inovke1);
    cal_time(message1);
}

fn cal_time<F: Fn()>(func: F) {
    use std::time;
    use std::time::SystemTime;
    let t1 = std::time::SystemTime::now();

    func();

    let t2 = SystemTime::now();
    let d = t2.duration_since(t1).unwrap();
    println!(">>>> Time spent: {} milli second", d.as_millis())
}
// Some bench results:
//      Invoke pb encode speed: 8 Million per second (22 bytes)
//      Invoke pb allocate and encode speed: 7.3 Million per second (22 bytes > 160 MB/S )
//      every string or vector has 24 bytes in memory (3*8 in 64 bit sys)
//      pb::Message has 2.5 Mill encode rate (68 bytes > 170 MB/S )
//      pb::Message has 58 Mill allocate rate
//      Invoke2 has 1000 Mill/sec allocate rate!
//      Invoke has 1 billion allocation in 11 millisecond > very fast for empty or none empty
//      200K an vector of 1 million pb::message per second
//
//      Note: some rates above is not real world sceanrio as memory is local to cpu cache and reused.

fn inovke1() {
    for i in 0..10_000_000 {
        let invoke1 = Invoke {
            namespace: 132,
            method: 234,
            user_id: 234,
            action_id: 234,
            session: b"sdfsdfas".to_vec(),
            rpc_data: vec![],
        };
        let p = shared::common::prost_encode(&invoke1);
        if i % 1000_000 == 0 {
            println!(
                "{} {} - mem size: {}",
                i,
                p.unwrap().len(),
                std::mem::size_of::<Invoke>()
            );
        }
    }
}

fn mem_size() {
    println!("+++ mem size: {}", std::mem::size_of::<Invoke2>())
}

fn message1() {
    let msg = pb::Message {
        message_gid: 123897432894,
        by_profile_cid: 23,
        message_type: 2,
        text: "hi there to all".to_string(),
        via_app_id: 23,
        seq: 2,
        created_time: 32432,
        edited_time: 23423434,
        delivery_status: 2,
        delivery_time: 298472349,
        verified: true,
        deleted: true,
        forward: None,
        reply_to: None,
        channel_cid: 2,
        setting: None,
        counts: None,
        group_cid: 3,
        files: vec![],
        product: None,
    };
    for i in 0..10_000_000 {
        let p = shared::common::prost_encode(&msg);
        if i % 1000_000 == 0 {
            // this compact the size of vec buffer
            let v = p.unwrap();
            let mut v2 = Vec::with_capacity(v.len());
            v2.write(&v);

            println!(
                "{} {} - mem size: {} -- compact cap size: {} - buf cap {}",
                i,
                v.len(),
                std::mem::size_of::<pb::Message>(),
                v2.capacity(),
                v.capacity()
            )
        }
    }
}

fn message_alloc() {
    for i in 0..100_000_000 {
        let msg = pb::Message {
            message_gid: 123897432894,
            by_profile_cid: 23,
            message_type: 2,
            text: "hi there to all".to_string(),
            via_app_id: 23,
            seq: 2,
            created_time: 32432,
            edited_time: 23423434,
            delivery_status: 2,
            delivery_time: 298472349,
            verified: true,
            deleted: true,
            forward: None,
            reply_to: None,
            channel_cid: 2,
            setting: None,
            counts: None,
            group_cid: 3,
            files: vec![],
            product: None,
        };
    }
}

fn incoke2_alloc() {
    for i in 0..100_000_000 {
        let p = Invoke2 {
            rpc_data: vec![],
            s1: "".to_string(),
        };
        // if i% 10_000_000 == 0 {
        //     println!("{}",p.s1)
        // }
    }
}

fn incoke_alloc() {
    for i in 0..1000_000_000 {
        let p = Invoke {
            namespace: 0,
            method: 0,
            user_id: 0,
            action_id: 0,
            session: vec![],
            rpc_data: vec![],
        };
        // if i% 10_000_000 == 0 {
        //     println!("{}",p.s1)
        // }
    }
}

fn pb_alloc2() {
    for i in 0..1000_000_000 {
        let p = Invoke {
            namespace: i,
            method: i,
            user_id: i,
            action_id: 0,
            session: vec![],
            rpc_data: vec![],
        };
        // if i% 10_000_000 == 0 {
        //     println!("{}",p.s1)
        // }
    }
}

fn vec_alloc() {
    for i in 0..1_000_000 {
        let mut a: Vec<pb::Message> = Vec::with_capacity(1000_000);
        let p = pb::Message {
            message_gid: 345,
            by_profile_cid: 234,
            message_type: 2,
            text: "".to_string(),
            via_app_id: 0,
            seq: 0,
            created_time: 0,
            edited_time: 234,
            delivery_status: 0,
            delivery_time: 0,
            verified: false,
            deleted: true,
            forward: None,
            reply_to: None,
            channel_cid: 0,
            setting: None,
            counts: None,
            group_cid: 0,
            files: vec![],
            product: None,
        };
        a.push(p);
        // if i% 10_000_000 == 0 {
        //     println!("{}",p.s1)
        // }
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Invoke {
    /// imut
    #[prost(uint32, tag = "6")]
    pub namespace: u32,
    /// imut
    #[prost(uint32, tag = "1")]
    pub method: u32,
    /// imut
    #[prost(uint32, tag = "7")]
    pub user_id: u32,
    /// imut
    #[prost(uint64, tag = "2")]
    pub action_id: u64,
    /// imut
    #[prost(bytes, tag = "8")]
    pub session: std::vec::Vec<u8>,
    /// imut
    #[prost(bytes, tag = "4")]
    pub rpc_data: std::vec::Vec<u8>,
}

#[derive(Clone, PartialEq)]
pub struct Invoke2 {
    pub rpc_data: std::vec::Vec<u8>,
    s1: String,
}
