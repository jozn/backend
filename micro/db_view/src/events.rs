use shared::{pb, xc};

use crate::session;
use crate::{channel_events, group_events};
use shared::pb::event_command::Command;

use std::collections::{HashMap, HashSet};
use std::thread;
use std::thread::Thread;
use std::time::Duration;

use std::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct FEventReq {
    pub event: pb::EventCommand,
    pub result: tokio::sync::oneshot::Sender<u8>,
}

#[derive(Debug)]
pub struct EventHandler {
    threads: HashMap<u32, mpsc::SyncSender<FEventReq>>,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            threads: HashMap::new(),
        }
    }

    pub fn process_event_shared(&mut self, event_req: FEventReq) {
        let cmd = event_req.event.command.clone().unwrap();

        println!(
            ">>>>>> proccessing threading event id: {:?}",
            &event_req.event.event_id
        );

        match cmd.clone() {
            Command::Channel(ch_cmd) => {
                let sub_cmd = ch_cmd.sub_command.unwrap();//del
                let ch = channel_events::ChannelEvents::default();
                self.get_thread_chan(5, event_req, ch);
            }
            Command::Group(gr_cmd) => {
                let sub_cmd = gr_cmd.sub_command.unwrap();
                let gr = group_events::GroupEvents::default();
                self.get_thread_chan(5, event_req, gr);
            }
        }
    }

    fn get_thread_chan(&mut self, shared_id: u32, event_req: FEventReq, proc: impl EventProcess) {
        match self.threads.get(&shared_id) {
            None => {
                let (ch_send, ch_rec) = mpsc::sync_channel::<FEventReq>(5);

                let handle = thread::spawn(move || {
                    for e in ch_rec {
                        // println!("+++ ++++{:?}", 88888);
                        let out_res = proc.process_event(e.event);

                        e.result.send(out_res);
                    }
                });

                // This could block if Shared thread is slow
                ch_send.send(event_req);
                self.threads.insert(4, ch_send);
            }

            Some(th_handler) => {
                th_handler.send(event_req);
            }
        }
    }
}

pub trait EventProcess: Send + Sync + 'static {
    fn process_event(&self, event: pb::EventCommand) -> u8;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn play_sample_channel_event() {
        let mut h = EventHandler::new();

        for i in 1..=10 {
            let q = pb::channel_command::QCreateChannel {
                channel_cid: i,
                creator_profile_cid: i % 10 + 1,
                channel_title: format!("channel #{}", i),
                user_name: format!("ch_username#{}", i),
                history_viewable: false,
                force_join: false,
                global_search: false,
                about: "XXXXMy channel".to_string(),
            };

            let cmd = pb::channel_command::SubCommand::CreateChannel(q);

            let chcmd = pb::ChannelCommand {
                channel_id: 4,
                sub_command: Some(cmd),
            };

            let qevent = shared::pb::EventCommand {
                event_id: i as u64,
                user_id: 2,
                command: Some(pb::event_command::Command::Channel(chcmd)),
                // command: pb::event_command::Command::Channel(pb::ChannelCommand)
            };

            let (send, rec) = oneshot::channel();
            let event_req = FEventReq {
                event: qevent,
                result: send,
            };

            h.process_event_shared(event_req);
            let r = rec.await;
            println!("{:?}", r);
            // send_channel_cmd(i as u64, cmd);
        }
    }

    #[test]
    fn it_works() {
        println!("it works!");
    }
}
