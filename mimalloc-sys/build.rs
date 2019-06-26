//! Builds and links mimalloc.  Emulates behavior of CMakeLists
// Based on flate2-rs's miniz-sys

use std::env;

fn main() {
    // Get environment variables:
    let target = env::var("TARGET").expect("TARGET was not set");
    let secure = env::var_os("CARGO_FEATURE_SECURE").is_some();
    let check_full = env::var_os("CARGO_FEATURE_CHECK_FULL").is_some();
    let stats = env::var_os("CARGO_FEATURE_STATS").is_some();

    // Build mimalloc
    let mut build = cc::Build::new();
    let compiler = build.get_compiler();

    build
        .file("mimalloc/src/static.c")
        .include("mimalloc/include")
        .include("mimalloc/src") // For .c includes from static.c and page.c
        .define("MI_STATIC_LIB", "1") // Tell mimalloc this is a static build
        .warnings(false);

    if !target.contains("darwin") && !target.contains("windows") {
        build.flag("-fvisibility=hidden");
    }

    if compiler.is_like_clang() || compiler.is_like_gnu() {
        build.flag("-ftls-model=initial-exec");
    }

    if compiler.is_like_gnu() {
        build.flag_if_supported("-Wno-invalid-memory-model");
    }

    // Setup features flags

    if secure {
        build.define("MI_SECURE", "2");
    }

    if check_full {
        build.define("MI_DEBUG", "3");
    }

    if !stats {
        build.define("NDEBUG", "1"); // Stats are on by default unless NDEBUG is defined
    }

    build.compile("mimalloc");

    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());
}
