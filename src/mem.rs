use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn c_alloc(size: usize) -> *mut u8;
    fn kfree(ptr: *mut u8);
}

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        c_alloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        kfree(ptr);
    }
}

#[cfg(not(test))]
#[alloc_error_handler]
pub fn alloc_error_handler(_: Layout) -> ! {
    loop {}
}