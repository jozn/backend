use shared::{pb, xc};

use crate::{db, db_mem, db_trait, events, session};
use shared::errors::GenErr;
use shared::pb::channel_command::SubCommand;
use shared::pb::event_command::Command;
use shared::pb::EventCommand;

// todo: message+about texts to rich text
// Single Threaded
// #[derive(Debug)]
pub struct EventChannel {
    db_old: db::DBCassandra,
    db: Box<dyn db_trait::DB + Send>,
}

// Make it Single Thread
// impl !Sync for ChannelEvents{}

impl EventChannel {
    pub fn new_mem() -> Self {
        EventChannel {
            db_old: db::DBCassandra::new(),
            db: Box::new(db_mem::DBMem::new()),
        }
    }
}

impl events::EventProcess for EventChannel {
    fn process_event(&self, event: EventCommand) -> Result<bool, GenErr> {
        let ch_sub = conv_to_channel_sub_command(event.clone());

        use SubCommand::*;
        match ch_sub {
            CreateChannel(q) => {
                let ch = pb::Channel {
                    // todo add derive Default
                    channel_cid: q.channel_cid,
                    creator_profile_cid: q.creator_profile_cid,
                    is_profile_channel: false, // todo
                    created_time: 3,           //todo
                    user_name: q.user_name,
                    channel_title: q.channel_title,
                    about: q.about,
                    is_verified: false,
                    sync_time_ms: 0,
                    is_deleted: 0,
                    is_banned: 0,
                    invite_link_hash: "".to_string(),
                    notification_setting: None,
                    privacy: 0,
                    last_message: None,
                    message_seq: 0,
                    pinned_message: None,
                    avatar: None,
                    avatar_count: 0,
                    inboxer: None,
                    followers_count: 0,
                    posts_count: 0,
                    likes_count: 0,
                    reshared_count: 0,
                    counts: None,
                };
                self.db.save_channel(&ch)?;

                //todo remove below
                let c = self.db.get_channel(ch.channel_cid as i64)?;
                println!("--> title >> {:}", c.channel_title);
                //self.db_old.save_channel(&ch);
                // self.db.save_channel_message()
            }
            EditChannel(q) => {
                let mut ch = self.db.get_channel(q.channel_cid as i64)?;

                if q.set_new_title && q.new_title.len() > 0 {
                    ch.channel_title = q.new_title;
                }
                if q.set_new_about {
                    ch.about = q.new_about;
                }

                self.db.save_channel(&ch)?;
            }
            DeleteChannel(q) => {
                let mut ch = self.db.get_channel(q.channel_cid as i64)?;

                ch.is_deleted = 1; //todo

                self.db.save_channel(&ch)?;
                // todo delete msgs, follwers,...
            }
            FollowChannel(q) => {
                self.db
                    .save_channel_follower(q.channel_cid as i64, q.by_profile_cid as i64)?;
            }
            UnFollowChannel(_) => {}
            Subscribe(_) => {} // todo @ it 4
            UnSubscribe(_) => {}
            SendMessage(q) => {
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
                    channel_cid: q.channel_cid,
                    setting: None,
                    counts: None,
                    group_cid: 0,
                    files: vec![],
                    product: None,
                };

                self.db.save_channel_message(&m)?;
                //todo add media
            }
            EditMessage(q) => {
                let mut msg = self
                    .db
                    .get_channel_message(q.channel_cid as i64, q.message_gid as i64)?; //todo

                msg.text = q.new_text;

                self.db.save_channel_message(&msg)?;
            }
            DeleteMessages(q) => {
                for mgid in q.message_gids {
                    let mut msg = self
                        .db
                        .get_channel_message(q.channel_cid as i64, mgid as i64)?; //todo
                    if msg.by_profile_cid == q.by_profile_cid {
                        msg.deleted = true;
                        self.db.save_channel_message(&msg);
                        //todo delete media
                    }
                }
            }
            LikeMessage(q) => {
                self.db
                    .save_message_like(q.message_gid as i64, q.by_profile_cid as i64)?;
            }
            UnLikeMessage(q) => {
                //todo it2
            }
            ReShareMessage(_) => {} // todo @ it 4
            UnReShareMessage(_) => {}
            AddComment(q) => {
                let com = pb::Comment {
                    comment_gid: 1, // todo add in QEvent comment_gid
                    message_gid: q.message_gid,
                    profile_cid: q.by_profile_cid,
                    created_time: 16000000,
                    text: q.comment_text,
                };
                self.db.save_message_comment(&com)?;
            }
            DeleteComment(_) => {}
            AvatarAdd(_) => {}
            AvatarDelete(_) => {}
        }

