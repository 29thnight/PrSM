# PrSM 언어 5 — 구현 계획

**상태:** Draft v0.1
**날짜:** 2026-04-07
**선행 조건:** 언어 4 (Prism v2.0.0)
**대상:** Unity 2022.3+ (IL2CPP / Mono)
**도구 버전 (예정):** Prism v2.1.0 / Prism v3.0.0

---

## 배경

언어 4는 PrSM 역사상 가장 큰 단일 문법 갭 해소 릴리스였다 (30개 기능, 47개 진단). 이후 감사를 통해 PrSM이 직접 표현하지 못하는 C# 구조를 식별했고, 이 문서는 그 감사 결과를 **Unity 관련 항목으로 한정**하여 정리한 구현 계획이다.

감사는 두 부분으로 나뉜다:

- **Part A** — PrSM이 현재 표현하지 못하는 C# 문법 (개발자가 `intrinsic`으로 우회해야 하는 항목)
- **Part B** — 언어 4에서 구현되었지만 알려진 제약이 있는 기능

Unity 워크플로와 무관한 항목 (`checked`/`unchecked`, `volatile`, `goto`, finalizer, top-level statements, LINQ 쿼리 문법, 익명 타입, file-scoped namespace, `lock`, `record`/`record struct`, 리스트 패턴, 제네릭 변성, `IAsyncEnumerable`/`await foreach`/`await using`)은 이 계획에서 제외하지만, 완전성을 위해 `syntax-gap-analysis.md`에 보존되어 있다.

---

## Part A — PrSM 문법 부재

### 매개변수

