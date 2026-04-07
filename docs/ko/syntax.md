---
title: Syntax
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 1
---

# Syntax

PrSM의 표면 문법은 의도적으로 작게 유지됩니다.

- 파일당 하나의 최상위 선언
- 보통 `using` 임포트로 시작
- 세미콜론 없이 줄바꿈으로 문장 종료
- 괄호 없는 중괄호 기반 제어문
- 생성된 C#이 원본 구조와 가깝게 유지됨

기본 파일 형태:

```prsm
using UnityEngine

component PlayerController : MonoBehaviour {
    update {
    }
}
```

## 연산자 우선순위

낮은 결합력에서 높은 결합력 순:

| 우선순위 | 연산자 | 결합성 | 설명 |
|:---:|---|---|---|
| 1 | `?:` | 오른쪽 | Elvis (null 병합) |
| 2 | `\|\|` | 왼쪽 | 논리 OR |
| 3 | `&&` | 왼쪽 | 논리 AND |
| 4 | `==` `!=` | 왼쪽 | 동등 비교 |
| 5 | `<` `>` `<=` `>=` `is` `as` `as!` `in` | 왼쪽 | 비교, 타입 검사, 캐스트, 멤버십 |
| 6 | `..` `until` `downTo` | — | 범위 |
| 7 | `+` `-` | 왼쪽 | 덧셈/뺄셈 |
| 8 | `*` `/` `%` | 왼쪽 | 곱셈/나눗셈/나머지 |
| 9 | `!` `-` (단항) `await` | 오른쪽 | 단항 부정/NOT, await |
| 10 | `.` `?.` `!!` `[]` `?[]` `()` | 왼쪽 | 후위 (멤버, 안전 호출, 단언, 인덱스, 안전 인덱스, 호출) |

`as`, `as!`, `in`은 PrSM 4에서 도입되었습니다. `await`는 `async`/`await`(PrSM 4 부터)에서 추가된 prefix 형식입니다. `?[]`는 안전 인덱스 형식 (PrSM 5 부터)입니다.

## 대입 연산자

| 연산자 | 설명 |
|---|---|
| `=` | 대입 |
| `+=` `-=` `*=` `/=` `%=` | 복합 대입 |
| `?:=` (PrSM 4 부터) | Null 병합 대입 — 좌변이 `null`인 경우에만 대입 |

대입은 문(statement)이며 식(expression)이 아닙니다.

## 원시 문자열 리터럴 (PrSM 4 부터)

삼중 따옴표 문자열은 이스케이프 없이 줄바꿈과 특수 문자를 보존합니다. 보간(`$var`, `${expr}`)은 원시 문자열 내부에서도 활성 상태로 유지됩니다.

```prsm
val json = """
    {
        "name": "Player",
        "level": 42
    }
    """

val query = """
    SELECT * FROM users
    WHERE name = '${userName}'
    """
```

지원되는 곳에서는 C# 11 원시 문자열 리터럴로, 이전 타깃에서는 `@"..."` 축자 문자열로 변환됩니다.

## 문자열 이스케이프 시퀀스

문자열 리터럴 내부:

| 이스케이프 | 문자 |
|---|---|
| `\n` | 줄바꿈 |
| `\t` | 탭 |
| `\r` | 캐리지 리턴 |
| `\\` | 백슬래시 |
| `\"` | 큰따옴표 |
| `\$` | 달러 기호 (보간 방지) |

## 문자열 보간

두 가지 형식:

```prsm
val greeting = "hello $name"              // 축약형
val info = "score: ${player.score + 1}"   // 식 형식
```

`${}` 형식은 중괄호 중첩을 포함한 모든 식을 지원합니다. 생성 C#은 `$"..."` 보간을 사용합니다.

## 시간 리터럴

시간 접미사가 붙은 숫자 리터럴:

```prsm
wait 1.5s     // 1.5초 → new WaitForSeconds(1.5f)
wait 500ms    // 500밀리초 → new WaitForSeconds(0.5f)
```

## 전처리 디렉티브 (PrSM 5 부터)

`#if` / `#elif` / `#else` / `#endif` 디렉티브는 모든 statement, member, top-level 위치에서 사용할 수 있습니다. PrSM은 자주 쓰이는 플랫폼 심볼 집합을 정의하여 대응하는 `UNITY_*` 정의로 변환합니다. 그 외 식별자는 그대로 통과됩니다.

```prsm
component Player : MonoBehaviour {
    update {
        move()

        #if editor
            drawDebugGizmos()
        #endif

        #if ios && !editor
            handleHaptics()
        #elif android
            handleVibration()
        #endif
    }

    #if debug
        func logState() { log("hp=$hp, pos=${transform.position}") }
    #endif
}
```

규범적 심볼 매핑:

| PrSM 심볼 | C# 정의 |
|-----------|---------|
| `editor` | `UNITY_EDITOR` |
| `debug` | `DEBUG` |
| `release` | `!DEBUG` |
| `ios` | `UNITY_IOS` |
| `android` | `UNITY_ANDROID` |
| `standalone` | `UNITY_STANDALONE` |
| `il2cpp` | `ENABLE_IL2CPP` |
| `mono` | `ENABLE_MONO` |
| `unity20223` | `UNITY_2022_3_OR_NEWER` |
| `unity20231` | `UNITY_2023_1_OR_NEWER` |
| `unity6` | `UNITY_6000_0_OR_NEWER` |

종료되지 않은 `#if` 블록은 E151, 대응하는 `#if`가 없는 `#elif` / `#else`는 E152, 알려지지 않은 심볼은 그대로 통과되며 W034가 발생합니다.

## 형식 문법

전체 EBNF 명세는 [형식 문법](grammar.md)을 참조하세요.
