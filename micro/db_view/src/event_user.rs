use shared::{pb, xc};

use crate::{db, db_mem, db_trait, events, session};
use shared::errors::GenErr;
use shared::pb::event_command::Command;
use shared::pb::user_command::SubCommand;
use shared::pb::EventCommand;

pub struct EventUser {
    db: Box<dyn db_trait::DB + Send>,
}

impl EventUser {
    pub fn new_mem() -> Self {
        EventUser {
            db: Box::new(db_mem::DBMem::new()),
        }
    }
}

impl events::EventProcess for EventUser {
    fn process_event(&self, event: EventCommand) -> Result<bool, GenErr> {
        let ch_sub = conv_to_user_sub_command(event.clone());

        use SubCommand::*;
        match ch_sub {
            RegisterUser(_) => {}
            EditUser(_) => {}
            DeleteSendCode(_) => {}
            DeleteConfirmCode(_) => {}
            DeleteUser(_) => {}
            ChangePhoneNumber(_) => {}
            RemoveSession(_) => {}
            RemoveOtherSessions(_) => {}
        }

        let x = 1;
        Ok(true)
    }
}

fn conv_to_user_sub_command(event: EventCommand) -> pb::user_command::SubCommand {
    let cmd = event.command.unwrap();
    match cmd {
        Command::User(ch_cmd) => ch_cmd.sub_command.unwrap(),
        _ => (panic!("can not convert to channel sub command")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventProcess;
    use pb::channel_command::SubCommand::*;
    use pb::channel_command::*;
    use shared::pb::NewMessageInput;
    use std::ops::Index;

    struct ChannelTester {
        proc: EventUser,
    }

    impl ChannelTester {
        fn new() -> Self {
            ChannelTester {
                proc: EventUser::new_mem(),
            }
        }

        fn send(&self, cmd: pb::channel_command::SubCommand) {
            let chcmd = pb::ChannelCommand {
                channel_cid: 4,
                sub_command: Some(cmd.clone()),
            };

            let qevent = shared::pb::EventCommand {
                event_id: 1 as u64,
                user_id: 2,
                command: Some(pb::event_command::Command::Channel(chcmd)),
                // command: pb::event_command::Command::Channel(pb::ChannelCommand)
            };

            self.proc.process_event(qevent).unwrap();
        }

        fn start_tests(&self) {
            let CHANNEL_CID = 101;
            let PROFILE_CID = 100;

            //====== CreateChannel
            let q = QCreateChannel {
                channel_cid: CHANNEL_CID,
                creator_profile_cid: PROFILE_CID,
                channel_title: "t".to_string(),
                user_name: "".to_string(),
                about: "a".to_string(),
            };

            self.send(CreateChannel(q));

            let db = &self.proc.db;

            let ch = self.proc.db.get_channel(101).unwrap();
            assert_eq!(ch.creator_profile_cid, 100);

            println!("Channel Tests Worked Correctly");
        }
    }

    #[test]
    fn channels_tests() {
        println!("it works!");
        let ct = ChannelTester::new();
        ct.start_tests();
    }

    #[test]
    fn it_works() {
        println!("it works!");
    }
}
