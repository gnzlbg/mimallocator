Raw C FFI bindings to mimalloc 
===

This crates provides raw C FFI bindings to the  [`mimalloc`] library.

## Documentation

The documentation of the FFI bindings can be found in [docs.rs].

**Current mimalloc version**: 1.0.1

**Build dependencies**: `cmake`.

## Cargo features

The [`mimalloc`] CMake configuration is exposed with these features

- __stats__: Print statistics at program exit
- __secure__: Build in secure mode
- __check_full__: Enable full internal checks and asserts

## Platform support

The following table describes the supported platforms: 

* `build`: does the library compile for the target?
* `run`: do our own tests pass on the target?
* `mimalloc`: do `mimalloc`'s tests pass on the target?
* `valgrind`: do our own tests pass under valgrind?

Tier 1 targets are tested on all Rust channels (stable, beta, and nightly). 
All other targets are only tested on Rust nightly.

| **Apple** targets:               | build     | run     | mimalloc | valgrind |
|----------------------------------|-----------|---------|----------|----------|
| `x86_64-apple-darwin`         | ✓         | ✓      | ✗         | ✗       |

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

[`mimalloc`]: https://github.com/microsoft/mimalloc
