#![no_std]
#![no_main]

extern crate alloc;

mod console;
mod heap;
mod sched;
mod sync;
mod time;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;

fn test_heap() {
    let mut v: Vec<u32> = Vec::new();
    for i in 0..64 {
        v.push(i * 3);
    }
    if v.len() != 64 || v[63] != 189 {
        am::halt(20);
    }
    let b = Box::new(0x1234_5678u32);
    if *b != 0x1234_5678 {
        am::halt(21);
    }
    if heap::used() == 0 {
        am::halt(22);
    }
}

fn test_sync() {
    static LOCK: sync::SpinLock = sync::SpinLock::new();
    LOCK.lock();
    sync::COUNTER.fetch_add(1, Ordering::SeqCst);
    LOCK.unlock();
    if sync::COUNTER.load(Ordering::SeqCst) != 1 {
        am::halt(30);
    }
}

#[cfg(not(feature = "difftest"))]
fn test_time() {
    let t0 = time::now_us();
    time::wait_us(1000);
    let t1 = time::now_us();
    if t1 < t0 + 1000 {
        am::halt(40);
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    console::print("os selftest\n");
    test_heap();
    test_sync();
    #[cfg(not(feature = "difftest"))]
    test_time();
    #[cfg(not(feature = "difftest"))]
    console::print("heap/sync/time ok\n");
    #[cfg(feature = "difftest")]
    console::print("heap/sync ok\n");
    sched::init_and_start();
}
