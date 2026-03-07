#!/bin/bash
set -e

echo "Starting wezterm-gui in background..."
xvfb-run -s "-screen 0 1024x768x24" cargo run --release -p wezterm-gui -- --config periodic_stat_logging=1 start &
WEZ_PID=$!

sleep 5

echo "Measuring IDLE Memory (RSS/VSZ)..."
ps -o pid,rss,vsz,comm -p $WEZ_PID > perf-memory-idle.txt
cat perf-memory-idle.txt

echo "Preparing 100k lines of stress data..."
yes "This is a memory stress test line to fill up the scrollback buffer in wezterm. " | head -n 100000 > /tmp/stress.txt

echo "Injecting stress data into Wezterm..."
# We can't easily pipe to a specific pane from outside if we don't know the pane ID,
# but we can run a wezterm cli command to spawn a process inside the gui that cats the file.
cargo run --release -p wezterm-gui -- cli spawn -- bash -c 'cat /tmp/stress.txt; sleep 2' || true

sleep 5

echo "Measuring ACTIVE Memory (RSS/VSZ)..."
ps -o pid,rss,vsz,comm -p $WEZ_PID > perf-memory-active.txt
cat perf-memory-active.txt

echo "Cleaning up..."
kill $WEZ_PID || true
