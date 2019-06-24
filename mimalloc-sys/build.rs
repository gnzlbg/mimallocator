//! Builds and links mimalloc.  Emulates behavior of CMakeLists
// Based on flate2-rs's miniz-sys

use std::env;

fn main() {
    // Get environment variables:
    let target = env::var("TARGET").expect("TARGET was not set");

    // Build mimalloc
    let mut build = cc::Build::new();

    build
        .file("mimalloc/src/static.c")
        .include("mimalloc/include")
        .include("mimalloc/src") // For .c includes from static.c and page.c
        .define("MI_STATIC_LIB", "1") // Tell mimalloc this is a static build
        .warnings(false);

    if !target.contains("darwin") && !target.contains("windows") {
        build.flag("-fvisibility=hidden");
    }

    if target.contains("gnu") {
        build.flag("-Wno-invalid-memory-model");
    }

    //
    if !target.contains("windows-msvc") {
        build.flag("-ftls-model=initial-exec");
    }

    // Setup features flags

    if cfg!(feature = "secure") {
        build.define("MI_SECURE", "2");
    }

    if cfg!(feature = "check_full") {
        build.define("MI_DEBUG", "3");
    }

    if !cfg!(feature = "stats") {
        build.define("NDEBUG", "1"); // Stats are on by default unless NDEBUG is defined
    }

    build.compile("mimalloc");

    println!("cargo:root={}", env::var("OUT_DIR").unwrap());
}
