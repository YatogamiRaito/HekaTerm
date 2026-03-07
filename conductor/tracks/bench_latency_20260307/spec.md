# Spec: Benchmarking & Minimizing Latency

## Overview
Terminal kullanıcısının fiziksel klavyede bir tuşa basması ile o karakterin ekranda ışık olarak (pixel) belirmesi arasında geçen süreyi (Typing Latency) ve programın açılış süresini (Startup Time) ölçüp optimize etmek.

## Functional Requirements
- **Typing Latency (Girdi Gecikmesi):** Klavyeden PTY'ye, oradan render engine'e kadar giden hattı (event loop) izole etmek ve gecikmeleri (buffer/wait süreleri) minimuma indirmek.
- **Startup Time (Açılış Süresi):** `hyperfine` aracıyla WezTerm'in terminal olarak başlatılıp ilk prompt'u verme süresinin profilini çıkarmak. PTY tahsisi ve pencere sisteminin başlatılmasındaki asenkron bloklamaları bulmak.

## Acceptance Criteria
- [ ] Girdi gecikmesi donanımsal ölçümler veya Typometer ile end-to-end (E2E) ölçülmüş ve belgelenmiş olmalı.
- [ ] Başlangıç süresinde (baseline) tespit edilen darboğazlar çözülmeli ve CI regresyon testine eklenmeli.
- [ ] Asenkron event-loop'taki gereksiz context switchler veya uykular (sleep) minimize edilmeli.
