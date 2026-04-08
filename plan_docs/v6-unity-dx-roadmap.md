# PrSM v3.6 → v5.0 로드맵 — "Unity 전용 코틀린" 극단화

**상태:** Draft v0.1
**날짜:** 2026-04-08
**선행 조건:** PrSM v3.5.0 (6 라운드 감사 완료, 100 개 이슈 closed)
**대상:** Unity 2022.3 LTS → Unity 6.8 CoreCLR (2026 Q4)
**도구 버전 (예정):** Prism v3.6.0 → v5.0.0

---

## 전략

**Unity-first DX 극단화.** 범용 .NET 언어로 피벗하지 않는다. 대신 Unity 에서만큼은 C# 이 부끄러워지게 만든다.

### 핵심 원칙

1. **Unity 개발자가 하루에 10 번 만지는 영역부터 공격** — tag, scene, animator, input, UI 순
2. **"C# 에서 string 이던 것을 PrSM 에서 compile-time 타입으로"** — 한 세대 DX 점프
3. **각 feature 는 독립 sugar pack** — 프로젝트가 안 쓰면 코드 크기 0
4. **Unity 의 공식 API 와 정렬** — Unity 가 deprecate 하면 따라가기
5. **매 버전마다 "1개 메이저 테마 + 폴리싱"** — 범위 폭발 방지
6. **Spec 파일 분리** — `lang-5.md` 뒤에 `lang-6.md`, `lang-7.md` 로 신기능을 얹는 방식

### 명시적 비-목표

| 안 함 | 이유 |
|-------|------|
| 범용 .NET 언어 (ASP.NET, Blazor, MAUI) | Unity DX 에 집중 |
| Godot 타겟 | 다른 엔진 지원은 v6.0+ 검토 |
| Hot reload 자체 구현 | Unity 자체 hot reload 나 Rider 에 위임 |
| 런타임 인터프리터 | AOT 만 지원 (IL2CPP 호환) |
| 새로운 GC / 메모리 모델 | C# 으로 transpile 하는 한 그대로 |
| PrSM 플러그인 시스템 | 언어 본체 안정화 후 검토 |
| 메타프로그래밍 / 매크로 | 복잡도 폭발 |
| Microsoft C# 와 문법 우위만으로 경쟁 | Unity 통합 + 문법 우위 조합이 강점 |

### 의사 결정 원칙

의심될 때마다 돌아올 3가지:

1. **"이 기능은 C# 으로 쓸 때 얼마나 고통스러운가?"** — 고통 큰 것부터 잡는다
2. **"이 기능 덕분에 Unity 외 환경에서도 쓸 만해지는가?"** — 그렇다면 **안 만든다** (방향 위배)
3. **"이 기능 때문에 기존 PrSM 유저가 프로젝트 리라이트를 강요받는가?"** — 그렇다면 **신중하게**, deprecation 기간 필수

---

## 현재 위치 (v3.5.0) 복기

### 이미 가진 것

| 영역 | 기능 |
|------|------|
| 컴포넌트 | `component X : MonoBehaviour`, `require`, `optional`, `child`, `parent`, `pool`, `singleton` |
| 수명주기 | `awake/start/update/fixedUpdate/lateUpdate/onEnable/onDisable/onDestroy/onTrigger*/onCollision*` |
| 이벤트/리스너 | `listen ... until disable`, `manual + unlisten`, member-level `listen` |
| 상태 관리 | `state machine`, `command`, `bind` + `bind to` + `PropertyChanged` |
| 코루틴 | `coroutine`, `wait 1.0s`, `wait nextFrame`, `wait fixedFrame`, `start/stop/stopAll` |
| 타입 시스템 | data class, sealed class, enum with payload, pattern matching (`or`/`and`/`not`/relational/range), smart cast, null safety (`?`, `!!`, `?.`, `?:`), named tuple |
| 성능 | `@burst` + Burst analyzer, `opt.linq`/`opt.string`/`opt.structcopy` optimizer, `ref struct`, `stackalloc`, `Span<T>` slice |
| Input System | `input.axis/getKey/...` sugar (기초 수준) |
| 디버거 | Source map v2 (스택 트레이스 역추적, DAP 어댑터) |

### 공백 영역 (약점)

