# Modernization Phase 2 - Analysis & Planning

## Hedefler
- **Performans:** CPU cycle israf eden, aşırı bellek allocation'ı (tahsisatı) yapan veya yavaş kilit (Mutex vb.) kullanan eski kodları daha verimli yapılarla değiştirmek.
- **Modern Rust Pratikleri:** Daha güncel Rust özelliklerini (örn. const generics, let-else, Iterator yöntemleri) ve deyimlerini (idioms) kullanmak.
- **Kütüphane Optimizasyonları:** Daha hızlı alternatifleri olan (ahash/rustc-hash vs std::collections::HashMap) bağımlılıkların kullanımını tespit etmek ve adapte etmek.
- **Asenkron/Concurrency İyileştirmeleri:** Gereksiz thread bloklamalarını kaldırmak, RwLock kullanım alanlarını genişletmek.

## Araştırma Planı
1. **Allocation Hotspots:** Term logları ve ekran güncellemeleri (wezterm-term, wezterm-surface) içinde sık yapılan allocation'ları `String` -> `&str` veya `Cow` geçişleri için incele.
2. **Koleksiyonlar (Collections):** `std::collections::HashMap` gibi default yavaş hash'leme algoritmaları yerine fxhash/rustc-hash/ahash entegrasyonlarını kontrol et.
3. **Senkronizasyon (Synchronization):** Kod boyunca `std::sync::Mutex` kullanım noktaları. Terminalin read-heavy durumları için kilit çekişmesi (contention) yaratan yerlerde `RwLock` ya da `ArcSwap` / `Atomic` kullanım fırsatları.
4. **I/O Overhead:** wezterm-ssh, mux, pty gibi kanallardaki buffer kopyalama sayıları.

Çıkan sonuçlara göre detaylı bir `01_TODO.md` eklenecektir.
