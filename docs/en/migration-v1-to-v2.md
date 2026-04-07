---
title: Version Migration
parent: Language Guide
grand_parent: English Docs
nav_order: 13
---

# Version Migration

This page covers migrating between PrSM language versions. Each version is fully backward-compatible — all programs from the previous version compile without changes.

---

## PrSM 1 → PrSM 2

### Activation

```toml
[language]
version = "2"
features = ["pattern-bindings", "input-system", "auto-unlisten"]
```

### What's new

| Feature | Description |
|---------|-------------|
| Pattern matching with bindings | `EnemyState.Chase(target) =>` in `when` branches |
| Listen lifetime model | `until disable`, `until destroy`, `manual` + `unlisten` |
| Destructuring | `val Stats(hp, speed) = getStats()` |
| New Input System sugar | `input.action("Jump").pressed` |
| Generic type inference | `val rb: Rigidbody = get()` |

### Breaking changes

None. `listen` without a modifier is unchanged (register-only). Pattern bindings are now validated — previously unchecked patterns may produce E081/E082.

### Migration steps

1. Set `version = "2"` in `.prsmproject`
2. Add desired features to `features` array
3. Run `prism build` — fix any E081/E082/E083 diagnostics
4. Adopt new features incrementally:
   - Add `until disable` to long-lived listen statements
   - Replace manual cleanup intrinsic blocks with `unlisten`
   - Use `input.action()` instead of legacy `input.getKey()`

### Rollback

Change `version = "1"` and remove v2-only syntax (`until disable`, `unlisten`, pattern bindings).

---

## PrSM 2 → PrSM 3

### Activation

```toml
[language]
version = "3"
features = ["pattern-bindings", "input-system", "auto-unlisten", "interface", "generics", "singleton", "pool", "solid-analysis", "optimizer"]
```

Or use the Unity Editor GUI: **Window > PrSM > Project Settings**.

### What's new

| Feature | Description |
|---------|-------------|
| Interface declaration | `interface IDamageable { func takeDamage(amount: Int) }` |
| Generic declaration | `class Registry<T> where T : Component { }` |
| `singleton` keyword | `singleton component GameManager : MonoBehaviour { }` |
| `pool` modifier | `pool bullets: Bullet(capacity = 20, max = 100)` |
| SOLID analysis | W010/W011/W012 warnings for design issues |
| Code optimizer | Single-binding destructure inlining |
| Reserved names | `get`, `find` are reserved built-in method names (E101) |

### Breaking changes

**E101 — Reserved method names:** If you have user-defined functions named `get` or `find`, rename them. These names are reserved for built-in sugar methods (`GetComponent`, `FindFirstObjectByType`).

```prsm
// Before (v2) — compiles but may conflict with sugar
func get(): Item { return items[0] }

// After (v3) — rename to avoid E101
func getItem(): Item { return items[0] }
```

### Migration steps

1. Set `version = "3"` in `.prsmproject` (or use **Window > PrSM > Project Settings**)
2. Run `prism build` — fix any E101 diagnostics (rename `get`/`find` functions)
3. Review SOLID warnings (W010/W011/W012) and refactor if desired
4. Adopt new features incrementally:
   - Define interfaces for component contracts
   - Use `singleton` instead of manual singleton boilerplate
   - Use `pool` instead of manual object pool management
   - Add generics to utility classes

### Rollback

Change `version = "2"` and remove v3-only syntax (`interface`, generic params, `singleton`, `pool`).

---

## PrSM 3 → PrSM 4

### Activation

```toml
[language]
version = "4"
```

`version = "4"` implicitly enables all 30 Language 4 features. The full feature flag list is documented in [PrSM 4](spec/lang-4.md).

### What's new

PrSM 4 is the largest single release of the language. It adds 30 features across four categories:

| Category | Highlights |
|----------|------------|
| Syntax extensions | `try`/`catch`/`throw`, lambdas with function types, `static`/`const`, collection literals `[1,2,3]`/`{"k":v}`, `as`/`as!` casting, property `get`/`set`, `extend Type {}`, `abstract`/`sealed`/`open`, tuples, `use`, `struct`, `event`, operator overloading, indexers, `typealias`, raw strings `"""..."""`, `in` operator, OR/range patterns, `?:=`, default interface methods |
| Async + performance | `async`/`await` (UniTask preferred), v4 optimizer rules (`opt.string`, `opt.linq`, `opt.structcopy`), Burst compatibility analysis |
| Pattern sugar | `state machine` blocks, `command` declarations, `bind` reactive properties (MVVM) |
| Developer experience | Rust/Elm-style multi-line diagnostics, LSP refactoring code actions, flat `.prsm.map` source maps |

### Breaking changes

None. All Language 3 programs compile under Language 4 without modification. Several new contextual keywords (`async`, `await`, `event`, `use`, `state`, `machine`, `command`, `bind`, `to`, `enter`, `exit`, `on`, `get`, `set`, `field`) remain valid as identifiers in non-keyword positions.

