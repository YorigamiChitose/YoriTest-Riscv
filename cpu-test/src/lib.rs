//! cpu-test 测试基座：断言、退出封装与常用测试宏。
//!
//! 每个指令测试 bin 通过 `check(cond, id)` 断言结果，全部通过后调用 `finish()`。
//! 失败时立即以 `FAIL(id)` 退出（id ∈ [1,127]，0 保留给 PASS），便于定位到具体用例。

#![no_std]

/// 断言：条件为假则立即以 FAIL(id) 退出（永不返回）。
///
/// `id` 为用例编号，需在 [1,127] 内（halt 失败码仅 7 位）。
#[inline(always)]
pub fn check(cond: bool, id: u32) {
    if !cond {
        am::halt(id & 0x7f);
    }
}

/// 结束测试：全部用例通过，halt(0) = PASS。
#[inline(always)]
pub fn finish() -> ! {
    am::halt(0);
}

/// 测试双寄存器源指令：`rd = op(rs1, rs2)`，如 add/sub/and/or/xor/slt/sll/srl/sra/mul。
///
/// `$insn` 为完整指令串，操作数占位符用 `{r}`/`{a}`/`{b}`。
#[macro_export]
macro_rules! t_rr {
    ($id:expr, $insn:literal, $a:expr, $b:expr, $e:expr) => {{
        let a: u32 = $a;
        let b: u32 = $b;
        let r: u32;
        unsafe {
            core::arch::asm!($insn, a = in(reg) a, b = in(reg) b, r = out(reg) r);
        }
        $crate::check(r == $e, $id);
    }};
}

/// 测试单寄存器源 + 立即数指令（立即数写死在指令串里）：
/// `rd = op(rs1, imm)`，如 addi/andi/ori/xori/slti/sltiu/slli/srli/srai。
#[macro_export]
macro_rules! t_ri {
    ($id:expr, $insn:literal, $a:expr, $e:expr) => {{
        let a: u32 = $a;
        let r: u32;
        unsafe {
            core::arch::asm!($insn, a = in(reg) a, r = out(reg) r);
        }
        $crate::check(r == $e, $id);
    }};
}
