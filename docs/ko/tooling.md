---
title: 도구
parent: 시작하기 (KO)
nav_order: 4
---

# 도구

PrSM은 컴파일러, 에디터 지원, Unity 통합으로 구성된 완전한 개발 툴킷입니다.

## 구성요소

| 도구 | 설명 | 상세 |
|------|------|------|
| `prism` CLI | 컴파일러, 검사기, LSP 서버, 프로젝트 도구 | [CLI 레퍼런스](cli.md) |
| Unity 패키지 | ScriptedImporter, 인스펙터, 스택트레이스 리맵 | [Unity 통합](unity-integration.md) |
| VS Code 확장 | 구문, 진단, LSP, 탐색, 스니펫 | [VS Code 확장](vscode-extension.md) |
| 소스맵 | `.prsmmap.json` 양방향 매핑 | [Generated C# & Source Maps](generated-csharp-and-source-maps.md) |

## VS Code 확장 기능

확장은 `.prsm` 파일에서 활성화되며 다음을 제공합니다:

**언어 기능 (LSP 경유):**
- 실시간 진단 (입력 중 에러/경고 표시)
- 정의로 이동 (Ctrl+Click 또는 F12)
- 모든 참조 찾기 (Shift+F12)
- 호버 정보 (타입 정보 + 생성 C# 세부 사항)
- 심볼 이름 변경 (F2)
- 문서/워크스페이스 심볼 (Ctrl+Shift+O)
- 코드 액션 (명시적 제네릭 타입 인자, import 정리)
- 자동완성 (Unity API + 사용자 심볼 + 키워드)

**에디터 기능:**
- TextMate 구문 강조 (55개 스코프)
- 20+ 코드 스니펫 (component, lifecycle, listen, coroutine 등)
- 라이프사이클 블록 삽입 (Ctrl+Shift+L)
- PrSM 탐색기 사이드바 (파일 트리)
- 의존성 그래프 뷰 (Ctrl+Shift+G)

**탐색:**
- 생성 C#으로 이동 (Ctrl+Shift+G)
- 생성 C#에서 `.prsm` 소스로 역이동
- 스택트레이스 탐색 (Ctrl+Shift+T) — 리맵된 스택 프레임 클릭

**키바인딩:**

| 단축키 | 동작 |
|--------|------|
| Ctrl+Shift+G | 생성 C# 보기 |
| Ctrl+Shift+V | 그래프 뷰 |
| Ctrl+Shift+L | 라이프사이클 삽입 |
| Ctrl+Shift+T | 스택트레이스에서 열기 |

**설정:**

| 설정 | 기본값 | 설명 |
|------|--------|------|
| `prsm.compilerPath` | `""` (자동 감지) | `prism` 바이너리 경로 |
| `prsm.checkOnSave` | `true` | 저장 시 진단 실행 |
| `prsm.showWarnings` | `true` | 경고 수준 진단 표시 |
| `prsm.unityApiDbPath` | `""` (번들됨) | Unity API SQLite 데이터베이스 경로 |

### SOLID 분석 (PrSM 3 부터)

컴파일러는 일반적인 설계 문제에 대한 정적 분석 경고를 포함합니다:

| 경고 | 조건 |
|------|------|
| W010 | Component에 public 메서드가 8개 이상 |
| W011 | Component에 의존성 필드가 6개 이상 |
| W012 | 메서드/라이프사이클에 구문이 50개 이상 |

`.prsmproject`에서 설정 가능합니다:

```toml
[analysis]
solid_warnings = true
disabled_warnings = ["W012"]
```

### 코드 최적화기 (PrSM 3 부터)

`--optimize` 옵션을 사용하면 컴파일러가 생성 C#을 최적화합니다:
- 단일 바인딩 구조 분해 인라이닝 (불필요한 임시 변수 제거)
- 향후 버전에서 추가 최적화 규칙 예정

## 설치 방법

MSI, winget, 소스 빌드를 포함한 전체 설치 안내는 [시작하기](getting-started.md)를 참조하세요.
