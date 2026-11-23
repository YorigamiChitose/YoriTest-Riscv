//! SLT / SLTU / SLTI / SLTIU 比较置位指令测试。

#![no_std]
#![no_main]

use cpu_test::{finish, t_ri, t_rr};

#[no_mangle]
pub extern "C" fn main() -> ! {
    // ---- SLT（有符号小于）----
    t_rr!(1, "slt {r}, {a}, {b}", 0x0000_0001, 0x0000_0002, 0x0000_0001);
    t_rr!(2, "slt {r}, {a}, {b}", 0x0000_0002, 0x0000_0001, 0x0000_0000);
    t_rr!(3, "slt {r}, {a}, {b}", 0xFFFF_FFFF, 0x0000_0001, 0x0000_0001); // -1 < 1
    t_rr!(4, "slt {r}, {a}, {b}", 0x0000_0001, 0xFFFF_FFFF, 0x0000_0000); // 1 < -1 假
    t_rr!(5, "slt {r}, {a}, {b}", 0x0000_000A, 0x0000_000A, 0x0000_0000); // 相等

    // ---- SLTU（无符号小于）----
    t_rr!(6, "sltu {r}, {a}, {b}", 0xFFFF_FFFF, 0x0000_0001, 0x0000_0000); // 无符号下 0xFFFFFFFF > 1
    t_rr!(7, "sltu {r}, {a}, {b}", 0x0000_0001, 0xFFFF_FFFF, 0x0000_0001);
    t_rr!(8, "sltu {r}, {a}, {b}", 0x8000_0000, 0x7FFF_FFFF, 0x0000_0000); // 无符号下 0x80000000 大

    // ---- SLTI（有符号小于立即数）----
    t_ri!(9, "slti {r}, {a}, 5", 0x0000_0003, 0x0000_0001);
    t_ri!(10, "slti {r}, {a}, -1", 0xFFFF_FFFF, 0x0000_0000); // -1 < -1 假
    t_ri!(11, "slti {r}, {a}, -1", 0xFFFF_FFFE, 0x0000_0001); // -2 < -1 真

    // ---- SLTIU（无符号小于立即数，立即数符号扩展后按无符号比）----
    t_ri!(12, "sltiu {r}, {a}, 5", 0x0000_0003, 0x0000_0001);
    t_ri!(13, "sltiu {r}, {a}, -1", 0x0000_0000, 0x0000_0001); // 0 < 0xFFFFFFFF 真
    t_ri!(14, "sltiu {r}, {a}, -1", 0xFFFF_FFFF, 0x0000_0000); // 0xFFFFFFFF < 0xFFFFFFFF 假

    finish();
}
