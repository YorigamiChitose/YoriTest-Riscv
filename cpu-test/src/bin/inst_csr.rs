//! CSR 指令测试：CSRRW/CSRRS/CSRRC/CSRRWI/CSRRSI/CSRRCI，
//! 目标 CSR 为 MTVEC(0x305)，并读 MSTATUS(0x300) 初始值。

#![no_std]
#![no_main]

use cpu_test::{check, finish};

/// 读 CSR（CSRRS rd, csr, x0），返回 CSR 值。
macro_rules! csr_get {
    ($csr:literal) => {{
        let r: u32;
        unsafe {
            core::arch::asm!(concat!("csrrs {r}, ", $csr, ", zero"), r = out(reg) r);
        }
        r
    }};
}

/// CSR 写测试：执行 `$insn`（用 `{v}` 作源），断言旧值==`$old`，
/// 再读回断言新值==`$new`。
macro_rules! csr_rw {
    ($id:expr, $insn:literal, $v:expr, $old:expr, $new:expr) => {{
        let v: u32 = $v;
        {
            let r: u32;
            unsafe {
                core::arch::asm!($insn, v = in(reg) v, r = out(reg) r);
            }
            cpu_test::check(r == $old, $id);
        }
        {
            let r: u32;
            unsafe {
                core::arch::asm!("csrrs {r}, 0x305, zero", r = out(reg) r);
            }
            cpu_test::check(r == $new, $id + 1);
        }
    }};
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // 读 MSTATUS 初始值 = 0x1800
    check(csr_get!("0x300") == 0x1800, 1);

    // 复位 MTVEC = 0
    unsafe {
        core::arch::asm!("csrrw zero, 0x305, zero");
    }

    // CSRRW：rd=旧值，csr=rs1
    csr_rw!(2, "csrrw {r}, 0x305, {v}", 0x1234, 0, 0x1234);

    // CSRRS：rd=旧值，csr=旧|rs1
    csr_rw!(4, "csrrs {r}, 0x305, {v}", 0x0F00, 0x1234, 0x1F34);

    // CSRRC：rd=旧值，csr=旧&~rs1
    csr_rw!(6, "csrrc {r}, 0x305, {v}", 0x0F00, 0x1F34, 0x1034);

    // CSRRWI：rd=旧值，csr=立即数
    {
        let r: u32;
        unsafe {
            core::arch::asm!("csrrwi {r}, 0x305, 0x10", r = out(reg) r);
        }
        check(r == 0x1034, 8);
        check(csr_get!("0x305") == 0x10, 9);
    }

    // CSRRSI：rd=旧值，csr=旧|立即数
    {
        let r: u32;
        unsafe {
            core::arch::asm!("csrrsi {r}, 0x305, 0x0E", r = out(reg) r);
        }
        check(r == 0x10, 10);
        check(csr_get!("0x305") == 0x1E, 11);
    }

    // CSRRCI：rd=旧值，csr=旧&~立即数
    {
        let r: u32;
        unsafe {
            core::arch::asm!("csrrci {r}, 0x305, 0x06", r = out(reg) r);
        }
        check(r == 0x1E, 12);
        check(csr_get!("0x305") == 0x18, 13);
    }

    finish();
}
