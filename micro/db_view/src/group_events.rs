use shared::{pb, xc};

use crate::{db, events, session};
use shared::pb::group_command::SubCommand;
use shared::pb::event_command::Command;
use shared::pb::EventCommand;

#[derive(Default, Debug)]
pub struct GroupEvents {}

impl events::EventProcess for GroupEvents {
    fn process_event(&self, event: EventCommand) -> u8 {
        let ch_sub = conv_to_group_sub_command(event.clone());

        use SubCommand::*;
        match ch_sub {
            CreateGroup(_) => {}
            EditGroup(_) => {}
            DeleteGroup(_) => {}
            AddMember(_) => {}
            RemoveMember(_) => {}
            ChangeMemberLevel(_) => {}
            ChangeMemberPermission(_) => {}
            JoinGroup(_) => {}
            LeaveGroup(_) => {}
            BanMember(_) => {}
            ChangePrivacy(_) => {}
            RevokeLink(_) => {}
            ChangeUsername(_) => {}
            SendMessage(_) => {}
            EditMessage(_) => {}
            PinMessage(_) => {}
            UnPinMessage(_) => {}
            DeleteMessages(_) => {}
            DeleteHistory(_) => {}
            ClearHistory(_) => {}
            AvatarAdd(_) => {}
            AvatarDelete(_) => {}
            ReportGroup(_) => {}
        }

        let x = 1;
        7
    }
}

fn conv_to_group_sub_command(event: EventCommand) -> pb::group_command::SubCommand {
    let cmd = event.command.unwrap();
    match cmd {
        Command::Group(ch_cmd) => ch_cmd.sub_command.unwrap(),
        _ => (panic!("can not convert to channel sub command")),
    }
}
