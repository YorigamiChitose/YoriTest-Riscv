#![no_std]
#![no_main]

use cpu_test::{check, finish};

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut id = 0u32;
    let mut c = |cond: bool| {
        id += 1;
        check(cond, id);
    };

    c(1 + 2 == 3);
    c(100 - 37 == 63);
    c(7 * 8 == 56);
    c(100 / 7 == 14);
    c(100 % 7 == 2);

    c(0xFFFF_FFFFu32.wrapping_add(1) == 0);
    c(0u32.wrapping_sub(1) == 0xFFFF_FFFF);

    c((0xF0u32 & 0x3C) == 0x30);
    c((0xF0u32 | 0x0F) == 0xFF);
    c((0xFFu32 ^ 0x0F) == 0xF0);
    c((1u32 << 10) == 1024);
    c((0x8000_0000u32 >> 31) == 1);
    c((0x8000_0000u32 as i32) >> 31 == -1);

    c(3i32 < 5 && 5 < 8);
    c(-5i32 < 3);
    c(5u32 > 3u32);

    finish();
}
