---
title: Declarations & Fields
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 5
---

# Declarations & Fields

## 최상위 선언

`.prsm` 파일 하나에는 정확히 하나의 최상위 선언만 들어갑니다.

| 키워드 | C# 대응 | 목적 |
|---|---|---|
| `component` | `MonoBehaviour` 서브클래스 | GameObject에 부착하는 게임플레이 로직 |
| `asset` | `ScriptableObject` 서브클래스 | 데이터 컨테이너, 설정, 공유 상태 |
| `class` | 일반 C# `class` | 유틸리티, 서비스, 순수 데이터 |
| `data class` | 직렬화 가능한 값 클래스 | 생성 equality를 갖는 경량 데이터 |
| `enum` | `enum` | 명명된 상수 집합 |
| `attribute` | `Attribute` 서브클래스 | 커스텀 C# 어노테이션 |

## `component`

```prsm
using UnityEngine

component PlayerController : MonoBehaviour {
    @header("이동")
    serialize speed: Float = 5.0

    require rb: Rigidbody

    update {
        move()
    }

    func move() {
        rb.MovePosition(rb.position + transform.forward * speed * Time.fixedDeltaTime)
    }
}
```

## `asset`

```prsm
using UnityEngine

asset WeaponConfig : ScriptableObject {
    serialize damage: Int = 10
    serialize fireRate: Float = 0.2
    serialize projectilePrefab: GameObject = null
}
```

Unity 에디터에서 `ScriptableObject.CreateInstance<T>()`로 생성한 에셋은 값을 `.asset` 파일에 영구 저장합니다.

## `class`

```prsm
class DamageCalculator {
    func calculate(base: Int, multiplier: Float): Float {
        return base * multiplier
    }
}
```

`class`는 Unity 의존성 없이 일반 C# 클래스로 매핑됩니다.

## 직렬화 필드

`serialize`로 표시된 필드는 Unity Inspector에 노출됩니다. 데코레이터 어노테이션으로 표시 방식을 제어합니다.

```prsm
@header("스탯")
serialize maxHp: Int = 100

@tooltip("초당 이동 거리")
serialize speed: Float = 5.0

@range(0.0, 1.0)
serialize damageMultiplier: Float = 0.5

@space
serialize weaponSlot: GameObject = null
```

지원 데코레이터: `@header(label)`, `@tooltip(text)`, `@range(min, max)`, `@space`, `@hideInInspector`.

## `val`과 `var`

- `val` — 초기화 후 재할당 불가
- `var` — 가변 필드 또는 로컬

```prsm
val gravity: Float = 9.81      // 상수
var hp: Int = 100               // 가변
```

## 가시성 한정자

`public`, `private`, `protected`는 C#에 그대로 매핑됩니다. 대부분 컨텍스트에서 기본값은 `public`입니다.

```prsm
private var invincible: Bool = false
protected var baseSpeed: Float = 5.0
```

## 컴포넌트 룩업 필드

아래 네 가지 한정자는 `component` 선언 안에서만 유효합니다. 생성된 `Awake()` 안에서 사용자 `awake` 바디보다 **먼저** 룩업 코드를 실행합니다.

| 한정자 | 생성되는 C# | null 계약 |
|---|---|---|
| `require name: Type` | `GetComponent<Type>()` | 없으면 오류 로그 + 비null 보장 |
| `optional name: Type?` | `GetComponent<Type>()` | null 허용, nullable로 저장 |
| `child name: Type` | `GetComponentInChildren<Type>()` | 비null 보장 |
| `parent name: Type` | `GetComponentInParent<Type>()` | 비null 보장 |

```prsm
require animator: Animator
optional shield: Shield?
child muzzle: Transform
parent vehicle: Vehicle
```
