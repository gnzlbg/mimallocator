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
cargo test --features stats
cargo test --features secure
cargo test --features check_full
cargo test --release
cargo test --release --features secure
cargo test --release --features secure,check_full,stats

# Test mimalloc-sys ABI:
(
    cd mimalloc-sys-test
    cargo run
    cargo run --release
)
