#![no_std]
#![no_main]

use cpu_test::{check, finish};

#[no_mangle]
pub extern "C" fn main() -> ! {

    unsafe {
        let ra: u32;
        let flag: u32;
        let after: u32;
        core::arch::asm!(
            "jal {ra}, 2f",
            "1:",
            "addi {flag}, zero, 1",
            "j 3f",
            "2:",
            "addi {flag}, zero, 0",
            "la {after}, 1b",
            "3:",
            ra = out(reg) ra,
            flag = out(reg) flag,
            after = out(reg) after
        );
        check(flag == 0, 1);
        check(ra == after, 2);
    }

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
        check(flag == 0, 3);
        check(ra == after, 4);
    }

    unsafe {
        let mut base: u32 = 0;
        let ra: u32;
        let flag: u32;
        core::arch::asm!(
            "la {base}, 2f",
            "addi {base}, {base}, 1",
            "jalr {ra}, 0({base})",
            "addi {flag}, zero, 1",
            "j 3f",
            "2: addi {flag}, zero, 0",
            "3:",
            base = inout(reg) base,
            ra = out(reg) ra,
            flag = out(reg) flag
        );
        check(flag == 0, 5);
    }

    finish();
}
