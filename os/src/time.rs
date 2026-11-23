pub fn now_us() -> u64 {
    am::rtc_us()
}

pub fn wait_us(us: u64) {
    let start = now_us();
    while now_us().wrapping_sub(start) < us {
        core::hint::spin_loop();
    }
}