| 영역 | 현재 Unity 의 고통 | PrSM 기회 |
|------|-------------------|-----------|
| **Tag / Layer / Scene / Sort Layer** | 전부 string 기반, typo 가 runtime error | 컴파일 타임 상수화 |
| **Animator parameters** | `SetFloat("Speed", v)` string 기반 | type-safe enum |
| **Shader properties / keywords** | `material.SetFloat("_MainTex_ST", ...)` string 기반 | type-safe |
| **Input System actions** | generated C# class 는 subscribe/unsubscribe 지옥 | `input action` DSL |
| **Addressables** | `LoadAssetAsync` → `Release` 생명주기 오류 빈번 | RAII-style `asset` 선언 |
| **UI Toolkit** | UXML + USS + C# 3중 파편화 | `bind uxml` DSL |
| **ECS / DOTS** | SystemBase, IJobFor, EntityQuery 보일러플레이트 | `job`, `system`, `archetype` sugar |
| **Netcode** | NetworkVariable, RPC, Spawn/Despawn 장황 | `netvar`, `rpc`, `netcomponent` |
| **에디터 툴** | `EditorWindow`, `[CustomEditor]`, `OnGUI`, `SerializedProperty` 난해 | `editor window`, `@inspector` DSL |
| **세이브/로드** | `JsonUtility` 제약, binary 수동, 폴리모피즘 불가 | `savable` 선언, 자동 snapshot |
| **테스팅** | Unity Test Runner 불편, Mock MonoBehaviour 어려움 | `test` 블록 + 자동 모킹 |
| **로컬라이제이션** | `Localization.GetLocalizedString("key")` string 키 | type-safe table |

---

## 전체 타임라인

```
v3.5.0 (현재) ─── 안정화 완료
      │
v3.6  ─── "String 박멸"              (2-3 months, 2026 Q2)
      │
v3.7  ─── "Input & Addressables"     (2-3 months, 2026 Q2-Q3)
      │
v3.8  ─── "UI Toolkit First"         (2-3 months, 2026 Q3)
      │
──────────────────────────  Unity 6.8 CoreCLR GA (2026 Q4) ──────────
      │
v4.0  ─── "ECS/DOTS 네이티브" (MAJOR) (4-6 months, 2026 Q4 - 2027 Q1)
      │
v4.1  ─── "Netcode 내장"             (3-4 months, 2027 Q1)
      │
v4.2  ─── "Editor DSL"               (3-4 months, 2027 Q2)
      │
──────────────────────────
      │
v5.0  ─── "Save / Test / Profile" (MAJOR)  (6 months, 2027 Q3-Q4)
```

**Unity 6.8 (2026 Q4) CoreCLR GA 와 v4.0 을 맞추는 것이 핵심.** 그때 "CoreCLR 이 왜 우리 DX 변화를 제일 잘 활용하는가" 를 보여줄 수 있어야 한다.

---

## v3.6 — "String 박멸"

**테마**: Unity 의 string-based API 를 모두 컴파일 타임 타입으로 승격. ROI 최고.

### 3.6.1 Type-safe Tags & Layers

```prsm
component Enemy : MonoBehaviour {
  tag "Enemy"               // compile-time, UnityEditorInternal.InternalEditorUtility.tags 로 검증
  layer "Characters"        // compile-time, LayerMask.NameToLayer 로 검증

  onTriggerEnter(other) {
    if other.tag == Tags.Player {      // ← 컴파일 타임 enum, typo 불가
      takeDamage(10)
    }
  }
}
```

**구현**:
- 파서: `tag "..."` / `layer "..."` 선언, 상수 파일 자동 생성
- 컴파일러가 Unity 프로젝트의 `TagManager.asset` 을 파싱해 `Tags` enum 생성
- 비교 연산자는 `== Tags.X` 로 강제 (string 비교 금지 warning W038)
- Diagnostic: `E213: unknown tag 'Enmey' — did you mean 'Enemy'?` (편집 거리 기반)

**영향**: Unity 프로젝트의 #1 런타임 버그 (tag typo) 완전 제거

### 3.6.2 Type-safe Scene references

```prsm
scene MainMenu = "Assets/Scenes/MainMenu.unity"
scene Gameplay = "Assets/Scenes/Gameplay.unity"

component SceneManager : MonoBehaviour {
  func startGame() {
    load scene Gameplay           // → SceneManager.LoadSceneAsync(1, ...)
    unload scene MainMenu
  }

  func reload() {
    reload scene Gameplay mode: Single
  }
}
```

