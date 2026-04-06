---
title: PrSM 3
parent: Specification
nav_order: 5
---

# PrSM Language 3

PrSM 3 introduces interface and generic declarations, design pattern sugar (`singleton`, `pool`), code quality analysis, and the first compiler optimizer. This release targets **Prism v1.0.0**.

**Activation:** `language.version = "3"` in `.prsmproject`

## New language features

### Interface declaration

PrSM-native interface definitions with method signatures and properties:

```prsm
interface IDamageable {
    func takeDamage(amount: Int)
    val isAlive: Bool
}

interface IHealable : IDamageable {
    func heal(amount: Int)
}
```

- Lowered to standard C# `interface`
- Interface members shall not have implementation bodies (E091)
- Implementing component/class must define all members (E090)
- Supported in `require` fields: `require target: IDamageable`

### Generic declaration

Type parameters and `where` constraints on `class` and `func`:

```prsm
class Registry<T> where T : Component {
    var items: List<T> = null
    func register(item: T) { items.add(item) }
}

func findAll<T>(): List<T> where T : Component {
    return FindObjectsByType<T>(FindObjectsSortMode.None).toList()
}
```

- Multiple type params: `class Pair<K, V>`
- Multiple constraints: `where T : MonoBehaviour, IDamageable`
- Generic `interface` supported: `interface IPool<T> { func get(): T }`
- Not supported on `component`, `asset`, `enum`, `data class` (E096)

### `singleton` keyword

Singleton component with one keyword:

```prsm
singleton component AudioManager : MonoBehaviour {
    serialize volume: Float = 1.0
    func playSound(clip: AudioClip) { /* ... */ }
}

// Usage from anywhere:
AudioManager.instance.playSound(clip)
```

Auto-generates:
- `private static T _instance` field
- `public static T Instance` property with lazy init (`FindFirstObjectByType` + `AddComponent` fallback)
- `Awake` guard: duplicate destroy + `DontDestroyOnLoad`

### `pool` modifier

Object pooling with two lines:

```prsm
component BulletSpawner : MonoBehaviour {
    serialize bulletPrefab: Bullet
    pool bullets: Bullet(capacity = 20, max = 100)

    func fire(direction: Vector3) {
        val bullet = bullets.get()
        bullet.launch(direction)
    }
}
```

- Based on `UnityEngine.Pool.ObjectPool<T>`
- Auto-generates: `createFunc`, `actionOnGet`, `actionOnRelease`, `actionOnDestroy` callbacks
- Matches `serialize` field by type for prefab (E098 if missing)
- Component-only (E099 outside)

## Compiler improvements

### SOLID analysis warnings

Static analysis pass detecting common design issues:

| Code | Principle | Condition |
|------|-----------|-----------|
| W010 | Single Responsibility | Component has 8+ public methods |
| W011 | Dependency Inversion | Component has 6+ dependency fields |
| W012 | Single Responsibility | Method/lifecycle has 50+ statements |

Configurable in `.prsmproject`:

```toml
[analysis]
solid_warnings = true
disabled_warnings = ["W012"]
```

### Code optimizer

Lowering optimizations for cleaner and faster C# output:

**Single-binding destructure inline:**
```prsm
val Stats(hp) = getStats()
```
Before: `var _prsm_d = getStats(); var hp = _prsm_d.hp;`
After: `var hp = getStats().hp;`

### Reserved sugar names

`get` and `find` are now reserved as built-in method names (E101). User-defined functions with these names produce a compile error to prevent silent sugar hijacking.

## New diagnostics

| Code | Severity | Description |
|------|----------|-------------|
| E090 | Error | Interface member not implemented |
| E091 | Error | Interface member has implementation body |
| E095 | Error | Type argument violates `where` constraint |
| E096 | Error | Generic params on component/asset/enum/data class |
| E097 | Error | `singleton` on non-component declaration |
| E098 | Error | Pool type has no matching serialize prefab |
| E099 | Error | `pool` outside component |
| E101 | Error | Reserved built-in method name (`get`, `find`) |
| W010 | Warning | Too many public methods (SOLID) |
| W011 | Warning | Too many dependency fields (SOLID) |
| W012 | Warning | Method too long (SOLID) |

## Feature gates

All Language 3 features are implicitly enabled with `version = "3"`. Individual features can be selectively enabled from Language 2:

```toml
[language]
version = "2"
features = ["interface", "generics"]
```

| Feature flag | Description |
|-------------|-------------|
| `interface` | Interface declaration |
| `generics` | Generic class/func with where clauses |
| `singleton` | Singleton component keyword |
| `pool` | Object pool modifier |
| `solid-analysis` | SOLID warnings |
| `optimizer` | Code optimizer |

## Toolchain improvements

- MSI installer for Windows (one-click: compiler + VS Code extension + Unity guide)
- `winget install PrSM.PrSM` support
- GitHub Actions release pipeline (3-platform build + VSIX + MSI + Marketplace + winget)
- `scripts/bump-version.sh` for mono-version management across all components
- Dynamic docs navigation via `_nav.json`
