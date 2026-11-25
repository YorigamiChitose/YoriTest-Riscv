#![no_std]
#![no_main]

use am::{halt, init_trap, timer_start, Context, Event};

static mut TICKS: u32 = 0;

extern "C" fn schedule(ev: Event, prev: *mut Context) -> *mut Context {
    unsafe {
        if ev != Event::Timer {
            halt(1);
        }
        let t = core::ptr::read_volatile(core::ptr::addr_of!(TICKS)) + 1;
        core::ptr::write_volatile(core::ptr::addr_of_mut!(TICKS), t);
        if t == 3 {
            halt(0);
        }
        timer_start(20000);
        prev
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    init_trap(schedule);
    timer_start(20000);
    loop {
        core::hint::spin_loop();
    }
}
