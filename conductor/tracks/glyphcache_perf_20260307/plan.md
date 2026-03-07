# Plan: GlyphCache Hot Path Performans Optimizasyonu

**Track ID:** `glyphcache_perf_20260307`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: generation++ Hot Path Fix

- [ ] Task: `glyphcache.rs`'de `cached_glyph()` fonksiyonunu analiz et
    - [ ] `generation` alanının tüm kullanım yerlerini tespit et
    - [ ] `last_used` Cell alanının `CachedGlyph` içindeki kullanımlarını tespit et
- [ ] Task: `generation` artırımını sadece cache miss path'e taşı
    - [ ] Hit branch'ten `self.generation += 1` satırını kaldır
    - [ ] Miss path'te (glyph insert'ten önce) `self.generation += 1` bırak
- [ ] Task: Birim test yaz: hit path'te generation artmamalı
- [ ] Task: `cargo clippy -- -D warnings` ve `cargo fmt` çalıştır
- [ ] Task: Commit: `perf(render): Only increment glyph cache generation on cache miss`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: generation++ Fix' (Protocol in workflow.md)

---

## Phase 2: LruCache ile HashMap Değişimi

- [ ] Task: `lru` crate'ini `glyphcache.rs`'e import et (`lru::LruCache`)
- [ ] Task: `glyph_cache: HashMap<...>` → `glyph_cache: LruCache<GlyphKey, Rc<CachedGlyph>>` değiştir
    - [ ] `GlyphCache::new_gl()` ve `GlyphCache::new_webgpu()` içinde `LruCache::new(NonZeroUsize::new(capacity).unwrap())` kullan
    - [ ] `capacity` alanı korunuyor (LruCache'in max_capacity'sine karşılık gelir)
- [ ] Task: `evict_lru()` metodunu sil
    - [ ] `cached_glyph()` içindeki `evict_lru` çağrısını kaldır (LruCache otomatik yönetir)
- [ ] Task: `CachedGlyph`'ten `last_used: Cell<u64>` alanını kaldır
    - [ ] `CachedGlyph` tüm oluşturma yerlerinden `last_used` başlatmasını kaldır
    - [ ] `Debug` impl'ini güncelle
- [ ] Task: `generation` alanını değerlendir — LruCache sonrası gerekli değilse kaldır
- [ ] Task: Birim testler yaz
    - [ ] Capacity dolduktan sonra yeni glyph ekleme: en eski otomatik evict ediliyor mu?
    - [ ] Cache hit: O(1) döndürüyor mu?
    - [ ] Yeniden erişilen glyph evict edilmiyor mu?
- [ ] Task: `cargo clippy -- -D warnings` ve `cargo fmt` çalıştır
- [ ] Task: Commit: `perf(render): Replace HashMap with LruCache for O(1) glyph eviction`
- [ ] Task: Conductor - User Manual Verification 'Phase 2: LruCache Değişimi' (Protocol in workflow.md)

---

## Phase 3: Doğrulama ve Benchmark

- [ ] Task: `cargo test` çalıştır — tüm testler geçmeli
- [ ] Task: ANSI color throughput benchmark çalıştır
    - [ ] Önceki: 112 ms (avg) — bu değer eşit veya daha iyi olmalı
    - [ ] Sonuçları `benchmark_results.md`'e kaydet
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Doğrulama' (Protocol in workflow.md)
