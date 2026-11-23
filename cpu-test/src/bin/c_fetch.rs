#![no_std]
#![no_main]

use cpu_test::Checks;

extern "C" {
    fn c_fetch_odd() -> u32;
    fn c_fetch_jodd() -> u32;
    fn c_nop(x: u32) -> u32;
}

core::arch::global_asm!(
    r#"
    .option norvc
.section .text.c_fetch_odd, "ax", @progbits
.balign 4
.globl c_fetch_odd
c_fetch_odd:
    .option push
    .option rvc
    c.nop
    .option pop
    .org 2
    .option push
    .option rvc
    c.li a0, 0x11
    .option pop
    .org 4
    .option push
    .option rvc
    c.li a1, 0x1e
    .option pop
    .org 6
    lui a2, 0x12345
    add a0, a0, a1
    add a0, a0, a2
    ret

.section .text.c_fetch_jodd, "ax", @progbits
.balign 4
.globl c_fetch_jodd
c_fetch_jodd:
    .option push
    .option rvc
    c.j 1f
    .option pop
    .org 2
1:
    .option push
    .option rvc
    c.li a0, 0x1f
    c.nop
    .option pop
    lui a1, 0xABC
    add a0, a0, a1
    ret

.section .text.c_nop, "ax", @progbits
.globl c_nop
c_nop:
    .option push
    .option rvc
    c.nop
    .option pop
    ret
"#
);

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut c = Checks::new();
    unsafe {
        c.check(1, c_fetch_odd() == 0x11 + 0x1e + 0x1234_5000);
        c.check(2, c_fetch_jodd() == 0x1f + 0x00AB_C000);
        c.check(3, c_nop(0x1234) == 0x1234);
    }
    c.finish();
}
