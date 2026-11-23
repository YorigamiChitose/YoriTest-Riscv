#![no_std]
#![no_main]

use cpu_test::{check, finish};

static mut MEM: [u32; 4096] = [0; 4096];

fn base() -> u32 {
    unsafe { core::ptr::addr_of_mut!(MEM) as u32 }
}

fn set(off: u32, v: u32) {
    unsafe { core::ptr::write_volatile((base() + off) as *mut u32, v); }
}

fn get(off: u32) -> u32 {
    unsafe { core::ptr::read_volatile((base() + off) as *const u32) }
}

fn evict() {
    for off in [0x800u32, 0x1000, 0x1800, 0x2000, 0x2800, 0x3000] {
        let _ = get(off);
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        set(0, 0x100);
        evict();
        let old: u32;
        core::arch::asm!(
            "amoadd.w {r}, {v}, ({a})",
            r = out(reg) old,
            v = in(reg) 0x10u32,
            a = in(reg) base(),
            options(nostack)
        );
        check(old == 0x100, 1);
        check(get(0) == 0x110, 2);

        set(0, 0x200);
        evict();
        let old: u32;
        core::arch::asm!(
            "amoadd.w {r}, {v}, ({a})",
            r = out(reg) old,
            v = in(reg) 0x10u32,
            a = in(reg) base(),
            options(nostack)
        );
        check(old == 0x200, 3);
        check(get(0) == 0x210, 4);

        set(28, 0x10);
        set(32, 0x20);
        let o1: u32;
        let o2: u32;
        core::arch::asm!(
            "amoadd.w {o1}, {v1}, ({a1})",
            "amoadd.w {o2}, {v2}, ({a2})",
            o1 = out(reg) o1,
            o2 = out(reg) o2,
            v1 = in(reg) 1u32,
            v2 = in(reg) 2u32,
            a1 = in(reg) base() + 28,
            a2 = in(reg) base() + 32,
            options(nostack)
        );
        check(o1 == 0x10, 5);
        check(o2 == 0x20, 6);
        check(get(28) == 0x11, 7);
        check(get(32) == 0x22, 8);

        set(28, 0x30);
        set(32, 0x40);
        let ok: u32;
        core::arch::asm!(
            "lr.w {o}, ({a})",
            "sc.w {ok}, {v}, ({b})",
            o = out(reg) _,
            ok = out(reg) ok,
            a = in(reg) base() + 28,
            b = in(reg) base() + 32,
            v = in(reg) 0x50u32,
            options(nostack)
        );
        check(ok == 1, 9);
        check(get(32) == 0x40, 10);

        set(28, 0x60);
        let ok: u32;
        core::arch::asm!(
            "lr.w {o}, ({a})",
            "sc.w {ok}, {v}, ({a})",
            o = out(reg) _,
            ok = out(reg) ok,
            a = in(reg) base() + 28,
            v = in(reg) 0x70u32,
            options(nostack)
        );
        check(ok == 0, 11);
        check(get(28) == 0x70, 12);

        set(0, 0x300);
        let old: u32;
        let ok: u32;
        core::arch::asm!(
            "lr.w {old}, ({a})",
            "sc.w {ok}, {v}, ({a})",
            old = out(reg) old,
            ok = out(reg) ok,
            a = in(reg) base(),
            v = in(reg) 0x400u32,
            options(nostack)
        );
        evict();
        let ok2: u32;
        core::arch::asm!(
            "sc.w {ok2}, {v}, ({a})",
            ok2 = out(reg) ok2,
            a = in(reg) base(),
            v = in(reg) 0x500u32,
            options(nostack)
        );
        check(old == 0x300, 13);
        check(ok == 0, 14);
        check(ok2 == 1, 15);
        check(get(0) == 0x400, 16);

        set(0, 0x600);
        let old: u32;
        let ok: u32;
        core::arch::asm!(
            "lr.w {old}, ({a})",
            "sc.w {ok}, {v}, ({a})",
            old = out(reg) old,
            ok = out(reg) ok,
            a = in(reg) base(),
            v = in(reg) 0x700u32,
            options(nostack)
        );
        check(old == 0x600, 17);
        check(ok == 0, 18);
        evict();
        check(get(0) == 0x700, 19);

        set(0, 0x800);
        evict();
        core::arch::asm!(
            "amoadd.w x0, {v}, ({a})",
            v = in(reg) 0x20u32,
            a = in(reg) base(),
            options(nostack)
        );
        check(get(0) == 0x820, 20);
    }

    finish();
}
