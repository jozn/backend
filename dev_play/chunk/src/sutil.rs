// Each file id will be stored in 100 folder based on their last two digits.
// ex: file id 345 will be at 45 sub folder
pub fn file_id_to_folder(id: u64) -> String {
    let rem = id % 100;
    format!("{:02}", rem)
}

// Each bucket will be present at sub folder. This is based on decimal number.
// ex: 5 to x, 67 to xx 234 to 2xx 3451 to 34xx and anything above 10K like 42344 to 42xxx
pub fn bucket_to_folder(id: u32) -> String {
    match id {
        0..=9 => {
            format!("x")
        }
        10..=99 => {
            format!("xx")
        }
        100..=999 => {
            let num = id / 100; // 0 to 9
            format!("{}xx", num)
        }
        1000..=9_999 => { // exactly like 1K to 10K
            let num = id / 100; // 10 to 99
            format!("{}xx", num)
        }
        10_000..=99_999 => { // max 1000 each
            let num = id / 1000; // 10 to 99
            format!("{}xxx", num)
        }
        _ => { // like range 10K to 100K
            let num = id / 1000;
            format!("{}xxx", num)
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_file_id_folder(){
        let arr = vec![
            (0,"00"),
            (4,"04"),
            (10,"10"),
            (24,"24"),
            (75,"75"),
            (234,"34"),
            (999,"99"),
            (1000,"00"),
            (1001,"01"),
            (125_761,"61"),
        ];

        for t in arr {
            // println!("{} {}",t.0,file_id_to_folder(t.0));
            assert_eq!(file_id_to_folder(t.0), t.1);
        }
    }

    #[test]
    fn test_folder(){
        let arr = vec![
            (0,"x"),
            (4,"x"),
            (10,"xx"),
            (24,"xx"),
            (75,"xx"),
            (99,"xx"),
            (100,"1xx"),
            (129,"1xx"),
            (199,"1xx"),
            (200,"2xx"),
            (234,"2xx"),
            (999,"9xx"),
            (1000,"10xx"),
            (1999,"19xx"),
            (5099,"50xx"),
            (9999,"99xx"),
            (10000,"10xxx"),
            (11000,"11xxx"),
            (71981,"71xxx"),
            (99_999,"99xxx"),
            (125_761,"125xxx"),
        ];

        for t in arr {
            // println!("{} {}",t.0,bucket_to_folder(t.0));
            assert_eq!(bucket_to_folder(t.0), t.1);
        }
    }
}