#![no_std]
#![no_main]

use cpu_test::finish;
use am::halt;

const CLINT: usize = 0x0200_0000;

fn wr(off: usize, v: u32) {
    unsafe { core::ptr::write_volatile((CLINT + off) as *mut u32, v) }
}

core::arch::global_asm!(
    r#"
    .section .text.cte_timer, "ax", @progbits
    .balign 4
    .globl cte_timer_entry
cte_timer_entry:
    j cte_timer_handler
"#
);

extern "C" {
    fn cte_timer_entry();
}

#[no_mangle]
pub extern "C" fn cte_timer_handler() {
    let cause: u32;
    unsafe {
        core::arch::asm!("csrr {c}, 0x342", c = out(reg) cause);
    }
    if cause != 0x8000_0007 {
        halt(1);
    }
    finish();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        core::arch::asm!(
            "csrw 0x305, {h}",
            "csrw 0x304, {m}",
            "csrrs zero, 0x300, {s}",
            h = in(reg) cte_timer_entry as *const () as usize,
            m = in(reg) 0x80u32,
            s = in(reg) 0x8u32
        );
    }

    wr(0x4000, 0x1000);
    wr(0x4004, 0);

    loop {
        unsafe { core::arch::asm!("nop"); }
    }
}
