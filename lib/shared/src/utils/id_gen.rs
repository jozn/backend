extern crate rand;
use rand::Rng;
use std::fmt::Formatter;
use std::fmt::{Display, Result};
use std::thread;
use std::time;
use std::time::Duration;

const SEQ_MAX: u16 = 4096 - 1; // 1..=4095 - 2^12
const SHARD_MAX: u16 = 1024 - 1; // 1..=1023 - 2^10
const STARTING_TIME: u64 = 1262304000_000; // Friday, January 1, 2010 12:00:00 AM

trait GenTime {
    fn get_time_ms() -> u64 {
        get_time_milli()
    }
}

/// This class makes sequntioal id gen - u64 with +50 years support + it 63 bit int so is safe to
/// use in java. seq will always be greater than pervious ones even if clocks go backward.
/// single threaded
/// sample:
///    let mut gen = ::SeqTimeIdGen::new(75); //75 is the number of shared
///    let id = gen.get_next_id().to_u64();

#[derive(Default, Debug)]
pub struct SeqTimeIdGen {
    last_gen_id: SeqId,
    last_time_ms: u64,
    share_id: u16,
    seq: u16,              // consumed
    seq_started_from: u16, // immutable - we can change it with seq_rotation_time_ms and it gives us more seq, but this is good enough
    seq_rotation_time_ms: u64,
    count: u64,
    is_debug: bool,
    //fx: Box<dyn Fn() -> u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct SeqId(u64, u16, u16);

impl SeqTimeIdGen {
    pub fn new_default() -> Self {
        let mut d = SeqTimeIdGen {
            share_id: rand::thread_rng().gen_range(1, SHARD_MAX),
            ..Default::default()
        };
        d.set_seq_start_from(rand::thread_rng().gen_range(1, SEQ_MAX));
        d
    }

    pub fn new(shared_id: u16) -> Self {
        let mut d = Self::new_default();
        d.share_id = shared_id;
        d
    }

    fn set_seq_start_from(&mut self, seq: u16) {
        self.seq = seq;
        self.seq_started_from = seq;
    }

    pub fn get_next_id(&mut self) -> SeqId {
        let mut now_ms = if self.is_debug {
            get_time_milli_debug()
        } else {
            get_time_milli()
        };
        let mut next_seq = self.seq + 1;
        if next_seq >= SEQ_MAX {
            next_seq = 0;
        }

        if now_ms > self.last_time_ms {
            // everything is fine
            self.seq_rotation_time_ms = now_ms;
        } else {
            // set time our current time to last id genrated time - do not let go backward
            now_ms = self.last_time_ms;
            // if we consumed enough seq (at max 40960 just go to next millisecond
            if next_seq == self.seq_started_from {
                now_ms = self.last_time_ms + 1;
                self.seq_rotation_time_ms = now_ms;
            }
        }

        let time_adjusted = now_ms - STARTING_TIME;

        let id2 = SeqId(time_adjusted, self.share_id, next_seq);
        self.count += 1;
        self.seq = next_seq;
        self.last_time_ms = now_ms;
        self.last_gen_id = id2.clone();

        id2.clone()
    }
}

impl SeqId {
    pub fn to_u64(&self) -> u64 {
        let time_part = self.0 << (64 - 1 - 41);
        let shared_part = (self.1 as u64) << (64 - 1 - 41 - 10);
        let seq_part = (self.2 as u64);

        let id = time_part | shared_part | seq_part;
        id
    }

    pub fn to_string(&self) -> String {
        let (time_part, shared_part, seq_part, id) = self.build();

        let str_id = format!("{:b}", id);
        let str = format!("{:b} {:b} {:b} {:b}", time_part, shared_part, seq_part, id);
        // println!("{}", str);
        str.clone()
    }

