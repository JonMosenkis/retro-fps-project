#!/usr/bin/env bash
set -euo pipefail

if ! cargo clippy --version >/dev/null 2>&1; then
  echo "error: cargo-clippy is not installed." >&2
  echo "Install it once with: rustup component add clippy" >&2
  exit 1
fi

cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
