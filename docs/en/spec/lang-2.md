---
title: PrSM 2
parent: Specification
nav_order: 4
---

# PrSM Language 2

PrSM 2 extends Language 1 with pattern matching, event lifetime management, Input System support, and generic type inference. All Language 1 programs are valid Language 2 programs with identical semantics.

**Activation:** `language.version = "2"` in `.prsmproject`

## New language features

### Pattern matching with bindings

Enum payload extraction in `when` branches:

```prsm
when state {
    EnemyState.Idle => idle()
    EnemyState.Chase(target) => moveTo(target)
    EnemyState.Stunned(duration) if duration > 0.0 => wait(duration)
}
```

- Enum payload bindings via tuple-style access (`Item1`, `Item2`)
- When guards (`if condition`) for post-match filtering
- Binding arity validated against enum parameter count (E082)
- Unknown variant detection (E081)

### Destructuring

Data class decomposition in `val` and `for`:

```prsm
val PlayerStats(hp, speed) = getStats()
for Spawn(pos, delay) in wave.spawns { spawnAt(pos, delay) }
```

### Listen lifetime model

Explicit listener cleanup (component-only, E083 outside):

```prsm
listen button.onClick until disable { fire() }
listen spawner.onSpawn until destroy { count += 1 }
val token = listen timer.finished manual { reset() }
unlisten token
```

- `until disable` — auto-cleanup in `OnDisable`
- `until destroy` — auto-cleanup in `OnDestroy`
- `manual` + `unlisten` — explicit control with field nulling
- Default (no modifier): register-only, same as Language 1

### New Input System sugar

Unity New Input System package support (requires `input-system` feature, E070 without):

```prsm
if input.action("Jump").pressed { jump() }
val look = input.player("Gameplay").action("Look").vector2
```

States: `pressed`, `released`, `held`, `vector2`, `scalar`

### Generic type inference

Limited context-based inference for sugar methods:

```prsm
val rb: Rigidbody = get()           // GetComponent<Rigidbody>()
val health: Health? = child()       // GetComponentInChildren<Health>()
```

Inference from: variable type annotation, return type, argument type.

### Feature gates

`.prsmproject` controls feature availability:

| Feature | Description |
|---------|-------------|
| `pattern-bindings` | Enum payload binding, destructuring, when guards |
| `input-system` | Input System sugar (requires Unity Input System package) |
| `auto-unlisten` | Listen lifetime modifiers and unlisten |

## New diagnostics

| Code | Description |
|------|-------------|
| E070 | Input System sugar without `input-system` feature |
| E081 | Unknown variant in pattern |
| E082 | Pattern binding arity mismatch |
| E083 | Listen lifetime modifier outside component |

## Toolchain improvements

- Typed HIR layer separating syntax from semantics
- Incremental build cache with FNV-1a hash-based invalidation
- `prism lsp` — LSP server with completion, definition, hover, references, rename, code actions
- VS Code extension: thin LSP client, stack trace remapping, source map navigation
- `.prsmmap.json` source maps for Unity Console and VS Code bidirectional navigation
