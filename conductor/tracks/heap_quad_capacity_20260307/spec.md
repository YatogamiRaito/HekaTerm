# Spec: HeapQuadAllocator Pre-allocation

## Overview

`HeapQuadAllocator` her glyph için `Vec::resize` çağrısı yapıyor:

```rust
impl TripleLayerQuadAllocatorTrait for HeapQuadAllocator {
    fn allocate(&mut self, layer_num: usize) -> anyhow::Result<QuadImpl<'_>> {
        let vertices = match layer_num { ... };
        let start = vertices.len();
        vertices.resize(start + VERTICES_PER_CELL, Vertex::default()); // ← her glyph'te
        // ...
    }
}
```

`HeapQuadAllocator::default()` çağrılırken (pane.rs:476) sıfır kapasiteyle başlatılıyor. Satırdaki her glyph için `resize` → potansiyel realloc. Bir terminal satırında 80-200 glyph olabilir, her satır için `HeapQuadAllocator` yeniden oluşturuluyor.

## Functional Requirements

### FR-1: HeapQuadAllocator'a capacity hint ekle
```rust
pub struct HeapQuadAllocator {
    layer0: Vec<Vertex>,
    layer1: Vec<Vertex>,
    layer2: Vec<Vertex>,
}

impl HeapQuadAllocator {
    pub fn with_capacity(quads_per_layer: usize) -> Self {
        let cap = quads_per_layer * VERTICES_PER_CELL;
        Self {
            layer0: Vec::with_capacity(cap),
            layer1: Vec::with_capacity(cap),
            layer2: Vec::with_capacity(cap),
        }
    }
}
```

### FR-2: pane.rs'de with_capacity kullan
- `HeapQuadAllocator::default()` → `HeapQuadAllocator::with_capacity(cols)` (terminal kolon sayısı kapasiteyi belirler).
- `cols` değeri render context'ten erişilebilir (`self.dims.cols`).

### FR-3: allocate() içinde with_capacity sonrası resize yerine push/extend
- `resize` yerine `extend` + `get_unchecked_mut` ile doğrudan slot yazımı.
- Ya da: `Vec::spare_capacity_mut` + `MaybeUninit` pattern — daha güvenli.
- En basit: `resize` devam eder ancak `with_capacity` sayesinde realloc olmaz.

## Non-Functional Requirements
- Her satır render'ında `HeapQuadAllocator` içinde realloc sayısı: 0 (normal koşullar).
- `cargo test` geçmeli.

## Acceptance Criteria
- [ ] `HeapQuadAllocator::with_capacity()` metodu mevcut.
- [ ] `pane.rs:476`'da `with_capacity(self.dims.cols)` kullanılıyor.
- [ ] `cargo test` geçiyor.
- [ ] `cargo clippy -- -D warnings` temiz.
- [ ] Render throughput benchmark'ta iyileşme veya eşit sonuç.

## Out of Scope
- `HeapQuadAllocator`'ın frame'ler arası yeniden kullanımı (daha büyük refactor).
- Layer başına farklı kapasite stratejisi.
