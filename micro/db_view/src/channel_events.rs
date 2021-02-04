use shared::{pb, xc};

use crate::{db, events, session,db_trait, db_mem};
use shared::pb::channel_command::SubCommand;
use shared::pb::event_command::Command;
use shared::pb::EventCommand;
use shared::errors::GenErr;

// todo: message+about texts to rich text
// Single Threaded
// #[derive(Debug)]
pub struct ChannelEvents {
    db_old: db::DBCassandra,
    db: Box<dyn db_trait::DB + Send>
}

// Make it Single Thread
// impl !Sync for ChannelEvents{}

impl ChannelEvents {
    pub fn new_mem() -> Self {
        ChannelEvents {
            db_old: db::DBCassandra::new(),
            db: Box::new(db_mem::DBMem::new())
        }
    }
}

impl events::EventProcess for ChannelEvents {
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
                    followers_count: 0,
                    posts_count: 0,
                    likes_count: 0,
                    reshared_count: 0,
                    counts: None,
                };
                self.db.save_channel(&ch)?;

                //todo remove below
                let c = self.db.get_channel(ch.channel_cid as i64)?;
                println!("--> title >> {:}",c.channel_title);
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
                self.db.save_channel_follower(q.channel_cid as i64, q.by_profile_cid as i64)?;
            }
            UnFollowChannel(_) => {}
            Subscribe(_) => {}
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
                    channel_cid: 0,
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
                let mut msg = self.db.get_channel_message(q.channel_cid as i64, q.message_gid as i64)?; //todo

                msg.text = q.new_text;

                self.db.save_channel_message(&msg)?;
            }
            DeleteMessages(q) => {
                for mgid in q.message_gids {
                    let mut msg = self
                        .db
                        .get_channel_message(q.channel_cid as i64, mgid as i64)?;//todo
                    if msg.by_profile_cid == q.by_profile_cid {
                        msg.deleted = true;
                        self.db.save_channel_message(&msg);
                        //todo delete media
                    }
                }
            }
            LikeMessage(q) => {
                self.db.save_message_like(q.message_gid as i64,q.by_profile_cid as i64)?;
            }
            UnLikeMessage(q) => {
                //todo it2
            }
            ReShareMessage(_) => {}
            UnReShareMessage(_) => {}
            AddComment(q) => {
                let com = pb::Comment{
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
