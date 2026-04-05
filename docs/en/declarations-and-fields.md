---
title: Declarations & Fields
parent: Language Guide
grand_parent: English Docs
nav_order: 5
---

# Declarations & Fields

## Top-level declarations

Each `.prsm` file contains exactly one top-level declaration.

| Keyword | C# equivalent | Purpose |
|---|---|---|
| `component` | `MonoBehaviour` subclass | Gameplay logic attached to a GameObject |
| `asset` | `ScriptableObject` subclass | Data containers, config, shared state |
| `class` | Regular C# `class` | Utilities, services, plain data |
| `data class` | Serializable value class | Lightweight data with generated equality |
| `enum` | `enum` | Named constant sets |
| `attribute` | `Attribute` subclass | Custom C# annotations |

## `component`

```prsm
using UnityEngine

component PlayerController : MonoBehaviour {
    @header("Movement")
    serialize speed: Float = 5.0

    require rb: Rigidbody

    update {
        move()
    }

    func move() {
        rb.MovePosition(rb.position + transform.forward * speed * Time.fixedDeltaTime)
    }
}
```

## `asset`

```prsm
using UnityEngine

asset WeaponConfig : ScriptableObject {
    serialize damage: Int = 10
    serialize fireRate: Float = 0.2
    serialize projectilePrefab: GameObject = null
}
```

 Assets created via `ScriptableObject.CreateInstance<T>()` in the Unity Editor store their values persistently in `.asset` files.

## `class`

```prsm
class DamageCalculator {
    func calculate(base: Int, multiplier: Float): Float {
        return base * multiplier
    }
}
```

`class` maps to a regular C# class with no Unity dependency.

## Serialized fields

Fields marked `serialize` are exposed in the Unity Inspector. Several decorator annotations control how they appear:

```prsm
@header("Stats")
serialize maxHp: Int = 100

@tooltip("Units per second")
serialize speed: Float = 5.0

@range(0.0, 1.0)
serialize damageMultiplier: Float = 0.5

@space
serialize weaponSlot: GameObject = null
```

Supported decorators: `@header(label)`, `@tooltip(text)`, `@range(min, max)`, `@space`, `@hideInInspector`.

## `val` and `var`

- `val` — immutable; cannot be reassigned after initialization
- `var` — mutable field or local

```prsm
val gravity: Float = 9.81      // constant
var hp: Int = 100               // mutable
```

## Visibility modifiers

`public`, `private`, and `protected` map directly to C#. In most contexts members default to `public`.

```prsm
private var invincible: Bool = false
protected var baseSpeed: Float = 5.0
```

## Component lookup fields

Four qualifiers are only valid inside `component` declarations. They generate lookup code in the synthesized `Awake()` **before** the user `awake` body runs:

| Qualifier | Generated C# | Null contract |
|---|---|---|
| `require name: Type` | `GetComponent<Type>()` | Logs an error and asserts non-null if missing |
| `optional name: Type?` | `GetComponent<Type>()` | May be null, stored as nullable |
| `child name: Type` | `GetComponentInChildren<Type>()` | Asserts non-null |
| `parent name: Type` | `GetComponentInParent<Type>()` | Asserts non-null |

```prsm
require animator: Animator
optional shield: Shield?
child muzzle: Transform
parent vehicle: Vehicle
```
