---
title: PrSM 5
parent: Specification
nav_order: 7
---

# PrSM Language 5

PrSM 5 closes the remaining Unity-relevant gaps between PrSM and C# and resolves the known limitations from Language 4. It adds **22 new syntactic features** organized into six implementation sprints, plus **12 limitation fixes** to existing Language 4 features. This release ships as **Prism v3.0.0**. All Language 4 programs continue to compile without changes.

**Activation:** `language.version = "5"` in `.prsmproject`

## Part I — High-impact syntax

### General `yield return`

PrSM coroutines now accept arbitrary `yield` and `yield break` statements in addition to the `wait` shortcuts inherited from earlier versions. The enclosing function shall be a `coroutine` declaration or a `func` whose return type is `Seq<T>`, `IEnumerator`, `IEnumerator<T>`, `IEnumerable`, or `IEnumerable<T>`.

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

`yield` outside a coroutine or iterator-returning function produces E147. A `yield` value whose type does not match the declared element type produces E148. A `Seq<T>` coroutine that never yields any `T` value emits W033.

### Attribute targets on properties

A `serialize` modifier on an auto-property now lowers to a `[field: SerializeField]` backing field, the idiomatic Unity pattern for exposing an auto-property in the Inspector while keeping the public surface a property. The general form `@field(name)`, `@property(name)`, `@param(name)`, `@return(name)`, `@type(name)` lets the user attach any C# attribute to a non-default target on any declaration.

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

`serialize` on an auto-property is sugar for `@field(serializeField)`. An attribute target that does not apply to the chosen declaration produces E149. `serialize` on a property whose accessors prevent auto-property lowering produces E150.

### Preprocessor directives

`#if` / `#elif` / `#else` / `#endif` directives now appear at any statement, member, or top-level position. PrSM defines a curated set of platform symbols that translate to the corresponding `UNITY_*` defines; any other identifier passes through verbatim.

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

The normative symbol mapping is:

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

An unterminated `#if` block produces E151. `#elif`/`#else` without a matching `#if` produces E152. An unknown symbol passes through verbatim and emits W034.

## Part II — Common Unity API needs

### `ref` / `out` parameters

`ref` parameters allow a method to modify the caller's variable in place. `out` parameters require the callee to assign before returning. Call sites use `out val name` for declaration expressions (matching the C# `out var name` form).

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

A `ref`/`out` argument that does not match its parameter modifier produces E153. An `out` parameter not assigned before all return paths produces E154. A `ref` parameter passed an immutable `val` produces E155.

### `vararg` (params) parameters

A parameter declared with `vararg` accepts zero or more arguments of the declared type. Only the **last** parameter of a function may be `vararg`. The function body sees the parameter as `Array<T>` and lowers to a C# `params T[]`.

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

```csharp
public void log(params string[] messages)
{
    foreach (var msg in messages)
    {
        Debug.Log(msg);
    }
}
```

A `vararg` modifier on a non-final parameter produces E156. More than one `vararg` parameter in a single function produces E157.

### Default parameter values

A parameter declaration may include `= expr` to provide a default value. The default expression shall be a compile-time constant (literal, `null`, `default`, or a `const` reference). All parameters with defaults shall appear after all required parameters.

```prsm
func instantiate(prefab: GameObject, parent: Transform? = null, worldSpace: Bool = false): GameObject {
    return GameObject.Instantiate(prefab, parent, worldSpace)
}

instantiate(bulletPrefab)
instantiate(bulletPrefab, weaponSocket)
instantiate(bulletPrefab, weaponSocket, true)
```

```csharp
public GameObject instantiate(GameObject prefab, Transform parent = null, bool worldSpace = false)
{
    return GameObject.Instantiate(prefab, parent, worldSpace);
}
```

A non-constant default value produces E158. A required parameter following a parameter with a default value produces E159.

### Named arguments

