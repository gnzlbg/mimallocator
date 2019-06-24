//! Generate C FFI binding tests

extern crate ctest;

fn main() {
    let mut cfg = ctest::TestGenerator::new();
    cfg.header("mimalloc.h")
        .include(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../mimalloc-sys/mimalloc/include"
        ))
        .fn_cname(|rust, link_name| link_name.unwrap_or(rust).to_string())
        .skip_signededness(|c| c.ends_with("_t"));

    cfg.generate("../mimalloc-sys/src/lib.rs", "all.rs");
}
