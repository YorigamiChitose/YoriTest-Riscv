#![no_std]
#![no_main]

use am::halt;
use cpu_test::finish;

const CLINT: usize = 0x0200_0000;
const FLAG: usize = 0x8000_1000;

fn wr(off: usize, v: u32) {
    unsafe { core::ptr::write_volatile((CLINT + off) as *mut u32, v) }
}

core::arch::global_asm!(
    r#"
    .section .text.cte_timer_mret, "ax", @progbits
    .balign 4
    .globl cte_timer_mret_entry
cte_timer_mret_entry:
    j cte_timer_mret_handler
"#
);

extern "C" {
    fn cte_timer_mret_entry();
}

#[no_mangle]
pub extern "C" fn cte_timer_mret_handler() {
    let cause: u32;
    unsafe {
        core::arch::asm!("csrr {c}, 0x342", c = out(reg) cause);
        core::ptr::write_volatile(FLAG as *mut u32, 1);
        core::arch::asm!("csrw 0x300, zero");
    }
    if cause != 0x8000_0007 {
        halt(1);
    }
    wr(0x4000, 0xffff_ffff);
    wr(0x4004, 0xffff_ffff);
    unsafe {
        core::arch::asm!("mret");
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        core::arch::asm!(
            "csrw 0x305, {h}",
            "csrw 0x304, {m}",
            "csrrs zero, 0x300, {s}",
            h = in(reg) cte_timer_mret_entry as *const () as usize,
            m = in(reg) 0x80u32,
            s = in(reg) 0x8u32
        );
    }

    wr(0x4000, 0x1000);
    wr(0x4004, 0);

    loop {
        let flag = unsafe { core::ptr::read_volatile(FLAG as *const u32) };
        if flag == 1 {
            finish();
        }
        unsafe { core::arch::asm!("nop"); }
    }
}
