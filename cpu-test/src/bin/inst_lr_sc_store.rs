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

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        set(0, 0x1000);
        let old: u32;
        let ok: u32;
        core::arch::asm!(
            "lr.w {old}, ({a})",
            "sw {s}, 0({a})",
            "sc.w {ok}, {v}, ({a})",
            old = out(reg) old,
            ok = out(reg) ok,
            s = in(reg) 0x2000u32,
            a = in(reg) base(),
            v = in(reg) 0x3000u32,
            options(nostack)
        );
        check(old == 0x1000, 1);
        let m = get(0);
        check((ok == 0 && m == 0x3000) || (ok == 1 && m == 0x2000), 2);

        set(0, 0x100);
        let old: u32;
        let amo: u32;
        let ok: u32;
        core::arch::asm!(
            "lr.w {old}, ({a})",
            "amoadd.w {amo}, {inc}, ({a})",
            "sc.w {ok}, {v}, ({a})",
            old = out(reg) old,
            amo = out(reg) amo,
            ok = out(reg) ok,
            a = in(reg) base(),
            inc = in(reg) 0x10u32,
            v = in(reg) 0x9999u32,
            options(nostack)
        );
        check(old == 0x100, 3);
        check(amo == 0x100, 4);
        let m = get(0);
        check((ok == 0 && m == 0x9999) || (ok == 1 && m == 0x110), 5);

        set(0, 0x200);
        set(4, 0x300);
        let ok: u32;
        core::arch::asm!(
            "lr.w {o}, ({a})",
            "sw {s}, 0({b})",
            "sc.w {ok}, {v}, ({a})",
            o = out(reg) _,
            ok = out(reg) ok,
            a = in(reg) base(),
            b = in(reg) base() + 4,
            s = in(reg) 0x400u32,
            v = in(reg) 0x500u32,
            options(nostack)
        );
        let m = get(0);
        check((ok == 0 && m == 0x500) || (ok == 1 && m == 0x200), 6);
    }

    finish();
}
