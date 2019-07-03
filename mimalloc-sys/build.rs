//! Builds and links mimalloc.  Emulates behavior of CMakeLists
// Based on flate2-rs's miniz-sys

use std::env;

fn main() {
    // Get environment variables:
    // let target = env::var("TARGET").expect("TARGET was not set");
    let secure = env::var_os("CARGO_FEATURE_SECURE").is_some();
    let check_full = env::var_os("CARGO_FEATURE_CHECK_FULL").is_some();
    let stats = env::var_os("CARGO_FEATURE_STATS").is_some();
    let override_ = env::var_os("CARGO_FEATURE_OVERRIDE").is_some();
    let warnings = env::var_os("MIMALLOC_SYS_ENABLE_WARNINGS").is_some();
    let profile = env::var("PROFILE").unwrap_or(String::new());

    // Build mimalloc
    let mut build = cc::Build::new();
    let compiler = build.get_compiler();
    let clang = compiler.is_like_clang();
    let gnu = compiler.is_like_gnu();
    // let msvc = compiler.is_like_msvc();

    build
        // mimalloc provides a static library:
        .file("mimalloc/src/static.c")
        // it imports includes:
        .include("mimalloc/include")
        // and other .c files from the src directory:
        .include("mimalloc/src")
        // this needs to be defined
        .define("MI_STATIC_LIB", None)
        .warnings(warnings);

    // mimalloc is a C11 library
    build.flag(if gnu || clang {
        "-std=gnu11"
    } else {
        "/std:c++11"
    });

    if clang || gnu {
        [
            "-Wall",
            "-Wextra",
            "-Wno-unknown-pragmas",
            "-ftls-model=initial-exec",
        ]
        .iter()
        .for_each(|f| {
            build.flag(f);
        });
        if gnu {
            ["-Wno-invalid-memory-model" /*"-fvisibility=hidden"*/]
                .iter()
                .for_each(|f| {
                    build.flag(f);
                });
        }
    }

    // Overrides malloc with mimalloc
    if override_ {
        build.define("MI_MALLOC_OVERRIDE", None);
    }

    // Enable debug and secure in for debug builds, or when the cargo flag is active:
    if check_full || profile == "debug" {
        build.define("MI_DEBUG", "3");
    }
    if secure || profile == "debug" {
        build.define("MI_SECURE", "2");
    }

    if profile == "release" {
        build.define("NDEBUG", None);
        // FIXME:
        // build.flag_if_supported("-flto");
    }

    if stats {
        build.define("MI_STAT", "2");
    } else {
        build.define("MI_STAT", "0");
    }

    build.compile("mimalloc");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=mimalloc");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());
}
