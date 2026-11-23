//! 素数判定与计数（试除法），覆盖取模、比较、嵌套循环。

#![no_std]
#![no_main]

use cpu_test::{check, finish};

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2u32;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn count_primes(limit: u32) -> u32 {
    let mut cnt = 0u32;
    let mut i = 2u32;
    while i <= limit {
        if is_prime(i) {
            cnt += 1;
        }
        i += 1;
    }
    cnt
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    check(is_prime(2), 1);
    check(is_prime(3), 2);
    check(is_prime(97), 3);
    check(!is_prime(1), 4);
    check(!is_prime(100), 5);
    check(!is_prime(51), 6);
    // 100 以内 25 个素数，1000 以内 168 个
    check(count_primes(100) == 25, 7);
    check(count_primes(1000) == 168, 8);

    finish();
}
