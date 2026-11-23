#![no_std]
#![no_main]

use cpu_test::{check, finish};

#[no_mangle]
pub extern "C" fn trap_handler() {
    let m: u32;
    unsafe {
        core::arch::asm!(
            "csrrs {m}, 0x341, zero",
            "addi {m}, {m}, 4",
            "csrrw zero, 0x341, {m}",
            "mret",
            m = lateout(reg) m
        );
    }
    let _ = m;
}

#[no_mangle]
pub extern "C" fn main() -> ! {

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
