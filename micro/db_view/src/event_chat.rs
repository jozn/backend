use shared::{pb, xc};

use crate::{db, db_mem, db_trait, events, session};
use shared::errors::GenErr;
use shared::pb::chat_command::SubCommand;
use shared::pb::event_command::Command;
use shared::pb::EventCommand;

pub struct ChatEventProcessor {
    db: Box<dyn db_trait::DB + Send>,
}

impl ChatEventProcessor {
    pub fn new_mem() -> Self {
        ChatEventProcessor {
            db: Box::new(db_mem::DBMem::new()),
        }
    }
}

impl events::EventProcess for ChatEventProcessor {
    fn process_event(&self, event: EventCommand) -> Result<bool, GenErr> {
        let (chat_event, ch_sub) = conv_to_chat_sub_command(event.clone());

        let profile_cid = event.profile_cid;

        use SubCommand::*;
        match ch_sub {
            DeleteChat(q) => {
                let chat = self.db.get_chat(event.profile_cid as i64,q.chat_gid as i64)?;
                 // todo delete each message > extract
            }
            SendMessage(q) => {
                let chat = self
                    .db
                    .get_chat(event.profile_cid as i64, q.chat_gid as i64)?;
                let i = q.message_input.unwrap();
                let m = pb::Message {
                    message_gid: i.message_gid,
                    by_profile_cid: i.by_profile_cid,
                    message_type: i.message_type,
                    text: i.text,
                    via_app_id: 0,
                    seq: 0,
                    created_time: 0, //todo
                    edited_time: 0,
                    delivery_status: 0, // todo: move this stuff to a subset PeerMsgInfo pb message
                    delivery_time: 0,
                    verified: false,
                    deleted: false,
                    forward: None,
                    reply_to: None,
                    channel_cid: 0,
                    setting: None,
                    counts: None,
                    group_cid: 0,
                    files: vec![],
                    product: None,
                };
                
                self.db.save_chat_message(chat.chat_gid as i64, &m)?;
                //todo add media
            }
            EditMessage(_) => {
                // Not now
            }
            DeleteMessages(_) => {
                // Not now
            }
            DeleteHistory(_) => {
                // Not now
            }
        }

        let x = 1;
        Ok(true)
    }
}

fn conv_to_chat_sub_command(
    event: EventCommand,
) -> (pb::ChatCommand, pb::chat_command::SubCommand) {
    let cmd = event.command.unwrap();
    match cmd {
        Command::Chat(ch_cmd) => (ch_cmd.clone(), ch_cmd.sub_command.unwrap()),
        _ => (panic!("can not convert to channel sub command")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventProcess;
    use pb::chat_command::SubCommand::*;
    use pb::chat_command::*;
    use shared::pb::NewMessageInput;
    use std::ops::Index;

    struct ChatTester {
        proc: ChatEventProcessor,
    }

    impl ChatTester {
        fn new() -> Self {
            ChatTester {
                proc: ChatEventProcessor::new_mem(),
            }
        }

        fn send(&self, cmd: pb::chat_command::SubCommand) {
            let db = &self.proc.db;

            let user_cmd = pb::ChatCommand {
                profile_cid: 0,
                sub_command: Some(cmd.clone()),
            };

            let qevent = shared::pb::EventCommand {
                event_id: 0,
                user_cid: 0,
                profile_cid: 5,
                channel_cid: 0,
                chat_gid: 0,
                group_cid: 0,
                command: Some(pb::event_command::Command::Chat(user_cmd)),
            };

            //===== QRegisterUser
            let q = QSendMessage {
                profile_cid: 2,
                chat_gid: 1,
                message_input: Some(NewMessageInput {
                    message_gid: 21,
                    by_profile_cid: 2,
                    message_type: 1,
                    text: "texting".to_string(),
                    via_app_id: 0,
                    seq: 1,
                    created_time: 123,
                    verified: false
                })
            };

            self.send(SendMessage(q));

            let chat = db.get_chat_message(1,21).unwrap();
            assert_eq!(chat.text, "texting");

            self.proc.process_event(qevent).unwrap();
        }

        fn start_tests(&self) {
            let db = &self.proc.db;

            println!("Chat tests works correctly.");
        }
    }

    #[test]
    fn chat_tests() {
        println!("Chat events processing tests started.");
        let tester = ChatTester::new();
        tester.start_tests();
    }
}
