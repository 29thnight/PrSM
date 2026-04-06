---
title: Source Maps
parent: 내부 구조
grand_parent: 한국어 문서
nav_order: 3
---

# Source Maps

PrSM 컴파일러는 생성된 모든 `.cs` 파일과 함께 **`.prsmmap.json`** 사이드카
파일을 출력합니다. 이 소스 맵은 원본 `.prsm` 소스와 생성된 C# 출력 사이의
양방향 매핑을 설정하여, 스택 트레이스 리매핑, 에디터 탐색 및 진단을
가능하게 합니다.

## 스키마 구조

소스 맵 파일은 다음과 같은 최상위 필드를 가집니다:

```json
{
  "version": 1,
  "source_file": "src/PlayerController.prsm",
  "generated_file": "Generated/PlayerController.g.cs",
  "declaration": { ... },
  "members": [ ... ]
}
```

| 필드 | 설명 |
|---|---|
| `version` | 스키마 버전 번호 (현재 `1`). |
| `source_file` | 원본 `.prsm` 소스 파일의 상대 경로. |
| `generated_file` | 생성된 `.cs` 파일의 상대 경로. |
| `declaration` | 최상위 타입 선언을 기술하는 앵커. |
| `members` | 멤버 앵커의 배열 (메서드, 필드, 프로퍼티). |

## 선언 앵커

`declaration` 객체는 클래스 또는 구조체 선언 자체를 매핑합니다:

```json
{
  "type": "class",
  "name": "PlayerController",
  "spans": {
    "prsm": { "line": 1, "col": 1, "end_line": 45, "end_col": 1 },
    "cs":   { "line": 5, "col": 1, "end_line": 82, "end_col": 1 }
  }
}
```

## Members 배열

`members` 배열의 각 항목은 타입의 멤버 하나를 기술합니다:

```json
{
  "kind": "method",
  "name": "update",
  "spans": {
    "prsm": { "line": 10, "col": 5, "end_line": 25, "end_col": 5 },
    "cs":   { "line": 20, "col": 9, "end_line": 48, "end_col": 9 }
  },
  "segments": [ ... ]
}
```

`kind`는 `method`, `field`, `property`, `event` 중 하나입니다.

## Span 형식

모든 span은 **1 기반** 행 및 열 번호를 사용합니다:

```json
{ "line": 10, "col": 5, "end_line": 25, "end_col": 5 }
```

| 필드 | 설명 |
|---|---|
| `line` | 시작 행 (1 기반). |
| `col` | 시작 열 (1 기반). |
| `end_line` | 종료 행 (1 기반, 포함). |
| `end_col` | 종료 열 (1 기반, 미포함). |

## 세그먼트 중첩

세그먼트는 멤버 본문 내의 세밀한 매핑을 제공합니다. `if`, `for`, `while`과
같은 블록 문을 표현하기 위해 중첩될 수 있습니다:

```
declaration
  +-- member (method "update")
        +-- segment (if-block, line 12-18)
        |     +-- segment (nested for-loop, line 14-16)
        +-- segment (return statement, line 20)
```

이 앵커 계층 구조를 통해 도구들은 깊이 중첩된 제어 흐름 내부에서도
모든 C# 행을 정확한 `.prsm` 원본으로 역추적할 수 있습니다.

## 컴파일러의 소스 맵 생성 방식

컴파일러의 `source_map.rs` 모듈은 C# 출력을 내보내면서 점진적으로 맵을
구성합니다. 코드 생성기가 선언, 멤버 또는 구문을 작성할 때마다 현재
PrSM span과 해당하는 C# 출력 span을 기록합니다. 최종 JSON은 `.cs` 파일과
함께 원자적으로 작성됩니다.

소스 맵 생성은 기본적으로 활성화되어 있습니다. 사이드카 파일이 필요하지 않은
릴리스 빌드에서 유용한 `--no-source-maps` 컴파일러 플래그로 비활성화할 수
있습니다.

## Unity 패키지 통합

PrSM Unity 패키지에는 C# 파일 경로와 행 번호를 받아 해당하는 PrSM 파일
경로와 행을 반환하는 `PrismSourceMap.TryResolveSourceLocation()`이
포함되어 있습니다. Unity 런타임은 예외 스택 트레이스를 포맷할 때 이 메서드를
호출하여, 생성된 C# 위치를 `.prsm` 원본으로 교체함으로써 콘솔 출력이
소스 코드를 직접 가리키도록 합니다.

## VS Code 확장 통합

VS Code 확장은 두 가지 기능에 소스 맵을 사용합니다:

1. **양방향 탐색** -- `.prsm` 파일을 열면 확장이 해당하는 생성된 `.cs`
   위치로 이동할 수 있으며, 그 반대도 가능합니다.
2. **스택 트레이스 클릭 처리** -- Unity 콘솔 출력의 클릭 가능한 파일 링크가
   올바른 행의 `.prsm` 소스를 열도록 재작성됩니다.

## 디버깅 워크플로우

소스 맵을 사용하는 일반적인 디버깅 세션은 다음과 같습니다:

1. Unity에서 런타임 예외가 발생합니다.
2. PrSM Unity 패키지가 스택 트레이스를 가로채고 각 프레임에 대해
   `PrismSourceMap.TryResolveSourceLocation()`을 호출합니다.
3. 생성된 C# 경로와 행 번호가 `.prsm` 경로로 교체됩니다.
4. 리매핑된 스택 트레이스가 Unity Console에 표시됩니다.
5. 스택 프레임을 클릭하면 실패한 생성 코드를 만든 정확한 `.prsm` 행에서
   VS Code가 열립니다.

이 엔드투엔드 흐름 덕분에 런타임 오류를 진단할 때 생성된 C#을 직접
확인할 필요가 거의 없습니다.
