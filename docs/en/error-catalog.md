---
title: Error Catalog
parent: Language Guide
grand_parent: English Docs
nav_order: 12
---

# Error Catalog

Every diagnostic the PrSM compiler emits carries a stable code. This page lists all codes, their severity, the message text, and how to fix the underlying issue.

---

## Errors

### E000 -- I/O error during compilation

**Severity:** Error
**Message:** `Cannot read source file: {path}`
**Explanation:** The compiler could not open or read a `.prsm` source file. This typically means the file was deleted, moved, or locked by another process after the file list was resolved.
**Fix:** Verify the file exists and is not locked. Check `.prsmproject` include/exclude patterns for stale entries.

---

### E012 -- Lifecycle block in wrong context

**Severity:** Error
**Message:** `Lifecycle block '{name}' is only valid inside a component declaration`
**Explanation:** Lifecycle blocks such as `update` or `awake` can only appear inside `component` bodies. They are not valid in `asset`, `class`, or other declarations.

```prsm
// triggers E012
asset GameConfig : ScriptableObject {
    update {
        tick()
    }
}
```

**Fix:** Move the lifecycle block into a `component`, or convert the declaration to a `component` if it needs frame callbacks.

---

### E013 -- Component-only field qualifier in wrong context

**Severity:** Error
**Message:** `'{qualifier}' fields are only valid inside a component declaration`
**Explanation:** The field qualifiers `require`, `optional`, `child`, and `parent` rely on `GetComponent` lookups generated in `Awake()`. They are only meaningful in a `component`.

```prsm
// triggers E013
class Utility {
    require rb: Rigidbody
}
```

**Fix:** Use a regular `val` or `var` field instead, or change the declaration to a `component`.

---

### E014 -- Duplicate lifecycle block

**Severity:** Error
**Message:** `Duplicate lifecycle block '{name}'; only one per component is allowed`
**Explanation:** Each lifecycle block may appear at most once per component. The compiler merges the block into a single generated Unity method and cannot handle duplicates.

```prsm
component Player : MonoBehaviour {
    update { movePlayer() }
    update { rotatePlayer() }  // E014
}
```

**Fix:** Combine the logic into a single lifecycle block, or extract one part into a helper function.

---

### E020 -- Type mismatch

**Severity:** Error
**Message:** `Type mismatch: expected '{expected}', found '{found}'`
**Explanation:** An expression produced a type that does not match what the surrounding context requires.

```prsm
component Demo : MonoBehaviour {
    serialize speed: Float = "fast"  // E020: expected Float, found String
}
```

**Fix:** Change the expression to produce the expected type, or update the type annotation.

---

### E022 -- Variable without type and without initializer

**Severity:** Error
**Message:** `Variable '{name}' must have a type annotation or an initializer`
**Explanation:** PrSM requires enough information to infer every variable's type. A bare declaration with neither a type nor an initial value is ambiguous.

```prsm
func demo() {
    val x  // E022: no type, no initializer
}
```

**Fix:** Add a type annotation (`val x: Int`) or an initializer (`val x = 0`), or both.

---

### E031 -- break/continue outside loop

**Severity:** Error
**Message:** `'{keyword}' can only be used inside a loop`
**Explanation:** `break` and `continue` must appear within a `for` or `while` body.

```prsm
func demo() {
    break  // E031
}
```

**Fix:** Move the statement inside a loop, or use `return` to exit the function instead.

---

### E032 -- wait outside coroutine

**Severity:** Error
**Message:** `'wait' can only be used inside a coroutine`
**Explanation:** `wait` lowers to `yield return` and is only valid inside a `coroutine` declaration.

```prsm
func fire() {
    wait 1.0s  // E032
}
```

**Fix:** Change `func` to `coroutine`, or remove the `wait` and use a different timing strategy.

---

### E040 -- Assignment to immutable val

**Severity:** Error
**Message:** `Cannot assign to immutable value '{name}'`
**Explanation:** A `val` binding is immutable after initialization. Attempting to reassign it is an error.

```prsm
func demo() {
    val hp = 100
    hp = 50  // E040
}
```

**Fix:** Change the declaration from `val` to `var` if the value needs to change.

---

### E041 -- Assignment to require field

**Severity:** Error
**Message:** `Cannot assign to 'require' field '{name}'`
**Explanation:** `require` fields are resolved once in `Awake()` and are treated as immutable for the lifetime of the component.

```prsm
component Demo : MonoBehaviour {
    require rb: Rigidbody

    func reset() {
        rb = null  // E041
    }
}
```

**Fix:** Use `optional` instead of `require` if the reference needs to change at runtime.

---

### E050 -- Empty enum

**Severity:** Error
**Message:** `Enum '{name}' must have at least one entry`
**Explanation:** An enum with zero entries is not valid. The compiler needs at least one variant to generate the backing C# enum.

```prsm
enum Status {}  // E050
```

