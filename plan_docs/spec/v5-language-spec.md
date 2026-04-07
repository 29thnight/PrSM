# PrSM Language 5 Specification — Draft

**Status:** Draft v0.1
**Date:** 2026-04-07
**Prerequisite:** Language 4 (PrSM Language Standard, Prism v2.0.0)
**Target:** Unity 2022.3+ (IL2CPP / Mono)
**Tool version:** Prism v3.0.0 (Language 5 = Prism 3.0)

---

Language 5 closes the remaining Unity-relevant gaps between PrSM and C# and resolves the known limitations in Language 4. It adds **22 syntactic features** organized into six implementation sprints, plus **12 limitation fixes** to existing Language 4 features. All Language 4 programs are retained and continue to compile without changes. The new contextual keywords (`yield`, `partial`, `nameof`, `vararg`, `unmanaged`, `notnull`, `ref`, `stackalloc`, `with`) shall not break existing identifiers because they are recognized only in their respective syntactic positions.

This document defines additions and changes relative to Language 4. The full specification will be merged into `docs/en/spec/standard.md` upon Language 5 finalization.

---

# Part I. Sprint 1 — High-impact syntax (1-3)

The three features in this part account for the largest single drop in `intrinsic` usage observed across realistic Unity projects. They are implemented first to maximize developer impact.

---

## 1. General `yield return` [stmt.yield]

### 1.1 Grammar

```ebnf
YieldStmt      = "yield" Expr
              | "yield" "break"
CoroutineDecl  = "coroutine" Identifier "(" [ ParamList ] ")" [ ":" TypeRef ] Block
```

The `coroutine` declaration form is reused from earlier PrSM versions but is now extended to allow general `yield return` and `yield break` statements in its body, in addition to the `wait` shortcuts that were the only supported form in Language 1–4.

### 1.2 Semantics

`yield expr` suspends the enclosing coroutine and emits `expr` as the next value from the iterator. `yield break` terminates the coroutine immediately. The enclosing function shall be either:

- A `coroutine` declaration (lowered to `IEnumerator` or `IEnumerator<T>`), or
- A `func` whose declared return type is `Seq<T>`, `IEnumerator`, `IEnumerator<T>`, `IEnumerable`, or `IEnumerable<T>`

If `yield` appears outside any of the above contexts, the compiler emits **E147**.

The existing `wait` / `start` / `stop` / `stopAll` statements remain valid and lower to their established forms. `wait <duration>` is sugar for `yield return new WaitForSeconds(d)` and may freely coexist with general `yield` statements within the same coroutine body.

### 1.3 Example

```prsm
component Cutscene : MonoBehaviour {
    coroutine countdown(): Seq<Int> {
        for i in 5 downTo 1 {
            yield i
            wait 1s
        }
        yield 0
        yield break
    }

    coroutine fadeOut(): IEnumerator {
        var t = 1.0
        while t > 0.0 {
            t -= Time.deltaTime
            canvasGroup.alpha = t.toFloat()
            yield return null
        }
    }
}
```

### 1.4 Lowering

```csharp
public IEnumerator<int> countdown()
{
    for (int i = 5; i >= 1; i--)
    {
        yield return i;
        yield return new WaitForSeconds(1.0f);
    }
    yield return 0;
    yield break;
}

public IEnumerator fadeOut()
{
    var t = 1.0;
    while (t > 0.0)
    {
        t -= Time.deltaTime;
        canvasGroup.alpha = (float)t;
        yield return null;
    }
}
```

`Seq<T>` lowers to `IEnumerator<T>`. `IEnumerator` and `IEnumerable` map straight through. `wait`-style statements continue to lower to their established `WaitForSeconds`/`WaitForFixedUpdate`/`WaitUntil`/`WaitWhile` forms.

### 1.5 Diagnostics

| Code | Condition |
|------|-----------|
| E147 | `yield` used outside a coroutine or iterator-returning function |
| E148 | `yield` value type does not match the declared element type |
| W033 | Coroutine declares `Seq<T>` return type but never `yield`s a value of `T` |

---

## 2. Attribute target on properties [attr.target]

### 2.1 Grammar

```ebnf
PropertyDecl     = ( "val" | "var" ) [ "serialize" ] Identifier ":" TypeRef [ "=" Expr ]
                  [ Getter ] [ Setter ]
AttrTargetDecl   = "@" AttrTarget "(" AttrName [ "," AttrArgs ] ")"
AttrTarget       = "field" | "property" | "param" | "return" | "type"
```

### 2.2 Semantics

A property declared with the `serialize` modifier and explicit `get` / `set` accessors lowers to a C# auto-property whose **backing field** carries the `[SerializeField]` attribute. This is the idiomatic Unity pattern for exposing an auto-property in the Inspector while keeping the public surface a property.

The general form `@field(name)`, `@property(name)`, `@param(name)`, `@return(name)`, `@type(name)` lets the user attach any C# attribute to a non-default target on any declaration. `field`/`property`/`param`/`return`/`type` correspond to the C# attribute targets `[field: ...]`, `[property: ...]`, `[param: ...]`, `[return: ...]`, `[type: ...]`.

A `serialize` modifier on an auto-property is itself sugar for `@field(serializeField)` and is the recommended form for the Unity case.

### 2.3 Example

```prsm
component Player : MonoBehaviour {
    serialize var hp: Int = 100
        get
        set { field = Mathf.clamp(value, 0, maxHp) }

    @field(nonSerialized)
    var transientCache: Map<String, Int>

    @return(notNull)
    func getTarget(): Transform = currentTarget
}
```

### 2.4 Lowering

```csharp
[field: SerializeField]
public int hp
{
    get;
    set { field = Mathf.Clamp(value, 0, maxHp); }
}

[field: NonSerialized]
public Dictionary<string, int> transientCache { get; set; }

[return: NotNull]
public Transform getTarget() => currentTarget;
```

The `field` keyword inside the setter is the same Kotlin-style backing field reference introduced in Language 4. When combined with attribute targets, the lowered C# uses C# 11+ field-keyword auto-properties on supported targets, falling back to a synthetic backing field on older C# versions.

### 2.5 Diagnostics

| Code | Condition |
|------|-----------|
| E149 | `@field`/`@property`/etc. on a declaration that does not support the chosen target |
| E150 | `serialize` on a property whose accessors are absent or have a body that prevents auto-property lowering |

---

## 3. Preprocessor directives [pp]

### 3.1 Grammar

```ebnf
IfDirective    = "#if" Condition Block { ElseIfDirective } [ ElseDirective ] "#endif"
ElseIfDirective = "#elif" Condition Block
ElseDirective  = "#else" Block
Condition      = SymbolName | SymbolName "(" Args ")" | "!" Condition
                | Condition "&&" Condition | Condition "||" Condition | "(" Condition ")"
SymbolName     = "editor" | "debug" | "release" | "ios" | "android" | "standalone"
                | "il2cpp" | "mono" | "unity20223" | "unity20231" | "unity6"
                | Identifier  // raw C# define passes through
```

