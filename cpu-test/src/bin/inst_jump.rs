//! JAL / JALR 跳转链接指令测试。

#![no_std]
#![no_main]

use cpu_test::{check, finish};

#[no_mangle]
pub extern "C" fn main() -> ! {
    // ---- JAL：rd = pc + 4，跳转到目标 ----
    unsafe {
        let ra: u32;
        let flag: u32;
        let after: u32;
        core::arch::asm!(
            "jal {ra}, 2f",
            "1:",
            "addi {flag}, zero, 1",   // 未跳转标记（不应执行）
            "j 3f",
            "2:",
            "addi {flag}, zero, 0",   // 跳转目标
            "la {after}, 1b",         // after = 紧跟 jal 之后指令的地址
            "3:",
            ra = out(reg) ra,
            flag = out(reg) flag,
            after = out(reg) after
        );
        check(flag == 0, 1);   // 确实跳转
        check(ra == after, 2); // ra == pc + 4
    }

    // ---- JALR：rd = pc + 4，跳转到 rs1 + imm ----
    unsafe {
        let mut base: u32 = 0;
        let ra: u32;
        let flag: u32;
        let after: u32;
        core::arch::asm!(
            "la {base}, 2f",
            "jalr {ra}, 0({base})",
            "1:",
            "addi {flag}, zero, 1",
            "j 3f",
            "2:",
            "addi {flag}, zero, 0",
            "la {after}, 1b",
            "3:",
            base = inout(reg) base,
            ra = out(reg) ra,
            flag = out(reg) flag,
            after = out(reg) after
        );
        check(flag == 0, 3);   // 确实跳转
        check(ra == after, 4); // ra == pc + 4
    }

    // ---- JALR 的 LSB 清零：rs1 的 bit0 被忽略 ----
    unsafe {
        let mut base: u32 = 0;
        let ra: u32;
        let flag: u32;
        core::arch::asm!(
            "la {base}, 2f",
            "addi {base}, {base}, 1", // base = 目标地址 + 1（LSB=1）
            "jalr {ra}, 0({base})",   // 跳转时 LSB 清零
            "addi {flag}, zero, 1",
            "j 3f",
            "2: addi {flag}, zero, 0",
            "3:",
            base = inout(reg) base,
            ra = out(reg) ra,
            flag = out(reg) flag
        );
        check(flag == 0, 5); // LSB 被清零，仍成功跳转
    }

    finish();
}
