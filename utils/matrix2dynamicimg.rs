use image::{DynamicImage, ImageBuffer, Rgb};


pub fn matrix_to_dynamicimg_pub(pixels_matrix: Vec<Vec<[u8; 3]>>) -> DynamicImage {
    matrix_to_dynamicimg(pixels_matrix)
}

fn matrix_to_dynamicimg(pixels_matrix: Vec<Vec<[u8; 3]>>) -> DynamicImage {
    let new_rows = pixels_matrix.len() as u32;
    let new_cols = pixels_matrix[0].len() as u32;
    let mut buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(new_cols, new_rows);

    for (y, row) in pixels_matrix.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            buffer.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
        }
    }

    DynamicImage::ImageRgb8(buffer)
}
