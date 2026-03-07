# Project Tracks

This file tracks all major tracks for the project. Each track has its own detailed plan in its respective folder.

---

- [x] **Track: WebGPU render pipeline optimizasyonu — draw call batch'leme ve glyph cache iyileştirmesi**
  *Link: [./tracks/webgpu_render_20260306/](./tracks/webgpu_render_20260306/)*

---

- [x] **Track: Evaluate and optimize terminal throughput (ASCII, ANSI, Unicode/BiDi)**
  *Link: [./tracks/bench_throughput_20260307/](./tracks/bench_throughput_20260307/)*

---

- [x] **Track: Evaluate and minimize input-to-pixel (typing) and startup latency**
  *Link: [./tracks/bench_latency_20260307/](./tracks/bench_latency_20260307/)*

---

- [x] **Track: Profile and optimize terminal resource usage (RAM, CPU, GPU, VRAM)**
  *Link: [./tracks/bench_resource_20260307/](./tracks/bench_resource_20260307/)*

---

- [x] **Track: Benchmark and improve PTY parser and render synchronization (FPS stability)**
  *Link: [./tracks/bench_pty_sync_20260307/](./tracks/bench_pty_sync_20260307/)*

---

- [ ] **Track: Optimize release build profile — LTO, codegen-units=1, panic=abort, strip symbols**
  *Link: [./tracks/build_profile_20260307/](./tracks/build_profile_20260307/)*

---

- [ ] **Track: Cache wgpu ShaderUniform buffer to eliminate per-frame GPU allocations**
  *Link: [./tracks/gpu_uniform_cache_20260307/](./tracks/gpu_uniform_cache_20260307/)*

---

- [ ] **Track: Fix GlyphCache hot path — generation++ on every hit and O(n log n) eviction**
  *Link: [./tracks/glyphcache_perf_20260307/](./tracks/glyphcache_perf_20260307/)*

---

- [ ] **Track: Fix QuadBuffer::allocate() double-init redundant Quad construction**
  *Link: [./tracks/quadbuffer_fix_20260307/](./tracks/quadbuffer_fix_20260307/)*

---

- [ ] **Track: Pre-allocate HeapQuadAllocator with column count to eliminate per-glyph Vec::resize**
  *Link: [./tracks/heap_quad_capacity_20260307/](./tracks/heap_quad_capacity_20260307/)*

---

- [ ] **Track: Implement wgpu PipelineCache to persist compiled shaders and accelerate startup**
  *Link: [./tracks/wgpu_pipeline_cache_20260307/](./tracks/wgpu_pipeline_cache_20260307/)*

---

- [ ] **Track: Gate metrics::histogram! calls behind cfg(debug_assertions) in release builds**
  *Link: [./tracks/metrics_release_opt_20260307/](./tracks/metrics_release_opt_20260307/)*
