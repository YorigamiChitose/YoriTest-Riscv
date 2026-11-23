#![no_std]
#![no_main]

use cpu_test::Checks;

extern "C" {
    fn c_j_far() -> u32;
    fn c_jal_far() -> u32;
}

core::arch::global_asm!(
    r#"
    .option norvc
.section .text.c_j_far, "ax", @progbits
.globl c_j_far
c_j_far:
    li a0, 2
1:
    .option push
    .option rvc
    c.j 3f
    .option pop
    .org 0x44
2:
    ret
    .org 0x84
3:
    li a0, 1
    ret

.section .text.c_jal_far, "ax", @progbits
.globl c_jal_far
c_jal_far:
    addi sp, sp, -16
    sw ra, 12(sp)
    li a0, 2
1:
    .option push
    .option rvc
    c.jal 3f
    .option pop
    .org 0x4c
2:
    li a0, 2
    j 4f
    .org 0x8c
3:
    la a1, 1b
    addi a1, a1, 2
    sub a0, ra, a1
4:
    lw ra, 12(sp)
    addi sp, sp, 16
    ret
"#
);

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut c = Checks::new();
    unsafe {
        c.check(1, c_j_far() == 1);
        c.check(2, c_jal_far() == 0);
    }
    c.finish();
}
