extern crate play;
use byteorder::{ByteOrder, BE};
use prost::Message;
use std::io::Write;

use shared::aof;
use shared::pb;
use shared::pb::channel_command::SubCommand;

use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::Session;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::frame::Frame;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::{QueryExecutor, QueryValues};
use play::xc;
use play::xc::FCQueryExecutor;

struct MyCassandraSession {
    session: Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>,
}

impl FCQueryExecutor for &MyCassandraSession {
    fn query_with_values<Q: ToString, V: Into<QueryValues>>(
        &self,
        query: Q,
        values: V,
    ) -> cdrs::error::Result<Frame>
    where
        Self: Sized,
    {
        self.session.query_with_values(query, values)
    }
}

fn handle_channel_events(command: pb::channel_command::SubCommand) {
    let node = NodeTcpConfigBuilder::new("185.239.107.163:9042", NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

    let my_session = MyCassandraSession {
        session: no_compression,
    };

    let m = play::xc::ChannelMsg {
        channel_id: 1,
        msg_id: 1,
        pb_data: b"msg 1".into(),
    };

    use pb::channel_command::SubCommand::*;

    match command {
        CreateChannel(p) => {
            let ch = pb::Channel {
                cid: 0,
                user_name: "".to_string(),
                channel_name: p.channel_title,
                creator_profile_cid: p.creator_profile_cid,
                is_verified: false,
                is_profile_channel: false,
                avatar_count: 0,
                about: p.about,
                invite_link_hash: "".to_string(),
                message_seq: 0,
                sync_time_ms: 0,
                created_time: 0,
                is_deleted: 0,
                is_banned: 0,
                notification_setting: None,
                privacy: 0,
                counts: None,
                last_message: None,
                pinned_message: None,
                avatar: None,
            };
            true
        }
        EditChannel(p) => {
            let mut c = xc::Channel_Selector::new()
                .channel_id_eq(p.channel_cid.into())
                .get_row(&my_session)
                .unwrap();
            c.pb_data;

            c.save(&my_session);
            false
        }
        DeleteChannel(_) => false,
        AddAuthor(_) => false,
        ChangeAuthorPermission(_) => false,
        RemoveAuthor(_) => false,
        FollowChannel(_) => false,
        UnFollowChannel(_) => false,
        RemoveFollowers(_) => false,
        Subscribe(_) => false,
        UnSubscribe(_) => false,
        RemoveSubscribers(_) => false,
        ChangePrivacy(_) => false,
        ChangeDefaultPermission(_) => false,
        RevokeLink(_) => false,
        ChangeUsername(_) => false,
        BlockChannel(_) => false,
        SendMessage(_) => false,
        EditMessage(_) => false,
        PinMessage(_) => false,
        UnPinMessage(_) => false,
        DeleteMessage(_) => false,
        DeleteMessages(_) => false,
        ClearHistory(_) => false,
        AvatarAdd(p) => true,
        AvatarChange(_) => false,
        AvatarDelete(_) => false,
        SendDoingAction(_) => false,
        ReportChannel(_) => false,
        ReportMessage(_) => false,
    };
}

fn main() {
    // this is the way to create commands
    let qcreat = pb::channel_command::QCreateChannel {
        ..Default::default()
    };
    let chcmd = pb::ChannelCommand {
        sub_command: Some(pb::channel_command::SubCommand::CreateChannel(qcreat)),
    };
    let e = shared::pb::EventCommand {
        event_id: 23,
        user_id: 2,
        command: Some(pb::event_command::Command::Channel(chcmd)),
        // command: pb::event_command::Command::Channel(pb::ChannelCommand)
    };

    // consume commands
    let qevent = pb::EventCommand {
        event_id: 0,
        user_id: 0,
        command: None,
    };
    let cmd = qevent.command.unwrap();

    match cmd {
        pb::event_command::Command::Channel(ch) => {
            let s = ch.sub_command.unwrap();
        }
        _ => {}
    }

    let af = aof::AofFile::new();
    let hand = af.get_file_handler();

    let msg = shared::pb::Message {
        gid: 234,
        by_profile_cid: 23,
        message_type: 54,
        text:
            "aaaaaaaaaaaaaaaaaaa \n dfjlaskdf slskfj lsdk fasdlfjsd fsd aaaaabbbbbbbbbbbbbbbbbbbb"
                .to_string(),
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
        files: vec![],
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

        if i % 10000 == 0 {
            println!("{}", i);
        }
    }

    // If we do not call this, program will shutdown and will not give background thread the chance to wirte it's data
    hand.join.join();
}
