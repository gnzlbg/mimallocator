//! Generate C FFI binding tests

extern crate ctest;

use std::{env, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var_os("DEP_MIMALLOC_ROOT").unwrap());
    eprintln!("ROOT={:?}", root);
    let mut cfg = ctest::TestGenerator::new();
    cfg.header("mimalloc.h")
        .include(root.join("lib").join("mimalloc-1.0").join("include"))
        .fn_cname(|rust, link_name| link_name.unwrap_or(rust).to_string())
        .skip_signededness(|c| c.ends_with("_t"));

    cfg.generate("../mimalloc-sys/src/lib.rs", "all.rs");
}
