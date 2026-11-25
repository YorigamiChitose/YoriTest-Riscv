#![no_std]
#![no_main]

use am::halt;
use cpu_test::finish;

core::arch::global_asm!(
    r#"
    .section .text.smode, "ax", @progbits
    .balign 4
    .globl smode_entry
smode_entry:
    csrw 0x140, zero
    csrw 0x180, zero
    ecall
1:
    j 1b
    .balign 4
    .globl mtrap_entry
mtrap_entry:
    j mtrap_handler
"#
);

extern "C" {
    fn smode_entry();
    fn mtrap_entry();
}

#[no_mangle]
pub extern "C" fn mtrap_handler() {
    let cause: u32;
    unsafe {
        core::arch::asm!("csrr {c}, 0x342", c = out(reg) cause);
    }
    if cause != 9 {
        halt(1);
    }
    finish();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        core::arch::asm!(
            "csrw 0x305, {t}",
            "csrw 0x341, {e}",
            "csrw 0x300, {s}",
            "mret",
            t = in(reg) mtrap_entry as *const () as usize,
            e = in(reg) smode_entry as *const () as usize,
            s = in(reg) 0x880u32
        );
    }
    loop {
        unsafe { core::arch::asm!("nop"); }
    }
}
