---
title: PrSM 5
parent: 사양
nav_order: 7
---

# PrSM 언어 5

PrSM 5는 PrSM과 C# 사이에 남아 있던 Unity 관련 갭을 마지막으로 메우고, 언어 4에서 알려진 제약을 해소합니다. **22개의 새 문법 기능**을 6개 구현 스프린트로 묶고, 추가로 언어 4 기능에 대한 **12개의 제약 수정**을 포함합니다. 이 릴리스는 **Prism v3.0.0**으로 출시됩니다. 모든 언어 4 프로그램은 변경 없이 그대로 컴파일됩니다.

**활성화:** `.prsmproject`에서 `language.version = "5"` 설정

## Part I — 고임팩트 문법

### 일반 `yield return`

PrSM 코루틴은 이제 기존 `wait` 단축 외에도 임의의 `yield`와 `yield break` 문장을 받아들입니다. 이를 사용하는 함수는 `coroutine` 선언이거나, 반환 타입이 `Seq<T>`, `IEnumerator`, `IEnumerator<T>`, `IEnumerable`, `IEnumerable<T>` 중 하나인 `func`여야 합니다.

```prsm
component Cutscene : MonoBehaviour {
    coroutine countdown(): Seq<Int> {
        for i in 5 downTo 1 {
            yield i
            wait 1s
        }
        yield 0
        yield break
    }

    coroutine fadeOut(): IEnumerator {
        var t = 1.0
        while t > 0.0 {
            t -= Time.deltaTime
            canvasGroup.alpha = t.toFloat()
            yield return null
        }
    }
}
```

```csharp
public IEnumerator<int> countdown()
{
    for (int i = 5; i >= 1; i--)
    {
        yield return i;
        yield return new WaitForSeconds(1.0f);
    }
    yield return 0;
    yield break;
}

public IEnumerator fadeOut()
{
    var t = 1.0;
    while (t > 0.0)
    {
        t -= Time.deltaTime;
        canvasGroup.alpha = (float)t;
        yield return null;
    }
}
```

코루틴이나 이터레이터 반환 함수 밖에서 `yield`를 사용하면 E147이 발생합니다. `yield` 값의 타입이 선언된 요소 타입과 다르면 E148이 발생합니다. `Seq<T>` 코루틴이 어떤 `T` 값도 yield하지 않으면 W033이 발생합니다.

### 프로퍼티의 어트리뷰트 타깃

자동 프로퍼티에 `serialize` 한정자가 붙으면 이제 `[field: SerializeField]` 백킹 필드로 변환됩니다. 이는 자동 프로퍼티를 인스펙터에 노출하면서 공개 표면은 프로퍼티로 유지하는 Unity 표준 패턴입니다. 일반 형식인 `@field(name)`, `@property(name)`, `@param(name)`, `@return(name)`, `@type(name)`을 사용하면 임의 선언의 비기본 타깃에 임의의 C# 어트리뷰트를 부착할 수 있습니다.

```prsm
component Player : MonoBehaviour {
    serialize var hp: Int = 100
        get
        set { field = Mathf.clamp(value, 0, maxHp) }

    @field(nonSerialized)
    var transientCache: Map<String, Int>

    @return(notNull)
    func getTarget(): Transform = currentTarget
}
```

```csharp
[field: SerializeField]
public int hp
{
    get;
    set { field = Mathf.Clamp(value, 0, maxHp); }
}

[field: NonSerialized]
public Dictionary<string, int> transientCache { get; set; }

[return: NotNull]
public Transform getTarget() => currentTarget;
```

자동 프로퍼티의 `serialize`는 `@field(serializeField)`의 편의 표기입니다. 선택한 선언이 어트리뷰트 타깃을 지원하지 않으면 E149, 자동 프로퍼티로 변환할 수 없는 액세서를 가진 프로퍼티에 `serialize`가 붙으면 E150이 발생합니다.

### 전처리 디렉티브

`#if` / `#elif` / `#else` / `#endif` 디렉티브를 이제 모든 statement, member, top-level 위치에서 사용할 수 있습니다. PrSM은 자주 쓰이는 플랫폼 심볼 집합을 정의하여 대응하는 `UNITY_*` 정의로 변환합니다. 그 외 식별자는 그대로 통과됩니다.

```prsm
component Player : MonoBehaviour {
    update {
        move()

        #if editor
            drawDebugGizmos()
        #endif

        #if ios && !editor
            handleHaptics()
        #elif android
            handleVibration()
        #endif
    }

    #if debug
        func logState() { log("hp=$hp, pos=${transform.position}") }
    #endif
}
```

```csharp
public class Player : MonoBehaviour
{
    void Update()
    {
        move();

        #if UNITY_EDITOR
            drawDebugGizmos();
        #endif

        #if UNITY_IOS && !UNITY_EDITOR
            handleHaptics();
        #elif UNITY_ANDROID
            handleVibration();
        #endif
    }

    #if DEBUG
        public void logState() { Debug.Log($"hp={hp}, pos={transform.position}"); }
    #endif
}
```