**구현**:
- `scene NAME = "path"` 선언 → 컴파일 타임 검증 (파일 존재 여부)
- `load scene X [mode: Single | Additive] [async]` → 자동 `SceneManager.LoadSceneAsync` + 콜백
- Scene build index 를 `ProjectSettings/EditorBuildSettings.asset` 에서 읽어 자동 매핑

**해결되는 고통**: `SceneManager.LoadScene("Gameplay")` 오타, build index 불일치

### 3.6.3 Type-safe Animator parameters

```prsm
component Player : MonoBehaviour {
  require animator: Animator

  animator params {
    speed: Float
    isGrounded: Bool
    jumpTrigger: Trigger
    state: Int
  }

  update {
    animator.params.speed = velocity.magnitude       // type-checked
    if isGrounded {
      animator.params.isGrounded = true
    }
    if input.getKeyDown(Space) {
      animator.params.jumpTrigger.fire()             // → SetTrigger
    }
  }
}
```

**구현**:
- `animator params { ... }` 블록 → Unity `.controller` 파일 파싱해 검증
- 컴파일러가 `Animator.StringToHash("speed")` 를 상수로 인라인 (최적화)
- `Trigger.fire()` → `SetTrigger`, `Float` → `SetFloat`, `Bool` → `SetBool`, `Int` → `SetInteger`

**해결되는 고통**: 파라미터 이름 오타, `StringToHash` 수동 최적화

### 3.6.4 Type-safe Shader properties

```prsm
shader params CharacterShader {
  MainColor: Color           // _MainColor
  Glossiness: Float          // _Glossiness
  MainTexture: Texture       // _MainTex
}

component Outfit : MonoBehaviour {
  require renderer: Renderer

  func setColor(c: Color) {
    renderer.material.params<CharacterShader>.MainColor = c
  }
}
```

**구현**:
- `shader params X { ... }` → shader 파일의 property 리스트 검증
- `material.params<T>.X` → `material.SetColor("_MainColor", ...)` 로 lowering
- `Shader.PropertyToID` 를 static readonly 로 캐시

### 3.6.5 기타 string API 박멸

- `audio mixer params` — AudioMixer parameter 타입 세이프
- `sort layer` — SortingLayer.NameToID
- `input button` — `getButton("Fire1")` 을 `input.buttons.Fire1` 으로
- `@profile "Player.Update"` — Unity Profiler sample 자동 생성

### 3.6.6 CoreCLR 대비 (앞서 분리 분석한 필수 항목)

- [ ] `#if coreclr` preprocessor 심볼 추가
- [ ] `unity67`, `unity68` 버전 심볼 추가
- [ ] `.prsmproject` 에 `target_runtime = "mono" | "coreclr"` 필드 (기본 auto)
- [ ] Spec doc: Unity 런타임 호환성 매트릭스

### 3.6.7 진단 메시지 대혁신 (Rust-style diagnostics)

현재:
```
error [E020]: Type mismatch. Expected 'Int', found 'String'
```

목표:
```
error[E020]: type mismatch
  ┌─ Player.prsm:12:17
  │
 10 │   var hp: Int = "oops"
    │          ---   ^^^^^^ expected `Int`, found `String`
    │          │
    │          expected because of this annotation
    │
  = help: did you mean to parse the string?
  = help: try `var hp: Int = "oops".toInt()`
```

**구현 단위**:
- [ ] `DiagnosticLabel` 이미 AST 에 존재 — 렌더러만 Rust-style 로 업그레이드
- [ ] 관련 힌트 (`with_help`) 를 5개 이상 자주 발생하는 에러에 추가
- [ ] LSP 에도 `relatedInformation` 필드로 노출

### 3.6.8 Quick Fix / Code Action 확장

현재 `lsp.rs` 에 `collect_code_actions_json` 존재. 더 많은 액션 추가:
- [ ] `Missing component lookup` → `require X: Rigidbody` 자동 삽입
- [ ] `Listen without cleanup` → `until disable` 자동 추가
- [ ] `val → var` / `var → val` 토글
- [ ] `if cond { } else { }` → `when` 으로 변환
- [ ] `data class Stats(...)` → destructure 할당 자동 제안
- [ ] `Convert trailing lambda to parentheses` 토글
- [ ] `Missing null check` → `?.` 삽입

### 3.6.9 v3.6 성공 지표

