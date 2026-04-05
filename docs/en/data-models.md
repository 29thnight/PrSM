---
title: Data Models & Attributes
parent: Language Guide
grand_parent: English Docs
nav_order: 6
---

# Data Models & Attributes

## Data class

PrSM does not expose a `struct` keyword. The implemented data-model feature is `data class`.

```prsm
data class DamageInfo(
    val amount: Int,
    val crit: Bool
)
```

This lowers to a serializable C# class with public fields, a generated primary constructor, `Equals`, `GetHashCode`, and `ToString`.

### Using a data class

```prsm
val hit = DamageInfo(amount: 42, crit: true)
if hit.crit {
    showCritFX()
}
```

Data classes are value-typed by equality: two `DamageInfo` instances with the same field values compare equal.

## Enum

### Simple enum

```prsm
enum EnemyState {
    Idle,
    Chase,
    Attack
}
```

Lowers to a standard C# `enum`.

### Parameterized enum

Enum variants can carry typed payloads:

```prsm
enum AbilityResult {
    Hit(damage: Int),
    Miss,
    Reflect(damage: Int, angle: Float)
}
```

The compiler generates a nested payload struct for each variant that has parameters, plus extension methods `IsHit()`, `HitPayload()`, `IsMiss()`, `IsReflect()`, `ReflectPayload()` and so on for ergonomic access.

```prsm
val result: AbilityResult = AbilityResult.Hit(damage: 30)
if result.IsHit() {
    applyDamage(result.HitPayload().damage)
}
```

## Attribute

Custom C# attributes are declared with the `attribute` keyword:

```prsm
@targets(Method, Property)
attribute Cooldown(
    val duration: Float,
    val resetOnHit: Bool
)
```

This lowers to a C# `Attribute` subclass with a generated constructor and `[AttributeUsage]` metadata derived from the `@targets` decorator.

### Applying an attribute

```prsm
@Cooldown(duration: 1.5, resetOnHit: true)
func fireProjectile() {
    // ...
}
```
