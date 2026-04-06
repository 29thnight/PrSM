---
title: PrSM 2
parent: 사양
nav_order: 4
---

# PrSM 언어 2

PrSM 2는 언어 1에 패턴 매칭, 이벤트 수명 관리, Input System 지원, 제네릭 타입 추론을 추가합니다. 언어 1의 모든 프로그램은 동일한 의미로 언어 2에서도 유효합니다.

**활성화:** `.prsmproject`에서 `language.version = "2"` 설정

## 새로운 언어 기능

### 바인딩을 사용한 패턴 매칭

`when` 분기에서 enum 페이로드 추출:

```prsm
when state {
    EnemyState.Idle => idle()
    EnemyState.Chase(target) => moveTo(target)
    EnemyState.Stunned(duration) if duration > 0.0 => wait(duration)
}
```

- 튜플 스타일 접근(`Item1`, `Item2`)을 통한 enum 페이로드 바인딩
- 매칭 후 필터링을 위한 when 가드 (`if condition`)
- enum 매개변수 개수에 대한 바인딩 인수 유효성 검사 (E082)
- 알 수 없는 변형 탐지 (E081)

### 구조 분해

`val` 및 `for`에서의 data class 분해:

```prsm
val PlayerStats(hp, speed) = getStats()
for Spawn(pos, delay) in wave.spawns { spawnAt(pos, delay) }
```

### listen 수명 모델

명시적 리스너 정리 (component 전용, 외부에서는 E083):

```prsm
listen button.onClick until disable { fire() }
listen spawner.onSpawn until destroy { count += 1 }
val token = listen timer.finished manual { reset() }
unlisten token
```

- `until disable` — `OnDisable`에서 자동 정리
- `until destroy` — `OnDestroy`에서 자동 정리
- `manual` + `unlisten` — 필드 무효화를 통한 명시적 제어
- 기본값 (수정자 없음): 등록 전용, 언어 1과 동일

### 새로운 Input System 편의 기능

Unity 새 Input System 패키지 지원 (`input-system` 기능 필요, 없으면 E070):

```prsm
if input.action("Jump").pressed { jump() }
val look = input.player("Gameplay").action("Look").vector2
```

상태: `pressed`, `released`, `held`, `vector2`, `scalar`

### 제네릭 타입 추론

편의 메서드를 위한 제한적 컨텍스트 기반 추론:

```prsm
val rb: Rigidbody = get()           // GetComponent<Rigidbody>()
val health: Health? = child()       // GetComponentInChildren<Health>()
```

추론 출처: 변수 타입 어노테이션, 반환 타입, 인수 타입.

### 기능 게이트

`.prsmproject`로 기능 사용 가능 여부를 제어합니다:

| 기능 | 설명 |
|------|------|
| `pattern-bindings` | enum 페이로드 바인딩, 구조 분해, when 가드 |
| `input-system` | Input System 편의 기능 (Unity Input System 패키지 필요) |
| `auto-unlisten` | listen 수명 수정자 및 unlisten |

## 새로운 진단

| 코드 | 설명 |
|------|------|
| E070 | `input-system` 기능 없이 Input System 편의 기능 사용 |
| E081 | 패턴에서 알 수 없는 변형 |
| E082 | 패턴 바인딩 인수 개수 불일치 |
| E083 | component 외부의 listen 수명 수정자 |

## 툴체인 개선

- 구문과 의미를 분리하는 타입 HIR 레이어
- FNV-1a 해시 기반 무효화를 사용하는 증분 빌드 캐시
- `prism lsp` — 완성, 정의, 호버, 참조, 이름 변경, 코드 액션을 지원하는 LSP 서버
- VS Code 확장: 경량 LSP 클라이언트, 스택 트레이스 리매핑, 소스 맵 탐색
- Unity Console과 VS Code 양방향 탐색을 위한 `.prsmmap.json` 소스 맵
