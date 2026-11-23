//! SLL / SRL / SRA 及立即数移位指令测试（含移位量取低 5 位的边界）。

#![no_std]
#![no_main]

use cpu_test::{finish, t_ri, t_rr};

#[no_mangle]
pub extern "C" fn main() -> ! {
    // ---- SLL（左移，低 32 位有效）----
    t_rr!(1, "sll {r}, {a}, {b}", 0x0000_0001, 0x0000_0004, 0x0000_0010);
    t_rr!(2, "sll {r}, {a}, {b}", 0x0000_0001, 0x0000_001F, 0x8000_0000);
    t_rr!(3, "sll {r}, {a}, {b}", 0xFFFF_FFFF, 0x0000_0008, 0xFFFF_FF00);
    // 移位量 >= 32 取低 5 位：32 & 31 == 0，等价不移
    t_rr!(4, "sll {r}, {a}, {b}", 0x0000_00FF, 0x0000_0020, 0x0000_00FF);

    // ---- SLLI ----
    t_ri!(5, "slli {r}, {a}, 4", 0x0000_0001, 0x0000_0010);
    t_ri!(6, "slli {r}, {a}, 31", 0x0000_0001, 0x8000_0000);
    t_ri!(7, "slli {r}, {a}, 0", 0xDEAD_BEEF, 0xDEAD_BEEF);

    // ---- SRL（逻辑右移）----
    t_rr!(8, "srl {r}, {a}, {b}", 0x8000_0000, 0x0000_001F, 0x0000_0001);
    t_rr!(9, "srl {r}, {a}, {b}", 0xFFFF_FFFF, 0x0000_0010, 0x0000_FFFF);
    // 移位量 >= 32 取低 5 位：32 & 31 == 0
    t_rr!(10, "srl {r}, {a}, {b}", 0xFFFF_FFFF, 0x0000_0020, 0xFFFF_FFFF);

    // ---- SRLI ----
    t_ri!(11, "srli {r}, {a}, 16", 0xDEAD_BEEF, 0x0000_DEAD);
    t_ri!(12, "srli {r}, {a}, 0", 0xDEAD_BEEF, 0xDEAD_BEEF);

    // ---- SRA（算术右移，符号扩展）----
    t_rr!(13, "sra {r}, {a}, {b}", 0x8000_0000, 0x0000_001F, 0xFFFF_FFFF);
    t_rr!(14, "sra {r}, {a}, {b}", 0xFFFF_F000, 0x0000_0004, 0xFFFF_FF00);
    t_rr!(15, "sra {r}, {a}, {b}", 0x7FFF_FFFF, 0x0000_001F, 0x0000_0000); // 正数补 0
    t_rr!(16, "sra {r}, {a}, {b}", 0xFFFF_FFFF, 0x0000_0020, 0xFFFF_FFFF); // 32 & 31 == 0

    // ---- SRAI ----
    t_ri!(17, "srai {r}, {a}, 31", 0x8000_0000, 0xFFFF_FFFF);
    t_ri!(18, "srai {r}, {a}, 8", 0xFFFF_8000, 0xFFFF_FF80);
    t_ri!(19, "srai {r}, {a}, 0", 0xDEAD_BEEF, 0xDEAD_BEEF);

    finish();
}
