---
title: 소개
nav_order: 100
has_children: true
permalink: /ko/
---

# PrSM 한국어 문서

**PrSM**(**P**ragmatic **R**educed **S**yntax for **M**etascript)은 Unity에 최적화된 스크립팅 언어 툴킷입니다. `.prsm` 소스 파일을 깔끔하고 가독성 높은 C#으로 컴파일하여 Unity 프로젝트에서 바로 사용할 수 있습니다. 게임 로직을 간결하게 유지하면서 Unity 런타임과의 완전한 호환성을 보장합니다.

## PrSM을 사용하는 이유

- **간결한 문법** — 라이프사이클 메서드, 컴포넌트 조회, 코루틴에 전용 문법 제공으로 불필요한 보일러플레이트 제거
- **강력한 null 안전성** — `require`, `optional`, `child` 한정자로 컴파일 타임에 필드 존재 여부를 검증
- **소스 맵 기반 툴링** — `.prsmmap.json` 사이드카 파일을 통해 VS Code 및 Unity 에디터에서 진단 정보와 스택 트레이스를 원본 `.prsm` 파일로 추적
- **가독성 높은 출력** — 생성된 C# 코드는 읽기 쉽고 디버깅하기 쉽게 구조화
- **Unity 네이티브** — 런타임 오버헤드 없음; 컴파일러가 순수 C#을 생성하여 Unity가 정상적으로 빌드·실행

## 툴킷 구성

| 컴포넌트 | 설명 |
|---|---|
| `crates/refraction` | Rust 컴파일러 코어 및 `prism` CLI |
| `unity-package` | Unity 에디터 통합, 임포트 훅, 소스 맵 헬퍼 |
| `vscode-prsm` | 문법 강조, 진단, LSP 탐색, 스니펫 |
| `samples` | 회귀 검증용 샘플 `.prsm` 파일 |

