---
title: Operators
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 2
---

# Operators

## 산술

| 연산자 | 설명 |
|---|---|
| `+` | 덧셈 |
| `-` | 뺄셈 |
| `*` | 곱셈 |
| `/` | 나눗셈 |
| `%` | 나머지 |

```prsm
val damage = baseDamage * multiplier
val remaining = maxHp - hp
```

## 비교

| 연산자 | 설명 |
|---|---|
| `==` | 같음 |
| `!=` | 다름 |
| `<` | 미만 |
| `>` | 초과 |
| `<=` | 이하 |
| `>=` | 이상 |

## 논리

| 연산자 | 설명 |
|---|---|
| `&&` | 논리 AND |
| `\|\|` | 논리 OR |
| `!` | 논리 NOT |

## 대입

| 연산자 | 설명 |
|---|---|
| `=` | 대입 |
| `+=` | 더하고 대입 |
| `-=` | 빼고 대입 |
| `*=` | 곱하고 대입 |
| `/=` | 나누고 대입 |
| `%=` | 나머지 대입 |
| `?:=` (PrSM 4 부터) | Null 병합 대입 — 좌변이 `null`인 경우에만 대입 |

```prsm
var _instance: GameManager? = null

func getInstance(): GameManager {
    _instance ?:= FindFirstObjectByType<GameManager>()
    return _instance!!
}
```

`_instance ?:= expr`은 `_instance ??= expr`로 변환됩니다. 좌변은 nullable 가변 변수여야 합니다. 그렇지 않으면 컴파일러가 E132 (non-nullable) 또는 E133 (`val`)을 발생시킵니다.

## Null 안전성

| 연산자 | 설명 |
|---|---|
| `?.` | 안전 멤버 접근 — null이면 단락 |
| `?:` | null 병합 (Elvis) — null일 때 대체값 |
| `!!` | non-null 단언 — null이면 예외 |

```prsm
val name = player?.name ?: "Unknown"
val rb = body!!
```

## 범위와 루프 연산자

| 연산자 | 설명 |
|---|---|
| `..` | 닫힌 범위 |
| `until` | 배타적 상한 |
| `downTo` | 내림차순 범위 |
| `step` | 범위 단계 크기 |

```prsm
for i in 0 until count { tick(i) }
for i in 10 downTo 0 step 2 { countdown(i) }
```

## 타입 검사 및 캐스팅

`is`는 값이 주어진 타입인지 테스트합니다:

```prsm
if collider is BoxCollider {
    handleBox()
}
```

`is` 검사 후 변수는 같은 스코프 내에서 검사된 타입으로 스마트 캐스트됩니다 (PrSM 4 부터).

### 캐스트 연산자 (PrSM 4 부터)

| 연산자 | 설명 |
|---|---|
| `as Type?` | 안전 캐스트 — 실패 시 `null` 반환 |
| `as! Type` | 강제 캐스트 — 실패 시 `InvalidCastException` 발생 |

```prsm
val enemy = collider as Enemy?      // Enemy 또는 null
val boss = collider as! Boss        // 불일치 시 예외
```

명백히 무관한 타입으로의 `as!`는 E109를 발생시킵니다. null 검사되지 않는 `as?` 결과는 W021을 발생시킵니다.

## `in` 멤버십 연산자 (PrSM 4 부터)

`in`은 범위, 리스트, 맵에 대한 멤버십을 테스트합니다:

```prsm
if x in 1..10 { log("In range") }
if name in ["Alice", "Bob"] { log("Known user") }
if key in lookup { log("Key exists") }
```

`Contains`도 `ContainsKey`도 가지지 않는 타입은 E129를 발생시킵니다.

## `await` (PrSM 4 부터)

`await`는 `async func` 본문 내의 prefix 연산자로, awaited 태스크가 완료될 때까지 일시 중단합니다:

```prsm
async func loadData(url: String): String {
    val response = await Http.get(url)
    return response.body
}
```

`async func` 외부에서 `await`를 사용하면 E135가 발생합니다.

## 연산자 오버로딩 (PrSM 4 부터)

사용자 정의 타입은 연산자 함수를 정의할 수 있습니다. PrSM은 Kotlin 규칙을 따릅니다:

| 연산자 이름 | 기호 |
|---|---|
| `plus` | `+` |
| `minus` | `-` |
| `times` | `*` |
| `div` | `/` |
| `mod` | `%` |
| `compareTo` | `<` `>` `<=` `>=` |
| `equals` | `==` `!=` |
| `unaryMinus` | `-` (prefix) |
| `not` | `!` |

```prsm
data class Vec2i(x: Int, y: Int) {
    operator plus(other: Vec2i): Vec2i = Vec2i(x + other.x, y + other.y)
}

val c = Vec2i(1, 2) + Vec2i(3, 4)
```

`operator equals`는 일치하는 `GetHashCode` 오버라이드를 요구합니다 (E124).

## 조건부 인덱서 `?[i]` (PrSM 5 부터)

`arr?[i]`는 `arr`을 평가합니다. `arr`이 `null`이면 전체 식이 `null`입니다. 그렇지 않으면 `arr[i]`에 접근합니다. C# `arr?[i]`로 직접 변환됩니다.

```prsm
val first = inventory?.items?[0]
```

```csharp
var first = inventory?.items?[0];
```

## Throw 표현식 (PrSM 5 부터)

`throw`는 (statement 형식 외에) expression 위치에 나타날 수 있습니다. 일반적으로 elvis 연산자와 함께 필수 필드 검증에 사용됩니다.

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

## 배열과 Span의 range 슬라이싱 (PrSM 5 부터)

receiver가 배열, `Span<T>`, `ReadOnlySpan<T>`, 또는 `Slice(int, int)`와 `Length` 멤버를 가진 타입일 때, range 식으로 인덱싱하면 슬라이스가 생성됩니다. 기존 range 연산자를 재사용합니다.

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

`Slice` 또는 range 인덱서를 지원하지 않는 타입에 대한 range 슬라이싱은 E183을 발생시킵니다.

## `stackalloc` (PrSM 5 부터)

`stackalloc[T](n)`은 스택에 `T` 요소 `n`개를 할당하고 `Span<T>`를 반환합니다. 할당된 메모리는 enclosing 메서드가 반환할 때까지 유효합니다. 결과는 `Span<T>` 로컬에 할당되거나 `Span<T>` / `ReadOnlySpan<T>`를 받는 함수에 전달되어야 합니다.

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

`stackalloc` 결과가 `Span<T>` 또는 `ReadOnlySpan<T>`에 할당되지 않으면 E181, 상수가 아닌 크기는 E182를 발생시킵니다.
