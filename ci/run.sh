#!/usr/bin/sh

set -ex

# Build mimalloc-sys
(
    cd mimalloc-sys
    cargo build
    cargo build --release
)

# Test mimallocator
cargo test
cargo test --release

# Test mimalloc-sys ABI:
(
    cd mimalloc-sys-test
    cargo test
    cargo test --release
)
