---
name: prsm-grammar
description: "PrSM 문법 레퍼런스. 'PrSM 문법', 'syntax', 'PrSM 코드 작성', 'C# 변환', 'component 작성법', 'when 문법', 'listen 사용법', 'coroutine 작성법', '필드 선언', '타입 매핑', 'PrSM 예시', 'how to write PrSM', '패턴 매칭', 'state machine', 'enum', 'data class', 'when expression' 등 PrSM 언어의 구문, 작성법, 변환 규칙에 대한 질문 시 사용."
---

# PrSM Grammar Reference

PrSM 문법에 대한 질문에 정확한 레퍼런스 기반 답변을 제공한다.

## 사용할 때

- PrSM 문법, 구문, 코드 작성법에 대한 질문
- PrSM 코드가 C#으로 어떻게 변환되는지 알고 싶을 때
- 특정 PrSM 구문의 사용 예시를 원할 때
- PrSM 코드를 작성하거나 수정할 때 문법 확인이 필요할 때

## 문법 카테고리 매핑

질문의 키워드로 해당 references 파일을 Read 도구로 로드한다.

| 카테고리 | 키워드 | 레퍼런스 |
|---------|--------|---------|
| 선언 | component, asset, class, data class, enum, interface, struct, extend, typealias, attribute, singleton, partial, abstract, sealed | [01-declarations](./references/01-declarations.md) |
| 필드/멤버 | serialize, require, optional, child, parent, val, var, pool, event, bind, command, property, nested, const | [02-fields-and-members](./references/02-fields-and-members.md) |
| 라이프사이클 | awake, start, update, fixedUpdate, lateUpdate, onEnable, onDisable, onDestroy, onTrigger, onCollision | [03-lifecycle](./references/03-lifecycle.md) |
| 함수 | func, coroutine, async, intrinsic, ref, out, vararg, override, open, static, expression body | [04-functions](./references/04-functions.md) |
| 제어 흐름 | if, when, for, while, try, catch, use, listen, unlisten, wait, yield, start, stop, return, break, continue, #if | [05-control-flow](./references/05-control-flow.md) |
| 식 | 연산자, ?., ?:, !!, as, as!, is, lambda, tuple, collection, string interpolation, vec3, print, input, get, find, nameof, with, await, stackalloc | [06-expressions](./references/06-expressions.md) |
| 타입 | Int, Float, Bool, String, nullable, generic, function type, tuple type, ref type, List, Map, Set | [07-types](./references/07-types.md) |
| 어노테이션 | @header, @range, @serializable, @deprecated, @burst, @field, @property, #if, #elif, #else, #endif, preprocessor | [08-annotations](./references/08-annotations.md) |
| 고급 | listen lifetime, until disable, state machine, command, bind, MVVM, property accessor, operator overloading, singleton, extension, destructure | [09-advanced](./references/09-advanced.md) |

## PrSM -> C# 타입 매핑 조견표

| PrSM | C# |
|------|-----|
| `Int` | `int` |
| `Float` | `float` |
| `Double` | `double` |
| `Bool` | `bool` |
| `String` | `string` |
| `Char` | `char` |
| `Long` | `long` |
| `Byte` | `byte` |
| `Unit` | `void` |
| `T?` | `T?` (nullable) |
| `List<T>` | `List<T>` |
| `Map<K, V>` | `Dictionary<K, V>` |
| `Set<T>` | `HashSet<T>` |
| `Seq<T>` | `IEnumerable<T>` |
| `(A, B)` | `(A, B)` (ValueTuple) |
| `(P) => R` | `Func<P, R>` |
| `(P) => Unit` | `Action<P>` |

## 핵심 변환 패턴 Top 15

| PrSM | C# |
|------|-----|
| `component X : MonoBehaviour {}` | `public class X : MonoBehaviour {}` |
| `serialize speed: Float = 5.0` | `[SerializeField] private float _speed = 5.0f;` + property |
| `require rb: Rigidbody` | Awake() 내 `GetComponent<Rigidbody>()` + null 검사 |
| `update { }` | `private void Update() { }` |
| `coroutine x() { wait 1.0s }` | `IEnumerator X() { yield return new WaitForSeconds(1.0f); }` |
| `start x()` | `StartCoroutine(X())` |
| `listen btn.onClick { }` | `btn.onClick.AddListener(() => { })` |
| `print(x)` | `Debug.Log(x)` |
| `vec3(1, 2, 3)` | `new Vector3(1, 2, 3)` |
| `"hello $name"` | `$"hello {name}"` |
| `x?.method()` | null check + `x.Method()` |
| `x ?: default` | `x ?? default` |
| `obj as Enemy?` | `obj as Enemy` |
| `obj as! Enemy` | `((Enemy)obj)` |
| `when x { A => 1; else => 0 }` | `x switch { A => 1, _ => 0, }` |

## 응답 절차

1. 사용자 질문에서 관련 카테고리를 파악한다.
2. 해당 카테고리의 references 파일을 Read 도구로 읽는다.
3. 필요시 추가 카테고리 references도 읽는다 (예: "listen until disable" → 05-control-flow + 09-advanced).
4. 답변 시 다음 형식을 따른다:

```
**PrSM:**
```prsm
// PrSM 예시 코드
```

**생성 C#:**
```csharp
// 대응하는 C# 코드
```

설명 + 주의사항
```

5. 사용자가 PrSM 코드 작성을 요청하면, 레퍼런스에 맞는 정확한 문법으로 코드를 작성해준다.

## 추가 참조 소스

references로 답할 수 없는 경우 컴파일러 소스를 직접 확인:

- AST 정의: `crates/refraction/src/ast/mod.rs` (모든 노드 타입)
- 코드 생성 테스트: `crates/refraction/src/codegen/tests.rs` (검증된 PrSM -> C# 변환 쌍)
- 로우어링 구현: `crates/refraction/src/lowering/lower.rs` (타입 매핑, 코드 변환 규칙)
- 통합 테스트: `crates/refraction/tests/` (end-to-end 컴파일 테스트)

## 금지 패턴

- 추측으로 문법 규칙을 만들지 않는다. 레퍼런스에 없으면 AST/codegen에서 직접 확인한다.
- 아직 구현되지 않은 기능을 완성된 것처럼 설명하지 않는다.
- C# 변환 결과를 추측하지 않는다. codegen tests에서 검증된 패턴만 제시한다.
- 사용자에게 "이건 지원 안 된다"고 섣불리 단정하지 않는다. 확실하지 않으면 codegen/tests.rs를 확인한다.
