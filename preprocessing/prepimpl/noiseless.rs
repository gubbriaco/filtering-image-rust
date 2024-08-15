use crate::preprocessing::prepimpl::filtering::mac_pub;
use crate::utils::padding::get_padded_matrix_pub;


pub fn noise_removing_pub(pixels_matrix:&Vec<Vec<[u8;3]>>, width:u32, height:u32, padding:u32, kernel:&Vec<Vec<u8>>, ksize:u32) -> Vec<Vec<[u8;3]>> {
    noise_removing(pixels_matrix, width, height, padding, kernel, ksize)
}


fn noise_removing(pixels_matrix: &Vec<Vec<[u8;3]>>, width:u32, height:u32, padding:u32, kernel:&Vec<Vec<u8>>, ksize:u32) -> Vec<Vec<[u8;3]>> {

    let new_cols: u32 = width + padding; // Padding columns
    let new_rows: u32 = height + padding; // Padding rows
    let mut noiseless_matrix: Vec<Vec<[u8;3]>> = get_padded_matrix_pub(&pixels_matrix, new_cols, new_rows, padding, width, height);
    
    for i in padding..(new_rows-padding) {
        for j in padding..(new_cols-padding) {
            mac_pub(&mut noiseless_matrix, &kernel, ksize, i, j);
        }
    }
    
    noiseless_matrix
}
