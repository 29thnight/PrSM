# PrSM v3 분석 — 코드 최적화 + 디자인 패턴 언어 지원

## 1. PDF 분석 요약

Unity 공식 가이드 "Level Up Your Code with Design Patterns" (148페이지)에서 다루는 패턴:

| 패턴 | C# 보일러플레이트 | PrSM v3 언어 지원 가능성 |
|------|-------------------|----------------------|
| **싱글톤** | 30줄+ (Instance, Awake 중복 체크, DontDestroyOnLoad, 제네릭 상속) | **높음** — `singleton` 키워드로 1줄 |
| **관찰자/이벤트** | UnityEvent/Action 선언, AddListener/RemoveListener | **이미 구현** — `listen` 문법 |
| **상태 머신** | IState 인터페이스, StateMachine 클래스, Enter/Execute/Exit, 전환 로직 (60줄+) | **높음** — `state` 블록 문법 |
| **커맨드** | ICommand 인터페이스, Execute/Undo, CommandInvoker, 스택 관리 (40줄+) | **중간** — `command` 문법 가능 |
| **팩토리** | IProduct 인터페이스, abstract Factory, Instantiate + GetComponent (30줄+) | **중간** — `factory` 문법 가능 |
| **오브젝트 풀** | Pool 클래스, Stack, Get/Return, 콜백 4개 (50줄+) | **중간** — `pool` 수식자 가능 |
| **MVP/MVVM** | Model/View/Presenter 분리, 바인딩, 이벤트 전파 (80줄+) | **높음** — `bind` 문법 |
| **전략** | 인터페이스 + 여러 구현 클래스 + 런타임 교체 | **낮음** — 기존 인터페이스로 충분 |
| **플라이웨이트** | ScriptableObject 기반 공유 데이터 | **낮음** — `asset` 문법으로 충분 |
| **더티 플래그** | 변경 추적 bool + 조건부 업데이트 | **중간** — `tracked` 수식자 가능 |

---

## 2. 사용자 요청 분석

### 2.1 코드 옵티마이저 (C# 자동 최적화)

현재 lowering은 "읽기 쉬운 C#"을 우선합니다. v3에서 최적화 패스를 추가하면:

**실현 가능한 최적화:**
- **불필요한 임시 변수 제거**: `var _prsm_d = expr; var a = _prsm_d.a;` → `var a = expr.a;` (단일 사용 시)
- **문자열 보간 최적화**: 상수 보간 → 컴파일 타임 연결
- **GetComponent 캐싱**: 같은 타입 반복 호출 감지 → 필드로 승격
- **박싱 회피**: `List<int>` foreach → for 루프 변환 (IL2CPP 최적화)
- **널 체크 병합**: 연속 `?.` 체인 → 단일 null 체크
- **Burst 호환 코드 생성**: `[BurstCompile]` 가능 여부 분석 + 자동 어노테이션

**실현 어려운 최적화 (v3 범위 밖):**
- 전역 흐름 분석 기반 최적화 (전체 프로젝트 의존성 필요)
- 런타임 성능 프로파일링 기반 최적화

### 2.2 디자인 패턴 보일러플레이트 제거

PDF의 패턴을 분석한 결과, PrSM이 언어 수준에서 제거할 수 있는 보일러플레이트:

#### A. 싱글톤 (30줄 → 1줄)

```prsm
// v3
singleton component GameManager : MonoBehaviour {
    var score: Int = 0
}
// 생성: static Instance, Awake 중복 체크, DontDestroyOnLoad 전부 자동
```

#### B. 상태 머신 (60줄+ → 15줄)

```prsm
// v3
component PlayerController : MonoBehaviour {
    state machine {
        idle {
            enter { playAnim("idle") }
            execute { if input.action("Move").held { transition walk } }
            exit { }
        }
        walk {
            enter { playAnim("walk") }
            execute {
                move(input.action("Move").vector2)
                if input.action("Jump").pressed { transition jump }
            }
        }
        jump {
            enter { rb.addForce(vec3(0, jumpForce, 0)) }
            execute { if isGrounded { transition idle } }
        }
    }
}
// 생성: IState 인터페이스, StateMachine 클래스, 각 State 클래스, Enter/Execute/Exit 메서드 전부 자동
```

#### C. 커맨드 (40줄+ → 선언형)

