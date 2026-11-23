//! LB / LBU / LH / LHU / LW 载入指令测试（含符号/零扩展）。

#![no_std]
#![no_main]

use cpu_test::finish;

// 已知字节模式（小端）。非零常量 → 编译期落入 .rodata，无需运行时初始化。
static MEM: [u8; 16] = [
    0x80, 0x7F, 0xFF, 0x00, // [0..4]
    0x34, 0x12, 0x78, 0x56, // [4..8]  half 0x1234 @4, half 0x5678 @6
    0xEF, 0xBE, 0xAD, 0xDE, // [8..12] word 0xDEADBEEF @8
    0x00, 0x80, 0xFF, 0x7F, // [12..16] half 0x8000 @12, half 0x7FFF @14
];

/// 载入测试：执行 `$insn`，断言结果 == `$e`。
macro_rules! t_load {
    ($id:expr, $insn:literal, $e:expr) => {{
        let base = MEM.as_ptr() as u32;
        let r: u32;
        unsafe {
            core::arch::asm!($insn, base = in(reg) base, r = out(reg) r);
        }
        cpu_test::check(r == $e, $id);
    }};
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // ---- LB（符号扩展）----
    t_load!(1, "lb {r}, 0({base})", 0xFFFF_FF80); // 0x80 → 符号扩展
    t_load!(2, "lb {r}, 1({base})", 0x0000_007F); // 0x7F → 正
    t_load!(3, "lb {r}, 2({base})", 0xFFFF_FFFF); // 0xFF → -1

    // ---- LBU（零扩展）----
    t_load!(4, "lbu {r}, 0({base})", 0x0000_0080);
    t_load!(5, "lbu {r}, 2({base})", 0x0000_00FF);

    // ---- LH（符号扩展）----
    t_load!(6, "lh {r}, 4({base})", 0x0000_1234);
    t_load!(7, "lh {r}, 12({base})", 0xFFFF_8000); // 0x8000 → 符号扩展
    t_load!(8, "lh {r}, 14({base})", 0x0000_7FFF);

    // ---- LHU（零扩展）----
    t_load!(9, "lhu {r}, 12({base})", 0x0000_8000);
    t_load!(10, "lhu {r}, 6({base})", 0x0000_5678);

    // ---- LW（整字）----
    t_load!(11, "lw {r}, 8({base})", 0xDEAD_BEEF);
    t_load!(12, "lw {r}, 0({base})", 0x00FF_7F80); // 0x00,0xFF,0x7F,0x80
    t_load!(13, "lw {r}, 12({base})", 0x7FFF_8000); // 0x00,0x80,0xFF,0x7F

    finish();
}