### Migration steps

1. Set `version = "4"` in `.prsmproject` (or use **Window > PrSM > Project Settings**)
2. Run `prism build` — all existing Language 3 code should compile unchanged
3. Replace existing `intrinsic { try { … } catch (Exception e) { … } }` blocks with native `try`/`catch`
4. Replace function-passing `Action`/`Func` intrinsic boilerplate with lambda expressions
5. Adopt new patterns incrementally:
   - Use collection literals (`[1, 2, 3]`) instead of `new List<int>{1, 2, 3}`
   - Use `bind` reactive properties for HUD components
   - Convert state-machine `enum` + `switch` patterns to `state machine` blocks
   - Add `async`/`await` for asynchronous content loading
6. Enable the v4 optimizer (`--optimize`) and review W026/W027 warnings to audit hot-path allocations

### Rollback

Change `version = "3"` and remove v4-only syntax (`try`/`catch`, lambdas, collection literals, `state machine`, `command`, `bind`, etc.).

---

## PrSM 4 → PrSM 5

### Activation

```toml
[language]
version = "5"
```

`version = "5"` implicitly enables all 22 Language 5 features and the 12 limitation fixes that complete partial Language 4 features. The full feature flag list is documented in [PrSM 5](spec/lang-5.md).

### What's new

PrSM 5 closes the remaining Unity-relevant gaps with C# and resolves Language 4 limitations. It adds 22 syntactic features and 12 limitation fixes:

| Category | Highlights |
|----------|------------|
| High-impact syntax | General `yield return`, attribute targets (`@field`/`@property`/`@param`/`@return`/`@type`), preprocessor directives (`#if editor`/`#if debug` etc.) |
| Common API needs | `ref`/`out` parameters, `vararg` (params), default parameter values, named arguments, `nameof` operator, `@burst` annotation, UniTask auto-detection |
| Pattern matching | Relational patterns (`> 80`), pattern combinators (`and`/`or`/`not`), positional patterns (`Point(x, y)`), property patterns (`{ hp: > 0 }`), `with` expression |
| Type system | `unmanaged`/`notnull`/`default`/`new()` constraints, `ref` local and `ref` return, `ref struct`, `stackalloc`, `Span<T>` slice syntax, `partial` declarations, generalized nested declarations |
| Tooling and DX | Discard `_`, conditional indexer `?[i]`, throw expression, LSP refactor dispatch, DAP debugger adapter |
| Limitation fixes | `bind X to Y` continuous push, W031 (bind never read), state-machine reserved-name relaxation, `opt.linq` element type inference, `opt.structcopy` realization, optimizer CLI flag, cross-context `unlisten` resolution |

### Breaking changes

None. All Language 4 programs compile under Language 5 without modification. The new contextual keywords (`yield`, `partial`, `nameof`, `vararg`, `unmanaged`, `notnull`, `ref`, `stackalloc`, `with`) remain valid as identifiers in non-keyword positions.

The `@burst` annotation now triggers diagnostics E137–E139 and W028 directly. If existing code relies on the Language 4 naming heuristic (`burst_*`) for Burst analysis, attach `@burst` explicitly to retain the same behavior.

The `async` lowering now defaults to `Task` when the UniTask package is not detected in `Packages/manifest.json`. Projects that previously relied on UniTask being emitted unconditionally should either install the UniTask package or set `[language.async] backend = "unitask"` and accept the W035 warning.

### Migration steps

1. Set `version = "5"` in `.prsmproject` (or use **Window > PrSM > Project Settings**)
2. Run `prism build` — all existing Language 4 code should compile unchanged
3. Replace `intrinsic { yield return ...; }` blocks with native `yield` / `yield return` / `yield break`
4. Replace `intrinsic { #if UNITY_EDITOR ... #endif }` blocks with native `#if editor` directives
5. Replace `intrinsic` blocks for `Physics.Raycast(out var hit)` and other `out`-parameter calls with native `out val` syntax
6. Adopt new patterns incrementally:
   - Use relational patterns and combinators in `when` instead of nested `if` chains
   - Convert manual `Deconstruct` use to positional patterns (`Point(x, y)`)
   - Convert HUD update glue to `bind X to Y` (now performs continuous push)
   - Use `@burst` annotations explicitly instead of relying on the `burst_*` naming heuristic
7. Add `@field(serializeField)` (or just `serialize`) on auto-properties exposed in the Inspector
8. Split large components into `partial component` declarations across multiple files
9. Use the DAP debugger adapter from VS Code to set breakpoints directly on `.prsm` lines

### Rollback

Change `version = "4"` and remove v5-only syntax (`yield`, `#if`, `partial`, relational patterns, `with`, `ref` locals, `stackalloc`, `?[]`, `throw` expressions, etc.).

---

## Feature flag reference

