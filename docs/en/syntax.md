---
title: Syntax
parent: Language Guide
grand_parent: English Docs
nav_order: 1
---

# Syntax

PrSM keeps its surface syntax small and regular. There are no semicolons, no parentheses on control flow, and no mandatory boilerplate around the common Unity patterns.

## File structure

Each `.prsm` file contains exactly one top-level declaration. Files typically start with `using` imports followed by the single declaration body.

```prsm
using UnityEngine
using System.Collections

component PlayerController : MonoBehaviour {
    serialize speed: Float = 5.0

    update {
        move()
    }

    func move() {
        transform.Translate(Vector3.forward * speed * Time.deltaTime)
    }
}
```

## Key rules

- **Newline-terminated statements** — no semicolons
- **Brace-based blocks** — `{}` delimit all bodies
- **Parenthesis-free control flow** — `if`, `when`, `for`, `while` do not wrap conditions in `()`
- **One declaration per file** — the top-level type is the file unit
- **`using` for imports** — brings Unity and .NET namespaces into scope

## Identifiers and naming

- Type names are `PascalCase`
- Member names and field names are `camelCase`
- Lifecycle keywords (`awake`, `update`, etc.) are lowercase reserved words

## Comments

```prsm
// single-line comment

/*
  multi-line comment
*/
```

## Minimal single-file example

```prsm
using UnityEngine

component Empty : MonoBehaviour {
    update {
    }
}
```

The generated C# for this is a normal `MonoBehaviour` subclass with an `Update()` method.

## Operator precedence

From lowest to highest binding power:

| Precedence | Operator(s) | Associativity | Description |
|:---:|---|---|---|
| 1 | `?:` | Right | Elvis (null coalescing) |
| 2 | `\|\|` | Left | Logical OR |
| 3 | `&&` | Left | Logical AND |
| 4 | `==` `!=` | Left | Equality |
| 5 | `<` `>` `<=` `>=` `is` | Left | Comparison, type check |
| 6 | `..` `until` `downTo` | — | Range |
| 7 | `+` `-` | Left | Additive |
| 8 | `*` `/` `%` | Left | Multiplicative |
| 9 | `!` `-` (unary) | Right | Unary negation/not |
| 10 | `.` `?.` `!!` `[]` `()` | Left | Postfix (member, safe call, assert, index, call) |

## Assignment operators

| Operator | Description |
|---|---|
| `=` | Assign |
| `+=` `-=` `*=` `/=` `%=` | Compound assign |

Assignment is a statement, not an expression.

## String escape sequences

Inside string literals:

| Escape | Character |
|---|---|
| `\n` | Newline |
| `\t` | Tab |
| `\r` | Carriage return |
| `\\` | Backslash |
| `\"` | Double quote |
| `\$` | Dollar sign (prevents interpolation) |

## String interpolation

Two forms:

```prsm
val greeting = "hello $name"              // shorthand
val info = "score: ${player.score + 1}"   // expression
```

The `${}` form supports any expression including nested braces. Generated C# uses `$"..."` interpolation.

## Duration literals

Numeric literals with a time suffix:

```prsm
wait 1.5s     // 1.5 seconds → new WaitForSeconds(1.5f)
wait 500ms    // 500 milliseconds → new WaitForSeconds(0.5f)
```

## Formal grammar

See [Formal Grammar](grammar.md) for the complete EBNF specification.
