extern crate db_view;

use shared::{pb, xc};

fn main() {
    // this is the way to create commands
    let qcreat = pb::channel_command::QCreateChannel {
        about: "textinf".to_string(),
        channel_cid: 5,
        ..Default::default()
    };

    let chcmd = pb::ChannelCommand {
        sub_command: Some(pb::channel_command::SubCommand::CreateChannel(qcreat)),
    };

    let qevent = shared::pb::EventCommand {
        event_id: 23,
        user_id: 2,
        command: Some(pb::event_command::Command::Channel(chcmd)),
        // command: pb::event_command::Command::Channel(pb::ChannelCommand)
    };

    db_view::events::process_event(qevent);
    // save_channels();
}

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
