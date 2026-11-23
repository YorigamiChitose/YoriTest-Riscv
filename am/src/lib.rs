//! YoriTest-Riscv 抽象机（AM）—— 唯一对接硬件的层。
//!
//! 职责：
//! - halt 退出 / 串口（远期）等设备访问
//! - 裸机启动 `_start`（清 bss → 设栈 → 调 `main`）
//! - `#[panic_handler]`（panic → FAIL 退出）
//! - 链接脚本（见 `am/link/linker.lds`）
//!
//! 硬件契约（与 YoriCore-Rev 对齐，见 `sim/inc/autoconf/autoconf.h`）：
//! - 复位向量 / 镜像加载基址：`0x8000_0000`（`CONFIG_MBASE`）
//! - halt 设备：`0xF000_0000`（`CONFIG_HALT_ADDR`），单字节写
//! - 串口：`0x1000_0000`（`CONFIG_SERIAL_PORT`）
//! - RTC：`0x0200_b000`（`CONFIG_RTC_ADDR`），只读 64 位微秒

#![no_std]

use core::panic::PanicInfo;

/// halt 设备地址，与 YoriCore-Rev 的 `CONFIG_HALT_ADDR` 一致。
pub const HALT_ADDR: usize = 0xF000_0000;

/// 串口基址，与 YoriCore-Rev 的 `CONFIG_SERIAL_PORT` 一致。
pub const SERIAL_PORT: usize = 0x1000_0000;

/// RTC 基址，与 YoriCore-Rev 的 `CONFIG_RTC_ADDR` 一致（只读 64 位微秒）。
pub const RTC_ADDR: usize = 0x0200_B000;

/// 写 halt 设备并停机（永不返回）。
///
/// 编码约定（见 YoriCore-Rev `sim/src/cpu/cpu-exec.cpp` 的 `handle_halt`）：
/// - `code == 0` → 写 `1` → PASS（HIT GOOD TRAP）
/// - `code >  0` → 写 `(code << 1) | 1` → FAIL（HIT BAD TRAP，`code` 为失败码）
///
/// halt 设备要求恰好 1 字节写（`sb`），故这里用 `write_volatile` 写 `u8`。
#[no_mangle]
pub extern "C" fn halt(code: u32) -> ! {
    unsafe {
        core::ptr::write_volatile(HALT_ADDR as *mut u8, ((code << 1) | 1) as u8);
    }
    // 模拟器命中 halt 后即停摆，不会回到这里；死循环仅作兜底。
    loop {
        core::hint::spin_loop();
    }
}

/// ns16550 线路状态寄存器（LSR）偏移与「发送保持寄存器空」状态位。
const LSR_OFFSET: usize = 5;
const LSR_TX_EMPTY: u8 = 0x20;

/// 写串口单字符：轮询 LSR 直到发送寄存器空，再写 THR 发送。
pub fn putchar(c: u8) {
    unsafe {
        while core::ptr::read_volatile((SERIAL_PORT + LSR_OFFSET) as *const u8) & LSR_TX_EMPTY == 0 {
            core::hint::spin_loop();
        }
        core::ptr::write_volatile(SERIAL_PORT as *mut u8, c);
    }
}

/// 读 RTC 的 64 位微秒时间戳（只读设备；低 32 位 @+0，高 32 位 @+4）。
pub fn rtc_us() -> u64 {
    unsafe {
        let lo = core::ptr::read_volatile(RTC_ADDR as *const u32) as u64;
        let hi = core::ptr::read_volatile((RTC_ADDR + 4) as *const u32) as u64;
        (hi << 32) | lo
    }
}

/// panic 处理：直接以 FAIL（失败码 1）退出，不依赖默认运行库。
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt(1);
}

// 裸机启动入口（链接脚本 `ENTRY(_start)` 指向此处）。
// 仅使用最基础指令：lui/addi/lw/sw/beq/bne/jal/jalr，避免与被测指令形成循环依赖。
core::arch::global_asm!(
    r#"
    .section .text._start
    .globl _start
_start:
    // 清零 .bss 段（_bss_start ~ _bss_end，按字节清零，兼容任意对齐/大小）
    la   t0, _bss_start
    la   t1, _bss_end
1:
    bgeu t0, t1, 2f
    sb   zero, 0(t0)
    addi t0, t0, 1
    j    1b
2:
    // 设置栈指针（栈向下增长）
    la   sp, _stack_top
    // 调用用户 main（bin crate 以 #[no_mangle] extern "C" fn main 提供）
    call main
    // main 返回视为异常，以 FAIL(1) 退出
    li   a0, 1
    call halt
    j    .
"#
);
