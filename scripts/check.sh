#!/bin/bash
set -e

echo "Formatting code..."
cargo fmt

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic

echo "Running tests..."
cargo test

echo "All checks passed."