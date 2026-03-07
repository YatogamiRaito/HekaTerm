# Spec: Release Build Profile Optimizasyonu

## Overview

HekaTerm'in `Cargo.toml` release profili yalnızca `opt-level = 3` içeriyor. LTO, codegen-units, panic stratejisi ve symbol stripping eksik. Bu eksiklikler:

- Crate sınırları arasında inlining yapılmamasına (LTO yok)
- Binary'ye 130.657 debug sembol gömülmesine (strip yok)
- Unwinding tablolarının binary'de yer kaplamasına (panic=abort yok)
- Paralel codegen'in optimizasyon kalitesini düşürmesine (çok codegen-unit)

yol açıyor. Beklenen genel performans kazanımı: **%10-20**.

## Functional Requirements

### FR-1: LTO
```toml
lto = "thin"
```
- `"thin"` LTO: crate-arası inlining sağlar, build süresi makul kalır.
- `"fat"` ile kıyasla benchmark yapılacak; eğer build süresi kabul edilebilirse `"fat"` tercih edilecek.

### FR-2: codegen-units
```toml
codegen-units = 1
```
- Tek codegen unit ile LLVM daha agresif optimize eder.
- LTO ile birlikte en iyi sonucu verir.

### FR-3: panic = "abort"
```toml
panic = "abort"
```
- Unwinding tabloları binary'den çıkar.
- `anyhow` ve `thiserror` ile uyumlu; propagation `?` ile yapılıyor zaten.

### FR-4: strip = "symbols"
```toml
strip = "symbols"
```
- Release binary'deki 130K debug sembolü kaldırır.
- Binary boyutu 85 MB → ~73 MB'a düşer.

### FR-5: Bağımlılıklar için opt-level
```toml
[profile.release.package."*"]
opt-level = 3
```
- Tüm bağımlılık crate'leri de `opt-level = 3` ile derlenir (bazıları varsayılan 0 veya 1 ile derleniyor olabilir).

## Non-Functional Requirements
- Release build süresi 15 dakikayı geçmemeli (LTO ile uzayabilir, CI cache önemli).
- Tüm testler `cargo test --release` ile geçmeli.
- `cargo clippy -- -D warnings` temiz kalmalı.

## Acceptance Criteria
- [ ] `cargo build --release` başarıyla tamamlanıyor.
- [ ] `cargo test --release` tüm testler geçiyor.
- [ ] `nm target/release/wezterm-gui | wc -l` → 0 (sembol yok).
- [ ] Binary boyutu ≤ 74 MB (stripped).
- [ ] `hyperfine` ile startup time benchmark: sistem versiyonuna eşit veya daha hızlı.
- [ ] Render throughput benchmark: önceki release ile eşit veya daha hızlı.

## Out of Scope
- Profile-Guided Optimization (PGO) — ayrı bir track olarak değerlendirilebilir.
- MSRV değişikliği.
- Bağımlılık kaldırma veya değiştirme.
