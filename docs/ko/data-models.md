---
title: Data Models & Attributes
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 6
---

# Data Models & Attributes

## Data class

PrSM에는 `struct` 키워드가 없습니다. 구현된 데이터 모델 기능은 `data class`입니다.

```prsm
data class DamageInfo(
    val amount: Int,
    val crit: Bool
)
```

public 필드, 기본 생성자, `Equals`, `GetHashCode`, `ToString`을 갖는 직렬화 가능한 C# 클래스로 lowering됩니다.

### Data class 사용 예

```prsm
val hit = DamageInfo(amount: 42, crit: true)
if hit.crit {
    showCritFX()
}
```

Data class는 equality가 값 기반으로 동작합니다. 같은 필드값을 가진 두 인스턴스는 equal로 비교됩니다.

## Enum

### 단순 enum

```prsm
enum EnemyState {
    Idle,
    Chase,
    Attack
}
```

표준 C# `enum`으로 lowering됩니다.

### 파라미터가 있는 enum

enum 변형(variant)은 타입이 있는 payload를 가질 수 있습니다.

```prsm
enum AbilityResult {
    Hit(damage: Int),
    Miss,
    Reflect(damage: Int, angle: Float)
}
```

컴파일러는 payload를 갖는 각 variant마다 중첩 struct와 확장 메서드(`IsHit()`, `HitPayload()`, `IsMiss()`, `IsReflect()`, `ReflectPayload()` 등)를 자동으로 생성합니다.

```prsm
val result: AbilityResult = AbilityResult.Hit(damage: 30)
if result.IsHit() {
    applyDamage(result.HitPayload().damage)
}
```

## Attribute

커스텀 C# attribute는 `attribute` 키워드로 선언합니다.

```prsm
@targets(Method, Property)
attribute Cooldown(
    val duration: Float,
    val resetOnHit: Bool
)
```

`@targets` 데코레이터에서 파생된 `[AttributeUsage]` 메타데이터와 생성된 생성자를 포함한 C# `Attribute` 서브클래스로 lowering됩니다.

### Attribute 적용

```prsm
@Cooldown(duration: 1.5, resetOnHit: true)
func fireProjectile() {
    // ...
}
```
