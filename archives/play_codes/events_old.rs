use shared::{pb, xc};

use crate::session;
use crate::{channel_events, groups_events};
use shared::pb::event_command::Command;

use std::collections::{HashMap, HashSet};
use std::thread;
use std::thread::Thread;
use std::time::Duration;

use prost::encoding::WireType::ThirtyTwoBit;
use std::sync::mpsc;

pub fn process_event(event: pb::EventCommand) {
    let cmd = event.command.unwrap();

    println!("proccing event id: {:?}", &event.event_id);

    match cmd {
        Command::Channel(ch_cmd) => {
            let sub_cmd = ch_cmd.sub_command.unwrap();
            channel_events::handle_channel_events(sub_cmd);
        }
        Command::Group(gr_cmd) => {
            let sub_cmd = gr_cmd.sub_command.unwrap();

            groups_events::handle_group_events(sub_cmd);
        }
    }
}

#[derive(Debug)]
struct EventSharedSelect {
    cmd: pb::event_command::Command,
    shared: u32,
}

#[derive(Debug)]
pub struct EventHandler2<T> {
    threads: std::collections::HashMap<u32, T>,
    threads_channel: Vec<T>,
    threads_channel2: Vec<mpsc::Sender<T>>,
    threads_channel3: HashMap<u32, mpsc::Sender<T>>,
}

#[derive(Debug)]
pub struct EventHandler {
    threads: std::collections::HashMap<u32, ThreadHandler>,
}

#[derive(Debug)]
struct ThreadHandler {
    thread: thread::JoinHandle<()>,
    shared_id: u32,
    ch: std::sync::mpsc::Sender<EventSharedSelect>,
}

#[derive(Debug)]
struct ThreadHandler2<T> {
    ch: std::sync::mpsc::Sender<EventSharedSelect>,
}

///////////// impl ////////////
impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            threads: HashMap::new(),
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
                let es = EventSharedSelect {
                    cmd: cmd,
                    shared: 4,
                };

                match self.threads.get(&es.shared) {
                    None => {
                        let (ch_send, ch_rec) = mpsc::channel();

                        let handle = thread::spawn(|| {
                            for e in ch_rec {
                                println!("+++ {:?}", e);
                                // channel_events::handle_channel_events(e.cmd);
                            }
                            // for Some(e) in ch_rec.try_recv() {
                            //     // println!("hi number {} from the spawned thread!", i);
                            //     // thread::sleep(Duration::from_millis(1));
                            // }
                        });

                        let th = ThreadHandler {
                            thread: handle,
                            shared_id: 4,
                            ch: ch_send.clone(),
                        };

                        self.threads.insert(th.shared_id, th);
                        ch_send.send(es);
                    }

                    Some(th_handler) => {
                        th_handler.ch.send(es);
                    }
                }

                let sub_cmd = ch_cmd.sub_command.unwrap();

                // channel_events::handle_channel_events(sub_cmd);
            }
            Command::Group(gr_cmd) => {
                let sub_cmd = gr_cmd.sub_command.unwrap();

                groups_events::handle_group_events(sub_cmd);
            }
        }
    }

    pub fn sleep(&self) {
        for (_, h) in &self.threads {
            // h.thread.join().unwrap();
        }
    }
}

pub fn process_event_shared(event: pb::EventCommand) {
    let cmd = event.command.unwrap();

    println!("proccing event id: {:?}", &event.event_id);

    match cmd.clone() {
        Command::Channel(ch_cmd) => {
            let es = EventSharedSelect {
                cmd: cmd,
                shared: 4,
            };

            let sub_cmd = ch_cmd.sub_command.unwrap();

            channel_events::handle_channel_events(sub_cmd);
        }
        Command::Group(gr_cmd) => {
            let sub_cmd = gr_cmd.sub_command.unwrap();

            groups_events::handle_group_events(sub_cmd);
        }
    }

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
