use std::time::{Instant};
mod utils;
mod preprocessing;
use crate::preprocessing::prep::preprocess_pub;
use std::env;
use std::fs;


pub fn main() {

    let start = Instant::now();

    // Define padding as a configurable variable
    let ksize: u32 = 1;  // You can change this value to 1 or 2 to test different kernel size // TODO MODIFY FILTERING CODE
    let padding: u32 = ksize;
    let kernel = vec![
        vec![1, 2, 1],
        vec![2, 4, 2],
        vec![1, 2, 1],
    ];

    let curr = std::env::current_dir().expect("Failed to get current directory");
    let args:Vec<String> = env::args().collect();
    let idx_img = args[1].parse::<u8>().unwrap();

    let mut inpath = curr.join(&format!("images/{:?}/in.png", idx_img));
    let mut outpath = curr.join(&format!("images/{:?}/out.png", idx_img));

    if !fs::metadata(inpath.clone()).is_ok() {
        inpath = curr.join(&format!("images/{:?}/in.jpg", idx_img));
        outpath = curr.join(&format!("images/{:?}/out.jpg", idx_img));
    }


    let inimg = image::open(&inpath).expect(&format!("Failed to open image {}", inpath.display()));

    let outimg = preprocess_pub(inimg, padding, &kernel, ksize);

    outimg.save(&outpath).expect(&format!("Failed to save image {}", outpath.display()));

    let duration = start.elapsed();
    println!("duration={:?}", duration);

}

