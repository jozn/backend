use shared::{pb, xc};

use crate::{db, db_mem, db_trait, events, session};
use shared::errors::GenErr;
use shared::pb::event_command::Command;
use shared::pb::user_command::SubCommand;
use shared::pb::EventCommand;

pub struct UserEventProcessor {
    db: Box<dyn db_trait::DB + Send>,
}

impl UserEventProcessor {
    pub fn new_mem() -> Self {
        UserEventProcessor {
            db: Box::new(db_mem::DBMem::new()),
        }
    }
}

impl events::EventProcess for UserEventProcessor {
    fn process_event(&self, event: EventCommand) -> Result<bool, GenErr> {
        let ch_sub = conv_to_user_sub_command(event.clone());

        use SubCommand::*;
        match ch_sub {
            RegisterUser(q) => {
                let user_pb = pb::User {
                    user_cid: q.user_cid,
                    phone: q.phone,
                    email: "".to_string(),
                    created_time: 0, //todo
                    version_time: 0,
                    first_name: q.first_name,
                    last_name: q.last_name,
                    is_deleted: false,
                    is_banned: false,
                    password_hash: "".to_string(),
                    password_salt: "".to_string(),
                    def_profile: None,
                    sessions: vec![],
                    shopping_profile: None,
                    stores: vec![],
                    profiles: vec![],
                };

                self.db.save_user(&user_pb).unwrap();
            }
            EditUser(q) => {
                if q.set_new_name {
                    let mut user_pb = self.db.get_user_by_cid(q.user_cid as i64).unwrap();
                    user_pb.first_name = q.new_first_name;
                    user_pb.last_name = q.new_last_name;
                    self.db.save_user(&user_pb).unwrap();
                }
            }
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
    use pb::user_command::SubCommand::*;
    use pb::user_command::*;
    use shared::pb::NewMessageInput;
    use std::ops::Index;

    struct UserTester {
        proc: UserEventProcessor,
    }

    impl UserTester {
        fn new() -> Self {
            UserTester {
                proc: UserEventProcessor::new_mem(),
            }
        }

        fn send(&self, cmd: pb::user_command::SubCommand) {
            let user_cmd = pb::UserCommand {
                sub_command: Some(cmd.clone()),
            };

            let qevent = shared::pb::EventCommand {
                event_id: 1 as u64,
                user_id: 2,
                command: Some(pb::event_command::Command::User(user_cmd)),
            };

            self.proc.process_event(qevent).unwrap();
        }

        fn start_tests(&self) {
            let db = &self.proc.db;

            //===== QRegisterUser
            let q = QRegisterUser {
                user_cid: 1,
                first_name: "Beth".to_string(),
                last_name: "Smith".to_string(),
                phone: "+989015131122".to_string(),
                created_time: 160000000,
            };

            self.send(RegisterUser(q));

            let user1 = db.get_user_by_cid(1).unwrap();
            assert_eq!(user1.last_name, "Smith");

            //===== QRegisterUser
            let q = QEditUser {
                user_cid: 1,
                set_new_name: true,
                new_first_name: "Mona".to_string(),
                new_last_name: "Holla".to_string(),
            };

            self.send(EditUser(q));

            let user1 = db.get_user_by_cid(1).unwrap();
            assert_eq!(user1.last_name, "Holla");

            println!("User tests works correctly.");
        }
    }

    #[test]
    fn user_tests() {
        println!("User events processing tests started.");
        let tester = UserTester::new();
        tester.start_tests();
    }
}
