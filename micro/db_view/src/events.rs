use shared::{pb, xc};

use crate::{channel_events, session};
use shared::pb::event_command::Command;

pub fn process_event(event: pb::EventCommand) {
    let cmd = event.command.unwrap();

    println!("proccing event id: {:?}", &event.event_id);

    match cmd {
        Command::Channel(ch_cmd) => {
            let sub_cmd = ch_cmd.sub_command.unwrap();
            channel_events::handle_channel_events(sub_cmd);
        }
        Command::Group(_) => {}
    }
}

fn main() {}
