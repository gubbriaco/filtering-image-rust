pub fn get_padded_matrix_pub(pixels_matrix:&Vec<Vec<[u8;3]>>, new_cols:u32, new_rows:u32, padding:u32, width:u32, height:u32) -> Vec<Vec<[u8; 3]>> {
    get_padded_matrix(pixels_matrix, new_cols, new_rows, padding, width, height)
}



fn get_padded_matrix(pixels_matrix:&Vec<Vec<[u8;3]>>, new_cols:u32, new_rows:u32, padding:u32, width:u32, height:u32) -> Vec<Vec<[u8;3]>> {

    let mut padded_matrix: Vec<Vec<[u8;3]>> = vec![vec![[0, 0, 0]; new_cols as usize]; new_rows as usize];

    // Copy the original image into the new matrix with padding
    for i in 0..height {
        for j in 0..width {
            padded_matrix[(i + padding) as usize][(j + padding) as usize] = pixels_matrix[i as usize][j as usize];
        }
    }

    // Copy the edges
    for j in 0..new_cols {
        padded_matrix[0 as usize][j as usize] = padded_matrix[(padding) as usize][j as usize]; // Top padding
        padded_matrix[(new_rows - 1) as usize][j as usize] = padded_matrix[(new_rows - padding - 1) as usize][j as usize]; // Bottom padding
    }

    for i in 0..new_rows {
        padded_matrix[i as usize][0 as usize] = padded_matrix[i as usize][(padding) as usize]; // Left padding
        padded_matrix[i as usize][(new_cols - 1) as usize] = padded_matrix[i as usize][(new_cols - padding - 1) as usize]; // Right padding
    }

    padded_matrix
}
