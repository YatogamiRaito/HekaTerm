# Modernization 2

1. [x] Refactor code that uses `push_str(&format!(...))` to use `write!` instead, avoiding unnecessary string allocations.
2. [x] Research SIMD integration, missing parts, and optimize existing methods (`find_escape_avx2`, `vtparse`, `wezterm-surface`).
3. [x] Fix unresolved module `std` errors in `wezterm-surface` (use `core::arch` instead of `std::arch`).
4. [x] Clean up unused functions and modernise imports across the workspace.
