#![no_std]
use core::alloc::{GlobalAlloc, Layout};

// Copied from https://github.com/rust-lang/rust/blob/master/src/libstd/sys_common/alloc.rs
#[cfg(all(any(target_arch = "x86",
              target_arch = "arm",
              target_arch = "mips",
              target_arch = "powerpc",
              target_arch = "powerpc64",
              target_arch = "asmjs",
              target_arch = "wasm32")))]
const MIN_ALIGN: usize = 8;
#[cfg(all(any(target_arch = "x86_64",
              target_arch = "aarch64",
              target_arch = "mips64",
              target_arch = "s390x",
              target_arch = "sparc64")))]
const MIN_ALIGN: usize = 16;

#[derive(Copy, Clone, Default, Debug)]
pub struct Mimalloc;

unsafe impl GlobalAlloc for Mimalloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            mimalloc_sys::mi_malloc(layout.size() as _)
        } else {
            mimalloc_sys::mi_malloc_aligned(
                layout.size() as _,
                layout.align() as _,
            )
        };

        ptr as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            mimalloc_sys::mi_zalloc(layout.size() as _)
        } else {
            mimalloc_sys::mi_zalloc_aligned(
                layout.size() as _,
                layout.align() as _,
            )
        };

        ptr as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mimalloc_sys::mi_free(ptr as *mut _);
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let ptr = if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            mimalloc_sys::mi_realloc(ptr as *mut _, new_size)
        } else {
            mimalloc_sys::mi_realloc_aligned(
                ptr as *mut _,
                new_size,
                layout.align() as _,
            )
        };

        ptr as *mut u8
    }
}
