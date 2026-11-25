#![no_std]
#![no_main]

use am::{halt, init_trap, timer_start, timer_stop, Context, Event};

static mut TICKS: u32 = 0;

extern "C" fn schedule(ev: Event, _prev: *mut Context) -> *mut Context {
    unsafe {
        match ev {
            Event::Timer => {
                TICKS += 1;
                timer_stop();
                halt(0);
            }
            _ => halt(1),
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    init_trap(schedule);
    timer_start(1000);
    loop {
        core::hint::spin_loop();
    }
}
