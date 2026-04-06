---
title: Unity Integration
parent: 도구
grand_parent: 한국어 문서
nav_order: 2
---

# Unity Integration

`unity-package` 폴더에는 PrSM 소스 파일을 Unity 프로젝트 워크플로와 연결하는 Unity 에디터 패키지(`com.prsm.editor`)가 들어 있습니다. 런타임 오버헤드는 없으며, 컴파일러는 Unity가 그대로 빌드·실행할 수 있는 순수 C#을 생성합니다.

## 동작 원리

`.prsm` 파일이 저장되거나 임포트되면 패키지가 `prism build` 파이프라인을 호출하고, 생성된 `.cs` 파일을 설정된 출력 디렉토리에 배치합니다. Unity는 스크립트 컴파일 단계에서 이를 자동으로 감지합니다.

```
.prsm 소스 파일
        │
        ▼
  prism build
        │
        ├──► 생성된 .cs   ──► Unity 스크립트 컴파일 ──► 런타임
        └──► .prsmmap.json ──► 에디터 툴링 (진단, 탐색)
```

## 임포트 및 컴파일 워크플로

1. `MoonAssetPostprocessor`가 `OnPostprocessAllAssets`를 통해 `.prsm` 변경을 감지
2. `prism` 바이너리를 해석 — 로컬 워크스페이스 개발 빌드를 확장 번들 바이너리보다 우선 사용
3. `prism build`가 영향받은 `.cs` 파일과 `.prsmmap.json` 사이드카를 재생성
4. Unity가 변경된 스크립트를 정상적으로 재컴파일

## 진단 리매핑

생성된 `.cs` 파일의 오류는 `.prsmmap.json`을 통해 원본 `.prsm` 파일의 줄·열로 리매핑됩니다. Unity Console 메시지에는 `.prsm` 경로가 표시되며, 더블 클릭하면 올바른 소스 위치로 이동합니다.

## 런타임 스택 트레이스 리매핑

`MoonStackTraceFormatter`는 `Application.logMessageReceived`를 인터셉트하고, 생성된 C#을 가리키는 스택 프레임을 원본 `.prsm` 파일 경로와 줄 번호로 재작성합니다. Unity 스타일 `(at path:line)` 과 .NET 스타일 `in path:line` 형식을 모두 처리하며, 원본 트레이스는 그대로 보존됩니다.

## 스크립트 탐색

Unity가 생성된 `.cs` 파일을 열려고 할 때(예: Console 오류 더블 클릭), `MoonScriptProxy`와 `MoonScriptRedirector`가 해당 요청을 가로채고 `.prsmmap.json` 앵커 맵을 사용해 원본 `.prsm` 소스로 리다이렉트합니다.

## Project Settings 창 (PrSM 3 부터)

모든 `.prsmproject` 설정을 Unity 에디터 GUI에서 편집할 수 있습니다:

**Window > PrSM > Project Settings**

설정 창은 모든 구성 섹션에 대한 컨트롤을 제공합니다:

| 섹션 | 설정 |
|------|------|
| **Project** | Name, PrSM Version |
| **Language** | Version 드롭다운 (1/2/3), Feature 체크박스 (pattern-bindings, input-system, auto-unlisten, interface, generics, singleton, pool, solid-analysis, optimizer) |
| **Compiler** | Compiler Path + Browse 버튼, Output Directory + Browse 버튼 |
| **Source** | Include/Exclude glob 패턴 (콤마 구분) |
| **Build Features** | Auto Compile on Save, Generate Meta Files, PascalCase Methods |
| **Analysis** | SOLID Warnings 토글, Max Public Methods / Max Dependencies / Max Method Length 임계값 |

하단 버튼:
- **Save** — `.prsmproject`에 변경 사항을 저장하고 컴파일러 캐시를 클리어
- **Revert** — 디스크에서 다시 로드하여 미저장 변경 취소
- **Open .prsmproject** — 기본 텍스트 에디터에서 TOML 파일 열기

변경 사항은 프로젝트 루트의 `.prsmproject` 파일에 기록됩니다. 전체 TOML 형식 레퍼런스는 [프로젝트 설정](project-configuration.md)을 참조하세요.

## 템플릿

**Assets → Create → PrSM**에서 스타터 템플릿을 사용할 수 있습니다. `component`, `asset`, `class` 선언의 최소 스캐폴드를 생성합니다.
