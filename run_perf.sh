#!/usr/bin/env bash
set -e

# Build release without stripping to ensure perf has symbols
cargo build --release

echo "Running perf for ASCII..."
perf record -o perf-ascii.data -F 999 --call-graph dwarf -- ./target/release/wezterm-gui start --config term="wezterm" --config enable_wayland=false -- bash -c "for i in {1..200}; do cat ascii.log; done"

echo "Running perf for SGR..."
perf record -o perf-sgr.data -F 999 --call-graph dwarf -- ./target/release/wezterm-gui start --config term="wezterm" --config enable_wayland=false -- bash -c "for i in {1..200}; do cat sgr.log; done"

echo "Running perf for UTF-8..."
perf record -o perf-utf8.data -F 999 --call-graph dwarf -- ./target/release/wezterm-gui start --config term="wezterm" --config enable_wayland=false -- bash -c "for i in {1..200}; do cat utf8.log; done"

# We won't generate actual svgs right now unless flamegraph scripts are available, but perf report can be used.
echo "Perf run complete."
