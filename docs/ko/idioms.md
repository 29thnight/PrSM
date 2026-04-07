---
title: Idioms & Patterns
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 14
---

# 관용구 & 패턴

이 페이지는 PrSM 개발에서 권장되는 패턴과 흔한 안티패턴을 모았습니다. 각 섹션에는 짧은 코드 예제가 포함되어 있습니다.

## 컴포넌트 설계

컴포넌트는 하나의 책임에 집중하세요. 런타임에 반드시 존재해야 하는 의존성에는 `require`를, 있을 수도 없을 수도 있는 의존성에는 `optional`을 사용합니다.

```prsm
component DamageReceiver : MonoBehaviour {
    require collider: Collider
    optional animator: Animator

    serialize maxHp: Int = 100
    var hp: Int = maxHp

    func takeDamage(amount: Int) {
        hp = Math.Max(0, hp - amount)
        animator?.SetTrigger("Hit")
        if hp <= 0 {
            die()
        }
    }
}
```

이동, 입력, 체력, UI, 오디오를 하나의 선언에서 처리하는 "만능 컴포넌트"는 피하세요. 별도의 컴포넌트로 분리하고 이벤트나 공유 `asset` 데이터를 통해 통신합니다.

### 싱글톤 패턴 (PrSM 3 부터)

싱글톤 패턴을 직접 구현하는 대신 `singleton component`를 사용하세요:

```prsm
// 권장 — 키워드 하나로 해결
singleton component GameManager : MonoBehaviour {
    var score: Int = 0
}

// 안티패턴 — 수동 싱글톤 보일러플레이트
component GameManager : MonoBehaviour {
    // 이렇게 하지 마세요 — 대신 singleton 키워드를 사용하세요
    // private static instance, Awake 검사, DontDestroyOnLoad...
}
```

### 오브젝트 풀링 (PrSM 3 부터)

수동 풀 관리 대신 `pool` 수정자를 사용하세요:

```prsm
// 권장 — 선언적 풀
component Spawner : MonoBehaviour {
    serialize prefab: Bullet
    pool bullets: Bullet(capacity = 20, max = 100)

    func fire() {
        val bullet = bullets.get()
        bullet.launch(direction)
    }
}
```

## 이벤트 구독 패턴

### 권장 — `until disable`로 자동 정리

v2 컴포넌트에서 가장 안전한 패턴입니다. 컴포넌트가 비활성화되면 리스너가 자동으로 제거되어, 파괴된 오브젝트에서 오래된 콜백이 호출되는 것을 방지합니다.

```prsm
component ShopUI : MonoBehaviour {
    require buyButton: Button
    require sellButton: Button

    listen buyButton.onClick until disable {
        purchaseSelectedItem()
    }

    listen sellButton.onClick until disable {
        sellSelectedItem()
    }
}
```

### 임시 리스너 — 수동 수명

제한된 시간 동안만 활성화해야 하는 리스너는 토큰을 캡처하고 완료 시 unlisten합니다.

```prsm
component Tutorial : MonoBehaviour {
    require skipButton: Button

    var skipToken: ListenToken? = null

    func startTutorial() {
        skipToken = listen skipButton.onClick manual {
            endTutorial()
        }
    }

    func endTutorial() {
        if skipToken != null {
            unlisten skipToken!!
            skipToken = null
        }
    }
}
```

### 안티패턴 — v2 컴포넌트에서 등록 전용

v2에서 `auto-unlisten`이 활성화된 경우, 수명 수정자 없는 `listen`은 기본적으로 `until disable`로 동작합니다. 의도적으로 등록 전용 시맨틱이 필요하다면 `manual`을 명시하고 이유를 문서화하세요.

## 코루틴 패턴

### 시간 기반 동작

타이머를 수동으로 추적하는 대신 `wait`에 지속 시간을 사용합니다.

```prsm
coroutine flashDamage() {
    spriteRenderer.color = Color.red
    wait 0.15s
    spriteRenderer.color = Color.white
}
```

### `wait until`을 사용한 폴링

수동 `Update` 기반 검사를 `wait until`로 대체하면 의도가 더 명확해집니다.

```prsm
coroutine waitForDoorOpen() {
    wait until door.isOpen
    playOpenAnimation()
}
```

### 다단계 시퀀스

`wait` 구문을 연결하여 스크립트 시퀀스를 만듭니다. 각 단계가 위에서 아래로 읽힙니다.

```prsm
coroutine introSequence() {
    fadeIn(title)
    wait 2.0s
    fadeOut(title)
    wait 0.5s
    fadeIn(subtitle)
    wait 3.0s
    fadeOut(subtitle)
    loadGameplay()
}
```

