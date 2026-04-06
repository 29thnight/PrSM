---
title: Migrating from v1 to v2
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 13
---

# v1에서 v2로 마이그레이션

## 개요

v2는 완전한 옵트인 방식입니다. 기존 v1 프로젝트는 아무런 변경 없이 계속 컴파일되고 실행됩니다. `.prsmproject` 파일을 업데이트하여 원하는 속도로 v2 기능을 도입할 수 있습니다.

v2는 listen 수명 정책, `when`에서의 패턴 바인딩, 새로운 Input System 편의 문법, 개선된 제네릭 타입 추론을 도입합니다. 이 모든 기능은 버전 플래그와 선택적 기능 플래그로 제어되므로, 명시적으로 옵트인하기 전까지 아무것도 변경되지 않습니다.

## 단계별 마이그레이션

### 1단계 — `.prsmproject` 업데이트

`.prsmproject` 파일을 열고 언어 버전을 설정합니다:

```toml
[language]
version = "2.0"
```

동시에 특정 기능 플래그를 활성화할 수도 있지만 (아래 기능 플래그 표 참조), 필수는 아닙니다. 버전만 설정해도 핵심 v2 시맨틱이 활성화됩니다.

### 2단계 — listen 구문 검토

v2는 `listen`에 명시적 수명 수정자를 도입하지만, 기본 동작은 **변경하지 않습니다**. 수정자 없이 `listen`을 사용하면 v1과 동일하게 등록만 수행합니다 (자동 정리 없음).

**v2에서 새로 추가된 것:**

- `listen event until disable { }` — OnDisable에서 자동 정리
- `listen event until destroy { }` — OnDestroy에서 자동 정리
- `val token = listen event manual { }` + `unlisten token` — 명시적 제어
- 이 수정자들은 `component` 선언 내부에서만 유효합니다 (외부 사용 시 E083)

**확인할 사항:**

- v1 코드에서 intrinsic 블록으로 `RemoveListener`를 수동 호출하고 있다면, `until disable` 또는 `until destroy`로 교체하여 더 깔끔한 코드를 작성할 수 있습니다.
- 수정자 없는 `listen`은 동작이 변경되지 않습니다 — 한 번 등록하고 자동 정리 없음.

```prsm
// v1 동작 — 한 번 등록, 자동 정리 없음
listen button.onClick {
    fire()
}

// v2 동등 구문 — OnDisable에서 명시적 자동 정리
listen button.onClick until disable {
    fire()
}

// v2 — 수동 수명, 직접 제거 관리
val token = listen button.onClick manual {
    fire()
}
unlisten token
```

### 3단계 — 기능 플래그 활성화

`.prsmproject`의 `features` 배열에 원하는 기능을 추가합니다:

```toml
[language]
version = "2.0"
features = ["pattern-bindings", "input-system", "auto-unlisten"]
```

각 기능은 독립적입니다. 필요한 것만 활성화하세요. 아래 기능 플래그 참조를 확인하세요.

### 4단계 — 리빌드

모든 소스를 v2 시맨틱으로 다시 컴파일합니다:

```bash
prism build
```

새로운 진단 메시지를 수정합니다. 가장 흔한 것은 E081, E082 (패턴 바인딩 검증)와 E083 (컴포넌트 외부에서 listen 수명 사용)입니다.

## v2의 브레이킹 체인지

| 변경 사항 | v1 동작 | v2 동작 | 진단 코드 |
|---|---|---|---|
| `when`에서의 패턴 바인딩 | enum 정의에 대해 검증하지 않음 | 바인딩이 검증됨; arity 불일치 또는 누락 variant 시 에러 | E081, E082 |
| `listen until disable` / `listen manual` / `unlisten` | 사용 불가 | 새 수명 수정자, `component` 선언 내부에서만 유효 | E083 |
| `listen` 기본값 (수정자 없음) | 등록만, 정리 없음 | **변경 없음** — 여전히 등록만, 정리 없음 | — |

### listen 자동 정리

