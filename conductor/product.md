# HekaTerm — Product Guide

## Initial Concept

HekaTerm is a GPU-accelerated, cross-platform terminal emulator and multiplexer written in Rust. It is a modernized fork of WezTerm with a WebGPU-first rendering architecture, replacing the legacy Glium/EGL/WGL pipeline with `wgpu` (Vulkan / Metal / DX12 backends).

---

## Vision

Build the fastest, most maintainable GPU-accelerated terminal emulator for power users and developers — with zero legacy rendering overhead and a clean, modern Rust codebase.

## Target Users

- **Developers / Power users** — keyboard-driven, terminal-first workflows
- **Sysadmins / DevOps** — SSH multiplexing, persistent sessions, remote pane management
- **Open source contributors** — Rust community members who want a modern, auditable codebase

## Core Differentiators

| Feature | HekaTerm | WezTerm |
|---|---|---|
| Rendering backend | **wgpu (WebGPU)** | Glium (OpenGL/EGL) |
| GPU pipeline | Vulkan / Metal / DX12 | OpenGL / EGL |
| Legacy deps | None | libwayland-egl, libGL |
| Shader format | WGSL | GLSL |

## Primary Platforms

- **Linux** — X11 and Wayland (primary development target)
- **macOS** — Metal GPU pipeline
- **Windows** — DirectX 12 / ANGLE

## Key Features (Inherited + Enhanced)

- GPU-accelerated text rendering via wgpu + custom WGSL shaders
- Lua-based configuration and scripting (mlua)
- SSH multiplexer with persistent sessions
- Unicode / BiDi text support
- Tab, pane, and window management
- Ligature and font fallback support (FreeType + Cairo)
- Drop-in WezTerm configuration compatibility (Lua API)

## Non-Goals

- Browser/Electron-based UI
- Python/JS scripting (Lua only)
- Wayland EGL dependency
