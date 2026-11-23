#![no_std]
#![no_main]

use cpu_test::finish;

#[repr(align(4))]
struct Mem([u8; 12]);

static mut MEM: Mem = Mem([0xAA; 12]);

macro_rules! t_store {
    ($id:expr, $store:literal, $v:expr, $load:literal, $e:expr) => {{
        let v: u32 = $v;
        unsafe {
            let base = core::ptr::addr_of_mut!(MEM.0) as u32;
            core::arch::asm!($store, v = in(reg) v, base = in(reg) base);
            let r: u32;
            core::arch::asm!($load, base = in(reg) base, r = out(reg) r);
            cpu_test::check(r == $e, $id);
        }
    }};
}

macro_rules! t_load {
    ($id:expr, $insn:literal, $e:expr) => {{
        unsafe {
            let base = core::ptr::addr_of_mut!(MEM.0) as u32;
            let r: u32;
            core::arch::asm!($insn, base = in(reg) base, r = out(reg) r);
            cpu_test::check(r == $e, $id);
        }
    }};
}

#[no_mangle]
pub extern "C" fn main() -> ! {

    t_store!(1, "sb {v}, 3({base})", 0x11, "lw {r}, 0({base})", 0x11AA_AAAA);

    t_store!(2, "sh {v}, 4({base})", 0x2233, "lw {r}, 4({base})", 0xAAAA_2233);

    t_store!(3, "sw {v}, 4({base})", 0xDEAD_BEEF, "lw {r}, 4({base})", 0xDEAD_BEEF);

    t_load!(4, "lw {r}, 0({base})", 0x11AA_AAAA);
    t_load!(5, "lw {r}, 8({base})", 0xAAAA_AAAA);

    finish();
}
