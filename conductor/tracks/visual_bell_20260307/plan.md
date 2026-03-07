# Plan: WebGPU Visual Bell Optimizasyonu

**Track ID:** `visual_bell_20260307`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: Context ve Analysis
- [ ] Task: WezTerm'in mevcut visual bell state yönetimini analiz et
    - [ ] `wezterm-gui/src/termwindow/render/pane.rs` içindeki `TODO: visual bell background layer` kısmını (saturation/hue vs) kontrol et.
    - [ ] WezTerm config `visual_bell` ayarının modelini (`config/src/config.rs`) incele.
- [ ] Task: WebGPU render pipeline'ı incele
    - [ ] Background'un nereye çizildiğini (`background_rect` vb.) ve hangi aşamada foreground'dan ayrıldığını belirle.
- [ ] Task: Conductor - User Manual Verification 'Phase 1' (Protocol in workflow.md)

---

## Phase 2: Render Layer Implementation
- [ ] Task: Quad/Triangle oluştur
    - [ ] Görsel zilin gösterileceği tüm aktif bölgeye tam oturan bir quad (2 ucu birleşik üçgen) ekle.
- [ ] Task: Renk ve Opasite bağlama
    - [ ] `visual_bell` animasyon state'ine göre hedeflenen renk ve opasiteyi bu quad'a aktar.
- [ ] Task: Blending ayarları
    - [ ] Görsel zil quad'ının alpha blending ile arka plana karışmasını, ancak foreground metinlerin üstüne (veya metne zarar vermeyecek şekilde altına) çizilmesini sağla.
- [ ] Task: Conductor - User Manual Verification 'Phase 2' (Protocol in workflow.md)

---

## Phase 3: Doğrulama
- [ ] Task: Birim testleri (eğer uygulanabiliyorsa) / config parse testlerini koştur.
- [ ] Task: Manuel doğrulama (Kullanıcıdan beklenecek).
- [ ] Task: Conductor - User Manual Verification 'Phase 3' (Protocol in workflow.md)

