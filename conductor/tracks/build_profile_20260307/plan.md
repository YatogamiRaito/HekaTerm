# Plan: Release Build Profile Optimizasyonu

**Track ID:** `build_profile_20260307`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: Baseline ve Profil Güncellemesi

- [x] Task: Mevcut binary boyutu ve startup baseline'ını kaydet
    - [x] `ls -lh target/release/wezterm-gui` → **85 MB**
    - [x] `hyperfine --warmup 3 --runs 10 'target/release/wezterm-gui --version'` → **13.4ms ± 1.7ms**
    - [x] `nm target/release/wezterm-gui | wc -l` → **130,657 symbols**
- [x] Task: `Cargo.toml` release profilini güncelle
    - [x] `lto = "thin"` eklendi
    - [x] `codegen-units = 1` eklendi
    - [x] `panic = "abort"` eklendi
    - [x] `strip = "symbols"` eklendi
    - [x] `[profile.release.package."*"]` ile bağımlılıklar için `opt-level = 3` eklendi
- [x] Task: `cargo build --release` ile derleme yap ve hataları gider → 9m 11s, exit 0
- [x] Task: `cargo clippy -- -D warnings` çalıştır, tüm uyarıları gider
    - Fixd: unnecessary `as u64` cast (`max_fps`)
    - Fixed: `map_or(true, ...)` → `is_none_or(...)`
- [x] Task: Commit: 447b12dad `chore(build): Enable LTO, single codegen unit, panic=abort, strip symbols`
    - Post-build metrics: binary **64 MB** (-25%), symbols **0**, startup **10.6ms ± 1.5ms** (-21%)
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Baseline ve Profil Güncellemesi' (Protocol in workflow.md)

---

## Phase 2: LTO Strateji Karşılaştırması

- [x] Task: `thin` vs `fat` LTO benchmark
    - [x] `lto = "thin"` ile build süresi → **3m 00s** (incremental), startup **10.6ms ± 1.5ms**
    - [x] `lto = "fat"` ile build süresi → **10m 40s** (incremental), startup **9.8ms ± 1.8ms**
    - [x] Sonuçları `benchmark_results.md`'e kaydet
- [x] Task: Optimum LTO seçeneğini uygula → `thin` seçildi (3.5x hızlı build, marjinal startup farkı)
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
