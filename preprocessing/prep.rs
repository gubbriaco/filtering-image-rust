use crate::utils::matrix2dynamicimg::matrix_to_dynamicimg_pub;
use crate::preprocessing::prepimpl::noiseless::noise_removing_pub;
use image::{DynamicImage, ImageBuffer, Rgb};
use crate::utils::imagebuffer2pixels::imagebuffer_to_pixels_pub;


pub fn preprocess_pub(img:DynamicImage, padding:u32, kernel:&Vec<Vec<u8>>, ksize:u32) -> DynamicImage {
    preprocess(img, padding, kernel, ksize)
}


fn preprocess(img:DynamicImage, padding:u32, kernel:&Vec<Vec<u8>>, ksize:u32) -> DynamicImage {

    let img_rgb: ImageBuffer<Rgb<u8>,Vec<u8>> = img.to_rgb8();
    let (width,height): (u32,u32) = img_rgb.dimensions();

    let pixels_matrix: Vec<Vec<[u8;3]>> = imagebuffer_to_pixels_pub(img_rgb, width, height);

    let out_pixels_matrix = noise_removing_pub(&pixels_matrix, width, height, padding, &kernel, ksize);

    matrix_to_dynamicimg_pub(out_pixels_matrix)
}
