# HekaTerm — Product Guidelines

## Prose & Documentation Style

- **Tone:** Technical, concise, direct. No filler. No over-explanation.
- **Language:** English only for code, comments, and docs.
- **Comments:** Only where logic is non-obvious. Self-documenting code is preferred.
- **Doc comments (`///`):** Required on all public API items.

## Code Quality Standards

All of the following are mandatory and enforced in CI:

### Rust
- **`cargo clippy -- -D warnings`** — zero clippy warnings allowed; all warnings are errors.
- **`cargo fmt --check`** — unformatted code is rejected. Run `cargo fmt` before committing.
- **`unsafe` blocks** — forbidden unless strictly necessary. Every `unsafe` block must have:
  - A `// SAFETY:` comment explaining the invariant being upheld.
  - A corresponding issue or PR reference if it's a known limitation.
- **Dependencies** — always use the most recent stable version. Prefer well-maintained crates with active development. Avoid crates with no recent commits or deprecated status.
- **Performance** — prefer zero-copy, allocation-free paths in hot loops (rendering, PTY parsing). Profile before optimizing.
- **Efficiency** — minimize GPU draw calls, batch operations where possible, avoid redundant state changes in wgpu pipelines.

### Testing
- **Unit tests** required for every new module.
- Integration tests required for any feature that touches the PTY, mux, or SSH layers.
- Test coverage tracked; regressions are not acceptable.

## Commit Convention

Format: **Conventional Commits**

```
<type>(<scope>): <short description>

[optional body]
```

**Types:** `feat`, `fix`, `refactor`, `perf`, `test`, `docs`, `chore`, `build`, `ci`

**Scopes:** `wezterm-gui`, `wezterm-mux`, `wezterm-ssh`, `config`, `font`, `render`, `conductor`

**Examples:**
```
feat(render): Replace GLSL shaders with WGSL for wgpu pipeline
fix(font): Correct glyph cache invalidation on DPI change
perf(render): Batch quad uploads to reduce draw call count
refactor(wezterm-gui): Remove Glium backend and EGL initialization
```

- Subject line: max 72 characters, imperative mood, no period.
- Breaking changes: append `!` after scope and add `BREAKING CHANGE:` in body.

## Dependency Policy

- Prefer `wgpu` over any OpenGL/EGL abstraction.
- Do not reintroduce `glium`, `glutin`, or `egl` crates.
- Keep `mlua` as the single Lua runtime.
- Regularly run `cargo update` and `cargo audit`.

## Architecture Principles

- **Crate boundaries matter.** Keep rendering in `wezterm-gui`, multiplexer in `wezterm-mux`, SSH in `wezterm-ssh`. Cross-crate dependencies must be justified.
- **No God objects.** Break large structs and impls into focused, single-responsibility units.
- **Async over threads** where latency matters (SSH, network I/O).
