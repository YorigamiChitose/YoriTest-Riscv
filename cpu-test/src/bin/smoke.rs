//! 冒烟测试：验证「启动 → 调用 am::halt → PASS」的最小链路。

#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn main() -> ! {
    // 极小自检：1 + 1 应等于 2，否则以 FAIL 退出。
    let x = 1u32 + 1u32;
    if x == 2 {
        am::halt(0); // PASS
    } else {
        am::halt(1); // FAIL
    }
}
