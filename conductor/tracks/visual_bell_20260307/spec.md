# Spec: WebGPU Visual Bell Optimizasyonu

## Overview

HekaTerm'in WebGPU tabanlı render pipeline'ına geçişi sırasında, görsel zil (visual bell) geribildirimi şu an `pane.rs` içinde TODO olarak beklemektedir. Görsel zil, terminale `BEL` (Ctrl+G veya `\\a`) karakteri gönderildiğinde, işitsel bir ses yerine kısa süreliğine ekranın flash yapması (renk değiştirmesi) işlemidir. 

Bu özellik terminal kullanıcıları için önemlidir ve donanım hızlandırmalı WebGPU pipeline'ı içinde en düşük maliyetle (örneğin sadece ek bir shader pass veya tinted quad ile) uygulanmalıdır. 

## Functional Requirements

### FR-1: Visual Bell Rendering
- `Pane` içinde visual bell aktif olduğunda (bell animasyon süresi bitene kadar) terminal arka planının üzerine belirli bir opasitede bir quad çizilmeli.
- Quad'ın rengi yapılandırmada belirlenen `visual_bell` ayarlarına (config) göre alınmalı.
- Render pass'te arka plan çizimi ile foreground (yazı) çizimi arasına entegre edilmeli ki yazılar okunmaya devam etsin.

### FR-2: Configuration Support
- `wezterm.lua` içerisindeki mevcut WezTerm `visual_bell` ayarlarıyla uyumlu çalışmalı (örn: `audible_bell`, `visual_bell`).

### FR-3: Animation/Fading (Optional/Bonus)
- Yapılandırmada belirtildiyse, bell quad'ının opasitesi zamanla azalacak (fade-out) şekilde anime edilmeli.

## Acceptance Criteria
- [ ] Görsel zil aktif edildiğinde ekran belirtilen renkte/opasitede flash yapıyor.
- [ ] Metinler görsel zilin altında kaybolmuyor (doğru blending/z-index sağlanmış).
- [ ] Animasyon sorunsuz çalışıyor ve CPU/GPU'yu gereksiz meşgul etmiyor.
- [ ] Yeni render call (quad) mevcut draw call batching optimizasyonlarıyla çelişmiyor.

## Out of Scope
- İşitsel (Audio) bell implementasyonu.
- Terminal multiplexer tarafındaki zil durumu senkronizasyonu (varsayıyoruz ki GUI'ye event geliyor).
