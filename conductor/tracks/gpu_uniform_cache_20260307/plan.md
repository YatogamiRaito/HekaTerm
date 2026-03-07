# Plan: GPU ShaderUniform Buffer Cache

**Track ID:** `gpu_uniform_cache_20260307`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: WebGpuState'e Kalıcı Uniform Buffer Ekle

- [ ] Task: `webgpu.rs` içinde mevcut `create_uniform()` implementasyonunu analiz et
    - [ ] `ShaderUniform` struct'ının alanlarını ve boyutunu incele
    - [ ] `create_uniform()` çağrıldığı tüm yerleri tespit et (`draw.rs`)
- [ ] Task: `WebGpuState` struct'ına `uniform_buffer` ve `uniform_bind_group` alanları ekle
    - [ ] `WebGpuState::new()` içinde buffer ve bind group'u bir kez oluştur
    - [ ] Buffer boyutu: `std::mem::size_of::<ShaderUniform>()` bytes
    - [ ] Usage: `UNIFORM | COPY_DST`
- [ ] Task: `update_uniform(&self, uniform: ShaderUniform)` metodu ekle
    - [ ] `self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniform]))`
    - [ ] Yeni GPU alloc yok
- [ ] Task: `cargo clippy -- -D warnings` ve `cargo fmt` çalıştır
- [ ] Task: Commit: `perf(render): Add persistent uniform buffer to WebGpuState`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: WebGpuState Uniform Buffer' (Protocol in workflow.md)

---

## Phase 2: draw.rs'de create_uniform Çağrısını Değiştir

- [ ] Task: `draw.rs` içindeki `webgpu.create_uniform(...)` çağrısını `webgpu.update_uniform(...)` ile değiştir
- [ ] Task: `render_pass.set_bind_group(0, &webgpu.uniform_bind_group, &[])` kullan
- [ ] Task: `create_uniform()` metodunu `webgpu.rs`'den sil
- [ ] Task: `cargo build --release` ile derleme doğrula
- [ ] Task: `cargo clippy -- -D warnings` ve `cargo fmt` çalıştır
- [ ] Task: Commit: `perf(render): Replace per-frame create_uniform with persistent bind group`
- [ ] Task: Conductor - User Manual Verification 'Phase 2: draw.rs Güncelleme' (Protocol in workflow.md)

---

## Phase 3: Doğrulama

- [ ] Task: `cargo test` çalıştır — tüm testler geçmeli
- [ ] Task: Görsel doğrulama — terminal render çıktısı değişmemiş olmalı
- [ ] Task: Render throughput benchmark çalıştır, sonuçları `benchmark_results.md`'e kaydet
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Doğrulama' (Protocol in workflow.md)
