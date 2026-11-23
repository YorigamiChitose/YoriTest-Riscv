#![no_std]
#![no_main]

use cpu_test::Checks;

extern "C" {
    fn c_beqz(x: u32) -> u32;
    fn c_bnez(x: u32) -> u32;
}

core::arch::global_asm!(
    r#"
    .option norvc
.section .text.c_beqz, "ax", @progbits
.globl c_beqz
c_beqz:
    mv a1, a0
    li a0, 2
1:
    .option push
    .option rvc
    c.beqz a1, 3f
    .option pop
    j 4f
    .org 0x10
3:
    li a0, 1
    ret
4:
    ret

.section .text.c_bnez, "ax", @progbits
.globl c_bnez
c_bnez:
    mv a1, a0
    li a0, 2
1:
    .option push
    .option rvc
    c.bnez a1, 3f
    .option pop
    j 4f
    .org 0x10
3:
    li a0, 1
    ret
4:
    ret
"#
);

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut c = Checks::new();
    unsafe {
        c.check(1, c_beqz(0) == 1);
        c.check(2, c_beqz(7) == 2);
        c.check(3, c_bnez(7) == 1);
        c.check(4, c_bnez(0) == 2);
    }
    c.finish();
}
