use shared::{pb, xc};

use crate::{db, session};
use shared::pb::group_command::SubCommand;

// Single threaded
pub fn handle_group_events(command: pb::group_command::SubCommand) {
    use pb::group_command::SubCommand::*;

    match command {
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
    };
}
