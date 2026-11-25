#![no_std]
#![no_main]

use am::{halt, init_trap, make_context, timer_start, yield_now, Context, Event};

const STACK_SIZE: usize = 2048;

#[repr(align(16))]
struct Stack([u8; STACK_SIZE]);

static mut S0: Stack = Stack([0; STACK_SIZE]);
static mut S1: Stack = Stack([0; STACK_SIZE]);
static mut C0: *mut Context = core::ptr::null_mut();
static mut C1: *mut Context = core::ptr::null_mut();
static mut CURRENT: usize = 2;
static mut TICKS: u32 = 0;

extern "C" fn t0(_: usize) {
    loop { unsafe { core::arch::asm!("nop"); } }
}
extern "C" fn t1(_: usize) {
    loop { unsafe { core::arch::asm!("nop"); } }
}

extern "C" fn schedule(ev: Event, prev: *mut Context) -> *mut Context {
    unsafe {
        match ev {
            Event::Yield => {}
            Event::Timer => {
                let t = core::ptr::read_volatile(core::ptr::addr_of!(TICKS)) + 1;
                core::ptr::write_volatile(core::ptr::addr_of_mut!(TICKS), t);
                if t >= 3 {
                    halt(0);
                }
                timer_start(10000);
            }
            _ => halt(1),
        }
        if CURRENT < 2 {
            if CURRENT == 0 { C0 = prev; } else { C1 = prev; }
        }
        CURRENT = if CURRENT == 0 { 1 } else { 0 };
        if CURRENT == 0 { C0 } else { C1 }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        C0 = make_context(&mut (*core::ptr::addr_of_mut!(S0)).0, t0, 0);
        C1 = make_context(&mut (*core::ptr::addr_of_mut!(S1)).0, t1, 0);
        init_trap(schedule);
        timer_start(10000);
        yield_now();
    }
    loop { unsafe { core::arch::asm!("nop"); } }
}
