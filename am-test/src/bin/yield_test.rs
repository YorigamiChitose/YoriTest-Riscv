//! yield 测试（异常实现）：两个协程通过 `ecall` 陷入异常入口切换，
//! 交替输出 A/B，共切换 SWITCH_TOTAL 次后正常退出（halt(0)）。

#![no_std]
#![no_main]

use core::ptr::addr_of_mut;

use am::{halt, init_trap, make_context, putchar, yield_now, Context, Event};

/// 每个协程的独立栈大小（字节）。
const STACK_SIZE: usize = 4096;
/// 总切换次数（= 交替输出的字符数，输出 "ABAB..."）。
const SWITCH_TOTAL: u32 = 10;

/// 16 字节对齐的栈（满足 RISC-V ABI 栈对齐）。字段仅作栈空间，从不读内容。
#[repr(align(16))]
#[allow(dead_code)]
struct Stack([u8; STACK_SIZE]);

static mut STACK_A: Stack = Stack([0; STACK_SIZE]);
static mut STACK_B: Stack = Stack([0; STACK_SIZE]);

/// 两个协程的上下文指针（由 make_context 创建，schedule 里更新）。
static mut CTX_A: *mut Context = core::ptr::null_mut();
static mut CTX_B: *mut Context = core::ptr::null_mut();
/// 当前运行的协程：0=A，1=B，2=boot（初始）。
static mut CURRENT: usize = 2;
/// 已发生的切换次数（协程间共享）。
static mut SWITCHES: u32 = 0;

/// 协程 A：输出 'A' 后让出。
extern "C" fn co_a(_arg: usize) {
    loop {
        putchar(b'A');
        unsafe { SWITCHES += 1; }
        yield_now();
    }
}

/// 协程 B：输出 'B' 后让出。
extern "C" fn co_b(_arg: usize) {
    loop {
        putchar(b'B');
        unsafe { SWITCHES += 1; }
        yield_now();
    }
}

/// 调度器：保存 prev 到当前协程，切换到另一个；达到切换次数后退出。
extern "C" fn schedule(_ev: Event, prev: *mut Context) -> *mut Context {
    unsafe {
        if SWITCHES >= SWITCH_TOTAL {
            putchar(b'\n');
            halt(0);
        }
        // 保存 prev 到当前协程（boot 阶段不保留）。
        match CURRENT {
            0 => CTX_A = prev,
            1 => CTX_B = prev,
            _ => {}
        }
        // 切换到另一个协程。
        if CURRENT == 0 {
            CURRENT = 1;
            CTX_B
        } else {
            CURRENT = 0;
            CTX_A
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        let stack_a: &mut [u8] = &mut (*addr_of_mut!(STACK_A)).0;
        let stack_b: &mut [u8] = &mut (*addr_of_mut!(STACK_B)).0;
        CTX_A = make_context(stack_a, co_a, 0);
        CTX_B = make_context(stack_b, co_b, 0);
        init_trap(schedule);
        // 首次 yield：保存 main 现场（丢弃），切入协程 A。
        yield_now();
    }
    // 兜底（正常流程在 schedule 内 halt 退出）。
    halt(0);
}
