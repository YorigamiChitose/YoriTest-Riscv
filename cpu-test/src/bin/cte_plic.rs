#![no_std]
#![no_main]

use am::halt;
use cpu_test::finish;

const CLINT: usize = 0x0200_0000;
const PLIC: usize = 0x0c00_0000;

fn wr(base: usize, off: usize, v: u32) {
    unsafe { core::ptr::write_volatile((base + off) as *mut u32, v) }
}

fn rd(base: usize, off: usize) -> u32 {
    unsafe { core::ptr::read_volatile((base + off) as *const u32) }
}

core::arch::global_asm!(
    r#"
    .section .text.cte_plic, "ax", @progbits
    .balign 4
    .globl cte_plic_entry
cte_plic_entry:
    j cte_plic_handler
"#
);

extern "C" {
    fn cte_plic_entry();
}

#[no_mangle]
pub extern "C" fn cte_plic_handler() {
    let cause: u32;
    unsafe {
        core::arch::asm!("csrr {c}, 0x342", c = out(reg) cause);
    }
    if cause != 0x8000_000b {
        halt(1);
    }
    let id = rd(PLIC, 0x200004);
    if id != 1 {
        halt(2);
    }
    wr(CLINT, 0, 0);
    wr(PLIC, 0x200004, id);
    finish();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        core::arch::asm!(
            "csrw 0x305, {h}",
            "csrw 0x304, {m}",
            "csrrs zero, 0x300, {s}",
            h = in(reg) cte_plic_entry as *const () as usize,
            m = in(reg) 0x800u32,
            s = in(reg) 0x8u32
        );
    }

    wr(PLIC, 0x0004, 1);
    wr(PLIC, 0x2000, 2);
    wr(PLIC, 0x200000, 0);
    wr(CLINT, 0, 1);

    loop {
        unsafe { core::arch::asm!("nop"); }
    }
}
