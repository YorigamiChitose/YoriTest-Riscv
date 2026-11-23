//! DIV / DIVU / REM / REMU 除法指令测试（含除零、溢出边界）。

#![no_std]
#![no_main]

use cpu_test::{finish, t_rr};

#[no_mangle]
pub extern "C" fn main() -> ! {
    // ---- DIV（有符号，向零舍入）----
    t_rr!(1, "div {r}, {a}, {b}", 10, 3, 3);
    t_rr!(2, "div {r}, {a}, {b}", 0xFFFF_FFF6, 3, 0xFFFF_FFFD); // -10 / 3 = -3
    t_rr!(3, "div {r}, {a}, {b}", 0xFFFF_FFF6, 0xFFFF_FFFD, 3); // -10 / -3 = 3
    t_rr!(4, "div {r}, {a}, {b}", 5, 0, 0xFFFF_FFFF); // 除零 → -1
    t_rr!(5, "div {r}, {a}, {b}", 0x8000_0000, 0xFFFF_FFFF, 0x8000_0000); // 溢出：INT_MIN / -1

    // ---- REM（有符号余数，符号同被除数）----
    t_rr!(6, "rem {r}, {a}, {b}", 10, 3, 1);
    t_rr!(7, "rem {r}, {a}, {b}", 0xFFFF_FFF6, 3, 0xFFFF_FFFF); // -10 % 3 = -1
    t_rr!(8, "rem {r}, {a}, {b}", 0xFFFF_FFF6, 0xFFFF_FFFD, 0xFFFF_FFFF); // -10 % -3 = -1
    t_rr!(9, "rem {r}, {a}, {b}", 5, 0, 5); // 除零 → 被除数
    t_rr!(10, "rem {r}, {a}, {b}", 0x8000_0000, 0xFFFF_FFFF, 0); // 溢出余数 0

    // ---- DIVU（无符号）----
    t_rr!(11, "divu {r}, {a}, {b}", 10, 3, 3);
    t_rr!(12, "divu {r}, {a}, {b}", 0xFFFF_FFFF, 2, 0x7FFF_FFFF);
    t_rr!(13, "divu {r}, {a}, {b}", 5, 0, 0xFFFF_FFFF); // 除零 → 全 1

    // ---- REMU（无符号余数）----
    t_rr!(14, "remu {r}, {a}, {b}", 10, 3, 1);
    t_rr!(15, "remu {r}, {a}, {b}", 0xFFFF_FFFF, 2, 1);
    t_rr!(16, "remu {r}, {a}, {b}", 5, 0, 5); // 除零 → 被除数

    finish();
}
