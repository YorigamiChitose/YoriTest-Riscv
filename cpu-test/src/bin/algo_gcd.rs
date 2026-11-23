//! 最大公约数 / 最小公倍数（欧几里得辗转相除），覆盖取模与循环。

#![no_std]
#![no_main]

use cpu_test::{check, finish};

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn lcm(a: u32, b: u32) -> u32 {
    a / gcd(a, b) * b
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    check(gcd(48, 18) == 6, 1);
    check(gcd(17, 13) == 1, 2);
    check(gcd(100, 10) == 10, 3);
    check(gcd(0, 5) == 5, 4);
    check(gcd(2_147_483_647, 1) == 1, 5);
    check(gcd(123_456, 78_912) == 192, 6);
    check(lcm(4, 6) == 12, 7);
    check(lcm(21, 6) == 42, 8);

    finish();
}
