---
title: Functions
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 8
---

# Functions

PrSM의 함수는 `func`로 선언되며 최상위 선언의 멤버로 존재합니다.

## 블록 본문 함수

```prsm
func jump() {
    rb.AddForce(Vector3.up * jumpForce)
}

func takeDamage(amount: Int) {
    hp -= amount
    if hp <= 0 {
        die()
    }
}
```

## 표현식 본문 함수

본문이 단일 식인 함수는 `=`을 사용합니다:

```prsm
func isDead(): Bool = hp <= 0
func label(): String = "HP: $hp"
```

## 반환 타입

반환 타입은 명시적이며 `:` 뒤에 옵니다:

```prsm
func getCurrentSpeed(): Float {
    return rb.velocity.magnitude
}
```

`Unit` 반환 타입의 함수는 주석을 생략할 수 있습니다.

## 가시성 수정자

- `public` — 다른 C# 코드에서 접근 가능 (대부분의 멤버 기본값)
- `private` — component 클래스로 스코프 제한
- `protected` — 서브클래스에서 접근 가능

```prsm
private func handleInput() {
    // ...
}

public func TakeDamage(amount: Int) {
    hp -= amount
}
```

## Override

기본 클래스 메서드를 오버라이드하는 함수에는 `override`를 사용합니다:

```prsm
override func ToString(): String = "Player[$name]"
```

## 매개변수

모든 매개변수는 명시적 타입을 가진 위치 기반입니다:

```prsm
func move(direction: Vector3, speed: Float) {
    transform.Translate(direction * speed * Time.deltaTime)
}
```

## 람다식 (PrSM 4 부터)

람다는 `{ }`로 둘러싸인 익명 함수입니다. 단일 매개변수 람다는 암묵적 `it` 식별자를 사용할 수 있습니다. 호출의 마지막 인자가 람다이면 괄호 밖으로 빼낼 수 있습니다 (후행 람다).

```prsm
val callback: (Int) => Unit = { x => log(x) }
val add: (Int, Int) => Int = { a, b => a + b }

list.filter { it > 10 }
list.where({ x => x > 10 }).select({ x => x * 2 })
```

`(A, B) => R`은 `Func<A, B, R>`로, `() => Unit`은 `Action`으로 변환됩니다. 클로저 캡처는 C# 람다와 동일한 참조 의미를 따릅니다.

## `static` 및 `const` 멤버 (PrSM 4 부터)

`static`은 인스턴스 없이 접근 가능한 멤버를 선언합니다. `const`는 초기화자가 리터럴이어야 하는 컴파일 타임 상수를 선언합니다.

```prsm
class MathHelper {
    static val PI: Float = 3.14159
    static func lerp(a: Float, b: Float, t: Float): Float = a + (b - a) * t
}

const MAX_HEALTH: Int = 100
const VERSION: String = "1.0.0"
```

`static`은 라이프사이클 블록에 허용되지 않습니다 (E106). `const` 초기화자는 리터럴이어야 합니다 (E105).

## `abstract` / `open` / `override` (PrSM 4 부터)

기본적으로 클래스와 메서드는 final입니다. 수정자가 상속을 제어합니다:

| 수정자 | 효과 |
|---|---|
| `open` | 상속 / 오버라이드 허용 |
| `abstract` | 서브클래스 구현 요구; 인스턴스화 불가 |
| `sealed` | 서브클래스를 같은 파일로 제한 (`when` 완전성 활성화) |
| `override` | 부모 `open`/`abstract` 메서드를 대체하는 메서드 표시 |

```prsm
abstract class Weapon {
    abstract func attack()
    open func reload() { }
}

class Sword : Weapon {
    override func attack() { swing() }
}
```

일치하는 부모 메서드 없는 `override`는 E114를 발생시킵니다. `abstract` 클래스의 인스턴스화는 E116을 발생시킵니다.

## `async` / `await` (PrSM 4 부터)

`async func`는 비동기 함수를 선언합니다. `await`는 awaited 태스크가 완료될 때까지 일시 중단합니다. 컴파일러는 Unity 컨텍스트에서 UniTask를 우선하며, 사용 불가 시 `Task`로 폴백합니다.

```prsm
async func loadData(url: String): String {
    val response = await Http.get(url)
    return response.body
}
```

```csharp
public async UniTask<string> loadData(string url)
{
    var response = await Http.Get(url);
    return response.body;
}
```

