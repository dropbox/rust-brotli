#!/usr/bin/env just --justfile

@_default:
    just --list --unsorted

# Clean all build artifacts
clean:
    cargo clean

# Build everything
build:
    RUSTFLAGS='-D warnings' cargo build --workspace --all-targets --bins --tests --lib --benches --examples

# Run cargo fmt and cargo clippy
lint: fmt clippy

# Run cargo fmt with optional params
fmt *ARGS:
    cargo fmt --all -- {{ ARGS }}

# Run cargo clippy
clippy:
    cargo clippy -- -D warnings
    cargo clippy --workspace --all-targets --bins --tests --lib --benches --examples -- -D warnings

# Build and open code documentation
docs:
    cargo doc --no-deps --open

# Test documentation
test-doc:
    cargo test --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

# Test using cargo test with optional params
test *ARGS:
    cargo test {{ ARGS }}

# Report current versions of rustc, cargo, and other utils
sys-info:
    rustc --version
    cargo --version
    {{ just_executable() }} --version

# All tests to run for CI
ci-test: sys-info (fmt "--check") build test test-doc
# TODO: clippy

# All stable tests to run for CI with the earliest supported Rust version. Assumes the Rust version is already set by rustup.
ci-test-msrv: sys-info build test
