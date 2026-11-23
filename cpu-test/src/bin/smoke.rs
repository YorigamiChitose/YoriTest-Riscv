#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn main() -> ! {

    let x = 1u32 + 1u32;
    if x == 2 {
        am::halt(0);
    } else {
        am::halt(1);
    }
}
