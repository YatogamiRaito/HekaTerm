# Plan: WebGPU Render Pipeline Optimizasyonu

**Track ID:** `webgpu_render_20260306`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: Analiz ve Baseline

- [x] Task: Mevcut render pipeline'ı analiz et ca9c5da6b
    - [x] `wezterm-gui/src/renderstate.rs` dosyasını oku ve draw call yapısını belgele
    - [x] `wezterm-gui/src/quad.rs` dosyasındaki vertex/index yapısını incele
    - [x] `wezterm-gui/src/glyphcache.rs` dosyasında atlas ve eviction mantığını analiz et
    - [x] `wezterm-gui/src/shader.wgsl` shader pipeline'ını incele
- [x] Task: Benchmark baseline kaydet a3d64e7f3
    - [x] Mevcut `hyperfine` startup sonuçlarını `/tmp/baseline_bench.txt` dosyasına kaydet
    - [x] Render throughput testini çalıştır ve sonuçları kaydet
- [x] Task: Conductor - User Manual Verification 'Phase 1: Analiz ve Baseline' (Protocol in workflow.md)

---

## Phase 2: Vertex Buffer Pre-allocation (FR-3)

- [ ] Task: Vertex buffer pre-allocation yaz
    - [ ] `quad.rs` içinde sabit kapasiteli `QuadBuffer` struct'ı tanımla
    - [ ] `wgpu::Buffer` için `MAP_WRITE | COPY_SRC` kullanımını incele, staging buffer pattern uygula
    - [ ] Frame başında buffer'ı reuse et, sadece kapasiteyi aşınca grow et
- [ ] Task: Pre-allocation için birim testler yaz
    - [ ] `QuadBuffer::new()` doğru kapasiteyle başlatılıyor mu?
    - [ ] Capacity aşımında `grow()` doğru çalışıyor mu?
    - [ ] Clear/reuse sonrası len() sıfırlanıyor mu?
- [ ] Task: `cargo clippy` ve `cargo fmt` çalıştır, tüm uyarıları gider
- [ ] Task: Commit: `perf(render): Pre-allocate vertex buffer to avoid per-frame allocation`
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Vertex Buffer Pre-allocation' (Protocol in workflow.md)

---

## Phase 3: Draw Call Batching (FR-1)

- [ ] Task: Mevcut draw call yapısını tek batch'e dönüştür
    - [ ] `renderstate.rs` içindeki render loop'u incele
    - [ ] Tüm quad'ları önce vertex buffer'a topla, sonra tek `draw_indexed` ile gönder
    - [ ] Atlas texture layer'larına göre gruplandırma mantığını ekle
- [ ] Task: Batch render için birim testler yaz
    - [ ] N glyph için tam olarak ceil(N/atlas_layers) draw call yapılıyor mu?
    - [ ] Batch sonrası output correctness: render edilen glyph pozisyonları doğru mu?
- [ ] Task: `cargo clippy` ve `cargo fmt` çalıştır
- [ ] Task: Commit: `perf(render): Batch all quad draw calls per frame to reduce GPU submissions`
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Draw Call Batching' (Protocol in workflow.md)

---

## Phase 4: Glyph Cache LRU Eviction (FR-2)

- [ ] Task: LRU eviction policy implement et
    - [ ] `glyphcache.rs` içinde mevcut eviction mantığını tespit et
    - [ ] `lru` crate veya custom doubly-linked list + hashmap ile LRU implement et
    - [ ] Cache max size: `config` üzerinden konfigüre edilebilir yap (default: 4096)
    - [ ] Eviction sadece atlas full olduğunda tetiklenmeli
- [ ] Task: LRU cache için birim testler yaz
    - [ ] 4096 glyph dolduğunda eviction tetikleniyor mu?
    - [ ] En az son kullanılan glyph evict ediliyor mu?
    - [ ] Eviction sonrası yeni glyph doğru slot'a yerleştiriliyor mu?
- [ ] Task: `cargo clippy` ve `cargo fmt` çalıştır
- [ ] Task: Commit: `perf(render): Implement LRU eviction policy for glyph atlas cache`
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Glyph Cache LRU Eviction' (Protocol in workflow.md)

---

## Phase 5: Doğrulama ve Benchmark

- [ ] Task: Tüm testleri çalıştır
    - [ ] `cargo test` — tüm testler geçmeli
    - [ ] `cargo clippy -- -D warnings` — sıfır uyarı
    - [ ] `cargo fmt --check` — temiz
- [ ] Task: Render throughput benchmark'ı tekrarla
    - [ ] ANSI color rendering: sistem versiyonuyla karşılaştır
    - [ ] Unicode throughput: karşılaştır
    - [ ] ASCII dense scroll: karşılaştır
    - [ ] Sonuçları `conductor/tracks/webgpu_render_20260306/benchmark_results.md` dosyasına kaydet
- [ ] Task: Regression testi
    - [ ] `hyperfine` ile startup time regression olmadığını doğrula
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Doğrulama ve Benchmark' (Protocol in workflow.md)
