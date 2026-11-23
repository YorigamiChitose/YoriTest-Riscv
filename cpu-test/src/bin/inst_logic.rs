#![no_std]
#![no_main]

use cpu_test::{finish, t_ri, t_rr};

#[no_mangle]
pub extern "C" fn main() -> ! {

    t_rr!(1, "and {r}, {a}, {b}", 0xF0F0_F0F0, 0xFFFF_0000, 0xF0F0_0000);
    t_rr!(2, "and {r}, {a}, {b}", 0xFFFF_FFFF, 0x0000_0000, 0x0000_0000);

    t_ri!(3, "andi {r}, {a}, 0xFF", 0xABCD_EF12, 0x0000_0012);
    t_ri!(4, "andi {r}, {a}, -256", 0xFFFF_FFFF, 0xFFFF_FF00);

    t_rr!(5, "or {r}, {a}, {b}", 0xF0F0_F0F0, 0x0F0F_0F0F, 0xFFFF_FFFF);
    t_rr!(6, "or {r}, {a}, {b}", 0x0000_0000, 0x0000_0000, 0x0000_0000);

    t_ri!(7, "ori {r}, {a}, 0xFF", 0x1234_0000, 0x1234_00FF);

    t_rr!(8, "xor {r}, {a}, {b}", 0xFFFF_FFFF, 0x0F0F_0F0F, 0xF0F0_F0F0);
    t_rr!(9, "xor {r}, {a}, {b}", 0xAAAA_AAAA, 0xAAAA_AAAA, 0x0000_0000);

    t_ri!(10, "xori {r}, {a}, -1", 0x0000_0000, 0xFFFF_FFFF);

    finish();
}
