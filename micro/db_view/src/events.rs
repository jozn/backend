use shared::{pb, xc};

use crate::session;
use crate::{channel_events, channel_events_old, groups_events};
use shared::pb::event_command::Command;

use std::collections::{HashMap, HashSet};
use std::thread;
use std::thread::Thread;
use std::time::Duration;

use std::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct FEventReq {
    event: pb::EventCommand,
    result: tokio::sync::oneshot::Sender<u8>,
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
                let sub_cmd = ch_cmd.sub_command.unwrap();

                let ch = channel_events::ChannelEvents::default();
                self.get_thread_chan(5, event_req, ch);
                /*                match self.threads.get(&4) {
                    None => {
                        let (ch_send, ch_rec) = mpsc::sync_channel::<FEventReq>(5);

                        let handle = thread::spawn(|| {
                            for e in ch_rec {
                                println!("+++ ++++{:?}", 88888);
                                println!(">>>>>>>>{:?}", e.event.event_id);
                                e.result.send(13);

                                // e.result.send(5);
                                // channel_events::handle_channel_events(e);
                            }
                        });

                        // This could block if Shared thread is slow
                        ch_send.send(event_req);

                        self.threads.insert(4, ch_send);
                    }

                    Some(th_handler) => {
                        th_handler.send(event_req);
                    }
                }*/

                // channel_events::handle_channel_events(sub_cmd);
            }
            Command::Group(gr_cmd) => {
                let sub_cmd = gr_cmd.sub_command.unwrap();

                groups_events::handle_group_events(sub_cmd);
            }
        }
    }

    fn get_thread_chan(&mut self, shared_id: u32, event_req: FEventReq, proc: impl EventProcess) {
        match self.threads.get(&shared_id) {
            None => {
                let (ch_send, ch_rec) = mpsc::sync_channel::<FEventReq>(5);

                let handle = thread::spawn(move || {
                    for e in ch_rec {
                        println!("+++ ++++{:?}", 88888);
                        // println!(">>>>>>>>{:?}", &(e.event).event_id);

                        let out_res = proc.process_event(e.event);

                        e.result.send(out_res);

                        // e.result.send(5);
                        // channel_events::handle_channel_events(e);
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
    async fn rpc_user_space_test1() {
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
        println!("hwerwe");
        assert_eq!(2 + 2, 4);
    }
}

////////////////// Old > del
type SMap<T> = HashMap<u32, mpsc::SyncSender<T>>;

#[derive(Debug)]
pub struct EventHandler_Dep {
    threads_channel: SMap<pb::channel_command::SubCommand>,
}

impl EventHandler_Dep {
    pub fn new() -> Self {
        EventHandler_Dep {
            threads_channel: HashMap::new(),
        }
    }

    pub fn process_event_shared(&mut self, event: pb::EventCommand) {
        let cmd = event.command.unwrap();

        println!(
            ">>>>>> proccessing threading event id: {:?}",
            &event.event_id
        );

        match cmd.clone() {
            Command::Channel(ch_cmd) => {
                let sub_cmd = ch_cmd.sub_command.unwrap();

                match self.threads_channel.get(&4) {
                    None => {
                        let (ch_send, ch_rec) = mpsc::sync_channel(5);

                        let handle = thread::spawn(|| {
                            for e in ch_rec {
                                println!("+++ ++++{:?}", 1);
                                channel_events_old::handle_channel_events_dep(e);
                            }
                        });

                        // This could block if Shared thread is slow
                        ch_send.send(sub_cmd);

                        self.threads_channel.insert(4, ch_send);
                    }

                    Some(th_handler) => {
                        th_handler.send(sub_cmd);
                    }
                }

                // channel_events::handle_channel_events(sub_cmd);
            }
            Command::Group(gr_cmd) => {
                let sub_cmd = gr_cmd.sub_command.unwrap();

                groups_events::handle_group_events(sub_cmd);
            }
        }
    }
}
