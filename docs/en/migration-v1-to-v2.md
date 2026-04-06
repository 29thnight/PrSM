---
title: Migrating from v1 to v2
parent: Language Guide
grand_parent: English Docs
nav_order: 13
---

# Migrating from v1 to v2

## Overview

v2 is fully opt-in. Existing v1 projects continue to compile and run without any changes. You adopt v2 features at your own pace by updating your `.prsmproject` file.

v2 introduces listen lifetimes, pattern bindings in `when`, the new Input System sugar, and improved generic type inference. All of these are gated behind a version flag and optional feature flags, so nothing changes until you explicitly opt in.

## Step-by-step migration

### Step 1 — Update `.prsmproject`

Open your `.prsmproject` file and set the language version:

```toml
[language]
version = "2.0"
```

You can also enable specific feature flags at the same time (see the feature flag table below), but this is not required. Setting the version alone activates the core v2 semantics.

### Step 2 — Review listen statements

v2 introduces explicit lifetime modifiers for `listen`, but does **not** change the default behavior. `listen` without a modifier still registers only (no auto-cleanup), same as v1.

**What's new in v2:**

- `listen event until disable { }` — auto-cleanup in OnDisable
- `listen event until destroy { }` — auto-cleanup in OnDestroy
- `val token = listen event manual { }` + `unlisten token` — explicit control
- These modifiers are only valid inside `component` declarations (E083 outside)

**What to check:**

- If your v1 code has manual `RemoveListener` in intrinsic blocks, consider replacing with `until disable` or `until destroy` for cleaner code.
- `listen` without a modifier is unchanged in behavior — it registers once with no automatic cleanup.

```prsm
// v1 behavior — register once, never auto-cleanup
listen button.onClick {
    fire()
}

// v2 equivalent — explicit auto-cleanup in OnDisable
listen button.onClick until disable {
    fire()
}

// v2 — manual lifetime, you control removal
val token = listen button.onClick manual {
    fire()
}
unlisten token
```

### Step 3 — Enable feature flags

Add the features you want to the `features` array in `.prsmproject`:

```toml
[language]
version = "2.0"
features = ["pattern-bindings", "input-system", "auto-unlisten"]
```

Each feature is independent — enable only what you need. See the feature flag reference below.

### Step 4 — Rebuild

Run a clean build to recompile all sources against v2 semantics:

```bash
prism build
```

Fix any new diagnostics. The most common ones are E081, E082 (pattern binding validation) and E083 (listen lifetime used outside a component).

## Breaking changes in v2

| Change | v1 behavior | v2 behavior | Diagnostic |
|---|---|---|---|
| Pattern bindings in `when` | Bindings were not validated against enum definitions | Bindings are validated; mismatched arity or missing variants produce errors | E081, E082 |
| `listen until disable` / `listen manual` / `unlisten` | Not available | New lifetime modifiers, only valid inside `component` declarations | E083 |
| `listen` default (no modifier) | Register only, no cleanup | **Unchanged** — still register only, no cleanup | — |

### Listen auto-cleanup

In v1, all `listen` blocks were fire-and-forget. In v2 components, listeners are automatically cleaned up when the component is disabled. This prevents common bugs like listeners firing on destroyed objects.

If you relied on listeners surviving disable/enable cycles, switch to `listen manual`:

```prsm
val token = listen manager.onScoreChanged manual { val score ->
    updateUI(score)
}

// Later, when you are done:
unlisten token
```

### Pattern binding validation

v2 validates that pattern bindings in `when` expressions match the actual enum definition. Code that previously compiled with unchecked bindings may now produce E081 (unknown variant) or E082 (wrong number of parameters) errors.

```prsm
enum Result {
    Ok(value: Int),
    Err(message: String)
}

when result {
    Result.Ok(v)     => handleOk(v)       // valid
    Result.Err(m)    => handleErr(m)      // valid
    Result.Unknown   => { }               // E081 — no such variant
}
```

### Listen lifetime scope

`listen until disable`, `listen manual`, and `unlisten` are only valid inside `component` declarations. Using them in a `class`, `asset`, or top-level scope produces E083.

## New features available in v2

| Feature | Description | Documentation |
|---|---|---|
| Pattern matching with bindings | Destructure enum variants in `when` branches | [Pattern Matching & Control Flow](pattern-matching-and-control-flow.md) |
| Listen lifetime model | `until disable`, `manual`, and `unlisten` for event subscriptions | [Events & Intrinsic](events-and-intrinsic.md) |
| New Input System sugar | `on input action { }` syntax for Unity Input System | [Input System](input-system.md) |
| Generic type inference | Compiler infers generic arguments from usage context | [Generic Inference](generic-inference.md) |

## Feature flag reference

| Flag | Requires | Description |
|---|---|---|
| `"pattern-bindings"` | v2.0 | Enables destructuring bindings in `when` branches. Without this flag, `when` branches use v1 syntax only. |
| `"input-system"` | v2.0 | Enables `on input` sugar for the new Unity Input System. Requires the Input System package in your Unity project. |
| `"auto-unlisten"` | v2.0 | Enables the `until disable` / `manual` / `unlisten` lifetime model for `listen`. Without this flag, `listen` uses v1 register-only semantics even in v2. |

Example `.prsmproject` with all flags:

```toml
[project]
name = "MyGame"
unity = "2022.3"

[language]
version = "2.0"
features = ["pattern-bindings", "input-system", "auto-unlisten"]

[output]
path = "Assets/Generated"
```

## Rollback to v1

If you need to revert to v1:

1. Change the version back:

```toml
[language]
version = "1.0"
```

2. Remove any v2-only syntax from your `.prsm` files:
   - Replace `listen ... until disable { }` with plain `listen ... { }`
   - Remove `listen ... manual { }` and `unlisten` statements
   - Remove pattern bindings from `when` branches (use plain matching)
   - Remove `on input` blocks (use `listen` with the appropriate Unity event instead)

3. Rebuild:

```bash
prism build
```

The compiler will report errors for any remaining v2 syntax, so you can fix them incrementally.