- 예제 프로젝트 한 개 (Player Controller 풀 샘플) 에서 string literal 5개 이상이 타입 세이프 DSL 로 전환
- 새 diagnostic W038 (string-tag) + E213-E218 (5개) 등록
- 벤치마크: `Animator.SetFloat("Speed", ...)` vs `animator.params.speed = ...` → 동등 성능 (`StringToHash` 인라인 효과)
- 에러 메시지가 C# 의 3배 이상 도움이 되고, VS Code 확장에 quick fix 10+ 개 추가

### 3.6.10 바로 착수 가능한 PR 3개

**PR #1 (v3.6-0): Type-safe Tags**
- 파일: `crates/refraction/src/parser/parser.rs` (tag 선언 파싱)
- 파일: `crates/refraction/src/semantic/analyzer.rs` (tag 이름 검증)
- 파일: `crates/refraction/src/lowering/lower.rs` (`Tags.Enemy` 상수 생성)
- 파일: `docs/en/spec/standard.md` (문법 추가)
- **~400 라인 추가, 2-3일 작업**

**PR #2 (v3.6-1): Scene references**
- `scene NAME = "path"` 선언
- `load scene X [async]`, `unload scene X`
- `SceneManager.LoadSceneAsync` lowering
- **~500 라인 추가, 3-4일 작업**

**PR #3 (v3.6-2): Animator parameters**
- `animator params { }` 블록
- `.controller` 파일 파싱 (JSON)
- `animator.params.X = v` lowering
- `StringToHash` 상수 캐싱
- **~700 라인 추가, 4-5일 작업**

이 3개 PR 이 들어가면 v3.6 는 "string 박멸 얼리 액세스" 로 릴리스 가능. 나머지 string API (shader, audio mixer, sort layer) 는 v3.6.x 패치로 확장.

---

## v3.7 — "Input & Addressables"

**테마**: Unity 의 두 주요 async/resource API 를 PrSM DSL 로 래핑. 수명 주기 실수 제거.

### 3.7.1 Input System DSL (기존 sugar 확장)

```prsm
component PlayerInput : MonoBehaviour {
  input actions "Assets/Input/PlayerControls.inputactions" {
    move: Vector2             // "Player/Move"
    jump: Button              // "Player/Jump"
    look: Vector2 delta       // "Player/Look"
    attack: Button hold 0.5s  // "Player/Attack" 0.5s 이상 홀드
  }

  on input.move(v: Vector2) {
    velocity = v * speed
  }

  on input.jump.pressed {
    rigidbody.AddForce(Vector3.up * jumpForce)
  }

  on input.attack.held {
    chargeAttack()
  }
}
```

**구현**:
- `.inputactions` 파일을 컴파일 타임에 파싱 (JSON 구조)
- 액션 이름 / 컨트롤 타입 / 인터랙션을 PrSM 타입으로 검증
- `on input.X.event { }` 블록 → `InputAction.started/performed/canceled` 자동 구독
- 구독/해제는 `OnEnable`/`OnDisable` 에 자동 emit (PrSM 의 `listen until disable` 과 동일 원리)

**해결되는 고통**: Generated C# wrapper class 의 subscribe/unsubscribe 지옥, 실수로 leak 되는 이벤트 핸들러

### 3.7.2 Addressables RAII DSL

```prsm
component Enemy : MonoBehaviour {
  asset bulletPrefab: GameObject by "Prefabs/Bullet"      // Addressable key
  asset hitSound: AudioClip by label("impact")            // Addressable label
  asset portraitAtlas: SpriteAtlas lazy by "UI/Portraits" // 지연 로드

  awake {
    await load bulletPrefab      // RAII: Release on OnDestroy
    await load hitSound
  }

  func shoot() {
    val bullet = instantiate(bulletPrefab)  // 자동 unwrap
    playSound(hitSound)
  }
}
```

**구현**:
- `asset NAME: TYPE by "KEY"` → `AsyncOperationHandle<T>` 필드 생성
- `await load X` → `Addressables.LoadAssetAsync` + `await` (UniTask)
- `OnDestroy` 에 `Addressables.Release` 자동 삽입
- `lazy by` 는 첫 사용 시점까지 로드 지연
- `label("x")` 는 label 기반 다중 로드

**해결되는 고통**: Addressables handle 수동 관리, Release 누락으로 인한 메모리 leak

### 3.7.3 Resource scoping

```prsm
component Scene : MonoBehaviour {
  asset level1: GameObject by "Levels/Level1" scope disable
  asset musicTrack: AudioClip by "Music/Battle" scope persistent

  // scope disable: OnDisable 시 자동 Release
  // scope persistent: 수동 Release 까지 유지
}
```

