#![no_std]
#![no_main]

use cpu_test::{check, finish};

static mut MEM: [u32; 64] = [0; 64];

fn base() -> u32 {
    unsafe { core::ptr::addr_of_mut!(MEM) as u32 }
}

fn set(off: u32, v: u32) {
    unsafe { core::ptr::write_volatile((base() + off) as *mut u32, v); }
}

fn get(off: u32) -> u32 {
    unsafe { core::ptr::read_volatile((base() + off) as *const u32) }
}

macro_rules! t_amo {
    ($id:expr, $insn:literal, $old:expr, $v:expr, $exp:expr) => {{
        set(0, $old as u32);
        let r: u32;
        unsafe {
            core::arch::asm!(
                $insn,
                r = out(reg) r,
                v = in(reg) $v as u32,
                a = in(reg) base(),
                options(nostack)
            );
        }
        check(r == $old as u32, $id);
        check(get(0) == $exp as u32, $id + 1);
    }};
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        t_amo!(1, "amoswap.w {r}, {v}, ({a})", 0x1111_1111, 0x2222_2222, 0x2222_2222);
        t_amo!(3, "amoadd.w {r}, {v}, ({a})", 0x10, 0x20, 0x30);
        t_amo!(5, "amoxor.w {r}, {v}, ({a})", 0xF0, 0x3C, 0xCC);
        t_amo!(7, "amoand.w {r}, {v}, ({a})", 0xF0, 0x3C, 0x30);
        t_amo!(9, "amoor.w {r}, {v}, ({a})", 0xF0, 0x0F, 0xFF);
        t_amo!(11, "amomin.w {r}, {v}, ({a})", 0x8000_0000, 0x7FFF_FFFF, 0x8000_0000);
        t_amo!(13, "amomax.w {r}, {v}, ({a})", 0x8000_0000, 0x7FFF_FFFF, 0x7FFF_FFFF);
        t_amo!(15, "amominu.w {r}, {v}, ({a})", 0xFFFF_FFFF, 1, 1);
        t_amo!(17, "amomaxu.w {r}, {v}, ({a})", 0xFFFF_FFFF, 1, 0xFFFF_FFFF);
        t_amo!(19, "amoadd.w {r}, {v}, ({a})", 0xFFFF_FFFF, 2, 1);
        t_amo!(21, "amomax.w {r}, {v}, ({a})", 0xFFFF_FFFF, 1, 1);
        t_amo!(23, "amomin.w {r}, {v}, ({a})", 0xFFFF_FFFF, 1, 0xFFFF_FFFF);
        t_amo!(25, "amominu.w {r}, {v}, ({a})", 1, 0x8000_0000, 1);
        t_amo!(27, "amomaxu.w {r}, {v}, ({a})", 1, 0x8000_0000, 0x8000_0000);

        set(0, 5);
        core::arch::asm!(
            "amoadd.w x0, {v}, ({a})",
            v = in(reg) 7u32,
            a = in(reg) base(),
            options(nostack)
        );
        check(get(0) == 12, 29);

        set(0, 9);
        let r: u32;
        core::arch::asm!(
            "amoor.w {r}, x0, ({a})",
            r = out(reg) r,
            a = in(reg) base(),
            options(nostack)
        );
        check(r == 9, 30);
        check(get(0) == 9, 31);

        set(0, 10);
        let mut a = base();
        core::arch::asm!(
            "amoadd.w {a}, {v}, ({a})",
            a = inout(reg) a,
            v = in(reg) 5u32,
            options(nostack)
        );
        check(a == 10, 32);
        check(get(0) == 15, 33);

        set(0, 20);
        let mut v: u32 = 6;
        core::arch::asm!(
            "amoadd.w {v}, {v}, ({a})",
            v = inout(reg) v,
            a = in(reg) base(),
            options(nostack)
        );
        check(v == 20, 34);
        check(get(0) == 26, 35);

        set(0, 1);
        let r: u32;
        core::arch::asm!(
            "amoadd.w.aqrl {r}, {v}, ({a})",
            r = out(reg) r,
            v = in(reg) 2u32,
            a = in(reg) base(),
            options(nostack)
        );
        check(r == 1, 36);
        check(get(0) == 3, 37);

        set(0, 0x55);
        core::arch::asm!(
            "amoadd.w x0, x0, ({a})",
            a = in(reg) base(),
            options(nostack)
        );
        check(get(0) == 0x55, 38);
    }

    finish();
}
