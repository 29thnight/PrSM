---
title: PrSM 1
parent: 사양
nav_order: 3
---

# PrSM 언어 1

PrSM 1은 PrSM 언어의 최초 릴리스입니다. 핵심 문법, 타입 시스템, Unity 통합 모델을 확립했습니다.

## 언어 기능

### 선언

- [component](../declarations-and-fields.md) — 라이프사이클 블록, 필드 한정자, 직렬화를 갖춘 MonoBehaviour 서브클래스
- [asset](../declarations-and-fields.md) — `[CreateAssetMenu]` 자동 생성을 포함한 ScriptableObject 서브클래스
- [class](../declarations-and-fields.md) — 선택적 단일 상속을 지원하는 일반 C# 클래스
- [data class](../declarations-and-fields.md) — `Equals`, `GetHashCode`, `ToString`이 자동 생성되는 값 형식 클래스
- [enum](../declarations-and-fields.md) — 페이로드 접근자 확장 메서드를 갖춘 단순 및 매개변수화 enum
- [attribute](../declarations-and-fields.md) — 사용자 정의 C# 어트리뷰트 선언

### 타입 시스템

- 기본 타입: `Int`, `Float`, `Double`, `Bool`, `String`, `Char`, `Long`, `Byte`, `Unit`
- 널 허용 타입: `Type?` — 안전 호출 `?.`, 엘비스 연산자 `?:`, 널 불허 단언 `!!`
- 제네릭 타입 참조: `List<T>`, `Map<K,V>`, `Array<T>`, `Set<T>`, `Queue<T>`, `Stack<T>`, `Seq<T>`
- Unity 및 외부 타입은 변경 없이 통과

### 필드

- `serialize`와 데코레이터: `@header`, `@tooltip`, `@range`, `@space`, `@hideInInspector`
- `val` / `var` 불변성
- `public` / `private` / `protected` 가시성
- 컴포넌트 조회: `require`, `optional`, `child`, `parent`

### 함수

- 블록 본문 또는 표현식 본문을 갖는 `func`
- `override` 수정자
- 기본 매개변수 값
- 호출 시 명명된 인수
- `intrinsic func` / `intrinsic coroutine` — 원시 C# 이스케이프 해치

### 라이프사이클 블록

- `awake`, `start`, `update`, `fixedUpdate`, `lateUpdate`
- `onEnable`, `onDisable`, `onDestroy`
- `onTriggerEnter` / `Exit` / `Stay`, `onCollisionEnter` / `Exit` / `Stay`

### 제어 흐름

- `if` / `else` — 문 형식 및 표현식 형식
- `when` — 대상 형식 및 조건 형식(`else` 분기 포함)
- `for` — 범위 기반 (`until`, `downTo`, `step`)
- `while`, `break`, `continue`, `return`

### 표현식

- 연산자 우선순위: `?:` → `||` → `&&` → `==`/`!=` → `<`/`>`/`<=`/`>=`/`is` → `..`/`until`/`downTo` → `+`/`-` → `*`/`/`/`%` → `!`/`-` → `.`/`?.`/`!!`/`[]`/`()`
- 문자열 보간: `$identifier` 및 `${expression}`
- 시간 리터럴: `1.5s`, `500ms`
- 편의 생성자: `vec2()`, `vec3()`, `color()`
- 편의 메서드: `get<T>()`, `find<T>()`, `child<T>()`, `parent<T>()`, `log()`, `warn()`, `error()`
- 입력 편의 기능: `input.axis()`, `input.getKey()`, `input.getButton()`

### 코루틴

- `coroutine` 선언 (component 전용)
- `wait` 형식: 지속 시간, `nextFrame`, `fixedFrame`, `until`, `while`
- `start` / `stop` / `stopAll`

### 이벤트

- `listen` — 이벤트 구독 (등록 전용, 자동 해제 없음)

### 진단

| 코드 | 설명 |
|------|------|
| E012 | 잘못된 컨텍스트의 라이프사이클 블록 |
| E013 | 잘못된 컨텍스트의 component 전용 필드 한정자 |
| E014 | 중복 라이프사이클 블록 |
| E020 | 타입 불일치 |
| E022 | 타입과 초기화 값이 모두 없는 변수 |
| E031 | 루프 외부의 break/continue |
| E032 | 코루틴 외부의 wait |
| E040 | 불변 val에 대한 대입 |
| E041 | require 필드에 대한 대입 |
| E050 | 비어 있는 enum |
| E051 | enum 항목 인수 개수 불일치 |
| E052 | 중복 enum 항목 |
| E060 | asset/class 내의 코루틴 |
| W001 | 불필요한 널 불허 단언 |
| W003 | 불완전한 when 패턴 |
| W005 | 필드가 없는 data class |

## 툴체인

- `prism` CLI: `compile`, `check`, `build`, `init`, `where`, `version`
- 감시 모드: `prism build --watch`
- JSON 진단: `--json` 플래그
- `.prsmproject` TOML 구성
- Unity 패키지: ScriptedImporter, 커스텀 인스펙터, 컨텍스트 메뉴
- VS Code 확장: 구문 강조, 진단, 스니펫, 사이드바
