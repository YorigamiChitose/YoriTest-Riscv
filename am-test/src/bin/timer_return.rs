#![no_std]
#![no_main]

use am::{halt, init_trap, timer_start, timer_stop, Context, Event};

static mut FLAG: u32 = 0;

extern "C" fn schedule(ev: Event, prev: *mut Context) -> *mut Context {
    unsafe {
        if ev != Event::Timer {
            halt(1);
        }
        core::ptr::write_volatile(core::ptr::addr_of_mut!(FLAG), 1);
        timer_stop();
        prev
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    init_trap(schedule);
    timer_start(1000);
    loop {
        unsafe {
            if core::ptr::read_volatile(core::ptr::addr_of!(FLAG)) == 1 {
                halt(0);
            }
        }
        core::hint::spin_loop();
    }
}
