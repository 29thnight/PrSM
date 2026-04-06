---
title: Getting Started
parent: Introduction
nav_order: 2
---

# Getting Started

## Installation

### Option A: MSI Installer (Windows, recommended)

Download the latest `.msi` from [GitHub Releases](https://github.com/29thnight/PrSM/releases) and run it. The installer:
- Installs `prism.exe` to `C:\Program Files\PrSM\`
- Adds the install directory to system PATH
- Installs the VS Code extension automatically (if VS Code is detected)
- Shows Unity package install instructions on completion

### Option B: winget (Windows)

```powershell
winget install PrSM.PrSM
```

### Option C: Build from source

Prerequisites:
- [Rust toolchain](https://rustup.rs/) (stable)
- Node.js 20+ and npm (for the VS Code extension)

```powershell
cargo build --release -p refraction
```

The compiler binary is at `target/release/prism.exe` (Windows) or `target/release/prism` (macOS/Linux).

For the VS Code extension:

```powershell
cd vscode-prsm
npm install
npm run bundle    # bundles extension + copies prism binary
npm run package   # creates .vsix
```

Install the `.vsix` via **Extensions > Install from VSIX** in VS Code.

## First steps

### 1. Verify installation

```powershell
prism version
```

Expected output: `prism 0.1.0` (or similar).

### 2. Initialize a project

```powershell
cd MyUnityProject
prism init
```

This creates a `.prsmproject` file. See [Project Configuration](project-configuration.md) for all options.

### 3. Write your first component

Create `Assets/Player.prsm`:

```prsm
using UnityEngine

component Player : MonoBehaviour {
    serialize speed: Float = 5.0

    require rb: Rigidbody

    update {
        val h = input.axis("Horizontal")
        val v = input.axis("Vertical")
        rb.velocity = vec3(h, 0, v) * speed
    }
}
```

### 4. Compile

```powershell
prism build
```

This generates `Player.cs` and `Player.prsmmap.json` in the configured output directory.

### 5. Use in Unity

Open the Unity project. The generated C# is compiled by Unity automatically. Add the `Player` component to a GameObject with a Rigidbody.

## Unity package setup

Add the PrSM Unity package via Package Manager:

1. Open **Window > Package Manager**
2. Click **+** > **Add package from git URL**
3. Enter: `https://github.com/29thnight/PrSM.git?path=unity-package`

The package provides:
- Automatic `.prsm` file import (ScriptedImporter)
- Compile/Check/Build context menus
- Custom inspectors for PrSM components
- Stack trace remapping from generated C# to `.prsm` source
- Drag-and-drop component addition

## Watch mode

For continuous development:

```powershell
prism build --watch
```

The compiler watches source directories and recompiles changed files automatically.

## Analysis commands

These power the VS Code extension's navigation features:

```powershell
prism check Assets/Player.prsm              # diagnostics only
prism check Assets/Player.prsm --json       # machine-readable output
prism hir . --json                            # dump Typed HIR
prism definition . --json --file Player.prsm --line 10 --col 5
prism references . --json --file Player.prsm --line 10 --col 5
prism index . --json --symbol Player
```

## Troubleshooting

### `prism` command not found

- **MSI install**: Restart your terminal after installation for PATH to take effect.
- **Source build**: Add `target/release/` to your PATH, or copy `prism.exe` to a directory in PATH.

### VS Code extension not activating

- Ensure the workspace is **trusted** (File > Manage Workspace Trust). The LSP only runs in trusted workspaces.
- Check the Output panel (**View > Output > PrSM Language Server**) for startup errors.
- Verify the compiler path: **Settings > prsm.compilerPath** should point to a valid `prism` binary.

### Unity not detecting `.prsm` files

- Ensure the PrSM Unity package is installed (check **Packages** in the Project window).
- Verify `.prsmproject` exists at the Unity project root.
- Try **PrSM > Build Project** from the menu bar.

### Compilation errors in generated C#

- This usually means the `.prsm` source has issues that PrSM's semantic checker doesn't catch (e.g., referencing undefined Unity types). Check the Unity Console for specific C# errors and fix the corresponding `.prsm` source.
