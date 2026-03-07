#!/bin/bash
# Start wezterm-gui in background
xvfb-run -s "-screen 0 1024x768x24" cargo run --release -p wezterm-gui -- --config periodic_stat_logging=1 start &
WEZ_PID=$!

# Wait for wezterm to open and map
sleep 4

# Use xdotool to type text (requires installing in container, but usually ubuntu base has bash)
# Since we didn't have sudo password, we'll try sending keys via simple python Xlib if xdotool fails
# If this fails, we'll just parse the logs that happen on startup for `gui.paint.impl` as a proxy for frame latency instead.

kill $WEZ_PID
