---
title: Generic Inference
parent: Language Guide
grand_parent: English Docs
nav_order: 10
---

# Generic Inference

PrSM v2 introduces **limited context-based generic type inference**, allowing
you to omit explicit type arguments on common generic helper methods when the
target type can be determined from surrounding context.

## Overview

Instead of writing the type argument explicitly:

```prsm
val rb: Rigidbody = get<Rigidbody>()
```

You can let the compiler infer it from the variable's type annotation:

```prsm
val rb: Rigidbody = get()
```

The compiler resolves the omitted type parameter and emits the fully qualified
generic call in the generated C#.

## Supported Methods

Inference applies to the following built-in generic helpers:

| PrSM method | Generated C# |
|---|---|
| `get<T>()` | `GetComponent<T>()` |
| `require<T>()` | `GetComponent<T>()` with a null-check assertion |
| `find<T>()` | `FindFirstObjectByType<T>()` |
| `child<T>()` | `GetComponentInChildren<T>()` |
| `parent<T>()` | `GetComponentInParent<T>()` |

Inference is **not** available for arbitrary user-defined generic functions.

## Inference Contexts

The compiler recognizes three contexts where a type argument can be inferred.

### 1. Variable Type Annotation

When the left-hand side of a declaration has an explicit type, the compiler
uses it to fill the missing type argument.

```prsm
val rb: Rigidbody = get()
val col: BoxCollider = child()
```

Generated C#:

```csharp
Rigidbody rb = GetComponent<Rigidbody>();
BoxCollider col = GetComponentInChildren<BoxCollider>();
```

### 2. Return Type Context

When a generic call is the operand of a `return` statement, the compiler
infers the type from the enclosing function's return type.

```prsm
func getPlayer(): Player {
    return find()
}
```

Generated C#:

```csharp
Player GetPlayer()
{
    return FindFirstObjectByType<Player>();
}
```

### 3. Argument Type Context

When a generic call is passed directly as an argument, the compiler infers the
type from the corresponding parameter type of the called function.

```prsm
func setup(rb: Rigidbody) { ... }

func awake() {
    setup(get())
}
```

Generated C#:

```csharp
void Awake()
{
    Setup(GetComponent<Rigidbody>());
}
```

## Rules

- There must be a **single unambiguous** solution. If the surrounding context
  does not uniquely determine one type, the compiler requires an explicit type
  argument and emits an error.
- Inference is purely **local**: it does not propagate across multiple
  assignment steps or through intermediate variables.
- If a variable has no type annotation and relies on `val` type deduction, the
  compiler cannot infer the generic argument because there is no target type to
  read from.

## What Is NOT Supported

PrSM's inference is intentionally limited in scope:

- **Hindley-Milner unification** -- the compiler does not perform global
  constraint solving across an entire function body.
- **Lambda + overload inference** -- generic types cannot be inferred from
  lambda argument types in the presence of overloaded candidates.
- **Generic declaration expansion** -- user-defined generic classes or methods
  are not eligible for inference; only the built-in helpers listed above
  participate.

When in doubt, supply the type argument explicitly. The explicit form is always
accepted and never ambiguous.
