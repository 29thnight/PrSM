# Control Flow

## if / else

```prsm
if hp <= 0 {
    die()
} else if hp < 30 {
    warn("low health")
} else {
    heal()
}
```
```csharp
if (_hp <= 0) { Die(); }
else if (_hp < 30) { Warn("low health"); }
else { Heal(); }
```

if expression: [06-expressions.md](./06-expressions.md) 참조.

## when (Pattern Matching)

### Subject 있는 when

```prsm
when state {
    EnemyState.Idle => sleep()
    EnemyState.Chase => pursue(target)
    EnemyState.Attack => attack()
    else => idle()
}
```
```csharp
switch (state) {
    case EnemyState.Idle: Sleep(); break;
    case EnemyState.Chase: Pursue(_target); break;
    case EnemyState.Attack: Attack(); break;
    default: Idle(); break;
}
```

### Subject 없는 when (조건 분기)

```prsm
when {
    hp > 80 => status = "healthy"
    hp > 30 => status = "wounded"
    else => status = "critical"
}
```
```csharp
if (_hp > 80) _status = "healthy";
else if (_hp > 30) _status = "wounded";
else _status = "critical";
```

### when Expression (값 반환)

subject 있는 경우 C# switch expression으로 변환:

```prsm
val label = when score {
    in 90..100 => "A"
    in 80..89 => "B"
    else => "F"
}
```
```csharp
var label = _score switch {
    >= 90 and <= 100 => "A",
    >= 80 and <= 89 => "B",
    _ => "F",
};
```

subject 없는 경우 삼항 연산자 체인:

```prsm
val msg = when {
    x > 0 => "positive"
    x < 0 => "negative"
    else => "zero"
}
```
```csharp
var msg = (x > 0 ? "positive" : (x < 0 ? "negative" : "zero"));
```

expression body 함수에서 직접 반환:

```prsm
func grade(score: Int): String = when score {
    in 90..100 => "A"
    in 80..89 => "B"
    in 70..79 => "C"
    else => "F"
}
```
```csharp
public string Grade(int score) => score switch {
    >= 90 and <= 100 => "A",
    >= 80 and <= 89 => "B",
    >= 70 and <= 79 => "C",
    _ => "F",
};
```

### 패턴 종류

#### 값 매칭

```prsm
when direction {
    "up" => moveUp()
    "down" => moveDown()
    else => stop()
}
```

#### Is 패턴 (타입 테스트)

```prsm
when obj {
    is Enemy => obj.attack()
    is Ally => obj.heal()
    else => print("unknown")
}
```

#### Enum 바인딩 패턴

```prsm
when result {
    Result.Ok(value) => process(value)
    Result.Err(msg) => print("Error: $msg")
}
```

#### OR 패턴 (쉼표)

```prsm
when value {
    1, 2, 3 => print("1-3")
    4, 5, 6 => print("4-6")
    else => print("other")
}
```
```csharp
switch (value) {
    case 1: case 2: case 3: Debug.Log("1-3"); break;
    case 4: case 5: case 6: Debug.Log("4-6"); break;
    default: Debug.Log("other"); break;
}
```

`or` 키워드도 사용 가능: `1 or 2 or 3 => ...`

#### Range 패턴

```prsm
when score {
    in 90..100 => print("A")
    in 80..89 => print("B")
    else => print("F")
}
```
```csharp
// switch expression에서: >= 90 and <= 100 => "A"
// switch statement에서: if (score >= 90 && score <= 100) 패턴
```

#### Relational 패턴

```prsm
when hp {
    > 80 => "healthy"
    > 30 => "hurt"
    <= 30 => "critical"
    != 0 => "alive"
}
```

C# 9 relational patterns로 직접 변환: `> 80 => "healthy"`

#### AND 패턴

```prsm
when value {
    > 0 and < 100 => print("in range")
    else => print("out of range")
}
```
```csharp
// switch expression: > 0 and < 100 => "in range"
```

#### NOT 패턴

```prsm
when x {
    not 0 => print("non-zero")
    else => print("zero")
}
```
```csharp
// switch expression: not 0 => "non-zero"
```

#### Guard 조건

```prsm
when state {
    EnemyState.Stunned(duration) if duration > 0.5 => heavyStun()
    EnemyState.Stunned(duration) => lightStun()
    else => normalUpdate()
}
```

