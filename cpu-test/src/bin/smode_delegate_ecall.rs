#![no_std]
#![no_main]

use am::halt;
use cpu_test::finish;

core::arch::global_asm!(
    r#"
    .section .text.sdeleg, "ax", @progbits
    .balign 4
    .globl s_entry
s_entry:
    ecall
1:
    j 1b
    .balign 4
    .globl s_handler
s_handler:
    j s_handler_rust
    .balign 4
    .globl m_fail
m_fail:
    li a0, 1
    j halt
"#
);

extern "C" {
    fn s_entry();
    fn s_handler();
    fn m_fail();
}

#[no_mangle]
pub extern "C" fn s_handler_rust() {
    let cause: u32;
    unsafe {
        core::arch::asm!("csrr {c}, 0x142", c = out(reg) cause);
    }
    if cause != 9 {
        halt(2);
    }
    finish();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        core::arch::asm!(
            "csrw 0x305, {m}",
            "csrw 0x105, {s}",
            "csrw 0x302, {d}",
            "csrw 0x341, {e}",
            "csrw 0x300, {x}",
            "mret",
            m = in(reg) m_fail as *const () as usize,
            s = in(reg) s_handler as *const () as usize,
            d = in(reg) 0x200u32,
            e = in(reg) s_entry as *const () as usize,
            x = in(reg) 0x880u32
        );
    }
    loop {
        unsafe { core::arch::asm!("nop"); }
    }
}
