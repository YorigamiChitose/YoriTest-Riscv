//! 串口测试：验证 `am::putchar` 能正确把字符输出到仿真器 stdout。

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
    // 正常退出
    halt(0);
}
