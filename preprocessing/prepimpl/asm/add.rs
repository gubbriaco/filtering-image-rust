use std::arch::asm;


pub fn add_pub(ret:u32, product:u32) -> u32 {
    add(ret, product)
}


fn add(ret:u32, product:u32) -> u32 {
    let add: u32;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, {2}",
            out(reg) add,
            in(reg) ret,
            in(reg) product,
        );
    }
    add
}
