#![no_std]
#![no_main]

use cpu_test::{check, finish};

fn matmul(a: &[[u32; 4]; 4], b: &[[u32; 4]; 4]) -> [[u32; 4]; 4] {
    let mut c = [[0u32; 4]; 4];
    let mut i = 0usize;
    while i < 4 {
        let mut j = 0usize;
        while j < 4 {
            let mut s = 0u32;
            let mut k = 0usize;
            while k < 4 {
                s += a[i][k] * b[k][j];
                k += 1;
            }
            c[i][j] = s;
            j += 1;
        }
        i += 1;
    }
    c
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let a = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ];
    let ident = [
        [1, 0, 0, 0],
        [0, 1, 0, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 1],
    ];

    let c = matmul(&a, &ident);
    check(c == a, 1);

    let d = matmul(&a, &a);
    check(d[0][0] == 90, 2);
    check(d[0][3] == 120, 3);
    check(d[3][0] == 426, 4);
    check(d[3][3] == 600, 5);

    finish();
}
