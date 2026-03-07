# Spec: GPU ShaderUniform Buffer Cache

## Overview

`WebGpuState::create_uniform()` her frame'de yeni bir `wgpu::Buffer` ve `wgpu::BindGroup` oluşturuyor:

```rust
pub fn create_uniform(&self, uniform: ShaderUniform) -> wgpu::BindGroup {
    let buffer = self.device.create_buffer_init(...); // GPU alloc HER FRAME
    self.device.create_bind_group(...)                // GPU alloc HER FRAME
}
```

`ShaderUniform` şunları içeriyor: `foreground_text_hsb` (nadiren değişir), `milliseconds` (her frame değişir), `projection` (pencere resize'da değişir). Bu yüzden buffer her frame reuse edilebilir; sadece içerik `queue.write_buffer` ile güncellenebilir.

## Functional Requirements

### FR-1: Persistent Uniform Buffer
- `WebGpuState`'e kalıcı `uniform_buffer: wgpu::Buffer` ve `uniform_bind_group: wgpu::BindGroup` alanları ekle.
- Bu buffer `WebGpuState::new()` içinde **bir kez** oluşturulacak.
- Her frame'de `queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniform]))` ile sadece içerik güncellenir.

### FR-2: create_uniform() kaldır
- `create_uniform()` metodu silinecek.
- `draw.rs`'de `webgpu.create_uniform(...)` çağrısı `webgpu.update_uniform(...)` ile değiştirilecek (içerik günceller, yeni alloc yapmaz).

### FR-3: Bind Group yeniden kullanımı
- `uniform_bind_group` bir kez oluşturulacak, her frame'de `render_pass.set_bind_group(0, &webgpu.uniform_bind_group, &[])` ile set edilecek.

## Non-Functional Requirements
- Her frame'de GPU allocation sayısı: ShaderUniform için sıfır.
- `cargo test --release` geçmeli.
- Davranış değişikliği olmamalı (sadece allocation azalır).

## Acceptance Criteria
- [ ] `create_uniform()` metodu silinmiş.
- [ ] `WebGpuState` struct'ında `uniform_buffer` ve `uniform_bind_group` alanları mevcut.
- [ ] Her frame'de sadece `write_buffer` çağrısı yapılıyor, yeni `create_buffer` yok.
- [ ] Render çıktısı görsel olarak değişmemiş.
- [ ] `cargo test` geçiyor.

## Out of Scope
- Diğer BindGroup'ların (texture bind groups) cachelanması.
- Shader pipeline değişiklikleri.
