# Modernization Phase 3 - Context

## Hedefler (Goals)
1. **Sıfır Kopya (Zero-Copy) & I/O Optimizasyonları:**
   Veri akışında (I/O operasyonlarında) gereksiz buffer kopyalamalarını engellemek, `&[u8]`, `Bytes` veya `Cow` üzerinden akışı hızlandırmak. Ana hedefler: `wezterm-ssh`, `mux`, `pty`.
2. **Bağımlılık (Dependency) ve Sürüm Modernizasyonu:**
   `Cargo.toml` altındaki eski veya ağır crate (paket) sürümlerini güncellemek. Gerekirse `flume` veya `crossbeam` gibi alternatif kanal (channel) yapılarına geçmek.
3. **Gelişmiş Pattern Matching (let-else) ve Typestate Pattern:**
   `match` ve `if let` bloklarını `let ... else` yapısıyla düzleştirerek (flattening) kod okunabilirliğini artırmak.
4. **Async & Concurrency Altyapı İncelemesi (Tokio / Channels):**
   Uygulama genelinde CPU'yu bloke eden senkron kanalları (`std::sync::mpsc`) asenkron yapılara dönüştürmek.

## Araştırma Adımları
1. Clippy aracılığıyla `manual_let_else` denetimi yapmak.
2. `std::sync::mpsc` ve benzeri bloke eden yapıları bulup analiz etmek.
3. Network ve I/O üzerindeki ağır kopyalama operasyonlarını bulmak.
