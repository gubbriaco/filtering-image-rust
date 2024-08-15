use std::arch::asm;


pub fn add_asm_pub(ret:u32, product:u32) -> u32 {
    add_asm(ret, product)
}


fn add_asm(ret:u32, product:u32) -> u32 {
    let add: u32;
    unsafe {
        asm!(
            "mov {0:e}, {1:e}",
            "add {0:e}, {2:e}",
            out(reg) add,
            in(reg) ret,
            in(reg) product,
        );
    }
    add
}