Call sites may specify arguments by parameter name. Named arguments may appear in any order, but no positional argument shall follow a named argument (consistent with C#).

```prsm
GameObject.Instantiate(
    original: bulletPrefab,
    position: spawnPoint.position,
    rotation: Quaternion.identity,
    parent: bulletContainer,
)
```

A positional argument after a named argument produces E160. An unknown parameter name produces E161. Providing the same parameter twice (positional + named) produces E162.

### `nameof` operator

`nameof(x)` evaluates at compile time to the string `"x"`. `nameof(Type.Member)` evaluates to `"Member"` (just the trailing identifier, matching C# behavior). The argument shall reference a real symbol.

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

```csharp
public int hp { get => _hp; set { _hp = value; OnPropertyChanged(nameof(hp)); } }
if (rb == null) { Debug.LogError($"Required component {nameof(Rigidbody)} is missing"); }
```

`nameof` is a contextual keyword. An unresolved `nameof` symbol produces E163. A `nameof` argument that does not resolve to a single identifier path produces E164.

### `@burst` annotation

The `@burst` annotation marks a function or struct for Unity Burst compilation. The compiler emits the corresponding `[BurstCompile]` attribute and runs the Burst compatibility analyzer (introduced in Language 4) on the annotated definition. Diagnostics E137–E139 and W028 from Language 4 are now triggered by the annotation rather than by the naming heuristic (`burst_*`) used previously.

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

```csharp
[BurstCompile]
public void calculateForces(NativeArray<float3> positions, NativeArray<float3> forces)
{
    for (int i = 0; i < positions.Length; i++) { forces[i] = computeGravity(positions[i]); }
}

[BurstCompile(CompileSynchronously = true)]
public struct DamageJob : IJobParallelFor
{
    public NativeArray<int> damages;
    public void execute(int index) { damages[index] = damages[index] * 2; }
}
```

A `@burst` annotation on a `component`, `asset`, `interface`, or other unsupported declaration kind produces E165.

### UniTask auto-detection

The Language 4 compiler always emitted `Cysharp.Threading.Tasks.UniTask` for `async func` lowering, which broke compilation for projects without the UniTask package. Language 5 detects `com.cysharp.unitask` in `Packages/manifest.json` or `.prsmproject` and falls back to `System.Threading.Tasks.Task` when the package is not present.

```toml
[language.async]
backend = "auto"  # "unitask" | "task" | "auto"  (default: "auto")
```

```csharp
// With UniTask:
public async UniTask<string> loadData() { ... }

// Without UniTask:
public async Task<string> loadData() { ... }
```

Requesting `backend = "unitask"` while the UniTask package is missing emits W035.

## Part III — Language 4 limitation fixes

### `bind X to Y` continuous push

Language 4 implemented `bind X to Y` with initial synchronization only. Language 5 completes the reactive contract: the setter generated for the bind property iterates the registered push targets after `OnPropertyChanged` and invokes each lambda with the new value.

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

`bind X to Y` lowers to:

```csharp
_hpPushTargets ??= new List<System.Action<int>>();
_hpPushTargets.Add(v => hpLabel.text = v.ToString());
hpLabel.text = _hp.ToString();
```

The Language 4 diagnostics E143 (bind target not writable) and E144 (type mismatch) continue to apply.

### `bind` never-read warning

The W031 diagnostic code reserved in Language 4 is now actually emitted. After semantic analysis completes for a component, the compiler scans all expressions inside the component for references to each `bind` member. A bind member with zero references (other than its own setter) emits W031 at the bind declaration site.

### State machine reserved-name relaxation

State names inside a `state machine` block now accept PrSM reserved keywords (`Start`, `Stop`, `Update`, etc.). The relaxation applies only to the state name position; reserved keywords retain their normal meaning everywhere else.

```prsm
state machine playerState {
    state Start { on begin => Update }
    state Update { on pause => Stop }
    state Stop { }
}
```

```csharp
private enum PlayerState { Start, Update, Stop }
```

### `opt.linq` element type inference

Language 4's `opt.linq` rewrite emitted rewritten LINQ chains as `for` loops with `var` element types, which fell back to `object` when the source list's static type could not be inferred. The Language 5 optimizer now propagates element type information through the IR walk and skips rewrites where element type cannot be statically determined, eliminating the boxing fallback.

```csharp
var alive = new List<Enemy>();
for (int i = 0; i < enemies.Count; i++)
{
    if (enemies[i].IsAlive) alive.Add(enemies[i]);
}
```

W027 continues to apply.

### `opt.structcopy` realization via `ref readonly`

Language 4's `opt.structcopy` pass only inserted `// opt.structcopy` comment hints because PrSM had no syntax for `ref readonly` locals. With `ref local` introduced in Sprint 5, the optimizer now performs actual `ref readonly` substitution on hot-path large struct locals.

```csharp
ref readonly Vector3 pos = ref transform.position;
```

A new diagnostic W036 is emitted at each rewrite site to make the optimization auditable.

### Optimizer driver auto-wire and CLI flag

The compiler driver gains a new compile option `optimize: bool` (default `false` in debug, `true` in release). The optimizer pass runs after lowering and before codegen.

```bash
prism build --optimize          # enable optimizer
prism build --no-optimize       # disable optimizer
prism build                     # default (debug=off, release=on)
```

```toml
[compiler]
optimize = true                 # default override
```

W026, W027, and W036 continue to apply.

### `unlisten` cross-context resolution

Language 4 emitted a placeholder comment for `unlisten` statements that appeared outside the original `listen` context (e.g., in a helper function called from a lifecycle block). Language 5 collects all `unlisten` call sites per component and post-processes each site to find the matching listen handler field, regardless of which member contains the `unlisten` statement.

A token name with no matching `listen … manual` declaration in the enclosing component produces E166.

## Part IV — Pattern matching expansion

### Relational patterns

A relational pattern matches if the subject value compares to the operand using the specified operator. Permitted on integral, floating-point, and types implementing `IComparable<T>`.

```prsm
when hp {
    > 80 => "Healthy"
    > 30 => "Hurt"
    > 0  => "Critical"
    else => "Dead"
}
```

```csharp
hp switch
{
    > 80 => "Healthy",
    > 30 => "Hurt",
    > 0  => "Critical",
    _    => "Dead",
}
```

A relational pattern operand whose type does not match the subject produces E167. A subsequent relational arm covered by an earlier arm emits W037.

### Pattern combinators

`and`, `or`, and `not` form a pattern algebra. Precedence: `not` highest, then `and`, then `or`. The new `or` keyword unifies with the existing comma-OR pattern from Language 4.

```prsm
when x {
    > 0 and < 100 => "valid range"
    is Enemy or is Boss => "hostile"
    not null => "present"
    else => "missing"
}
```

```csharp
x switch
{
    > 0 and < 100 => "valid range",
    Enemy or Boss => "hostile",
    not null => "present",
    _ => "missing",
}
```

`or` pattern arms binding different variables produces E168 (extends Language 4 E130).

### Positional patterns

A positional pattern matches if the subject is of the specified type and each subpattern matches the corresponding `Deconstruct` output. Generalizes Language 2 enum payload binding to all destructurable types (`data class`, `struct`, tuples).

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

For `data class` and `struct`, the compiler auto-generates a `Deconstruct` method during lowering. A positional pattern arity that does not match the type's deconstruction produces E169.

### Property patterns

A property pattern matches if the subject is of the (optionally specified) type and each named property's value matches the corresponding subpattern. Property names shall be public readable members.

```prsm
when target {
    Enemy { hp: > 0, level: > 10 } => "tough enemy"
    Enemy { hp: 0 } => "dead enemy"
    Player { isInvincible: true } => "untouchable"
    else => "ignore"
}
```

```csharp
target switch
{
    Enemy { hp: > 0, level: > 10 } => "tough enemy",
    Enemy { hp: 0 } => "dead enemy",
    Player { isInvincible: true } => "untouchable",
    _ => "ignore",
}
```

A property pattern referencing a non-existent member produces E170. A non-readable member produces E171.

### `with` expression

`expr with { f = v, … }` produces a copy of `expr` with the specified fields replaced. `data class` lowers to a C# `record with` expression. `struct` declarations and Unity built-in struct types use a temporary-copy form.

```prsm
val origin = transform.position
val grounded = origin with { y = 0.0 }

data class PlayerStats(hp: Int, mp: Int, level: Int)

val current = PlayerStats(100, 50, 5)
val healed = current with { hp = 100 }
val leveled = healed with { level = 6, mp = 100 }
```

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

`with` on a type that is not a `data class`, `struct`, or known Unity struct produces E172. `with` on a non-writable field produces E173.

## Part V — Type system extensions

### `unmanaged` and other where constraints

Language 5 extends the Language 3 generic constraints with `unmanaged`, `notnull`, `default`, and `new()`. The `unmanaged` constraint requires `T` to be a value type with no managed references at any depth — the standard constraint required for Burst-compatible generic methods.

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

```csharp
[BurstCompile]
public T sum<T>(NativeArray<T> arr) where T : unmanaged, INumber<T>
{
    var total = T.Zero;
    for (int i = 0; i < arr.Length; i++) { total += arr[i]; }
    return total;
}
```

`unmanaged` and `class` constraints on the same parameter produces E174. `notnull` on a value type parameter produces E175.

### `ref` local and `ref` return

`val ref name = ref expr` creates a read-only reference local. `var ref name = ref expr` creates a mutable reference local. A function may declare a `ref` return type to return a reference.

```prsm
struct Particles(positions: NativeArray<Float3>) {
    func getPosition(index: Int): ref Float3 = ref positions[index]
}

func process(particles: Particles) {
    val ref pos = ref particles.getPosition(0)
    log("position: $pos")  // no copy
}
```

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

`val ref` lowers to C# `ref readonly`; `var ref` lowers to C# `ref`. A `ref` local outliving its referenced storage produces E176. A `ref` return that references a local variable produces E177. A `val ref` used in a write context produces E178.

### `ref struct`

`ref struct` declares a stack-only value type that may contain `ref` fields. Subject to C# ref struct restrictions: cannot be a field of a non-ref struct, cannot be boxed, cannot be used as a generic type argument unless the constraint is `allows ref struct` in C# 13+.

```prsm
ref struct Slice<T>(start: Int, length: Int) {
    func get(i: Int): T = intrinsic { return _data[start + i]; }
}
```

```csharp
public ref struct Slice<T>
{
    public int start;
    public int length;
    public Slice(int start, int length) { this.start = start; this.length = length; }
    public T get(int i) { return _data[start + i]; }
}
```

A `ref struct` declared as a field of a non-ref struct or class produces E179. A `ref struct` used as a generic type argument without `allows ref struct` produces E180.

### `stackalloc`

`stackalloc[T](n)` allocates `n` elements of type `T` on the stack and returns a `Span<T>`. The allocated memory is valid until the enclosing method returns. Permitted only when the result is assigned to a `Span<T>` local or passed to a function expecting `Span<T>` / `ReadOnlySpan<T>`.

```prsm
func sumFirst10(): Int {
    val buffer: Span<Int> = stackalloc[Int](10)
    for i in 0..10 { buffer[i] = i }
    var total = 0
    for i in 0..10 { total += buffer[i] }
    return total
}
```

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

A `stackalloc` result not assigned to `Span<T>` or `ReadOnlySpan<T>` produces E181. A non-constant size produces E182.

### Span slice syntax

When the receiver is an array, `Span<T>`, `ReadOnlySpan<T>`, or any type with `Slice(int, int)` and `Length` members, indexing with a range expression produces a slice. Reuses the existing range operator (`..`, `until`, `downTo`).

```prsm
val arr = [1, 2, 3, 4, 5]
val middle = arr[1..4]      // [2, 3, 4]
val tail = arr[2..]         // [3, 4, 5]
val head = arr[..3]         // [1, 2, 3]
```

```csharp
var arr = new int[] { 1, 2, 3, 4, 5 };
var middle = arr[1..4];
var tail = arr[2..];
var head = arr[..3];
```

Range slicing on a type that does not support `Slice` or a range indexer produces E183.

### `partial` declarations

A `partial` declaration relaxes the "one declaration per file" rule for that specific type. Multiple files may contribute to the same `partial` declaration as long as all parts share the same identifier, the `partial` modifier, the same kind, and matching type parameters and where clauses.

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

`partial` is a contextual keyword. Two declarations with the same name and only one with `partial` produces E184. Mismatched type parameters across parts produces E185. Mismatched base class or interfaces produces E186.

### Generalized nested declarations

Language 4 only allowed nested declarations inside `sealed class` (for the discriminated union pattern). Language 5 generalizes this: any `class`, `component`, or `struct` body may contain nested type declarations.

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

Nested `component` declarations are forbidden (E187). Components shall be top-level.

## Part VI — Tooling, low-priority syntax, and DX

### Discard `_`

`_` in an `out` argument position, in a destructuring binding, or in a `when` pattern means "this value is intentionally ignored". Reading from `_` is forbidden.

```prsm
physics.raycast(ray, out _)

val (_, name) = getResult()

when point {
    Point(0, _) => "on x = 0"
    Point(_, 0) => "on y = 0"
    _ => "elsewhere"
}
```

```csharp
Physics.Raycast(ray, out _);
var (_, name) = getResult();
point switch { Point(0, _) => "on x = 0", Point(_, 0) => "on y = 0", _ => "elsewhere" };
```

Reading from a discard `_` produces E188.

### Conditional indexer `arr?[i]`

`arr?[i]` evaluates `arr`; if `arr` is `null`, the entire expression is `null`. Otherwise, it accesses `arr[i]`. Lowers directly to C# `arr?[i]`.

```prsm
val first = inventory?.items?[0]
```

```csharp
var first = inventory?.items?[0];
```

### Throw expression

`throw` may appear in expression position (in addition to its existing statement form). Commonly used with the elvis operator for required field validation.

```prsm
val rb = body ?: throw IllegalStateException("Rigidbody required")

func divide(a: Int, b: Int): Int =
    if b == 0 { throw ArgumentException("divide by zero") }
    else { a / b }
```

```csharp
var rb = body ?? throw new InvalidOperationException("Rigidbody required");
public int divide(int a, int b) => b == 0 ? throw new ArgumentException("divide by zero") : a / b;
```

### Refactoring LSP dispatch

Language 4 added the refactor module (Extract Method, Extract Component, Inline Variable, Rename, Convert to State Machine) and advertised the code action kinds in LSP capabilities. Language 5 wires the actual `textDocument/codeAction` handler:

| LSP code action | Trigger condition |
|-----------------|-------------------|
| Extract Method | Selection contains 2+ statements inside a function body |
| Extract Component | Selection contains member declarations |
| Inline Variable | Cursor on a `val` declaration with single use |
| Rename Symbol | Cursor on any declaration site |
| Convert to State Machine | Cursor on an `enum` + `switch` pattern |

Refactoring failures return LSP error responses with descriptive messages.

### Debugger DAP adapter

Language 4 emitted flat `.prsm.map` source maps but did not provide a Debug Adapter Protocol implementation to consume them. Language 5 ships a minimal DAP adapter inside the VS Code extension that:

- launches or attaches to a Unity process
- translates `.prsm` line breakpoints to generated `.cs` line numbers via the source map
- proxies threads, stackTrace, scopes, and variables and remaps variable names through the variable name table
- applies step filters to skip compiler-generated boilerplate

W032 continues to apply when source map generation fails.

## New diagnostics

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

The Language 4 warning W031 (`bind` never read) is now actually emitted.

## Feature gates

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

## Toolchain

- **Prism v3.0.0** — bundles the Language 5 compiler, the v5 LSP, and the v5 VS Code extension
- **22 new features + 12 limitation fixes** implemented across six compiler sprints
- All Language 4 tests continue to pass without modification
- **DAP adapter** ships with the VS Code extension for source-level debugging in Unity
- **UniTask auto-detection** removes the hard dependency on the UniTask package for `async` lowering
- **MIT license** is retained from the Language 4 release window
