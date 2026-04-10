# Advanced Features

## Listen Lifetime

### 기본 (register, cleanup 없음)

```prsm
start {
    listen button.onClick {
        fire()
    }
}
```
```csharp
private void Start() {
    button.onClick.AddListener(() => { Fire(); });
}
```

### until disable

OnDisable에서 자동 RemoveListener:

```prsm
listen button.onClick until disable {
    fire()
}
```
```csharp
private System.Action _handler_0;

private void Awake() {
    _handler_0 = () => { Fire(); };
    button.onClick.AddListener(_handler_0);
}

private void OnDisable() {
    button.onClick.RemoveListener(_handler_0);
}
```

### until destroy

OnDestroy에서 자동 RemoveListener:

```prsm
listen health.onChanged until destroy {
    updateUI()
}
```

### manual

명시적 unlisten 필요:

```prsm
listen button.onClick manual {
    fire()
}

// 나중에
unlisten button.onClick
```

### Lambda 매개변수

```prsm
listen slider.onValueChanged {
    newValue => setVolume(newValue)
}
```
```csharp
slider.onValueChanged.AddListener((newValue) => { SetVolume(newValue); });
```

### Member-level listen

lifecycle 블록 밖에서 직접 선언하면 synthetic Awake에 등록:

```prsm
component UI : MonoBehaviour {
    serialize button: Button

    listen button.onClick until disable {
        print("clicked")
    }
}
```

## State Machine

```prsm
component Door : MonoBehaviour {
    state machine doorState {
        state Closed {
            enter { animator.play("Close") }
            on open => Opening
        }

        state Opening {
            enter { animator.play("Open") }
            on opened => Open
        }

        state Open {
            enter { print("door is open") }
            on close => Closing
        }

        state Closing {
            enter { animator.play("Close") }
            on closed => Closed
        }
    }
}
```

생성: enum `DoorState { Closed, Opening, Open, Closing }` + 상태 전환 메서드 + enter/exit 훅.

트리거 호출:

```prsm
func onPlayerInteract() {
    doorState.trigger("open")
}
```

## Command Pattern

```prsm
component Editor : MonoBehaviour {
    command moveTo(target: Vector3) {
        execute {
            transform.position = target
        }
        undo {
            transform.position = previousPosition
        }
        canExecute = target != transform.position
    }
}
```

ICommand 인터페이스 구현 + undo stack 자동 생성.

## Bind / MVVM

### 바인딩 필드

```prsm
component PlayerUI : MonoBehaviour {
    bind hp: Int = 100
}
```

INotifyPropertyChanged 구현 + backing field + OnPropertyChanged 자동 호출.

### bind to (push target)

```prsm
serialize hpLabel: Text

awake {
    bind hp to hpLabel.text
}
```

`hp` 값이 변경되면 `hpLabel.text`에 자동 반영.

## Property Accessor

### 읽기 전용

```prsm
val isDead: Bool get = hp <= 0
```
```csharp
public bool isDead => _hp <= 0;
```

### 커스텀 getter/setter

```prsm
var hp: Int {
    get { return _hp }
    set {
        _hp = Math.clamp(value, 0, maxHp)
        onHpChanged?.invoke(_hp)
    }
}
```
```csharp
public int hp {
    get { return _hp; }
    set {
        _hp = Mathf.Clamp(value, 0, _maxHp);
        onHpChanged?.Invoke(_hp);
    }
}
```

## Operator Overloading

```prsm
data class Vec2i(val x: Int, val y: Int) {
    operator func +(other: Vec2i): Vec2i = Vec2i(x + other.x, y + other.y)
    operator func -(other: Vec2i): Vec2i = Vec2i(x - other.x, y - other.y)
    operator func *(scalar: Int): Vec2i = Vec2i(x * scalar, y * scalar)
}
```
```csharp
public static Vec2i operator +(Vec2i left, Vec2i right) =>
    new Vec2i(left.x + right.x, left.y + right.y);
```

## singleton

```prsm
singleton component GameManager : MonoBehaviour {
    var score: Int = 0

    func addScore(points: Int) {
        score += points
    }
}
```

생성 패턴:
- `private static GameManager _instance;`
- `public static GameManager Instance { get { ... FindFirstObjectByType fallback ... } }`
- `DontDestroyOnLoad(gameObject)` in Awake
- 중복 인스턴스 자동 파괴

사용: `GameManager.Instance.addScore(10)`

## partial

```prsm
partial component Player : MonoBehaviour {
    serialize hp: Int = 100
}

// 다른 파일
partial component Player {
    func heal(amount: Int) { hp += amount }
}
```
```csharp
public partial class Player : MonoBehaviour {
    [SerializeField] private int _hp = 100;
}

// 다른 파일
public partial class Player {
    public void Heal(int amount) { _hp += amount; }
}
```

## Inheritance Modifiers

| PrSM | C# | 용도 |
|------|-----|------|
| `abstract` | `abstract` | 구현 없는 추상 선언 |
| `open` | `virtual` | 오버라이드 허용 |
| `override` | `override` | 부모 메서드 재정의 |
| `sealed` | `sealed` | 클래스 상속 또는 메서드 오버라이드 차단 |

## Extension Methods

```prsm
extend Transform {
    func resetLocal() {
        this.localPosition = Vector3.zero
        this.localRotation = Quaternion.identity
        this.localScale = Vector3.one
    }
}
```
```csharp
public static class TransformExtensions {
    public static void ResetLocal(this Transform self) {
        self.localPosition = Vector3.zero;
        self.localRotation = Quaternion.identity;
        self.localScale = Vector3.one;
    }
}
```

## Destructuring

### val destructure

```prsm
val (x, y) = getPosition()
val Stats(hp, speed) = getStats()
```
```csharp
var (x, y) = GetPosition();
var __stats = GetStats();
var hp = __stats.hp;
var speed = __stats.speed;
```

### for destructure

```prsm
for (key, value) in map {
    print("$key: $value")
}
```
```csharp
foreach (var (key, value) in map) {
    Debug.Log($"{key}: {value}");
}
```

## Nested Declarations

component/class 내부에 타입 선언:

```prsm
component Inventory : MonoBehaviour {
    data class Slot(val itemId: Int, val count: Int)
    enum Category { Weapon, Armor, Consumable }

    var slots: List<Slot> = List<Slot>()
}
```

중첩된 타입은 `Inventory.Slot`, `Inventory.Category`로 접근.
