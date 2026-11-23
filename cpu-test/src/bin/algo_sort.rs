#![no_std]
#![no_main]

use cpu_test::{check, finish};

fn bubble_sort(a: &mut [u32; 8]) {
    let n = a.len();
    let mut i = 0usize;
    while i < n {
        let mut j = 0usize;
        while j < n - 1 - i {
            if a[j] > a[j + 1] {
                let t = a[j];
                a[j] = a[j + 1];
                a[j + 1] = t;
            }
            j += 1;
        }
        i += 1;
    }
}

fn is_sorted(a: &[u32; 8]) -> bool {
    let mut i = 1usize;
    while i < a.len() {
        if a[i - 1] > a[i] {
            return false;
        }
        i += 1;
    }
    true
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut a: [u32; 8] = [5, 2, 8, 1, 9, 3, 7, 4];
    bubble_sort(&mut a);
    check(is_sorted(&a), 1);
    check(a == [1, 2, 3, 4, 5, 7, 8, 9], 2);

    let mut b: [u32; 8] = [9, 8, 7, 6, 5, 4, 3, 2];
    bubble_sort(&mut b);
    check(is_sorted(&b), 3);
    check(b[0] == 2 && b[7] == 9, 4);

    let mut c: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    bubble_sort(&mut c);
    check(c == [1, 2, 3, 4, 5, 6, 7, 8], 5);

    finish();
}
