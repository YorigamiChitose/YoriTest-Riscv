#![no_std]
#![no_main]

use cpu_test::{finish, t_ri, t_rr};

#[no_mangle]
pub extern "C" fn main() -> ! {

    t_rr!(1, "add {r}, {a}, {b}", 0x1234_5678, 0x1111_1111, 0x2345_6789);
    t_rr!(2, "add {r}, {a}, {b}", 0xFFFF_FFFF, 0x0000_0001, 0x0000_0000);
    t_rr!(3, "add {r}, {a}, {b}", 0x0000_0000, 0x0000_0000, 0x0000_0000);
    t_rr!(4, "add {r}, {a}, {b}", 0x8000_0000, 0x7FFF_FFFF, 0xFFFF_FFFF);

    t_rr!(5, "sub {r}, {a}, {b}", 0x0000_000A, 0x0000_0003, 0x0000_0007);
    t_rr!(6, "sub {r}, {a}, {b}", 0x0000_0000, 0x0000_0001, 0xFFFF_FFFF);
    t_rr!(7, "sub {r}, {a}, {b}", 0x1234_5678, 0x1234_5678, 0x0000_0000);
    t_rr!(8, "sub {r}, {a}, {b}", 0x0000_0001, 0x8000_0000, 0x8000_0001);

    t_ri!(9, "addi {r}, {a}, 0", 0x1234_5678, 0x1234_5678);
    t_ri!(10, "addi {r}, {a}, -1", 0x0000_0000, 0xFFFF_FFFF);
    t_ri!(11, "addi {r}, {a}, 2047", 0x0000_0001, 0x0000_0800);
    t_ri!(12, "addi {r}, {a}, -2048", 0x0000_0000, 0xFFFF_F800);

    finish();
}