### 3.7.4 v3.7 성공 지표

- Input System 예제 프로젝트 한 개 (FPS player controller) 풀 전환
- Addressables 전환 프로젝트에서 handle leak detection 툴 돌려 0 건
- `OnEnable`/`OnDisable` 에 주입된 subscription 코드 라인 수 측정 → C# 대비 70% 감소

---

## v3.8 — "UI Toolkit First"

**테마**: UXML + USS + C# 3중 파편화 해결. 단일 `.prsm` 파일로 UI 선언.

### 3.8.1 Inline UXML binding

```prsm
component HUD : MonoBehaviour {
  require uiDocument: UIDocument

  bind ui "HUD.uxml" {
    "hp-label": Label       // query → compile-time
    "mp-bar": ProgressBar
    "attack-button": Button
  }

  bind var hp: Int = 100
  bind var mp: Float = 50.0

  awake {
    // 자동 양방향 바인딩
    ui["hp-label"].text <- "HP: ${hp}"
    ui["mp-bar"].value <- mp

    ui["attack-button"].onClick {
      log("attack")
    }
  }
}
```

**구현**:
- `bind ui "X.uxml" { ... }` 블록 → UXML 파일을 파싱해 `Query<T>` 결과를 타입 세이프 필드로 제공
- `<-` (bind arrow) → `PropertyChanged` 이벤트 기반 양방향 바인딩
- `ui["hp-label"].onClick { }` → callback 등록 + `OnDisable` 자동 해제

### 3.8.2 USS class toggle DSL

```prsm
component Button : MonoBehaviour {
  bind ui "Button.uxml" {
    "root": VisualElement
  }

  bind var disabled: Bool = false

  awake {
    ui["root"].classes.disabled <- disabled   // toggle USS class "disabled"
  }
}
```

### 3.8.3 Inline USS (experimental)

```prsm
component Card : MonoBehaviour {
  bind ui "Card.uxml" with styles {
    .card { background-color: #222; padding: 10px; }
    .card:hover { background-color: #333; }
  }
}
```

파서가 CSS 서브셋을 인식해 USS 파일을 자동 생성.

### 3.8.4 v3.8 성공 지표

- UI Toolkit 샘플 (Settings menu) 을 PrSM 으로 재작성, C# + UXML + USS 3파일 → `.prsm` 1파일
- 라인 수 절반 이하

---

## v4.0 — "ECS/DOTS 네이티브" (MAJOR)

**테마**: Unity ECS 가 지금까지 가장 장황한 C# API. 여기서 PrSM 이 압도적 우위 확보.

### 4.0.1 IComponentData 선언

```prsm
// Unity.Entities 자동 import
component struct Position : IComponentData {
  value: Float3
}

component struct Velocity : IComponentData {
  value: Float3
}

component struct Health : IComponentData {
  current: Int
  max: Int
}
```

**구현**:
- `component struct X : IComponentData` → 이미 `ref struct` lowering 경로가 있으므로 확장
- `Float3`, `Quaternion`, etc. → `Unity.Mathematics.float3`, `quaternion` 자동 매핑

### 4.0.2 Archetype 선언

```prsm
archetype PlayerArch = [Position, Velocity, Health, PlayerTag]
archetype EnemyArch = [Position, Velocity, Health, AIState]
```

→ 컴파일 타임에 `EntityArchetype` 정적 캐시 생성.

### 4.0.3 Job DSL

```prsm
@burst
job MoveJob : IJobFor {
  read positions: NativeArray<Position>
  write velocities: NativeArray<Velocity>
  param deltaTime: Float

  execute(i: Int) {
    velocities[i].value += positions[i].value * deltaTime
  }
}

system MovementSystem : SystemBase {
  query players = [read Position, write Velocity]

  onUpdate {
    schedule MoveJob {
      positions: players.positions
      velocities: players.velocities
      deltaTime: Time.deltaTime
    }
  }
}
```

**구현**:
- `@burst` 는 이미 있음 — burst analyzer 가 E137/E138/E139 이미 검증
- `job X : IJobFor { read/write/param }` → struct + `[ReadOnly]`/`[WriteOnly]` 자동
- `system X : SystemBase { query, onUpdate, schedule }` → SystemBase 보일러플레이트 전부 생성
- `query` 선언 → `EntityQuery` + `GetComponentTypeHandle` 자동 생성

