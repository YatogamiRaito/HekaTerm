# Benchmark Results: build_profile_20260307

**Date:** 2026-03-07  
**Track:** Release Build Profile Optimization

---

## Phase 1: Baseline vs Optimized (thin LTO)

| Metric | Baseline (opt-level=3 only) | Optimized (thin LTO + all opts) | Delta |
|--------|-----------------------------|---------------------------------|-------|
| Binary size (`wezterm-gui`) | 85 MB | 64 MB | **-25%** |
| Debug symbols (`nm \| wc -l`) | 130,657 | 0 | **-100%** |
| Startup time (mean) | 13.4ms ± 1.7ms | 10.6ms ± 1.5ms | **-21%** |

---

## Phase 2: thin LTO vs fat LTO Comparison

Both variants use: `codegen-units = 1`, `panic = "abort"`, `strip = "symbols"`

| Metric | `lto = "thin"` | `lto = "fat"` |
|--------|----------------|----------------|
| Incremental build time | **3m 00s** | 10m 40s |
| Startup time (mean) | 10.6ms ± 1.5ms | **9.8ms ± 1.8ms** |
| Binary size | 64 MB | 64 MB |

**Decision: `lto = "thin"` selected.**  
Rationale: fat LTO is 3.5x slower to build for only ~0.8ms startup improvement (~7%). 
The startup difference is within noise margin (overlapping σ ranges). Thin LTO gives excellent 
cross-crate optimization at a CI-friendly build time.

---

## Summary

Final release profile (`lto = "thin"`):
- Binary size reduced **85 MB → 64 MB** (−25%)
- Debug symbols eliminated (**130,657 → 0**)
- Startup time improved **13.4ms → 10.6ms** (−21%)
- Build time (full clean): ~9m 11s
- Build time (incremental): ~3m 00s
