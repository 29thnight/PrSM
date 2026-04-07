---
title: Coroutines
parent: Language Guide
grand_parent: English Docs
nav_order: 10
---

# Coroutines

Coroutines are a first-class feature in PrSM. They lower to Unity's `IEnumerator`-based coroutine system and are started, stopped, and waited on through dedicated syntax.

## Declaring a coroutine

Use the `coroutine` keyword:

```prsm
coroutine spawnWave() {
    for i in 0 until waveSize {
        spawn(enemyPrefab)
        wait 0.5s
    }
}
```

## Wait forms

PrSM provides several `wait` forms that map to Unity's yield instructions:

| Syntax | Unity equivalent |
|---|---|
| `wait 1.0s` | `yield return new WaitForSeconds(1.0f)` |
| `wait nextFrame` | `yield return null` |
| `wait fixedFrame` | `yield return new WaitForFixedUpdate()` |
| `wait until condition` | `yield return new WaitUntil(() => condition)` |
| `wait while condition` | `yield return new WaitWhile(() => condition)` |

```prsm
coroutine hitInvincible() {
    invincible = true
    wait invincibleTime.s     // invincibleTime is Float
    invincible = false
}

coroutine waitForDoor() {
    wait until door.isOpen
    enterRoom()
}
```

## Control forms

| Syntax | Effect |
|---|---|
| `start coroutineName()` | Starts the coroutine (calls `StartCoroutine`) |
| `stop coroutineName()` | Stops the named coroutine |
| `stopAll()` | Stops all running coroutines on this component |

```prsm
awake {
    start spawnWave()
}

onDestroy {
    stopAll()
}
```

## Duration literals

A number followed by `.s` is a duration literal and is emitted as a float representing seconds:

```prsm
wait 2.5s
wait cooldown.s
```

## General `yield` (since PrSM 5)

In addition to the `wait` shortcuts, coroutines may use general `yield` and `yield break` statements. The same form is also valid in any `func` whose return type is `Seq<T>`, `IEnumerator`, `IEnumerator<T>`, `IEnumerable`, or `IEnumerable<T>`.

```prsm
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

`Seq<T>` lowers to `IEnumerator<T>`. `wait` statements continue to coexist with general `yield` inside the same coroutine body. `yield` outside a coroutine or iterator-returning function produces E147; a yield value whose type does not match the declared element type produces E148. A `Seq<T>` coroutine that never yields any `T` value emits W033.
