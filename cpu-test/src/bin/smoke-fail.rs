//! 冒烟测试：验证「启动 → 调用 am::halt → FAIL」的最小链路。

#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn main() -> ! {
    // 极小自检：直接以 FAIL 退出。
    am::halt(3); // FAIL
}
