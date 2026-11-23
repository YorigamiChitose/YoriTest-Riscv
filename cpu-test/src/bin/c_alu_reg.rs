#![no_std]
#![no_main]

use cpu_test::Checks;

extern "C" {
    fn c_mv(x: u32, y: u32) -> u32;
    fn c_add(x: u32, y: u32) -> u32;
    fn c_sub(x: u32, y: u32) -> u32;
    fn c_xor(x: u32, y: u32) -> u32;
    fn c_or(x: u32, y: u32) -> u32;
    fn c_and(x: u32, y: u32) -> u32;
}

core::arch::global_asm!(
    r#"
    .option norvc
.section .text.c_alu_reg, "ax", @progbits

.globl c_mv
c_mv:
    .option push
    .option rvc
    c.mv a0, a1
    .option pop
    ret

.globl c_add
c_add:
    .option push
    .option rvc
    c.add a0, a1
    .option pop
    ret

.globl c_sub
c_sub:
    .option push
    .option rvc
    c.sub a0, a1
    .option pop
    ret

.globl c_xor
c_xor:
    .option push
    .option rvc
    c.xor a0, a1
    .option pop
    ret

.globl c_or
c_or:
    .option push
    .option rvc
    c.or a0, a1
    .option pop
    ret

.globl c_and
c_and:
    .option push
    .option rvc
    c.and a0, a1
    .option pop
    ret
"#
);

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut c = Checks::new();
    unsafe {
        c.check(1, c_mv(1, 0xCAFE) == 0xCAFE);
        c.check(2, c_add(10, 3) == 13);
        c.check(3, c_sub(10, 3) == 7);
        c.check(4, c_xor(0xF0, 0x3C) == 0xCC);
        c.check(5, c_or(0xF0, 0x0F) == 0xFF);
        c.check(6, c_and(0xF0, 0x3C) == 0x30);
    }
    c.finish();
}