### 안티패턴 — wait 없는 무한 루프

yield 없이 루프를 도는 코루틴은 Unity를 멈추게 합니다. 루프 본문 안에 항상 `wait`를 포함하세요.

```prsm
// 나쁜 예 — Unity 멈춤
coroutine badLoop() {
    while true {
        checkSomething()
    }
}

// 좋은 예 — 매 프레임 yield
coroutine goodLoop() {
    while true {
        checkSomething()
        wait nextFrame
    }
}
```

## null 안전 패턴

### 존재하지 않을 수 있는 컴포넌트에 `optional`과 `?.` 사용

```prsm
component Interactable : MonoBehaviour {
    optional outline: OutlineEffect

    func highlight() {
        outline?.Enable()
    }

    func unhighlight() {
        outline?.Disable()
    }
}
```

### `?:`로 기본값 제공

엘비스 연산자는 값이 null일 수 있을 때 간결한 기본값을 제공합니다.

```prsm
func getDisplayName(): String {
    return player.customName ?: "Unknown Player"
}
```

### `!!` 사용 자제 — `require` 선호

not-null 단언 `!!`은 값이 null이면 런타임에 예외를 던집니다. 컴포넌트에서는 게임플레이 도중이 아닌 초기화 시점에 누락된 참조를 잡을 수 있도록 `require`를 선호하세요.

```prsm
// 위험 — 누락 시 런타임 실패
optional rb: Rigidbody
func move() {
    rb!!.MovePosition(target)   // 누락 시 NullReferenceException
}

// 안전 — Awake에서 명확한 메시지와 함께 즉시 실패
require rb: Rigidbody
func move() {
    rb.MovePosition(target)
}
```

## 데이터 모델링

### 값 타입에 `data class` 사용

데이터를 담는 구조체에는 `data class`를 사용합니다. 컴파일러가 동등성, 해싱, 문자열 표현을 생성합니다.

```prsm
data class DamageInfo {
    amount: Int
    source: GameObject
    damageType: DamageType
}

data class SpawnConfig {
    prefab: GameObject
    position: Vector3
    rotation: Quaternion = Quaternion.identity
}
```

### 유한 상태에 `enum` 사용

단순한 상태 머신에는 일반 enum이 적합합니다.

```prsm
enum EnemyState {
    Idle,
    Chase,
    Attack,
    Flee
}
```

### 데이터를 가진 상태에 매개변수화된 `enum` 사용

상태에 연관 데이터가 있을 때는 매개변수화된 배리언트를 사용합니다.

```prsm
enum AICommand {
    MoveTo(target: Vector3),
    Attack(enemy: GameObject),
    Wait(duration: Float),
    Patrol(waypoints: List<Vector3>)
}
```

## intrinsic 탈출구

`intrinsic`은 PrSM이 아직 지원하지 않는 Unity API에 대해 원시 C#로 직접 작성할 수 있게 합니다. 절제하여 사용하세요.

### 재사용성을 위해 `intrinsic func` 선호

탈출구를 이름 있는 함수로 감싸면 호출 지점이 깔끔하게 유지됩니다.

```prsm
intrinsic func setLayerRecursive(obj: GameObject, layer: Int) {
    obj.layer = layer;
    foreach (Transform child in obj.transform) {
        SetLayerRecursive(child.gameObject, layer);
    }
}
```

### 인라인 `intrinsic {}` 블록은 짧게 유지

인라인 블록을 사용해야 한다면 몇 줄로 제한하세요. 큰 intrinsic 블록은 PrSM을 작성하는 목적을 훼손합니다.

```prsm
func captureScreenshot() {
    val path = "Screenshots/" + System.DateTime.Now.ToString("yyyyMMdd_HHmmss") + ".png"
    intrinsic {
        ScreenCapture.CaptureScreenshot(path);
    }
}
```

### intrinsic을 사용해야 할 때

- PrSM이 래핑하지 않는 Unity API (예: 저수준 렌더링, 네이티브 플러그인)
- 정확한 C# 제어가 필요한 성능 중요 내부 루프
- 특정 C# 패턴이 필요한 서드파티 라이브러리 연동

### intrinsic을 사용하지 말아야 할 때

- 표준 Unity 라이프사이클 — `awake`, `start`, `update`, `onDestroy` 블록 사용
- 이벤트 연결 — `listen` 사용
- 코루틴 — `coroutine`과 `wait` 사용
- 입력 — `on input` 사용 (PrSM 2 부터)

큰 `intrinsic` 블록을 자주 작성하게 된다면, 해당 패턴을 네이티브로 지원할 수 있도록 기능 요청을 제출하는 것을 고려하세요.