규범적 심볼 매핑은 다음과 같습니다:

| PrSM 심볼 | C# 정의 |
|-----------|---------|
| `editor` | `UNITY_EDITOR` |
| `debug` | `DEBUG` |
| `release` | `!DEBUG` |
| `ios` | `UNITY_IOS` |
| `android` | `UNITY_ANDROID` |
| `standalone` | `UNITY_STANDALONE` |
| `il2cpp` | `ENABLE_IL2CPP` |
| `mono` | `ENABLE_MONO` |
| `unity20223` | `UNITY_2022_3_OR_NEWER` |
| `unity20231` | `UNITY_2023_1_OR_NEWER` |
| `unity6` | `UNITY_6000_0_OR_NEWER` |

종료되지 않은 `#if` 블록은 E151, 대응하는 `#if`가 없는 `#elif`/`#else`는 E152, 알려지지 않은 심볼은 그대로 통과되며 W034가 발생합니다.

## Part II — 일반적인 Unity API 요구사항

### `ref` / `out` 매개변수

`ref` 매개변수는 메서드가 호출자의 변수를 직접 수정할 수 있게 합니다. `out` 매개변수는 callee가 반환 전에 값을 할당하도록 요구합니다. 호출 사이트는 선언 표현식 형식으로 `out val name`을 사용합니다 (C#의 `out var name` 형식에 해당).

```prsm
func tryParse(input: String, out value: Int): Bool {
    intrinsic { return int.TryParse(input, out value); }
}

if physics.raycast(ray, out val hit) {
    log("hit ${hit.collider.name}")
}

if physics.raycast(ray, out _) {
    log("something hit")
}
```

```csharp
public bool tryParse(string input, out int value)
{
    return int.TryParse(input, out value);
}

if (Physics.Raycast(ray, out var hit))
{
    Debug.Log($"hit {hit.collider.name}");
}

if (Physics.Raycast(ray, out _))
{
    Debug.Log("something hit");
}
```

`ref`/`out` 인자가 매개변수 한정자와 일치하지 않으면 E153, `out` 매개변수가 모든 반환 경로 전에 할당되지 않으면 E154, `ref` 매개변수에 불변 `val`을 전달하면 E155가 발생합니다.

### `vararg` (params) 매개변수

`vararg` 한정자가 붙은 매개변수는 선언된 타입의 인자를 0개 이상 받습니다. **마지막** 매개변수에만 `vararg`를 사용할 수 있습니다. 함수 본문에서는 `Array<T>`로 보이며, C#의 `params T[]`로 변환됩니다.

```prsm
func log(vararg messages: String) {
    for msg in messages {
        Debug.Log(msg)
    }
}

log()
log("loading")
log("step 1", "step 2", "step 3")
```

```csharp
public void log(params string[] messages)
{
    foreach (var msg in messages)
    {
        Debug.Log(msg);
    }
}
```

마지막이 아닌 매개변수에 `vararg`를 사용하면 E156, 단일 함수에 `vararg` 매개변수가 둘 이상이면 E157이 발생합니다.

### 매개변수 기본값

매개변수 선언에 `= expr`을 추가해 기본값을 지정할 수 있습니다. 기본값 식은 컴파일 타임 상수(literal, `null`, `default`, `const` 참조)여야 합니다. 기본값을 가진 매개변수는 모두 필수 매개변수 뒤에 와야 합니다.

```prsm
func instantiate(prefab: GameObject, parent: Transform? = null, worldSpace: Bool = false): GameObject {
    return GameObject.Instantiate(prefab, parent, worldSpace)
}

instantiate(bulletPrefab)
instantiate(bulletPrefab, weaponSocket)
instantiate(bulletPrefab, weaponSocket, true)
```

```csharp
public GameObject instantiate(GameObject prefab, Transform parent = null, bool worldSpace = false)
{
    return GameObject.Instantiate(prefab, parent, worldSpace);
}
```

상수가 아닌 기본값은 E158, 기본값 매개변수 뒤에 필수 매개변수가 오면 E159가 발생합니다.

### 명명 인자

호출 사이트에서 매개변수 이름으로 인자를 지정할 수 있습니다. 명명 인자는 임의 순서로 나타날 수 있지만 명명 인자 뒤에 위치 인자가 와서는 안 됩니다 (C#과 동일).

```prsm
GameObject.Instantiate(
    original: bulletPrefab,
    position: spawnPoint.position,
    rotation: Quaternion.identity,
    parent: bulletContainer,
)
```

명명 인자 뒤의 위치 인자는 E160, 알려지지 않은 매개변수 이름은 E161, 같은 매개변수가 (위치 + 명명) 두 번 제공되면 E162가 발생합니다.

### `nameof` 연산자

`nameof(x)`는 컴파일 타임에 문자열 `"x"`로 평가됩니다. `nameof(Type.Member)`는 `"Member"`로 평가됩니다 (C# 동작과 동일). 인자는 실제 심볼을 참조해야 합니다.

```prsm
component Player : MonoBehaviour {
    bind hp: Int = 100
        set {
            field = value
            onPropertyChanged(nameof(hp))
        }

    require rb: Rigidbody

    awake {
        if rb == null {
            error("Required component ${nameof(Rigidbody)} is missing")
        }
    }
}
```

```csharp
public int hp { get => _hp; set { _hp = value; OnPropertyChanged(nameof(hp)); } }
if (rb == null) { Debug.LogError($"Required component {nameof(Rigidbody)} is missing"); }
```

`nameof`는 컨텍스트 키워드입니다. 해석되지 않는 `nameof` 심볼은 E163, 단일 식별자 경로로 해석되지 않는 `nameof` 인자는 E164를 발생시킵니다.

### `@burst` 어노테이션

`@burst` 어노테이션은 함수나 struct를 Unity Burst 컴파일 대상으로 표시합니다. 컴파일러는 대응하는 `[BurstCompile]` 어트리뷰트를 출력하고, 언어 4에서 도입된 Burst 호환성 분석기를 어노테이션 대상에 실행합니다. 언어 4의 진단 E137–E139와 W028은 이제 명명 휴리스틱(`burst_*`) 대신 어노테이션을 통해 발생합니다.

```prsm
@burst
func calculateForces(positions: NativeArray<Float3>, forces: NativeArray<Float3>) {
    for i in 0..positions.length {
        forces[i] = computeGravity(positions[i])
    }
}

@burst(compileSynchronously = true)
struct DamageJob : IJobParallelFor {
    var damages: NativeArray<Int>
    func execute(index: Int) {
        damages[index] = damages[index] * 2
    }
}
```

```csharp
[BurstCompile]
public void calculateForces(NativeArray<float3> positions, NativeArray<float3> forces)
{
    for (int i = 0; i < positions.Length; i++) { forces[i] = computeGravity(positions[i]); }
}

[BurstCompile(CompileSynchronously = true)]
public struct DamageJob : IJobParallelFor
{
    public NativeArray<int> damages;
    public void execute(int index) { damages[index] = damages[index] * 2; }
}
```

`component`, `asset`, `interface`처럼 지원되지 않는 선언 종류에 `@burst`가 붙으면 E165가 발생합니다.

### UniTask 자동 감지

언어 4 컴파일러는 항상 `Cysharp.Threading.Tasks.UniTask`를 `async func` 변환에 출력했고, 이는 UniTask 패키지가 없는 프로젝트의 컴파일을 깨뜨렸습니다. 언어 5는 `Packages/manifest.json` 또는 `.prsmproject`에서 `com.cysharp.unitask`를 감지하여, 패키지가 없으면 `System.Threading.Tasks.Task`로 폴백합니다.

```toml
[language.async]
backend = "auto"  # "unitask" | "task" | "auto"  (기본: "auto")
```

```csharp
// UniTask 있을 때:
public async UniTask<string> loadData() { ... }

// UniTask 없을 때:
public async Task<string> loadData() { ... }
```

UniTask 패키지가 없는데 `backend = "unitask"`를 요청하면 W035가 발생합니다.

## Part III — 언어 4 제약 수정

### `bind X to Y` 연속 푸시

언어 4는 `bind X to Y`를 초기 동기화만 수행하도록 구현했습니다. 언어 5는 반응형 계약을 완성합니다: bind 프로퍼티에 생성된 setter는 `OnPropertyChanged` 이후 등록된 push target 람다를 순회하며 새 값으로 호출합니다.

```csharp
private int _hp = 100;
private List<System.Action<int>> _hpPushTargets;

public int hp
{
    get => _hp;
    set
    {
        if (_hp != value)
        {
            _hp = value;
            OnPropertyChanged(nameof(hp));
            if (_hpPushTargets != null)
                foreach (var t in _hpPushTargets) t(value);
        }
    }
}
```

`bind X to Y`는 다음으로 변환됩니다:

```csharp
_hpPushTargets ??= new List<System.Action<int>>();
_hpPushTargets.Add(v => hpLabel.text = v.ToString());
hpLabel.text = _hp.ToString();
```

언어 4의 진단 E143 (bind 대상 쓰기 불가)과 E144 (타입 불일치)는 그대로 적용됩니다.

### `bind` 미사용 경고

언어 4에서 예약만 되어 있던 W031 진단 코드가 이제 실제로 발생합니다. 컴포넌트의 의미 분석이 끝난 후, 컴파일러는 컴포넌트 내부의 모든 표현식에서 각 `bind` 멤버 참조를 스캔합니다. 자기 setter 외에 참조가 0개인 bind 멤버는 선언 사이트에 W031을 발생시킵니다.

### state machine 예약 이름 완화

`state machine` 블록 안의 상태 이름은 이제 PrSM 예약 키워드(`Start`, `Stop`, `Update` 등)를 받아들입니다. 이 완화는 상태 이름 위치에만 적용되며, 예약 키워드는 다른 모든 곳에서 정상 의미를 유지합니다.

```prsm
state machine playerState {
    state Start { on begin => Update }
    state Update { on pause => Stop }
    state Stop { }
}
```

```csharp
private enum PlayerState { Start, Update, Stop }
```

### `opt.linq` 요소 타입 추론

언어 4의 `opt.linq` 재작성은 LINQ 체인을 `var` 요소 타입의 `for` 루프로 출력했고, 소스 리스트의 정적 타입을 옵티마이저 패스에서 추론할 수 없을 때 `object`로 폴백되어 박싱이 발생했습니다. 언어 5 옵티마이저는 IR walk를 통해 요소 타입 정보를 전파하고, 정적으로 결정할 수 없는 사이트에서는 재작성을 건너뛰어 박싱 폴백을 제거합니다.

```csharp
var alive = new List<Enemy>();
for (int i = 0; i < enemies.Count; i++)
{
    if (enemies[i].IsAlive) alive.Add(enemies[i]);
}
```

W027은 그대로 적용됩니다.

### `ref readonly`를 통한 `opt.structcopy` 실효화

언어 4의 `opt.structcopy` 패스는 `// opt.structcopy` 주석 힌트만 삽입했습니다. PrSM에 `ref readonly` 로컬 문법이 없었기 때문입니다. 스프린트 5에서 `ref local`이 도입되면서 옵티마이저는 이제 핫 경로의 큰 struct 로컬에 실제 `ref readonly` 치환을 수행합니다.

```csharp
ref readonly Vector3 pos = ref transform.position;
```

각 재작성 사이트에 새 진단 W036이 발생하여 최적화가 감사 가능합니다.

### 옵티마이저 driver 자동 연결과 CLI 플래그

컴파일러 driver는 새 컴파일 옵션 `optimize: bool`을 갖습니다 (debug에서 기본 `false`, release에서 `true`). 옵티마이저 패스는 lowering 이후, codegen 이전에 실행됩니다.

```bash
prism build --optimize          # 옵티마이저 활성화
prism build --no-optimize       # 옵티마이저 비활성화
prism build                     # 기본 (debug=꺼짐, release=켜짐)
```

```toml
[compiler]
optimize = true                 # 기본값 재정의
```

W026, W027, W036은 그대로 적용됩니다.

### `unlisten` 컨텍스트 교차 해소

언어 4는 원래 `listen` 컨텍스트 밖에서 나타나는 `unlisten` 문장(예: 라이프사이클 블록에서 호출되는 헬퍼 함수)에 placeholder 주석만 출력했습니다. 언어 5는 컴포넌트별로 모든 `unlisten` 호출 사이트를 수집하고 각 사이트를 후처리하여, 어느 멤버가 `unlisten` 문장을 포함하든 일치하는 listen 핸들러 필드를 찾아냅니다.

대응하는 `listen … manual` 선언이 컴포넌트 내에 없는 토큰 이름은 E166을 발생시킵니다.

## Part IV — 패턴 매칭 확장

### 관계 패턴

관계 패턴은 주어진 연산자로 subject 값과 피연산자를 비교하여 일치하면 매칭됩니다. 정수, 부동소수점, `IComparable<T>`를 구현한 타입에 사용할 수 있습니다.

```prsm
when hp {
    > 80 => "Healthy"
    > 30 => "Hurt"
    > 0  => "Critical"
    else => "Dead"
}
```

```csharp
hp switch
{
    > 80 => "Healthy",
    > 30 => "Hurt",
    > 0  => "Critical",
    _    => "Dead",
}
```

피연산자 타입이 subject 타입과 일치하지 않으면 E167, 이전 arm에 의해 가려지는 후속 관계 arm은 W037을 발생시킵니다.

### 패턴 결합자

`and`, `or`, `not`은 패턴 대수를 형성합니다. 우선순위는 `not`이 가장 높고, 그 다음 `and`, 그 다음 `or`입니다. 새 `or` 키워드는 언어 4의 쉼표-OR 패턴과 통합됩니다.

```prsm
when x {
    > 0 and < 100 => "valid range"
    is Enemy or is Boss => "hostile"
    not null => "present"
    else => "missing"
}
```

```csharp
x switch
{
    > 0 and < 100 => "valid range",
    Enemy or Boss => "hostile",
    not null => "present",
    _ => "missing",
}
```

`or` 패턴 arm이 서로 다른 변수를 바인딩하면 E168이 발생합니다 (언어 4의 E130 확장).

### 위치 패턴

위치 패턴은 subject가 지정된 타입이고 각 서브 패턴이 대응하는 `Deconstruct` 출력과 매칭될 때 일치합니다. 언어 2의 enum payload binding을 모든 분해 가능한 타입(`data class`, `struct`, 튜플)으로 일반화합니다.

```prsm
data class Point(x: Int, y: Int)

when point {
    Point(0, 0) => "origin"
    Point(0, _) => "on y axis"
    Point(_, 0) => "on x axis"
    Point(x, y) if x == y => "diagonal"
    else => "elsewhere"
}
```

```csharp
point switch
{
    Point(0, 0) => "origin",
    Point(0, _) => "on y axis",
    Point(_, 0) => "on x axis",
    Point(var x, var y) when x == y => "diagonal",
    _ => "elsewhere",
}
```

`data class`와 `struct`의 경우 컴파일러가 lowering 중에 `Deconstruct` 메서드를 자동 생성합니다. 위치 패턴 arity가 타입의 분해와 일치하지 않으면 E169가 발생합니다.

### 프로퍼티 패턴

프로퍼티 패턴은 subject가 (선택적으로 지정된) 타입이고 각 명명 프로퍼티 값이 대응하는 서브 패턴과 매칭될 때 일치합니다. 프로퍼티 이름은 공개 읽기 가능 멤버여야 합니다.

```prsm
when target {
    Enemy { hp: > 0, level: > 10 } => "tough enemy"
    Enemy { hp: 0 } => "dead enemy"
    Player { isInvincible: true } => "untouchable"
    else => "ignore"
}
```

```csharp
target switch
{
    Enemy { hp: > 0, level: > 10 } => "tough enemy",
    Enemy { hp: 0 } => "dead enemy",
    Player { isInvincible: true } => "untouchable",
    _ => "ignore",
}
```

존재하지 않는 멤버를 참조하는 프로퍼티 패턴은 E170, 읽기 불가능한 멤버는 E171을 발생시킵니다.

### `with` 표현식

`expr with { f = v, … }`는 지정된 필드를 교체한 `expr`의 복사본을 만듭니다. `data class`는 C# `record with` 표현식으로 변환됩니다. `struct` 선언과 Unity 내장 struct 타입은 임시 복사본 형식을 사용합니다.

```prsm
val origin = transform.position
val grounded = origin with { y = 0.0 }

data class PlayerStats(hp: Int, mp: Int, level: Int)

val current = PlayerStats(100, 50, 5)
val healed = current with { hp = 100 }
val leveled = healed with { level = 6, mp = 100 }
```

```csharp
var origin = transform.position;
Vector3 grounded;
{
    var _t = origin;
    _t.y = 0.0f;
    grounded = _t;
}

public record PlayerStats(int hp, int mp, int level);
var current = new PlayerStats(100, 50, 5);
var healed = current with { hp = 100 };
var leveled = healed with { level = 6, mp = 100 };
```

`data class`, `struct`, 알려진 Unity struct가 아닌 타입에 `with`을 사용하면 E172, 쓰기 불가능한 필드에 사용하면 E173이 발생합니다.

## Part V — 타입 시스템 확장

### `unmanaged` 와 기타 where 제약

언어 5는 언어 3의 제네릭 제약을 `unmanaged`, `notnull`, `default`, `new()`로 확장합니다. `unmanaged` 제약은 `T`가 어떤 깊이에서도 관리 참조를 갖지 않는 값 타입이어야 함을 요구하며, Burst 호환 제네릭 메서드의 표준 제약입니다.

```prsm
@burst
func sum<T>(arr: NativeArray<T>): T where T : unmanaged, INumber<T> {
    var total = T.Zero
    for i in 0..arr.length {
        total += arr[i]
    }
    return total
}
```

```csharp
[BurstCompile]
public T sum<T>(NativeArray<T> arr) where T : unmanaged, INumber<T>
{
    var total = T.Zero;
    for (int i = 0; i < arr.Length; i++) { total += arr[i]; }
    return total;
}
```

같은 매개변수에 `unmanaged`와 `class` 제약을 동시에 사용하면 E174, 값 타입 매개변수에 `notnull`을 사용하면 E175가 발생합니다.

### `ref` 로컬과 `ref` 반환

`val ref name = ref expr`은 읽기 전용 참조 로컬을 만듭니다. `var ref name = ref expr`은 가변 참조 로컬을 만듭니다. 함수는 `ref` 반환 타입을 선언해 참조를 반환할 수 있습니다.

```prsm
struct Particles(positions: NativeArray<Float3>) {
    func getPosition(index: Int): ref Float3 = ref positions[index]
}

func process(particles: Particles) {
    val ref pos = ref particles.getPosition(0)
    log("position: $pos")  // 복사 없음
}
```

```csharp
public struct Particles
{
    public NativeArray<float3> positions;
    public ref float3 getPosition(int index) => ref positions[index];
}

public void process(Particles particles)
{
    ref readonly float3 pos = ref particles.getPosition(0);
    Debug.Log($"position: {pos}");
}
```

`val ref`는 C# `ref readonly`로, `var ref`는 C# `ref`로 변환됩니다. `ref` 로컬이 참조된 저장소보다 오래 살면 E176, 로컬 변수를 참조하는 `ref` 반환은 E177, 쓰기 컨텍스트에서 사용된 `val ref`는 E178이 발생합니다.

### `ref struct`

`ref struct`는 `ref` 필드를 포함할 수 있는 스택 전용 값 타입을 선언합니다. C# ref struct 제약을 따릅니다: 비-ref struct의 필드가 될 수 없고, 박싱될 수 없으며, C# 13+의 `allows ref struct` 제약 없이는 제네릭 타입 인자로 사용할 수 없습니다.

```prsm
ref struct Slice<T>(start: Int, length: Int) {
    func get(i: Int): T = intrinsic { return _data[start + i]; }
}
```

```csharp
public ref struct Slice<T>
{
    public int start;
    public int length;
    public Slice(int start, int length) { this.start = start; this.length = length; }
    public T get(int i) { return _data[start + i]; }
}
```

비-ref struct 또는 클래스의 필드로 선언된 `ref struct`는 E179, `allows ref struct` 없이 제네릭 타입 인자로 사용된 `ref struct`는 E180을 발생시킵니다.

### `stackalloc`

`stackalloc[T](n)`은 스택에 `T` 요소 `n`개를 할당하고 `Span<T>`를 반환합니다. 할당된 메모리는 enclosing 메서드가 반환할 때까지 유효합니다. 결과가 `Span<T>` 로컬에 할당되거나 `Span<T>` / `ReadOnlySpan<T>`를 받는 함수에 전달될 때만 허용됩니다.

```prsm
func sumFirst10(): Int {
    val buffer: Span<Int> = stackalloc[Int](10)
    for i in 0..10 { buffer[i] = i }
    var total = 0
    for i in 0..10 { total += buffer[i] }
    return total
}
```

```csharp
public int sumFirst10()
{
    Span<int> buffer = stackalloc int[10];
    for (int i = 0; i < 10; i++) { buffer[i] = i; }
    var total = 0;
    for (int i = 0; i < 10; i++) { total += buffer[i]; }
    return total;
}
```

`stackalloc` 결과가 `Span<T>` 또는 `ReadOnlySpan<T>`에 할당되지 않으면 E181, 상수가 아닌 크기는 E182를 발생시킵니다.

### Span 슬라이스 문법

receiver가 배열, `Span<T>`, `ReadOnlySpan<T>`, 또는 `Slice(int, int)`와 `Length` 멤버를 가진 타입일 때, range 식으로 인덱싱하면 슬라이스가 생성됩니다. 기존 range 연산자(`..`, `until`, `downTo`)를 재사용합니다.

```prsm
val arr = [1, 2, 3, 4, 5]
val middle = arr[1..4]      // [2, 3, 4]
val tail = arr[2..]         // [3, 4, 5]
val head = arr[..3]         // [1, 2, 3]
```

```csharp
var arr = new int[] { 1, 2, 3, 4, 5 };
var middle = arr[1..4];
var tail = arr[2..];
var head = arr[..3];
```

`Slice` 또는 range 인덱서를 지원하지 않는 타입에 대한 range 슬라이싱은 E183을 발생시킵니다.

### `partial` 선언

`partial` 선언은 해당 특정 타입에 대해 "파일당 단일 선언" 규칙을 완화합니다. 모든 부분이 동일한 식별자, `partial` 한정자, 동일한 종류, 동일한 타입 매개변수와 where clause를 공유하면 여러 파일이 같은 `partial` 선언에 기여할 수 있습니다.

`Player.prsm`:
```prsm
partial component Player : MonoBehaviour {
    serialize speed: Float = 5.0
    require rb: Rigidbody

    update { move() }
}
```

`Player.combat.prsm`:
```prsm
partial component Player {
    bind hp: Int = 100

    func takeDamage(amount: Int) {
        hp -= amount
        if hp <= 0 { die() }
    }
}
```

```csharp
public partial class Player : MonoBehaviour
{
    [SerializeField] private float speed = 5.0f;
    private Rigidbody rb;

    void Awake() { rb = GetComponent<Rigidbody>(); }
    void Update() { move(); }
}

public partial class Player : INotifyPropertyChanged
{
    private int _hp = 100;
    public int hp { get => _hp; set { ... } }

    public void takeDamage(int amount)
    {
        hp -= amount;
        if (hp <= 0) { die(); }
    }
}
```

`partial`은 컨텍스트 키워드입니다. 같은 이름의 두 선언 중 하나만 `partial`을 가지면 E184, 부분 간 타입 매개변수 불일치는 E185, 베이스 클래스 또는 인터페이스 불일치는 E186이 발생합니다.

### 일반화된 nested 선언

언어 4는 (구별 가능한 union 패턴을 위해) `sealed class` 안에서만 nested 선언을 허용했습니다. 언어 5는 이를 일반화합니다: 모든 `class`, `component`, `struct` 본문에 nested 타입 선언을 둘 수 있습니다.

```prsm
component Inventory : MonoBehaviour {
    data class Slot(item: Item, count: Int)

    enum SortOrder { ByName, ByValue, ByRarity }

    var slots: List<Slot> = []
    var sortOrder: SortOrder = SortOrder.ByName

    func addItem(item: Item) {
        slots.add(Slot(item, 1))
    }
}
```

```csharp
public class Inventory : MonoBehaviour
{
    [System.Serializable]
    public class Slot
    {
        public Item item;
        public int count;
        public Slot(Item item, int count) { this.item = item; this.count = count; }
    }

    public enum SortOrder { ByName, ByValue, ByRarity }

    public List<Slot> slots = new List<Slot>();
    public SortOrder sortOrder = SortOrder.ByName;

    public void addItem(Item item) { slots.Add(new Slot(item, 1)); }
}
```

nested `component` 선언은 금지됩니다 (E187). 컴포넌트는 top-level이어야 합니다.

## Part VI — 도구, 저우선순위 문법, DX

### Discard `_`

`_`는 `out` 인자 위치, 분해 바인딩, `when` 패턴에서 "이 값은 의도적으로 무시된다"를 의미합니다. `_`로부터 읽기는 금지됩니다.

```prsm
physics.raycast(ray, out _)

val (_, name) = getResult()

when point {
    Point(0, _) => "on x = 0"
    Point(_, 0) => "on y = 0"
    _ => "elsewhere"
}
```

```csharp
Physics.Raycast(ray, out _);
var (_, name) = getResult();
point switch { Point(0, _) => "on x = 0", Point(_, 0) => "on y = 0", _ => "elsewhere" };
```

discard `_`로부터 읽기는 E188을 발생시킵니다.

### 조건부 인덱서 `arr?[i]`

`arr?[i]`는 `arr`을 평가합니다. `arr`이 `null`이면 전체 식이 `null`입니다. 그렇지 않으면 `arr[i]`에 접근합니다. C# `arr?[i]`로 직접 변환됩니다.

```prsm
val first = inventory?.items?[0]
```

```csharp
var first = inventory?.items?[0];
```

### Throw 표현식

`throw`는 (기존 statement 형식 외에) expression 위치에 나타날 수 있습니다. 일반적으로 elvis 연산자와 함께 필수 필드 검증에 사용됩니다.

```prsm
val rb = body ?: throw IllegalStateException("Rigidbody required")

func divide(a: Int, b: Int): Int =
    if b == 0 then throw ArgumentException("divide by zero")
    else a / b
```

```csharp
var rb = body ?? throw new InvalidOperationException("Rigidbody required");
public int divide(int a, int b) => b == 0 ? throw new ArgumentException("divide by zero") : a / b;
```

### Refactoring LSP dispatch

언어 4는 refactor 모듈(Extract Method, Extract Component, Inline Variable, Rename, Convert to State Machine)을 추가하고 LSP capabilities에 code action 종류를 광고했습니다. 언어 5는 실제 `textDocument/codeAction` 핸들러를 연결합니다:

| LSP code action | 트리거 조건 |
|-----------------|-------------|
| Extract Method | 함수 본문 안에 2개 이상의 statement 선택 |
| Extract Component | 멤버 선언 선택 |
| Inline Variable | 단일 사용 `val` 선언에 커서 |
| Rename Symbol | 모든 선언 사이트에 커서 |
| Convert to State Machine | `enum` + `switch` 패턴에 커서 |

리팩토링 실패는 설명 메시지가 있는 LSP 에러 응답을 반환합니다.

### 디버거 DAP 어댑터

언어 4는 flat `.prsm.map` 소스맵을 출력했지만 이를 소비할 Debug Adapter Protocol 구현을 제공하지 않았습니다. 언어 5는 VS Code 확장 내부에 다음을 수행하는 최소 DAP 어댑터를 제공합니다:

- Unity 프로세스 launch 또는 attach
- 소스맵을 통한 `.prsm` 줄 brakpoint를 생성된 `.cs` 줄 번호로 변환
- threads, stackTrace, scopes, variables proxy 및 변수 이름 테이블을 통한 이름 재매핑
- 컴파일러 생성 boilerplate를 건너뛰는 step filter 적용

소스맵 생성이 실패하면 W032가 그대로 적용됩니다.

## 새로운 진단

### 오류

| 코드 | 기능 | 조건 |
|------|------|------|
| E147 | yield | 코루틴/이터레이터 반환 함수 외부의 `yield` |
| E148 | yield | yield 값 타입이 선언된 요소 타입과 불일치 |
| E149 | 어트리뷰트 타깃 | 선언이 타깃을 지원하지 않음 |
| E150 | 어트리뷰트 타깃 | 자동 프로퍼티가 아닌 곳의 `serialize` |
| E151 | 전처리 | 종료되지 않은 `#if` |
| E152 | 전처리 | `#if` 없는 `#elif` / `#else` |
| E153 | ref/out | 인자 한정자 불일치 |
| E154 | out | 모든 반환 전에 매개변수 미할당 |
| E155 | ref | 불변 `val` 매개변수 전달 |
| E156 | vararg | 마지막이 아닌 매개변수 |
| E157 | vararg | vararg 매개변수 다중 |
| E158 | 기본 매개변수 | 상수가 아닌 기본값 |
| E159 | 기본 매개변수 | 기본값 뒤 필수 매개변수 |
| E160 | 명명 인자 | 명명 뒤 위치 |
| E161 | 명명 인자 | 알려지지 않은 매개변수 이름 |
| E162 | 명명 인자 | 매개변수 두 번 제공 |
| E163 | nameof | 미해결 심볼 |
| E164 | nameof | 단일 식별자 경로 아님 |
| E165 | @burst | 지원되지 않는 선언 종류 |
| E166 | unlisten | 일치하는 listen … manual 없음 |
| E167 | 관계 패턴 | 타입 불일치 |
| E168 | or 패턴 | 바인딩 불일치 |
| E169 | 위치 패턴 | arity 불일치 |
| E170 | 프로퍼티 패턴 | 알려지지 않은 멤버 |
| E171 | 프로퍼티 패턴 | 읽기 불가능 멤버 |
| E172 | with | 지원되지 않는 타입 |
| E173 | with | 쓰기 불가능 필드 |
| E174 | unmanaged | 충돌하는 class 제약 |
| E175 | notnull | 값 타입 매개변수 |
| E176 | ref local | 참조 저장소보다 오래 살음 |
| E177 | ref return | 로컬 변수 참조 |
| E178 | val ref | 쓰기 컨텍스트에서 사용 |
| E179 | ref struct | 비-ref struct 필드 |
| E180 | ref struct | allows 없는 제네릭 타입 인자 |
| E181 | stackalloc | 결과가 Span 아님 |
| E182 | stackalloc | 상수 아닌 크기 |
| E183 | span slice | 지원되지 않는 타입 |
| E184 | partial | partial 한정자 불일치 |
| E185 | partial | 타입 매개변수 불일치 |
| E186 | partial | 베이스/인터페이스 불일치 |
| E187 | nested | 다른 타입 안에 nested된 component |
| E188 | discard | `_`로부터 읽기 |

### 경고

| 코드 | 기능 | 조건 |
|------|------|------|
| W033 | yield | `Seq<T>` 코루틴이 T를 yield하지 않음 |
| W034 | 전처리 | 알려지지 않은 심볼 통과 |
| W035 | UniTask | `unitask` backend 요청했으나 패키지 없음 |
| W036 | opt.structcopy | `ref readonly`로 재작성됨 |
| W037 | 관계 패턴 | 도달 불가능한 후속 arm |

언어 4 경고 W031 (`bind` 미사용)는 이제 실제로 발생합니다.

## 기능 게이트

언어 5의 모든 기능은 `version = "5"` 설정 시 암묵적으로 활성화됩니다. 개별 기능은 언어 4에서 선택적으로 활성화할 수 있습니다:

```toml
[language]
version = "4"
features = ["yield-general", "attribute-target", "preprocessor"]
```

| 플래그 | 설명 |
|--------|------|
| `yield-general` | 코루틴의 일반 `yield return` / `yield break` |
| `attribute-target` | `@field` / `@property` 등 어트리뷰트 타깃 |
| `preprocessor` | `#if` / `#elif` / `#else` / `#endif` 디렉티브 |
| `ref-out-params` | `ref` / `out` 매개변수 |
| `vararg` | `vararg` (params) 매개변수 |
| `default-params` | 매개변수 기본값 |
| `named-args` | 명명 호출 인자 |
| `nameof` | `nameof` 연산자 |
| `burst-annotation` | `@burst` 어노테이션 |
| `unitask-detect` | UniTask 자동 감지 |
| `bind-push` | bind X to Y 연속 푸시 |
| `bind-unread-warn` | bind W031 구현 |
| `state-name-relax` | state machine 예약 이름 허용 |
| `opt-linq-types` | opt.linq 요소 타입 추론 |
| `opt-structcopy-ref` | opt.structcopy ref readonly 실효화 |
| `optimizer-cli` | 옵티마이저 driver 자동 연결과 CLI |
| `unlisten-cross` | 컨텍스트 교차 unlisten 해소 |
| `relational-pattern` | 관계 패턴 |
| `pattern-combinator` | and / or / not 패턴 결합자 |
| `positional-pattern` | 위치 패턴 |
| `property-pattern` | 프로퍼티 패턴 |
| `with-expr` | with 표현식 |
| `unmanaged-constraint` | unmanaged / notnull / default 제약 |
| `ref-local` | ref local 과 ref return |
| `ref-struct` | ref struct 선언 |
| `stackalloc` | stackalloc 표현식 |
| `span-slice` | span / array range 슬라이싱 |
| `partial` | partial class / component / struct |
| `nested-decl` | 일반화된 nested 선언 |
| `discard` | discard `_` 표현식 / 패턴 |
| `safe-index` | 조건부 인덱서 `?[i]` |
| `throw-expr` | throw 표현식 |
| `lsp-refactor-dispatch` | LSP refactor code action dispatch |
| `dap-adapter` | DAP 디버그 어댑터 |

## 툴체인

- **Prism v3.0.0** — 언어 5 컴파일러, v5 LSP, v5 VS Code 확장을 번들
- **22개 새 기능 + 12개 제약 수정**을 6개 컴파일러 스프린트에 걸쳐 구현
- 모든 언어 4 테스트가 변경 없이 그대로 통과
- VS Code 확장과 함께 **DAP 어댑터** 제공으로 Unity에서 소스 수준 디버깅 가능
- **UniTask 자동 감지**로 `async` 변환을 위한 UniTask 패키지에 대한 강제 의존이 제거됨
- 언어 4 릴리스 윈도우에서 설정된 **MIT 라이선스** 유지
