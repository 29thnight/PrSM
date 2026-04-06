# PrSM 구현 로드맵

## 마일스톤 0: 기반 구축 ✅
- [x] v0 언어 스펙 작성
- [x] 전체 아키텍처 플랜 작성
- [x] Cargo 워크스페이스 스캐폴드
- [x] 문서 파일 생성
- [x] 샘플 .prsm 파일

## 마일스톤 1: 렉서 ✅
- [x] TokenKind 정의 (키워드, 연산자, 리터럴)
- [x] Lexer 구현 (logos 기반)
- [x] 문자열 보간 토큰화
- [x] 시간 접미사 (1.0s) 처리
- [x] 60개 테스트 케이스

## 마일스톤 2: 파서 ✅
- [x] AST 노드 정의
- [x] 재귀 하강 파서
- [x] 괄호 없는 제어문 파싱
- [x] wait DSL 파싱
- [x] listen 이벤트 파싱
- [x] intrinsic 블록 파싱
- [x] AST 프리티 프린터
- [x] 에러 복구

## 마일스톤 3: 의미 분석 ✅
- [x] 심볼 테이블
- [x] 이름 해결
- [x] 타입 체크
- [x] Null safety 분석
- [x] 선언 검증
- [x] when 완전성 검사
- [x] 확정 할당 분석

## 마일스톤 4: 로우어링 ✅
- [x] C# IR 노드 정의
- [x] AST → C# IR 변환
- [x] Awake 조립
- [x] 코루틴 lowering
- [x] sugar 매핑 (vec3, input, listen)
- [x] intrinsic verbatim 출력

## 마일스톤 5: 코드 생성 ✅
- [x] C# IR → 소스 텍스트
- [x] 포맷팅/들여쓰기
- [x] #line 디렉티브
- [x] 골든 파일 테스트
- [x] 생성된 C# 컴파일 검증

## 마일스톤 6: CLI ✅
- [x] `prism compile <file>`
- [x] `prism check <file>`
- [x] 배치 컴파일 (`prism build`)
- [x] 진단 메시지 포맷 (텍스트/JSON)
- [x] `prism init`, `prism where`, `prism version`
- [x] Watch 모드 (`prism build --watch`)

## 마일스톤 7: Unity 에디터 패키지 ✅
- [x] ScriptedImporter (`.prsm` 파일 자동 임포트)
- [x] PrismCompilerBridge (`prism where`로 바이너리 탐색)
- [x] AssetPostprocessor (파일 변경/삭제/이름변경 감지)
- [x] 커스텀 인스펙터 (PrismComponentEditor, PrismScriptInspector)
- [x] 템플릿 (component, asset, class)
- [x] 컨텍스트 메뉴 (Compile/Check/Build)
- [x] 생성 코드 패키지 (`com.prsm.generated`) 구조
- [x] Unity Console 진단 매핑 (클릭→소스 이동)

## 마일스톤 8: VSCode 확장 (v1.1) ✅
- [x] TextMate 구문 강조 (59개 스코프)
- [x] 실시간 진단 (`prism check --json` 연동)
- [x] 코드 스니펫 20개
- [x] 사이드바 탐색기
- [x] 그래프 뷰 (컴포넌트 관계 시각화)
- [x] 라이프사이클 삽입 (Ctrl+Shift+L)
- [x] Unity API 자동완성 (SQLite DB)
- [x] C# DevKit 연동
- [x] trusted workspace `prism lsp` 경로 (completion, definition, hover, references, rename, document/workspace symbols)
- [x] VSIX 패키징 완료

## 마일스톤 9: 통합 테스트 ✅
- [x] BlazeTest Unity 프로젝트 구성 (Unity 6.0.4)
- [x] `.prsmproject` 기반 프로젝트 빌드
- [x] TestScript.prsm — serialize, when, 함수, 로깅
- [x] DirTestScript.prsm — 컴포넌트 상속
- [x] 생성 C# → Unity 컴파일 성공
- [x] `unity-package/Tests/Editor` 패키지 내부 EditMode 테스트 기반 추가
- [x] 네거티브 테스트 케이스 확충 (E081/E082/E083 + 네거티브 픽스처)
- [x] 추가 문법 커버리지 (listen, intrinsic 등)
- [x] v2 기능 골든 테스트 (listen 수명, 패턴 매칭, input sugar, 제네릭 추론)
- [x] 패키징 / 설치 검증 자동화
- [x] 실제 Unity 프로젝트 기준 end-to-end 검증 확대
- [x] 배포 인프라 (GitHub Actions release, MSI, winget, VS Code Marketplace)

## 마일스톤 10: v2.1 문서 및 품질 (계획)

상세 설계: `plan_docs/v2.1-design.md`

### 문서 — Tier 1 레퍼런스 매뉴얼
- [ ] `grammar.md` — EBNF 전체 형식 문법
- [ ] `syntax.md` 보강 — 연산자 우선순위, 이스케이프 시퀀스
- [ ] `types.md` 보강 — 타입 추론, 널 안전성, 제네릭 제약
- [ ] `pattern-matching-and-control-flow.md` 보강 — v2 패턴 바인딩/가드/구조 분해
- [ ] `declarations-and-fields.md` 보강 — data class/enum/attribute 예시
- [ ] `events-and-intrinsic.md` 보강 — v2 listen 수명 모델
- [ ] `input-system.md` — v2 New Input System sugar
- [ ] `generic-inference.md` — v2 제네릭 추론 규칙
- [ ] `error-catalog.md` — 에러 코드 전체 카탈로그 (E012~E083)
- [ ] `project-configuration.md` — `.prsmproject` TOML 포맷
- [ ] `source-maps.md` — `.prsmmap.json` 스키마 및 디버깅 워크플로

### 문서 — Tier 2 실용 가이드
- [ ] `getting-started.md` 보강 — 트러블슈팅, Unity 통합
- [ ] `generated-csharp-and-source-maps.md` 보강 — before/after 예시
- [ ] `migration-v1-to-v2.md` — v1→v2 마이그레이션 가이드
- [ ] `idioms.md` — 공통 패턴/안티패턴

### 도구 품질
- [ ] HIR 보강 (제네릭 치환, 식 타입)
- [ ] 소스맵 v2 sugar 세부 매핑
- [ ] VS Code C# bridge 소스맵 전환
- [ ] LSP 증분 인덱싱

### 릴리스 안정화
- [ ] v0.1.0 첫 릴리스 (CI 파이프라인 검증)
- [ ] MSI 설치 E2E 검증
- [ ] winget 등록 확인
