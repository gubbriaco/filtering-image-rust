use std::arch::asm;


pub fn product_pub(pixel:u8, k_val:u8) -> u32 {
    product(pixel, k_val)
}


fn product(pixel:u8, k_val:u8) -> u32 {
    let product: u32;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "imul {0}, {2}",
            out(reg) product,
            in(reg) pixel as u32,
            in(reg) k_val as u32,
        );
    }
    product
}
