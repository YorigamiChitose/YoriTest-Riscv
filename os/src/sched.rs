use am::{Context, Event};

use crate::{console, time};

const MAX_TASKS: usize = 3;
const STACK_SIZE: usize = 4096;

const READY: u8 = 0;
const SLEEPING: u8 = 2;
const DEAD: u8 = 3;

#[repr(align(16))]
#[derive(Clone, Copy)]
struct Stack([u8; STACK_SIZE]);

static mut STACKS: [Stack; MAX_TASKS] = [Stack([0; STACK_SIZE]); MAX_TASKS];
static mut CTX: [*mut Context; MAX_TASKS] = [core::ptr::null_mut(); MAX_TASKS];
static mut STATE: [u8; MAX_TASKS] = [READY; MAX_TASKS];
static mut WAKE: [u64; MAX_TASKS] = [0; MAX_TASKS];
static mut CURRENT: usize = MAX_TASKS;
static mut EXIT_CODE: u32 = 0;

extern "C" fn task_a(_: usize) {
    let msg = b"A";
    if sys_write(msg.as_ptr(), 1) != 1 {
        sys_exit(10);
    }
    am::yield_now();
    if sys_write(msg.as_ptr(), 1) != 1 {
        sys_exit(10);
    }
    sys_exit(0);
}

extern "C" fn task_b(_: usize) {
    let t1 = sys_gettime();
    let t2 = sys_gettime();
    if t2 < t1 {
        sys_exit(11);
    }
    let msg = b"B";
    if sys_write(msg.as_ptr(), 1) != 1 {
        sys_exit(11);
    }
    #[cfg(not(feature = "difftest"))]
    sys_sleep(1000);
    if sys_write(msg.as_ptr(), 1) != 1 {
        sys_exit(11);
    }
    sys_exit(0);
}

extern "C" fn task_c(_: usize) {
    let mut n: u32 = 0;
    let r = sys_taskinfo(&mut n);
    if r != MAX_TASKS as u32 || n != MAX_TASKS as u32 {
        sys_exit(12);
    }
    let msg = b"C";
    if sys_write(msg.as_ptr(), 1) != 1 {
        sys_exit(12);
    }
    sys_exit(0);
}

pub fn init_and_start() -> ! {
    unsafe {
        let stacks = core::ptr::addr_of_mut!(STACKS);
        CTX[0] = am::make_context(&mut (*stacks)[0].0, task_a, 0);
        CTX[1] = am::make_context(&mut (*stacks)[1].0, task_b, 0);
        CTX[2] = am::make_context(&mut (*stacks)[2].0, task_c, 0);
        for i in 0..MAX_TASKS {
            STATE[i] = READY;
        }
        CURRENT = MAX_TASKS;
        am::init_trap(schedule);
        am::yield_now();
    }
    am::halt(0)
}

extern "C" fn schedule(ev: Event, prev: *mut Context) -> *mut Context {
    unsafe {
        match ev {
            Event::Yield => {
                if CURRENT < MAX_TASKS {
                    STATE[CURRENT] = READY;
                }
                switch(prev)
            }
            Event::Syscall => handle_syscall(prev),
            Event::Error => am::halt(1),
        }
    }
}

fn switch(prev: *mut Context) -> *mut Context {
    unsafe {
        if CURRENT < MAX_TASKS {
            CTX[CURRENT] = prev;
        }
        let next = pick_next();
        CURRENT = next;
        CTX[next]
    }
}

fn pick_next() -> usize {
    unsafe {
        loop {
            let now = time::now_us();
            let mut sleeping = false;
            let mut earliest = u64::MAX;
            let start = if CURRENT < MAX_TASKS {
                (CURRENT + 1) % MAX_TASKS
            } else {
                0
            };
            for k in 0..MAX_TASKS {
                let i = (start + k) % MAX_TASKS;
                if STATE[i] == SLEEPING {
                    if WAKE[i] <= now {
                        STATE[i] = READY;
                    } else {
                        sleeping = true;
                        if WAKE[i] < earliest {
                            earliest = WAKE[i];
                        }
                    }
                }
                if STATE[i] == READY {
                    return i;
                }
            }
            if !sleeping {
                if EXIT_CODE == 0 {
                    console::print("\nos selftest pass\n");
                } else {
                    console::print("\nos selftest fail: ");
                    console::print_u32(EXIT_CODE);
                    console::print("\n");
                }
                am::halt(EXIT_CODE);
            }
            while time::now_us() < earliest {
                core::hint::spin_loop();
            }
        }
    }
}

fn handle_syscall(prev: *mut Context) -> *mut Context {
    unsafe {
        let a7 = (*prev).gpr[17];
        let a0 = (*prev).gpr[10];
        match a7 {
            0 => {
                let ptr = a0 as *const u8;
                let len = (*prev).gpr[11];
                for i in 0..len {
                    am::putchar(core::ptr::read_volatile(ptr.add(i)));
                }
                (*prev).gpr[10] = len;
                prev
            }
            1 => {
                if CURRENT < MAX_TASKS {
                    STATE[CURRENT] = READY;
                }
                switch(prev)
            }
            2 => {
                let code = a0 as u32;
                if code != 0 {
                    EXIT_CODE = code;
                }
                if CURRENT < MAX_TASKS {
                    STATE[CURRENT] = DEAD;
                }
                switch(prev)
            }
            3 => {
                (*prev).gpr[10] = time::now_us() as usize;
                prev
            }
            4 => {
                if CURRENT < MAX_TASKS {
                    WAKE[CURRENT] = time::now_us() + a0 as u64;
                    STATE[CURRENT] = SLEEPING;
                }
                switch(prev)
            }
            5 => {
                let p = a0 as *mut u32;
                if !p.is_null() {
                    core::ptr::write_volatile(p, MAX_TASKS as u32);
                }
                (*prev).gpr[10] = MAX_TASKS;
                prev
            }
            _ => {
                (*prev).gpr[10] = usize::MAX;
                prev
            }
        }
    }
}

fn sys_write(ptr: *const u8, len: usize) -> usize {
    let ret: usize;
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") ptr as usize,
            in("a1") len,
            in("a7") 0usize,
            lateout("a0") ret,
            options(nostack)
        );
    }
    ret
}

fn sys_gettime() -> u32 {
    let ret: u32;
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") 3usize,
            lateout("a0") ret,
            options(nostack)
        );
    }
    ret
}

fn sys_sleep(us: u64) {
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") us as usize,
            in("a7") 4usize,
            options(nostack)
        );
    }
}

fn sys_taskinfo(p: *mut u32) -> u32 {
    let ret: u32;
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") p as usize,
            in("a7") 5usize,
            lateout("a0") ret,
            options(nostack)
        );
    }
    ret
}

fn sys_exit(code: u32) -> ! {
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") code as usize,
            in("a7") 2usize,
            options(noreturn, nostack)
        );
    }
}
