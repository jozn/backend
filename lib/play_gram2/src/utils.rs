use rand;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::ops::Index;
use std::str::FromStr;

// A general username validity check: relatively relaxed | not suitable for flip
// Not checking length: length should be validated at some other places:
// no start with number but twitter allows
// Instagram: 1..=30 _. no repeated dots | allowed __ | allowed _ start | not allowed num start
// Telegram: 5..=32 _ no ending with _ | allowed __ | no _ start | not allowed num start
// Twitter: 1..=15 _ | allowed __ | allowed _ start | allowed num start and total number
pub fn is_valid_username_pattern(s: &str) -> bool {
    if s.len() == 0 || !s.is_ascii() {
        return false;
    }

    let all_char_res = s.chars().all(|c| match c {
        'A'..='Z' | 'a'..='z' | '0'..='9' | '.' | '_' => true,
        _ => false,
    });

    let first_char: char = s.chars().next().unwrap();
    let first_char_res = match first_char {
        'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => true,
        _ => false,
    };

    let last_char: char = s.chars().last().unwrap();
    let last_char_res = match last_char {
        'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => true,
        _ => false,
    };

    let no_repeated_dots = !s.contains("..");

    all_char_res && first_char_res && last_char_res && no_repeated_dots
}

pub fn insert_tkanals_into_db() {
    println!("Start inserting into username table ...");
    let chs = read_tkanal_channels();

    for ch in chs {
        crate::db_dep::save_queue_username(&ch);
    }
    println!("Inserted usernames into table.");
}

pub fn read_tkanal_channels() -> Vec<String> {
    let f = fs::read("./lib/play_gram1/src/tkanal.txt").unwrap();
    let s = String::from_utf8(f).unwrap();
    let arr: Vec<String> = s.split("\n").map(|z| z.to_string()).collect();
    arr
}

pub fn read_tkanal_channels_rand() -> String {
    let f = fs::read("./lib/play_gram1/src/tkanal.txt").unwrap();
    // let s = f.to_bytes().to_str().unwrap();
    let s = String::from_utf8(f).unwrap();
    let arr: Vec<&str> = s.split("\n").collect();
    let rnd = rand::thread_rng().gen_range(0, arr.len());

    let kanal = arr.index(rnd).to_string();

    kanal
}

pub fn time_now_sec() -> u32 {
    let t = std::time::SystemTime::now();
    t.duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}

pub fn get_file_extension_from_mime_type(mt: &str) -> String {
    let out = match mt {
        "image/jpeg" => "jpg",
        "audio/mpeg" => "mp3",
        "audio/midi" => "midi",
        "audio/ogg" => "ogg",
        "text/x-pascal" => "pas",
        "text/x-asm" => "asm",
        "video/quicktime" => "mov",
        _ => mime_db::extension(mt).unwrap_or("unk"),
    };

    format!(".{}", out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types;

    #[test]
    fn valid_username() {
        struct T(&'static str, bool);
        let arr = vec![
            T("absdef", true),
            T("absdef25", true),
            T("absef_sd", true),
            T("absdef_25", true),
            T("a", true),
            T("ABC25_tUlM_23t", true),
            T("12abcdef", true),
            T("abcdef_", true),
            T("_abcdef", true),
            T("_abcdef_", true),
            T("_", true),
            T("8abcdef", true),
            T("abf45sdef-12", false),
            T("ABC25-tUlM_23t-", false),
            T("abcd ef", false),
            T("@abcdef", false),
            T("abcd!ef", false),
            T("abcd-ef", false),
            T("abcd?ef", false),
            T("abcd/ef", false),
            T("abcd\\ef", false),
            T("abcdسef", false),
            T("", false),
            T(".", false),
        ];
        for i in arr {
            assert_eq!(is_valid_username_pattern(i.0), i.1, "{}", i.0);
        }
    }

    #[test]
    fn test_mime_types() {
        assert_eq!(mime_db::extension("image/png").unwrap(), "png");
        assert_eq!(mime_db::extension("image/jpeg").unwrap(), "jpeg");
        assert_eq!(mime_db::extension("image/gif").unwrap(), "gif");
        assert_eq!(mime_db::extension("image/webp").unwrap(), "webp");
        assert_eq!(mime_db::extension("video/mp4").unwrap(), "mp4");
        assert_eq!(mime_db::extension("audio/midi").unwrap(), "mid"); // midi
        assert_eq!(mime_db::extension("audio/mpeg").unwrap(), "mpga"); // mp3
        assert_eq!(mime_db::extension("audio/ogg").unwrap(), "oga"); //
        assert_eq!(mime_db::extension("video/quicktime").unwrap(), "qt"); // mov
    }

    #[test]
    fn utf_16() {
        use types::{MsgTextMeta, MsgTextMetaType::*};

        let msg = types::Msg{
            silent: false,
            post: true,
            id: 33,
            from_id: 33,
            via_bot_id: 0,
            reply_to_msg_id: 0,
            date: 1615726102,
            message: "utf16 تست👌at strikes bold under  hyperlink 😍 #iraq space ططط hhw@gmail.com dd اا ایتالیک mono  漢字 Google.com".to_string(),
            text_meta: vec![
                MsgTextMeta {
                    meta_type: Strike,
                    offset: 14,
                    length: 7,
                    ..Default::default()
                },
                MsgTextMeta {
                    meta_type: Bold,
                    offset: 22,
                    length: 4,
                    ..Default::default()
                },
                MsgTextMeta {
                    meta_type: Underline,
                    offset: 27,
                    length: 5,
                    ..Default::default()
                },
                MsgTextMeta {
                    meta_type: TextUrl,
                    offset: 34,
                    length: 9,
                    url: "http://flip.ir/".to_string(),
                },
                MsgTextMeta {
                    meta_type: Hashtag,
                    offset: 47,
                    length: 5,
                    ..Default::default()
                },
                MsgTextMeta {
                    meta_type: Email,
                    offset: 63,
                    length: 13,
                    ..Default::default()
                },
                MsgTextMeta {
                    meta_type: Italic,
                    offset: 83,
                    length: 7,
                    ..Default::default()
                },
                MsgTextMeta {
                    meta_type: Code,
                    offset: 91,
                    length: 4,
                    ..Default::default()
                },
                MsgTextMeta {
                    meta_type: Url,
                    offset: 100,
                    length: 10,
                    ..Default::default()
                },
            ],
            views: 1,
            edit_date: 1615749669,
            restricted: false,
            forward: None,
            media_old: None,
            webpage_old: None,
            medias: vec![],
            glassy_urls: None,
        };

        let utf_16_txt: Vec<u16> = msg.message.encode_utf16().collect();

        let check = |txt, offset, length| {
            assert_eq!(
                txt,
                String::from_utf16(&utf_16_txt[offset..(offset + length)]).unwrap()
            );
        };

        check("strikes", 14, 7);
        check("bold", 22, 4);
        check("under", 27, 5);
        check("hyperlink", 34, 9);
        check("#iraq", 47, 5);
        check("hhw@gmail.com", 63, 13);
        check("ایتالیک", 83, 7);
        check("mono", 91, 4);
        check("Google.com", 100, 10);

        println!("utf16 slice: {:?}", &utf_16_txt[14..21]);
        println!(
            "utf16 slice str: {:?}",
            String::from_utf16(&utf_16_txt[14..21])
        );

        println!("utf16 works correctly");
    }

    fn tuf16_to_string(bts: &[u16]) -> String {
        String::from_utf16(bts).unwrap()
    }
}
