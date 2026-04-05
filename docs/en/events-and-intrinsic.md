---
title: Events & Intrinsic
parent: Language Guide
grand_parent: English Docs
nav_order: 11
---

# Events & Intrinsic

## `listen` — event wiring

The `listen` keyword wires a code block to a Unity `UnityEvent` or `UnityAction` field. It lowers to an `AddListener(...)` call that is appended to the generated `Awake()` body after all component lookups are resolved.

### Basic usage

```prsm
listen startButton.onClick {
    SceneManager.LoadScene("Game")
}

listen slider.onValueChanged {
    updateVolume(slider.value)
}
```

### With parameters

`UnityEvent<T>` callbacks automatically receive the event payload:

```prsm
listen healthBar.onValueChanged { val newValue ->
    if newValue <= 0.0 {
        triggerDeath()
    }
}
```

### Cleanup note

PrSM does not yet emit automatic `RemoveListener` calls in `OnDestroy`. For long-lived event sources, emit cleanup manually through an `intrinsic` block inside `onDestroy`.

---

## `intrinsic` — raw C# escape hatch

When PrSM's syntax does not cover a Unity API or pattern, `intrinsic` lets you embed raw C# without leaving the component structure. `intrinsic` code is passed through verbatim and is only validated by the C# compiler — PrSM's semantic checker does not inspect it.

### Statement block intrinsic

Inserts one or more raw C# statements inline in a method body:

```prsm
update {
    intrinsic {
        var ray = Camera.main.ScreenPointToRay(Input.mousePosition);
        if (Physics.Raycast(ray, out var hit, 100f)) {
            Debug.DrawLine(ray.origin, hit.point, Color.red);
        }
    }
}
```

### Typed intrinsic expression

Treats a raw C# expression as a typed PrSM value:

```prsm
val hitPoint: Vector3 = intrinsic<Vector3> { hit.point }
```

### Intrinsic function

Declares a function whose entire body is raw C#:

```prsm
intrinsic func getMouseWorldPos(): Vector3 {
    """
    var ray = Camera.main.ScreenPointToRay(Input.mousePosition);
    Physics.Raycast(ray, out RaycastHit hit, 100f);
    return hit.point;
    """
}
```

### Intrinsic coroutine

Declares a coroutine whose body contains raw C# `yield` statements:

```prsm
intrinsic coroutine fadeOut(): IEnumerator {
    """
    float t = 1f;
    while (t > 0f) {
        t -= Time.deltaTime;
        canvasGroup.alpha = t;
        yield return null;
    }
    """
}
```

Use `intrinsic` sparingly. It is an escape valve for uncommon patterns, not a replacement for PrSM syntax.
