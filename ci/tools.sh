#!/usr/bin/sh

set -ex

# Documentation
(
    cd mimalloc-sys
    cargo doc
)
cargo doc

# Formatting
if rustup component add rustfmt-preview ; then
    (
        cd mimalloc-sys
        cargo fmt -- --check
    )
    cargo fmt -- --check
fi

# Clippy
if rustup component add clippy-preview ; then
    (
        cd mimalloc-sys
        cargo clippy -- -D clippy::pedantic
    )
    cargo clippy -- -D clippy::pedantic
fi

shellcheck ci/*.sh
