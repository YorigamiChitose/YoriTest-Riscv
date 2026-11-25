#![no_std]
#![no_main]

use am::halt;
use cpu_test::finish;

const CLINT: usize = 0x0200_0000;
const PLIC: usize = 0x0c00_0000;

fn wr(base: usize, off: usize, v: u32) {
    unsafe { core::ptr::write_volatile((base + off) as *mut u32, v) }
}

core::arch::global_asm!(
    r#"
    .section .text.sext, "ax", @progbits
    .balign 4
    .globl s_ext_entry
s_ext_entry:
    li t0, 0x200
    csrw 0x104, t0
    j 1f
1:
    j 1b
    .balign 4
    .globl s_ext_handler
s_ext_handler:
    j s_ext_handler_rust
    .balign 4
    .globl m_fail
m_fail:
    li a0, 1
    j halt
"#
);

extern "C" {
    fn s_ext_entry();
    fn s_ext_handler();
    fn m_fail();
}

#[no_mangle]
pub extern "C" fn s_ext_handler_rust() {
    let cause: u32;
    unsafe {
        core::arch::asm!("csrr {c}, 0x142", c = out(reg) cause);
    }
    if cause != 0x8000_0009 {
        halt(2);
    }
    wr(CLINT, 0, 0);
    finish();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    wr(PLIC, 0x0004, 1);
    wr(PLIC, 0x2000, 2);
    wr(PLIC, 0x200000, 0);
    wr(CLINT, 0, 1);
    unsafe {
        core::arch::asm!(
            "csrw 0x305, {m}",
            "csrw 0x105, {s}",
            "csrw 0x303, {d}",
            "csrw 0x341, {e}",
            "csrw 0x300, {x}",
            "mret",
            m = in(reg) m_fail as *const () as usize,
            s = in(reg) s_ext_handler as *const () as usize,
            d = in(reg) 0x200u32,
            e = in(reg) s_ext_entry as *const () as usize,
            x = in(reg) 0x802u32
        );
    }
    loop {
        unsafe { core::arch::asm!("nop"); }
    }
}
