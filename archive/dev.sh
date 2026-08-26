#!/usr/bin/env bash
set -euo pipefail

# If cargo-watch is not installed, install it (requires cargo in PATH)
if ! command -v cargo-watch >/dev/null 2>&1; then
  echo "cargo-watch not found — installing via 'cargo install cargo-watch'..."
  cargo install cargo-watch
fi

echo "Starting auto-reload: cargo watch -x run"
cargo watch -x run
