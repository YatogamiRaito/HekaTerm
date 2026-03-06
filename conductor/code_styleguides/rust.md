# Rust Code Style Guide

## Formatting
- **Always** run `cargo fmt` before committing. CI enforces this.
- Line length: 100 characters (rustfmt default).
- Trailing commas in multi-line expressions: required.

## Naming Conventions
- Types, traits, enums: `PascalCase`
- Functions, methods, variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Crate-level modules: `snake_case`
- Lifetimes: short, single-letter (`'a`, `'b`) unless clarity demands more.

## Clippy
Run with `-D warnings` — all warnings are errors:
```bash
cargo clippy -- -D warnings
```
Common rules enforced:
- `clippy::unwrap_used` — use `?`, `expect("reason")`, or proper error handling
- `clippy::expect_used` — `expect` must have a descriptive reason string
- `clippy::panic` — panics forbidden in library code
- `clippy::todo` — no `todo!()` in committed code

## Error Handling
- Use `anyhow::Result` for application-level errors in binaries.
- Use `thiserror` for library-level typed errors.
- Never use `unwrap()` in production paths. Use `?` or `.expect("clear reason")`.
- Never use `panic!()` in library crates.

## Unsafe Code
```rust
// SAFETY: <explain the invariant being upheld and why this is sound>
unsafe { ... }
```
- Every `unsafe` block requires a `// SAFETY:` comment immediately above.
- Minimize the scope of `unsafe` blocks to the smallest possible unit.
- Prefer safe abstractions; only use `unsafe` when no safe alternative exists.

## Async
- Use `smol` or `async-std` patterns consistent with the existing codebase.
- Do not introduce `tokio` unless the existing async runtime changes.
- Avoid blocking calls inside async contexts.

## Documentation
- All `pub` items must have `///` doc comments.
- Include an example in doc comments for non-trivial public APIs.
- Module-level `//!` doc comments required for every `pub mod`.

## Performance
- Avoid allocations in hot paths (PTY parsing, render loop).
- Prefer `&str` over `String`, `&[T]` over `Vec<T>` in function signatures.
- Use `smallvec` or stack allocation where the size is known and small.
- Profile with `cargo flamegraph` or `perf` before claiming a performance improvement.

## Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        // arrange
        // act
        // assert
    }
}
```
- Tests in the same file as the code under test.
- One logical assertion per test where possible.
- Test names describe behavior: `test_render_batch_reduces_draw_calls`, not `test1`.
