#![no_std]
#![no_main]

use am::halt;
use cpu_test::finish;

core::arch::global_asm!(
    r#"
    .section .text.sill, "ax", @progbits
    .balign 4
    .globl s_illegal_entry
s_illegal_entry:
    csrr t0, 0x300
    j 1f
1:
    j 1b
    .balign 4
    .globl m_illegal_trap
m_illegal_trap:
    j m_illegal_handler
"#
);

extern "C" {
    fn s_illegal_entry();
    fn m_illegal_trap();
}

#[no_mangle]
pub extern "C" fn m_illegal_handler() {
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
            "csrw 0x305, {m}",
            "csrw 0x341, {e}",
            "csrw 0x300, {x}",
            "mret",
            m = in(reg) m_illegal_trap as *const () as usize,
            e = in(reg) s_illegal_entry as *const () as usize,
            x = in(reg) 0x880u32
        );
    }
    loop {
        unsafe { core::arch::asm!("nop"); }
    }
}
