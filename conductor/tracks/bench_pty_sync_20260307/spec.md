# Spec: Benchmarking & Optimizing PTY/Render Sync

## Overview
Terminalin PTY (Pseudo-Terminal) üzerinden gelen devasa veri akışını işlerken asıl UI/Render thread'ini boğmaması ve (örneğin ekranda cat komutu dönerken) tutarlı bir framerate (örn: 60 FPS) suması.

## Functional Requirements
- **FPS Kararlılığı (Frame Stability):** Saniyede MB seviyesinde log aktığında terminal ekranının V-Sync veya monitör yenileme hızına (hz) paralel olarak yırtılmadan çizilmesini sağlamak.
- **Drop Frames:** CPU üzerinde parser %100 meşgulken bile UI/Render engine'nin (WebGPU) kilitlenmeden, "frame drop" yapsa dahi tepkisel (responsive) kalabilmesi.

## Acceptance Criteria
- [ ] Yüksek PTY throughput testlerinde render thread'inin bloke olmadığı (`Ctrl+C` gibi sinyallerin anlık iletildiği) doğrulanacak.
- [ ] Gerekirse PTY okuma ve parse işlemleri ile GPU render (frame present) adımları arasındaki mutex/lock çekişmelerinin (contention) profili çıkarılacak.
- [ ] Frame present süresinin aşırı gecikmeleri (frame spikes) saptanacak ve çözülecek.
