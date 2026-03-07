# Latency Benchmark Results (Phase 1)

## Baseline Measurements

### 1. Startup Time
Measuring cold/warm startup time using `hyperfine` on `target/release/wezterm start --always-new-process -- bash -c 'exit 0'`:
- **Mean Time**: 103.7 ms ± 12.9 ms
- **Range**: 83.5 ms … 140.9 ms
- **Notes**: CPU Overhead accounts for ~77ms user and ~29ms system. Window and PTY spawning takes a bulk of this time.

### 2. Typing Latency
- Measured via internal window event tracing (event timestamp compared to quad render finish time).
- Baseline TBD based on trace profile.

## Next Profiling Steps
Investigate the `window` crate's `X11`/`Wayland` event loops and WezTerm's `termwindow` PTY interaction lock logic to identify stalling.

### 2. Rendering Throughput Latency
We isolated `gui.paint.impl` metric profiling during continuous frame invalidation to approximate processing input without X11 binding latency.
- **Mean Pipeline Time (`gui.paint.impl`)**: ~8.45ms (P50) up to ~11.60ms (P95).
- **Quad Iteration**: ~278.53µs (`paint_pane.lines`) 
- **Notes**: Overall latency per frame averages 8.45ms, adding to the startup footprint. Optimizing `paint_pane.lines` draw call batches could improve this.

## Next Steps (Phase 2 & 3)
1. Proceed with async `spawn_delay` optimizations around startup (PTY spawning/event loop setup).
2. Continue rendering draw call optimizations for minimizing frame time, cutting input-to-pixel presentation.
