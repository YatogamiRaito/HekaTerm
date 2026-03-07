# Throughput Benchmark Results (Phase 1)

## Baseline Measurements

Using `vtebench` logs piped through WezTerm with internal `periodic_stat_logging` enabled, the following latencies and bottlenecks were identified:

### 1. ASCII Throughput
- **Average Frame Latency (`gui.paint.impl`)**: ~1.55ms - 3.18ms
- **Screen Line Rendering (`render_screen_line`)**: ~6.37µs - 10.50µs per line.
- **Bottleneck**: The primary bottleneck in pure ASCII rendering is memory copies during line invalidation and `paint_pane.lines` iteration overhead.

### 2. ANSI / SGR Throughput
- **Bottleneck**: Heavily bottlenecked by the `wezterm-term` parser processing escape sequences (`termwiz::escape::parser::Parser`). Each state transition during fast SGR changes incurs branching overhead.

### 3. Unicode / BiDi Throughput
- **Average Shaping Latency (`shape.harfbuzz`)**: ~36.10µs - 236.54µs per cluster.
- **Bottleneck**: HarfBuzz shaping is significantly slower than ASCII rendering, forming the main bottleneck for complex text. 

## CPU Profiling (Flamegraph Analysis)
Profiling indicates hot paths in:
1. `termwiz::escape::parser::Parser::parse` (Escape sequence state machine)
2. `wezterm_gui::termwindow::render::paint::paint_pane` (Quad buffer generation)
3. `wezterm_font::shaper::harfbuzz::shape` (Text shaping)

## Next Steps (Phase 2)
Focus optimization on the escape sequence parser's hot loop or reducing `paint_pane` quad iteration overhead by implementing draw call batching (syncing with `webgpu_render` track if applicable).
