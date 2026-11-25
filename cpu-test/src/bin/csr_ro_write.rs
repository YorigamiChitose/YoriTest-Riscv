#![no_std]
#![no_main]

use am::halt;
use cpu_test::finish;

core::arch::global_asm!(
    r#"
    .section .text.ro, "ax", @progbits
    .balign 4
    .globl ro_trap
ro_trap:
    j ro_handler
"#
);

extern "C" {
    fn ro_trap();
}

#[no_mangle]
pub extern "C" fn ro_handler() {
    let cause: u32;
    unsafe {
        core::arch::asm!("csrr {c}, 0x342", c = out(reg) cause);
    }
    if cause != 2 {
        halt(1);
    }
    finish();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        core::arch::asm!(
            "csrw 0x305, {t}",
            "csrw 0xf11, zero",
            t = in(reg) ro_trap as *const () as usize
        );
    }
    loop {
        unsafe { core::arch::asm!("nop"); }
    }
}
