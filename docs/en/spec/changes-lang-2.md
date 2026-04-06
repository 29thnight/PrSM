---
title: "Language 1 → 2 Changes"
parent: Specification
grand_parent: English Docs
nav_order: 2
---

# PrSM Language 1 → Language 2 Changes

This document summarizes the changes between Language 1 and Language 2. For the complete specification, see the [PrSM Language Standard](standard.md).

## Overview

Language 2 is a strict superset of Language 1. All valid Language 1 programs are valid Language 2 programs with identical semantics. Language 2 adds new syntax and validation rules but introduces no breaking changes to existing code.

To opt into Language 2, set `language.version = "2"` in `.prsmproject`:

```toml
[language]
version = "2"
features = ["pattern-bindings", "input-system", "auto-unlisten"]
```

## New Features

### 1. Pattern Matching with Bindings (§9, §10)

When branches can now bind enum payload variables:

```prsm
when state {
    EnemyState.Idle => idle()
    EnemyState.Chase(target) => moveTo(target)
    EnemyState.Stunned(duration) if duration > 0.0 => wait(duration)
}
```

- Enum payload bindings extract values via tuple-style access (`Item1`, `Item2`)
- When guards (`if condition`) add post-match filtering
- Binding arity is validated against enum parameter count

### 2. Listen Lifetime Model (§10)

Listen statements support explicit lifetime modifiers (component-only):

```prsm
listen button.onClick until disable { fire() }
listen spawner.onSpawn until destroy { count += 1 }
val token = listen timer.finished manual { reset() }
unlisten token
```

- `until disable` — auto-cleanup in `OnDisable`
- `until destroy` — auto-cleanup in `OnDestroy`
- `manual` — returns subscription token for explicit `unlisten`
- Without modifier: register-only (same as Language 1)
- `unlisten` removes the listener and nulls the handler field

### 3. Destructuring (§10)

Val and for statements support data class destructuring:

```prsm
val PlayerStats(hp, speed) = getStats()

for Spawn(pos, delay) in wave.spawns {
    spawnAt(pos, delay)
}
```

Binding count shall match the data class field count.

### 4. New Input System Sugar (§10)

Sugar for Unity's New Input System package (requires `input-system` feature):

```prsm
if input.action("Jump").pressed { jump() }
val look = input.player("Gameplay").action("Look").vector2
```

States: `pressed`, `released`, `held`, `vector2`, `scalar`.

### 5. Generic Type Inference (§9)

Limited context-based inference for generic sugar methods:

```prsm
val rb: Rigidbody = get()        // infers GetComponent<Rigidbody>()
val health: Health? = child()    // infers GetComponentInChildren<Health>()
```

Inference contexts: variable type annotation, return type, argument type.

### 6. Feature Gates (§5)

`.prsmproject` controls feature availability:

| Feature | Description |
|---------|-------------|
| `pattern-bindings` | Enum payload binding, destructuring, when guards |
| `input-system` | Input System sugar (requires Unity Input System package) |
| `auto-unlisten` | Listen lifetime modifiers and unlisten |

## New Diagnostics

| Code | Severity | Message | Condition |
|------|----------|---------|-----------|
| E081 | Error | Unknown variant '{v}' for enum '{e}' | When pattern references nonexistent enum variant |
| E082 | Error | Pattern binds N variable(s) but '{t}' expects M | Binding count mismatch with enum payload or data class fields |
| E083 | Error | 'listen {modifier} { }' is only valid inside a component | Listen lifetime modifier used outside component |

## Breaking Changes

None. All Language 1 programs compile without modification under Language 2.

## Migration Checklist

1. Set `language.version = "2"` in `.prsmproject`
2. Optionally add features to `language.features` array
3. Run `prism build` — fix any new E081/E082/E083 diagnostics if patterns were previously unchecked
4. Adopt new features incrementally:
   - Add `until disable` to long-lived listen statements
   - Replace manual cleanup intrinsic blocks with `unlisten`
   - Use `input.action()` instead of legacy `input.getKey()`/`input.axis()`
