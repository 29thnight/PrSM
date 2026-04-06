---
title: Tooling
parent: Introduction
nav_order: 4
---

# Tooling

PrSM ships as a complete development toolkit: compiler, editor support, and Unity integration.

## Components

| Tool | Description | Details |
|------|-------------|---------|
| `prism` CLI | Compiler, checker, LSP server, project tools | [CLI Reference](cli.md) |
| Unity Package | ScriptedImporter, inspectors, stack trace remap | [Unity Integration](unity-integration.md) |
| VS Code Extension | Syntax, diagnostics, LSP, navigation, snippets | [VS Code Extension](vscode-extension.md) |
| Source Maps | `.prsmmap.json` bidirectional mapping | [Generated C# & Source Maps](generated-csharp-and-source-maps.md) |

## VS Code extension features

The extension activates for `.prsm` files and provides:

**Language features (via LSP):**
- Real-time diagnostics (errors and warnings as you type)
- Go-to-definition (Ctrl+Click or F12)
- Find all references (Shift+F12)
- Hover information (type info + generated C# details)
- Rename symbol (F2)
- Document and workspace symbols (Ctrl+Shift+O)
- Code actions (explicit generic type arguments, organize imports)
- Completion (Unity API + user symbols + keywords)

**Editor features:**
- TextMate syntax highlighting (55 scopes)
- 20+ code snippets (component, lifecycle, listen, coroutine, etc.)
- Lifecycle block insertion (Ctrl+Shift+L)
- PrSM Explorer sidebar (file tree)
- Dependency graph view (Ctrl+Shift+G)

**Navigation:**
- Jump to generated C# (Ctrl+Shift+G on a symbol)
- Jump from generated C# back to `.prsm` source
- Stack trace navigation (Ctrl+Shift+T) — click remapped stack frames

**Keybindings:**

| Shortcut | Action |
|----------|--------|
| Ctrl+Shift+G | Show generated C# |
| Ctrl+Shift+V | Graph view |
| Ctrl+Shift+L | Insert lifecycle |
| Ctrl+Shift+T | Open from stack trace |

**Settings:**

| Setting | Default | Description |
|---------|---------|-------------|
| `prsm.compilerPath` | `""` (auto-detect) | Path to `prism` binary |
| `prsm.checkOnSave` | `true` | Run diagnostics on save |
| `prsm.showWarnings` | `true` | Show warning-level diagnostics |
| `prsm.unityApiDbPath` | `""` (bundled) | Path to Unity API SQLite database |

## Installation methods

See [Getting Started](getting-started.md) for full installation instructions including MSI, winget, and source build options.
