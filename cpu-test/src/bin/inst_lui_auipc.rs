//! LUI / AUIPC 指令测试。

#![no_std]
#![no_main]

use cpu_test::{check, finish};

#[no_mangle]
pub extern "C" fn main() -> ! {
    // ---- LUI：rd = imm << 12（符号扩展）----
    unsafe {
        let r: u32;
        core::arch::asm!("lui {r}, 0x12345", r = out(reg) r);
        check(r == 0x1234_5000, 1);
    }
    unsafe {
        let r: u32;
        core::arch::asm!("lui {r}, 0xFFFFF", r = out(reg) r);
        check(r == 0xFFFF_F000, 2);
    }
    unsafe {
        let r: u32;
        core::arch::asm!("lui {r}, 0x00000", r = out(reg) r);
        check(r == 0x0000_0000, 3);
    }
    unsafe {
        let r: u32;
        core::arch::asm!("lui {r}, 0x80000", r = out(reg) r);
        check(r == 0x8000_0000, 4);
    }

    // ---- AUIPC：rd = pc + (imm << 12) ----
    unsafe {
        // imm = 0：结果等于 auipc 自身的 pc
        let r: u32;
        let label: u32;
        core::arch::asm!(
            "1: auipc {r}, 0",
            "la {label}, 1b",
            r = out(reg) r,
            label = out(reg) label
        );
        check(r == label, 5);
    }
    unsafe {
        // imm = 1：结果等于 pc + 0x1000
        let r: u32;
        let label: u32;
        core::arch::asm!(
            "1: auipc {r}, 1",
            "la {label}, 1b",
            r = out(reg) r,
            label = out(reg) label
        );
        check(r == label.wrapping_add(0x1000), 6);
    }
    unsafe {
        // imm = -1（0xFFFFF）：结果等于 pc - 0x1000
        let r: u32;
        let label: u32;
        core::arch::asm!(
            "1: auipc {r}, 0xFFFFF",
            "la {label}, 1b",
            r = out(reg) r,
            label = out(reg) label
        );
        check(r == label.wrapping_sub(0x1000), 7);
    }

    finish();
}
