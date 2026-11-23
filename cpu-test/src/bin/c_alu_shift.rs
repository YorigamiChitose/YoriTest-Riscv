#![no_std]
#![no_main]

use cpu_test::Checks;

extern "C" {
    fn c_slli(x: u32) -> u32;
    fn c_srli(x: u32) -> u32;
    fn c_srai(x: u32) -> u32;
    fn c_andi(x: u32) -> u32;
}

core::arch::global_asm!(
    r#"
    .option norvc
.section .text.c_alu_shift, "ax", @progbits

.globl c_slli
c_slli:
    .option push
    .option rvc
    c.slli a0, 3
    .option pop
    ret

.globl c_srli
c_srli:
    .option push
    .option rvc
    c.srli a0, 6
    .option pop
    ret

.globl c_srai
c_srai:
    .option push
    .option rvc
    c.srai a0, 7
    .option pop
    ret

.globl c_andi
c_andi:
    .option push
    .option rvc
    c.andi a0, 15
    .option pop
    ret
"#
);

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut c = Checks::new();
    unsafe {
        c.check(1, c_slli(3) == 24);
        c.check(2, c_srli(0x8000_0000) == 0x0200_0000);
        c.check(3, c_srai(0x8000_0000) == 0xFF00_0000);
        c.check(4, c_andi(0xFF) == 0x0F);
    }
    c.finish();
}