### 3.2 Semantics

Preprocessor directives in PrSM map directly to C# preprocessor directives. PrSM provides a curated set of platform symbols that are commonly used in Unity projects; these are translated to the corresponding `UNITY_*` defines. Any other identifier is passed through verbatim, allowing user-defined symbols (e.g. `MY_FEATURE`).

The following symbol mapping is normative:

| PrSM symbol | C# define |
|-------------|-----------|
| `editor` | `UNITY_EDITOR` |
| `debug` | `DEBUG` |
| `release` | `!DEBUG` |
| `ios` | `UNITY_IOS` |
| `android` | `UNITY_ANDROID` |
| `standalone` | `UNITY_STANDALONE` |
| `il2cpp` | `ENABLE_IL2CPP` |
| `mono` | `ENABLE_MONO` |
| `unity20223` | `UNITY_2022_3_OR_NEWER` |
| `unity20231` | `UNITY_2023_1_OR_NEWER` |
| `unity6` | `UNITY_6000_0_OR_NEWER` |

`#if` blocks are valid in any statement, member, or top-level position.

### 3.3 Example

```prsm
component Player : MonoBehaviour {
    update {
        move()

        #if editor
            drawDebugGizmos()
        #endif

        #if ios && !editor
            handleHaptics()
        #elif android
            handleVibration()
        #endif
    }

    #if debug
        func logState() { log("hp=$hp, pos=${transform.position}") }
    #endif
}
```

### 3.4 Lowering

```csharp
public class Player : MonoBehaviour
{
    void Update()
    {
        move();

        #if UNITY_EDITOR
            drawDebugGizmos();
        #endif

        #if UNITY_IOS && !UNITY_EDITOR
            handleHaptics();
        #elif UNITY_ANDROID
            handleVibration();
        #endif
    }

    #if DEBUG
        public void logState() { Debug.Log($"hp={hp}, pos={transform.position}"); }
    #endif
}
```

### 3.5 Diagnostics

| Code | Condition |
|------|-----------|
| E151 | Unterminated `#if` block (missing `#endif`) |
| E152 | `#elif` or `#else` without matching `#if` |
| W034 | Unknown preprocessor symbol — passed through verbatim, may not exist in target |

---

# Part II. Sprint 2 — Common Unity API needs (4-7)

The four features in this part target the most common reasons developers reach for `intrinsic` when calling Unity API methods.

---

## 4. `ref` / `out` parameters [param.ref]

### 4.1 Grammar

```ebnf
Param        = [ ParamMod ] Identifier ":" TypeRef [ "=" Expr ]
ParamMod     = "ref" | "out"
RefArg       = "ref" Expr
OutArg       = "out" ( Expr | "val" Identifier | "var" Identifier | "_" )
```

### 4.2 Semantics

`ref` parameters allow a method to modify the caller's variable in place. `out` parameters require the callee to assign before returning. Both lower to the matching C# parameter modifier. Call sites use `out val name` for declaration expressions (the C# `out var name` form).

### 4.3 Example

```prsm
func tryParse(input: String, out value: Int): Bool {
    intrinsic { return int.TryParse(input, out value); }
}

if physics.raycast(ray, out val hit) {
    log("hit ${hit.collider.name}")
}

if physics.raycast(ray, out _) {
    log("something hit")
}
```

### 4.4 Lowering

```csharp
public bool tryParse(string input, out int value)
{
    return int.TryParse(input, out value);
}

if (Physics.Raycast(ray, out var hit))
{
    Debug.Log($"hit {hit.collider.name}");
}

if (Physics.Raycast(ray, out _))
{
    Debug.Log("something hit");
}
```

### 4.5 Diagnostics

| Code | Condition |
|------|-----------|
| E153 | `ref`/`out` argument must match parameter modifier |
| E154 | `out` parameter not assigned before all return paths |
| E155 | `ref` parameter passed an immutable value (`val`) |

---

## 5. `vararg` (params) parameters [param.vararg]

### 5.1 Grammar

```ebnf
Param        = [ ParamMod ] Identifier ":" TypeRef [ "=" Expr ]
ParamMod     = "ref" | "out" | "vararg"
```

### 5.2 Semantics

A parameter declared with the `vararg` modifier accepts zero or more arguments of the declared type. Only the **last** parameter of a function may be `vararg`. The function body sees the parameter as an `Array<T>`. Lowers to a C# `params T[]` parameter.

### 5.3 Example

```prsm
func log(vararg messages: String) {
    for msg in messages {
        Debug.Log(msg)
    }
}

log()
log("loading")
log("step 1", "step 2", "step 3")
```

### 5.4 Lowering

```csharp
public void log(params string[] messages)
{
    foreach (var msg in messages)
    {
        Debug.Log(msg);
    }
}

log();
log("loading");
log("step 1", "step 2", "step 3");
```

### 5.5 Diagnostics

| Code | Condition |
|------|-----------|
| E156 | `vararg` modifier on a non-final parameter |
| E157 | More than one `vararg` parameter in a single function |

---

## 6. Default parameter values [param.default]

### 6.1 Grammar

```ebnf
Param = [ ParamMod ] Identifier ":" TypeRef [ "=" DefaultExpr ]
DefaultExpr = LiteralExpr | NullLiteral | "default"
```

### 6.2 Semantics

A parameter declaration may include `= expr` to provide a default value. The default expression shall be a compile-time constant (literal, `null`, `default`, or a `const` reference). All parameters with defaults shall appear after all required parameters. Call sites may omit trailing arguments when defaults are available.

### 6.3 Example

```prsm
func instantiate(prefab: GameObject, parent: Transform? = null, worldSpace: Bool = false): GameObject {
    return GameObject.Instantiate(prefab, parent, worldSpace)
}

instantiate(bulletPrefab)
instantiate(bulletPrefab, weaponSocket)
instantiate(bulletPrefab, weaponSocket, true)
```

### 6.4 Lowering

```csharp
public GameObject instantiate(GameObject prefab, Transform parent = null, bool worldSpace = false)
{
    return GameObject.Instantiate(prefab, parent, worldSpace);
}
```

### 6.5 Diagnostics

| Code | Condition |
|------|-----------|
| E158 | Default value is not a compile-time constant |
| E159 | Required parameter follows a parameter with a default value |

---

## 7. Named arguments [call.named]

### 7.1 Grammar

```ebnf
Arg          = [ Identifier ":" ] Expr
```

### 7.2 Semantics

