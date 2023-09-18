#![no_std]

use core::alloc::GlobalAlloc;

pub fn done() -> ! {
    loop {}
}

pub mod int {
    pub fn disable() {}
}

struct DummyAlloc;

#[global_allocator]
static DUMMYALLOC: DummyAlloc = DummyAlloc;

unsafe impl GlobalAlloc for DummyAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}
