use std::time::{Instant};
mod utils;
mod preprocessing;
use crate::preprocessing::prep::preprocess_pub;

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
    let inpath = curr.join("noisy.png");
    let outpath = curr.join("out.png");

    let inimg = image::open(&inpath).expect(&format!("Failed to open image {}", inpath.display()));

    let outimg = preprocess_pub(inimg, padding, &kernel, ksize);

    outimg.save(&outpath).expect(&format!("Failed to save image {}", outpath.display()));

    let duration = start.elapsed();
    println!("duration={:?}", duration);

}