Call sites may specify arguments by parameter name. Named arguments may appear in any order, but no positional argument shall follow a named argument (consistent with C#). Named arguments are particularly useful for Unity APIs with long parameter lists or boolean flags.

### 7.3 Example

```prsm
GameObject.Instantiate(
    original: bulletPrefab,
    position: spawnPoint.position,
    rotation: Quaternion.identity,
    parent: bulletContainer,
)
```

### 7.4 Lowering

C# uses identical syntax — the lowering is a direct pass-through.

```csharp
GameObject.Instantiate(
    original: bulletPrefab,
    position: spawnPoint.position,
    rotation: Quaternion.identity,
    parent: bulletContainer);
```

### 7.5 Diagnostics

| Code | Condition |
|------|-----------|
| E160 | Positional argument after a named argument |
| E161 | Unknown parameter name |
| E162 | Argument provided for a parameter twice (positional + named) |

---

## 8. `nameof` operator [expr.nameof]

### 8.1 Grammar

```ebnf
NameOfExpr = "nameof" "(" QualifiedIdent ")"
```

### 8.2 Semantics

`nameof(x)` evaluates at compile time to the string `"x"`. `nameof(Type.Member)` evaluates to `"Member"` (just the trailing identifier, matching C# behavior). The argument shall reference a real symbol (variable, parameter, member, type) — the compiler verifies its existence and emits **E163** otherwise.

`nameof` is recognized as a contextual keyword: existing user identifiers named `nameof` remain valid in expression positions where the `(` lookahead does not follow.

### 8.3 Example

```prsm
component Player : MonoBehaviour {
    bind hp: Int = 100
        set {
            field = value
            onPropertyChanged(nameof(hp))
        }

    require rb: Rigidbody

    awake {
        if rb == null {
            error("Required component ${nameof(Rigidbody)} is missing")
        }
    }
}
```

### 8.4 Lowering

```csharp
public int hp { get => _hp; set { _hp = value; OnPropertyChanged(nameof(hp)); } }
// ...
if (rb == null) { Debug.LogError($"Required component {nameof(Rigidbody)} is missing"); }
```

### 8.5 Diagnostics

| Code | Condition |
|------|-----------|
| E163 | `nameof` argument does not reference an existing symbol |
| E164 | `nameof` argument is an expression that does not resolve to a single identifier path |

---

## 9. `@burst` annotation [annotation.burst]

### 9.1 Grammar

```ebnf
Annotation     = "@" Identifier [ "(" Args ")" ]
FuncDecl       = { Annotation } [ FuncMod ] "func" Identifier "(" [ ParamList ] ")" ...
StructDecl     = { Annotation } [ "ref" ] "struct" ...
```

### 9.2 Semantics

The `@burst` annotation marks a function or struct for Unity Burst compilation. The compiler emits the corresponding `[BurstCompile]` attribute and runs the Burst compatibility analyzer (introduced in Language 4) on the annotated definition. Diagnostics **E137**–**E139** and **W028** from Language 4 are now triggered by the annotation rather than by the naming heuristic (`burst_*`) used previously.

The annotation may carry options (`@burst(compileSynchronously = true)`) that lower to the matching `[BurstCompile]` attribute arguments.

### 9.3 Example

```prsm
@burst
func calculateForces(positions: NativeArray<Float3>, forces: NativeArray<Float3>) {
    for i in 0..positions.length {
        forces[i] = computeGravity(positions[i])
    }
}

@burst(compileSynchronously = true)
struct DamageJob : IJobParallelFor {
    var damages: NativeArray<Int>

    func execute(index: Int) {
        damages[index] = damages[index] * 2
    }
}
```

### 9.4 Lowering

```csharp
[BurstCompile]
public void calculateForces(NativeArray<float3> positions, NativeArray<float3> forces)
{
    for (int i = 0; i < positions.Length; i++)
    {
        forces[i] = computeGravity(positions[i]);
    }
}

[BurstCompile(CompileSynchronously = true)]
public struct DamageJob : IJobParallelFor
{
    public NativeArray<int> damages;
    public void execute(int index)
    {
        damages[index] = damages[index] * 2;
    }
}
```

### 9.5 Diagnostics

| Code | Condition |
|------|-----------|
| E165 | `@burst` annotation on an unsupported declaration kind (component, asset, interface, etc.) |

The Language 4 diagnostics **E137**, **E138**, **E139**, and **W028** continue to apply, but are now triggered by the `@burst` annotation rather than by naming heuristics.

---

## 10. UniTask auto-detection [async.unitask]

### 10.1 Overview

This is a compiler driver behavior change with no syntactic surface. The Language 4 implementation always emitted `Cysharp.Threading.Tasks.UniTask` for `async func` lowering, which broke compilation for projects without the UniTask package.

### 10.2 Detection algorithm

The compiler scans the project's `Packages/manifest.json` and `.prsmproject`. If `com.cysharp.unitask` is present in the dependencies, the compiler emits `UniTask` / `UniTask<T>`. Otherwise it falls back to `System.Threading.Tasks.Task` / `Task<T>`.

The detection result is stored in compile options and can be overridden via:

```toml
[language.async]
backend = "unitask"  # "unitask" | "task" | "auto"  (default: "auto")
```

### 10.3 Lowering

For `async func loadData(): String` with UniTask detected:

```csharp
public async UniTask<string> loadData() { ... }
```

Without UniTask:

```csharp
public async Task<string> loadData() { ... }
```

### 10.4 Diagnostics

| Code | Condition |
|------|-----------|
| W035 | `language.async.backend = "unitask"` requested but UniTask package not found in manifest |

---

# Part III. Sprint 3 — Language 4 limitation fixes (11-15)

This part resolves the known limitations in Language 4 features that were marked as "implemented but partial". Each item upgrades a v4 feature from "partial" to "complete".

---

## 11. `bind X to Y` continuous push [sugar.bind.push]

### 11.1 Overview

Language 4 implemented `bind X to Y` with initial synchronization only — the bound target was set once when the `bind to` statement executed but did not update on subsequent property changes. Language 5 completes the reactive contract.

### 11.2 Semantics

For each `bind` member declared in a component, the compiler maintains a list of **push targets** (lambdas of type `(T) => Unit` where `T` is the bind member type). A `bind X to Y` statement registers a push target lambda `v => Y = v` and immediately invokes it once for the initial sync.

The setter generated for the bind property iterates the push target list after `OnPropertyChanged` and invokes each lambda with the new value.

### 11.3 Lowering

```csharp
private int _hp = 100;
private List<System.Action<int>> _hpPushTargets;

public int hp
{
    get => _hp;
    set
    {
        if (_hp != value)
        {
            _hp = value;
            OnPropertyChanged(nameof(hp));
            if (_hpPushTargets != null)
                foreach (var t in _hpPushTargets) t(value);
        }
    }
}
```

The `bind X to Y` statement lowers to:

```csharp
_hpPushTargets ??= new List<System.Action<int>>();
_hpPushTargets.Add(v => hpLabel.text = v.ToString());
hpLabel.text = _hp.ToString();
```

### 11.4 Diagnostics

The Language 4 diagnostics **E143** (bind target not writable) and **E144** (type mismatch) continue to apply.

---

## 12. `bind` never-read warning [sugar.bind.unread]

### 12.1 Overview

Language 4 reserved the diagnostic code **W031** for "bind property never read" but the analyzer did not implement the use-site tracking necessary to detect the condition.

### 12.2 Semantics

After semantic analysis completes for a component, the compiler scans all expressions inside the component for references to each `bind` member. A bind member with zero references (other than its own setter) emits **W031** at the bind declaration site.

### 12.3 Diagnostics

| Code | Condition |
|------|-----------|
| W031 | `bind` member declared but never read elsewhere in the component (now implemented) |

---

## 13. State machine reserved-name relaxation [sugar.state.naming]

### 13.1 Overview

Language 4 used `expect_ident` for state names in `state machine` blocks, which forbade PrSM reserved keywords (`Start`, `Stop`, `Update`, etc.) as state names — colliding with common Unity state naming conventions.

### 13.2 Semantics

Within the body of a `state` declaration in a `state machine` block, the parser shall accept reserved keywords as state name identifiers. This relaxation applies only to the state name position and only inside `state machine` blocks; reserved keywords retain their normal meaning everywhere else.

### 13.3 Example

```prsm
state machine playerState {
    state Start { on begin => Update }
    state Update { on pause => Stop }
    state Stop { }
}
```

### 13.4 Lowering

The reserved names are emitted verbatim into the generated enum (Roslyn accepts these as enum members):

```csharp
private enum PlayerState { Start, Update, Stop }
```

If the chosen names happen to be C# reserved keywords (none of the PrSM keywords are also C# reserved), they would be prefixed with `@`. PrSM keywords are not a subset of C# keywords so no prefixing is required.

---

## 14. `opt.linq` element type inference [opt.linq.types]

### 14.1 Overview

Language 4's `opt.linq` rewrite emitted rewritten LINQ chains as `for` loops with `var` element types, which fell back to `object` when the source list's static type could not be inferred at the optimizer pass — causing boxing.

### 14.2 Algorithm

The optimizer now propagates element type information from the AST through the IR walk by consulting the `collect_callable_signatures` map. For a chain `xs.Where(p).ToList()` where `xs : List<T>`, the rewritten loop uses `T` directly:

```csharp
var alive = new List<Enemy>();
for (int i = 0; i < enemies.Count; i++)
{
    if (enemies[i].IsAlive) alive.Add(enemies[i]);
}
```

If element type cannot be statically determined, the optimizer skips the rewrite for that site rather than falling back to `object`.

### 14.3 Diagnostics

The Language 4 diagnostic **W027** continues to apply.

---

## 15. `opt.structcopy` realization via `ref readonly` [opt.structcopy.ref]

### 15.1 Overview

Language 4's `opt.structcopy` pass only inserted `// opt.structcopy` comment hints because PrSM had no syntax for `ref readonly` locals. With `ref local` introduced in Sprint 5 (Section 24), the optimizer can now perform actual `ref readonly` substitution.

### 15.2 Algorithm

For a hot-path local declaration `val pos: Vector3 = transform.position` (where `Vector3`, `Quaternion`, or `Matrix4x4` is the type and the local is only read), the optimizer rewrites it to:

```csharp
ref readonly Vector3 pos = ref transform.position;
```

This eliminates the structure copy that the original `var` declaration would have caused.

### 15.3 Diagnostics

A new informational diagnostic is added to make optimization auditable:

| Code | Condition |
|------|-----------|
| W036 | Large struct local in hot path rewritten as `ref readonly` |

---

## 16. Optimizer driver auto-wire and CLI flag [opt.cli]

### 16.1 Overview

Language 4 implemented the optimizer module but did not wire it into the `compile_file` pipeline; the optimizer was reachable only through the `run_optimizer()` API.

### 16.2 Driver integration

The compiler driver gains a new compile option `optimize: bool` (default `false` in debug, `true` in release). When set, the optimizer pass runs after lowering and before codegen.

CLI surface:

```bash
prism build --optimize          # enable optimizer
prism build --no-optimize       # disable optimizer
prism build                     # default (debug=off, release=on)
```

`.prsmproject` configuration:

```toml
[compiler]
optimize = true                 # default override
```

### 16.3 Diagnostics

The Language 4 optimizer diagnostics **W026**, **W027** continue to apply, plus **W036** introduced in Section 15.

---

## 17. `unlisten` cross-context resolution [stmt.unlisten.scope]

### 17.1 Overview

Language 4 emitted a placeholder comment for `unlisten` statements that appeared outside the original `listen` context (e.g., in a helper function called from a lifecycle block). Language 5 resolves these correctly.

### 17.2 Algorithm

The semantic analyzer collects all `unlisten` call sites per component during the walk. The component lowering pass then post-processes each site:

1. Look up the listen handler field by name in the component's listen registry
2. Emit `event.RemoveListener(_handlerField); _handlerField = null;` at the unlisten site
3. If no matching listen exists, emit **E166**

This works regardless of which member contains the `unlisten` statement, as long as the matching `listen … manual` is declared in the same component.

### 17.3 Diagnostics

| Code | Condition |
|------|-----------|
| E166 | `unlisten <token>` has no matching `listen … manual` declaration in the enclosing component |

---

# Part IV. Sprint 4 — Pattern matching expansion (18-22)

This part brings PrSM pattern matching to C# 9–11 parity. All five features compose with the existing `when` statement and OR/range patterns introduced in Language 4.

---

## 18. Relational pattern [pattern.relational]

### 18.1 Grammar

```ebnf
RelationalPattern = ( "<" | ">" | "<=" | ">=" ) Expr
```

### 18.2 Semantics

A relational pattern matches if the subject value compares to the operand using the specified operator. Permitted on integral, floating-point, and types implementing `IComparable<T>`.

### 18.3 Example

```prsm
when hp {
    > 80 => "Healthy"
    > 30 => "Hurt"
    > 0  => "Critical"
    else => "Dead"
}
```

### 18.4 Lowering

```csharp
hp switch
{
    > 80 => "Healthy",
    > 30 => "Hurt",
    > 0  => "Critical",
    _    => "Dead",
}
```

### 18.5 Diagnostics

| Code | Condition |
|------|-----------|
| E167 | Relational pattern operand type does not match the subject type |
| W037 | Subsequent relational arm is unreachable (covered by previous arm) |

---

## 19. Pattern combinators (`and` / `or` / `not`) [pattern.combine]

### 19.1 Grammar

```ebnf
Pattern        = OrPattern
OrPattern      = AndPattern { "or" AndPattern }
AndPattern     = NotPattern { "and" NotPattern }
NotPattern     = [ "not" ] PrimaryPattern
```

### 19.2 Semantics

`and`, `or`, `not` form a typical pattern algebra with the precedence shown above (`not` highest, then `and`, then `or`). The new `or` keyword unifies with the existing comma-OR pattern from Language 4: `A, B` and `A or B` are equivalent and produce the same lowering.

All pattern combinator arms in an `or` group shall bind the same variables (or none).

### 19.3 Example

```prsm
when x {
    > 0 and < 100 => "valid range"
    is Enemy or is Boss => "hostile"
    not null => "present"
    else => "missing"
}
```

### 19.4 Lowering

```csharp
x switch
{
    > 0 and < 100 => "valid range",
    Enemy or Boss => "hostile",
    not null => "present",
    _ => "missing",
}
```

### 19.5 Diagnostics

| Code | Condition |
|------|-----------|
| E168 | `or` pattern arms bind different variables (extends Language 4 E130) |

---

## 20. Positional pattern [pattern.positional]

### 20.1 Grammar

```ebnf
PositionalPattern = TypeName "(" [ Pattern { "," Pattern } ] ")"
```

### 20.2 Semantics

A positional pattern matches if the subject is of the specified type and each subpattern matches the corresponding `Deconstruct` output (or auto-generated deconstruction for `data class`, `struct`, and tuples). Generalizes the Language 2 enum payload binding to all destructurable types.

### 20.3 Example

```prsm
data class Point(x: Int, y: Int)

when point {
    Point(0, 0) => "origin"
    Point(0, _) => "on y axis"
    Point(_, 0) => "on x axis"
    Point(x, y) if x == y => "diagonal"
    else => "elsewhere"
}
```

### 20.4 Lowering

```csharp
point switch
{
    Point(0, 0) => "origin",
    Point(0, _) => "on y axis",
    Point(_, 0) => "on x axis",
    Point(var x, var y) when x == y => "diagonal",
    _ => "elsewhere",
}
```

For `data class`, the compiler auto-generates a `Deconstruct` method during lowering. For `struct` declarations, the same applies.

### 20.5 Diagnostics

| Code | Condition |
|------|-----------|
| E169 | Positional pattern arity does not match the type's deconstruction |

---

## 21. Property pattern [pattern.property]

### 21.1 Grammar

```ebnf
PropertyPattern = TypeName? "{" [ PropPatternEntry { "," PropPatternEntry } ] "}"
PropPatternEntry = Identifier ":" Pattern
```

### 21.2 Semantics

A property pattern matches if the subject is of the (optionally specified) type and each named property's value matches the corresponding subpattern. Property names shall be public readable members.

### 21.3 Example

```prsm
when target {
    Enemy { hp: > 0, level: > 10 } => "tough enemy"
    Enemy { hp: 0 } => "dead enemy"
    Player { isInvincible: true } => "untouchable"
    else => "ignore"
}
```

### 21.4 Lowering

```csharp
target switch
{
    Enemy { hp: > 0, level: > 10 } => "tough enemy",
    Enemy { hp: 0 } => "dead enemy",
    Player { isInvincible: true } => "untouchable",
    _ => "ignore",
}
```

### 21.5 Diagnostics

| Code | Condition |
|------|-----------|
| E170 | Property pattern references a member that does not exist on the subject type |
| E171 | Property pattern references a non-readable member |

---

## 22. `with` expression [expr.with]

### 22.1 Grammar

```ebnf
WithExpr = Expr "with" "{" FieldAssign { "," FieldAssign } "}"
FieldAssign = Identifier "=" Expr
```

### 22.2 Semantics

`expr with { f = v, … }` produces a copy of `expr` with the specified fields replaced. For `data class`, this lowers to a C# `record with` expression. For `struct` declarations, this lowers to a temporary copy with field mutations followed by the result. For Unity built-in struct types (`Vector3`, `Quaternion`, etc.), the lowering uses the temporary-copy form.

### 22.3 Example

```prsm
val origin = transform.position
val grounded = origin with { y = 0.0 }

data class PlayerStats(hp: Int, mp: Int, level: Int)

val current = PlayerStats(100, 50, 5)
val healed = current with { hp = 100 }
val leveled = healed with { level = 6, mp = 100 }
```

### 22.4 Lowering

```csharp
var origin = transform.position;
Vector3 grounded;
{
    var _t = origin;
    _t.y = 0.0f;
    grounded = _t;
}

public record PlayerStats(int hp, int mp, int level);
var current = new PlayerStats(100, 50, 5);
var healed = current with { hp = 100 };
var leveled = healed with { level = 6, mp = 100 };
```

For Unity struct types the temporary-copy form is used because they are not records. For `data class` types the compiler emits the type as a C# record so that `with` is supported natively.

### 22.5 Diagnostics

| Code | Condition |
|------|-----------|
| E172 | `with` expression on a type that is neither a `data class`, `struct`, nor a known Unity struct |
| E173 | `with` field is not a writable member |

---

# Part V. Sprint 5 — Type system extensions (23-29)

---

## 23. `unmanaged` and other where constraints [generic.constraint]

### 23.1 Grammar

```ebnf
WhereConstraint = TypeRef
                | "class"
                | "struct"
                | "unmanaged"
                | "notnull"
                | "default"
                | "new" "(" ")"
```

### 23.2 Semantics

Extends Language 3 generic constraints with `unmanaged`, `notnull`, `default`, and `new()`. The `unmanaged` constraint requires `T` to be a value type with no managed references at any depth — this is the standard constraint required for Burst-compatible generic methods.

### 23.3 Example

```prsm
@burst
func sum<T>(arr: NativeArray<T>): T where T : unmanaged, INumber<T> {
    var total = T.Zero
    for i in 0..arr.length {
        total += arr[i]
    }
    return total
}
```

### 23.4 Lowering

```csharp
[BurstCompile]
public T sum<T>(NativeArray<T> arr) where T : unmanaged, INumber<T>
{
    var total = T.Zero;
    for (int i = 0; i < arr.Length; i++)
    {
        total += arr[i];
    }
    return total;
}
```

### 23.5 Diagnostics

| Code | Condition |
|------|-----------|
| E174 | `unmanaged` and `class` constraints on the same type parameter |
| E175 | `notnull` constraint on a value type parameter |

---

## 24. `ref` local and `ref` return [type.ref]

### 24.1 Grammar

```ebnf
RefValDecl   = "val" "ref" Identifier [ ":" TypeRef ] "=" RefExpr
RefVarDecl   = "var" "ref" Identifier [ ":" TypeRef ] "=" RefExpr
RefExpr      = "ref" Expr
RefReturnTy  = "ref" TypeRef
```

### 24.2 Semantics

`val ref name = ref expr` creates a read-only reference local. `var ref name = ref expr` creates a mutable reference local. A function may declare a `ref` return type to return a reference. References are subject to C#'s ref safety rules — the compiler verifies via semantic analysis that the referenced storage outlives the reference.

`val ref` lowers to C# `ref readonly`. `var ref` lowers to C# `ref`.

### 24.3 Example

```prsm
struct Particles(positions: NativeArray<Float3>) {
    func getPosition(index: Int): ref Float3 = ref positions[index]
}

func process(particles: Particles) {
    val ref pos = ref particles.getPosition(0)
    log("position: $pos")  // no copy
}
```

### 24.4 Lowering

```csharp
public struct Particles
{
    public NativeArray<float3> positions;
    public ref float3 getPosition(int index) => ref positions[index];
}

public void process(Particles particles)
{
    ref readonly float3 pos = ref particles.getPosition(0);
    Debug.Log($"position: {pos}");
}
```

### 24.5 Diagnostics

| Code | Condition |
|------|-----------|
| E176 | `ref` local outlives the referenced storage |
| E177 | `ref` return references a local variable |
| E178 | `val ref` (read-only reference) used in a write context |

---

## 25. `ref struct` [decl.refstruct]

### 25.1 Grammar

```ebnf
StructDecl = { Annotation } [ "ref" ] "struct" Identifier "(" ParamList ")" [ "{" { Member } "}" ]
```

### 25.2 Semantics

`ref struct` declares a stack-only value type that may contain `ref` fields. Subject to C# ref struct restrictions: cannot be a field of a non-ref struct, cannot be boxed, cannot be used as a generic type argument (unless the constraint is `allows ref struct` in C# 13+).

### 25.3 Example

```prsm
ref struct Slice<T>(start: Int, length: Int) {
    func get(i: Int): T = intrinsic { return _data[start + i]; }
}
```

### 25.4 Lowering

```csharp
public ref struct Slice<T>
{
    public int start;
    public int length;
    public Slice(int start, int length) { this.start = start; this.length = length; }
    public T get(int i) { return _data[start + i]; }
}
```

### 25.5 Diagnostics

| Code | Condition |
|------|-----------|
| E179 | `ref struct` declared as a field of a non-ref struct or class |
| E180 | `ref struct` used as a generic type argument without `allows ref struct` constraint |

---

## 26. `stackalloc` [expr.stackalloc]

### 26.1 Grammar

```ebnf
StackallocExpr = "stackalloc" "[" TypeRef "]" "(" Expr ")"
```

### 26.2 Semantics

`stackalloc[T](n)` allocates `n` elements of type `T` on the stack and returns a `Span<T>`. The allocated memory is valid until the enclosing method returns. Permitted only when the result is assigned to a `Span<T>` local or passed to a function expecting `Span<T>` / `ReadOnlySpan<T>`.

### 26.3 Example

```prsm
func sumFirst10(): Int {
    val buffer: Span<Int> = stackalloc[Int](10)
    for i in 0..10 { buffer[i] = i }
    var total = 0
    for i in 0..10 { total += buffer[i] }
    return total
}
```

### 26.4 Lowering

```csharp
public int sumFirst10()
{
    Span<int> buffer = stackalloc int[10];
    for (int i = 0; i < 10; i++) { buffer[i] = i; }
    var total = 0;
    for (int i = 0; i < 10; i++) { total += buffer[i]; }
    return total;
}
```

### 26.5 Diagnostics

| Code | Condition |
|------|-----------|
| E181 | `stackalloc` result not assigned to `Span<T>` or `ReadOnlySpan<T>` |
| E182 | `stackalloc` size is not a constant or trivially bounded expression |

---

## 27. Span slice syntax [expr.span.slice]

### 27.1 Grammar

```ebnf
SliceExpr = Expr "[" Range "]"
```

### 27.2 Semantics

When the receiver is an array, `Span<T>`, `ReadOnlySpan<T>`, or any type with a `Slice(int, int)` and `Length` member, indexing with a range expression produces a slice. Reuses the existing range operator (`..`, `until`, `downTo`).

### 27.3 Example

```prsm
val arr = [1, 2, 3, 4, 5]
val middle = arr[1..4]      // [2, 3, 4]
val tail = arr[2..]         // [3, 4, 5]
val head = arr[..3]         // [1, 2, 3]
```

### 27.4 Lowering

```csharp
var arr = new int[] { 1, 2, 3, 4, 5 };
var middle = arr[1..4];
var tail = arr[2..];
var head = arr[..3];
```

### 27.5 Diagnostics

| Code | Condition |
|------|-----------|
| E183 | Range slicing on a type that does not support `Slice` / range indexer |

---

## 28. `partial` declarations [decl.partial]

### 28.1 Grammar

```ebnf
ComponentDecl = { Annotation } [ "partial" ] [ "singleton" ] "component" Identifier ...
ClassDecl     = { Annotation } [ "partial" ] [ ClassMod ] "class" Identifier ...
StructDecl    = { Annotation } [ "partial" ] [ "ref" ] "struct" Identifier ...
```

### 28.2 Semantics

A `partial` declaration relaxes the "one declaration per file" rule for that specific type. Multiple files may contribute to the same `partial` declaration as long as:

1. All declarations have the same identifier
2. All declarations have the `partial` modifier
3. All declarations have the same kind (component, class, struct, etc.)
4. Type parameters and where clauses match across all parts

The compiler combines all parts during lowering and emits a single C# `partial class` / `partial struct`.

`partial` is a contextual keyword — existing user identifiers named `partial` remain valid in non-modifier positions.

### 28.3 Example

`Player.prsm`:
```prsm
partial component Player : MonoBehaviour {
    serialize speed: Float = 5.0
    require rb: Rigidbody

    update { move() }
}
```

`Player.combat.prsm`:
```prsm
partial component Player {
    bind hp: Int = 100

    func takeDamage(amount: Int) {
        hp -= amount
        if hp <= 0 { die() }
    }
}
```

### 28.4 Lowering

The compiler emits a single C# file (or two files with `partial`) combining all parts:

```csharp
public partial class Player : MonoBehaviour
{
    [SerializeField] private float speed = 5.0f;
    private Rigidbody rb;

    void Awake() { rb = GetComponent<Rigidbody>(); }
    void Update() { move(); }
}

public partial class Player : INotifyPropertyChanged
{
    private int _hp = 100;
    public int hp { get => _hp; set { ... } }

    public void takeDamage(int amount)
    {
        hp -= amount;
        if (hp <= 0) { die(); }
    }
}
```

### 28.5 Diagnostics

| Code | Condition |
|------|-----------|
| E184 | Two declarations with the same name and one without `partial` |
| E185 | `partial` declarations have mismatched type parameters or constraints |
| E186 | `partial` declarations have mismatched base class or interfaces |

---

## 29. Generalized nested declarations [decl.nested]

### 29.1 Grammar

```ebnf
Member       = ... | NestedDecl
NestedDecl   = ClassDecl | StructDecl | EnumDecl | DataClassDecl | InterfaceDecl
```

### 29.2 Semantics

Language 4 only allowed nested declarations inside `sealed class` (for the discriminated union pattern). Language 5 generalizes this: any `class`, `component`, or `struct` body may contain nested type declarations. The nested type lowers to a C# nested class with the same accessibility rules as C# (default `private` if no visibility modifier is specified at the nested level).

### 29.3 Example

```prsm
component Inventory : MonoBehaviour {
    data class Slot(item: Item, count: Int)

    enum SortOrder { ByName, ByValue, ByRarity }

    var slots: List<Slot> = []
    var sortOrder: SortOrder = SortOrder.ByName

    func addItem(item: Item) {
        slots.add(Slot(item, 1))
    }
}
```

### 29.4 Lowering

```csharp
public class Inventory : MonoBehaviour
{
    [System.Serializable]
    public class Slot
    {
        public Item item;
        public int count;
        public Slot(Item item, int count) { this.item = item; this.count = count; }
    }

    public enum SortOrder { ByName, ByValue, ByRarity }

    public List<Slot> slots = new List<Slot>();
    public SortOrder sortOrder = SortOrder.ByName;

    public void addItem(Item item) { slots.Add(new Slot(item, 1)); }
}
```

### 29.5 Diagnostics

| Code | Condition |
|------|-----------|
| E187 | Nested `component` declaration (components shall be top-level) |

---

# Part VI. Sprint 6 — Tooling, low-priority syntax, and DX (30-34)

---

## 30. Discard `_` [expr.discard]

### 30.1 Grammar

```ebnf
DiscardExpr    = "_"
DiscardPattern = "_"
```

### 30.2 Semantics

`_` in an `out` argument position, in a destructuring binding, or in a `when` pattern means "this value is intentionally ignored". Reading from `_` is forbidden. Writing to `_` discards the value.

### 30.3 Example

```prsm
physics.raycast(ray, out _)

val (_, name) = getResult()

when point {
    Point(0, _) => "on x = 0"
    Point(_, 0) => "on y = 0"
    _ => "elsewhere"
}
```

### 30.4 Lowering

```csharp
Physics.Raycast(ray, out _);
var (_, name) = getResult();
point switch { Point(0, _) => "on x = 0", Point(_, 0) => "on y = 0", _ => "elsewhere" };
```

### 30.5 Diagnostics

| Code | Condition |
|------|-----------|
| E188 | Reading from a discard `_` |

---

## 31. Conditional indexer `arr?[i]` [expr.safeindex]

### 31.1 Grammar

```ebnf
SafeIndexExpr = Expr "?[" Expr "]"
```

### 31.2 Semantics

`arr?[i]` evaluates `arr`; if `arr` is `null`, the entire expression is `null`. Otherwise, it accesses `arr[i]`. Lowers directly to C# `arr?[i]`.

### 31.3 Example

```prsm
val first = inventory?.items?[0]
```

### 31.4 Lowering

```csharp
var first = inventory?.items?[0];
```

---

## 32. Throw expression [expr.throw]

### 32.1 Grammar

```ebnf
ThrowExpr = "throw" Expr
```

### 32.2 Semantics

`throw` may appear in expression position (in addition to its existing statement form). Commonly used with the elvis operator for required field validation.

### 32.3 Example

```prsm
val rb = body ?: throw IllegalStateException("Rigidbody required")

func divide(a: Int, b: Int): Int =
    if b == 0 then throw ArgumentException("divide by zero")
    else a / b
```

### 32.4 Lowering

```csharp
var rb = body ?? throw new InvalidOperationException("Rigidbody required");
public int divide(int a, int b) => b == 0 ? throw new ArgumentException("divide by zero") : a / b;
```

---

## 33. Refactoring LSP dispatch [dx.lsp.refactor]

### 33.1 Overview

Language 4 added the refactor module (Extract Method, Extract Component, Inline Variable, Rename, Convert to State Machine) and advertised the `refactor.extract` / `refactor.inline` code action kinds in LSP capabilities. The actual `textDocument/codeAction` handler routing was not implemented.

### 33.2 Algorithm

The LSP code action handler analyzes the selection range and the cursor position, determines which refactor helpers are applicable, and returns a `WorkspaceEdit` for the user to preview and apply.

The handler dispatch table:

| LSP code action | Trigger condition | refactor helper |
|-----------------|-------------------|-----------------|
| Extract Method | Selection contains 2+ statements inside a function body | `refactor::extract_method` |
| Extract Component | Selection contains member declarations | `refactor::extract_component` |
| Inline Variable | Cursor on a `val` declaration with single use | `refactor::inline_variable` |
| Rename Symbol | Cursor on any declaration site | `refactor::rename_symbol` |
| Convert to State Machine | Cursor on an `enum` + `switch` pattern | `refactor::convert_to_state_machine` |

### 33.3 Diagnostics

No new error codes. Refactoring failures return LSP error responses with descriptive messages.

---

## 34. Debugger DAP adapter [dx.dap]

### 34.1 Overview

Language 4 emitted flat `.prsm.map` source maps but did not provide a Debug Adapter Protocol (DAP) implementation to consume them. Language 5 ships a minimal DAP adapter inside the VS Code extension.

### 34.2 Adapter responsibilities

- **launch / attach** — start or attach to a Unity process
- **breakpoints** — translate `.prsm` line numbers to generated `.cs` line numbers via the flat source map
- **threads / stackTrace / scopes / variables** — proxy to the underlying Unity debugger; remap variable names through the variable name table
- **stepIn / stepOver / stepOut** — apply step filters to skip compiler-generated boilerplate

### 34.3 Implementation location

The DAP adapter lives in `vscode-prsm/src/dap/` and is registered via `vscode.debug.registerDebugAdapterDescriptorFactory` in the extension's activation function.

### 34.4 Diagnostics

The Language 4 diagnostic **W032** (source map generation failed) continues to apply.

---

# New diagnostics summary

### Errors

| Code | Feature | Condition |
|------|---------|-----------|
| E147 | yield | `yield` outside coroutine or iterator-returning function |
| E148 | yield | yield value type does not match declared element type |
| E149 | attribute target | target not supported on declaration |
| E150 | attribute target | `serialize` on non-auto-property |
| E151 | preprocessor | unterminated `#if` |
| E152 | preprocessor | `#elif` / `#else` without matching `#if` |
| E153 | ref/out | argument modifier mismatch |
| E154 | out | parameter not assigned before all returns |
| E155 | ref | parameter passed an immutable `val` |
| E156 | vararg | non-final parameter |
| E157 | vararg | multiple vararg parameters |
| E158 | default param | non-constant default value |
| E159 | default param | required parameter after default |
| E160 | named arg | positional after named |
| E161 | named arg | unknown parameter name |
| E162 | named arg | parameter provided twice |
| E163 | nameof | unresolved symbol |
| E164 | nameof | not a single identifier path |
| E165 | @burst | unsupported declaration kind |
| E166 | unlisten | no matching listen … manual |
| E167 | relational pattern | type mismatch |
| E168 | or pattern | mismatched bindings |
| E169 | positional pattern | arity mismatch |
| E170 | property pattern | unknown member |
| E171 | property pattern | non-readable member |
| E172 | with | unsupported type |
| E173 | with | non-writable field |
| E174 | unmanaged | conflicting class constraint |
| E175 | notnull | on value type parameter |
| E176 | ref local | outlives referenced storage |
| E177 | ref return | references local variable |
| E178 | val ref | used in write context |
| E179 | ref struct | as non-ref struct field |
| E180 | ref struct | as generic type arg without allows |
| E181 | stackalloc | result not Span |
| E182 | stackalloc | non-constant size |
| E183 | span slice | unsupported type |
| E184 | partial | mismatched partial modifier |
| E185 | partial | mismatched type parameters |
| E186 | partial | mismatched base/interfaces |
| E187 | nested | component nested in another type |
| E188 | discard | reading from `_` |

### Warnings

| Code | Feature | Condition |
|------|---------|-----------|
| W033 | yield | coroutine declares `Seq<T>` but never yields T |
| W034 | preprocessor | unknown symbol passed through |
| W035 | UniTask | `unitask` backend requested but package not found |
| W036 | opt.structcopy | rewritten as `ref readonly` |
| W037 | relational pattern | unreachable subsequent arm |

The Language 4 warning **W031** (`bind` never read) is now actually emitted (Section 12).

---

# Feature gates

All Language 5 features are implicitly enabled by `version = "5"`. Individual features may be selectively enabled from Language 4:

```toml
[language]
version = "4"
features = ["yield-general", "attribute-target", "preprocessor"]
```

| Flag | Description |
|------|-------------|
| `yield-general` | General `yield return` / `yield break` in coroutines |
| `attribute-target` | `@field` / `@property` / etc. attribute targets |
| `preprocessor` | `#if` / `#elif` / `#else` / `#endif` directives |
| `ref-out-params` | `ref` / `out` parameters |
| `vararg` | `vararg` (params) parameters |
| `default-params` | Default parameter values |
| `named-args` | Named call arguments |
| `nameof` | `nameof` operator |
| `burst-annotation` | `@burst` annotation |
| `unitask-detect` | UniTask auto-detection |
| `bind-push` | bind X to Y continuous push |
| `bind-unread-warn` | bind W031 implementation |
| `state-name-relax` | State machine reserved name allowance |
| `opt-linq-types` | opt.linq element type inference |
| `opt-structcopy-ref` | opt.structcopy ref readonly realization |
| `optimizer-cli` | optimizer driver auto-wire and CLI |
| `unlisten-cross` | cross-context unlisten resolution |
| `relational-pattern` | relational pattern |
| `pattern-combinator` | and / or / not pattern combinators |
| `positional-pattern` | positional pattern |
| `property-pattern` | property pattern |
| `with-expr` | with expression |
| `unmanaged-constraint` | unmanaged / notnull / default constraints |
| `ref-local` | ref local and ref return |
| `ref-struct` | ref struct declaration |
| `stackalloc` | stackalloc expression |
| `span-slice` | span / array range slicing |
| `partial` | partial class / component / struct |
| `nested-decl` | generalized nested declarations |
| `discard` | discard `_` expression / pattern |
| `safe-index` | conditional indexer `?[i]` |
| `throw-expr` | throw expression |
| `lsp-refactor-dispatch` | LSP refactor code action dispatch |
| `dap-adapter` | DAP debug adapter |

---

# Implementation order

Language 5 implementation follows the six-sprint structure introduced in `plan_docs/v5-implementation-plan.md`:

**Sprint 1 — High-impact syntax**
1. General `yield return` (Section 1)
2. Attribute target on properties (Section 2)
3. Preprocessor directives (Section 3)

**Sprint 2 — Common Unity API needs**
4. `ref` / `out` parameters (Section 4)
5. `vararg` parameters (Section 5)
6. Default parameter values (Section 6)
7. Named arguments (Section 7)
8. `nameof` operator (Section 8)
9. `@burst` annotation (Section 9)
10. UniTask auto-detection (Section 10)

**Sprint 3 — Language 4 limitation fixes**
11. `bind X to Y` continuous push (Section 11)
12. `bind` never-read warning (Section 12)
13. State machine reserved-name relaxation (Section 13)
14. `opt.linq` element type inference (Section 14)
15. `opt.structcopy` realization via `ref readonly` (Section 15) — depends on Section 24
16. Optimizer driver auto-wire and CLI flag (Section 16)
17. `unlisten` cross-context resolution (Section 17)

**Sprint 4 — Pattern matching expansion**
18. Relational pattern (Section 18)
19. Pattern combinators (Section 19)
20. Positional pattern (Section 20)
21. Property pattern (Section 21)
22. `with` expression (Section 22)

**Sprint 5 — Type system extensions**
23. `unmanaged` and other where constraints (Section 23)
24. `ref` local and `ref` return (Section 24)
25. `ref struct` (Section 25)
26. `stackalloc` (Section 26)
27. Span slice syntax (Section 27)
28. `partial` declarations (Section 28)
29. Generalized nested declarations (Section 29)

**Sprint 6 — Tooling, low-priority syntax, and DX**
30. Discard `_` (Section 30)
31. Conditional indexer `?[i]` (Section 31)
32. Throw expression (Section 32)
33. Refactoring LSP dispatch (Section 33)
34. Debugger DAP adapter (Section 34)

Sprint 3 Section 15 (opt.structcopy realization) depends on Sprint 5 Section 24 (`ref` local). Implementation order should therefore complete Section 24 before Section 15, even though Section 15 is in an earlier sprint.

---

# Release criteria

- Sprint 1–3 features implemented, tested, and golden-tested
- Sprint 4–6 features implemented and individually feature-gated for incremental rollout
- All Language 4 tests (385) continue to pass without modification
- New tests added for each Language 5 feature (parser, semantic, lowering, end-to-end)
- `docs/en/spec/lang-5.md` and `docs/ko/spec/lang-5.md` written following the documentation style guide
- `docs/en/migration-v1-to-v2.md` and Korean counterpart updated with v4 → v5 section
- `prism version` outputs **3.0.0**
- Release pipeline (`release.yml`) verifies all artifacts build cleanly
- LICENSE remains MIT (set in Language 4 release window)
