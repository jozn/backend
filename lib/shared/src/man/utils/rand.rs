use rand;

// sample output: "5Jn6POPBZZ2yiE3"
pub fn rand_string(length: u32) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect();

    rand_string
}

// sample output: "M(@y#JR(%6ATD4*"
pub fn rand_chars_string(length: u32) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    // const PASSWORD_LEN: usize = length as usize;
    let mut rng = rand::thread_rng();

    let password: String = (0..length as usize)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    password
}

// returns a random id for session. Session ids are ordered like UUID. The first 8 bytes are just
//  a clock generated part so choose a length well above 12;
// sample output: "1ad1f9e8cTh5whAvfd"
pub fn get_rand_session(length: u32) -> String {
    let length = if length >= 12 { length } else { 12 };

    // We manipulate time to not have strict format with current time, in order for formatted
    //  4 bytes to be 8 hex it should be bigger than ( 1<<28 ).  This number is a magic and
    //  arbitrary chosen.
    let time_part = super::time::get_time_now_sec() - 1_193_000_000;
    let hash = rand_string(length - 8).to_lowercase();
    format!("{:x}{}", time_part, hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_str() {
        let str = rand_string(20);
        println!("sample: {}", str)
    }

    #[test]
    fn test_rand_str_loop() {
        for i in 0..100 {
            test_rand_str()
        }
    }

    #[test]
    fn test_rand_chars_str() {
        let str = rand_chars_string(15);
        println!("sample: {}", str)
    }

    #[test]
    fn play_session_loop() {
        for i in 0..100 {
            let s = get_rand_session(18);
            println!("Session: {}", s)
        }
    }

    #[test]
    fn play_rand_str2() {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        println!("{}", rand_string);
    }

    #[test]
    fn play_rand_str3() {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
        const PASSWORD_LEN: usize = 30;
        let mut rng = rand::thread_rng();

        let password: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        println!("{:?}", password);
    }
}
