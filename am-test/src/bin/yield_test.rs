#![no_std]
#![no_main]

use core::ptr::addr_of_mut;

use am::{halt, init_trap, make_context, putchar, yield_now, Context, Event};

const STACK_SIZE: usize = 4096;

const SWITCH_TOTAL: u32 = 10;

#[repr(align(16))]
#[allow(dead_code)]
struct Stack([u8; STACK_SIZE]);

static mut STACK_A: Stack = Stack([0; STACK_SIZE]);
static mut STACK_B: Stack = Stack([0; STACK_SIZE]);

static mut CTX_A: *mut Context = core::ptr::null_mut();
static mut CTX_B: *mut Context = core::ptr::null_mut();

static mut CURRENT: usize = 2;

static mut SWITCHES: u32 = 0;

extern "C" fn co_a(_arg: usize) {
    loop {
        putchar(b'A');
        unsafe { SWITCHES += 1; }
        yield_now();
    }
}

extern "C" fn co_b(_arg: usize) {
    loop {
        putchar(b'B');
        unsafe { SWITCHES += 1; }
        yield_now();
    }
}

extern "C" fn schedule(_ev: Event, prev: *mut Context) -> *mut Context {
    unsafe {
        if SWITCHES >= SWITCH_TOTAL {
            putchar(b'\n');
            halt(0);
        }

        match CURRENT {
            0 => CTX_A = prev,
            1 => CTX_B = prev,
            _ => {}
        }

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

        yield_now();
    }

    halt(0);
}
