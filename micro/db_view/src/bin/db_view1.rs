extern crate db_view;

use shared::{pb, xc};

fn main() {
    cmd_save_channels_handler();
    // cmd_save_channels();
    // cmd_edit_channels();
}

////////////// thread handling /////////////
fn cmd_save_channels_handler() {
    let mut h = db_view::events_old::EventHandler::new();

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
}

///////////// Old Codes -- fn handling //////////////
fn cmd_save_channels() {
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

        send_channel_cmd(i as u64, cmd);
    }
}

fn cmd_edit_channels() {
    for i in 1..=10 {
        use pb::channel_command::SubCommand as cc;
        use pb::channel_command::SubCommand::*;
        use pb::channel_command::*;

        let cmd = EditChannel(QEditChannel {
            channel_cid: i,
            by_profile_cid: 777,
            set_new_title: false,
            new_title: "".to_string(),
            set_new_about: true,
            new_about: "EDITED".to_string(),
        });

        send_channel_cmd(i as u64, cmd);

        // play
        let x = cc::DeleteMessage(QDeleteMessage { channel_id: 0 });
    }
}

fn send_channel_cmd(event_id: u64, ch_cmd: pb::channel_command::SubCommand) {
    let chcmd = pb::ChannelCommand {
        channel_id: 4,
        sub_command: Some(ch_cmd),
    };

    let qevent = shared::pb::EventCommand {
        event_id: event_id,
        user_id: 2,
        command: Some(pb::event_command::Command::Channel(chcmd)),
        // command: pb::event_command::Command::Channel(pb::ChannelCommand)
    };

    db_view::events_old::process_event(qevent);
}

/////////// other codes
fn save_channels() {
    for i in 1..=100 {
        let ch = pb::Channel {
            cid: i,
            user_name: "".to_string(),
            channel_name: format!("channel #{}", i),
            creator_profile_cid: 5,
            is_verified: false,
            is_profile_channel: false,
            avatar_count: 0,
            about: format!("About this channel #{}", i),
            invite_link_hash: format!("cab#{}", i),
            message_seq: i,
            sync_time_ms: 0,
            created_time: 0,
            is_deleted: 0,
            is_banned: i,
            notification_setting: None,
            privacy: 0,
            counts: None,
            last_message: None,
            pinned_message: None,
            avatar: None,
        };

        db_view::db::save_channel_verify(&ch);
    }
}

fn main_bk() {
    // this is the way to create commands
    let qcreat = pb::channel_command::QCreateChannel {
        about: "textinf".to_string(),
        channel_cid: 5,
        ..Default::default()
    };

    let chcmd = pb::ChannelCommand {
        channel_id: 5,
        sub_command: Some(pb::channel_command::SubCommand::CreateChannel(qcreat)),
    };

    let qevent = shared::pb::EventCommand {
        event_id: 23,
        user_id: 2,
        command: Some(pb::event_command::Command::Channel(chcmd)),
        // command: pb::event_command::Command::Channel(pb::ChannelCommand)
    };

    db_view::events_old::process_event(qevent);
    // save_channels();
}
