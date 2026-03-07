# Spec: Release Build'de Metrics Overhead Eliminasyonu

## Overview

Render hot path'te `metrics::histogram!` çağrıları her frame ve her glyph lookup'ta çalışıyor:

```
glyphcache.rs:659  — her cache hit (200.000x / 10k ANSI satırı)
glyphcache.rs:662  — her cache miss
render/paint.rs    — her frame
screen_line.rs:713 — her satır render
quad.rs:296        — her HeapQuadAllocator::apply_to()
```

`metrics` crate her çağrıda atomik counter güncelleme yapıyor. Release build'de bu profiling verisi kullanılmıyor ancak CPU overhead bırakıyor.

## Functional Requirements

### FR-1: Hot path metrics'i cfg(debug_assertions) arkasına al

Aşağıdaki çağrılar `#[cfg(debug_assertions)]` ile sarılacak:

- `glyphcache.rs`: `glyph_cache.hit.rate` ve `glyph_cache.miss.rate`
- `quad.rs`: `quad_buffer_apply`
- `render/screen_line.rs`: `render_screen_line`
- `render/paint.rs`: `gui.paint.impl` ve `gui.paint.impl.rate`
- `termwindow/mod.rs`: `mux.pane_output_event.rate`

```rust
#[cfg(debug_assertions)]
metrics::histogram!("glyph_cache.glyph_cache.hit.rate").record(1.);
```

### FR-2: Frame-level metrics korunabilir (opsiyonel)
- `gui.paint.impl` gibi frame-level metrics isteğe bağlı `metrics` feature flag ile açık bırakılabilir.
- Default: debug_assertions (yani release'te kapalı).

### FR-3: metrics crate bağımlılığı kaldırılmayacak
- Crate bağımlılığı korunuyor; sadece hot path çağrıları koşullu hale geliyor.
- Debug build'lerde davranış değişmiyor.

## Non-Functional Requirements
- Release build'de hot path'te sıfır atomik metrics operasyonu.
- Debug build davranışı değişmiyor.
- `cargo test` geçmeli (testler debug_assertions ile çalışır).

## Acceptance Criteria
- [ ] `glyphcache.rs`'deki hit/miss histogram çağrıları `#[cfg(debug_assertions)]` ile korumalı.
- [ ] `quad.rs`, `screen_line.rs`, `paint.rs`, `mod.rs`'deki hot path histogram'lar korumalı.
- [ ] `cargo build --release` başarılı.
- [ ] `cargo test` geçiyor.
- [ ] `cargo clippy -- -D warnings` temiz.

## Out of Scope
- metrics crate'inin tamamen kaldırılması.
- Yeni metrics eklenmesi.
- Tracing/span overhead (ayrı track olarak değerlendirilebilir).
