#![no_std]
use core::alloc::{GlobalAlloc, Layout};

#[cfg(all(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "asmjs",
    target_arch = "wasm32"
)))]
const MAX_ALIGN_T: usize = 8;
#[cfg(all(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "mips64",
    target_arch = "s390x",
    target_arch = "sparc64"
)))]
const MAX_ALIGN_T: usize = 16;

#[derive(Copy, Clone, Default, Debug)]
pub struct Mimalloc;

fn fundamental_alignment(size: usize, align: usize) -> bool {
    align <= MAX_ALIGN_T && align <= size
}

unsafe impl GlobalAlloc for Mimalloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let ptr = if fundamental_alignment(size, align) {
            mimalloc_sys::mi_malloc(size as _)
        } else {
            mimalloc_sys::mi_malloc_aligned(size as _, align as _)
        };

        ptr as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let ptr = if fundamental_alignment(size, align) {
            mimalloc_sys::mi_zalloc(size as _)
        } else {
            mimalloc_sys::mi_zalloc_aligned(size as _, align as _)
        };

        ptr as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mimalloc_sys::mi_free(ptr as *mut _);
    }

    #[inline]
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let ptr = if fundamental_alignment(size, align) {
            mimalloc_sys::mi_realloc(ptr as *mut _, new_size)
        } else {
            mimalloc_sys::mi_realloc_aligned(ptr as *mut _, new_size, align)
        };

        ptr as *mut u8
    }
}
