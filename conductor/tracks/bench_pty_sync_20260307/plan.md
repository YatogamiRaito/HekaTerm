# Plan: Benchmarking & Optimizing PTY/Render Sync

**Track ID:** `bench_pty_sync_20260307`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: Metric Collection
- [x] Task: Yoğun `cat /dev/urandom | base64` döngüsünde terminal UI tepkiselliğini ve FPS düşüşlerini ölç (tracer ile dump frame times)
- [x] Task: PTY IO, Escape Parser ve GUI thread'leri arasındaki Lock (RwLock/Mutex) çekişmesini (contention) profille
- [x] Task: Conductor - User Manual Verification 'Phase 1' (Protocol in workflow.md)

## Phase 2: Async and Locking Optimizations
- [x] Task: Render thread'i bloke eden kritik kilitleri (critical sections) küçült veya lock-free/channel yapılarına geçir
- [x] Task: Conductor - User Manual Verification 'Phase 2' (Protocol in workflow.md)

## Phase 3: Stress Testing
- [x] Task: Yüksek IO altında `Ctrl+C` interrupt gecikmesini ölç ve iyileşmeyi dogrula
- [x] Task: Conductor - User Manual Verification 'Phase 3' (Protocol in workflow.md)
