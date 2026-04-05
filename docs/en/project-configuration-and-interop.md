---
title: Project Configuration & Interop
parent: Advanced
grand_parent: English Docs
nav_order: 1
---

# Project Configuration & Interop

## The `.prsmproject` file

Project-level configuration lives in a `.prsmproject` JSON file at the workspace root.

```json
{
  "name": "MyGame",
  "version": "0.1.0",
  "language": {
    "version": "1",
    "features": []
  },
  "compiler": {
    "outputDir": "Assets/Generated",
    "sourceDir": "src"
  },
  "include": ["src/**/*.prsm"],
  "exclude": ["src/tests/**"]
}
```

### Field reference

| Field | Type | Description |
|---|---|---|
| `name` | string | Display name of the project |
| `version` | string | SemVer project version |
| `language.version` | string | Target PrSM language version |
| `language.features` | array | Optional feature flags to enable |
| `compiler.outputDir` | string | Directory where generated `.cs` files are written |
| `compiler.sourceDir` | string | Root directory for source resolution |
| `include` | glob array | Source files to compile |
| `exclude` | glob array | Source files to exclude from compilation |

## Interop

Generated C# is intentionally readable and Unity-friendly. No bridging layers or wrappers are introduced.

| PrSM construct | Generated C# |
|---|---|
| `component T : MonoBehaviour` | `public class T : MonoBehaviour` |
| `asset T : ScriptableObject` | `public class T : ScriptableObject` |
| `class T` | `public class T` |
| `data class T(...)` | Serializable class with constructor and equality members |
| `coroutine f(): Unit` | `public IEnumerator f()` |
| `enum E { V(payload) }` | `enum E` + nested payload struct + extension methods |

## Calling C# APIs from PrSM

All Unity or C# APIs are accessible directly. PrSM resolves types through the Unity assembly references the project already depends on, so no extra import or bridging is required.

```prsm
val obj = GameObject.Find("Target")
obj.SetActive(false)
```

## Calling generated code from C#

Because the output is plain C#, any Unity script or editor code can reference generated classes, call component methods, or read asset fields without any special import.
