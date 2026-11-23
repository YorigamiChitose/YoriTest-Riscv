#![no_std]
#![no_main]

use cpu_test::finish;

#[repr(align(4))]
struct Mem([u8; 16]);

static MEM: Mem = Mem([
    0x80, 0x7F, 0xFF, 0x00,
    0x34, 0x12, 0x78, 0x56,
    0xEF, 0xBE, 0xAD, 0xDE,
    0x00, 0x80, 0xFF, 0x7F,
]);

macro_rules! t_load {
    ($id:expr, $insn:literal, $e:expr) => {{
        let base = MEM.0.as_ptr() as u32;
        let r: u32;
        unsafe {
            core::arch::asm!($insn, base = in(reg) base, r = out(reg) r);
        }
        cpu_test::check(r == $e, $id);
    }};
}

#[no_mangle]
pub extern "C" fn main() -> ! {

    t_load!(1, "lb {r}, 0({base})", 0xFFFF_FF80);
    t_load!(2, "lb {r}, 1({base})", 0x0000_007F);
    t_load!(3, "lb {r}, 2({base})", 0xFFFF_FFFF);

    t_load!(4, "lbu {r}, 0({base})", 0x0000_0080);
    t_load!(5, "lbu {r}, 2({base})", 0x0000_00FF);

    t_load!(6, "lh {r}, 4({base})", 0x0000_1234);
    t_load!(7, "lh {r}, 12({base})", 0xFFFF_8000);
    t_load!(8, "lh {r}, 14({base})", 0x0000_7FFF);

    t_load!(9, "lhu {r}, 12({base})", 0x0000_8000);
    t_load!(10, "lhu {r}, 6({base})", 0x0000_5678);

    t_load!(11, "lw {r}, 8({base})", 0xDEAD_BEEF);
    t_load!(12, "lw {r}, 0({base})", 0x00FF_7F80);
    t_load!(13, "lw {r}, 12({base})", 0x7FFF_8000);

    finish();
}
