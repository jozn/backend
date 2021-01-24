use shared::{pb, xc};

use crate::{db, events, session};
use shared::pb::channel_command::SubCommand;
use shared::pb::event_command::Command;
use shared::pb::EventCommand;

#[derive(Default, Debug)]
pub struct ChannelEvents {}

impl events::EventProcess for ChannelEvents {
    fn process_event(&self, event: EventCommand) -> u8 {
        let ch_sub = conv_to_channel_sub_command(event.clone());

        use SubCommand::*;
        match ch_sub {
            _ => {}
        }

        let x = 1;
        14
    }
}

fn conv_to_channel_sub_command(event: EventCommand) -> pb::channel_command::SubCommand {
    let cmd = event.command.unwrap();
    match cmd {
        Command::Channel(ch_cmd) => ch_cmd.sub_command.unwrap(),
        _ => (panic!("can not convert to channel sub command")),
    }
}
