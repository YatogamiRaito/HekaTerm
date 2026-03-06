# HekaTerm — Tech Stack

## Language

| Item | Detail |
|---|---|
| Language | **Rust** (stable toolchain) |
| Edition | 2021 |
| Build system | Cargo workspace (`resolver = "2"`) |
| MSRV | Follows wgpu's MSRV (currently ~1.70) |

## Architecture

**Monorepo** — Cargo workspace with the following primary crates:

| Crate | Role |
|---|---|
| `wezterm-gui` | Main GUI process, WebGPU rendering, window management |
| `wezterm-mux-server` | Multiplexer daemon |
| `wezterm-ssh` | SSH client and remote PTY |
| `wezterm-surface` | Terminal surface, cell grid, damage tracking |
| `wezterm-escape-parser` | VT/ANSI escape sequence parser |
| `config` | Configuration types and Lua API surface |
| `wezterm-font` | Font loading, shaping, atlas management |
| `termwiz` | Terminal widget library |
| `codec` | Mux network protocol (de)serialization |
| `bidi` | Unicode BiDi algorithm |
| `wezterm-cell` | Terminal cell representation |

## Rendering

| Item | Detail |
|---|---|
| GPU API | **wgpu** (WebGPU abstraction) |
| Backends | Vulkan (Linux), Metal (macOS), DX12 (Windows) |
| Shader format | **WGSL** (`shader.wgsl`) |
| Font rendering | FreeType + Cairo |
| Glyph atlas | Custom GPU texture atlas (`glyphcache.rs`) |
| Quad pipeline | Custom vertex/fragment pipeline (`quad.rs`) |

## Windowing & Input

| Item | Detail |
|---|---|
| Window system | `winit` (cross-platform) |
| Wayland | `wayland-client`, `wayland-protocols` |
| X11 | `xcb`, `x11` |
| Keyboard | `xkbcommon`, `xkbcommon-x11` |

## Scripting & Configuration

| Item | Detail |
|---|---|
| Scripting runtime | **mlua** (Lua 5.4) |
| Config format | Lua (`.wezterm.lua`) + TOML (internal) |

## Networking / SSH

| Item | Detail |
|---|---|
| SSH | Custom `wezterm-ssh` crate (async, via `smol`) |
| Async runtime | `smol` + `async-io` |
| TLS | `openssl` (async via `async_ossl`) |

## CI / Tooling

| Item | Detail |
|---|---|
| CI | GitHub Actions |
| Lint | `cargo clippy -- -D warnings` |
| Format | `cargo fmt` |
| Audit | `cargo audit` |
| Packaging | Flatpak (Linux), `.app` bundle (macOS), NSIS (Windows) |

## Removed Dependencies

The following were removed during modernization and must **not** be reintroduced:

- `glium` — OpenGL rendering backend
- `glutin` — OpenGL context creation
- `libwayland-egl` — EGL Wayland surface
- GLSL shaders — replaced by WGSL
