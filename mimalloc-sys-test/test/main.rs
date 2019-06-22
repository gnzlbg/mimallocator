#![allow(bad_style, improper_ctypes, dead_code, unused_imports)]

extern crate mimalloc_sys;
extern crate libc;

use mimalloc_sys::*;
use libc::{size_t, c_void};

include!(concat!(env!("OUT_DIR"), "/all.rs"));
