//! 系统指令测试：FENCE / FENCE.I / WFI。
//!
//! 这三条在当前 YoriCore-Rev 中均为 TODO 空操作，本测试验证其可正常译码、
//! 执行且不引发异常（程序能顺利走到 halt）。
//!
//! 注：ECALL/MRET/EBREAK 的异常处理与陷门往返测试，因涉及 MEPC 语义
//! 与流水 PC 偏移，单列后续跟进，暂不在此覆盖。

#![no_std]
#![no_main]

use cpu_test::finish;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        core::arch::asm!("fence");
        core::arch::asm!("fence.i");
        core::arch::asm!("wfi");
    }

    // 能走到这里说明三条系统指令均未引发异常。
    finish();
}
