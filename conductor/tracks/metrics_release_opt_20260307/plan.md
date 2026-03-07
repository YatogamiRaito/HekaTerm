# Plan: Release Build'de Metrics Overhead Eliminasyonu

**Track ID:** `metrics_release_opt_20260307`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: Hot Path Metrics'i Koru Altına Al

- [ ] Task: Tüm hot path `metrics::histogram!` çağrılarını tespit et
    - [ ] `glyphcache.rs:659` — `glyph_cache.hit.rate`
    - [ ] `glyphcache.rs:662` — `glyph_cache.miss.rate`
    - [ ] `quad.rs:296` — `quad_buffer_apply`
    - [ ] `render/screen_line.rs:713` — `render_screen_line`
    - [ ] `render/paint.rs:115-116` — `gui.paint.impl` ve `gui.paint.impl.rate`
    - [ ] `termwindow/mod.rs:1404` — `mux.pane_output_event.rate`
- [ ] Task: Her çağrıyı `#[cfg(debug_assertions)]` ile sar
    ```rust
    #[cfg(debug_assertions)]
    metrics::histogram!("glyph_cache.glyph_cache.hit.rate").record(1.);
    ```
- [ ] Task: `cargo build --release` ile derleme doğrula — metrics kapalı
- [ ] Task: `cargo build` (debug) ile metrics hâlâ aktif mi doğrula
- [ ] Task: `cargo clippy -- -D warnings` ve `cargo fmt` çalıştır
- [ ] Task: Commit: `perf(render): Gate hot-path metrics behind cfg(debug_assertions)`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Metrics Koru' (Protocol in workflow.md)

---

## Phase 2: Doğrulama

- [ ] Task: `cargo test` çalıştır — tüm testler geçmeli (debug build, metrics aktif)
- [ ] Task: `cargo test --release` çalıştır — tüm testler geçmeli
- [ ] Task: Render throughput benchmark çalıştır (release build)
    - [ ] Sonuçları `benchmark_results.md`'e kaydet
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Doğrulama' (Protocol in workflow.md)
