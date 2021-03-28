// Notes about Arabic, Persian characters:
// + Arabic (Farsi) characters (ex: ز) is represented once in unicode code points, all other form
//      of representation in word is handled by fonts,... . "Arabic Presentation" range is used for
//      compatibility with older device (like ms-doc), it's not used in our samples.
// + Farsi numbers and Arabic is different slightly and have two separate code range.
// + Farsi unique characters (گچپژ) is also represented in base arabic code range (0x0600-0x6ff).
// + We do not translate codes in "Arabic Presentation" to logical code range, Someone like google
//      in it's search result do this (keep this in mind) (ex: ﮑ > ک ).
// + "Harkat" in Arabic texts (ex Quran) is a character is bytes, it's presentation is handled by
//      other means.

// https://en.wikipedia.org/wiki/Arabic_(Unicode_block)
// https://en.wikipedia.org/wiki/Persian_alphabet
// https://en.wikipedia.org/wiki/Arabic_script_in_Unicode

const _FARSI: [char; 32] = [
    'ا', 'ب', 'پ', 'ت', 'ث', 'ج', 'چ', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'ژ', 'س', 'ش', 'ص', 'ض', 'ط',
    'ظ', 'ع', 'غ', 'ف', 'ق', 'ک', 'گ', 'ل', 'م', 'ن', 'و', 'ه', 'ی',
];

fn is_in_faris_range(c: char) -> bool {
    let ch = c as u32;

    if ch >= 0x0600 && ch <= 0x06ff {
        for fa_ch in _FARSI.clone().iter() {
            if ch == fa_ch.clone() as u32 {
                return true;
            }
        }

        // Check for numbers: we count both Farsi and Arabic numbers as persian
        if ch >= 0x06F0 && ch <= 0x06F9 || ch >= 0x0660 && ch <= 0x669 {
            return true;
        }
    }
    false
}

// see: https://en.wikipedia.org/wiki/Arabic_(Unicode_block)
// shared among all languages who uses Arabic alphabet
fn is_in_arabic_range(c: char) -> bool {
    let i = c as u32;
    if i >= 0x0600 && i <= 0x6ff {
        return true;
    }
    false
}

fn is_farsi_text_ratio(txt: &str, ratio: f32) -> bool {
    let t2 = txt.replace(" ", "").replace("\n", "");
    let mut farsi_cnt = 0;

    for c in t2.chars() {
        if is_in_faris_range(c) {
            farsi_cnt += 1;
        }
    }
    (farsi_cnt as f32 / t2.chars().count() as f32) >= ratio
}

pub fn is_farsi_text(txt: &str) -> bool {
    is_farsi_text_ratio(txt, 0.20)
}

fn is_arabic_text_ratio(txt: &str, ratio: f32) -> bool {
    let t2 = txt.replace(" ", "").replace("\n", "");
    let mut arabic_cnt = 0;

    for c in t2.chars() {
        if is_in_arabic_range(c) {
            arabic_cnt += 1;
        }
    }
    (arabic_cnt as f32 / t2.chars().count() as f32) >= ratio
}

pub fn is_arabic_text(txt: &str) -> bool {
    is_arabic_text_ratio(txt, 0.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_farsi_text_test() {
        let fa1 = "شعار ما برادری و باهمی";
        let ar1 = "لَا إِلَٰهَ إِلَّا ٱل‍لَّٰهُ";
        let en1 = "Hello world; this is the first tweet.";
        let mixed1 = "adc123 \n\r ت ❤'\"/\\ 🖐🏿🖐🏿🖐🏿😙漢🖐🏿字在";

        assert_eq!(is_farsi_text_ratio(fa1, 0.7), true);

        let tests: Vec<(&str, f32, bool)> = vec![
            (fa1, 0.7, true),
            (fa1, 0.05, true),
            (ar1, 0.2, true),
            (en1, 0.7, false),
            (en1, 0.05, false),
            (mixed1, 0.2, false),
            (mixed1, 0.02, true),
        ];

        for t in tests {
            assert_eq!(is_farsi_text_ratio(t.0, t.1), t.2);
        }
    }

    // Just used for running playground fns.
    #[test]
    fn play_main() {
        // play_persian();
        // play_chars();
    }

    fn play_persian() {
        let s = "شعار ما برادری و باهمی";
        let s = "لَا إِلَٰهَ إِلَّا ٱل‍لَّٰهُ";
        let s = "ـاُ";
        let s = "گچپژ";

        for z in s.chars() {
            let res = is_in_faris_range(z);
            println!("is char {:} -> out {:#X}", z, z as u32);
        }
    }

    fn play_chars() {
        let s = "adc123 \n\r ت ❤'\"/\\ 🖐🏿🖐🏿🖐🏿😙漢🖐🏿字在";

        for z in s.chars() {
            let res = is_in_faris_range(z);
            println!("is char {:} -> {}", z, res);
        }
    }
}
