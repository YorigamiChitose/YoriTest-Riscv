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

// ---- 上下文切换（异常实现，参考 ysyx 的 AM）----

/// 事件类型（`am_trap_handler` 分发给用户调度器）。
#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Error = 0,
    Yield = 1,
    Syscall = 2,
}

/// ecall from M-mode 的 mcause 值（与 YoriCore CsrFile 一致：mod==b11 → 0xb）。
const MCAUSE_ECALL_M: usize = 11;

/// mstatus 复位值 0x1800（MPP=M-mode），`make_context` 用于首次 mret 回到 M-mode。
const MSTATUS_INIT: usize = 0x1800;

/// 完整上下文：32 个通用寄存器（x0..x31）+ mcause/mstatus/mepc。
/// 布局与 `am_trap_entry` 汇编的存取偏移严格对应（gpr[i]@i*4，mcause@128、mstatus@132、mepc@136）。
/// x0 恒为 0、x2(sp) 由汇编隐式恢复，二者不单独保存。
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Context {
    pub gpr: [usize; 32],
    pub mcause: usize,
    pub mstatus: usize,
    pub mepc: usize,
}

impl Context {
    /// 全零上下文（用于 static 初始化）。
    pub const fn new() -> Self {
        Context {
            gpr: [0; 32],
            mcause: 0,
            mstatus: 0,
            mepc: 0,
        }
    }
}

/// 用户调度器：接收 `(event, 上一个 Context)`，返回下一个要运行的 Context。
pub type EventHandler = extern "C" fn(Event, *mut Context) -> *mut Context;

/// 用户注册的调度器（裸机单线程，用 static mut）。
static mut USER_HANDLER: Option<EventHandler> = None;

extern "C" {
    /// 异常入口（汇编实现），由 `mtvec` 指向。
    fn am_trap_entry();
}

/// 初始化上下文与异常：设置 `mtvec` 指向异常入口，并注册用户调度器。
pub fn init_trap(handler: EventHandler) {
    unsafe {
        USER_HANDLER = Some(handler);
        let f: unsafe extern "C" fn() = am_trap_entry;
        let entry = f as usize;
        core::arch::asm!("csrw mtvec, {}", in(reg) entry);
    }
}

/// 让出 CPU：以 `a7 = -1` 触发 ecall，陷入异常入口，由调度器切换到下一个上下文。
pub fn yield_now() {
    unsafe {
        core::arch::asm!("li a7, -1", "ecall", out("a7") _);
    }
}

/// 创建内核上下文：在栈顶伪造一个 Context，首次被切换（mret）时跳到 `entry`，`arg` 作为 a0。
pub fn make_context(stack: &mut [u8], entry: extern "C" fn(usize), arg: usize) -> *mut Context {
    let size = core::mem::size_of::<Context>();
    let stack_top = stack.as_ptr() as usize + stack.len();
    // Context 放在栈顶（栈向下增长，协程运行后会被覆盖）。
    let ctx = (stack_top - size) as *mut Context;
    unsafe {
        let c = &mut *ctx;
        *c = Context::new();
        c.mstatus = MSTATUS_INIT; // MPP = M-mode，mret 后回到 M-mode
        c.mepc = entry as usize;  // 入口地址（mret 跳转目标）
        c.gpr[10] = arg;          // a0 = 参数
    }
    ctx
}

