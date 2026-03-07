# Plan: Benchmarking & Minimizing Latency

**Track ID:** `bench_latency_20260307`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: Baseline Measurements
- [ ] Task: Typing latency testi için altyapı kur (Typometer veya kernel düzeyinde tracing)
- [x] Task: `hyperfine` ile cold / warm startup time testlerini çalıştır ve sonuçları not et
- [x] Task: Conductor - User Manual Verification 'Phase 1' (Protocol in workflow.md)

## Phase 2: Code Profiling
- [x] Task: Window event loop üzerindeki input processing aşamalarını trace et (loglama/span eklentisi kullan)
- [x] Task: Conductor - User Manual Verification 'Phase 2' (Protocol in workflow.md)

## Phase 3: Optimizations & Validation
- [ ] Task: Belirlenen darboğazlar üzerinde kod optimizasyonu uygula
- [ ] Task: Değişiklikler sonrası benchmarkları tekrar et ve gelişimi tablolandır
- [ ] Task: Conductor - User Manual Verification 'Phase 3' (Protocol in workflow.md)
