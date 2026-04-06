---
title: PrSM 1
parent: Specification
nav_order: 3
---

# PrSM Language 1

PrSM 1 is the initial release of the PrSM language. It established the core syntax, type system, and Unity integration model.

## Language features

### Declarations

- [component](../declarations-and-fields.md) — MonoBehaviour subclass with lifecycle blocks, field qualifiers, and serialization
- [asset](../declarations-and-fields.md) — ScriptableObject subclass with `[CreateAssetMenu]` auto-generation
- [class](../declarations-and-fields.md) — plain C# class with optional single inheritance
- [data class](../declarations-and-fields.md) — value-like class with auto-generated `Equals`, `GetHashCode`, `ToString`
- [enum](../declarations-and-fields.md) — simple and parameterized enums with payload accessor extension methods
- [attribute](../declarations-and-fields.md) — custom C# attribute declarations

### Type system

- Primitive types: `Int`, `Float`, `Double`, `Bool`, `String`, `Char`, `Long`, `Byte`, `Unit`
- Nullable types: `Type?` with safe-call `?.`, elvis `?:`, non-null assert `!!`
- Generic type references: `List<T>`, `Map<K,V>`, `Array<T>`, `Set<T>`, `Queue<T>`, `Stack<T>`, `Seq<T>`
- Unity and external types passed through unchanged

### Fields

- `serialize` with decorators: `@header`, `@tooltip`, `@range`, `@space`, `@hideInInspector`
- `val` / `var` immutability
- `public` / `private` / `protected` visibility
- Component lookup: `require`, `optional`, `child`, `parent`

### Functions

- `func` with block body or expression body
- `override` modifier
- Default parameter values
- Named arguments at call site
- `intrinsic func` / `intrinsic coroutine` — raw C# escape hatch

### Lifecycle blocks

- `awake`, `start`, `update`, `fixedUpdate`, `lateUpdate`
- `onEnable`, `onDisable`, `onDestroy`
- `onTriggerEnter` / `Exit` / `Stay`, `onCollisionEnter` / `Exit` / `Stay`

### Control flow

- `if` / `else` — statement and expression form
- `when` — subject form and condition form with `else` branch
- `for` — range-based (`until`, `downTo`, `step`)
- `while`, `break`, `continue`, `return`

### Expressions

- Operator precedence: `?:` → `||` → `&&` → `==`/`!=` → `<`/`>`/`<=`/`>=`/`is` → `..`/`until`/`downTo` → `+`/`-` → `*`/`/`/`%` → `!`/`-` → `.`/`?.`/`!!`/`[]`/`()`
- String interpolation: `$identifier` and `${expression}`
- Duration literals: `1.5s`, `500ms`
- Sugar constructors: `vec2()`, `vec3()`, `color()`
- Sugar methods: `get<T>()`, `find<T>()`, `child<T>()`, `parent<T>()`, `log()`, `warn()`, `error()`
- Input sugar: `input.axis()`, `input.getKey()`, `input.getButton()`

### Coroutines

- `coroutine` declaration (component-only)
- `wait` forms: duration, `nextFrame`, `fixedFrame`, `until`, `while`
- `start` / `stop` / `stopAll`

### Events

- `listen` — event subscription (register-only, no auto-cleanup)

### Diagnostics

| Code | Description |
|------|-------------|
| E012 | Lifecycle block in wrong context |
| E013 | Component-only field qualifier in wrong context |
| E014 | Duplicate lifecycle block |
| E020 | Type mismatch |
| E022 | Variable without type and without initializer |
| E031 | break/continue outside loop |
| E032 | wait outside coroutine |
| E040 | Assignment to immutable val |
| E041 | Assignment to require field |
| E050 | Empty enum |
| E051 | Enum entry argument count mismatch |
| E052 | Duplicate enum entry |
| E060 | Coroutine in asset/class |
| W001 | Unnecessary non-null assertion |
| W003 | Incomplete when pattern |
| W005 | Data class with no fields |

## Toolchain

- `prism` CLI: `compile`, `check`, `build`, `init`, `where`, `version`
- Watch mode: `prism build --watch`
- JSON diagnostics: `--json` flag
- `.prsmproject` TOML configuration
- Unity package: ScriptedImporter, custom inspectors, context menus
- VS Code extension: syntax highlighting, diagnostics, snippets, sidebar
