use crate::session;
use shared::{common::prost_decode, common::prost_encode, errors::GenErr, pb, xc};

pub fn get_channel(channel_id: u64) -> Result<pb::Channel, GenErr> {
    let sess = session::get_session();

    let x = xc::Channel_Selector::new()
        .channel_id_eq(channel_id as i64)
        .get_row(&sess)
        .unwrap();
    println!("channel data: {:?}", x.pb_data.as_slice());
    let ch: pb::Channel = shared::common::prost_decode(&x.pb_data)?;
    // let ch = ::prost::Message::decode(x.pb_data.as_slice())?;

    Ok(ch)
}

pub fn get_channel_messages(channel_id: u64) -> Result<pb::Channel, GenErr> {
    let sess = session::get_session();

    let x = xc::ChannelMsg_Selector::new()
        .channel_id_eq(channel_id as i64)
        .and_msg_id_eq(234)
        .get_row(&sess)
        .unwrap();
    println!("channel data: {:?}", x.pb_data.as_slice());
    let ch: pb::Channel = shared::common::prost_decode(&x.pb_data)?;
    // let ch = ::prost::Message::decode(x.pb_data.as_slice())?;

    Ok(ch)
}

pub fn save_channel(channel: &pb::Channel) -> Result<(), GenErr> {
    let sess = session::get_session();

    let pb = prost_encode(channel)?;
    let xch = xc::Channel {
        channel_id: channel.cid as i64,
        pb_data: pb,
    };

    xch.save(&sess)?;

    Ok(())
}

pub fn save_channel_verify(channel: &pb::Channel) -> Result<(), GenErr> {
    let sess = session::get_session();

    let pb = prost_encode(channel)?;
    let xch = xc::Channel {
        channel_id: channel.cid as i64,
        pb_data: pb.clone(),
    };
    xch.save(&sess)?;

    let x = xc::Channel_Selector::new()
        .channel_id_eq(channel.cid as i64)
        .get_row(&sess)
        .unwrap();

    assert_eq!(x.pb_data, pb);

    Ok(())
}
