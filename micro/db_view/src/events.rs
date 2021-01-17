use shared::{xc,pb};

use crate::session;

fn main(){

    // this is the way to create commands
    let qcreat = pb::channel_command::QCreateChannel{
        ..Default::default()
    };

    let chcmd = pb::ChannelCommand{
        sub_command: Some(pb::channel_command::SubCommand::CreateChannel(qcreat))
    };

    let e = shared::pb::EventCommand{
        event_id: 23,
        user_id: 2,
        command: Some(pb::event_command::Command::Channel(chcmd)),

        // command: pb::event_command::Command::Channel(pb::ChannelCommand)
    };


    // consume commands
    let qevent = pb::EventCommand{
        event_id: 0,
        user_id: 0,
        command: None
    };
    let cmd = qevent.command.unwrap();

    match cmd {
        pb::event_command::Command::Channel(ch) => {
            let s = ch.sub_command.unwrap();

        },
        _ => {}

    }

}

