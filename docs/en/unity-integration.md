---
title: Unity Integration
parent: Tooling
grand_parent: English Docs
nav_order: 2
---

# Unity Integration

The `unity-package` folder contains a Unity Editor package (`com.prsm.editor`) that connects PrSM source files to the Unity project workflow. There is no runtime overhead — the compiler produces plain C# that Unity builds and runs normally.

## How it works

When a `.prsm` file is saved or imported, the package invokes the `prism build` pipeline and places generated `.cs` files in the configured output directory. Unity picks those up through its normal script compilation step.

```
.prsm source files
        │
        ▼
  prism build
        │
        ├──► generated .cs   ──► Unity script compilation ──► runtime
        └──► .prsmmap.json   ──► editor tooling (diagnostics, navigation)
```

## Import and compile workflow

1. `MoonAssetPostprocessor` detects `.prsm` changes via `OnPostprocessAllAssets`
2. It resolves the `prism` binary — preferring a local workspace dev build over the extension-bundled path
3. `prism build` regenerates affected `.cs` files and `.prsmmap.json` sidecars
4. Unity recompiles the updated scripts normally

## Diagnostics

Errors from generated `.cs` files are remapped to the original `.prsm` line and column via `.prsmmap.json`. Unity Console messages show `.prsm` paths and are double-clickable to the correct source location.

## Runtime stack trace remapping

`MoonStackTraceFormatter` intercepts `Application.logMessageReceived` and rewrites any stack frames that point at generated C# back to their original `.prsm` file and line number. Both Unity-style `(at path:line)` and .NET-style `in path:line` frames are handled. The original trace is preserved alongside the remapped version so nothing is lost.

## Script navigation

When Unity tries to open a generated `.cs` file (for example, from a double-clicked Console error), `MoonScriptProxy` and `MoonScriptRedirector` intercept the request and redirect it to the original `.prsm` source using the `.prsmmap.json` anchor map.

## Project settings

Settings are accessible under **Edit → Project Settings → PrSM**:

- Active `.prsmproject` path
- Output directory override
- Compiler binary path override
- Auto-compile on save toggle

## Templates

Starter templates are available via **Assets → Create → PrSM**. They create minimal scaffolds for `component`, `asset`, and `class` declarations.
