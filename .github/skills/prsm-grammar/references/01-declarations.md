# Top-Level Declarations

PrSM 파일은 하나의 최상위 선언을 포함한다.

## component

Unity MonoBehaviour 기반 게임로직:

```prsm
component PlayerController : MonoBehaviour {
    serialize speed: Float = 5.0
    require rb: Rigidbody

    update {
        rb.velocity = vec3(input.axis("Horizontal") * speed, rb.velocity.y, 0)
    }

    func jump() {
        rb.addForce(vec3(0, 10, 0), ForceMode.Impulse)
    }
}
```
```csharp
public class PlayerController : MonoBehaviour {
    [SerializeField] private float _speed = 5.0f;
    public float speed { get => _speed; set => _speed = value; }
    private Rigidbody _rb;

    private void Awake() {
        _rb = GetComponent<Rigidbody>();
        if (_rb == null) { Debug.LogError("..."); enabled = false; return; }
    }

    private void Update() {
        _rb.velocity = new Vector3(Input.GetAxis("Horizontal") * _speed, _rb.velocity.y, 0);
    }

    public void Jump() {
        _rb.AddForce(new Vector3(0, 10, 0), ForceMode.Impulse);
    }
}
```

### 인터페이스 구현

```prsm
component Enemy : MonoBehaviour, IDamageable, IPoolable {
    // ...
}
```

### singleton

```prsm
singleton component AudioManager : MonoBehaviour {
    func playSound(clip: AudioClip) { }
}
```

`Instance` 프로퍼티 + `DontDestroyOnLoad` + `FindFirstObjectByType` fallback 자동 생성.

### partial

```prsm
partial component Player : MonoBehaviour {
    // 다른 .prsm 파일에서 확장 가능
}
```

## asset

ScriptableObject 기반 데이터:

```prsm
asset WeaponData : ScriptableObject {
    serialize damage: Int = 10
    serialize range: Float = 5.0
    serialize icon: Sprite
}
```
```csharp
[CreateAssetMenu(menuName = "WeaponData")]
public class WeaponData : ScriptableObject {
    [SerializeField] private int _damage = 10;
    [SerializeField] private float _range = 5.0f;
    [SerializeField] private Sprite _icon;
    // + public properties
}
```

## class

일반 C# 클래스:

```prsm
class DamageCalculator {
    static func calculate(base: Int, multiplier: Float): Int =
        (base.toFloat() * multiplier).toInt()
}
```
```csharp
public class DamageCalculator {
    public static int Calculate(int @base, float multiplier) =>
        (int)((float)@base * multiplier);
}
```

### abstract / sealed / open

```prsm
abstract class Weapon {
    abstract func attack()
    open func reload() { }
}

sealed class FinalWeapon : Weapon {
    override func attack() { print("pow") }
}
```
```csharp
public abstract class Weapon {
    public abstract void Attack();
    public virtual void Reload() { }
}

public sealed class FinalWeapon : Weapon {
    public override void Attack() { Debug.Log("pow"); }
}
```

### Generic + where

```prsm
class Registry<T> where T : Component {
    val items: List<T> = List<T>()
    func register(item: T) { items.add(item) }
}
```
```csharp
public class Registry<T> where T : Component {
    private readonly List<T> _items = new List<T>();
    public void Register(T item) { _items.Add(item); }
}
```

다중 제약: `where T : A, B`

## data class

불변 데이터 구조 (Equals, GetHashCode, ToString 자동 생성):

```prsm
data class DamageInfo(val amount: Int, val crit: Bool)
```
```csharp
[System.Serializable]
public class DamageInfo {
    public int amount;
    public bool crit;

    public DamageInfo(int amount, bool crit) {
        this.amount = amount;
        this.crit = crit;
    }

    public override bool Equals(object obj) { ... }
    public override int GetHashCode() { ... }
    public override string ToString() => $"DamageInfo(amount={amount}, crit={crit})";
}
```

본문 블록 포함 가능:

```prsm
data class Vec2i(val x: Int, val y: Int) {
    func magnitude(): Float = Math.sqrt((x * x + y * y).toFloat())
}
```

## enum

### Simple enum

```prsm
enum EnemyState {
    Idle,
    Chase,
    Attack
}
```
```csharp
public enum EnemyState { Idle, Chase, Attack, }
```

### Parameterized enum

```prsm
enum Weapon(val damage: Int, val range: Float) {
    Sword(10, 1.5),
    Bow(7, 8.0),
    Staff(15, 3.0)
}
```
```csharp
public enum Weapon { Sword, Bow, Staff, }

public static class WeaponExtensions {
    public static int Damage(this Weapon value) => value switch {
        Weapon.Sword => 10, Weapon.Bow => 7, Weapon.Staff => 15, _ => 0
    };
    public static float Range(this Weapon value) => value switch {
        Weapon.Sword => 1.5f, Weapon.Bow => 8.0f, Weapon.Staff => 3.0f, _ => 0f
    };
}
```

### Sum-type enum (payload)

```prsm
enum Result {
    Ok(value: Int),
    Err(message: String)
}
```

abstract base class + sealed variant 클래스로 변환.

## interface

```prsm
interface IDamageable {
    func takeDamage(amount: Int)
    val isAlive: Bool
}

interface IHealable : IDamageable {
    func heal(amount: Int)
}
```
```csharp
public interface IDamageable {
    void TakeDamage(int amount);
    bool isAlive { get; }
}

public interface IHealable : IDamageable {
    void Heal(int amount);
}
```

## struct

값 타입 구조체:

```prsm
struct GridPos(val x: Int, val y: Int) {
    static val zero: GridPos = GridPos(0, 0)

    func distanceTo(other: GridPos): Float =
        Math.sqrt(((x - other.x) * (x - other.x) + (y - other.y) * (y - other.y)).toFloat())
}
```
```csharp
public struct GridPos {
    public int x;
    public int y;
    public GridPos(int x, int y) { this.x = x; this.y = y; }

    public static readonly GridPos zero = new GridPos(0, 0);
    public float DistanceTo(GridPos other) =>
        (float)Math.Sqrt((float)((x - other.x) * (x - other.x) + (y - other.y) * (y - other.y)));
}
```

`ref struct`도 지원: `ref struct Slice(val begin: Int, val length: Int)`

## extend

기존 타입에 메서드 추가:

```prsm
extend String {
    func isBlank(): Bool = this.trim().length == 0
}
```
```csharp
public static class StringExtensions {
    public static bool IsBlank(this string self) => self.Trim().Length == 0;
}
```

## typealias

타입 별칭:

```prsm
typealias Position = Vector3
typealias EnemyList = List<Enemy>
```
```csharp
using Position = UnityEngine.Vector3;
using EnemyList = System.Collections.Generic.List<Enemy>;
```

## attribute

커스텀 C# 어트리뷰트 클래스:

```prsm
attribute Cooldown(val duration: Float, val resetOnHit: Bool = false)
```
```csharp
[System.AttributeUsage(System.AttributeTargets.All)]
public class CooldownAttribute : System.Attribute {
    public float duration;
    public bool resetOnHit;
    public CooldownAttribute(float duration, bool resetOnHit = false) {
        this.duration = duration;
        this.resetOnHit = resetOnHit;
    }
}
```
