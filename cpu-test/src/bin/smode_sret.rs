#![no_std]
#![no_main]

use am::halt;
use cpu_test::finish;

core::arch::global_asm!(
    r#"
    .section .text.ssret, "ax", @progbits
    .balign 4
    .globl s_sret_entry
s_sret_entry:
    csrw 0x100, zero
    csrw 0x141, t0
    sret
    .balign 4
    .globl u_entry
u_entry:
    ecall
1:
    j 1b
    .balign 4
    .globl m_sret_trap
m_sret_trap:
    j mtrap_entry_sret
"#
);

extern "C" {
    fn s_sret_entry();
    fn u_entry();
    fn m_sret_trap();
}

#[no_mangle]
pub extern "C" fn mtrap_handler_sret() {
    let cause: u32;
    unsafe {
        core::arch::asm!("csrr {c}, 0x342", c = out(reg) cause);
    }
    if cause != 8 {
        halt(2);
    }
    finish();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        core::arch::asm!(
            "la t0, u_entry",
            "csrw 0x305, {m}",
            "csrw 0x341, {e}",
            "csrw 0x300, {x}",
            "mret",
            m = in(reg) m_sret_trap as *const () as usize,
            e = in(reg) s_sret_entry as *const () as usize,
            x = in(reg) 0x880u32
        );
    }
    loop {
        unsafe { core::arch::asm!("nop"); }
    }
}

#[no_mangle]
pub extern "C" fn mtrap_entry_sret() {
    mtrap_handler_sret();
}
