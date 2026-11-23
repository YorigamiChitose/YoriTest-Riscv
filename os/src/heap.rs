use core::alloc::{GlobalAlloc, Layout};

const HEAP_SIZE: usize = 64 * 1024;

#[repr(align(16))]
struct Heap([u8; HEAP_SIZE]);

static mut HEAP: Heap = Heap([0; HEAP_SIZE]);
static mut NEXT: usize = 0;

struct Bump;

unsafe impl GlobalAlloc for Bump {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let base = core::ptr::addr_of_mut!(HEAP.0) as *mut u8;
        let align = layout.align();
        let start = (NEXT + align - 1) & !(align - 1);
        let end = match start.checked_add(layout.size()) {
            Some(v) => v,
            None => return core::ptr::null_mut(),
        };
        if end > HEAP_SIZE {
            return core::ptr::null_mut();
        }
        NEXT = end;
        base.add(start)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOC: Bump = Bump;

pub fn used() -> usize {
    unsafe { NEXT }
}