### 4.0.4 Entity command buffer

```prsm
system SpawnerSystem : SystemBase {
  onUpdate {
    withECB { ecb =>
      for i in 0 until 10 {
        val e = ecb.create()
        ecb.add<Position>(e, { value: Float3(0, 0, 0) })
        ecb.add<Velocity>(e, { value: Float3(1, 0, 0) })
      }
    }
  }
}
```

**구현**:
- `withECB { ecb => ... }` → `EntityCommandBuffer.ParallelWriter` 자동 생성 + Playback 시점 자동
- `ecb.add<T>(e, { ... })` → `ecb.AddComponent(entity, new T { ... })`

### 4.0.5 v4.0 성공 지표

- DOTS 샘플 프로젝트 (5000 엔티티 시뮬레이션) 를 C# 대비 40% 적은 코드로 재작성
- Unity Burst Playground 샘플 의 "Boids" 데모 를 PrSM 으로 포팅 — 벤치마크 동등
- `prism build` 가 ECS 프로젝트를 ~10초 이내 (incremental cache 적용) 에 컴파일

---

## v4.1 — "Netcode 내장"

**테마**: Unity Netcode for GameObjects + Netcode for Entities 의 RPC / NetworkVariable 사탕화.

### 4.1.1 NetworkBehaviour + NetworkVariable

```prsm
component Player : NetworkBehaviour {
  netvar hp: Int = 100 {
    readPerm: Everyone
    writePerm: Owner
  }

  netvar position: Vector3 {
    writePerm: Server
  }

  rpc attack(target: NetworkObject) on Server {
    val dmg = calculateDamage(target)
    target.GetComponent<Player>().hp -= dmg
  }

  rpc playSound(id: Int) on Client {
    AudioSource.Play(soundBank[id])
  }

  on hp.changed(old: Int, new: Int) {
    if new <= 0 {
      rpc playSound(deathSound)
    }
  }
}
```

**구현**:
- `netvar X: T { readPerm, writePerm }` → `NetworkVariable<T>` 필드 + writer 래퍼
- `rpc X() on Server` → `[ServerRpc]` attribute + RPC method
- `rpc X() on Client` → `[ClientRpc]`
- `on netvar.changed(old, new) { }` → `OnValueChanged` 자동 구독

### 4.1.2 Network spawn/despawn

```prsm
component BulletSpawner : NetworkBehaviour {
  asset bulletPrefab: GameObject by "Prefabs/Bullet"

  rpc fire(origin: Vector3, dir: Vector3) on Server {
    val bullet = network spawn(bulletPrefab, at: origin)
    bullet.GetComponent<Bullet>().velocity = dir * 10
  }
}
```

**구현**:
- `network spawn(X, at: ...)` → `Instantiate` + `GetComponent<NetworkObject>().Spawn()`
- 자동 생명주기 관리

### 4.1.3 v4.1 성공 지표

