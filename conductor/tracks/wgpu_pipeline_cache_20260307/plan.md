# Plan: wgpu PipelineCache — Shader Disk Cache

**Track ID:** `wgpu_pipeline_cache_20260307`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: PipelineCache Altyapısı

- [ ] Task: wgpu 25.0.2'de `PipelineCache` API'sini incele
    - [ ] `wgpu::PipelineCacheDescriptor` struct'ının alanlarını incele
    - [ ] `device.create_pipeline_cache()` ve `pipeline_cache.get_data()` API'lerini doğrula
    - [ ] Platform desteği: Vulkan (Linux), Metal (macOS), DX12 (Windows) — hepsinde destekleniyor mu?
- [ ] Task: Cache dizin yolunu belirle
    - [ ] `dirs_next::cache_dir()` kullanarak `~/.cache/hekaterm/` yolunu oluştur
    - [ ] Cache dosya adına versiyon prefix'i ekle: `pipeline_cache_v{VERSION}.bin`
- [ ] Task: `WebGpuState` struct'ına `pipeline_cache: Option<wgpu::PipelineCache>` ekle
- [ ] Task: `WebGpuState::new()` içinde cache'i yükle veya oluştur
    - [ ] Dosyadan yükle: `std::fs::read(path).ok()` → `PipelineCacheDescriptor { data: ... }`
    - [ ] `fallback: true` — yükleme başarısız olsa bile devam et
    - [ ] Hata durumunda `log::warn!` yaz, crash etme
- [ ] Task: `cargo clippy -- -D warnings` ve `cargo fmt` çalıştır
- [ ] Task: Commit: `feat(render): Add wgpu PipelineCache infrastructure to WebGpuState`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: PipelineCache Altyapısı' (Protocol in workflow.md)

---

## Phase 2: Pipeline Oluşturmada Cache Kullanımı

- [ ] Task: `create_render_pipeline()` çağrısına `cache` parametresini ekle
    - [ ] `wgpu::RenderPipelineDescriptor { ..., cache: self.pipeline_cache.as_ref() }`
- [ ] Task: Pipeline oluşturulduktan sonra cache'i diske yaz
    - [ ] `pipeline_cache.get_data()` → `std::fs::write(path, data)`
    - [ ] Dizin yoksa `create_dir_all` ile oluştur
    - [ ] Yazma hatası: log yaz, panic etme
- [ ] Task: Birim test yaz
    - [ ] Cache dosyası pipeline oluşturulduktan sonra var mı?
    - [ ] Bozuk cache verisi ile başlatıldığında crash yok mu?
- [ ] Task: `cargo clippy -- -D warnings` ve `cargo fmt` çalıştır
- [ ] Task: Commit: `feat(render): Use PipelineCache in render pipeline creation and persist to disk`
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Pipeline Cache Kullanımı' (Protocol in workflow.md)

---

## Phase 3: Startup Benchmark ve Doğrulama

- [ ] Task: İlk çalıştırma startup süresi ölç (cache yok)
- [ ] Task: İkinci çalıştırma startup süresi ölç (cache var)
    - [ ] Fark `hyperfine` ile ölçülecek
    - [ ] Sonuçları `benchmark_results.md`'e kaydet
- [ ] Task: `cargo test` çalıştır
- [ ] Task: Cache dizinini silerek başlatma doğrula (graceful fallback)
- [ ] Task: Bozuk cache (rastgele byte'lar) ile başlatma doğrula
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Doğrulama' (Protocol in workflow.md)
