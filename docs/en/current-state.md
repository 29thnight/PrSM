---
title: Current State
parent: Internals
grand_parent: English Docs
nav_order: 2
---

# Current State

As of the current repository snapshot (v2.0 complete):

Core language and toolchain:
- lexer, parser, semantic analysis, lowering, and code generation are implemented
- the `prism` CLI is implemented and verified in-repo
- Unity package integration is implemented
- trusted-workspace `prism lsp` support is implemented for completion, definition, hover, references, rename, and document/workspace symbols
- VS Code hover now stays on the LSP path while the extension layers generated C# enrichment on top when available
- generated C# back-mapping through `.prsmmap.json` is implemented in both the VS Code extension and Unity package tooling

v2.0 language features:
- pattern matching with enum payload bindings, when guards, val/for destructuring
- listen lifetime model (until disable, until destroy, manual) with automatic cleanup generation
- New Input System sugar including `input.action()` and `input.player().action()` multiplayer form
- limited generic type inference from variable type, return type, and argument type
- incremental build cache with FNV-1a hash-based invalidation
- typed HIR layer separating syntax from semantics
- LSP code actions (explicit type argument, organize usings)

Validation and deployment:
- 251 tests total (204 unit + 47 integration) covering all v2 features
- semantic validation for pattern arity (E082), unknown variants (E081), and listen context (E083)
- unlisten works in all component methods (lifecycle + user functions) with field nulling
- BlazeTest smoke coverage and package-level editor tests exist
- VS Code extension distribution has automated VSIX packaging, bundled artifact verification, and isolated install smoke coverage
- GitHub Actions release workflow with cross-platform builds (Windows/macOS/Linux), MSI installer, VS Code Marketplace publishing, and winget submission

What is planned for v2.1 (see `plan_docs/v2.1-design.md`):

- language reference manual at Lua 5.5 / Rust Book density (currently ~635 lines, target 3,000+)
- v2 feature documentation (pattern matching, listen lifetimes, input system, generic inference)
- EBNF formal grammar, error code catalog, project configuration reference
- HIR enrichment (generic substitutions, expression types)
- source map v2 sugar detail mapping
- VS Code C# bridge transition to source-map-based resolution
- LSP incremental indexing
