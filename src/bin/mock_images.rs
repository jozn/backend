// extern crate backbone;

use backbone;
use image;
use image::imageops::crop_imm;
use image::GenericImageView;
use std::ops::Add;

fn main() {
    _extract_images_mod("/home/hamid/life/__files__/Telegram/images", "./src/mock/images.rs");
    _extract_images_mod("/home/hamid/life/__files__/avatars", "./src/mock/avatars.rs");
}

fn _extract_images_mod(input_dir: &str, out_rust_file: &str) {
    let imgs = std::fs::read_dir(input_dir).unwrap();
    println!("init vec of image files");

    let mut buff = String::new();

    let mut i = 0;
    for ig in imgs {
        let path = ig.unwrap().path();
        let dim = image::open(&path).unwrap().dimensions();
        let out = format!(
            "Image{{src: {:?},width:{},height:{} }},\n \t\t",
            &path, dim.0, dim.1
        );
        buff = buff.add(&out);
        println!("{} : {}", i, &out);

        i += 1;
        if i == 3 {
            // break;
        }
    }

    let out_file = format!(
        r#"//GENERATED CODE

pub struct Image {{
    pub src: &'static str,
    pub width: u32,
    pub height: u32,
}}

pub fn get_images() -> Vec<Image> {{
    vec![
        {:}
    ]
}}

"#,
        buff
    );

    // println!("{}",&out_file);

    std::fs::write(out_rust_file, out_file);
}
