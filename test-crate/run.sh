#!/bin/sh

cd ..
export CARGO_PRIMARY_PACKAGE=1
cargo clean
cargo build
cd test-crate
cargo clean
cargo run --bin cargo-rustc-ex --manifest-path ../Cargo.toml > ast # cargo run --bin cargo-print-hir-ast --manifest-path ../Cargo.toml > ast
# nvim ast
