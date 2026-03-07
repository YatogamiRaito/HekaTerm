#!/usr/bin/env bash
set -e

echo "Building wezterm in release mode..."
cargo build --release

echo "Running vtebench ASCII..."
vtebench -b ascii > ascii.log || true

echo "Running vtebench SGR (ANSI)..."
vtebench -b sgr > sgr.log || true

echo "Running vtebench UTF-8 (Unicode)..."
vtebench -b utf8-complex > utf8.log || true

echo "Benchmarking complete."
