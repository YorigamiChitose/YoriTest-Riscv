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
        set(0, 0x1111_2222);
        let old: u32;
        let ok: u32;
        core::arch::asm!(
            "lr.w {old}, ({a})",
            "sc.w {ok}, {v}, ({a})",
            old = out(reg) old,
            ok = out(reg) ok,
            a = in(reg) base(),
            v = in(reg) 0xAAAA_BBBBu32,
            options(nostack)
        );
        check(old == 0x1111_2222, 1);
        check(ok == 0, 2);
        check(get(0) == 0xAAAA_BBBB, 3);

        set(4, 0x1234_5678);
        let ok: u32;
        core::arch::asm!(
            "sc.w {ok}, {v}, ({a})",
            ok = out(reg) ok,
            a = in(reg) base() + 4,
            v = in(reg) 0xDEAD_BEEFu32,
            options(nostack)
        );
        check(ok == 1, 4);
        check(get(4) == 0x1234_5678, 5);

        set(0, 0xAA00_0001);
        set(4, 0xBB00_0002);
        let old: u32;
        let ok: u32;
        core::arch::asm!(
            "lr.w {old}, ({a})",
            "sc.w {ok}, {v}, ({b})",
            old = out(reg) old,
            ok = out(reg) ok,
            a = in(reg) base(),
            b = in(reg) base() + 4,
            v = in(reg) 0xCCCC_CCCCu32,
            options(nostack)
        );
        check(old == 0xAA00_0001, 6);
        check(ok == 1, 7);
        check(get(0) == 0xAA00_0001, 8);
        check(get(4) == 0xBB00_0002, 9);

        set(0, 0x1234);
        let old: u32;
        let tmp: u32;
        let ok: u32;
        core::arch::asm!(
            "lr.w {old}, ({a})",
            "lw {tmp}, 0({a})",
            "sc.w {ok}, {v}, ({a})",
            old = out(reg) old,
            tmp = out(reg) tmp,
            ok = out(reg) ok,
            a = in(reg) base(),
            v = in(reg) 0x5678u32,
            options(nostack)
        );
        check(old == 0x1234, 13);
        check(tmp == 0x1234, 14);
        check(ok == 0, 15);
        check(get(0) == 0x5678, 16);

        set(0, 0xAAAA);
        set(4, 0xBBBB);
        let o1: u32;
        let o2: u32;
        let ok: u32;
        core::arch::asm!(
            "lr.w {o1}, ({a})",
            "lr.w {o2}, ({b})",
            "sc.w {ok}, {v}, ({a})",
            o1 = out(reg) o1,
            o2 = out(reg) o2,
            ok = out(reg) ok,
            a = in(reg) base(),
            b = in(reg) base() + 4,
            v = in(reg) 0xCCCCu32,
            options(nostack)
        );
        check(o1 == 0xAAAA, 17);
        check(o2 == 0xBBBB, 18);
        check(ok == 1, 19);
        check(get(0) == 0xAAAA, 20);

        set(0, 0x600);
        let ok1: u32;
        let ok2: u32;
        core::arch::asm!(
            "lr.w {o}, ({a})",
            "sc.w {ok1}, {v1}, ({a})",
            "sc.w {ok2}, {v2}, ({a})",
            o = out(reg) _,
            ok1 = out(reg) ok1,
            ok2 = out(reg) ok2,
            a = in(reg) base(),
            v1 = in(reg) 0x700u32,
            v2 = in(reg) 0x800u32,
            options(nostack)
        );
        check(ok1 == 0, 26);
        check(ok2 == 1, 27);
        check(get(0) == 0x700, 28);

        set(0, 0x900);
        let ok: u32;
        core::arch::asm!(
            "lr.w x0, ({a})",
            "sc.w {ok}, {v}, ({a})",
            ok = out(reg) ok,
            a = in(reg) base(),
            v = in(reg) 0xA00u32,
            options(nostack)
        );
        check(ok == 0, 29);
        check(get(0) == 0xA00, 30);

        set(0, 0xB00);
        let ok: u32;
        core::arch::asm!(
            "lr.w {o}, ({a})",
            "sc.w {ok}, x0, ({a})",
            o = out(reg) _,
            ok = out(reg) ok,
            a = in(reg) base(),
            options(nostack)
        );
        check(ok == 0, 31);
        check(get(0) == 0, 32);

        set(0, 0xC00);
        core::arch::asm!(
            "lr.w {o}, ({a})",
            "sc.w x0, {v}, ({a})",
            o = out(reg) _,
            a = in(reg) base(),
            v = in(reg) 0xD00u32,
            options(nostack)
        );
        check(get(0) == 0xD00, 33);

        set(0, 0xE00);
        core::arch::asm!(
            "lr.w.aqrl {o}, ({a})",
            "sc.w.aqrl x0, {v}, ({a})",
            o = out(reg) _,
            a = in(reg) base(),
            v = in(reg) 0xF00u32,
            options(nostack)
        );
        check(get(0) == 0xF00, 34);

        set(0, 0x1000);
        set(128, 0x2000);
        let ok: u32;
        core::arch::asm!(
            "lr.w {o}, ({a})",
            "sc.w {ok}, {v}, ({b})",
            o = out(reg) _,
            ok = out(reg) ok,
            a = in(reg) base(),
            b = in(reg) base() + 128,
            v = in(reg) 0x3000u32,
            options(nostack)
        );
        check(ok == 1, 35);
        check(get(128) == 0x2000, 36);
    }

    finish();
}
