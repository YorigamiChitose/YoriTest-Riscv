#![no_std]

#[inline(always)]
pub fn check(cond: bool, id: u32) {
    if !cond {
        am::halt(id & 0x7f);
    }
}

#[inline(always)]
pub fn finish() -> ! {
    am::halt(0);
}

pub struct Checks(u32);

impl Checks {
    pub const fn new() -> Self {
        Checks(0)
    }

    #[inline(always)]
    pub fn check(&mut self, id: u32, cond: bool) {
        debug_assert!(id >= 1 && id <= 6);
        if !cond {
            self.0 |= 1 << id;
        }
    }

    #[inline(always)]
    pub fn finish(self) -> ! {
        am::halt(self.0 & 0x7f);
    }
}

#[macro_export]
macro_rules! t_rr {
    ($id:expr, $insn:literal, $a:expr, $b:expr, $e:expr) => {{
        let a: u32 = $a;
        let b: u32 = $b;
        let r: u32;
        unsafe {
            core::arch::asm!($insn, a = in(reg) a, b = in(reg) b, r = out(reg) r);
        }
        $crate::check(r == $e, $id);
    }};
}

#[macro_export]
macro_rules! t_ri {
    ($id:expr, $insn:literal, $a:expr, $e:expr) => {{
        let a: u32 = $a;
        let r: u32;
        unsafe {
            core::arch::asm!($insn, a = in(reg) a, r = out(reg) r);
        }
        $crate::check(r == $e, $id);
    }};
}
