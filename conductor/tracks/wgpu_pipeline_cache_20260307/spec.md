# Spec: wgpu PipelineCache — Shader Disk Cache

## Overview

HekaTerm her açılışta WGSL shader'ını `naga` ile parse edip Vulkan SPIR-V'ye derliyor. `wgpu::PipelineCache` API'si ile derlenmiş pipeline binary'si diske yazılıp sonraki açılışlarda doğrudan yüklenebilir. Bu, startup latency'yi belirgin azaltır.

wgpu 0.20+ `wgpu::PipelineCache` desteğine sahip. Mevcut wgpu versiyon: `25.0.2`.

## Functional Requirements

### FR-1: PipelineCache oluşturma
```rust
// WebGpuState::new() içinde
let pipeline_cache_path = dirs_next::cache_dir()
    .map(|d| d.join("hekaterm").join("pipeline_cache.bin"));

let pipeline_cache = unsafe {
    device.create_pipeline_cache(&wgpu::PipelineCacheDescriptor {
        label: Some("HekaTerm Pipeline Cache"),
        data: pipeline_cache_path.as_ref()
            .and_then(|p| std::fs::read(p).ok())
            .as_deref(),
        fallback: true, // hata halinde cache'siz devam et
    })
};
```

### FR-2: render_pipeline oluştururken cache kullan
```rust
let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    // ...
    cache: pipeline_cache.as_ref(), // ← ekle
});
```

### FR-3: Pipeline cache'i diske kaydet
- Uygulama kapanırken (ya da pipeline oluşturulduktan sonra):
```rust
if let Some(path) = &pipeline_cache_path {
    if let Ok(data) = pipeline_cache.get_data() {
        std::fs::create_dir_all(path.parent().unwrap()).ok();
        std::fs::write(path, data).ok();
    }
}
```

### FR-4: Cache geçersizleştirme
- wgpu driver veya GPU değiştiğinde cache otomatik geçersiz kalır (wgpu garantisi).
- HekaTerm versiyon değişikliğinde cache silinecek: cache dosyasına versiyon prefix'i ekle.

### FR-5: Feature flag ile koru
- `[features]` altında `pipeline-cache` feature ekle (default: enabled).
- Disable edildiğinde `cache: None` kullanılır.

## Non-Functional Requirements
- İlk açılış (cache yok): mevcut ile eşdeğer süre.
- Sonraki açılışlar (cache var): shader compile süresi sıfır.
- Cache dosyası bozulursa: `fallback: true` ile sessizce devam et, log yaz.
- Cache dizini oluşturulamazsa: cache devre dışı bırak, crash etme.

## Acceptance Criteria
- [ ] `~/.cache/hekaterm/pipeline_cache.bin` ilk çalıştırma sonrası oluşuyor.
- [ ] İkinci çalıştırmada startup süresi ölçülebilir şekilde azalıyor.
- [ ] Cache dosyası silindiğinde uygulama normal çalışıyor.
- [ ] `cargo test` geçiyor.
- [ ] Bozuk cache ile başlatıldığında crash yok.

## Out of Scope
- Compute pipeline cache (render pipeline'a odaklanılıyor).
- Wayland/X11 backend değişiklikleri.
