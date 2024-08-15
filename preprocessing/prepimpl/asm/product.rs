use std::arch::asm;


pub fn product_asm_pub(pixel:u32, k_val:u32) -> u32 {
    product_asm(pixel, k_val)
}


fn product_asm(pixel:u32, k_val:u32) -> u32 {
    let product: u32;
    unsafe {
        asm!(
            "mov {0:e}, {1:e}",
            "imul {0:e}, {2:e}",
            out(reg) product,
            in(reg) pixel,
            in(reg) k_val,
        );
    }
    product
}
