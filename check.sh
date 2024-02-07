#!/usr/bin/env bash
# This scripts runs various CI-like checks in a convenient way.
set -eux

cargo check --workspace --all-targets
cargo fmt --all -- --check
cargo test --workspace --all-targets --all-features
cargo test --workspace --doc
