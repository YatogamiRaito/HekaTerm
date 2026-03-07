# Spec: GlyphCache Hot Path Performans Optimizasyonu

## Overview

`GlyphCache::cached_glyph()` fonksiyonunda iki kritik sorun var:

**Sorun 1 — generation her cache hit'te artıyor (glyphcache.rs:656):**
```rust
self.generation += 1;  // HER lookup'ta — hit olsa bile
if let Some(entry) = self.glyph_cache.get(...) {
    entry.last_used.set(self.generation);  // Cell write hot path'te
    return Ok(...);
}
```
10.000 satır ANSI render ≈ 200.000 glyph lookup = 200.000 gereksiz u64 increment + Cell::set.

**Sorun 2 — evict_lru() O(n log n) sort (glyphcache.rs:717):**
```rust
let mut items: Vec<_> = self.glyph_cache.iter()
    .map(|(k, v)| (k.clone(), v.last_used.get()))
    .collect();        // 8192 entry clone + Vec alloc
items.sort_by_key(...); // O(8192 × log 8192) ≈ O(106.000 ops)
```

Workspace'de `lru = "0.16"` zaten mevcut — kullanılmıyor.

## Functional Requirements

### FR-1: generation sadece cache miss'te artacak
```rust
// HIT path:
if let Some(entry) = self.glyph_cache.get(...) {
    // generation güncelleme yok — lru crate bunu handle eder
    return Ok(Rc::clone(entry));
}
// MISS path:
self.generation += 1; // sadece burada
```

### FR-2: HashMap<GlyphKey, Rc<CachedGlyph>> → LruCache ile değiştir
- `lru::LruCache<GlyphKey, Rc<CachedGlyph>>` kullan.
- `LruCache` `get()` çağrısı otomatik olarak erişim sırasını günceller — O(1).
- `capacity` alanı `LruCache::new(NonZeroUsize::new(capacity))` ile set edilir.
- `evict_lru()` metodu silinir — `LruCache` kendi eviction'ını yönetir.
- `last_used: Cell<u64>` alanı `CachedGlyph`'ten kaldırılır.

### FR-3: GlyphCache::capacity konfigüre edilebilir kalacak
- Default: 8192 (mevcut değer).
- `config` üzerinden ileride ayarlanabilir yapı korunacak.

## Non-Functional Requirements
- Cache hit: O(1) (LruCache garantisi).
- Cache eviction: O(1) amortized (LruCache garantisi).
- `last_used` Cell overhead'i: sıfır (alan kaldırılıyor).
- Tüm testler geçmeli.

## Acceptance Criteria
- [ ] `glyph_cache` alanı `LruCache` tipinde.
- [ ] `evict_lru()` metodu silinmiş.
- [ ] `last_used` alanı `CachedGlyph`'ten kaldırılmış.
- [ ] `generation` sadece miss path'te artıyor.
- [ ] `cargo test` geçiyor.
- [ ] `cargo clippy -- -D warnings` temiz.
- [ ] ANSI render throughput benchmark'ta kayıp yok.

## Out of Scope
- Font atlas boyutu değişikliği.
- Glyph şekillendirme (harfbuzz) optimizasyonu.
