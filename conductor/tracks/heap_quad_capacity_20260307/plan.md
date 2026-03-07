# Plan: HeapQuadAllocator Pre-allocation

**Track ID:** `heap_quad_capacity_20260307`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: with_capacity Implementasyonu

- [ ] Task: `quad.rs`'de `HeapQuadAllocator` struct ve `Default` implementasyonunu incele
    - [ ] `Default::default()` → sıfır kapasiteli Vec'ler oluşturuyor mu? Doğrula.
    - [ ] `pane.rs:476`'da `HeapQuadAllocator::default()` çağrısını ve context'ini incele
- [ ] Task: `HeapQuadAllocator::with_capacity(quads_per_layer: usize) -> Self` ekle
    - [ ] Her layer için `Vec::with_capacity(quads_per_layer * VERTICES_PER_CELL)` kullan
- [ ] Task: `pane.rs:476`'da `default()` → `with_capacity(self.dims.cols as usize)` değiştir
    - [ ] `self.dims.cols` değerine render context içinden erişimi doğrula
    - [ ] Gerekirse uygun boyutu hesapla (cols * max_layers = üst sınır)
- [ ] Task: Birim testler yaz
    - [ ] `with_capacity(80)` sonrası `allocate()` çağrısında realloc olmadığını doğrula (capacity kontrolü ile)
    - [ ] 80'den fazla quad eklenince graceful grow yapıyor mu?
- [ ] Task: `cargo clippy -- -D warnings` ve `cargo fmt` çalıştır
- [ ] Task: Commit: `perf(render): Pre-allocate HeapQuadAllocator with column count capacity`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: with_capacity' (Protocol in workflow.md)

---

## Phase 2: Doğrulama

- [ ] Task: `cargo test` çalıştır — tüm testler geçmeli
- [ ] Task: Render throughput benchmark çalıştır, `benchmark_results.md`'e kaydet
- [ ] Task: Terminal normal render ediyor mu? (manuel kontrol)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Doğrulama' (Protocol in workflow.md)
