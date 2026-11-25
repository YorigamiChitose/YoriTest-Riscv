#![no_std]

use core::panic::PanicInfo;

pub const HALT_ADDR: usize = 0xF000_0000;

pub const SERIAL_PORT: usize = 0x1000_0000;

pub const RTC_ADDR: usize = 0x0010_1000;
pub const CLINT_ADDR: usize = 0x0200_0000;

#[no_mangle]
pub extern "C" fn halt(code: u32) -> ! {
    unsafe {
        core::arch::asm!(
            ".option push",
            ".option norvc",
            "call am_halt",
            ".option pop",
            in("a0") code,
            options(noreturn, nostack)
        );
    }
}

core::arch::global_asm!(
    r#"
    .section .text.am_halt, "ax", @progbits
    .globl am_halt
am_halt:
    .option push
    .option norvc
    slli a0, a0, 1
    ori  a0, a0, 1
    li   a1, 0xF0000000
    sb   a0, 0(a1)
1:
    j    1b
    .option pop
"#
);

const LSR_OFFSET: usize = 5;
const LSR_TX_EMPTY: u8 = 0x20;

pub fn putchar(c: u8) {
    unsafe {
        while core::ptr::read_volatile((SERIAL_PORT + LSR_OFFSET) as *const u8) & LSR_TX_EMPTY == 0 {
            core::hint::spin_loop();
        }
        core::ptr::write_volatile(SERIAL_PORT as *mut u8, c);
    }
}

pub fn mtime() -> u64 {
    unsafe {
        let lo = core::ptr::read_volatile((CLINT_ADDR + 0xbff8) as *const u32) as u64;
        let hi = core::ptr::read_volatile((CLINT_ADDR + 0xbffc) as *const u32) as u64;
        (hi << 32) | lo
    }
}

pub fn timer_start(ticks: u64) {
    let target = mtime().wrapping_add(ticks);
    unsafe {
        core::ptr::write_volatile((CLINT_ADDR + 0x4000) as *mut u32, target as u32);
        core::ptr::write_volatile((CLINT_ADDR + 0x4004) as *mut u32, (target >> 32) as u32);
        core::arch::asm!("csrs mie, {}", in(reg) 0x80u32);
    }
}

pub fn timer_stop() {
    unsafe {
        core::ptr::write_volatile((CLINT_ADDR + 0x4000) as *mut u32, 0xffff_ffff);
        core::ptr::write_volatile((CLINT_ADDR + 0x4004) as *mut u32, 0xffff_ffff);
        core::arch::asm!("csrc mie, {}", in(reg) 0x80u32);
    }
}

pub fn rtc_us() -> u64 {
    unsafe {
        let lo = core::ptr::read_volatile(RTC_ADDR as *const u32) as u64;
        let hi = core::ptr::read_volatile((RTC_ADDR + 4) as *const u32) as u64;
        (hi << 32) | lo
    }
}

#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Error = 0,
    Yield = 1,
    Syscall = 2,
    Timer = 3,
}

const MCAUSE_ECALL_M: usize = 11;
const MCAUSE_TIMER: usize = 0x8000_0007;

const MSTATUS_INIT: usize = 0x1880;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Context {
    pub gpr: [usize; 32],
    pub mcause: usize,
    pub mstatus: usize,
    pub mepc: usize,
}

impl Context {

    pub const fn new() -> Self {
        Context {
            gpr: [0; 32],
            mcause: 0,
            mstatus: 0,
            mepc: 0,
        }
    }
}

pub type EventHandler = extern "C" fn(Event, *mut Context) -> *mut Context;

static mut USER_HANDLER: Option<EventHandler> = None;

extern "C" {

    fn am_trap_entry();
}

pub fn init_trap(handler: EventHandler) {
    unsafe {
        USER_HANDLER = Some(handler);
        let f: unsafe extern "C" fn() = am_trap_entry;
        let entry = f as usize;
        core::arch::asm!("csrw mtvec, {}", in(reg) entry);
        core::arch::asm!("csrs mstatus, {}", in(reg) 0x8u32);
    }
}

pub fn yield_now() {
    unsafe {
        core::arch::asm!("li a7, -1", "ecall", out("a7") _);
    }
}

pub fn make_context(stack: &mut [u8], entry: extern "C" fn(usize), arg: usize) -> *mut Context {
    let size = core::mem::size_of::<Context>();
    let stack_top = stack.as_ptr() as usize + stack.len();

    let ctx = (stack_top - size) as *mut Context;
    unsafe {
        let c = &mut *ctx;
        *c = Context::new();
        c.mstatus = MSTATUS_INIT;
        c.mepc = entry as usize;
        c.gpr[10] = arg;
    }
    ctx
}

#[no_mangle]
extern "C" fn am_trap_handler(c: *mut Context) -> *mut Context {
    unsafe {
        let ctx = &mut *c;
        let event = if ctx.mcause == MCAUSE_ECALL_M {
            if ctx.gpr[17] == usize::MAX {
                Event::Yield
            } else {
                Event::Syscall
            }
        } else if ctx.mcause == MCAUSE_TIMER {
            Event::Timer
        } else {
            Event::Error
        };

        if ctx.mcause == MCAUSE_ECALL_M {
            ctx.mepc += 4;
        }
        ctx.mstatus |= 0x80;
        match USER_HANDLER {
            Some(h) => h(event, c),
            None => c,
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt(1);
}

core::arch::global_asm!(
    r#"
    .section .text._start
    .globl _start
_start:
    .option push
    .option norvc

    la   t0, _bss_start
    la   t1, _bss_end
1:
    bgeu t0, t1, 2f
    sb   zero, 0(t0)
    addi t0, t0, 1
    j    1b
2:

    la   sp, _stack_top

    call main

    li   a0, 1
    call halt
    j    .
    .option pop
"#
);

core::arch::global_asm!(
    r#"
    .section .text.trap
    .balign 4
    .globl am_trap_entry
am_trap_entry:
    .option push
    .option norvc
    addi sp, sp, -140

    sw   ra,    4(sp)
    sw   gp,   12(sp)
    sw   tp,   16(sp)
    sw   t0,   20(sp)
    sw   t1,   24(sp)
    sw   t2,   28(sp)
    sw   fp,   32(sp)
    sw   s1,   36(sp)
    sw   a0,   40(sp)
    sw   a1,   44(sp)
    sw   a2,   48(sp)
    sw   a3,   52(sp)
    sw   a4,   56(sp)
    sw   a5,   60(sp)
    sw   a6,   64(sp)
    sw   a7,   68(sp)
    sw   s2,   72(sp)
    sw   s3,   76(sp)
    sw   s4,   80(sp)
    sw   s5,   84(sp)
    sw   s6,   88(sp)
    sw   s7,   92(sp)
    sw   s8,   96(sp)
    sw   s9,  100(sp)
    sw   s10, 104(sp)
    sw   s11, 108(sp)
    sw   t3,  112(sp)
    sw   t4,  116(sp)
    sw   t5,  120(sp)
    sw   t6,  124(sp)

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
    ori  t1, t1, 128
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
    .option pop
"#
);
