//! RTC 测试：读 RTC 微秒时间戳，在测试端解析为当前时间并输出 `yy-mm-dd hh-mm-ss`。

#![no_std]
#![no_main]

use am::{halt, putchar, rtc_us};

/// 时区偏移（秒）：UTC+8（北京时间）。
const TZ_OFFSET_SEC: u32 = 8 * 3600;

/// 补零输出一个无符号整数（宽度 width）。
fn put_pad(n: u32, width: usize) {
    let mut buf = [0u8; 10];
    let mut i = 10;
    let mut v = n;
    loop {
        i -= 1;
        buf[i] = (v % 10) as u8 + b'0';
        v /= 10;
        if v == 0 {
            break;
        }
    }
    while 10 - i < width {
        i -= 1;
        buf[i] = b'0';
    }
    for j in i..10 {
        putchar(buf[j]);
    }
}

/// 把自 1970-01-01 的天数转成 (year, month, day)，month/day 从 1 开始（civil_from_days）。
fn civil_from_days(z: u32) -> (u32, u32, u32) {
    let z = z + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // 微秒时间戳（UTC）→ 秒 → 加时区偏移
    let sec = (rtc_us() / 1_000_000) as u32 + TZ_OFFSET_SEC;
    let days = sec / 86400;
    let tod = sec % 86400;
    let hour = tod / 3600;
    let minute = (tod % 3600) / 60;
    let second = tod % 60;
    let (year, month, day) = civil_from_days(days);

    put_pad(year % 100, 2);
    putchar(b'-');
    put_pad(month, 2);
    putchar(b'-');
    put_pad(day, 2);
    putchar(b' ');
    put_pad(hour, 2);
    putchar(b':');
    put_pad(minute, 2);
    putchar(b':');
    put_pad(second, 2);
    putchar(b'\n');

    if year < 2020 || month == 0 || month > 12 || day == 0 || day > 31
        || hour > 23 || minute > 59 || second > 59
    {
        halt(1);
    }
    halt(0);
}
