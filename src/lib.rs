#![no_std]
use core::alloc::{GlobalAlloc, Layout};

#[derive(Copy, Clone, Default, Debug)]
pub struct Mimalloc;

unsafe impl GlobalAlloc for Mimalloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = mimalloc_sys::mi_malloc_aligned(
            layout.size() as _,
            layout.align() as _,
        );
        ptr as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mimalloc_sys::mi_free(ptr as *mut _);
    }
}
