---
title: "언어 1 → 2 변경사항"
parent: 사양
nav_order: 2
---

# PrSM 언어 1 → 언어 2 변경사항

이 문서는 언어 1과 언어 2 사이의 변경사항을 요약합니다. 전체 사양은 [PrSM 언어 표준](standard.md)을 참조하세요.

## 개요

언어 2는 언어 1의 엄격한 상위 집합입니다. 모든 유효한 언어 1 프로그램은 동일한 의미론으로 언어 2에서도 유효합니다. 언어 2는 새 구문과 검증 규칙을 추가하지만 기존 코드에 대한 breaking change는 없습니다.

언어 2를 활성화하려면 `.prsmproject`에 `language.version = "2"`를 설정합니다:

```toml
[language]
version = "2"
features = ["pattern-bindings", "input-system", "auto-unlisten"]
```

## 새 기능

### 1. 패턴 매칭과 바인딩 (§9, §10)

when 분기에서 enum payload 변수를 바인딩할 수 있습니다:

```prsm
when state {
    EnemyState.Idle => idle()
    EnemyState.Chase(target) => moveTo(target)
    EnemyState.Stunned(duration) if duration > 0.0 => wait(duration)
}
```

- Enum payload 바인딩은 튜플 스타일 접근(`Item1`, `Item2`)으로 값을 추출
- when 가드(`if condition`)는 매칭 후 필터링 추가
- 바인딩 수는 enum 파라미터 수와 일치해야 함

### 2. listen 수명 모델 (§10)

listen 문에 명시적 수명 수정자 지원 (component 전용):

```prsm
listen button.onClick until disable { fire() }
listen spawner.onSpawn until destroy { count += 1 }
val token = listen timer.finished manual { reset() }
unlisten token
```

- `until disable` — `OnDisable`에서 자동 정리
- `until destroy` — `OnDestroy`에서 자동 정리
- `manual` — 명시적 `unlisten`을 위한 구독 토큰 반환
- 수정자 없음: 등록만 (언어 1과 동일)
- `unlisten`은 리스너를 제거하고 핸들러 필드를 null로 설정

### 3. 구조 분해 (§10)

val과 for 문에서 data class 구조 분해 지원:

```prsm
val PlayerStats(hp, speed) = getStats()

for Spawn(pos, delay) in wave.spawns {
    spawnAt(pos, delay)
}
```

바인딩 수는 data class 필드 수와 일치해야 합니다.

### 4. New Input System Sugar (§10)

Unity New Input System 패키지용 sugar (`input-system` 기능 필요):

```prsm
if input.action("Jump").pressed { jump() }
val look = input.player("Gameplay").action("Look").vector2
```

상태: `pressed`, `released`, `held`, `vector2`, `scalar`.

### 5. 제네릭 타입 추론 (§9)

제네릭 sugar 메서드에 대한 제한적 문맥 기반 추론:

```prsm
val rb: Rigidbody = get()        // GetComponent<Rigidbody>() 추론
val health: Health? = child()    // GetComponentInChildren<Health>() 추론
```

추론 문맥: 변수 타입 표기, 반환 타입, 인자 타입.

### 6. Feature Gate (§5)

`.prsmproject`가 기능 가용성을 제어합니다:

| 기능 | 설명 |
|------|------|
| `pattern-bindings` | Enum payload 바인딩, 구조 분해, when 가드 |
| `input-system` | Input System sugar (Unity Input System 패키지 필요) |
| `auto-unlisten` | listen 수명 수정자 및 unlisten |

## 새 진단 코드

| 코드 | 심각도 | 메시지 | 조건 |
|------|--------|--------|------|
| E081 | 에러 | Unknown variant '{v}' for enum '{e}' | when 패턴이 존재하지 않는 enum variant 참조 |
| E082 | 에러 | Pattern binds N variable(s) but '{t}' expects M | 바인딩 수가 enum payload 또는 data class 필드와 불일치 |
| E083 | 에러 | 'listen {modifier} { }' is only valid inside a component | component 외부에서 listen 수명 수정자 사용 |

## Breaking Changes

없음. 모든 언어 1 프로그램은 언어 2에서 수정 없이 컴파일됩니다.

## 마이그레이션 체크리스트

1. `.prsmproject`에 `language.version = "2"` 설정
2. 선택적으로 `language.features` 배열에 기능 추가
3. `prism build` 실행 — 이전에 검증되지 않았던 패턴에 대한 E081/E082/E083 진단 수정
4. 새 기능을 점진적으로 도입:
   - 수명이 긴 listen 문에 `until disable` 추가
   - 수동 정리 intrinsic 블록을 `unlisten`으로 교체
   - 레거시 `input.getKey()`/`input.axis()` 대신 `input.action()` 사용
