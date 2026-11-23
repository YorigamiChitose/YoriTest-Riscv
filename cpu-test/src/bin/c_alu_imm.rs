#![no_std]
#![no_main]

use cpu_test::Checks;

extern "C" {
    fn c_addi_pos(x: u32) -> u32;
    fn c_addi_neg(x: u32) -> u32;
    fn c_li_neg() -> u32;
    fn c_lui() -> u32;
    fn c_addi4spn() -> u32;
    fn c_addi16sp() -> u32;
}

core::arch::global_asm!(
    r#"
    .option norvc
.section .text.c_alu_imm, "ax", @progbits

.globl c_addi_pos
c_addi_pos:
    .option push
    .option rvc
    c.addi a0, 7
    .option pop
    ret

.globl c_addi_neg
c_addi_neg:
    .option push
    .option rvc
    c.addi a0, -3
    .option pop
    ret

.globl c_li_neg
c_li_neg:
    .option push
    .option rvc
    c.li a0, -3
    .option pop
    ret

.globl c_lui
c_lui:
    .option push
    .option rvc
    c.lui a0, 0x10
    .option pop
    ret

.globl c_addi4spn
c_addi4spn:
    mv a1, sp
    .option push
    .option rvc
    c.addi4spn a0, sp, 64
    .option pop
    sub a0, a0, a1
    ret

.globl c_addi16sp
c_addi16sp:
    mv a1, sp
    .option push
    .option rvc
    c.addi16sp sp, 32
    .option pop
    sub a0, sp, a1
    mv sp, a1
    ret
"#
);

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut c = Checks::new();
    unsafe {
        c.check(1, c_addi_pos(5) == 12);
        c.check(2, c_addi_neg(10) == 7);
        c.check(3, c_li_neg() == 0xFFFF_FFFD);
        c.check(4, c_lui() == 0x0001_0000);
        c.check(5, c_addi4spn() == 64);
        c.check(6, c_addi16sp() == 32);
    }
    c.finish();
}
