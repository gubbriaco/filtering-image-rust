use crate::preprocessing::prepimpl::asm::product::product_asm_pub;
use crate::preprocessing::prepimpl::asm::add::add_asm_pub;


pub fn mac_asm_pub(pixel:u8, k_val:u8, ret:u32) -> u32 {
    mac_asm(pixel as u32, k_val as u32, ret)
}


fn mac_asm(pixel:u32, k_val:u32, ret:u32) -> u32 {
    let product: u32;
    let result:u32;
    product = product_asm_pub(pixel, k_val);
    result = add_asm_pub(ret, product);
    result
}
