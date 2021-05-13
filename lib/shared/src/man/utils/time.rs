pub fn get_time_now_sec() -> u32 {
    use std::time::{Duration, SystemTime};

    let now = SystemTime::now();
    let res = now
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Wong clock");

    res.as_secs() as u32
}

pub fn get_time_now_milli() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Wrong Clock")
        .as_millis() as u64
}

pub fn get_time_now() -> std::time::Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Wrong Clock")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play1() {
        for _ in 0..100 {
            let t = get_time_now_milli();
            println!("{}", t);
        }
    }

    #[test]
    fn play2() {
        for _ in 0..100 {
            let t = get_time_now();
            println!("{}", t.as_nanos());
        }
    }
}
