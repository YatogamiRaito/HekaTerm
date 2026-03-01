# Modernization Phase 3 - Todo

- [ ] 1. Sık kullanılan senkron kanalların (`std::sync::mpsc`) asenkron veya hızlı (`flume`/`crossbeam`) alternatiflerle değiştirilmesi.
- [ ] 2. Clippy ile `clippy::manual_let_else` analizinin yapılıp uygun kodların 2021 edition pratiklerine uyarlanması.
- [ ] 3. I/O operasyonlarındaki `Vec<u8>` kopyalamalarının tespit edilip `Bytes` veya referanslarla (zero-copy) değiştirilmesi (`mux`, `wezterm-ssh`).
- [ ] 4. Cargo bağımlılıklarının kontrol edilmesi ve gereksiz ağırlık yapanların temizlenmesi.
