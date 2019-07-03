#!/usr/bin/env sh

set -ex

export MIMALLOC_SYS_ENABLE_WARNINGS=1

# Build mimalloc-sys
(
    cd mimalloc-sys
    cargo test
    cargo test --features override
    cargo test --release
    cargo test --release --features override
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
    cargo run --features override
    cargo run --release --features override

)
