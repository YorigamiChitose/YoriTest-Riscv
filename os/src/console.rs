pub fn print(s: &str) {
    for b in s.bytes() {
        am::putchar(b);
    }
}

pub fn print_u32(mut n: u32) {
    if n == 0 {
        am::putchar(b'0');
        return;
    }
    let mut buf = [0u8; 10];
    let mut i = buf.len();
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    for b in &buf[i..] {
        am::putchar(*b);
    }
}
