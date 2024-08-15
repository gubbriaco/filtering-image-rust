use crate::preprocessing::prepimpl::asm::product::product_pub;
use crate::preprocessing::prepimpl::asm::add::add_pub;


pub fn mac_pub(pixels_matrix: &mut Vec<Vec<[u8; 3]>>, kernel: &Vec<Vec<u8>>, ksize: u32, i: u32, j: u32) {
    mac(pixels_matrix, kernel, ksize, i, j);
}


fn mac(pixels_matrix: &mut Vec<Vec<[u8; 3]>>, kernel: &Vec<Vec<u8>>, ksize: u32, i: u32, j: u32) {

    let mut ret0: u32 = 0;
    let mut ret1: u32 = 0;
    let mut ret2: u32 = 0;

    for ki in 0..3 {
        for kj in 0..3 {
            let pi = (i as usize + ki as usize - ksize as usize) as usize;
            let pj = (j as usize + kj as usize - ksize as usize) as usize;

            let pixel = pixels_matrix[pi][pj];
            let k_val = kernel[ki][kj];

            /*
            ret0 += pixel[0] as u32 * k_val as u32;
            ret1 += pixel[1] as u32 * k_val as u32;
            ret2 += pixel[2] as u32 * k_val as u32;
            */

            let product0: u32;
            product0 = product_pub(pixel[0], k_val);
            ret0 = add_pub(ret0, product0);

            let product1: u32;
            product1 = product_pub(pixel[1], k_val);
            ret1 = add_pub(ret1, product1);

            let product2: u32;
            product2 = product_pub(pixel[2], k_val);
            ret2 = add_pub(ret2, product2);

        }
    }

    let kernel_sum: u32 = kernel.iter().flatten().map(|&x| x as u32).sum();

    pixels_matrix[i as usize][j as usize][0 as usize] = (ret0 / kernel_sum) as u8;
    pixels_matrix[i as usize][j as usize][1 as usize] = (ret1 / kernel_sum) as u8;
    pixels_matrix[i as usize][j as usize][2 as usize] = (ret2 / kernel_sum) as u8;

}
