#![no_std]
#![no_main]

use cpu_test::finish;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        core::arch::asm!("fence");
        core::arch::asm!("fence.i");
        core::arch::asm!("wfi");
    }

    finish();
}
