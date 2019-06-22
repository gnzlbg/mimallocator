use mimallocator::Mimalloc;
use std::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static A: Mimalloc = Mimalloc;

#[test]
fn smoke_vec() {
    let mut a = Vec::new();
    a.push(3);
}

/// https://github.com/rust-lang/rust/issues/45955
#[test]
fn overaligned() {
    let size = 8;
    let align = 16; // greater than size
    let iterations = 100;
    unsafe {
        let pointers: Vec<_> = (0..iterations)
            .map(|_| {
                let ptr = Mimalloc.alloc(Layout::from_size_align(size, align).unwrap());
                assert!(!ptr.is_null());
                ptr
            })
            .collect();
        for &ptr in &pointers {
            assert_eq!(
                (ptr as usize) % align,
                0,
                "Got a pointer less aligned than requested"
            )
        }

        // Clean up
        for &ptr in &pointers {
            Mimalloc.dealloc(ptr, Layout::from_size_align(size, align).unwrap())
        }
    }
}

#[test]
fn smoke_ffi() {
    unsafe {
        let ptr = mimalloc_sys::mi_malloc(4);
        *(ptr as *mut u32) = 0xDECADE;
        assert_eq!(*(ptr as *mut u32), 0xDECADE);
        mimalloc_sys::mi_free(ptr);
    }
}
