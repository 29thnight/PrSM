---
title: Source Maps
parent: Internals
grand_parent: English Docs
nav_order: 3
---

# Source Maps

The PrSM compiler emits a **`.prsmmap.json`** sidecar file alongside every
generated `.cs` file. These source maps establish a bidirectional mapping
between the original `.prsm` source and the generated C# output, enabling
stack trace remapping, editor navigation, and diagnostics.

## Schema Structure

A source map file has the following top-level fields:

```json
{
  "version": 1,
  "source_file": "src/PlayerController.prsm",
  "generated_file": "Generated/PlayerController.g.cs",
  "declaration": { ... },
  "members": [ ... ]
}
```

| Field | Description |
|---|---|
| `version` | Schema version number (currently `1`). |
| `source_file` | Relative path to the original `.prsm` source file. |
| `generated_file` | Relative path to the generated `.cs` file. |
| `declaration` | Anchor describing the top-level type declaration. |
| `members` | Array of member anchors (methods, fields, properties). |

## Declaration Anchor

The `declaration` object maps the class or struct declaration itself:

```json
{
  "type": "class",
  "name": "PlayerController",
  "spans": {
    "prsm": { "line": 1, "col": 1, "end_line": 45, "end_col": 1 },
    "cs":   { "line": 5, "col": 1, "end_line": 82, "end_col": 1 }
  }
}
```

## Members Array

Each entry in the `members` array describes one member of the type:

```json
{
  "kind": "method",
  "name": "update",
  "spans": {
    "prsm": { "line": 10, "col": 5, "end_line": 25, "end_col": 5 },
    "cs":   { "line": 20, "col": 9, "end_line": 48, "end_col": 9 }
  },
  "segments": [ ... ]
}
```

`kind` is one of `method`, `field`, `property`, or `event`.

## Span Format

All spans use **1-based** line and column numbers:

```json
{ "line": 10, "col": 5, "end_line": 25, "end_col": 5 }
```

| Field | Description |
|---|---|
| `line` | Starting line (1-based). |
| `col` | Starting column (1-based). |
| `end_line` | Ending line (1-based, inclusive). |
| `end_col` | Ending column (1-based, exclusive). |

## Segment Nesting

Segments provide fine-grained mappings within a member body. They can be
nested to represent block statements such as `if`, `for`, and `while`:

```
declaration
  +-- member (method "update")
        +-- segment (if-block, line 12-18)
        |     +-- segment (nested for-loop, line 14-16)
        +-- segment (return statement, line 20)
```

This anchor hierarchy allows tools to resolve any C# line back to its precise
`.prsm` origin, even inside deeply nested control flow.

## How the Compiler Generates Source Maps

The `source_map.rs` module in the compiler builds the map incrementally as it
emits C# output. Each time the code generator writes a declaration, member, or
statement, it records the current PrSM span and the corresponding C# output
span. The final JSON is written atomically alongside the `.cs` file.

Source map generation is enabled by default. It can be disabled with the
`--no-source-maps` compiler flag, which is occasionally useful for release
builds where the sidecar files are not needed.

## Unity Package Integration

The PrSM Unity package includes `PrismSourceMap.TryResolveSourceLocation()`,
which accepts a C# file path and line number and returns the corresponding
PrSM file path and line. The Unity runtime calls this method when formatting
exception stack traces, replacing generated C# locations with their `.prsm`
origins so that console output points directly to your source code.

## VS Code Extension Integration

The VS Code extension uses source maps for two features:

1. **Bidirectional navigation** -- when you open a `.prsm` file the extension
   can jump to the corresponding generated `.cs` location, and vice versa.
2. **Stack trace click handling** -- clickable file links in Unity console
   output are rewritten to open the `.prsm` source at the correct line.

## Debugging Workflow

A typical debugging session with source maps looks like this:

1. A runtime exception occurs in Unity.
2. The PrSM Unity package intercepts the stack trace and calls
   `PrismSourceMap.TryResolveSourceLocation()` for each frame.
3. Generated C# paths and line numbers are replaced with `.prsm` paths.
4. The remapped stack trace appears in the Unity Console.
5. Clicking a stack frame opens VS Code at the exact `.prsm` line that
   produced the failing generated code.

This end-to-end flow means you rarely need to look at generated C# when
diagnosing runtime errors.
