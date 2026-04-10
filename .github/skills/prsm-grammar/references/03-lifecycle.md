# Lifecycle Blocks

Unity MonoBehaviour 라이프사이클 메서드를 PrSM에서 블록 문법으로 선언한다.

## awake

```prsm
awake {
    print("initialized")
}
```
```csharp
private void Awake() {
    Debug.Log("initialized");
}
```

`require`, `optional`, `child`, `parent` 필드의 `GetComponent` 호출도 Awake에 합성된다.

## start

```prsm
start {
    rb.velocity = vec3(0, 1, 0)
}
```
```csharp
private void Start() {
    _rb.velocity = new Vector3(0, 1, 0);
}
```

## update

```prsm
update {
    transform.position += vec3(speed * Time.deltaTime, 0, 0)
}
```
```csharp
private void Update() {
    transform.position += new Vector3(_speed * Time.deltaTime, 0, 0);
}
```

## fixedUpdate

```prsm
fixedUpdate {
    rb.addForce(vec3(0, -9.81, 0))
}
```
```csharp
private void FixedUpdate() {
    _rb.AddForce(new Vector3(0, -9.81f, 0));
}
```

## lateUpdate

```prsm
lateUpdate {
    camera.position = target.position + offset
}
```
```csharp
private void LateUpdate() {
    _camera.position = _target.position + _offset;
}
```

## onEnable / onDisable

```prsm
onEnable {
    EventBus.subscribe(this)
}

onDisable {
    EventBus.unsubscribe(this)
}
```
```csharp
private void OnEnable() { EventBus.Subscribe(this); }
private void OnDisable() { EventBus.Unsubscribe(this); }
```

`listen ... until disable`의 cleanup 코드도 OnDisable에 합성된다.

## onDestroy

```prsm
onDestroy {
    pool.release()
}
```
```csharp
private void OnDestroy() { _pool.Release(); }
```

## onTriggerEnter / Exit / Stay

매개변수가 있는 라이프사이클 블록:

```prsm
onTriggerEnter(other: Collider) {
    if other.tag == "Player" {
        activate()
    }
}

onTriggerExit(other: Collider) {
    deactivate()
}

onTriggerStay(other: Collider) {
    damage(other.gameObject)
}
```
```csharp
private void OnTriggerEnter(Collider other) {
    if (other.tag == "Player") { Activate(); }
}
private void OnTriggerExit(Collider other) { Deactivate(); }
private void OnTriggerStay(Collider other) { Damage(other.gameObject); }
```

## onCollisionEnter / Exit / Stay

```prsm
onCollisionEnter(collision: Collision) {
    val force = collision.relativeVelocity.magnitude
    if force > 10 {
        takeDamage(force.toInt())
    }
}
```
```csharp
private void OnCollisionEnter(Collision collision) {
    var force = collision.relativeVelocity.magnitude;
    if (force > 10) { TakeDamage((int)force); }
}
```

## 주의사항

- 하나의 component에 같은 라이프사이클 블록을 여러 개 선언할 수 없다.
- `require`/`optional`/`child`/`parent` 필드의 GetComponent 코드는 사용자 `awake` 블록 **앞에** 합성된다.
- `listen ... until disable`의 RemoveListener 코드는 사용자 `onDisable` 블록 **뒤에** 합성된다.
