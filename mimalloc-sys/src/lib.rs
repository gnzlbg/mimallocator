//! Raw FFI wrapper over the mimalloc memory allocator
#![no_std]
#![allow(nonstandard_style)]
use libc::{
    c_char, c_int, c_long, c_uchar, c_ulonglong, c_ushort, c_void, c_double, size_t,
};

pub const MI_SMALL_WSIZE_MAX: size_t = 128;
pub const MI_SMALL_SIZE_MAX: size_t = MI_SMALL_WSIZE_MAX * core::mem::size_of::<*mut c_void>();

extern "C" {
    // Standard malloc interface

    pub fn mi_malloc(size: size_t) -> *mut c_void;
    pub fn mi_calloc(count: size_t, size: size_t) -> *mut c_void;
    pub fn mi_realloc(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_expand(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_posix_memalign(
        ptr: *mut *mut c_void,
        alignment: size_t,
        size: size_t,
    ) -> c_int;
    pub fn mi_aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void;
    pub fn mi_free(p: *mut c_void);
    pub fn mi_malloc_size(p: *const c_void) -> size_t;
    pub fn mi_malloc_usable_size(p: *const c_void) -> size_t;
    pub fn mi_memalign(alignment: size_t, size: size_t) -> *mut c_void;

    pub fn mi_strdup(s: *const c_char) -> *mut c_char;
    pub fn mi_strndup(s: *const c_char, n: size_t) -> *mut c_char;
    pub fn mi_realpath(fname: *const c_char, resolved_name: *mut c_char) -> *mut c_char;

    // mimalloc implementations of non-standard or uncommon functions from various oses.

    pub fn mi_cfree(p: *mut c_void);
    pub fn mi__expand(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_valloc(size: size_t) -> *mut c_void;
    pub fn mi_pvalloc(size: size_t) -> *mut c_void;

    pub fn mi_reallocarray(
        p: *mut c_void,
        count: size_t,
        size: size_t,
    ) -> *mut c_void;

    pub fn mi_aligned_recalloc(
        p: *mut c_void,
        newcount: size_t,
        size: size_t,
        alignment: size_t,
    ) -> *mut c_void;

    pub fn mi_aligned_offset_recalloc(
        p: *mut c_void,
        newcount: size_t,
        size: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    pub fn mi_wcsdup(s: *const c_ushort) -> *mut c_ushort;
    pub fn mi_mbsdup(s: *const c_uchar) -> *mut c_uchar;

    pub fn mi_dupenv_s(
        buf: *mut *mut c_char,
        size: *mut size_t,
        name: *const c_char,
    ) -> c_int;
    pub fn mi_wdupenv_s(
        buf: *mut *mut c_ushort,
        size: *mut size_t,
        name: *const c_ushort,
    ) -> c_int;

    pub fn mi_free_size(p: *mut c_void, size: size_t);
    pub fn mi_free_size_aligned(
        p: *mut c_void,
        size: size_t,
        alignment: size_t,
    );
    pub fn mi_free_aligned(p: *mut c_void, alignment: size_t);

    // FIXME: Don't expose these, These should be `extern "C unwind"` or
    // something, since they throw std::bad_alloc on failure.

    // pub fn mi_new(n: size_t) -> *mut c_void;
    // pub fn mi_new_aligned(n: size_t, alignment: size_t) -> *mut c_void;

    pub fn mi_new_nothrow(n: size_t) -> *mut c_void;
    pub fn mi_new_aligned_nothrow(n: size_t, alignment: size_t)
        -> *mut c_void;

    // Extended functionality

    pub fn mi_zalloc(size: size_t) -> *mut c_void;
    pub fn mi_malloc_small(size: size_t) -> *mut c_void;
    pub fn mi_zalloc_small(size: size_t) -> *mut c_void;

    pub fn mi_mallocn(count: size_t, size: size_t) -> *mut c_void;
    pub fn mi_reallocn(
        p: *mut c_void,
        count: size_t,
        size: size_t,
    ) -> *mut c_void;
    pub fn mi_reallocf(p: *mut c_void, newsize: size_t) -> *mut c_void;

    pub fn mi_usable_size(p: *const c_void) -> size_t;
    pub fn mi_good_size(size: size_t) -> size_t;

    pub fn mi_collect(force: bool);
    pub fn mi_stats_print(out: mi_output_fun);
    pub fn mi_stats_reset();
    pub fn mi_stats_merge();

    pub fn mi_process_init();
    pub fn mi_thread_init();
    pub fn mi_thread_done();

    pub fn mi_thread_stats_print(out: mi_output_fun);
    pub fn mi_register_output(out: mi_output_fun);

    pub fn mi_register_deferred_free(deferred_free: mi_deferred_free_fun);
    pub fn mi_version() -> c_int;
    pub fn mi_check_owned(p: *const c_void) -> bool;
    pub fn mi_is_redirected() -> bool;

    // option
    pub fn mi_option_is_enabled(option: mi_option_t) -> bool;
    pub fn mi_option_enable(option: mi_option_t);
    pub fn mi_option_disable(option: mi_option_t);
    pub fn mi_option_set_enabled(option: mi_option_t, enable: bool);
    pub fn mi_option_set_enabled_default(option: mi_option_t, enable: bool);
    pub fn mi_option_get(option: mi_option_t) -> c_long;
    pub fn mi_option_set(option: mi_option_t, value: c_long);
    pub fn mi_option_set_default(option: mi_option_t, value: c_long);

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

    // Offset-aligned allocation (e.g. if the returned value is nonnull, then
    // `p.cast::<u8>().add(offset)` will be aligned to `alignment`).

    pub fn mi_malloc_aligned_at(
        size: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    pub fn mi_zalloc_aligned_at(
        size: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    pub fn mi_calloc_aligned_at(
        count: size_t,
        size: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    pub fn mi_realloc_aligned_at(
        p: *mut c_void,
        newsize: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    // heap api

    pub fn mi_heap_new() -> *mut mi_heap_t;
    pub fn mi_heap_delete(heap: *mut mi_heap_t);
    pub fn mi_heap_destroy(heap: *mut mi_heap_t);
    pub fn mi_heap_set_default(heap: *mut mi_heap_t) -> *mut mi_heap_t;
    pub fn mi_heap_get_default() -> *mut mi_heap_t;
    pub fn mi_heap_get_backing() -> *mut mi_heap_t;
    pub fn mi_heap_collect(heap: *mut mi_heap_t, force: bool);

    pub fn mi_heap_malloc(heap: *mut mi_heap_t, size: size_t) -> *mut c_void;

    pub fn mi_heap_zalloc(heap: *mut mi_heap_t, size: size_t) -> *mut c_void;

    pub fn mi_heap_calloc(
        heap: *mut mi_heap_t,
        count: size_t,
        size: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_mallocn(
        heap: *mut mi_heap_t,
        count: size_t,
        size: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_malloc_small(
        heap: *mut mi_heap_t,
        size: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_realloc(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_reallocn(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        count: size_t,
        size: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_reallocf(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_strdup(
        heap: *mut mi_heap_t,
        s: *const c_char,
    ) -> *mut c_char;

    pub fn mi_heap_strndup(
        heap: *mut mi_heap_t,
        s: *const c_char,
        n: size_t,
    ) -> *mut c_char;

    pub fn mi_heap_realpath(
        heap: *mut mi_heap_t,
        fname: *const c_char,
        resolved_name: *mut c_char,
    ) -> *mut c_char;

    pub fn mi_heap_malloc_aligned(
        heap: *mut mi_heap_t,
        size: size_t,
        alignment: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_malloc_aligned_at(
        heap: *mut mi_heap_t,
        size: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_zalloc_aligned(
        heap: *mut mi_heap_t,
        size: size_t,
        alignment: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_zalloc_aligned_at(
        heap: *mut mi_heap_t,
        size: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_calloc_aligned(
        heap: *mut mi_heap_t,
        count: size_t,
        size: size_t,
        alignment: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_calloc_aligned_at(
        heap: *mut mi_heap_t,
        count: size_t,
        size: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_realloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: size_t,
        alignment: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_realloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    pub fn mi_rezalloc(p: *mut c_void, newsize: size_t) -> *mut c_void;

    pub fn mi_recalloc(
        p: *mut c_void,
        newcount: size_t,
        size: size_t,
    ) -> *mut c_void;

    pub fn mi_rezalloc_aligned(
        p: *mut c_void,
        newsize: size_t,
        alignment: size_t,
    ) -> *mut c_void;

    pub fn mi_rezalloc_aligned_at(
        p: *mut c_void,
        newsize: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    pub fn mi_recalloc_aligned(
        p: *mut c_void,
        newcount: size_t,
        size: size_t,
        alignment: size_t,
    ) -> *mut c_void;

    pub fn mi_recalloc_aligned_at(
        p: *mut c_void,
        newcount: size_t,
        size: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_rezalloc(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_recalloc(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: size_t,
        size: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_rezalloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: size_t,
        alignment: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_rezalloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_recalloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: size_t,
        size: size_t,
        alignment: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_recalloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: size_t,
        size: size_t,
        alignment: size_t,
        offset: size_t,
    ) -> *mut c_void;

    pub fn mi_heap_contains_block(
        heap: *mut mi_heap_t,
        p: *const c_void,
    ) -> bool;

    pub fn mi_heap_check_owned(heap: *mut mi_heap_t, p: *const c_void)
        -> bool;

    pub fn mi_heap_visit_blocks(
        heap: *const mi_heap_t,
        visit_all_blocks: bool,
        visitor: mi_block_visit_fun,
        arg: *mut c_void,
    ) -> bool;

    pub fn mi_is_in_heap_region(p: *const c_void) -> bool;

    pub fn mi_reserve_huge_os_pages(
        pages: size_t,
        max_secs: c_double,
        pages_reserved: *mut size_t,
    ) -> c_int;
}

pub type mi_deferred_free_fun = Option<unsafe extern "C" fn(force: bool, heartbeat: c_ulonglong)>;
pub type mi_output_fun = Option<unsafe extern "C" fn(msg: *const c_char)>;

pub enum mi_heap_t {}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct mi_heap_area_s {
    pub blocks: *mut c_void,
    pub reserved: size_t,
    pub committed: size_t,
    pub used: size_t,
    pub block_size: size_t,
}

pub type mi_heap_area_t = mi_heap_area_s;
// Note: This is defined as having the mi_cdecl calling convention, which does
// nothing unless `_MSC_VER` is defined, in which case it's `__cdecl`. `extern "C"`
// and `extern "cdecl"` should be the same on windows (32 and 64 bit), and I've
// been reassured this is the case, but the docs for `extern "cdecl"` don't
// exactly spell this out very well, so I'll leave this comment here for
// anybody who notices the same issue.
pub type mi_block_visit_fun = Option<
    unsafe extern "C" fn(
        heap: *const mi_heap_t,
        area: *const mi_heap_area_t,
        block: *mut c_void,
        block_size: size_t,
        arg: *mut c_void,
    ) -> bool
>;

pub type mi_option_e = c_int;
pub type mi_option_t = mi_option_e;
pub const mi_option_show_errors: mi_option_t = 0;
pub const mi_option_show_stats: mi_option_t = 1;
pub const mi_option_verbose: mi_option_t = 2;
pub const mi_option_eager_commit: mi_option_t = 3;
pub const mi_option_eager_region_commit: mi_option_t = 4;
pub const mi_option_large_os_pages: mi_option_t = 5;
pub const mi_option_reserve_huge_os_pages: mi_option_t = 6;
pub const mi_option_segment_cache: mi_option_t = 7;
pub const mi_option_page_reset: mi_option_t = 8;
pub const mi_option_cache_reset: mi_option_t = 9;
pub const mi_option_reset_decommits: mi_option_t = 10;
pub const mi_option_eager_commit_delay: mi_option_t = 11;
pub const mi_option_segment_reset: mi_option_t = 12;
pub const mi_option_os_tag: mi_option_t = 13;


#[cfg(feature = "override")]
extern "C" {
    // When the override feature is enabled, we also expose the standard C API here.
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn calloc(count: size_t, size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn posix_memalign(ptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
    pub fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void;
    pub fn malloc_size(p: *mut c_void) -> size_t;
    pub fn malloc_usable_size(p: *mut c_void) -> size_t;

    pub fn cfree(p: *mut c_void);
    pub fn valloc(p: *mut c_void) -> *mut c_void;
    pub fn pvalloc(p: *mut c_void) -> *mut c_void;
    pub fn reallocarray(p: *mut c_void, count: size_t, size: size_t) -> *mut c_void;
    pub fn memalign(alignment: size_t, size: size_t) -> *mut c_void;
    // Note that the mimalloc-doc.h lists some others too (mostly platform
    // specific) but they seem to be overridden by crazy binary patching. It's
    // not clear how we should handle that at all.
}
