// #![feature(negative_impls)]

extern crate db_view;

use db_view::events::FEventReq;
use shared::{pb, xc};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    cmd_save_channels_handler().await;
    // cmd_save_channels();
    // cmd_edit_channels();
}

////////////// thread handling /////////////
async fn cmd_save_channels_handler() {
    let mut h = db_view::events::EventHandler::new();

    for i in 1..=10 {
        let q = pb::channel_command::QCreateChannel {
            channel_cid: i,
            creator_profile_cid: i % 10 + 1,
            channel_title: format!("Titleing #{}", i),
            user_name: format!("xxmy#{}_ch", i),
            about: "about!".to_string(),
        };

        let cmd = pb::channel_command::SubCommand::CreateChannel(q);

        let chcmd = pb::ChannelCommand {
            channel_cid: 4,
            sub_command: Some(cmd),
        };

        let qevent = shared::pb::EventCommand {
            event_id: i as u64,
            user_id: 2,
            command: Some(pb::event_command::Command::Channel(chcmd)),
            // command: pb::event_command::Command::Channel(pb::ChannelCommand)
        };

        let (send, rec) = oneshot::channel();
        let event_req = FEventReq {
            event: qevent,
            result: send,
        };

        h.process_event_shared(event_req);
        let r = rec.await;
        println!("{:?}", r);
        // send_channel_cmd(i as u64, cmd);
    }

    std::thread::sleep(std::time::Duration::from_secs(40));
}
