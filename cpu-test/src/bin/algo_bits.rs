#![no_std]
#![no_main]

use cpu_test::{check, finish};

fn popcount(mut x: u32) -> u32 {
    let mut c = 0u32;
    while x != 0 {
        c += x & 1;
        x >>= 1;
    }
    c
}

fn bit_reverse(mut x: u32) -> u32 {
    let mut r = 0u32;
    let mut i = 0u32;
    while i < 32 {
        r = (r << 1) | (x & 1);
        x >>= 1;
        i += 1;
    }
    r
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    check(popcount(0) == 0, 1);
    check(popcount(0xFF) == 8, 2);
    check(popcount(0xFFFF_FFFF) == 32, 3);
    check(popcount(0xAAAA_AAAA) == 16, 4);
    check(bit_reverse(0x0000_0001) == 0x8000_0000, 5);
    check(bit_reverse(0x8000_0000) == 0x0000_0001, 6);
    check(bit_reverse(0xF0F0_F0F0) == 0x0F0F_0F0F, 7);
    check(bit_reverse(0x1234_5678) == 0x1E6A_2C48, 8);

    finish();
}
