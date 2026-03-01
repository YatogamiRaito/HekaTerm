# Modernization Phase 2 - TODO

Bu doküman, WezTerm projesi için planlanan ikinci modernizasyon fazının (Performans ve İdiomatik Rust) adımlarını listeler.

## Hedefler

- [ ] **T2-1: HashMap Optimizasyonu:** `std::collections::HashMap` varsayılan olarak yavaş (fakat güvenli) olan `SipHash` kullanır. Terminal içi veri yapıları (hücre cache'leri, font cache'leri, mux session listeleri) gibi hızın çok kritik olduğu (ama HashDoS saldırısına kapalı olan) yerlerde `rustc-hash` (`FxHashMap`) veya `ahash` kullanımına geçilmesi.
    - Tespit edilen hedefler: `term/src/terminalstate/kitty.rs`, `mux/src/pane.rs`, `mux/src/tab.rs`, `wezterm-gui/src/glyphcache.rs`
- [ ] **T2-2: Mutex -> RwLock/Atomic Dönüşümleri:** `std::sync::Mutex` read-heavy (çok fazla okunan, az yazılan) veriler için gereksiz kilitlenmelere ve thread beklemelerine yol açar. Bunları `std::sync::RwLock`, `parking_lot::RwLock` veya `ArcSwap` ile değiştirmek.
    - Tespit edilen hedefler: `wezterm-surface/src/line/line.rs` (appdata payload'ı), `mux/src/tmux.rs`, `wezterm-gui/src/termwindow/mod.rs` (frame state'leri veya ortak font durumları).
- [ ] **T2-3: Gereksiz Allocation / Clone Tespitleri:** `String` yaratmak yerine `&str` ya da `Cow<str>` kullanılabilecek yerlerin bulunup düzeltilmesi.
- [ ] **T2-4: Modern Iterators:** `for` döngüleriyle manuel filtreleme/map etme yapılan yerlerin idiomatik Rust iterator zincirlerine (chain) dönüştürülmesi.

Doğrulama, bir önceki aşamada olduğu gibi `cargo check`, `cargo test` ve `cargo bench` komutlarıyla yapılacaktır.
