---
title: Project Configuration & Interop
parent: 고급 주제
grand_parent: 한국어 문서
nav_order: 1
---

# Project Configuration & Interop

## `.prsmproject` 파일

프로젝트 단위 설정은 워크스페이스 루트의 `.prsmproject` JSON 파일로 관리됩니다.

```json
{
  "name": "MyGame",
  "version": "0.1.0",
  "language": {
    "version": "1",
    "features": []
  },
  "compiler": {
    "outputDir": "Assets/Generated",
    "sourceDir": "src"
  },
  "include": ["src/**/*.prsm"],
  "exclude": ["src/tests/**"]
}
```

### 필드 참조

| 필드 | 타입 | 설명 |
|---|---|---|
| `name` | string | 프로젝트 표시 이름 |
| `version` | string | SemVer 프로젝트 버전 |
| `language.version` | string | 대상 PrSM 언어 버전 |
| `language.features` | array | 선택적 활성화 기능 플래그 |
| `compiler.outputDir` | string | 생성된 `.cs` 파일이 기록되는 디렉토리 |
| `compiler.sourceDir` | string | 소스 해석의 루트 디렉토리 |
| `include` | glob 배열 | 컴파일 대상 소스 파일 |
| `exclude` | glob 배열 | 컴파일 제외 소스 파일 |

## Interop

생성되는 C#은 의도적으로 읽기 쉽고 Unity 친화적으로 설계되어 있습니다. 별도의 브리징 레이어나 래퍼는 도입되지 않습니다.

| PrSM 구문 | 생성되는 C# |
|---|---|
| `component T : MonoBehaviour` | `public class T : MonoBehaviour` |
| `asset T : ScriptableObject` | `public class T : ScriptableObject` |
| `class T` | `public class T` |
| `data class T(...)` | 생성자 + equality 멤버를 갖는 직렬화 클래스 |
| `coroutine f(): Unit` | `public IEnumerator f()` |
| `enum E { V(payload) }` | `enum E` + 중첩 payload struct + 확장 메서드 |

## PrSM에서 C# API 호출

모든 Unity 또는 C# API는 직접 접근 가능합니다. PrSM은 프로젝트가 이미 의존하는 Unity 어셈블리 참조를 통해 타입을 해석하기 때문에 특별한 import 없이 호출할 수 있습니다.

```prsm
val obj = GameObject.Find("Target")
obj.SetActive(false)
```

## 생성된 코드를 C#에서 호출

출력이 일반 C#이기 때문에, 어떤 Unity 스크립트나 에디터 코드도 생성된 클래스를 호출하거나 컴포넌트 메서드·에셋을 참조할 수 있습니다. 별도의 import가 필요하지 않습니다.
