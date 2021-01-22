use crate::session;
use shared::{common, common::prost_decode, common::prost_encode, errors::GenErr, pb, xc};

pub struct DBCassandra {
    pub session: session::FlipCassandraSession,
}

impl DBCassandra {
    pub fn new() -> Self {
        DBCassandra {
            session: session::get_session(),
        }
    }

    // =================== Channel ====================
    pub fn get_channel(&self, channel_id: i64) -> Result<pb::Channel, GenErr> {
        let sess = &self.session;

        let x = xc::Channel_Selector::new()
            .channel_id_eq(channel_id as i64)
            .get_row(sess)?;

        println!("channel data: {:?}", x.pb_data.as_slice());
        let ch: pb::Channel = common::prost_decode(&x.pb_data)?;
        // let ch = ::prost::Message::decode(x.pb_data.as_slice())?;

        Ok(ch)
    }

    pub fn save_channel(&self, channel: &pb::Channel) -> Result<(), GenErr> {
        let sess = &self.session;

        let pb = prost_encode(channel)?;
        let xch = xc::Channel {
            channel_id: channel.cid as i64,
            pb_data: pb,
        };

        xch.save(sess)?;

        Ok(())
    }

    // =================== Channel Message ====================
    pub fn get_channel_message(
        &self,
        channel_id: i64,
        message_id: i64,
    ) -> Result<pb::Message, GenErr> {
        let sess = &self.session;

        let r = xc::get_channel_msg_by_channel_id_and_msg_id(sess, channel_id, message_id)?;
        let ch = common::prost_decode(&r.pb_data)?;

        Ok(ch)
    }

    pub fn save_channel_message(&self, message: &pb::Message) -> Result<(), GenErr> {
        let sess = &self.session;

        let pb = prost_encode(message)?;
        let r = xc::ChannelMsg {
            channel_id: message.by_profile_cid as i64, //todo fixme bug
            msg_id: message.gid as i64,
            pb_data: pb,
        };
        r.save(sess)?;

        Ok(())
    }

    // =================== Chat ====================
    pub fn get_chat(&self, profile_id: i64, chat_id: i64) -> Result<pb::Chat, GenErr> {
        let sess = &self.session;

        let r = xc::get_chat_by_profile_cid_and_chat_id(sess, profile_id, chat_id)?;
        let chat = common::prost_decode(&r.pb_data)?;

        Ok(chat)
    }

    pub fn save_chat(&self, chat: &pb::Chat) -> Result<(), GenErr> {
        let sess = &self.session;

        let pb = prost_encode(chat)?;
        let r = xc::Chat {
            //todo fixme
            chat_id: 1,
            profile_cid: 0,
            pb_data: vec![],
        };
        r.save(sess)?;

        Ok(())
    }

    // =================== Chat Message ====================
    pub fn get_chat_message(
        //todo imple
        &self,
        channel_id: i64,
        message_id: i64,
    ) -> Result<pb::Message, GenErr> {
        /* let sess = &self.session;

        let r = xc::get_channel_msg_by_channel_id_and_msg_id(sess, channel_id, message_id)?;
        let ch = common::prost_decode(&r.pb_data)?;

        Ok(ch)*/
        unimplemented!("")
    }

    pub fn save_chat_message(&self, message: &pb::Message) -> Result<(), GenErr> {
        /*        let sess = &self.session;

        let pb = prost_encode(message)?;
        let r = xc::ChannelMsg {
            channel_id: message.cid as i64,
            msg_id: message.gid as i64,
            pb_data: pb,
        };
        r.save(sess)?;

        Ok(())*/
        unimplemented!("")
    }
}

// Deprecated > remove
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
