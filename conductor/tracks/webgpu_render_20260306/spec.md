# Spec: WebGPU Render Pipeline Optimizasyonu

## Overview

HekaTerm'in `wezterm-gui` katmanındaki WebGPU render pipeline'ı, Glium'dan wgpu'ya geçiş sırasında fonksiyonel doğruluk hedeflenerek yazıldı. Bu track, render throughput'unu artırmak için iki kritik optimizasyon alanına odaklanır:

1. **Draw call batching** — her frame'de GPU'ya gönderilen draw call sayısını minimize etme
2. **Glyph cache iyileştirme** — atlas texture yönetimini ve cache miss oranını optimize etme

Benchmark verileri (bu konuşmada yapılan testler) şunu gösterdi:
- ANSI color rendering: sistem versiyonundan %71 yavaş
- Unicode throughput: %46 yavaş
- ASCII dense scroll: eşdeğer (PTY parse hızı etkilenmemiş)

Bu fark render pipeline overhead'inden kaynaklanmaktadır.

## Functional Requirements

### FR-1: Draw Call Batching
- Aynı frame içindeki tüm glyph quad'ları tek bir `draw_indexed` call'ında gönderilmeli.
- Farklı texture atlas layer'larına ait quad'lar ayrı draw call'lara ayrılabilir, ancak her layer için tek call zorunludur.
- Mevcut per-glyph draw call yapısı kaldırılmalı.

### FR-2: Glyph Cache Eviction Policy
- LRU (Least Recently Used) eviction uygulanmalı.
- Cache boyutu konfigüre edilebilir olmalı (default: 4096 glyph).
- Eviction, atlas texture full olduğunda tetiklenmeli, her frame'de değil.

### FR-3: Vertex Buffer Pre-allocation
- Vertex buffer her frame'de yeniden allocate edilmemeli.
- Frame başına maksimum quad sayısı için buffer başlangıçta pre-allocate edilmeli.
- Buffer kapasitesi aşılırsa iki katına grow edilmeli (amortized O(1)).

### FR-4: Shader Uniform Batching
- Per-glyph uniform push yerine uniform buffer array kullanılmalı.
- Tek bind group ile tüm frame'in uniform verisi aktarılmalı.

## Non-Functional Requirements

- Render throughput: ANSI color testinde en az sistem versiyonu ile eşdeğer (hedef: %20 daha hızlı).
- Memory: glyph cache boyutu konfigüre edilebilir, default usage artmamalı.
- Correctness: mevcut tüm render test senaryoları geçmeli.

## Acceptance Criteria

- [ ] `cargo test` tüm testleri geçiyor.
- [ ] `cargo clippy -- -D warnings` temiz.
- [ ] `cargo fmt --check` temiz.
- [ ] Benchmark: `hyperfine` ile startup time regression yok.
- [ ] ANSI color render throughput: sistem versiyonuna eşit veya daha iyi.
- [ ] Glyph atlas eviction doğru çalışıyor (test ile doğrulanmış).

## Out of Scope

- Font shaping algoritması değişikliği
- Yeni shader effect'leri (blur, shadow vb.)
- Wayland veya X11 backend değişikliği
- Multiplexer veya SSH katmanı
