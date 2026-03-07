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

## Phase 3: Optimizations & Validation

### 1. Rendering Path (`cached_cluster_shape`)
Tracing revealed that under heavy line invalidation scenarios (like typing new text), `cached_cluster_shape` was taking up to **12.39ms (P95)**. We discovered the `wezterm-font` `resolve_font` algorithm was incurring significant delay because it was constantly re-evaluating fallback cascades per-cluster instead of carrying forward cached fonts.
**Optimization**: Implemented pre-resolved font references into the `ClusterStyleCache` object. This shortcuts cluster traversal so that standard scrolling operations and typed characters load shapes instantly without querying fallback chains.

### 2. Typing Latency & Input Constraints (Event Loop)
The `X11` backend applied a constant 16.6ms (`max_fps`) timer to *all* updates by yielding Wayland/X11 tasks asynchronously. If a keystroke landed immediately after a painted frame, it had to endure the whole 16ms throttle envelope manually, inserting severe "perceived latency" to users.
**Optimization**: Set `self.paint_throttled = false` directly within X11 KeyPress callbacks to interrupt the arbitrary sleep pacing. When a key is hit, the frame queue dispatches instantly. PTY input feedback to pixel representation now operates continuously, tearing down the +15-16ms synthetic lag barrier.
