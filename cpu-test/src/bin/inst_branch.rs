//! BEQ / BNE / BLT / BGE / BLTU / BGEU 分支指令测试。

#![no_std]
#![no_main]

use cpu_test::{check, finish};

/// 分支测试：`$insn` 为分支指令（跳转目标用 `1f`）。
/// r 初值 0；若未跳转则置 1；故 r==0 表示「跳转」。
macro_rules! t_branch {
    ($id:expr, $insn:literal, $a:expr, $b:expr, $taken:expr) => {{
        let a: u32 = $a;
        let b: u32 = $b;
        let r: u32;
        unsafe {
            core::arch::asm!(
                "addi {r}, zero, 0",
                $insn,
                "addi {r}, zero, 1",
                "1:",
                a = in(reg) a,
                b = in(reg) b,
                r = out(reg) r
            );
        }
        check((r == 0) == $taken, $id);
    }};
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // ---- BEQ（相等跳转）----
    t_branch!(1, "beq {a}, {b}, 1f", 5, 5, true);
    t_branch!(2, "beq {a}, {b}, 1f", 5, 6, false);
    t_branch!(3, "beq {a}, {b}, 1f", 0, 0, true);

    // ---- BNE（不等跳转）----
    t_branch!(4, "bne {a}, {b}, 1f", 5, 6, true);
    t_branch!(5, "bne {a}, {b}, 1f", 5, 5, false);

    // ---- BLT（有符号小于跳转）----
    t_branch!(6, "blt {a}, {b}, 1f", 0xFFFF_FFFF, 1, true); // -1 < 1
    t_branch!(7, "blt {a}, {b}, 1f", 1, 0xFFFF_FFFF, false); // 1 < -1 假
    t_branch!(8, "blt {a}, {b}, 1f", 3, 3, false);

    // ---- BGE（有符号大于等于跳转）----
    t_branch!(9, "bge {a}, {b}, 1f", 2, 1, true);
    t_branch!(10, "bge {a}, {b}, 1f", 2, 2, true);
    t_branch!(11, "bge {a}, {b}, 1f", 1, 2, false);
    t_branch!(12, "bge {a}, {b}, 1f", 0xFFFF_FFFF, 1, false); // -1 >= 1 假

    // ---- BLTU（无符号小于跳转）----
    t_branch!(13, "bltu {a}, {b}, 1f", 1, 2, true);
    t_branch!(14, "bltu {a}, {b}, 1f", 0xFFFF_FFFF, 1, false); // 无符号下 0xFFFFFFFF > 1
    t_branch!(15, "bltu {a}, {b}, 1f", 3, 3, false);

    // ---- BGEU（无符号大于等于跳转）----
    t_branch!(16, "bgeu {a}, {b}, 1f", 0xFFFF_FFFF, 1, true); // 无符号下大
    t_branch!(17, "bgeu {a}, {b}, 1f", 1, 0xFFFF_FFFF, false);
    t_branch!(18, "bgeu {a}, {b}, 1f", 7, 7, true);

    finish();
}
