//! Builds and links mimalloc

macro_rules! info {
    ($($args:tt)*) => { println!($($args)*) }
}

use std::{env, fs, path::PathBuf};

fn main() {
    // Get environment variables:
    let target = env::var("TARGET").expect("TARGET was not set");
    let host = env::var("HOST").expect("HOST was not set");
    let profile = env::var("PROFILE").expect("PROFILE was not set");
    let num_jobs = env::var("NUM_JOBS").expect("NUM_JOBS was not set");
    let out_dir =
        PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR was not set"));
    let src_dir = env::current_dir().expect("failed to get current directory");
    let build_dir = out_dir.join("build");
    let mimalloc_src_dir = src_dir.join("mimalloc");
    let mimalloc_out_src_dir = build_dir.join("mimalloc");

    info!("TARGET={}", target.clone());
    info!("HOST={}", host.clone());
    info!("PROFILE={}", profile.clone());
    info!("NUM_JOBS={}", num_jobs.clone());
    info!("OUT_DIR={:?}", out_dir);
    info!("BUILD_DIR={:?}", build_dir);
    info!("SRC_DIR={:?}", src_dir);
    info!("mimalloc_src_dir={:?}", mimalloc_src_dir);
    info!("mimalloc_out_src_dir={:?}", mimalloc_out_src_dir);

    // Copy the mimalloc source code to the OUT_DIR:
    //
    // This ensures that building mimalloc-sys does not modify
    // the source directory.
    fs::create_dir_all(&build_dir).expect("failed to create build directory");
    if mimalloc_out_src_dir.exists() {
        fs::remove_dir_all(mimalloc_out_src_dir.clone())
            .expect("failed to remove mimalloc source from the OUT_DIR");
    }
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    copy_options.copy_inside = true;
    fs_extra::dir::copy(
        &mimalloc_src_dir,
        &mimalloc_out_src_dir,
        &copy_options,
    )
    .expect("failed to copy jemalloc source code to OUT_DIR");
    assert!(mimalloc_out_src_dir.exists());

    // Build mimalloc
    let dst = cmake::Config::new(mimalloc_out_src_dir)
        .define("OVERRIDE", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    let lib_name = match profile.as_str() {
        "debug" => "mimalloc-debug",
        "release" => "mimalloc",
        p => panic!("unknown profile \"{}\"", p),
    };
    println!("cargo:rustc-link-lib=dylib={}", lib_name);
}