| Flag | Since | Description |
|------|-------|-------------|
| `pattern-bindings` | PrSM 2 | Enum payload binding, destructuring, when guards |
| `input-system` | PrSM 2 | Input System sugar (requires Unity Input System package) |
| `auto-unlisten` | PrSM 2 | Listen lifetime modifiers and unlisten |
| `interface` | PrSM 3 | Interface declaration |
| `generics` | PrSM 3 | Generic class/func with where clauses |
| `singleton` | PrSM 3 | Singleton component keyword |
| `pool` | PrSM 3 | Object pool modifier |
| `solid-analysis` | PrSM 3 | SOLID analysis warnings |
| `optimizer` | PrSM 3 | Code optimizer |
| `try-catch` | PrSM 4 | try/catch/finally and throw |
| `lambda` | PrSM 4 | Lambda expressions and function types |
| `static-const` | PrSM 4 | static and const members |
| `collection-literals` | PrSM 4 | List, Map, Set literals |
| `type-cast` | PrSM 4 | as / as! / smart cast |
| `property` | PrSM 4 | Property get/set with field |
| `extend` | PrSM 4 | Extension methods |
| `class-modifiers` | PrSM 4 | abstract / sealed / open |
| `tuple` | PrSM 4 | Tuple types and destructuring |
| `use` | PrSM 4 | use for IDisposable |
| `struct` | PrSM 4 | struct declaration |
| `event` | PrSM 4 | event multicast delegates |
| `operator` | PrSM 4 | Operator overloading |
| `indexer` | PrSM 4 | Indexer (operator get/set) |
| `typealias` | PrSM 4 | typealias declaration |
| `raw-string` | PrSM 4 | Raw string literals |
| `in-operator` | PrSM 4 | in membership operator |
| `or-pattern` | PrSM 4 | OR patterns in when |
| `range-pattern` | PrSM 4 | Range patterns in when |
| `null-coalesce-assign` | PrSM 4 | ?:= operator |
| `default-interface` | PrSM 4 | Default interface methods |
| `async-await` | PrSM 4 | async / await |
| `optimizer-v4` | PrSM 4 | v4 optimizer rules |
| `burst-analysis` | PrSM 4 | Burst compatibility analysis |
| `state-machine` | PrSM 4 | state machine sugar |
| `command` | PrSM 4 | command sugar |
| `bind` | PrSM 4 | bind reactive properties |
| `error-enhancement` | PrSM 4 | Rust/Elm-style diagnostics |
| `refactor-tools` | PrSM 4 | LSP refactoring code actions |
| `debugger` | PrSM 4 | Flat source map generation |
| `yield-general` | PrSM 5 | General `yield return` / `yield break` in coroutines |
| `attribute-target` | PrSM 5 | `@field` / `@property` etc. attribute targets |
| `preprocessor` | PrSM 5 | `#if` / `#elif` / `#else` / `#endif` directives |
| `ref-out-params` | PrSM 5 | `ref` / `out` parameters |
| `vararg` | PrSM 5 | `vararg` (params) parameters |
| `default-params` | PrSM 5 | Default parameter values |
| `named-args` | PrSM 5 | Named call arguments |
| `nameof` | PrSM 5 | `nameof` operator |
| `burst-annotation` | PrSM 5 | `@burst` annotation |
| `unitask-detect` | PrSM 5 | UniTask auto-detection |
| `bind-push` | PrSM 5 | bind X to Y continuous push |
| `bind-unread-warn` | PrSM 5 | bind W031 implementation |
| `state-name-relax` | PrSM 5 | State machine reserved name allowance |
| `opt-linq-types` | PrSM 5 | opt.linq element type inference |
| `opt-structcopy-ref` | PrSM 5 | opt.structcopy ref readonly realization |
| `optimizer-cli` | PrSM 5 | optimizer driver auto-wire and CLI |
| `unlisten-cross` | PrSM 5 | cross-context unlisten resolution |
| `relational-pattern` | PrSM 5 | relational pattern |
| `pattern-combinator` | PrSM 5 | and / or / not pattern combinators |
| `positional-pattern` | PrSM 5 | positional pattern |
| `property-pattern` | PrSM 5 | property pattern |
| `with-expr` | PrSM 5 | with expression |
| `unmanaged-constraint` | PrSM 5 | unmanaged / notnull / default constraints |
| `ref-local` | PrSM 5 | ref local and ref return |
| `ref-struct` | PrSM 5 | ref struct declaration |
| `stackalloc` | PrSM 5 | stackalloc expression |
| `span-slice` | PrSM 5 | span / array range slicing |
| `partial` | PrSM 5 | partial class / component / struct |
| `nested-decl` | PrSM 5 | generalized nested declarations |
| `discard` | PrSM 5 | discard `_` expression / pattern |
| `safe-index` | PrSM 5 | conditional indexer `?[i]` |
| `throw-expr` | PrSM 5 | throw expression |
| `lsp-refactor-dispatch` | PrSM 5 | LSP refactor code action dispatch |
| `dap-adapter` | PrSM 5 | DAP debug adapter |

Setting `version = "N"` implicitly enables all features for that version and below.
