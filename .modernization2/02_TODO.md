# WezTerm Modernizasyon 2 — İleri Seviye Optimizasyonlar

Bu belge, modernizasyon sürecinin ikinci etabını (`TIER 8` veya `Phase 3`) tanımlar.

## 1. Ölü Kod (Dead Code) ve SIMD Entegrasyonu
- [x] `vtparse/src/simd.rs` içindeki `find_first_escape` / `find_escape_avx2` fonksiyonlarının aktif `vtparse::Parser` veri yoluna entegre edilmesi.
- [x] `wezterm-surface/src/line/simd.rs` içindeki `lines_equal` fonksiyonlarının uygun yerlerde (ör. satır dirty hash kontrolü vb.) kullanılması.
- [x] `wezterm-escape-parser/src/simd_base64.rs` decode fonksiyonunun kitty image base64 operasyonlarıyla bağlanması.
- [x] Bu fonksiyonların mevcut sistemde tam olarak çağrıldığından ve AVX2 path'lerinin çalıştığından emin olmak.
- [ ] Daha fazla optimize edilebilecek alanların tespiti.

## 2. Gelişmiş Lints (Clippy Pedantic) ve Format
- [ ] Workspace genelinde `cargo fmt` çalıştırılarak standart formata geçilmesi.
- [ ] `cargo clippy --workspace -- -W clippy::pedantic` komutunun analiz edilmesi.
- [ ] En kritik (örn. gizli performans sorunları yaratan) pedantic lintlerin düzeltilmesi.

## 3. Senkronizasyon (Concurrency) İyileştirmeleri
- [ ] `std::sync::Mutex` ve `std::sync::RwLock` yapılarının performansı nerede kestiğinin analizi.
- [ ] Mümkün olan sıcak döngülerde (hot paths) `parking_lot::Mutex` veya `parking_lot::RwLock` crate'ine geçiş (daha önce bazı yerlerde kullanıldığı görülüyor, tam adaptasyon).
- [ ] Kanalların veya lock-free mimarilerin projeye uygunluğunun değerlendirilmesi.

## 4. Benchmark ve Test Kapsamı
- [ ] `criterion` ile entegre edilen SIMD kodları için karşılaştırmalı testler (scalar vs AVX2) oluşturulması.
- [ ] Termwiz ve Wezterm-gui render bileşenleri için wgpu/SIMD performans metriklerinin toplanması.
- [ ] Özellikle SIMD algoritmalarının uç (edge) senaryolar için test kapsamının genişletilmesi.