`Unit`을 반환하는 `async` 함수는 `UniTask`로 변환됩니다. `async func` 외부의 `await`는 E135를 발생시킵니다.

## 연산자 오버로딩 (PrSM 4 부터)

연산자 함수는 기호 연산자를 명명된 메서드에 매핑합니다. PrSM은 Kotlin 규칙을 따릅니다: `plus`, `minus`, `times`, `div`, `mod`, `compareTo`, `equals`, `unaryMinus`, `not`.

```prsm
data class Vec2i(x: Int, y: Int) {
    operator plus(other: Vec2i): Vec2i = Vec2i(x + other.x, y + other.y)
    operator minus(other: Vec2i): Vec2i = Vec2i(x - other.x, y - other.y)
}

val c = Vec2i(1, 2) + Vec2i(3, 4)  // Vec2i(4, 6)
```

`operator get`과 `operator set`은 `[]` 문법을 위한 인덱서 접근을 정의합니다.

## `ref` / `out` 매개변수 (PrSM 5 부터)

`ref` 매개변수는 메서드가 호출자의 변수를 직접 수정할 수 있게 합니다. `out` 매개변수는 callee가 반환 전에 값을 할당하도록 요구합니다. 호출 사이트는 선언 표현식 형식으로 `out val name`을 사용합니다.

```prsm
func tryParse(input: String, out value: Int): Bool {
    intrinsic { return int.TryParse(input, out value); }
}

if physics.raycast(ray, out val hit) {
    log("hit ${hit.collider.name}")
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
```

`ref`/`out` 인자가 매개변수 한정자와 일치하지 않으면 E153, `out` 매개변수가 모든 반환 경로 전에 할당되지 않으면 E154, `ref` 매개변수에 불변 `val`을 전달하면 E155가 발생합니다.

## `vararg` 매개변수 (PrSM 5 부터)

`vararg` 한정자가 붙은 매개변수는 선언된 타입의 인자를 0개 이상 받습니다. 함수의 마지막 매개변수에만 `vararg`를 사용할 수 있습니다. C#의 `params T[]`로 변환됩니다.

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

마지막이 아닌 매개변수에 `vararg`를 사용하면 E156, 단일 함수에 `vararg` 매개변수가 둘 이상이면 E157이 발생합니다.

## 매개변수 기본값 (PrSM 5 부터)

매개변수 선언에 `= expr`을 추가해 기본값을 지정할 수 있습니다. 기본값 식은 컴파일 타임 상수(literal, `null`, `default`, `const` 참조)여야 합니다. 기본값을 가진 매개변수는 모두 필수 매개변수 뒤에 와야 합니다.

```prsm
func instantiate(prefab: GameObject, parent: Transform? = null, worldSpace: Bool = false): GameObject {
    return GameObject.Instantiate(prefab, parent, worldSpace)
}

instantiate(bulletPrefab)
instantiate(bulletPrefab, weaponSocket)
instantiate(bulletPrefab, weaponSocket, true)
```

상수가 아닌 기본값은 E158, 기본값 매개변수 뒤에 필수 매개변수가 오면 E159가 발생합니다.

## 명명 인자 (PrSM 5 부터)

호출 사이트에서 매개변수 이름으로 인자를 지정할 수 있습니다. 명명 인자는 임의 순서로 나타날 수 있지만 명명 인자 뒤에 위치 인자가 와서는 안 됩니다.

```prsm
GameObject.Instantiate(
    original: bulletPrefab,
    position: spawnPoint.position,
    rotation: Quaternion.identity,
    parent: bulletContainer,
)
```

명명 인자 뒤의 위치 인자는 E160, 알려지지 않은 매개변수 이름은 E161, 같은 매개변수가 두 번 제공되면 E162가 발생합니다.

## `nameof` 연산자 (PrSM 5 부터)

`nameof(x)`는 컴파일 타임에 문자열 `"x"`로 평가됩니다. `nameof(Type.Member)`는 `"Member"`로 평가됩니다. 인자는 실제 심볼을 참조해야 합니다.

```prsm
component Player : MonoBehaviour {
    require rb: Rigidbody

    awake {
        if rb == null {
            error("Required component ${nameof(Rigidbody)} is missing")
        }
    }
}
```

`nameof`는 컨텍스트 키워드입니다. 해석되지 않는 심볼은 E163, 단일 식별자 경로로 해석되지 않는 인자는 E164를 발생시킵니다.