```prsm
// v3
command MoveCommand(player: PlayerMover, movement: Vector3) {
    execute { player.move(movement) }
    undo { player.move(-movement) }
}

// 사용
val cmd = MoveCommand(player, vec3(1, 0, 0))
cmd.execute()
cmd.undo()
```

#### D. 오브젝트 풀 (50줄+ → 수식자)

```prsm
// v3
component BulletSpawner : MonoBehaviour {
    pool bullets: Bullet(capacity = 20, max = 100)

    func fire() {
        val bullet = bullets.get()
        bullet.launch(direction)
    }
}
```

#### E. 데이터 바인딩/MVVM (80줄+ → bind 문법)

```prsm
// v3
component HealthUI : MonoBehaviour {
    bind healthBar.value to player.health / player.maxHealth
    bind healthText.text to "${player.health} / ${player.maxHealth}"
}
```

---

## 3. 추가 제안 사항 (PDF 외)

### 3.1 인터페이스 선언

현재 PrSM은 인터페이스 정의를 지원하지 않음 (C# 인터페이스를 직접 참조만 가능). v3에서:

```prsm
interface IDamageable {
    func takeDamage(amount: Int)
    val isAlive: Bool
}

component Enemy : MonoBehaviour, IDamageable {
    var hp: Int = 100
    val isAlive: Bool = hp > 0
    func takeDamage(amount: Int) { hp -= amount }
}
```

### 3.2 제네릭 선언

현재 제네릭 타입을 사용은 가능하지만 정의할 수 없음:

```prsm
class Pool<T>(capacity: Int) where T : MonoBehaviour {
    // ...
}
```

### 3.3 async/await (UniTask 통합)

Unity 코루틴의 한계를 넘는 비동기:

```prsm
async func loadLevel(name: String) {
    val scene = await SceneManager.LoadSceneAsync(name)
    await UniTask.delay(1000)
    fadeIn()
}
```

### 3.4 ECS 경량 지원

Unity DOTS와의 연결:

```prsm
system MoveSystem : SystemBase {
    query entities with(Translation, Velocity) {
        translation.value += velocity.value * Time.deltaTime
    }
}
```

### 3.5 시리얼라이제이션 강화

```prsm
@serializable
data class SaveData(
    playerName: String,
    level: Int,
    inventory: List<Item>
)
// 자동으로 JSON/Binary 직렬화 코드 생성
```

### 3.6 SOLID 원칙 강제

PDF에서 강조하는 SOLID를 언어 수준에서:

- **단일 책임**: 컴파일러가 컴포넌트의 책임 수를 분석하고 경고 (W010: "이 컴포넌트는 5개 이상의 독립적 관심사를 가집니다")
- **개방-폐쇄**: `sealed` 키워드로 확장 금지 명시, `open` 키워드로 확장 허용
- **종속성 역전**: `require` 에 인터페이스 타입 사용 시 자동 DI

---

## 4. v3 범위 우선순위 제안

### Tier 1 (핵심 — 가장 큰 보일러플레이트 제거)

1. **`singleton` 키워드** — 싱글톤 보일러플레이트 30줄 → 1줄
2. **`state machine` 블록** — 상태 머신 60줄 → 15줄
3. **C# 코드 옵티마이저 패스** — GetComponent 캐싱, 임시 변수 제거
4. **인터페이스 선언** — `interface` 키워드로 PrSM 내에서 인터페이스 정의

### Tier 2 (고가치)

5. **`command` 선언** — 커맨드 패턴 40줄 → 5줄
6. **`pool` 수식자** — 오브젝트 풀 50줄 → 2줄
7. **`bind` 문법** — MVVM 데이터 바인딩
8. **제네릭 선언** — `class<T>` 정의 가능

### Tier 3 (미래)

9. **async/await** (UniTask)
10. **ECS 경량 문법**
11. **SOLID 분석 경고**
12. **직렬화 자동 생성**

---

## 5. 개발 프로세스 (v2.1에서 확정)

**표준 문서 먼저 → 구현**

1. `plan_docs/spec/v3-language-spec.md` — 언어 3 표준 초안
2. 커뮤니티/내부 리뷰
3. 컴파일러 구현
4. 문서 업데이트 (`docs/en/spec/standard.md` 갱신 + `changes-lang-3.md`)
5. 릴리스 (Prism v1.0.0 = 언어 3)
