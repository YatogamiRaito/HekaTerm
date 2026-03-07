# Plan: QuadBuffer::allocate() Double-Init Bug Fix

**Track ID:** `quadbuffer_fix_20260307`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: Fix ve Test

- [ ] Task: `quad.rs`'de `QuadBuffer::allocate()` implementasyonunu incele (satır 243-263)
    - [ ] İlk `Quad` oluşturma ve `set_has_color(false)` çağrısının drop edildiğini doğrula
    - [ ] İkinci `Quad` oluşturmanın aynı vertex slot'unu kullandığını doğrula
- [ ] Task: Double-init'i gider
    - [ ] İlk dead `Quad { vert: &mut self.vertices[...] }` bloğunu ve `set_has_color(false)` çağrısını sil
    - [ ] Yalnızca `unsafe transmute` + tek `Quad` + tek `set_has_color(false)` bırak
    - [ ] `// SAFETY:` açıklaması ekle: transmute'un neden sound olduğunu belgele
- [ ] Task: Mevcut birim testi doğrula (`test_quad_buffer_allocation`)
    - [ ] Var olan test geçiyor mu? → `cargo test -p wezterm-gui quad`
- [ ] Task: Yeni birim test ekle: `allocate()` sonrası `has_color` flag false mu?
- [ ] Task: `cargo clippy -- -D warnings` ve `cargo fmt` çalıştır
- [ ] Task: Commit: `fix(render): Remove redundant double-init in QuadBuffer::allocate`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Fix ve Test' (Protocol in workflow.md)

---

## Phase 2: Doğrulama

- [ ] Task: `cargo test` çalıştır — tüm testler geçmeli
- [ ] Task: `cargo build --release` çalıştır — derleme başarılı
- [ ] Task: Terminal görsel olarak doğru render ediyor mu? (manuel kontrol)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Doğrulama' (Protocol in workflow.md)
