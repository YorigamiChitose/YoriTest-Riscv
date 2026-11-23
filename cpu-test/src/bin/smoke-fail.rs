#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn main() -> ! {

    am::halt(3);
}