        let x = 1;
        Ok(true)
    }
}

fn conv_to_channel_sub_command(event: EventCommand) -> pb::channel_command::SubCommand {
    let cmd = event.command.unwrap();
    match cmd {
        Command::Channel(ch_cmd) => ch_cmd.sub_command.unwrap(),
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
        proc: EventChannel,
    }

    impl ChannelTester {
        fn new() -> Self {
            ChannelTester {
                proc: EventChannel::new_mem(),
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

            //======  CreateChannel
            let q = QEditChannel {
                channel_cid: CHANNEL_CID,
                by_profile_cid: PROFILE_CID,
                set_new_title: true,
                new_title: "NT".to_string(),
                set_new_about: true,
                new_about: "NABOUT".to_string(),
            };
            self.send(EditChannel(q));

            let ch = db.get_channel(CHANNEL_CID as i64).unwrap();
            assert_eq!(ch.channel_title, "NT".to_string());
            assert_eq!(ch.about, "NABOUT".to_string());

            //======  FollowChannel
            let q = QFollowChannel {
                channel_cid: CHANNEL_CID,
                by_profile_cid: 107,
            };
            self.send(FollowChannel(q));

            let ch = db.get_channel_followers(CHANNEL_CID as i64).unwrap();
            assert!(ch.get(0).unwrap() == &107);

            //======  SendMessage
            let q = QSendMessage {
                channel_cid: CHANNEL_CID,
                message_input: Some(NewMessageInput {
                    message_gid: 1000,
                    by_profile_cid: PROFILE_CID,
                    message_type: 1,
                    text: "ver1".to_string(),
                    via_app_id: 0,
                    seq: 1,
                    created_time: 2,
                    verified: false,
                }),
            };
            self.send(SendMessage(q));

            let msg = db.get_channel_message(CHANNEL_CID as i64, 1000).unwrap();
            assert!(msg.text.eq("ver1"));

            //======  EditMessage
            let q = QEditMessage {
                channel_cid: CHANNEL_CID,
                message_gid: 1000,
                by_profile_cid: PROFILE_CID,
                new_text: "ver2".to_string(),
            };
            self.send(EditMessage(q));

            let msg = db.get_channel_message(CHANNEL_CID as i64, 1000).unwrap();
            assert!(msg.text.eq("ver2"));

            //======  DeleteMessages
            let q = QDeleteMessages {
                channel_cid: CHANNEL_CID,
                by_profile_cid: PROFILE_CID,
                message_gids: vec![1000],
            };
            self.send(DeleteMessages(q));

            let msg = db.get_channel_message(CHANNEL_CID as i64, 1000).unwrap();
            assert!(msg.deleted == true);

            //======  LikeMessage
            let q = QLikeMessage {
                channel_cid: CHANNEL_CID,
                message_gid: 1000,
                by_profile_cid: 212,
            };
            self.send(LikeMessage(q));

            let arr = db.get_message_likes(1000).unwrap();
            assert_eq!(arr.get(0).unwrap(), &212);

            //======  AddComment
            let q = QAddComment {
                channel_cid: CHANNEL_CID,
                message_gid: 1000,
                by_profile_cid: 212,
                comment_text: "comment1".to_string(),
            };
            self.send(AddComment(q));

            let arr = db.get_message_comments(1000).unwrap();
            assert_eq!(arr.get(0).unwrap().text, "comment1".to_string());

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
