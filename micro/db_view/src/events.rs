use shared::{pb, xc};

use crate::session;
use crate::{channel_events, groups_events};
use shared::pb::event_command::Command;

use std::thread;
use std::time::Duration;

pub fn process_event(event: pb::EventCommand) {
    let cmd = event.command.unwrap();

    println!("proccing event id: {:?}", &event.event_id);

    match cmd {
        Command::Channel(ch_cmd) => {
            let sub_cmd = ch_cmd.sub_command.unwrap();
            channel_events::handle_channel_events(sub_cmd);
        }
        Command::Group(gr_cmd) => {
            let sub_cmd = gr_cmd.sub_command.unwrap();

            groups_events::handle_group_events(sub_cmd);
        }
    }
}