### Interface 주도 설계 (PrSM 3 부터)

컴포넌트 계약을 위한 interface를 정의하세요:

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

느슨한 결합을 위해 `require`와 함께 interface를 사용하세요: `require target: Enemy` 대신 `require target: IDamageable`을 사용합니다.

## PrSM 5 관용구 (PrSM 5 부터)

### 스트리밍 시퀀스에서 `intrinsic`을 `yield`로 교체

PrSM 5 이전에는 코루틴 안에서 값 시퀀스를 생성하려면 `intrinsic`으로 떨어져야 하는 경우가 많았습니다. 일반 `yield`로 같은 로직을 PrSM 안에 유지할 수 있습니다:

```prsm
coroutine spawnPositions(): Seq<Vector3> {
    for x in 0..10 {
        for z in 0..10 {
            yield vec3(x, 0, z)
        }
    }
}
```

### 인스펙터 노출에 자동 프로퍼티의 `serialize` 사용

명시적 `get`/`set` 액세서가 있는 `var`에 `serialize` 한정자가 붙으면 `[field: SerializeField]`로 변환됩니다. 이는 공개 표면을 프로퍼티로 유지하면서 백킹 필드를 직렬화하는 Unity 표준 패턴으로, 이전에는 `intrinsic` 또는 수동 페어링된 private 필드가 필요했습니다.

```prsm
component Player : MonoBehaviour {
    serialize var hp: Int = 100
        get
        set { field = Mathf.clamp(value, 0, maxHp) }
}
```

### 에디터 전용 코드는 `#if editor`로 보호

에디터 전용 디버그 헬퍼와 기즈모 드로잉은 `intrinsic { #if UNITY_EDITOR ... #endif }` 대신 `#if editor` 블록 안에 두는 것이 좋습니다.

```prsm
update {
    move()

    #if editor
        drawDebugGizmos()
    #endif
}
```

### `Physics.Raycast` 류 API에서 `out val` 선호

네이티브 `out` 매개변수 지원은 가장 흔한 Unity API 호출 사이트에서 `intrinsic` 블록의 필요를 제거합니다.

```prsm
if physics.raycast(ray, out val hit) {
    log("hit ${hit.collider.name}")
}
```

### 관계/결합자 패턴으로 `if` 체인 평탄화

관계 패턴과 `and`/`or`/`not` 결합자는 중첩된 `if` 체인을 단일 `when` 표현식으로 변환합니다:

```prsm
val tier = when hp {
    > 80 => "Healthy"
    > 30 and < 80 => "Hurt"
    > 0 => "Critical"
    else => "Dead"
}
```

### 큰 컴포넌트는 `partial`로 분할

게임플레이 상태, 전투 동작, UI 바인딩이 섞인 컴포넌트는 단일 파일에 두기에 너무 커지는 경우가 많습니다. `partial component`를 사용하면 파일당 단일 선언 규칙을 깨지 않고 동일 컴포넌트를 여러 파일에 걸쳐 작성할 수 있습니다.

```prsm
// Player.movement.prsm
partial component Player : MonoBehaviour {
    serialize speed: Float = 5.0
    update { move() }
}

// Player.combat.prsm
partial component Player {
    bind hp: Int = 100
    func takeDamage(amount: Int) { hp -= amount }
}
```

### 가변 struct 복사 대신 `with` 사용

Unity struct 타입의 경우, `with` 표현식이 "복사하고 한 필드만 수정" 패턴을 수동 임시 변수 없이 캡처합니다:

```prsm
val grounded = transform.position with { y = 0.0 }
```

### 프로퍼티 알림에 문자열 리터럴 대신 `nameof` 사용

`nameof` 연산자는 컴파일 타임에 오타를 잡아냅니다. `bind`와 결합하면 컴파일러가 `OnPropertyChanged(nameof(hp))`를 자동으로 연결하지만, 멤버를 명명하는 모든 수동 이벤트에는 여전히 `nameof`가 적합합니다.

```prsm
event onPropertyChanged: (String) => Unit
onPropertyChanged.invoke(nameof(hp))
```

### Burst 대상은 명명 대신 `@burst`로 어노테이트

언어 5는 E137–E139와 W028을 `burst_*` 명명 휴리스틱 대신 `@burst` 어노테이션을 통해 라우팅합니다. 명시적으로 어노테이트하여 옵트인하세요:

```prsm
@burst
func calculateForces(positions: NativeArray<Float3>, forces: NativeArray<Float3>) {
    for i in 0..positions.length {
        forces[i] = computeGravity(positions[i])
    }
}
```
