---
title: Types
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 3
---

# Types

## 기본 타입

| PrSM | C# |
|---|---|
| `Int` | `int` |
| `Float` | `float` |
| `Double` | `double` |
| `Bool` | `bool` |
| `String` | `string` |
| `Long` | `long` |
| `Byte` | `byte` |
| `Unit` | `void` |

## Unity 및 외부 타입

`MonoBehaviour`, `ScriptableObject`, `Transform`, `Rigidbody`, `Animator`, `Vector2`, `Vector3`, `Quaternion` 같은 Unity 타입은 그대로 쓰고 그대로 C#으로 내려갑니다.


## 제네릭 타입

PrSM 제네릭 타입은 .NET 대응 타입으로 lowering됩니다:

| PrSM | C# |
|---|---|
| `Array<T>` | `T[]` |
| `List<T>` | `System.Collections.Generic.List<T>` |
| `Map<K,V>` | `System.Collections.Generic.Dictionary<K,V>` |
| `Set<T>` | `System.Collections.Generic.HashSet<T>` |
| `Queue<T>` | `System.Collections.Generic.Queue<T>` |
| `Stack<T>` | `System.Collections.Generic.Stack<T>` |
| `Seq<T>` | `System.Collections.Generic.IEnumerable<T>` |

## 타입 추론

우변에서 타입이 명확할 때 로컬 변수 타입을 생략할 수 있습니다:

```prsm
val name = "Player"       // String으로 추론
val hp = 100              // Int로 추론
val speed = 5.0           // Float로 추론
var alive = true          // Bool로 추론
```

명시적 타입 표기는 항상 유효하며, 초기값이 `null`인 경우 필수입니다.

### 제네릭 타입 추론 (PrSM 2 부터)

v2는 제네릭 메서드 호출에 대한 제한적 문맥 기반 추론을 도입합니다. 자세한 내용은 [제네릭 추론](generic-inference.md)을 참조하세요.

```prsm
val rb: Rigidbody = get()        // 추론: GetComponent<Rigidbody>()
val health: Health? = child()    // 추론: GetComponentInChildren<Health>()
```

## Null 안전성 모델

PrSM은 타입 표기를 통해 컴파일 타임에 null 안전성을 강제합니다.

**Non-nullable 타입** (`Type`)은 값이 존재함을 보장합니다:

```prsm
require rb: Rigidbody     // Awake 이후 non-null 보장
val speed: Float = 5.0    // 절대 null이 아님
```

**Nullable 타입** (`Type?`)은 사용 전 가드가 필요합니다:

```prsm
optional cam: Camera?     // null일 수 있음

// 안전한 접근 패턴:
cam?.enabled = false          // 안전 호출 — null이면 무시
val depth = cam?.depth ?: 0   // elvis — 대체값
val fov = cam!!.fieldOfView   // non-null 단언 (이미 non-null이면 경고 W001)
```

## 타입 변환

PrSM은 암묵적 변환을 가지지 않습니다. 모든 타입 관계는 C#으로 그대로 전달됩니다.

### 캐스트 연산자 (PrSM 4 부터)

PrSM 4는 명시적 캐스트 연산자와 변환 메서드를 도입합니다:

| 형식 | 동작 |
|---|---|
| `expr as Type?` | 안전 캐스트 — 실패 시 `null` 반환 |
| `expr as! Type` | 강제 캐스트 — 실패 시 `InvalidCastException` 발생 |
| `expr.toInt()` `.toFloat()` `.toDouble()` `.toString()` | 명시적 숫자/문자열 변환 |

```prsm
val enemy = collider as Enemy?      // Enemy 또는 null
val boss = collider as! Boss        // 불일치 시 예외
val pixels = 42.toFloat()           // 42.0f
```

### 스마트 캐스트 (PrSM 4 부터)

`is` 검사 후 변수는 같은 스코프 내에서 검사된 타입으로 좁혀집니다:

```prsm
if collider is BoxCollider {
    log(collider.size)   // 여기서 collider는 BoxCollider 타입
}

when target {
    is Enemy => target.takeDamage(10)
    is Ally  => target.heal(5)
}
```

명백히 무관한 타입으로의 `as!` 사용 시 E109가 발생합니다. `as?` 결과가 null 검사되지 않으면 W021이 경고됩니다.

## 튜플 (PrSM 4 부터)

튜플은 여러 값을 단일 합성 타입으로 묶습니다. 위치 기반 튜플과 명명된 튜플이 모두 지원되며, 튜플은 별도 변수로 구조 분해할 수 있습니다.

