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
        .skip_signededness(|c| c.ends_with("_t") || c.ends_with("_e"))
        .type_name(|ty, _is_struct, _is_union| {
            match ty {
                t if t.ends_with("_s") => format!("struct {}", t),
                t if t.ends_with("_e") => format!("enum {}", t),
                t if t.ends_with("_t") => t.to_string(),
                // mimalloc defines it's callbacks with the pointer at the
                // location of use, e.g. `typedef ret mi_some_fun(a0 x, a1 y);`
                // and then use `mi_some_fun *arg` as argument types, which
                // appears to upset ctest.
                t if t.ends_with("_fun") => format!("{}*", t),
                t => t.to_string(),
            }
        })
        ;

        cfg.generate("../mimalloc-sys/src/lib.rs", "all.rs");
}
