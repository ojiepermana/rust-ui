#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
if ! command -v watchexec >/dev/null 2>&1; then
  echo 'Install the optional watcher: cargo install watchexec-cli --version 2.8.0 --locked' >&2
  exit 1
fi
# Rebuild and restart the complete native process. This is not hot reload.
exec watchexec --restart --watch crates --watch apps/showcase --watch docs/getting-started \
  --watch Cargo.toml --watch Cargo.lock --watch rust-toolchain.toml \
  --exts rs,toml,json,svg,md -- cargo run -p showcase --locked
