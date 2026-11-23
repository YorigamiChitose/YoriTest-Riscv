#![no_std]
#![no_main]

use cpu_test::{check, finish};

macro_rules! csr_get {
    ($csr:literal) => {{
        let r: u32;
        unsafe {
            core::arch::asm!(concat!("csrrs {r}, ", $csr, ", zero"), r = out(reg) r);
        }
        r
    }};
}

macro_rules! csr_rw {
    ($id:expr, $insn:literal, $v:expr, $old:expr, $new:expr) => {{
        let v: u32 = $v;
        {
            let r: u32;
            unsafe {
                core::arch::asm!($insn, v = in(reg) v, r = out(reg) r);
            }
            cpu_test::check(r == $old, $id);
        }
        {
            let r: u32;
            unsafe {
                core::arch::asm!("csrrs {r}, 0x305, zero", r = out(reg) r);
            }
            cpu_test::check(r == $new, $id + 1);
        }
    }};
}

#[no_mangle]
pub extern "C" fn main() -> ! {

    check(csr_get!("0x300") == 0x1800, 1);

    unsafe {
        core::arch::asm!("csrrw zero, 0x305, zero");
    }

    csr_rw!(2, "csrrw {r}, 0x305, {v}", 0x1234, 0, 0x1234);

    csr_rw!(4, "csrrs {r}, 0x305, {v}", 0x0F00, 0x1234, 0x1F34);

    csr_rw!(6, "csrrc {r}, 0x305, {v}", 0x0F00, 0x1F34, 0x1034);

    {
        let r: u32;
        unsafe {
            core::arch::asm!("csrrwi {r}, 0x305, 0x10", r = out(reg) r);
        }
        check(r == 0x1034, 8);
        check(csr_get!("0x305") == 0x10, 9);
    }

    {
        let r: u32;
        unsafe {
            core::arch::asm!("csrrsi {r}, 0x305, 0x0C", r = out(reg) r);
        }
        check(r == 0x10, 10);
        check(csr_get!("0x305") == 0x1C, 11);
    }

    {
        let r: u32;
        unsafe {
            core::arch::asm!("csrrci {r}, 0x305, 0x04", r = out(reg) r);
        }
        check(r == 0x1C, 12);
        check(csr_get!("0x305") == 0x18, 13);
    }

    finish();
}