`if` 뒤에 조건식을 붙여 추가 필터링.

### when Exhaustiveness

expression 위치의 when에 `else`가 없으면 자동으로 `_ => throw new System.InvalidOperationException("PrSM when expression is not exhaustive")` 추가.

## for

### Range (exclusive)

```prsm
for i in 0 until 10 {
    print(i)  // 0, 1, 2, ..., 9
}
```
```csharp
for (int i = 0; i < 10; i++) { Debug.Log(i); }
```

### Range (inclusive)

```prsm
for i in 1..5 {
    print(i)  // 1, 2, 3, 4, 5
}
```
```csharp
for (int i = 1; i <= 5; i++) { Debug.Log(i); }
```

### Descending

```prsm
for i in 10 downTo 1 {
    print(i)  // 10, 9, 8, ..., 1
}
```
```csharp
for (int i = 10; i >= 1; i--) { Debug.Log(i); }
```

### Step

```prsm
for i in 0 until 100 step 5 {
    print(i)  // 0, 5, 10, ..., 95
}
```
```csharp
for (int i = 0; i < 100; i += 5) { Debug.Log(i); }
```

### Foreach

```prsm
for enemy in enemies {
    attack(enemy)
}
```
```csharp
foreach (var enemy in enemies) { Attack(enemy); }
```

### Destructure

```prsm
for (key, value) in dictionary {
    print("$key = $value")
}

for Spawn(pos, delay) in spawns {
    spawnAt(pos, delay)
}
```
```csharp
foreach (var (key, value) in dictionary) { ... }
```

## while

```prsm
while alive {
    update()
}
```
```csharp
while (_alive) { Update(); }
```

## try / catch / finally

```prsm
try {
    val result = riskyOperation()
} catch (e: IOException) {
    log("IO error: ${e.message}")
} catch (e: Exception) {
    log("General error: ${e.message}")
} finally {
    cleanup()
}
```
```csharp
try {
    var result = RiskyOperation();
} catch (IOException e) {
    Log($"IO error: {e.Message}");
} catch (Exception e) {
    Log($"General error: {e.Message}");
} finally {
    Cleanup();
}
```

### throw

```prsm
throw ArgumentException("invalid value")
throw IOException("connection failed")
```
```csharp
throw new ArgumentException("invalid value");
throw new IOException("connection failed");
```

throw expression: `val x = data ?? throw Exception("missing")`

## use (Dispose 자동 호출)

### 선언 형식

```prsm
use val file = openFile("data.txt")
process(file)
// scope 끝에서 file.Dispose() 자동 호출
```
```csharp
using var file = OpenFile("data.txt");
Process(file);
```

### 블록 형식

```prsm
use var conn = openConnection() {
    query(conn)
}
// 블록 끝에서 conn.Dispose() 자동 호출
```
```csharp
using (var conn = OpenConnection()) {
    Query(conn);
}
```

## listen / unlisten

이벤트 구독. 상세: [09-advanced.md](./09-advanced.md) 참조.

기본 사용:

```prsm
listen button.onClick {
    print("clicked!")
}
```
```csharp
button.onClick.AddListener(() => { Debug.Log("clicked!"); });
```

## return / break / continue

```prsm
func find(list: List<Int>, target: Int): Int {
    for i in 0 until list.count {
        if list[i] == target { return i }
    }
    return -1
}

for i in 0 until 100 {
    if i % 2 == 0 { continue }
    if i > 50 { break }
    print(i)
}
```

## wait

코루틴 내부에서만 사용 가능. 상세: [04-functions.md](./04-functions.md) 코루틴 섹션 참조.

```prsm
wait 1.0s           // WaitForSeconds(1.0f)
wait 500ms          // WaitForSeconds(0.5f)
wait nextFrame      // yield return null
wait fixedFrame     // WaitForFixedUpdate()
wait until ready    // WaitUntil(() => ready)
wait while loading  // WaitWhile(() => loading)
```

## start / stop

코루틴 제어:

```prsm
start blink()       // StartCoroutine(Blink())
stop blink          // StopCoroutine("Blink")
stopAll()           // StopAllCoroutines()
```

## Preprocessor

상세: [08-annotations.md](./08-annotations.md) 참조.

```prsm
#if editor
    drawGizmos()
#elif debug
    logDebugInfo()
#else
    // production
#endif
```
