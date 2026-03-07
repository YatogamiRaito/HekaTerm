# Spec: QuadBuffer::allocate() Double-Init Bug Fix

## Overview

`QuadBuffer::allocate()` içinde aynı vertex slot iki kez başlatılıyor:

```rust
pub fn allocate(&mut self) -> anyhow::Result<QuadImpl<'_>> {
    // ...
    let mut quad = Quad {                          // ← 1. init
        vert: &mut self.vertices[start..start + VERTICES_PER_CELL],
    };
    quad.set_has_color(false);                     // ← 1. set_has_color
    // quad burada drop ediliyor — işe yaramıyor

    let vert: &mut [Vertex] =
        unsafe { std::mem::transmute(...) };       // ← aynı bellek, 2. init
    let mut quad = Quad { vert };
    quad.set_has_color(false);                     // ← 2. set_has_color (redundant)

    Ok(QuadImpl::Vert(quad))
}
```

İlk `Quad` oluşturulup `set_has_color(false)` çağrılıyor, sonra hemen drop ediliyor. İkinci `Quad` aynı bellek aralığıyla yeniden oluşturuluyor ve aynı `set_has_color(false)` tekrar çağrılıyor. Bu, her glyph render'ında gereksiz bir başlatma işlemi yapıyor.

## Functional Requirements

### FR-1: İlk dead Quad kaldırılacak
```rust
pub fn allocate(&mut self) -> anyhow::Result<QuadImpl<'_>> {
    if self.num_quads >= self.capacity {
        self.grow();
    }
    let start = self.num_quads * VERTICES_PER_CELL;
    self.num_quads += 1;

    // SAFETY: QuadBuffer owns vertices; the returned QuadImpl is used
    // ephemerally and does not outlive self in practice (enforced by caller).
    let vert: &mut [Vertex] =
        unsafe { std::mem::transmute(&mut self.vertices[start..start + VERTICES_PER_CELL]) };
    let mut quad = Quad { vert };
    quad.set_has_color(false);

    Ok(QuadImpl::Vert(quad))
}
```

### FR-2: SAFETY comment zorunlu
Mevcut `unsafe transmute` korunacak ancak `// SAFETY:` açıklaması eklenecek (product-guidelines.md gereği).

## Non-Functional Requirements
- Davranış değişikliği yok — sadece gereksiz işlem kaldırılıyor.
- `cargo test` geçmeli.

## Acceptance Criteria
- [ ] `allocate()` içinde tek bir `Quad` oluşturma ve tek `set_has_color(false)` çağrısı.
- [ ] `unsafe` bloğu `// SAFETY:` açıklamasına sahip.
- [ ] `cargo test` geçiyor.
- [ ] `cargo clippy -- -D warnings` temiz.

## Out of Scope
- `unsafe transmute`'un safe bir alternatifle değiştirilmesi (ayrı bir inceleme gerektirir).
- `QuadBuffer` API değişikliği.