/// 异常分发（由 `am_trap_entry` 以 `a0 = 当前 Context` 调用，返回下一个 Context）。
#[no_mangle]
extern "C" fn am_trap_handler(c: *mut Context) -> *mut Context {
    unsafe {
        let ctx = &mut *c;
        let event = if ctx.mcause == MCAUSE_ECALL_M {
            if ctx.gpr[17] == usize::MAX {
                Event::Yield // a7 == -1
            } else {
                Event::Syscall
            }
        } else {
            Event::Error
        };
        // ecall 的 mepc 指向 ecall 指令，+4 跳过它（yield/syscall 均如此）。
        if ctx.mcause == MCAUSE_ECALL_M {
            ctx.mepc += 4;
        }
        match USER_HANDLER {
            Some(h) => h(event, c),
            None => c,
        }
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

// 异常入口：硬件在 ecall/中断等异常时跳转至此（由 mtvec 指向）。
// 流程：在栈上分配 Context → 保存 x1/x3..x31（跳过 x0 与 x2=sp）与 mcause/mstatus/mepc
//       → 调用 am_trap_handler（a0=当前 Context，返回下一个 Context）→ 切换栈到返回值
//       → 恢复 CSR 与寄存器 → addi sp + mret（跳到新 Context 的 mepc）。
// 偏移与 Context 布局严格一致：gpr[i]@i*4，mcause@128、mstatus@132、mepc@136。
core::arch::global_asm!(
    r#"
    .section .text.trap
    .globl am_trap_entry
am_trap_entry:
    addi sp, sp, -140

    sw   ra,    4(sp)   # x1
    sw   gp,   12(sp)   # x3
    sw   tp,   16(sp)   # x4
    sw   t0,   20(sp)   # x5
    sw   t1,   24(sp)   # x6
    sw   t2,   28(sp)   # x7
    sw   fp,   32(sp)   # x8  (s0)
    sw   s1,   36(sp)   # x9
    sw   a0,   40(sp)   # x10
    sw   a1,   44(sp)   # x11
    sw   a2,   48(sp)   # x12
    sw   a3,   52(sp)   # x13
    sw   a4,   56(sp)   # x14
    sw   a5,   60(sp)   # x15
    sw   a6,   64(sp)   # x16
    sw   a7,   68(sp)   # x17
    sw   s2,   72(sp)   # x18
    sw   s3,   76(sp)   # x19
    sw   s4,   80(sp)   # x20
    sw   s5,   84(sp)   # x21
    sw   s6,   88(sp)   # x22
    sw   s7,   92(sp)   # x23
    sw   s8,   96(sp)   # x24
    sw   s9,  100(sp)   # x25
    sw   s10, 104(sp)   # x26
    sw   s11, 108(sp)   # x27
    sw   t3,  112(sp)   # x28
    sw   t4,  116(sp)   # x29
    sw   t5,  120(sp)   # x30
    sw   t6,  124(sp)   # x31

    csrr t0, mcause
    csrr t1, mstatus
    csrr t2, mepc
    sw   t0, 128(sp)
    sw   t1, 132(sp)
    sw   t2, 136(sp)

    mv   a0, sp
    call am_trap_handler
    mv   sp, a0

    lw   t1, 132(sp)
    lw   t2, 136(sp)
    csrw mstatus, t1
    csrw mepc, t2

    lw   ra,    4(sp)
    lw   gp,   12(sp)
    lw   tp,   16(sp)
    lw   t0,   20(sp)
    lw   t1,   24(sp)
    lw   t2,   28(sp)
    lw   fp,   32(sp)
    lw   s1,   36(sp)
    lw   a0,   40(sp)
    lw   a1,   44(sp)
    lw   a2,   48(sp)
    lw   a3,   52(sp)
    lw   a4,   56(sp)
    lw   a5,   60(sp)
    lw   a6,   64(sp)
    lw   a7,   68(sp)
    lw   s2,   72(sp)
    lw   s3,   76(sp)
    lw   s4,   80(sp)
    lw   s5,   84(sp)
    lw   s6,   88(sp)
    lw   s7,   92(sp)
    lw   s8,   96(sp)
    lw   s9,  100(sp)
    lw   s10, 104(sp)
    lw   s11, 108(sp)
    lw   t3,  112(sp)
    lw   t4,  116(sp)
    lw   t5,  120(sp)
    lw   t6,  124(sp)

    addi sp, sp, 140
    mret
"#
);
