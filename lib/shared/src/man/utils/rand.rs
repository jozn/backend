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
