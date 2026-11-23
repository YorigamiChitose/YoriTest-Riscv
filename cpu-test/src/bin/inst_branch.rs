#![no_std]
#![no_main]

use cpu_test::{check, finish};

macro_rules! t_branch {
    ($id:expr, $insn:literal, $a:expr, $b:expr, $taken:expr) => {{
        let a: u32 = $a;
        let b: u32 = $b;
        let r: u32;
        unsafe {
            core::arch::asm!(
                "addi {r}, zero, 0",
                $insn,
                "addi {r}, zero, 1",
                "1:",
                a = in(reg) a,
                b = in(reg) b,
                r = out(reg) r
            );
        }
        check((r == 0) == $taken, $id);
    }};
}

#[no_mangle]
pub extern "C" fn main() -> ! {

    t_branch!(1, "beq {a}, {b}, 1f", 5, 5, true);
    t_branch!(2, "beq {a}, {b}, 1f", 5, 6, false);
    t_branch!(3, "beq {a}, {b}, 1f", 0, 0, true);

    t_branch!(4, "bne {a}, {b}, 1f", 5, 6, true);
    t_branch!(5, "bne {a}, {b}, 1f", 5, 5, false);

    t_branch!(6, "blt {a}, {b}, 1f", 0xFFFF_FFFF, 1, true);
    t_branch!(7, "blt {a}, {b}, 1f", 1, 0xFFFF_FFFF, false);
    t_branch!(8, "blt {a}, {b}, 1f", 3, 3, false);

    t_branch!(9, "bge {a}, {b}, 1f", 2, 1, true);
    t_branch!(10, "bge {a}, {b}, 1f", 2, 2, true);
    t_branch!(11, "bge {a}, {b}, 1f", 1, 2, false);
    t_branch!(12, "bge {a}, {b}, 1f", 0xFFFF_FFFF, 1, false);

    t_branch!(13, "bltu {a}, {b}, 1f", 1, 2, true);
    t_branch!(14, "bltu {a}, {b}, 1f", 0xFFFF_FFFF, 1, false);
    t_branch!(15, "bltu {a}, {b}, 1f", 3, 3, false);

    t_branch!(16, "bgeu {a}, {b}, 1f", 0xFFFF_FFFF, 1, true);
    t_branch!(17, "bgeu {a}, {b}, 1f", 1, 0xFFFF_FFFF, false);
    t_branch!(18, "bgeu {a}, {b}, 1f", 7, 7, true);

    finish();
}
