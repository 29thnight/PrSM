# Type System

## Primitive Type Mapping

| PrSM | C# | 비고 |
|------|-----|------|
| `Int` | `int` | 32-bit 정수 |
| `Float` | `float` | 단정밀도 실수 |
| `Double` | `double` | 배정밀도 실수 |
| `Bool` | `bool` | |
| `String` | `string` | |
| `Char` | `char` | |
| `Long` | `long` | 64-bit 정수 |
| `Byte` | `byte` | 8-bit |
| `Unit` | `void` | 반환 타입에서만 |

## Nullable Types

```prsm
val maybeInt: Int? = null
val name: String? = getSomething()
val value = name ?? "default"
```
```csharp
int? maybeInt = null;
string name = GetSomething();
var value = name ?? "default";
```

## Collection Type Mapping

| PrSM | C# |
|------|-----|
| `List<T>` | `System.Collections.Generic.List<T>` |
| `Map<K, V>` | `System.Collections.Generic.Dictionary<K, V>` |
| `Set<T>` | `System.Collections.Generic.HashSet<T>` |
| `Seq<T>` | `System.Collections.Generic.IEnumerable<T>` |
| `Queue<T>` | `System.Collections.Generic.Queue<T>` |
| `Stack<T>` | `System.Collections.Generic.Stack<T>` |
| `Array<T>` | `T[]` |

## Generic Types

```prsm
class Registry<T> where T : Component {
    val items: List<T> = List<T>()

    func add(item: T) {
        items.add(item)
    }
}
```
```csharp
public class Registry<T> where T : Component {
    private readonly List<T> _items = new List<T>();
    public void Add(T item) { _items.Add(item); }
}
```

다중 제약: `where T : A, B`

## Function Types

```prsm
val callback: () => Unit           // 반환 없음
val predicate: (Int) => Bool       // 매개변수 1개
val transform: (Int, Int) => Float // 매개변수 2개

event onDamaged: (Int) => Unit
```
```csharp
System.Action callback;
System.Func<int, bool> predicate;
System.Func<int, int, float> transform;

public event System.Action<int> onDamaged;
```

`=> Unit`이면 `Action`, 그 외는 `Func`으로 변환.

## Tuple Types

```prsm
val pair: (Int, String) = (42, "hello")
val named: (hp: Int, mp: Int) = (hp: 100, mp: 50)

func getStats(): (Int, Int) = (hp, mp)

val (a, b) = getStats()  // destructure
```
```csharp
(int, string) pair = (42, "hello");
(int hp, int mp) named = (hp: 100, mp: 50);

public (int, int) GetStats() => (_hp, _mp);

var (a, b) = GetStats();
```

## Ref Types

```prsm
func getRef(ref arr: List<Int>, idx: Int): ref Int {
    return ref arr[idx]
}
```
```csharp
public ref int GetRef(ref List<int> arr, int idx) {
    return ref arr[idx];
}
```

## Qualified Types

네임스페이스 포함 타입:

```prsm
val ex: System.Exception = System.Exception("error")
val list: System.Collections.Generic.List<Int> = System.Collections.Generic.List<Int>()
```

일반적으로 짧은 이름(Int, List 등)을 사용하고, 필요 시 정규화된 이름도 허용.
