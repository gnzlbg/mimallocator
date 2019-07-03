//! Raw FFI wrapper over the mimalloc memory allocator
#![no_std]
use libc::{c_void, size_t, FILE};

extern "C" {
    // Standard malloc interface

    pub fn mi_malloc(size: size_t) -> *mut c_void;
    pub fn mi_calloc(count: size_t, size: size_t) -> *mut c_void;
    pub fn mi_realloc(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_expand(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_free(p: *mut c_void);

    // Extended functionality

    pub fn mi_zalloc(size: size_t) -> *mut c_void;
    pub fn mi_usable_size(p: *mut c_void) -> size_t;
    pub fn mi_good_size(size: size_t) -> size_t;

    pub fn mi_collect(force: bool);
    pub fn mi_stats_print(out: *mut FILE);
    pub fn mi_stats_reset();

    // Aligned allocation

    pub fn mi_malloc_aligned(size: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_zalloc_aligned(size: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_calloc_aligned(
        count: size_t,
        size: size_t,
        alignment: size_t,
    ) -> *mut c_void;
    pub fn mi_realloc_aligned(
        p: *mut c_void,
        newsize: size_t,
        alignment: size_t,
    ) -> *mut c_void;
}