v1에서 모든 `listen` 블록은 등록 후 방치(fire-and-forget) 방식이었습니다. v2 컴포넌트에서는 컴포넌트가 비활성화될 때 리스너가 자동으로 정리됩니다. 이는 파괴된 오브젝트에서 리스너가 호출되는 일반적인 버그를 방지합니다.

리스너가 disable/enable 사이클에서 유지되어야 했다면, `listen manual`로 전환하세요:

```prsm
val token = listen manager.onScoreChanged manual { val score ->
    updateUI(score)
}

// 나중에 완료되면:
unlisten token
```

### 패턴 바인딩 검증

v2는 `when` 표현식의 패턴 바인딩이 실제 enum 정의와 일치하는지 검증합니다. 이전에 검증 없이 컴파일되던 코드에서 E081 (알 수 없는 배리언트) 또는 E082 (파라미터 개수 불일치) 에러가 발생할 수 있습니다.

```prsm
enum Result {
    Ok(value: Int),
    Err(message: String)
}

when result {
    Result.Ok(v)     => handleOk(v)       // 유효
    Result.Err(m)    => handleErr(m)      // 유효
    Result.Unknown   => { }               // E081 — 해당 배리언트 없음
}
```

### listen 수명 범위

`listen until disable`, `listen manual`, `unlisten`은 `component` 선언 내부에서만 유효합니다. `class`, `asset`, 또는 최상위 범위에서 사용하면 E083이 발생합니다.

## v2에서 사용 가능한 새로운 기능

| 기능 | 설명 | 문서 |
|---|---|---|
| 바인딩을 포함한 패턴 매칭 | `when` 분기에서 enum 배리언트 구조 분해 | [Pattern Matching & Control Flow](pattern-matching-and-control-flow.md) |
| listen 수명 모델 | 이벤트 구독을 위한 `until disable`, `manual`, `unlisten` | [Events & Intrinsic](events-and-intrinsic.md) |
| 새로운 Input System 편의 문법 | Unity Input System을 위한 `on input action { }` 구문 | [Input System](input-system.md) |
| 제네릭 타입 추론 | 사용 컨텍스트에서 제네릭 인자를 컴파일러가 추론 | [Generic Inference](generic-inference.md) |

## 기능 플래그 참조

| 플래그 | 요구 사항 | 설명 |
|---|---|---|
| `"pattern-bindings"` | v2.0 | `when` 분기에서 구조 분해 바인딩을 활성화합니다. 이 플래그 없이는 `when` 분기가 v1 구문만 사용합니다. |
| `"input-system"` | v2.0 | 새로운 Unity Input System을 위한 `on input` 편의 문법을 활성화합니다. Unity 프로젝트에 Input System 패키지가 필요합니다. |
| `"auto-unlisten"` | v2.0 | `listen`에 대한 `until disable` / `manual` / `unlisten` 수명 모델을 활성화합니다. 이 플래그 없이는 v2에서도 `listen`이 v1의 등록 전용 시맨틱을 사용합니다. |

모든 플래그를 포함한 `.prsmproject` 예제:

```toml
[project]
name = "MyGame"
unity = "2022.3"

[language]
version = "2.0"
features = ["pattern-bindings", "input-system", "auto-unlisten"]

[output]
path = "Assets/Generated"
```

## v1으로 롤백

v1으로 되돌려야 하는 경우:

1. 버전을 변경합니다:

```toml
[language]
version = "1.0"
```

2. `.prsm` 파일에서 v2 전용 구문을 제거합니다:
   - `listen ... until disable { }`를 일반 `listen ... { }`로 교체
   - `listen ... manual { }`과 `unlisten` 구문 제거
   - `when` 분기에서 패턴 바인딩 제거 (일반 매칭 사용)
   - `on input` 블록 제거 (해당하는 Unity 이벤트로 `listen` 사용)

3. 리빌드:

```bash
prism build
```

컴파일러가 남아있는 v2 구문에 대해 에러를 보고하므로, 점진적으로 수정할 수 있습니다.