```prsm
func getResult(): (Int, String) = (42, "answer")
val (num, name) = getResult()

func getStats(): (hp: Int, mp: Int) = (hp: 100, mp: 50)
val stats = getStats()
log(stats.hp)
```

튜플은 C# `ValueTuple`로 변환됩니다. 구조 분해 개수가 일치해야 합니다 (E117). 잘못된 라벨로 명명 튜플 필드를 접근하면 E118이 발생합니다.

## 타입 별칭 (PrSM 4 부터)

`typealias`는 기존 타입에 대한 컴파일 타임 별칭을 도입합니다. 별칭은 변환 단계에서 제거되며 런타임 비용이 없습니다.

```prsm
typealias Position = Vector3
typealias EnemyList = List<Enemy>

val pos: Position = vec3(1, 2, 3)
val enemies: EnemyList = []
```

별칭은 순환을 형성할 수 없으며 (E126) 내장 타입을 가릴 수 없습니다 (E127).

## `unmanaged` 와 기타 제네릭 제약 (PrSM 5 부터)

PrSM 5는 언어 3의 제네릭 제약을 `unmanaged`, `notnull`, `default`, `new()`로 확장합니다. `unmanaged` 제약은 `T`가 어떤 깊이에서도 관리 참조를 갖지 않는 값 타입이어야 함을 요구하며, Burst 호환 제네릭 메서드의 표준 제약입니다.

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

같은 매개변수에 `unmanaged`와 `class` 제약을 동시에 사용하면 E174, 값 타입 매개변수에 `notnull`을 사용하면 E175가 발생합니다.

## `ref` 로컬과 `ref` 반환 (PrSM 5 부터)

`val ref name = ref expr`은 읽기 전용 참조 로컬을 만듭니다. `var ref name = ref expr`은 가변 참조 로컬을 만듭니다. 함수는 `ref` 반환 타입을 선언해 참조를 반환할 수 있습니다.

```prsm
struct Particles(positions: NativeArray<Float3>) {
    func getPosition(index: Int): ref Float3 = ref positions[index]
}

func process(particles: Particles) {
    val ref pos = ref particles.getPosition(0)
    log("position: $pos")  // 복사 없음
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

`val ref`는 C# `ref readonly`로, `var ref`는 C# `ref`로 변환됩니다. `ref` 로컬이 참조된 저장소보다 오래 살면 E176, 로컬 변수를 참조하는 `ref` 반환은 E177, 쓰기 컨텍스트에서 사용된 `val ref`는 E178이 발생합니다.

## `Span<T>` 와 `ReadOnlySpan<T>` (PrSM 5 부터)

`Span<T>`와 `ReadOnlySpan<T>`는 내장 타입으로 인식되어 선언에서 직접 사용할 수 있습니다. `stackalloc`과 배열 range 슬라이싱의 결과 타입으로 사용됩니다.

```prsm
val buffer: Span<Int> = stackalloc[Int](10)
val arr = [1, 2, 3, 4, 5]
val middle: Span<Int> = arr[1..4]
```

`stackalloc`과 range 슬라이싱 세부 사항은 [연산자](operators.md)를 참조하세요.

## 전체 타입 매핑 참조

| PrSM | C# | 분류 |
|---|---|---|
| `Int` | `int` | 기본 타입 |
| `Float` | `float` | 기본 타입 |
| `Double` | `double` | 기본 타입 |
| `Bool` | `bool` | 기본 타입 |
| `String` | `string` | 기본 타입 |
| `Char` | `char` | 기본 타입 |
| `Long` | `long` | 기본 타입 |
| `Byte` | `byte` | 기본 타입 |
| `Unit` | `void` | 반환 타입 |
| `Array<T>` | `T[]` | 컬렉션 |
| `List<T>` | `System.Collections.Generic.List<T>` | 컬렉션 |
| `Map<K,V>` | `System.Collections.Generic.Dictionary<K,V>` | 컬렉션 |
| `Set<T>` | `System.Collections.Generic.HashSet<T>` | 컬렉션 |
| `Queue<T>` | `System.Collections.Generic.Queue<T>` | 컬렉션 |
| `Stack<T>` | `System.Collections.Generic.Stack<T>` | 컬렉션 |
| `Seq<T>` | `System.Collections.Generic.IEnumerable<T>` | 컬렉션 |
| *기타* | *변환 없이 그대로 전달* | Unity/.NET |
