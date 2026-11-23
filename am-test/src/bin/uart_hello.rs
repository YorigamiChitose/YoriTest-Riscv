#![no_std]
#![no_main]

use am::{halt, putchar};

fn putstr(s: &str) {
    for b in s.bytes() {
        putchar(b);
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    putstr("Hello, YoriCore!\n");
    putstr("UART test OK.\n");

    halt(0);
}
