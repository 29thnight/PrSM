---
title: 프로젝트 설정
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 11
---

# 프로젝트 설정

모든 PrSM 워크스페이스는 프로젝트 루트에 있는 `.prsmproject` 파일을 기반으로 합니다. 이 파일은 TOML 형식을 사용하며, 컴파일러가 소스 파일을 탐색, 컴파일, 출력하는 방식을 제어합니다.

## 최소 예제

```toml
[project]
name = "MyGame"
prsm_version = "0.1.0"

[language]
version = "1.0"

[compiler]
output_dir = "Assets/Generated"
```

## 섹션 참조

### `[project]`

| 키 | 타입 | 기본값 | 설명 |
|---|---|---|---|
| `name` | string | **필수** | 프로젝트의 표시 이름 |
| `prsm_version` | string | `"0.1.0"` | 이 프로젝트가 대상으로 하는 PrSM 툴체인의 SemVer 버전 |

### `[language]`

| 키 | 타입 | 기본값 | 설명 |
|---|---|---|---|
| `version` | string | `"1.0"` | 언어 버전: `"1.0"` 또는 `"2.0"` |
| `features` | 문자열 배열 | `[]` | 활성화할 기능 플래그 (아래 참조) |

### `[compiler]`

| 키 | 타입 | 기본값 | 설명 |
|---|---|---|---|
| `output_dir` | string | `"Assets/Generated"` | 생성된 `.cs` 파일이 기록되는 디렉터리 |
| `prism_path` | string | 자동 감지 | `prism` 컴파일러 바이너리의 명시적 경로; PATH 조회를 재정의 |

### `[source]`

| 키 | 타입 | 기본값 | 설명 |
|---|---|---|---|
| `include` | glob 배열 | `["**/*.prsm"]` | 컴파일할 소스 파일의 glob 패턴 |
| `exclude` | glob 배열 | `[]` | 컴파일에서 제외할 glob 패턴 |

### `[features]`

| 키 | 타입 | 기본값 | 설명 |
|---|---|---|---|
| `auto_compile_on_save` | bool | `true` | `.prsm` 파일 저장 시 자동으로 재컴파일 |
| `generate_meta_files` | bool | `true` | Unity 에셋 데이터베이스 통합을 위한 `.cs.meta` 파일 생성 |
| `pascal_case_methods` | bool | `true` | camelCase 대신 PascalCase C# 메서드 이름 생성 |

## 기능 플래그

기능 플래그는 `language.features`에 나열되며, 실험적이거나 명시적 참여가 필요한 구문을 제어합니다.

| 플래그 | 요구 사항 | 설명 |
|---|---|---|
| `auto-unlisten` | language 1.0+ | 모든 `listen` 문에 대해 `OnDestroy`에서 `RemoveListener` 호출을 자동으로 생성 |
| `input-system` | language 1.0+ | 새로운 Unity Input System 패키지를 위한 Input System 편의 구문 활성화 |
| `pattern-bindings` | language 1.0+ | enum 페이로드 구조 분해를 위해 `when` 패턴 내에서 `val` 바인딩 허용 |

`language.version`이 `"2.0"`인 경우, `auto-unlisten`과 `pattern-bindings` 플래그는 암묵적으로 활성화됩니다. 명시적으로 나열해도 오류가 발생하지 않습니다.

## Unity 통합

컴파일러는 프로젝트 루트의 `Packages/manifest.json`을 검사하여 Unity 프로젝트 기능을 감지합니다. `com.unity.inputsystem` 패키지가 있으면 Input System API를 해석에 사용할 수 있습니다. `com.unity.textmeshpro`가 있으면 TMP 타입을 인식합니다. 패키지 감지를 위한 수동 설정은 필요하지 않습니다.

Unity 프로젝트가 Assembly Definition(`.asmdef`)을 사용하는 경우, 생성된 출력 디렉터리는 동일한 어셈블리 범위 안에 있어야 Unity가 생성된 C#을 나머지 프로젝트 코드와 함께 컴파일합니다.

## 기본값 요약

```toml
[project]
name = "Untitled"
prsm_version = "0.1.0"

[language]
version = "1.0"
features = []

[compiler]
output_dir = "Assets/Generated"
# prism_path는 PATH에서 자동 감지

[source]
include = ["**/*.prsm"]
exclude = []

[features]
auto_compile_on_save = true
generate_meta_files = true
pascal_case_methods = true
```

## `.mnproject`에서의 레거시 마이그레이션

이전 버전의 툴체인은 `.mnproject` JSON 파일을 사용했습니다(원래 "Moon" 프로젝트 이름에서 유래). 컴파일러는 여전히 `.mnproject`를 인식하고 동일한 필드 의미로 로드하지만, 둘 다 존재하는 경우 TOML 기반의 `.prsmproject`가 우선합니다. 마이그레이션 방법:

1. 위에 표시된 TOML 형식을 사용하여 워크스페이스 루트에 새 `.prsmproject`를 생성합니다.
2. `.mnproject`의 필드 값을 해당하는 TOML 섹션으로 복사합니다.
3. 새 설정이 확인되면 이전 `.mnproject` 파일을 삭제합니다.

컴파일러는 `.mnproject`로 폴백할 때 마이그레이션을 알리는 일회성 안내 메시지를 출력합니다.
