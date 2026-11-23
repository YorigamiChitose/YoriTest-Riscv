//! 斐波那契数列（迭代 + 递归），覆盖循环与函数调用（jal/jalr、栈 lw/sw）。

#![no_std]
#![no_main]

use cpu_test::{check, finish};

fn fib_iter(n: u32) -> u32 {
    let (mut a, mut b) = (0u32, 1u32);
    for _ in 0..n {
        let t = a + b;
        a = b;
        b = t;
    }
    a
}

fn fib_rec(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib_rec(n - 1) + fib_rec(n - 2)
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // fib: 0,1,1,2,3,5,8,13,21,34,55,89,144,233,377,610,987,1597,2584,4181,6765
    check(fib_iter(0) == 0, 1);
    check(fib_iter(1) == 1, 2);
    check(fib_iter(10) == 55, 3);
    check(fib_iter(20) == 6765, 4);
    check(fib_rec(0) == 0, 5);
    check(fib_rec(10) == 55, 6);
    check(fib_rec(20) == 6765, 7);
    check(fib_iter(20) == fib_rec(20), 8);

    finish();
}
