---
title: PrSM 3
parent: 사양
nav_order: 5
---

# PrSM 언어 3

PrSM 3은 인터페이스 및 제네릭 선언, 디자인 패턴 편의 기능(`singleton`, `pool`), 코드 품질 분석, 그리고 첫 번째 컴파일러 최적화 도구를 도입합니다. 이 릴리스는 **Prism v1.0.0**을 대상으로 합니다.

**활성화:** `.prsmproject`에서 `language.version = "3"` 설정

## 새로운 언어 기능

### 인터페이스 선언

메서드 시그니처와 프로퍼티를 갖는 PrSM 네이티브 인터페이스 정의:

```prsm
interface IDamageable {
    func takeDamage(amount: Int)
    val isAlive: Bool
}

interface IHealable : IDamageable {
    func heal(amount: Int)
}
```

- 표준 C# `interface`로 변환
- 인터페이스 멤버는 구현 본문을 가질 수 없음 (E091)
- 구현하는 component/class는 모든 멤버를 정의해야 함 (E090)
- `require` 필드에서 지원: `require target: IDamageable`

### 제네릭 선언

`class` 및 `func`에 대한 타입 매개변수와 `where` 제약 조건:

```prsm
class Registry<T> where T : Component {
    var items: List<T> = null
    func register(item: T) { items.add(item) }
}

func findAll<T>(): List<T> where T : Component {
    return FindObjectsByType<T>(FindObjectsSortMode.None).toList()
}
```

- 여러 타입 매개변수: `class Pair<K, V>`
- 여러 제약 조건: `where T : MonoBehaviour, IDamageable`
- 제네릭 `interface` 지원: `interface IPool<T> { func get(): T }`
- `component`, `asset`, `enum`, `data class`에는 지원되지 않음 (E096)

### `singleton` 키워드

키워드 하나로 싱글턴 component 생성:

```prsm
singleton component AudioManager : MonoBehaviour {
    serialize volume: Float = 1.0
    func playSound(clip: AudioClip) { /* ... */ }
}

// 어디서든 사용:
AudioManager.instance.playSound(clip)
```

자동 생성 항목:
- `private static T _instance` 필드
- 지연 초기화를 포함한 `public static T Instance` 프로퍼티 (`FindFirstObjectByType` + `AddComponent` 폴백)
- `Awake` 가드: 중복 제거 + `DontDestroyOnLoad`

### `pool` 수정자

두 줄로 오브젝트 풀링 구현:

```prsm
component BulletSpawner : MonoBehaviour {
    serialize bulletPrefab: Bullet
    pool bullets: Bullet(capacity = 20, max = 100)

    func fire(direction: Vector3) {
        val bullet = bullets.get()
        bullet.launch(direction)
    }
}
```

- `UnityEngine.Pool.ObjectPool<T>` 기반
- 자동 생성: `createFunc`, `actionOnGet`, `actionOnRelease`, `actionOnDestroy` 콜백
- 프리팹을 위해 타입으로 `serialize` 필드를 매칭 (없으면 E098)
- component 전용 (외부에서는 E099)

## 컴파일러 개선

### SOLID 분석 경고

일반적인 설계 문제를 감지하는 정적 분석 패스:

| 코드 | 원칙 | 조건 |
|------|------|------|
| W010 | 단일 책임 | component에 공개 메서드가 8개 이상 |
| W011 | 의존성 역전 | component에 의존성 필드가 6개 이상 |
| W012 | 단일 책임 | 메서드/라이프사이클에 구문이 50개 이상 |

`.prsmproject`에서 구성 가능:

```toml
[analysis]
solid_warnings = true
disabled_warnings = ["W012"]
```

### 코드 최적화 도구

더 깔끔하고 빠른 C# 출력을 위한 변환 최적화:

**단일 바인딩 구조 분해 인라인:**
```prsm
val Stats(hp) = getStats()
```
변환 전: `var _prsm_d = getStats(); var hp = _prsm_d.hp;`
변환 후: `var hp = getStats().hp;`

### 예약된 편의 기능 이름

`get`과 `find`는 이제 내장 메서드 이름으로 예약됩니다 (E101). 이 이름을 사용하는 사용자 정의 함수는 편의 기능 하이재킹을 방지하기 위해 컴파일 오류를 발생시킵니다.

## 새로운 진단

| 코드 | 심각도 | 설명 |
|------|--------|------|
| E090 | 오류 | 인터페이스 멤버가 구현되지 않음 |
| E091 | 오류 | 인터페이스 멤버에 구현 본문이 있음 |
| E095 | 오류 | 타입 인수가 `where` 제약 조건을 위반함 |
| E096 | 오류 | component/asset/enum/data class에 제네릭 매개변수 사용 |
| E097 | 오류 | component가 아닌 선언에 `singleton` 사용 |
| E098 | 오류 | 풀 타입에 일치하는 serialize 프리팹이 없음 |
| E099 | 오류 | component 외부에서 `pool` 사용 |
| E101 | 오류 | 예약된 내장 메서드 이름 (`get`, `find`) |
| W010 | 경고 | 공개 메서드가 너무 많음 (SOLID) |
| W011 | 경고 | 의존성 필드가 너무 많음 (SOLID) |
| W012 | 경고 | 메서드가 너무 긺 (SOLID) |

## 기능 게이트

언어 3의 모든 기능은 `version = "3"` 설정 시 암묵적으로 활성화됩니다. 개별 기능은 언어 2에서 선택적으로 활성화할 수 있습니다:

```toml
[language]
version = "2"
features = ["interface", "generics"]
```

| 기능 플래그 | 설명 |
|------------|------|
| `interface` | 인터페이스 선언 |
| `generics` | where 절을 갖는 제네릭 class/func |
| `singleton` | 싱글턴 component 키워드 |
| `pool` | 오브젝트 풀 수정자 |
| `solid-analysis` | SOLID 경고 |
| `optimizer` | 코드 최적화 도구 |

## 툴체인 개선

- Windows용 MSI 설치 프로그램 (원클릭: 컴파일러 + VS Code 확장 + Unity 가이드)
- `winget install PrSM.PrSM` 지원
- GitHub Actions 릴리스 파이프라인 (3플랫폼 빌드 + VSIX + MSI + Marketplace + winget)
- 모든 구성 요소의 단일 버전 관리를 위한 `scripts/bump-version.sh`
- `_nav.json`을 통한 동적 문서 탐색
