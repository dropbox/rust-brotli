#!/usr/bin/env just --justfile

@_default:
    just --list --unsorted

# Clean all build artifacts
clean:
    cargo clean

# Build everything
build: build-brotli build-simd build-ffi

# Build the main crate
build-brotli:
    RUSTFLAGS='-D warnings' cargo build --workspace --all-targets --bins --tests --lib --benches --examples

# Build simd with nightly
build-simd:
    RUSTFLAGS='-D warnings' cargo +nightly build --features simd

# Build the brotli-ffi crate (in ./c dir)
build-ffi:
    RUSTFLAGS='-D warnings' cargo build --workspace --all-targets --bins --tests --lib --benches --examples --manifest-path c/Cargo.toml
    # For now, use original make file for building/testing the FFI crate
    cd c && make

# Run cargo fmt with optional params
fmt *ARGS:
    cargo fmt --all -- {{ ARGS }}
    cd c && cargo fmt --all -- {{ ARGS }}

# Run Nightly cargo fmt, ordering imports by groups
fmt2:
    cargo +nightly fmt -- --config imports_granularity=Module,group_imports=StdExternalCrate

# Run cargo clippy
clippy:
    cargo clippy -- -D warnings
    cargo clippy --workspace --all-targets --bins --tests --lib --benches --examples -- -D warnings
    cd c && cargo clippy -- -D warnings
    cd c && cargo clippy --workspace --all-targets --bins --tests --lib --benches --examples -- -D warnings

# Build and open code documentation
docs:
    cargo doc --no-deps --open
    cd c && cargo doc --no-deps --open

# Test documentation
test-doc:
    cargo test --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
    cd c && cargo test --doc
    cd c && RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

# Test using cargo test with optional params
test *ARGS:
    cargo test {{ ARGS }}
    cd c && cargo test {{ ARGS }}

# Report current versions of rustc, cargo, and other utils
sys-info:
    rustc --version
    cargo --version
    {{ just_executable() }} --version

# Get MSRV (Minimum Supported Rust Version) for the brotli crate
read-msrv:
    cargo metadata --no-deps --format-version 1 | jq -r -e '.packages[] | select(.name == "brotli").rust_version'

# All tests to run for CI (TODO: add clippy)
ci-test: sys-info (fmt "--check") build test test-doc

# All stable tests to run for CI with the earliest supported Rust version. Assumes the Rust version is already set by rustup.
ci-test-msrv: sys-info build-brotli build-ffi test

# Test if changes are backwards compatible (patch), or need a new minor/major version
semver-checks:
    cargo semver-checks
