#![no_std]
#![no_main]

use cpu_test::Checks;

extern "C" {
    fn c_lw(ptr: u32) -> u32;
    fn c_sw(ptr: u32, val: u32);
    fn c_lwsp(val: u32) -> u32;
    fn c_swsp(val: u32) -> u32;
}

core::arch::global_asm!(
    r#"
    .option norvc
.section .text.c_mem, "ax", @progbits

.globl c_lw
c_lw:
    mv a3, a0
    .option push
    .option rvc
    c.lw a0, 12(a3)
    .option pop
    ret

.globl c_sw
c_sw:
    mv a3, a0
    .option push
    .option rvc
    c.sw a1, 12(a3)
    .option pop
    ret

.globl c_lwsp
c_lwsp:
    addi sp, sp, -32
    sw a0, 12(sp)
    .option push
    .option rvc
    c.lwsp a0, 12(sp)
    .option pop
    addi sp, sp, 32
    ret

.globl c_swsp
c_swsp:
    addi sp, sp, -32
    sw zero, 12(sp)
    .option push
    .option rvc
    c.swsp a0, 12(sp)
    .option pop
    lw a0, 12(sp)
    addi sp, sp, 32
    ret
"#
);

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut buf = [0u32; 8];
    buf[3] = 0xDEAD_BEEF;

    let mut c = Checks::new();
    unsafe {
        c.check(1, c_lw(buf.as_ptr() as usize as u32) == 0xDEAD_BEEF);
        c_sw(buf.as_mut_ptr() as usize as u32, 0x1234_5678);
        c.check(2, buf[3] == 0x1234_5678);
        c.check(3, c_lwsp(0xABCD_1234) == 0xABCD_1234);
        c.check(4, c_swsp(0x5555_AAAA) == 0x5555_AAAA);
    }

    c.finish();
}
