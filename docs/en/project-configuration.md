---
title: Project Configuration
parent: Language Guide
grand_parent: English Docs
nav_order: 11
---

# Project Configuration

Every PrSM workspace is anchored by a `.prsmproject` file at the project root. The file uses TOML format and controls how the compiler discovers, compiles, and outputs source files.

## Minimal example

```toml
[project]
name = "MyGame"
prsm_version = "0.1.0"

[language]
version = "1.0"

[compiler]
output_dir = "Assets/Generated"
```

## Section reference

### `[project]`

| Key | Type | Default | Description |
|---|---|---|---|
| `name` | string | **required** | Display name of the project |
| `prsm_version` | string | `"0.1.0"` | SemVer version of the PrSM toolchain this project targets |

### `[language]`

| Key | Type | Default | Description |
|---|---|---|---|
| `version` | string | `"1.0"` | Language version: `"1.0"` or `"2.0"` |
| `features` | array of strings | `[]` | Feature flags to enable (see below) |

### `[compiler]`

| Key | Type | Default | Description |
|---|---|---|---|
| `output_dir` | string | `"Assets/Generated"` | Directory where generated `.cs` files are written |
| `prism_path` | string | auto-detected | Explicit path to the `prism` compiler binary; overrides PATH lookup |

### `[source]`

| Key | Type | Default | Description |
|---|---|---|---|
| `include` | array of globs | `["**/*.prsm"]` | Glob patterns for source files to compile |
| `exclude` | array of globs | `[]` | Glob patterns to exclude from compilation |

### `[features]`

| Key | Type | Default | Description |
|---|---|---|---|
| `auto_compile_on_save` | bool | `true` | Re-compile automatically when a `.prsm` file is saved |
| `generate_meta_files` | bool | `true` | Emit `.cs.meta` files for Unity asset database integration |
| `pascal_case_methods` | bool | `true` | Generate PascalCase C# method names instead of camelCase |

## Feature flags

Feature flags are listed in `language.features` and gate experimental or opt-in syntax.

| Flag | Requires | Description |
|---|---|---|
| `auto-unlisten` | language 1.0+ | Automatically emit `RemoveListener` calls in `OnDestroy` for every `listen` statement |
| `input-system` | language 1.0+ | Enable the Input System sugar syntax for the new Unity Input System package |
| `pattern-bindings` | language 1.0+ | Allow `val` bindings inside `when` patterns for destructuring enum payloads |

When `language.version` is `"2.0"`, the flags `auto-unlisten` and `pattern-bindings` are enabled implicitly. You may still list them explicitly without error.

## Unity integration

The compiler detects Unity project capabilities by inspecting `Packages/manifest.json` in the project root. When the `com.unity.inputsystem` package is present, Input System APIs are available for resolution. When `com.unity.textmeshpro` is present, TMP types are recognized. No manual configuration is needed for package detection.

If the Unity project uses an Assembly Definition (`.asmdef`), the generated output directory should be inside the same assembly scope so that Unity compiles the generated C# together with the rest of the project code.

## Default values summary

```toml
[project]
name = "Untitled"
prsm_version = "0.1.0"

[language]
version = "1.0"
features = []

[compiler]
output_dir = "Assets/Generated"
# prism_path is auto-detected from PATH

[source]
include = ["**/*.prsm"]
exclude = []

[features]
auto_compile_on_save = true
generate_meta_files = true
pascal_case_methods = true
```

## Legacy migration from `.mnproject`

Earlier versions of the toolchain used a `.mnproject` JSON file (from the original "Moon" project name). The compiler still recognizes `.mnproject` and loads it with the same field semantics, but the TOML-based `.prsmproject` takes precedence when both exist. To migrate:

1. Create a new `.prsmproject` at the workspace root using the TOML format shown above.
2. Copy field values from `.mnproject` into the corresponding TOML sections.
3. Delete the old `.mnproject` file once the new configuration is verified.

The compiler emits a one-time informational message when it falls back to `.mnproject`, reminding you to migrate.
