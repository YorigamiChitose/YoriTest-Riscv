//! SB / SH / SW 存储指令测试（含字节/半字写使能，不破坏相邻字节）。

#![no_std]
#![no_main]

use cpu_test::finish;

// 可变缓冲（落入 .bss，_start 清零）。用 0xAA 预填充以检验「只改目标字节」。
static mut MEM: [u8; 12] = [0xAA; 12];

/// 存储测试：执行 `$store`（`{v}` 数据 / `{base}` 基址），再 `$load` 读回断言 == `$e`。
macro_rules! t_store {
    ($id:expr, $store:literal, $v:expr, $load:literal, $e:expr) => {{
        let v: u32 = $v;
        unsafe {
            let base = core::ptr::addr_of_mut!(MEM) as u32;
            core::arch::asm!($store, v = in(reg) v, base = in(reg) base);
            let r: u32;
            core::arch::asm!($load, base = in(reg) base, r = out(reg) r);
            cpu_test::check(r == $e, $id);
        }
    }};
}

/// 纯载入校验（用于检查相邻字节未被破坏）。
macro_rules! t_load {
    ($id:expr, $insn:literal, $e:expr) => {{
        unsafe {
            let base = core::ptr::addr_of_mut!(MEM) as u32;
            let r: u32;
            core::arch::asm!($insn, base = in(reg) base, r = out(reg) r);
            cpu_test::check(r == $e, $id);
        }
    }};
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // SB：写 0x11 到 offset 3，仅改 1 字节
    t_store!(1, "sb {v}, 3({base})", 0x11, "lw {r}, 0({base})", 0x11AA_AAAA);

    // SH：写 0x2233 到 offset 4，仅改 2 字节
    t_store!(2, "sh {v}, 4({base})", 0x2233, "lw {r}, 4({base})", 0xAAAA_2233);

    // SW：写 0xDEADBEEF 到 offset 4，覆盖 4 字节
    t_store!(3, "sw {v}, 4({base})", 0xDEAD_BEEF, "lw {r}, 4({base})", 0xDEAD_BEEF);

    // 相邻字节未被破坏
    t_load!(4, "lw {r}, 0({base})", 0x11AA_AAAA); // offset 0..4 保持 SB 结果
    t_load!(5, "lw {r}, 8({base})", 0xAAAA_AAAA); // offset 8..12 仍 0xAA

    finish();
}
