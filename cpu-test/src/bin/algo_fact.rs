#![no_std]
#![no_main]

use cpu_test::{check, finish};

fn fact_rec(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * fact_rec(n - 1)
    }
}

fn fact_iter(n: u32) -> u32 {
    let mut r = 1u32;
    let mut i = 2u32;
    while i <= n {
        r *= i;
        i += 1;
    }
    r
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    check(fact_rec(0) == 1, 1);
    check(fact_rec(1) == 1, 2);
    check(fact_rec(5) == 120, 3);
    check(fact_rec(10) == 3_628_800, 4);
    check(fact_iter(0) == 1, 5);
    check(fact_iter(5) == 120, 6);
    check(fact_iter(12) == 479_001_600, 7);
    check(fact_rec(10) == fact_iter(10), 8);

    finish();
}
