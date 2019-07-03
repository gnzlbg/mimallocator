A Rust allocator backed by mimalloc
===

[![Travis-CI Status]][travis]

This crates provides a Rust `#[global_allocator]` backed by [`mimalloc`].

See also the [`mimalloc-sys`] crate providing raw FFI bindings to [`mimalloc`].

## Usage

```toml
# Cargo.toml
[dependencies]
mimallocator = "0.1" 
```

```rust
// main.rs
#[global_allocator]
static GLOBAL: mimallocator::Mimalloc = mimallocator::Mimalloc;
```

The [`mimalloc`] CMake configuration is exposed with these features

- __stats__: Print statistics at program exit
- __secure__: Build in secure mode
- __check_full__: Enable full internal checks and asserts

```toml
# Cargo.toml
[dependencies]
mimallocator = { version = "0.1", features = ["secure", "stats", "check_full"] }
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `mimalloc-sys` by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[`mimalloc-sys`]: https://crates.io/crates/mimalloc-sys
[`mimalloc`]: https://github.com/microsoft/mimalloc
[travis]: https://travis-ci.com/gnzlbg/mimallocator
[Travis-CI Status]: https://travis-ci.com/gnzlbg/mimallocator.svg?branch=master
