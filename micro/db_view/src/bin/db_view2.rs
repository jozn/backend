extern crate db_view;

use shared::{pb, xc};

fn main() {
    cmd_save_channels_handler();
    // cmd_save_channels();
    // cmd_edit_channels();
}

////////////// thread handling /////////////
fn cmd_save_channels_handler() {
    let mut h = db_view::events::EventHandler::new();

    for i in 1..=10 {
        let q = pb::channel_command::QCreateChannel {
            channel_cid: i,
            creator_profile_cid: i % 10 + 1,
            channel_title: format!("channel #{}", i),
            user_name: format!("ch_username#{}", i),
            history_viewable: false,
            force_join: false,
            global_search: false,
            about: "My channel".to_string(),
        };

        let cmd = pb::channel_command::SubCommand::CreateChannel(q);

        let chcmd = pb::ChannelCommand {
            channel_id: 4,
            sub_command: Some(cmd),
        };

        let qevent = shared::pb::EventCommand {
            event_id: i as u64,
            user_id: 2,
            command: Some(pb::event_command::Command::Channel(chcmd)),
            // command: pb::event_command::Command::Channel(pb::ChannelCommand)
        };

        h.process_event_shared(qevent)
        // send_channel_cmd(i as u64, cmd);
    }

    std::thread::sleep(std::time::Duration::from_secs(40));
}
