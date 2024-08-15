use crate::utils::channels::ChRGB;
use image::{ImageBuffer, Rgb};


pub fn imagebuffer_to_pixels_pub(img_rgb:ImageBuffer<Rgb<u8>,Vec<u8>>, width:u32, height:u32) -> Vec<Vec<[u8;3]>> {
    imagebuffer_to_pixels(img_rgb, width, height)
}


fn imagebuffer_to_pixels(img_rgb:ImageBuffer<Rgb<u8>,Vec<u8>>, width:u32, height:u32) -> Vec<Vec<[u8;3]>> {

    let mut pixels_matrix = vec![vec![[0;3]; width as usize]; height as usize];

    for i in 0..height {
        for j in 0..width {
            let pixel = img_rgb.get_pixel(j, i);
            pixels_matrix[i as usize][j as usize] = [
                pixel[ChRGB::RED.as_u8() as usize],
                pixel[ChRGB::GREEN.as_u8() as usize],
                pixel[ChRGB::BLUE.as_u8() as usize],
            ];
        }
    }

    pixels_matrix
}
