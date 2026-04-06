---
title: Current State
parent: 내부 구조
grand_parent: 한국어 문서
nav_order: 2
---

# Current State

현재 저장소 기준 상태 (v2.0 완료):

핵심 언어 및 도구 체인:
- lexer, parser, semantic analysis, lowering, code generation 구현 완료
- `prism` CLI 구현 및 저장소 내부 검증 완료
- Unity package integration 구현 완료
- trusted workspace 에서 `prism lsp` 기반 completion, definition, hover, references, rename, document/workspace symbol 구현 완료
- VS Code hover 는 LSP 경로를 유지하면서 가능한 경우 generated C# 정보만 확장에서 보강
- `.prsmmap.json` 기반 generated C# 역매핑이 VS Code 확장과 Unity package 양쪽에 구현 완료

v2.0 언어 기능:
- 패턴 매칭 — enum payload 바인딩, when 가드, val/for 구조 분해
- listen 수명 모델 — until disable, until destroy, manual + 자동 정리 코드 생성
- New Input System sugar — `input.action()` 및 `input.player().action()` 멀티플레이어 형식
- 제한적 제네릭 타입 추론 — 변수 타입, 반환 타입, 인자 타입 기반
- 증분 빌드 캐시 — FNV-1a 해시 기반 무효화
- Typed HIR 계층 — 구문과 의미론 분리
- LSP 코드 액션 — 명시적 타입 인자, import 정리

검증 및 배포:
- 총 251개 테스트 (unit 204 + integration 47), 모든 v2 기능 커버
- 패턴 arity(E082), 알 수 없는 variant(E081), listen 문맥(E083) 시맨틱 검증
- unlisten이 컴포넌트 전체 메서드(라이프사이클 + 사용자 함수)에서 동작, 필드 null 설정 포함
- BlazeTest smoke coverage 및 package-level editor test 존재
- VS Code 확장 VSIX 패키징, 번들 검증, 격리 설치 스모크 자동화
- GitHub Actions 릴리스 워크플로 — 크로스 플랫폼 빌드(Windows/macOS/Linux), MSI 인스톨러, VS Code Marketplace 배포, winget 자동 제출

v2.1 계획 (`plan_docs/v2.1-design.md` 참조):

- Lua 5.5 매뉴얼 / Rust Book 밀도의 언어 레퍼런스 매뉴얼 (현재 ~635줄 → 목표 3,000줄+)
- v2 기능 문서화 (패턴 매칭, listen 수명, input system, 제네릭 추론)
- EBNF 형식 문법, 에러 코드 카탈로그, 프로젝트 설정 레퍼런스
- HIR 보강 (제네릭 치환, 식 타입)
- 소스맵 v2 sugar 세부 매핑
- VS Code C# bridge 소스맵 기반 전환
- LSP 증분 인덱싱
