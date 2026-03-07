# Plan: Release Build Profile Optimizasyonu

**Track ID:** `build_profile_20260307`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: Baseline ve Profil Güncellemesi

- [ ] Task: Mevcut binary boyutu ve startup baseline'ını kaydet
    - [ ] `ls -lh target/release/wezterm-gui` çıktısını not et
    - [ ] `hyperfine --warmup 5 --runs 20 'target/release/wezterm --version'` çalıştır, kaydet
    - [ ] `nm target/release/wezterm-gui | wc -l` sembol sayısını kaydet
- [ ] Task: `Cargo.toml` release profilini güncelle
    - [ ] `lto = "thin"` ekle
    - [ ] `codegen-units = 1` ekle
    - [ ] `panic = "abort"` ekle
    - [ ] `strip = "symbols"` ekle
    - [ ] `[profile.release.package."*"]` ile bağımlılıklar için `opt-level = 3` ekle
- [ ] Task: `cargo build --release` ile derleme yap ve hataları gider
- [ ] Task: `cargo clippy -- -D warnings` çalıştır, tüm uyarıları gider
- [ ] Task: Commit: `chore(build): Enable LTO, single codegen unit, panic=abort, strip symbols`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Baseline ve Profil Güncellemesi' (Protocol in workflow.md)

---

## Phase 2: LTO Strateji Karşılaştırması

- [ ] Task: `thin` vs `fat` LTO benchmark
    - [ ] `lto = "thin"` ile build süresi ölç (`time cargo build --release`)
    - [ ] `lto = "fat"` ile build süresi ölç
    - [ ] Her iki variant için `hyperfine` startup benchmark çalıştır
    - [ ] Render throughput benchmark çalıştır (aynı `/tmp/bench_render.sh` script)
    - [ ] Sonuçları `benchmark_results.md`'e kaydet
- [ ] Task: Optimum LTO seçeneğini uygula ve Cargo.toml'u güncelle
- [ ] Task: Commit: `chore(build): Select optimal LTO strategy based on benchmarks`
- [ ] Task: Conductor - User Manual Verification 'Phase 2: LTO Strateji Karşılaştırması' (Protocol in workflow.md)

---

## Phase 3: Doğrulama

- [ ] Task: `cargo test --release` çalıştır — tüm testler geçmeli
- [ ] Task: Binary boyutunu doğrula
    - [ ] `ls -lh target/release/wezterm-gui` → ≤ 74 MB bekleniyor
    - [ ] `nm target/release/wezterm-gui | wc -l` → 0 bekleniyor
- [ ] Task: Startup benchmark karşılaştır (sistem vs HekaTerm)
- [ ] Task: Render throughput karşılaştır (önceki release vs yeni)
- [ ] Task: Sonuçları `benchmark_results.md`'e kaydet
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Doğrulama' (Protocol in workflow.md)