- Unity Netcode 샘플 (Multiplayer Shooter) 을 PrSM 으로 재작성 → 코드 라인 50% 감소
- RPC 정의/호출의 타입 안전성 100% (C# 은 attribute 기반이라 컴파일 타임 일부만 검증됨)

---

## v4.2 — "Editor DSL"

**테마**: 에디터 툴 작성이 C# 에서 가장 고통스러운 영역 중 하나.

### 4.2.1 Inspector DSL

```prsm
component PlayerStats : MonoBehaviour {
  serialize var hp: Int = 100
  serialize var speed: Float = 5.0
  serialize var weapon: Weapon?

  @inspector {
    section "Stats" expanded: true {
      slider hp range: 0..1000
      slider speed range: 0.0..20.0 step: 0.1
    }
    section "Equipment" {
      field weapon
      button "Equip Default" => equipDefault()
    }
    divider()
    readonly label "DPS: ${hp * speed}"
  }

  func equipDefault() { weapon = Weapon.Basic }
}
```

**구현**:
- `@inspector { }` 블록 → 자동 `Editor` 클래스 + `OnInspectorGUI` 생성
- `slider`, `field`, `button`, `section`, `divider`, `readonly label` DSL 요소
- `"DPS: ${...}"` 는 매 Repaint 시 재계산

### 4.2.2 Editor window DSL

```prsm
editor window LevelEditor : EditorWindow menu "Tools/Level Editor" {
  var selectedTile: Tile?
  var grid: Array<Tile>

  render {
    layout horizontal {
      layout vertical width: 200 {
        label "Tiles"
        for tile in availableTiles {
          button tile.name => selectedTile = tile
        }
      }
      layout vertical {
        label "Grid"
        gridView grid onClick: { pos => paint(pos) }
      }
    }
  }

  func paint(pos: Vector2Int) {
    if let tile = selectedTile {
      grid[pos.x, pos.y] = tile
    }
  }
}
```

**구현**:
- `editor window X : EditorWindow menu "..."` → `[MenuItem]` + `EditorWindow.GetWindow`
- `render { }` → `OnGUI` 재작성
- `layout horizontal/vertical`, `gridView` 등의 primitive

### 4.2.3 Custom gizmo DSL

```prsm
component Trigger : MonoBehaviour {
  serialize var radius: Float = 5.0

  @gizmo selected {
    color: Color.yellow
    sphere center: transform.position radius: radius
    line from: transform.position to: transform.position + Vector3.up * 2
  }
}
```

`OnDrawGizmos` / `OnDrawGizmosSelected` 자동 생성.

### 4.2.4 v4.2 성공 지표

- Odin Inspector 수준의 인스펙터 DSL (slider, section, expandable, ...)
- 샘플 에디터 툴 (Tile map editor) 을 200 라인 C# → 50 라인 PrSM 으로

---

## v5.0 — "Save / Test / Profile" (MAJOR)

**테마**: 게임 개발의 "그라운드 트루스" 작업 3개를 퍼스트클래스로.

### 5.0.1 Savable properties

```prsm
component Player : MonoBehaviour {
  savable var level: Int = 1
  savable var xp: Long = 0
  savable var inventory: List<ItemStack>
  savable var lastScene: SceneRef

  // 자동 생성: SaveData, LoadFrom, SaveTo 메서드
  // 폴리모피즘 지원 (Item 이 sealed class 계층일 때도 동작)
}
```

**구현**:
- `savable` 프로퍼티는 자동 serialize 리스트에 등록
- 내부 포맷: MessagePack 또는 JSON (사용자 선택)
- 버전 마이그레이션 지원 (`@savable(version: 2)`)

### 5.0.2 Test DSL

```prsm
test PlayerTest {
  @mock weapon: Weapon = Weapon.Sword

  test "player takes damage" {
    val player = spawn Player()
    player.takeDamage(10)
    assert player.hp == 90
  }

  test "player dies at zero hp" {
    val player = spawn Player() with { hp: 5 }
    player.takeDamage(10)
    assert player.isDead
  }

  test "async: heal over time" async {
    val player = spawn Player() with { hp: 50 }
    player.startRegen()
    wait 2.0s
    assert player.hp > 50
  }
}
```

**구현**:
- `test X { }` 블록 → Unity Test Runner `[TestFixture]` + `[Test]` 자동 생성
- `@mock X = ...` → `require` 필드 자동 주입 (의존성 주입)
- `spawn X() with { ... }` → `new GameObject().AddComponent<X>()` + 필드 초기화
- `test "..." async` → `[UnityTest]` + `IEnumerator` 자동

### 5.0.3 Profile annotations

```prsm
component Enemy : MonoBehaviour {
  @profile("Enemy.Update", thresholdMs: 0.5)
  update {
    // Unity Profiler sample 자동 삽입
    // 0.5ms 초과 시 warning W040
  }

  @profile track: allocations
  func heavyWork() {
    // Allocation tracker 자동 삽입
  }
}
```

**구현**:
- `@profile` → `Unity.Profiling.ProfilerMarker` 자동 생성
- `threshold` 초과 시 런타임 경고 emit (debug 빌드만)
- `track: allocations` → `GC.CollectionCount` delta 감시

---

## 상시 투자 트랙 (모든 버전 병행)

로드맵과 별개로 매 마이너 릴리스마다 계속 진행:

### Roslyn sidecar 강화
- 현재 sidecar 는 정적 인덱싱 + hover/completion 기초
- Unity API 변경 감지 (버전별 API 차이) 자동 반영
- `sourceGenerator` 호환 (예: Unity.Mathematics 의 swizzle operator)

### LSP 기능 확장
- Call hierarchy (`textDocument/callHierarchy`)
- Inlay hints (`textDocument/inlayHint`) — 추론된 타입 표시
- Semantic tokens (`textDocument/semanticTokens`)
- Document link (`textDocument/documentLink`) — UXML/USS 파일 연결
- Code lens (`textDocument/codeLens`) — 참조 수 표시

### 성능 / incremental
- v3.5 의 project-wide hash 무효화는 coarse. v4.x 에서 HIR 의존성 그래프 기반 세분화
- Parallel file compilation (rayon)
- Cached lex/parse/semantic 상태 디스크 저장

### Diagnostics 품질
- Error messages with Rust-style `^^^^` underlines
- "Did you mean ..." 편집 거리 제안 (모든 식별자에)
- Quick fixes in LSP code actions
- Multi-file error chains (A depends on B which has error)

### 포매터 (초기 버전)
- `prism fmt file.prsm` CLI 커맨드
- Prettier 스타일 규칙 (2 space, trailing comma, space around operators)
- VS Code "Format on Save" 통합
- EditorConfig 지원

### 회귀 테스트 자동 생성
- `prism snapshot compile foo.prsm` → `.snapshot.cs` 생성
- CI 에서 snapshot 비교로 lowering 변경 감지

### Docs & samples
- 매 마이너 버전마다 2-3 개 완전 동작하는 Unity 샘플 프로젝트 공개
- "Migrating from C#" 가이드 (C# ↔ PrSM 비교)
- 비디오 튜토리얼 (선택적)

---

## CoreCLR 대비 사전 작업 (v3.6 포함)

Unity 의 2026 CoreCLR 전환에 맞춰 사전 정렬:

| 우선순위 | 작업 | 파일 수 |
|----------|------|---------|
| HIGH | `#if coreclr` preprocessor 심볼 추가 | 3 (lower/analyzer/docs) |
| HIGH | `unity67`, `unity68` 심볼 추가 | 3 |
| MEDIUM | Spec 문서 런타임 호환 매트릭스 업데이트 | 1 |
| MEDIUM | `package.json` minimum Unity 버전 정책 결정 | 1 |
| LOW | Unity 6.7 experimental CoreCLR 에서 bridge 스모크 테스트 | N/A |
| LOW | Optimizer 의 `#if coreclr` 가드 | 1 |

**근거**: Unity CoreCLR 2026 상세 분석은 별도 분석 결과 참조. PrSM 은 이미 `.NET Standard 2.1` 타겟이라 구조적 리팩토링은 불필요. preprocessor 심볼과 메타 정보만 정렬하면 됨.

---

## 한 문장 요약

> **"Unity 에서만큼은 C# 이 부끄러워지게 만든다."** — 각 릴리스는 Unity 개발자의 특정 pain point 하나를 완전히 제거한다. 범용성은 버리고, 깊이를 판다.

---

## Appendix A — 매 버전 공통 체크리스트

각 릴리스마다 반드시 완수해야 할 항목:

- [ ] 신규 문법 `docs/en/spec/` 에 추가 (lang-6.md / lang-7.md / ...)
- [ ] 신규 에러 / 워닝 코드 `docs/en/error-catalog.md` 에 등록
- [ ] 회귀 테스트 최소 5개 (lib + integration)
- [ ] VS Code 확장 syntax highlighting / snippet / quick fix 업데이트
- [ ] `scripts/bump-version.sh` 로 4 개 컴포넌트 동기화
- [ ] `cargo test -p refraction` 전체 통과
- [ ] VS Code 확장 테스트 전체 통과
- [ ] `vscode-prsm/bin/prism.exe` 번들 바이너리 갱신
- [ ] GitHub release 노트 작성
- [ ] 샘플 프로젝트 최소 1개 신규 또는 업데이트

## Appendix B — 릴리스 히스토리 (v3.x 누적)

| 릴리스 | 주요 성과 | 이슈 범위 | 건수 |
|--------|-----------|-----------|------|
| v3.0.0 | lang-4 완성, 16 개 lang-4 이슈 해결 | #1-#16 | 16 |
| v3.1.0 | expanded audit round | #17-#32 | 17 + 1 |
| v3.2.0 | 16 audit issues | #33-#48 | 16 |
| v3.3.0 | 32 audit issues (parser + semantic + cross-component) | #49-#81 | 32 |
| v3.4.0 | 10 audit issues + v3.3.0 regression hotfix | #82-#91 | 10 |
| v3.5.0 | 9 audit issues (optimizer / sugar blocks / HIR / docs) | #92-#100 | 9 |
| **누적** | | | **100+** |
