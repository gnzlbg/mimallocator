#![allow(bad_style, improper_ctypes, dead_code, unused_imports, unused_macros)]

extern crate libc;
extern crate mimalloc_sys;

use libc::{c_void, size_t};
use mimalloc_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