**Fix:** Add at least one entry to the enum body.

---

### E051 -- Enum entry argument count mismatch

**Severity:** Error
**Message:** `Enum entry '{entry}' expects {expected} argument(s), but {found} given`
**Explanation:** When constructing an enum value that carries a payload, the number of arguments must match the entry definition.

```prsm
enum Result {
    Ok(Int),
    Err(String)
}

func demo() {
    val r = Result.Ok(1, 2)  // E051: Ok expects 1, got 2
}
```

**Fix:** Pass exactly the number of arguments declared in the enum entry.

---

### E052 -- Duplicate enum entry name

**Severity:** Error
**Message:** `Duplicate enum entry '{name}'`
**Explanation:** Each entry within a single enum must have a unique name.

```prsm
enum Direction {
    Up,
    Down,
    Up  // E052
}
```

**Fix:** Rename or remove the duplicate entry.

---

### E060 -- Coroutine in non-component declaration

**Severity:** Error
**Message:** `Coroutines are only valid inside a component declaration`
**Explanation:** Coroutines lower to `StartCoroutine` calls which require a `MonoBehaviour` context. They cannot appear in `asset` or `class` bodies.

```prsm
class Utility {
    coroutine delay() {  // E060
        wait 1.0s
    }
}
```

**Fix:** Move the coroutine into a `component`, or use a regular function with a callback pattern.

---

### E070 -- Input System sugar without feature flag

**Severity:** Error
**Message:** `Input System sugar requires the 'input-system' feature flag`
**Explanation:** The shorthand input binding syntax is gated behind a feature flag that must be enabled in `.prsmproject`.

**Fix:** Add `"input-system"` to the `language.features` array in your `.prsmproject` file.

---

### E081 -- Unknown enum variant in pattern

**Severity:** Error
**Message:** `Unknown variant '{variant}' for enum '{enum}'`
**Explanation:** A `when` branch references an enum variant that does not exist in the enum definition.

```prsm
enum State { Idle, Running }

func demo(s: State) {
    when s {
        State.Idle    => idle()
        State.Flying  => fly()  // E081: Flying not in State
    }
}
```

**Fix:** Check for typos and verify the variant name matches the enum definition.

---

### E082 -- Pattern binding arity mismatch

**Severity:** Error
**Message:** `Pattern for '{variant}' expects {expected} binding(s), found {found}`
**Explanation:** Destructuring a payload enum entry must bind the same number of values as the entry declares.

```prsm
enum Result { Ok(Int), Err(String) }

func demo(r: Result) {
    when r {
        Result.Ok(val a, val b) => log(a)  // E082: Ok has 1 field, 2 bound
        Result.Err(val msg)     => log(msg)
    }
}
```

**Fix:** Match the number of bindings to the enum entry's payload count.

---

### E083 -- Listen lifetime in wrong context

**Severity:** Error
**Message:** `Listen lifetime modifier is only valid inside a component`
**Explanation:** The `.once` and `.whileEnabled` listen lifetime modifiers depend on component lifecycle hooks to manage cleanup. They cannot be used in `asset` or `class` bodies.

**Fix:** Move the `listen` statement into a `component`, or wire the event manually.

---

### E100 -- Parser / syntax error

**Severity:** Error
**Message:** `Syntax error: {details}`
**Explanation:** The parser encountered a token it did not expect. This is the catch-all for malformed source text.

```prsm
component Demo : MonoBehaviour {
    func () { }  // E100: expected identifier after 'func'
}
```

**Fix:** Check the line indicated in the diagnostic for missing identifiers, unmatched braces, or misplaced keywords.

---

## Warnings

### W001 -- Unnecessary non-null assertion

**Severity:** Warning
**Message:** `Unnecessary '!!' on non-nullable type '{type}'`
**Explanation:** Applying `!!` to a value whose type is already non-nullable has no effect.

```prsm
val x: Int = 10
val y = x!!  // W001: Int is already non-nullable
```

**Fix:** Remove the `!!` operator.

---

### W003 -- Incomplete when pattern

**Severity:** Warning
**Message:** `'when' does not cover all variants of '{enum}'; missing: {variants}`
**Explanation:** A `when` expression over an enum does not list every variant and has no `else` branch. At runtime, unmatched values will fall through silently.

```prsm
enum Dir { Up, Down, Left, Right }

func demo(d: Dir) {
    when d {
        Dir.Up   => moveUp()
        Dir.Down => moveDown()
        // W003: missing Left, Right
    }
}
```

**Fix:** Add branches for the missing variants, or add an `else` branch.

---

### W005 -- Data class with no fields

**Severity:** Warning
**Message:** `Data class '{name}' has no fields`
**Explanation:** A `data class` with an empty parameter list is technically valid but almost certainly unintentional.

```prsm
data class Empty()  // W005
```

**Fix:** Add fields to the parameter list, or remove the data class if it is unused.
