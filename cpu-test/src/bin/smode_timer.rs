#![no_std]
#![no_main]

use am::halt;
use cpu_test::finish;

const CLINT: usize = 0x0200_0000;

fn wr(off: usize, v: u32) {
    unsafe { core::ptr::write_volatile((CLINT + off) as *mut u32, v) }
}

core::arch::global_asm!(
    r#"
    .section .text.smtimer, "ax", @progbits
    .balign 4
    .globl s_timer_entry
s_timer_entry:
    li t0, 0x20
    csrw 0x104, t0
    j 1f
1:
    j 1b
    .balign 4
    .globl s_timer_handler
s_timer_handler:
    j s_timer_handler_rust
    .balign 4
    .globl m_fail
m_fail:
    li a0, 1
    j halt
"#
);

extern "C" {
    fn s_timer_entry();
    fn s_timer_handler();
    fn m_fail();
}

#[no_mangle]
pub extern "C" fn s_timer_handler_rust() {
    let cause: u32;
    unsafe {
        core::arch::asm!("csrr {c}, 0x142", c = out(reg) cause);
    }
    if cause != 0x8000_0005 {
        halt(2);
    }
    finish();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    wr(0x4000, 0x1000);
    wr(0x4004, 0);
    unsafe {
        core::arch::asm!(
            "csrw 0x305, {m}",
            "csrw 0x105, {s}",
            "csrw 0x303, {d}",
            "csrw 0x341, {e}",
            "csrw 0x300, {x}",
            "mret",
            m = in(reg) m_fail as *const () as usize,
            s = in(reg) s_timer_handler as *const () as usize,
            d = in(reg) 0x20u32,
            e = in(reg) s_timer_entry as *const () as usize,
            x = in(reg) 0x802u32
        );
    }
    loop {
        unsafe { core::arch::asm!("nop"); }
    }
}
