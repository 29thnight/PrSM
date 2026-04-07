---
title: Pattern Matching & Control Flow
parent: Language Guide
grand_parent: English Docs
nav_order: 7
---

# Pattern Matching & Control Flow

## `if` / `else`

Conditions are written without parentheses:

```prsm
if hp <= 0 {
    die()
} else if hp < 20 {
    playLowHealthFX()
} else {
    run()
}
```

`if` is also an expression — it produces a value:

```prsm
val label = if hp <= 0 { "Dead" } else { "Alive" }
```

## `when`

`when` is PrSM's pattern matching construct. It replaces `switch` in the common case.

### Subject form

Matches branches against a value:

```prsm
when state {
    EnemyState.Idle   => idle()
    EnemyState.Chase  => chase()
    EnemyState.Attack => attack()
    else              => wait()
}
```

### Condition form

Matches the first true branch:

```prsm
when {
    hp <= 0        => die()
    hp < lowHpThreshold => playWarning()
    else           => run()
}
```

`when` is also an expression and can return values. The semantic layer checks for exhaustiveness where branch coverage can be determined (warning W003 when missing variants).

### Pattern bindings (since PrSM 2)

Enum payload bindings extract data from parameterized enum entries:

```prsm
enum EnemyState(val target: String) {
    Idle(""),
    Chase("player"),
    Stunned("player")
}

when state {
    EnemyState.Idle => idle()
    EnemyState.Chase(target) => moveTo(target)
    EnemyState.Stunned(duration) if duration > 0.0 => wait(duration)
}
```

Generated C# uses tuple-style access:

```csharp
case EnemyState.Chase _prsm_m8_5:
    var target = _prsm_m8_5.Item1;
    moveTo(target);
    break;
```

**Rules:**
- Binding count must match enum parameter count (error E082 if mismatched)
- Variant name must exist in the enum (error E081 if unknown)
- Empty bindings `EnemyState.Idle` match without extraction

### When guards (since PrSM 2)

Guards add a condition after a pattern:

```prsm
when state {
    EnemyState.Stunned(duration) if duration > 0.0 => wait(duration)
    EnemyState.Stunned(duration) => recover()
}
```

The guard expression is checked after the pattern matches. It generates an `&&` condition in the C# output.

### Destructuring in `val` (since PrSM 2)

Data class instances can be destructured into individual variables:

```prsm
data class PlayerStats(hp: Int, speed: Float)

val PlayerStats(hp, speed) = getStats()
```

Generated C#:

```csharp
var _prsm_d = getStats();
var hp = _prsm_d.hp;
var speed = _prsm_d.speed;
```

**Rules:**
- Binding count must match the data class field count (error E082)
- Binding names are used as local variable names

### Destructuring in `for` (since PrSM 2)

The same destructuring syntax works in `for` loops:

```prsm
for Spawn(pos, delay) in wave.spawns {
    spawnAt(pos, delay)
}
```

### OR patterns (since PrSM 4)

Multiple patterns separated by commas in a `when` arm match if any individual pattern matches. All arms in an OR group must bind the same variables (or none).

```prsm
when direction {
    Direction.Up, Direction.Down    => handleVertical()
    Direction.Left, Direction.Right => handleHorizontal()
}
```

Generated C#:

```csharp
switch (direction) {
    case Direction.Up:
    case Direction.Down:
        handleVertical();
        break;
    case Direction.Left:
    case Direction.Right:
        handleHorizontal();
        break;
}
```

OR pattern arms that bind different variables produce E130.

### Range patterns (since PrSM 4)

`in low..high` inside a `when` arm matches values in the inclusive range `[low, high]`. Only integral and floating-point types are supported.

```prsm
when score {
    in 90..100 => "A"
    in 80..89  => "B"
    in 70..79  => "C"
    else       => "F"
}
```

A range with `low > high` produces E131. Overlapping range patterns emit W023.

### Smart casts in `when` (since PrSM 4)

After an `is` arm matches, the subject is narrowed to the checked type within the arm body:

```prsm
when target {
    is Enemy => target.takeDamage(10)
    is Ally  => target.heal(5)
}
```

### Relational patterns (since PrSM 5)

A relational pattern matches if the subject value compares to the operand using the specified operator (`<`, `>`, `<=`, `>=`). Permitted on integral, floating-point, and `IComparable<T>` types.

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

### Pattern combinators (since PrSM 5)

`and`, `or`, and `not` form a pattern algebra with precedence `not` > `and` > `or`. The new `or` keyword unifies with the existing comma-OR pattern from Language 4.

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

`or` pattern arms binding different variables produces E168.

### Positional patterns (since PrSM 5)

A positional pattern matches if the subject is of the specified type and each subpattern matches the corresponding deconstruction output. Generalizes Language 2 enum payload binding to all destructurable types (`data class`, `struct`, tuples).

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

### Property patterns (since PrSM 5)

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

### `with` expression (since PrSM 5)

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

### Discard `_` (since PrSM 5)

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

Reading from a discard `_` produces E188.

## `try` / `catch` / `finally` (since PrSM 4)

Exceptions are first-class. The `new` keyword is omitted on `throw`. `try` may also be used as an expression when it has exactly one `catch` clause.

```prsm
try {
    val data = File.readAllText(path)
} catch (e: FileNotFoundException) {
    warn(e.message)
} catch (e: Exception) {
    error(e.message)
} finally {
    cleanup()
}

throw ArgumentException("Invalid value")

val result = try { parseInt(str) } catch (e: Exception) { -1 }
```

Generated C#:

```csharp
try
{
    var data = File.ReadAllText(path);
}
catch (FileNotFoundException e) { Debug.LogWarning(e.Message); }
catch (Exception e) { Debug.LogError(e.Message); }
finally { Cleanup(); }

throw new ArgumentException("Invalid value");
```

A `catch` clause whose type is already covered by a higher clause produces E100. `throw` of a non-Exception expression produces E101. Empty `catch` blocks emit W020.

## `use` (IDisposable) (since PrSM 4)

`use` ensures automatic disposal of `IDisposable` resources. The block form disposes at block exit; the declaration form disposes at the enclosing scope exit.

```prsm
use stream = FileStream(path, FileMode.Open) {
    val data = stream.readToEnd()
}

use val conn = DbConnection(connString)
// conn auto-disposed at scope end
```

Lowers to a C# `using` statement (block form) or `using` declaration (`use val`). Using `use` on a type that does not implement IDisposable produces E119.

## `for`

Range-based iteration:

```prsm
for i in 0 until count {
    process(i)
}

for i in count downTo 0 {
    countdown(i)
}

for i in 0 until 10 step 2 {
    evens(i)
}
```

## `while`

```prsm
while alive {
    updateState()
}
```

## `break` and `continue`

Both are supported inside loops:

```prsm
for i in 0 until items.Count {
    if items[i] == null { continue }
    if i > maxItems { break }
    process(items[i])
}
```

## `is` type check

Branch on runtime type:

```prsm
if collider is BoxCollider {
    handleBox()
}

when shape {
    is Circle => drawCircle()
    is Rect   => drawRect()
    else      => drawDefault()
}
```
