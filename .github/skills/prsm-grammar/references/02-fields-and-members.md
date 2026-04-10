# Fields & Members

## serialize

Inspector에 노출되는 직렬화 필드:

```prsm
serialize speed: Float = 5.0
serialize maxHp: Int = 100
serialize target: Transform
```
```csharp
[SerializeField] private float _speed = 5.0f;
public float speed { get => _speed; set => _speed = value; }

[SerializeField] private int _maxHp = 100;
public int maxHp { get => _maxHp; set => _maxHp = value; }

[SerializeField] private Transform _target;
public Transform target { get => _target; set => _target = value; }
```

annotation과 조합:

```prsm
@header("Combat")
@range(0, 200)
serialize damage: Int = 10
```
```csharp
[Header("Combat")]
[Range(0, 200)]
[SerializeField] private int _damage = 10;
```

## require

Awake()에서 자동 GetComponent + null 검사:

```prsm
require rb: Rigidbody
require col: Collider
```
```csharp
private Rigidbody _rb;
private Collider _col;

private void Awake() {
    _rb = GetComponent<Rigidbody>();
    if (_rb == null) { Debug.LogError("PlayerController requires Rigidbody"); enabled = false; return; }
    _col = GetComponent<Collider>();
    if (_col == null) { Debug.LogError("PlayerController requires Collider"); enabled = false; return; }
}
```

null이면 에러 로그 + 컴포넌트 비활성화.

## optional

null 허용 GetComponent (검사 없음):

```prsm
optional animator: Animator
```
```csharp
private Animator _animator;

private void Awake() {
    _animator = GetComponent<Animator>();
    // null 검사 없음 — 런타임에 null일 수 있음
}
```

사용 시 safe call 권장: `animator?.play("Run")`

## child

자식 오브젝트에서 GetComponentInChildren:

```prsm
child label: Text
child healthBar: Slider
```
```csharp
private Text _label;
private void Awake() {
    _label = GetComponentInChildren<Text>();
}
```

## parent

부모 오브젝트에서 GetComponentInParent:

```prsm
parent canvas: Canvas
```
```csharp
private Canvas _canvas;
private void Awake() {
    _canvas = GetComponentInParent<Canvas>();
}
```

## val (불변 필드)

```prsm
val maxSpeed: Float = 10.0
val teamName: String = "Blue"
```
```csharp
private readonly float _maxSpeed = 10.0f;
private readonly string _teamName = "Blue";
```

### static val

```prsm
static val PI: Float = 3.14159
```
```csharp
public static readonly float PI = 3.14159f;
```

### const

```prsm
const MAX_LEVEL: Int = 99
```
```csharp
public const int MAX_LEVEL = 99;
```

## var (가변 필드)

```prsm
var currentHp: Int = 100
var isAlive: Bool = true
```
```csharp
private int _currentHp = 100;
private bool _isAlive = true;
```

## event

이벤트 선언 + 발동:

```prsm
component Boss : MonoBehaviour {
    event onDamaged: (Int) => Unit
    event onDeath: () => Unit

    func takeDamage(amount: Int) {
        onDamaged?.invoke(amount)
        if hp <= 0 {
            onDeath?.invoke()
        }
    }
}
```
```csharp
public event System.Action<int> onDamaged;
public event System.Action onDeath;

public void TakeDamage(int amount) {
    onDamaged?.Invoke(amount);
    if (_hp <= 0) { onDeath?.Invoke(); }
}
```

## pool

오브젝트 풀:

```prsm
pool bullets: Bullet(capacity = 20, max = 100)

func fire() {
    val b = bullets.get()
    b.launch(transform.forward)
}
```

`ObjectPool<Bullet>` 자동 초기화. `get()`, `release()` 메서드 사용.

## Property Accessors

커스텀 getter/setter:

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
        _hp = Math.Clamp(value, 0, _maxHp);
        onHpChanged?.Invoke(_hp);
    }
}
```

Expression body:

```prsm
val isDead: Bool get = hp <= 0
```
```csharp
public bool isDead => _hp <= 0;
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
```csharp
public class Inventory : MonoBehaviour {
    [System.Serializable]
    public class Slot { ... }
    public enum Category { Weapon, Armor, Consumable, }

    private List<Slot> _slots = new List<Slot>();
}
```
