//! Tests that malloc was overriden globally with mimalloc
#![cfg(feature = "override")]

use libc::{c_void, size_t};
use mimalloc_sys::*;

#[cfg(not(teature = "override"))]
extern "C" {
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn malloc_usable_size(p: *mut c_void) -> size_t;
}

const PAGE_SIZE: usize = 4_096;
const HUGE: usize = PAGE_SIZE * 64 + 1;
const LARGE: usize = PAGE_SIZE * 3 + 1;
const MEDIUM: usize = PAGE_SIZE / 2 + 1;
const SMALL: usize = 3;
const ALLOCS: [usize; 4] = [SMALL, MEDIUM, LARGE, HUGE];

#[test]
fn malloc_mi_free() {
    unsafe {
        ALLOCS.iter().for_each(|&s| {
            let ptr = malloc(s);
            (ptr as *mut u8).write_volatile(42);
            let _ = mi_usable_size(ptr);
            mi_free(ptr);
        });
    }
}

#[test]
fn mi_malloc_free() {
    unsafe {
        ALLOCS.iter().for_each(|&s| {
            let ptr = mi_malloc(s);
            (ptr as *mut u8).write_volatile(42);
            let _ = malloc_usable_size(ptr);
            free(ptr);
        });
    }
}

#[test]
fn malloc_mi_realloc_free() {
    unsafe {
        ALLOCS.iter().for_each(|&s| {
            let ptr = malloc(s);
            (ptr as *mut u8).write_volatile(42);
            let ptr = mi_realloc(ptr, s * 7);
            (ptr as *mut u8).write_volatile(42);
            let _ = malloc_usable_size(ptr);
            free(ptr);
        });
    }
}

#[test]
fn mi_malloc_realloc_mi_free() {
    unsafe {
        ALLOCS.iter().for_each(|&s| {
            let ptr = mi_malloc(s);
            (ptr as *mut u8).write_volatile(42);
            let ptr = realloc(ptr, s * 7);
            (ptr as *mut u8).write_volatile(42);
            let _ = mi_usable_size(ptr);
            mi_free(ptr);
        });
    }
}

/*
#[test]
#[cfg(feature = "override")]
fn overriden_fn_ptrs() {
    assert_eq!(mi_malloc as usize, malloc as usize);
    assert_eq!(mi_calloc as usize, calloc as usize);
    assert_eq!(mi_realloc as usize, realloc as usize);
    assert_eq!(mi_free as usize, free as usize);
    assert_eq!(mi_posix_memalign as usize, posix_memalign as usize);
    assert_eq!(mi_aligned_alloc as usize, aligned_alloc as usize);
    assert_eq!(mi_malloc_size as usize, malloc_size as usize);
    assert_eq!(mi_malloc_usable_size as usize, malloc_usable_size as usize);
}
*/
