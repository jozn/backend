use shared::{pb, xc};

use crate::session;
use crate::{channel_events, groups_events};
use shared::pb::event_command::Command;

use std::collections::{HashMap, HashSet};
use std::thread;
use std::thread::Thread;
use std::time::Duration;

use std::sync::mpsc;

type SMap<T> = HashMap<u32, mpsc::SyncSender<T>>;

#[derive(Debug)]
pub struct EventHandler {
    threads_channel: SMap<pb::channel_command::SubCommand>,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
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
                                channel_events::handle_channel_events(e);
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
