#![no_std]
#![no_main]

use cpu_test::{finish, t_rr};

#[no_mangle]
pub extern "C" fn main() -> ! {

    t_rr!(1, "mul {r}, {a}, {b}", 3, 5, 15);
    t_rr!(2, "mul {r}, {a}, {b}", 0x0001_0000, 0x0001_0000, 0);
    t_rr!(3, "mul {r}, {a}, {b}", 0xFFFF_FFFF, 0xFFFF_FFFF, 1);
    t_rr!(4, "mul {r}, {a}, {b}", 0x8000_0000, 3, 0x8000_0000);
    t_rr!(5, "mul {r}, {a}, {b}", 0, 0xDEAD_BEEF, 0);

    t_rr!(6, "mulh {r}, {a}, {b}", 0xFFFF_FFFF, 0xFFFF_FFFF, 0);
    t_rr!(7, "mulh {r}, {a}, {b}", 0x8000_0000, 2, 0xFFFF_FFFF);
    t_rr!(8, "mulh {r}, {a}, {b}", 0x4000_0000, 4, 1);
    t_rr!(9, "mulh {r}, {a}, {b}", 0x7FFF_FFFF, 2, 0);

    t_rr!(10, "mulhu {r}, {a}, {b}", 0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFE);
    t_rr!(11, "mulhu {r}, {a}, {b}", 0x8000_0000, 2, 1);
    t_rr!(12, "mulhu {r}, {a}, {b}", 0x0000_0003, 0x0000_0005, 0);

    t_rr!(13, "mulhsu {r}, {a}, {b}", 0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF);
    t_rr!(14, "mulhsu {r}, {a}, {b}", 2, 0x8000_0000, 1);
    t_rr!(15, "mulhsu {r}, {a}, {b}", 0x8000_0000, 1, 0xFFFF_FFFF);

    finish();
}
