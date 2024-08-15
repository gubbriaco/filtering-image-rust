use std::arch::asm;

pub fn mac_inline_pub(pixel_value:u32, kernel_value:u32, current_result:u32) -> u32 {
    mac_inline(pixel_value, kernel_value, current_result)
}

fn mac_inline(pixel_value:u32, kernel_value:u32, current_result:u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "mov {0:e}, {1:e}",
            "imul {0:e}, {2:e}",
            "add {0:e}, {3:e}",
            out(reg) result,
            in(reg) pixel_value,
            in(reg) kernel_value,
            in(reg) current_result,
        );
    }

    let true_value:u32 = current_result + (pixel_value*kernel_value);
    assert_eq!(result, true_value);

    result
}
