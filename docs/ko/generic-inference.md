---
title: Generic Inference
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 10
---

# Generic Inference

PrSM v2는 **제한적 컨텍스트 기반 제네릭 타입 추론**을 도입하여,
주변 컨텍스트에서 대상 타입을 결정할 수 있는 경우 일반적인 제네릭
헬퍼 메서드에서 명시적 타입 인수를 생략할 수 있게 해줍니다.

## 개요

타입 인수를 명시적으로 작성하는 대신:

```prsm
val rb: Rigidbody = get<Rigidbody>()
```

변수의 타입 어노테이션으로부터 컴파일러가 추론하도록 할 수 있습니다:

```prsm
val rb: Rigidbody = get()
```

컴파일러는 생략된 타입 매개변수를 확인하고, 생성된 C#에 완전히 한정된
제네릭 호출을 출력합니다.

## 지원되는 메서드

추론은 다음 내장 제네릭 헬퍼에 적용됩니다:

| PrSM 메서드 | 생성된 C# |
|---|---|
| `get<T>()` | `GetComponent<T>()` |
| `require<T>()` | `GetComponent<T>()` (null 체크 어설션 포함) |
| `find<T>()` | `FindFirstObjectByType<T>()` |
| `child<T>()` | `GetComponentInChildren<T>()` |
| `parent<T>()` | `GetComponentInParent<T>()` |

사용자 정의 제네릭 함수에는 추론이 적용되지 **않습니다**.

## 추론 컨텍스트

컴파일러는 타입 인수를 추론할 수 있는 세 가지 컨텍스트를 인식합니다.

### 1. 변수 타입 어노테이션

선언의 좌변에 명시적 타입이 있는 경우, 컴파일러는 이를 사용하여
누락된 타입 인수를 채웁니다.

```prsm
val rb: Rigidbody = get()
val col: BoxCollider = child()
```

생성된 C#:

```csharp
Rigidbody rb = GetComponent<Rigidbody>();
BoxCollider col = GetComponentInChildren<BoxCollider>();
```

### 2. 반환 타입 컨텍스트

제네릭 호출이 `return` 문의 피연산자인 경우, 컴파일러는 감싸는 함수의
반환 타입에서 타입을 추론합니다.

```prsm
func getPlayer(): Player {
    return find()
}
```

생성된 C#:

```csharp
Player GetPlayer()
{
    return FindFirstObjectByType<Player>();
}
```

### 3. 인수 타입 컨텍스트

제네릭 호출이 인수로 직접 전달되는 경우, 컴파일러는 호출된 함수의
해당 매개변수 타입에서 타입을 추론합니다.

```prsm
func setup(rb: Rigidbody) { ... }

func awake() {
    setup(get())
}
```

생성된 C#:

```csharp
void Awake()
{
    Setup(GetComponent<Rigidbody>());
}
```

## 규칙

- **단일하고 모호하지 않은** 해가 존재해야 합니다. 주변 컨텍스트가
  하나의 타입을 고유하게 결정하지 못하면, 컴파일러는 명시적 타입 인수를
  요구하며 오류를 출력합니다.
- 추론은 순수하게 **로컬**입니다: 여러 대입 단계나 중간 변수를 통해
  전파되지 않습니다.
- 변수에 타입 어노테이션이 없고 `val` 타입 추론에 의존하는 경우,
  읽어올 대상 타입이 없으므로 컴파일러는 제네릭 인수를 추론할 수 없습니다.

## 지원되지 않는 기능

PrSM의 추론은 의도적으로 범위가 제한되어 있습니다:

- **Hindley-Milner 통합** -- 컴파일러는 전체 함수 본문에 걸친 전역
  제약 조건 풀이를 수행하지 않습니다.
- **Lambda + 오버로드 추론** -- 오버로드된 후보가 있는 상태에서
  람다 인수 타입으로부터 제네릭 타입을 추론할 수 없습니다.
- **제네릭 선언 확장** -- 사용자 정의 제네릭 클래스나 메서드는 추론
  대상이 아니며, 위에 나열된 내장 헬퍼만 참여합니다.

확실하지 않을 때는 타입 인수를 명시적으로 지정하세요. 명시적 형식은
항상 허용되며 결코 모호하지 않습니다.
