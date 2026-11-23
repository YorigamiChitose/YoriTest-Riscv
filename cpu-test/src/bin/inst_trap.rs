//! ECALL + MRET 闭环测试。
//!
//! 流程：设 MTVEC -> ecall 陷入 handler -> handler 读 mepc 并 +4 -> mret ->
//!       返回 ecall 下一条指令 -> 验证标志被置位。
//!
//! 该测试同时隐含验证 MEPC 语义：若 MEPC 错存为 ecall+4（而非 ecall 自身地址），
//! 则 handler 的 mepc+4 会跳到 ecall+8，跳过 `li` 标志指令，check 失败。

#![no_std]
#![no_main]

use cpu_test::{check, finish};

/// 异常处理入口：mepc += 4（跳过 ecall），mret 返回。
#[no_mangle]
pub extern "C" fn trap_handler() {
    let m: u32;
    unsafe {
        core::arch::asm!(
            "csrrs {m}, 0x341, zero",  // m = mepc
            "addi {m}, {m}, 4",        // m += 4
            "csrrw zero, 0x341, {m}",  // mepc = m
            "mret",
            m = lateout(reg) m
        );
    }
    let _ = m;
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // 设 MTVEC = trap_handler
    let t: u32;
    unsafe {
        core::arch::asm!(
            "la {t}, {h}",
            "csrrw zero, 0x305, {t}",
            t = lateout(reg) t,
            h = sym trap_handler
        );
    }
    let _ = t;

    // ecall 陷入 handler；mret 返回后执行 li 设标志（此处即 ecall+4）
    let returned: u32;
    unsafe {
        core::arch::asm!(
            "ecall",
            "li {r}, 1",
            r = out(reg) returned
        );
    }
    check(returned == 1, 1);

    finish();
}
