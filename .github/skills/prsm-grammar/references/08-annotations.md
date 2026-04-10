# Annotations & Preprocessor

## Unity Inspector Annotations

### @header

```prsm
@header("Movement Settings")
serialize speed: Float = 5.0
```
```csharp
[Header("Movement Settings")]
[SerializeField] private float _speed = 5.0f;
```

### @range

```prsm
@range(0, 100)
serialize health: Int = 100
```
```csharp
[Range(0, 100)]
[SerializeField] private int _health = 100;
```

### @tooltip

```prsm
@tooltip("Maximum movement speed in m/s")
serialize maxSpeed: Float = 10.0
```
```csharp
[Tooltip("Maximum movement speed in m/s")]
[SerializeField] private float _maxSpeed = 10.0f;
```

### @space

```prsm
@space(20)
serialize damage: Int = 10
```
```csharp
[Space(20)]
[SerializeField] private int _damage = 10;
```

## C# Attribute Annotations

### @serializable

```prsm
@serializable
data class Stats(val hp: Int, val mp: Int)
```
```csharp
[System.Serializable]
public class Stats { ... }
```

### @deprecated

```prsm
@deprecated("Use newMethod instead")
func oldMethod() { }
```
```csharp
[System.Obsolete("Use newMethod instead")]
public void OldMethod() { }
```

### @burst (named arguments)

```prsm
@burst(compileSynchronously = true)
func heavyCompute() { }
```
```csharp
[Unity.Burst.BurstCompile(CompileSynchronously = true)]
public void HeavyCompute() { }
```

## Attribute Target Annotations

특정 대상에 어트리뷰트를 적용:

```prsm
@field(NonSerialized)
var cachedValue: Int = 0

@property(Range(0, 100))
serialize health: Int = 100
```
```csharp
[field: NonSerialized]
private int _cachedValue = 0;

[property: Range(0, 100)]
[SerializeField] private int _health = 100;
```

대상 키워드: `@field(...)`, `@property(...)`, `@param(...)`, `@return(...)`, `@type(...)`

## Preprocessor Directives

### #if / #elif / #else / #endif

```prsm
update {
    move()

    #if editor
        drawDebugGizmos()
    #endif

    #if ios && !editor
        handleHaptics()
    #elif android
        handleVibration()
    #else
        // desktop - no haptics
    #endif
}
```
```csharp
private void Update() {
    Move();

    #if UNITY_EDITOR
    DrawDebugGizmos();
    #endif

    #if UNITY_IOS && !UNITY_EDITOR
    HandleHaptics();
    #elif UNITY_ANDROID
    HandleVibration();
    #else
    // desktop - no haptics
    #endif
}
```

### PrSM Symbol -> C# Define Mapping

| PrSM | C# |
|------|-----|
| `editor` | `UNITY_EDITOR` |
| `debug` | `DEBUG` |
| `release` | `!DEBUG` |
| `ios` | `UNITY_IOS` |
| `android` | `UNITY_ANDROID` |
| `standalone` | `UNITY_STANDALONE` |
| `webgl` | `UNITY_WEBGL` |
| `il2cpp` | `ENABLE_IL2CPP` |
| `mono` | `ENABLE_MONO` |
| `unity20223` | `UNITY_2022_3_OR_NEWER` |
| `unity20231` | `UNITY_2023_1_OR_NEWER` |
| `unity6` | `UNITY_6000_0_OR_NEWER` |

조합 연산자: `&&` (AND), `||` (OR), `!` (NOT)
