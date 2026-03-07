# Plan: Benchmarking & Optimizing PTY/Render Sync

**Track ID:** `bench_pty_sync_20260307`
**Spec:** [spec.md](./spec.md)

---

## Phase 1: Metric Collection
- [ ] Task: Yoğun `cat /dev/urandom | base64` döngüsünde terminal UI tepkiselliğini ve FPS düşüşlerini ölç (tracer ile dump frame times)
- [ ] Task: PTY IO, Escape Parser ve GUI thread'leri arasındaki Lock (RwLock/Mutex) çekişmesini (contention) profille
- [ ] Task: Conductor - User Manual Verification 'Phase 1' (Protocol in workflow.md)

## Phase 2: Async and Locking Optimizations
- [ ] Task: Render thread'i bloke eden kritik kilitleri (critical sections) küçült veya lock-free/channel yapılarına geçir
- [ ] Task: Conductor - User Manual Verification 'Phase 2' (Protocol in workflow.md)

## Phase 3: Stress Testing
- [ ] Task: Yüksek IO altında `Ctrl+C` interrupt gecikmesini ölç ve iyileşmeyi dogrula
- [ ] Task: Conductor - User Manual Verification 'Phase 3' (Protocol in workflow.md)
