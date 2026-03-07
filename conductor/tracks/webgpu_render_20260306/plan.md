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

- [x] Task: Vertex buffer pre-allocation yaz b81dba7a1
    - [x] `quad.rs` içinde sabit kapasiteli `QuadBuffer` struct'ı tanımla
    - [x] `wgpu::Buffer` için `MAP_WRITE | COPY_SRC` kullanımını incele, staging buffer pattern uygula
    - [x] Frame başında buffer'ı reuse et, sadece kapasiteyi aşınca grow et
- [x] Task: Pre-allocation için birim testler yaz b81dba7a1
    - [x] `QuadBuffer::new()` doğru kapasiteyle başlatılıyor mu?
    - [x] Capacity aşımında `grow()` doğru çalışıyor mu?
    - [x] Clear/reuse sonrası len() sıfırlanıyor mu?
- [x] Task: `cargo clippy` ve `cargo fmt` çalıştır, tüm uyarıları gider b81dba7a1
- [x] Task: Commit: `perf(render): Pre-allocate vertex buffer to avoid per-frame allocation` b81dba7a1
- [x] Task: Conductor - User Manual Verification 'Phase 2: Vertex Buffer Pre-allocation' (Protocol in workflow.md)

---

## Phase 3: Draw Call Batching (FR-1)

- [x] Task: Mevcut draw call yapısını tek batch'e dönüştür 376ff3409
    - [x] `renderstate.rs` içindeki render loop'u incele
    - [x] Tüm quad'ları önce vertex buffer'a topla, sonra tek `draw_indexed` ile gönder
    - [x] Atlas texture layer'larına göre gruplandırma mantığını ekle
- [x] Task: Batch render için birim testler yaz 376ff3409
    - [x] N glyph için tam olarak ceil(N/atlas_layers) draw call yapılıyor mu?
    - [x] Batch sonrası output correctness: render edilen glyph pozisyonları doğru mu?
- [x] Task: `cargo clippy` ve `cargo fmt` çalıştır 376ff3409
- [x] Task: Commit: `perf(render): Batch all quad draw calls per frame to reduce GPU submissions` 376ff3409
- [x] Task: Conductor - User Manual Verification 'Phase 3: Draw Call Batching' (Protocol in workflow.md)

---

## Phase 4: Glyph Cache LRU Eviction (FR-2)

- [x] Task: `cache_size` ve `eviction_threshold` ayarlarını ekle
    - [x] `GlyphCache` yapısına `pub capacity: usize` ekle
    - [x] `wezterm.lua` üzerinden ayarlanacak parametre bağla (opsiyonel/hardcode başla)
- [x] Task: LRU algoritmasını `glyphcache.rs`'e entegre et
    - [x] Texture atlas dolduğu zaman usage timestamp'i en eski olanları bul
    - [x] En eski N (örn. %10) glyph'i sil veya LRU queue kullan
- [x] Task: LRU evict logic için unit test yaz
    - [x] 4096 glyph dolduğunda eviction tetikleniyor mu?
    - [x] En az son kullanılan glyph evict ediliyor mu?
    - [x] Eviction sonrası yeni glyph doğru slot'a yerleştiriliyor mu?
- [x] Task: `cargo clippy` ve `cargo fmt` çalıştır
- [x] Task: Commit: `perf(render): Add LRU eviction policy to glyph cache`
- [x] Task: Conductor - User Manual Verification 'Phase 4: Glyph Cache LRU Eviction' (Protocol in workflow.md)

---

## Phase 5: Doğrulama ve Benchmark

- [x] Task: Tüm testleri çalıştır 2351b2967
    - [x] `cargo test` — tüm testler geçmeli
    - [x] `cargo clippy -- -D warnings` — sıfır uyarı
    - [x] `cargo fmt --check` — temiz
- [x] Task: Render throughput benchmark'ı tekrarla
    - [x] ANSI color rendering: sistem versiyonuyla karşılaştır
    - [x] Unicode throughput: karşılaştır
    - [x] ASCII dense scroll: karşılaştır
    - [x] Sonuçları `conductor/tracks/webgpu_render_20260306/benchmark_results.md` dosyasına kaydet
- [x] Task: Regression testi
    - [x] `hyperfine` ile startup time regression olmadığını doğrula
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Doğrulama ve Benchmark' (Protocol in workflow.md)