    fn debug1(&self) {
        let (time_part, shared_part, seq_part, id) = self.build();

        let s = format!("{:b}", time_part);
        let str_time = s.trim_end_matches('0');

        let s = format!("{:b}", shared_part);
        let str_shared = s.trim_end_matches('0');

        let s = format!("{:b}", seq_part);
        let str_seq = s.trim_end_matches('0');

        let str = format!("{:b} \n{:}.{:}.{:}", id, str_time, str_shared, str_seq);
        // let str = format!("{:b} \n{:b} {:b} {:b}", id, time_part, shared_part, seq_part);
        println!("**************************************\n{}", str);
    }

    fn build(&self) -> (u64, u64, u64, u64) {
        let time_part = self.0 << (64 - 1 - 41);
        let shared_part = (self.1 as u64) << (64 - 1 - 41 - 10);
        let seq_part = (self.2 as u64);

        let id = time_part | shared_part | seq_part;
        (time_part, shared_part, seq_part, id)
    }
}

fn get_time_nano() -> u64 {
    let m = time::SystemTime::now();
    let nano = m.duration_since(time::UNIX_EPOCH).unwrap().as_nanos();
    nano as u64
}

fn get_time_milli() -> u64 {
    get_time_nano() / 1000_000
}

// TODO: make this mock dynamic instead of hard coded
fn get_time_milli_debug() -> u64 {
    1_500_000_000__000
}

#[cfg(test)]
mod tests {
    use super::STARTING_TIME;
    use std::collections::HashSet;
    use std::thread;
    use std::time;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        play1();
    }

    #[test]
    fn time_gen() {
        let mut g = super::SeqTimeIdGen::new(50);
        g.is_debug = false;

        g.set_seq_start_from(1000);
        let id1 = g.get_next_id();
        assert_eq!(g.last_gen_id, id1);

        let time = g.last_time_ms - STARTING_TIME;
        assert_eq!(time, id1.0);

        for i in 1..8200 {
            let id1 = g.get_next_id();
            if i > 4900 {
                println!("{:?}  {:?}", id1, g);
            }
        }
    }

    // make sure if clocks goes backwards or stuck, or we producing a lot of ids they are unique
    #[test]
    fn no_backward_time_gen() {
        let mut g = super::SeqTimeIdGen::new(91);
        g.is_debug = true;
        g.set_seq_start_from(0);

        let mut set = std::collections::HashSet::new();
        for i in 1..1_000_000 {
            let id1 = g.get_next_id();
            assert!(!set.contains(&id1));
            set.insert(id1);
        }
        println!("No duplicate ids with fixed time!");
    }

    #[test]
    fn no_backward_time_gen_realtime() {
        let mut g = super::SeqTimeIdGen::new(91);
        g.is_debug = false;

        let mut set = std::collections::HashSet::new();
        for i in 1..=1_000_000 {
            let id1 = g.get_next_id();
            assert!(!set.contains(&id1));

            // Sample print
            // Result: it seems it pushes seconds 3 seconds to future to produce 1 million ids
            if i == 1 || i == 1_000_000 {
                println!("{:?} {}", id1, i);
            }
            set.insert(id1);
        }
        println!("No duplicate ids with real word time!");
    }

    #[test]
    fn no_duplicate_realtime() {
        let mut g = super::SeqTimeIdGen::new_default();

        let mut set = std::collections::HashSet::new();
        for i in 1..=1_000_000 {
            let id1 = g.get_next_id().to_u64();
            assert!(!set.contains(&id1));

            // Sample print
            if i == 1 || i == 1_000_000 {
                //println!("{} {}", id1, i);
            }
            set.insert(id1);
        }
        println!("No duplicate ids with real word time and real ids (u64)!");
    }

    fn play1() {
        let mut g = super::SeqTimeIdGen::new(763);
        for i in 1..100 {
            // println!("{} {:?}", g.get_str_for_debug() , g );
            let next = g.get_next_id().to_string();
            println!("{} {:?}", next, g);
            thread::sleep(time::Duration::from_millis(4));
        }

        for i in 1..10 {
            // println!("{} {:?}", g.get_str_for_debug() , g );
            let id = g.get_next_id();
            //        id.debug1();
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
