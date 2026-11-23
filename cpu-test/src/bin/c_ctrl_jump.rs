#![no_std]
#![no_main]

use cpu_test::Checks;

extern "C" {
    fn c_j_small() -> u32;
    fn c_jal_small() -> u32;
    fn c_jr(target: u32) -> u32;
    fn c_jalr(target: u32) -> u32;
}

extern "C" fn jr_target() -> u32 {
    0x1234_5678
}

extern "C" fn jalr_target() -> u32 {
    0x9ABC_DEF0
}

core::arch::global_asm!(
    r#"
    .option norvc
.section .text.c_j_small, "ax", @progbits
.globl c_j_small
c_j_small:
    li a1, 0
1:
    .option push
    .option rvc
    c.j 3f
    .option pop
    .option push
    .option rvc
    c.li a1, 1
    .option pop
3:
    mv a0, a1
    ret

.section .text.c_jal_small, "ax", @progbits
.globl c_jal_small
c_jal_small:
    addi sp, sp, -16
    sw ra, 12(sp)
    li a1, 0
1:
    .option push
    .option rvc
    c.jal 3f
    .option pop
    .option push
    .option rvc
    c.li a1, 1
    .option pop
3:
    la a2, 1b
    addi a2, a2, 2
    sub a0, ra, a2
    or a0, a0, a1
    lw ra, 12(sp)
    addi sp, sp, 16
    ret

.section .text.c_jr, "ax", @progbits
.globl c_jr
c_jr:
    .option push
    .option rvc
    c.jr a0
    .option pop

.section .text.c_jalr, "ax", @progbits
.globl c_jalr
c_jalr:
    addi sp, sp, -16
    sw ra, 12(sp)
1:
    .option push
    .option rvc
    c.jalr a0
    .option pop
    la a1, 1b
    addi a1, a1, 2
    sub a0, ra, a1
    lw ra, 12(sp)
    addi sp, sp, 16
    ret
"#
);

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut c = Checks::new();
    unsafe {
        c.check(1, c_j_small() == 0);
        c.check(2, c_jal_small() == 0);
        c.check(3, c_jr(jr_target as *const () as usize as u32) == 0x1234_5678);
        c.check(4, c_jalr(jalr_target as *const () as usize as u32) == 0);
    }
    c.finish();
}
