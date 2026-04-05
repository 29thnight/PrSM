---
title: Introduction
nav_order: 1
has_children: true
permalink: /en/
---

# PrSM Documentation

PrSM is a Unity-first scripting language toolkit. It compiles `.prsm` source files into clean, readable C# for Unity projects — keeping your game logic concise while staying fully compatible with the Unity runtime.

## Why PrSM?

- **Concise syntax** — lifecycle methods, component lookups, and coroutines have first-class syntax instead of boilerplate
- **Strong null-safety** — `require`, `optional`, and `child` qualifiers let the compiler reason about field presence at compile time
- **Source-aware tooling** — `.prsmmap.json` sidecars let the VS Code extension and Unity editor map diagnostics and stack traces back to original `.prsm` lines
- **Readable output** — generated C# is formatted and structured so it is easy to read and debug
- **Unity-native** — no runtime overhead; the compiler produces plain C# that Unity builds and runs normally

## What is in the toolkit

| Component | Description |
|---|---|
| `crates/refraction` | Rust compiler core and `prism` CLI |
| `unity-package` | Unity Editor integration, import hooks, and source-map helpers |
| `vscode-prsm` | Syntax highlighting, diagnostics, LSP navigation, and snippets |
| `samples` | Sample `.prsm` files used for regression validation |

## Quick Links

- [Overview](overview.md)
- [Getting Started](getting-started.md)
- [Language Guide](language-guide.md)
- [Tooling](tooling.md)
