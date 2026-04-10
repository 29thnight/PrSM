# Functions, Coroutines & Async

## func — Block Body

```prsm
func takeDamage(amount: Int) {
    hp -= amount
    if hp <= 0 { die() }
}
```
```csharp
public void TakeDamage(int amount) {
    _hp -= amount;
    if (_hp <= 0) { Die(); }
}
```

## func — Expression Body

`=` 뒤에 단일 식으로 선언:

```prsm
func isDead(): Bool = hp <= 0
func double(x: Int): Int = x * 2
func greet(name: String): String = "Hello, $name"
```
```csharp
public bool IsDead() => _hp <= 0;
public int Double(int x) => x * 2;
public string Greet(string name) => $"Hello, {name}";
```

## Access Modifiers

| PrSM | C# | 비고 |
|------|-----|------|
| (기본) | `public` | PrSM 메서드는 기본 public |
| `private` | `private` | |
| `static` | `public static` | |
| `abstract` | `public abstract` | abstract class 내부 |
| `open` | `public virtual` | 오버라이드 허용 |
| `override` | `public override` | |
| `sealed override` | `public sealed override` | 더 이상 오버라이드 불가 |

```prsm
abstract class Weapon {
    abstract func attack()
    open func reload() { print("reloading") }
}

class Sword : Weapon {
    override func attack() { print("slash!") }
    sealed override func reload() { print("sharpening") }
}
```

## Generic Function

```prsm
func findAll<T>(): List<T> where T : Component {
    return List<T>(FindObjectsByType<T>(FindObjectsSortMode.None))
}
```
```csharp
public List<T> FindAll<T>() where T : Component {
    return new List<T>(FindObjectsByType<T>(FindObjectsSortMode.None));
}
```

## Parameter Modifiers

### Default values

```prsm
func spawn(pos: Vector3, delay: Float = 0.0, active: Bool = true) { }
```
```csharp
public void Spawn(Vector3 pos, float delay = 0.0f, bool active = true) { }
```

### ref / out

```prsm
func swap(ref a: Int, ref b: Int) {
    val temp = a
    a = b
    b = temp
}

func tryParse(str: String, out result: Int): Bool {
    return Int.tryParse(str, out result)
}
```
```csharp
public void Swap(ref int a, ref int b) {
    var temp = a;
    a = b;
    b = temp;
}

public bool TryParse(string str, out int result) {
    return int.TryParse(str, out result);
}
```

### vararg (params)

```prsm
func sum(vararg numbers: Int): Int {
    var total = 0
    for n in numbers { total += n }
    return total
}
```
```csharp
public int Sum(params int[] numbers) {
    var total = 0;
    foreach (var n in numbers) { total += n; }
    return total;
}
```

## coroutine

Unity IEnumerator 코루틴:

```prsm
coroutine fadeOut(duration: Float) {
    var t: Float = 0
    while t < duration {
        renderer.alpha = 1.0 - (t / duration)
        wait nextFrame
        t += Time.deltaTime
    }
    renderer.alpha = 0
}
```
```csharp
private IEnumerator FadeOut(float duration) {
    float t = 0;
    while (t < duration) {
        _renderer.alpha = 1.0f - (t / duration);
        yield return null;
        t += Time.deltaTime;
    }
    _renderer.alpha = 0;
}
```

### wait 형식

| PrSM | C# |
|------|-----|
| `wait 1.0s` | `yield return new WaitForSeconds(1.0f)` |
| `wait 500ms` | `yield return new WaitForSeconds(0.5f)` |
| `wait nextFrame` | `yield return null` |
| `wait fixedFrame` | `yield return new WaitForFixedUpdate()` |
| `wait until condition` | `yield return new WaitUntil(() => condition)` |
| `wait while condition` | `yield return new WaitWhile(() => condition)` |

### start / stop

```prsm
start fadeOut(1.0)          // StartCoroutine(FadeOut(1.0f))
stop fadeOut                // StopCoroutine("FadeOut")
stopAll()                   // StopAllCoroutines()
```

### yield (typed coroutine)

```prsm
coroutine fibonacci(): Seq<Int> {
    var a = 0
    var b = 1
    while true {
        yield a
        val temp = a
        a = b
        b = temp + b
    }
}
```
```csharp
public IEnumerable<int> Fibonacci() {
    var a = 0;
    var b = 1;
    while (true) {
        yield return a;
        var temp = a;
        a = b;
        b = temp + b;
    }
}
```

## async func

```prsm
async func loadProfile(id: String): PlayerData {
    val json = await httpClient.getStringAsync("/api/player/$id")
    return PlayerData.fromJson(json)
}
```
```csharp
public async UniTask<PlayerData> LoadProfile(string id) {
    var json = await _httpClient.GetStringAsync($"/api/player/{id}");
    return PlayerData.FromJson(json);
}
```

## intrinsic

원시 C# 코드 직접 삽입:

```prsm
func nativeSort(arr: List<Int>) {
    intrinsic {
        arr.Sort((a, b) => b.CompareTo(a));
    }
}
```
```csharp
public void NativeSort(List<int> arr) {
    arr.Sort((a, b) => b.CompareTo(a));
}
```

intrinsic 블록 내부는 C# 그대로 출력됨 (PrSM 변환 없음).