| 기능 | PrSM 문법 | C# lowering | 컴파일러 작업 |
|------|----------|------------|--------------|
| `ref` / `out` 매개변수 | `func tryGet(out value: Item): Bool` — 호출: `physics.raycast(ray, out val hit)` | `bool tryGet(out Item value)` — 호출: `Physics.Raycast(ray, out var hit)` | parser: `Param` modifier 파싱. AST: `Param.modifier: ParamMod`. lowering: modifier 출력 + `out var x` declaration expression 형식 지원 |
| `params` (가변 인자) | `func log(vararg messages: String)` (Kotlin 스타일) | `void log(params string[] messages)` | lexer: `vararg`를 contextual keyword로 처리. parser: 마지막 매개변수에만 허용. AST: `Param.is_vararg: bool`. lowering: `params T[]` 출력 |
| 기본값 매개변수 | `func instantiate(prefab: GameObject, parent: Transform? = null)` | 동일 | parser: 타입 뒤 `=` 기본값 식 허용. AST: `Param.default_value: Option<Expr>`. semantic: literal/const 식만 허용 |
| 명명 인자 | `instantiate(prefab, parent: rootTransform)` | 동일 (C#도 같은 문법) | parser: AST의 `Arg.name: Option<String>`이 이미 존재 — 파서 경로만 활성화. semantic: "명명 인자 뒤 위치 인자 금지" 규칙 강제 |

### 저수준 / 성능

| 기능 | PrSM 문법 | C# lowering | 컴파일러 작업 |
|------|----------|------------|--------------|
| `unsafe` 블록 | (새 문법 없음) — `intrinsic { unsafe { ... } }` 사용 | `unsafe { ... }` | **컴파일러 변경 없음** — `intrinsic` 패턴을 문서화. unsafe 코드는 드물어서 전용 PrSM 구조의 가치가 낮음 |
| `stackalloc` | `val buffer: Span<Int> = stackalloc[Int](256)` | `Span<int> buffer = stackalloc int[256]` | lexer: `stackalloc` 키워드. parser: primary expression. AST: `Expr::Stackalloc { ty, size }`. lowering: 직접 |
| `ref struct` | `ref struct Slice<T>(start: Int, length: Int)` | `public ref struct Slice<T> { ... }` | parser: `struct` 앞 `ref` modifier. AST: `Decl::Struct.is_ref: bool`. semantic: ref struct 사용 제약 강제 (비-ref struct의 필드 금지, 박싱 금지) |
| `ref local` / `ref return` | `val ref pos = transform.position` / `func getRef(): ref Vector3 = ref _pos` | `ref var pos = ref transform.position` / `public ref Vector3 getRef() => ref _pos` | parser: `val ref` / `var ref`. AST: `ValDecl.is_ref` + `TypeRef.is_ref`. **opt.structcopy 실효화의 핵심** |
| `Span<T>` 슬라이스 문법 | `arr[1..5]` (기존 range 연산자 재사용) | `arr[1..5]` (C# 8 range) 또는 `arr.AsSpan(1, 4)` | parser: `IndexAccess`가 이미 `Range` 식을 받을 수 있음. lowering: 타깃이 Span/array일 때 메서드 호출 대신 C# range 문법 출력 |

### 패턴 매칭

| 기능 | PrSM 문법 | C# lowering | 컴파일러 작업 |
|------|----------|------------|--------------|
| 관계 패턴 | `when hp { > 80 => "Healthy"; > 30 => "Hurt"; else => "Dying" }` | `case > 80: ... case > 30: ...` (C# 9 switch) | parser: `WhenPattern::Relational { op, value }`. lowering: switch expression / statement에서 C# 9 패턴 직접 출력 |
| 패턴 결합자 (`and`/`or`/`not`) | `when x { > 0 and < 100 => ...; is Enemy or is Boss => ...; not null => ... }` | 동일 | parser: 우선순위가 있는 pattern combinator (`not` > `and` > `or`). AST: `WhenPattern::And/Or/Not`. **기존 쉼표-OR 패턴을 새 `or` 키워드와 통합** |
| 위치 패턴 | `Point(0, 0) => "origin"` (struct로 확장) | `case Point(0, 0):` | 기존 enum payload binding 로직을 `struct` 와 `data class`로 확장. AST: `WhenPattern::Positional`로 통합 |
| 프로퍼티 패턴 | `Point { x: 0, y: > 0 } => ...` | `case Point { x: 0, y: > 0 }:` | parser: 새 `WhenPattern::Property { fields: Vec<(Ident, Pattern)> }`. lowering: 직접 |
| `with` 표현식 | `transform.position with { y = 0 }` | data class: `record with` / Unity struct: 임시 변수 + 필드 복사 + 수정 | AST: `Expr::With { receiver, fields }`. lowering: data class → C# `with`; struct → `var _t = orig; _t.y = 0; _t` IIFE 패턴 |

### 비동기 / 코루틴

| 기능 | PrSM 문법 | C# lowering | 컴파일러 작업 |
|------|----------|------------|--------------|
| 일반 `yield return` | `coroutine fadeOut(): Seq<Float> { for t in 0.0..1.0 step 0.01 { yield t } yield break }` | `IEnumerator<float> fadeOut() { for(...) yield return t; yield break; }` | parser: `yield` / `yield break` 문장. AST: `Stmt::Yield { value }`, `Stmt::YieldBreak`. semantic: 반환 타입이 `Seq<T>` / `IEnumerator` / `IEnumerable`일 때만 허용. **기존 코루틴 구현(`wait` / `start`)과 통합 필수** |

### 타입 시스템

| 기능 | PrSM 문법 | C# lowering | 컴파일러 작업 |
|------|----------|------------|--------------|
| `nameof(x)` | `nameof(hp)` | `nameof(hp)` | lexer: `nameof` contextual keyword. parser: primary expression. AST: `Expr::NameOf { target: Path }`. semantic: 식별자 존재 검증. lowering: 직접. **`bind` setter의 자동 생성된 `OnPropertyChanged(nameof(hp))`에 재사용** |
| `partial class` | `partial component Player : MonoBehaviour { ... }` | `public partial class Player { ... }` | parser: `partial` modifier. AST: `Decl::Component.is_partial`. **"파일당 단일 선언" 규칙 완화** — partial일 때만 동일 이름 허용. lowering 패스에서 결합 |
| 임의 위치 nested class | 모든 component/class 본문에 `class Inner { }` 허용 | nested C# class | parser: member 위치에서 declaration 허용. AST: `Member::NestedDecl { decl: Box<Decl> }`. lowering: nested cs members로 출력. (현재는 `sealed class` 서브타입에서만 가능 — 일반화) |
| `unmanaged` 제약 | `func compute<T>(): T where T : unmanaged` | `where T : unmanaged` | parser: where clause에 `unmanaged` / `notnull` / `default` 키워드 허용. AST: `WhereConstraint::Unmanaged/NotNull`. **Burst 분석과 자동 연계** |

### 표현식

| 기능 | PrSM 문법 | C# lowering | 컴파일러 작업 |
|------|----------|------------|--------------|
| Discard `_` | `physics.raycast(ray, out _)` / `val (_, name) = getResult()` | `Physics.Raycast(ray, out _)` / `var (_, name) = ...` | parser: `_`를 special expr/pattern으로 인식. AST: `Expr::Discard`, `Pattern::Discard`. semantic: discard에서 읽기 금지 |
| 인덱서 조건부 접근 `arr?[0]` | `arr?[0]` | 동일 | parser: postfix `?[` 토큰 시퀀스. AST: `Expr::SafeIndexAccess { receiver, index }`. lowering: 직접 |
| Throw 표현식 | `val rb = body ?? throw Exception("missing")` | `var rb = body ?? throw new Exception("missing");` | parser: `throw`를 expression position에서 허용. AST: `Expr::Throw { exception }` (기존 `Stmt::Throw`와 별도). lowering: 직접 |

### 어트리뷰트 / 전처리

| 기능 | PrSM 문법 | C# lowering | 컴파일러 작업 |
|------|----------|------------|--------------|
| `[field: SerializeField]` | 자동 프로퍼티에 `serialize` modifier가 붙으면 어트리뷰트 자동 생성. 또는 명시적으로 `@field(serialize)` | `[field: SerializeField] public int hp { get; set; }` | **단순 접근**: `lower_property_member`에서 `serialize` modifier 검출 시 `[field: SerializeField]` 자동 출력. **일반 접근**: `@field(...)` / `@property(...)` / `@param(...)` 어노테이션 target 문법 추가 |
| `#if UNITY_EDITOR` | `#if editor { ... }` 또는 `#when editor { ... }` (PrSM 스타일) | `#if UNITY_EDITOR ... #endif` | lexer: `#if` / `#else` / `#endif` 디렉티브 토큰. parser: statement 위치에서 directive 블록. AST: `Stmt::Preprocessor { kind, body }` 또는 별도 노드. lowering: 그대로 전달. **사전 정의 매핑**: `editor` → `UNITY_EDITOR`, `ios` → `UNITY_IOS`, `standalone` → `UNITY_STANDALONE`, `il2cpp` → `ENABLE_IL2CPP` 등 |

---

## Part B — 언어 4 알려진 제약

| 제약 | 현재 상태 | 디자인 | 컴파일러 작업 |
|------|----------|--------|--------------|
| `bind X to Y` 연속 푸시 | 초기 동기화만 수행 | 컴포넌트마다 push target 람다 리스트를 등록, `bind to` 사이트에서 등록, setter에서 발화 | `lower_bind_property` setter: `if (_pushTargets_X != null) foreach (var t in _pushTargets_X) t(value);`. `BindTo` lowering: `_pushTargets_X.Add(v => target = v)` 등록 |
| `bind` W031 (never read) | 미구현 | 컴포넌트 전체에 걸쳐 `bind` 멤버 식별자의 표현식 사용을 추적 | `analyzer.rs`: 의미 분석 패스에 `bind_usage_count` HashMap 추가. 카운트가 0이면 W031 발생 |
| state machine 이름 충돌 | 예약어 (`Start`/`Stop`/`Update`)를 상태명으로 사용 불가 | `state` 키워드 컨텍스트에서 PrSM 키워드도 식별자로 허용 | parser `parse_state_decl`: `expect_ident` 대신 `expect_ident_or_keyword` 사용 |
| `@burst` 어노테이션 파싱 | 명명 휴리스틱(`burst_*`)만 | `Member::Func`에 `annotations: Vec<Annotation>` 필드 추가 | parser: 기존 `@header` 어노테이션 파싱 확장. burst analyzer: 명명 휴리스틱에서 어노테이션 lookup으로 전환. lowering: `[BurstCompile]` 출력 |
| `opt.linq` element type 추론 | `List<object>` fallback | IR walk를 통해 `xs.Where(...).ToList()`의 xs의 정적 타입 추적 | `collect_callable_signatures`: `List<T>`의 `T` 보존. `optimizer.rs`: 재작성된 `for` 루프에서 `var` 대신 명시적 element type 사용 |
| `opt.structcopy` 실효화 | 코멘트 힌트만 | `ref readonly` 로컬 도입 (Part A의 ref local 기능에 의존) | ref local 구현 후: optimizer가 큰 struct 로컬을 검출 → `ref readonly Vector3 pos = ref transform.position;` 출력 |
| 옵티마이저 driver 자동 연결 | API는 노출되었으나 `compile_file`에 미연결 | 파이프라인에 옵션 기반 `run_optimizer` 호출 추가 | `driver.rs`: `if options.optimize { run_optimizer(...) }`. `main.rs`: `--optimize` CLI 플래그 등록 |
| 리팩토링 LSP dispatch | capability advertise만, 핸들러 dispatch 없음 | LSP `textDocument/codeAction` 핸들러가 refactor 헬퍼로 라우팅 | `lsp.rs`: code action 핸들러가 selection range 분석, 적용 가능한 refactor 검출, `refactor::*` 함수 호출, `WorkspaceEdit` 반환 |
| 디버거 DAP 어댑터 | flat source map만 존재 | VS Code 확장이 flat source map을 소비하는 DAP 어댑터 등록 | `vscode-prsm`: `vscode.debug.registerDebugAdapterDescriptorFactory` 추가. DAP launch / attach / breakpoints / scopes / variables 핸들러 구현 |
| async UniTask 자동 감지 | 항상 UniTask 출력 | `.prsmproject` 또는 Unity `manifest.json`에서 `com.cysharp.unitask` 스캔 | `driver.rs`: 프로젝트 manifest 파싱. 패키지 존재 감지. 컴파일 옵션을 통해 lowering에 전달 |
| DIM 본문 component sugar | `lower_stmt` (no-context) — sugar 인식 못 함 | 두 가지 옵션: (1) 인터페이스가 어떤 컴포넌트 멤버를 요구하는지 선언; (2) 디폴트 본문에서 `this` 한정 호출만 허용 | (2) 권장 — spec 의도와 부합. **문서화 변경만**, 컴파일러 작업 없음 |
| `unlisten` 컴포넌트 컨텍스트 밖 | placeholder 코멘트 | 의미 분석에서 컴포넌트 단위로 unlisten 호출 사이트 수집 | `analyzer.rs`: 컴포넌트별 `unlisten_sites` 수집. 컴포넌트 lowering 패스: 모든 unlisten 사이트를 listen 토큰 lookup으로 후처리 |

---

## 우선순위 + 난이도 + 의존성

| # | 항목 | 영향도 | 난이도 | 의존 | 권장 sprint |
|---|------|--------|--------|------|-------------|
| 1 | 일반 `yield return` | ★★★★★ | M | 기존 코루틴 통합 | v5 sprint 1 |
| 2 | Attribute target (`[field: SerializeField]`) | ★★★★★ | S | 프로퍼티 lowering | v5 sprint 1 |
| 3 | `#if UNITY_EDITOR` 전처리 | ★★★★★ | M | lexer / parser 확장 | v5 sprint 1 |
| 4 | `params` + 기본값 매개변수 + `out` | ★★★★ | S | parser modifier 작업 | v5 sprint 2 |
| 5 | `@burst` 어노테이션 | ★★★★ | S | 어노테이션 AST | v5 sprint 2 |
| 6 | `nameof` | ★★★★ | S | 새 표현식 | v5 sprint 2 |
| 7 | UniTask 자동 감지 | ★★★★ | S | manifest 파싱 | v5 sprint 2 |
| 8 | `bind` 연속 푸시 + W031 | ★★★ | M | bind setter 확장 | v5 sprint 3 |
| 9 | `ref local` / `ref return` | ★★★ | M | 타입 시스템 확장 | v5 sprint 3 |
| 10 | opt.structcopy 실효화 | ★★★ | S | #9 항목 | v5 sprint 3 |
| 11 | 관계 패턴 + 결합자 | ★★★ | M | when 패턴 확장 | v5 sprint 4 |
| 12 | `unmanaged` 제약 | ★★★ | S | where clause 확장 | v5 sprint 4 |
| 13 | Discard `_` | ★★ | S | 패턴 / 표현식 확장 | v5 sprint 4 |
| 14 | `partial class` | ★★ | L | 파일 유일성 규칙 완화 | v5 sprint 5 |
| 15 | 일반화된 nested class | ★★ | M | sealed 로직 일반화 | v5 sprint 5 |
| 16 | 옵티마이저 CLI + LSP refactor dispatch | ★★ | S | driver 통합 | v5 sprint 5 |
| 17 | DAP 어댑터 구현 | ★★ | L | 별도 트랙 | v5 sprint 6 |
| 18 | 위치 / 프로퍼티 패턴 + `with` | ★★ | L | struct / data class 확장 | v5 sprint 6 |
| 19 | `arr?[0]`, throw 표현식 | ★ | S | 작은 parser 확장 | v5 sprint 6 |
| 20 | `stackalloc`, `ref struct`, Span 슬라이스 | ★ | M | 저수준 타입 시스템 | v5 sprint 6 |

**난이도 표기**: S = 1–2일, M = 3–7일, L = 1–2주

---

## 권장 사항

Sprint 1만 (`yield return` / `[field: SerializeField]` / `#if UNITY_EDITOR`) 도입해도 실제 Unity 프로젝트의 `intrinsic` 사용량이 가장 큰 폭으로 감소할 것이다. 이 세 가지가 현재 Unity 개발자가 raw C#으로 떨어지는 가장 흔한 이유이기 때문이다.

Sprint 2 (`params` / `out` / 기본값 매개변수 / `nameof` / `@burst` / UniTask 감지)를 추가하면 일상적인 Unity API 호출 사이트에서 `intrinsic`이 사실상 사라진다.

Sprint 3는 언어 4의 알려진 제약을 마무리한다 (`bind` 연속 푸시, `ref local`, `opt.structcopy`). v4 기능 중 "구현됐지만 부분적"이었던 것을 "구현되어 완성됨"으로 전환한다.

Sprint 4–6은 PrSM을 C# 11 패턴과 동등한 수준으로 확장하고, 개발자 경험 도구 스토리(옵티마이저 CLI, LSP refactor, DAP)를 완성하며, 남은 저빈도 갭을 메우는 quality-of-life 확장이다.

---

## 범위 외

다음 항목은 Unity 특화 동기가 없어서 이 계획에 의도적으로 **포함되지 않는다**. 미래 참조를 위해 `syntax-gap-analysis.md`에 문서화되어 있다:

- `checked` / `unchecked`, `volatile`, `goto`, finalizer
- `top-level statements`, file-scoped namespace
- LINQ 쿼리 문법 (`from x in xs select x`)
- 익명 타입
- `lock`
- `record` / `record struct` (`data class` / `struct`로 대체)
- 리스트 패턴
- 제네릭 변성 (`in T` / `out T`)
- `IAsyncEnumerable<T>` / `await foreach` / `await using`
- `using static`, global using

특정 Unity 워크플로가 위 항목 중 하나의 필요성을 표면화하면 (예: `IAsyncEnumerable`을 노출하는 Unity 기능), 해당 항목은 미래 v5/v6 계획 개정으로 이동한다.

---

## 출시 기준

- Sprint 1–3 항목 모두 구현, 테스트, 문서화 완료
- v4 알려진 제약이 해결되거나 명시적으로 연기됨
- Sprint 4–6 항목은 개별 feature gate로 단계적 출시 가능
- `docs/en/spec/lang-5.md` 와 `docs/ko/spec/lang-5.md` 가 documentation style guide를 따라 작성됨
- `docs/en/migration-v1-to-v2.md` 에 v4 → v5 섹션 추가
- `prism version` 이 v5 릴리스 버전 출력 (스코프에 따라 Prism v2.1.0 또는 v3.0.0)
