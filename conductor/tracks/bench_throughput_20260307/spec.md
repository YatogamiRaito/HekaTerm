# Spec: Benchmarking & Optimizing Throughput

## Overview
Terminalin bir saniyede ekrana basabildiği veri miktarını (throughput) ölçmek ve olası darboğazları (bottleneck) tespit edip düzeltmek.

## Functional Requirements
- **ASCII Throughput:** Düz font, renk kodu içermeyen hızlı metin akışında maksimum performansı bulmak (`cat` veya `vtebench` ile).
- **ANSI Color Throughput:** Sık renk değiştiren escape dizilimlerini (`\x1b[31m` vb.) işlerken parser'ın ve render'ın yavaşlamamasını sağlamak.
- **Unicode / BiDi Throughput:** HarfBuzz ile şekillendirilen (shaping) emoji, ligatür ve RTL (Arapça/İbranice vb.) metinlerin performansını artırmak.

## Acceptance Criteria
- [ ] Üç kategori için de (ASCII, ANSI, Unicode) baseline (başlangıç) benchmark sonuçları alınmalı.
- [ ] Piyasada öne çıkan (örneğin Alacritty, Kitty) alternatiflerle kıyaslanarak geride kalınan noktalar raporlanmalı.
- [ ] Profiling (örn. `perf` veya `flamegraph`) yapılarak en yavaş fonksiyonlar (hot path) bulunmalı ve plan oluşturulmalı.
