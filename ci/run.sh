#!/usr/bin/env sh

set -ex

export MIMALLOC_SYS_ENABLE_WARNINGS=1

# Build mimalloc-sys
(
    cd mimalloc-sys
    cargo test
    cargo test --release
    case $TARGET in
        *window*)
            echo "Override not available on Windows"
            ;;
        *)
            cargo test --features override
            cargo test --release --features override
            ;;
    esac
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
    case $TARGET in
        *window*)
            echo "Override not available on Windows"
            ;;
        *)
            cargo run --features override
            cargo run --release --features override
            ;;
    esac

)
