// extern crate backbone;

use backbone;
use image;
use image::GenericImageView;

fn main() {
    println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");

    let imgs = std::fs::read_dir("/home/hamid/life/__files__/Telegram/images").unwrap();
    for ig in imgs {
        let ig = ig.unwrap();
        let img = image::open(ig.path()).unwrap();

        println!("{:?}", img.dimensions());
    }
    // image::open()
}
