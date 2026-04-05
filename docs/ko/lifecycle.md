---
title: Lifecycle
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 9
---

# Lifecycle

PrSM은 `component` 선언 안에서 라이프사이클 메서드를 1급 문법으로 제공합니다. `MonoBehaviour` 메서드를 override하는 대신, 컴포넌트 바디에 이름 있는 블록을 직접 작성하면 컴파일러가 해당 Unity 메서드를 생성하고 연결합니다.

## 지원 라이프사이클 블록

| 블록 | 생성되는 Unity 메서드 | 실행 시점 |
|---|---|---|
| `awake` | `Awake()` | `Start` 이전, 오브젝트 생성 시 |
| `start` | `Start()` | 첫 프레임 업데이트 직전 |
| `update` | `Update()` | 매 프레임 |
| `fixedUpdate` | `FixedUpdate()` | 물리 업데이트마다 |
| `lateUpdate` | `LateUpdate()` | 모든 `Update` 호출 이후 |
| `onEnable` | `OnEnable()` | 컴포넌트 활성화 시 |
| `onDisable` | `OnDisable()` | 컴포넌트 비활성화 시 |
| `onDestroy` | `OnDestroy()` | 오브젝트 파괴 직전 |
| `onTriggerEnter` | `OnTriggerEnter(Collider other)` | 트리거 충돌 시작 |
| `onTriggerExit` | `OnTriggerExit(Collider other)` | 트리거 충돌 종료 |
| `onTriggerStay` | `OnTriggerStay(Collider other)` | 트리거 충돌 지속 |
| `onCollisionEnter` | `OnCollisionEnter(Collision other)` | 물리 충돌 시작 |
| `onCollisionExit` | `OnCollisionExit(Collision other)` | 물리 충돌 종료 |
| `onCollisionStay` | `OnCollisionStay(Collision other)` | 물리 충돌 지속 |

## 기본 예시

```prsm
component Enemy : MonoBehaviour {
    require rb: Rigidbody
    val speed: Float = 3.0

    awake {
        rb.useGravity = true
    }

    update {
        patrol()
    }

    onDisable {
        stopAll()
    }

    onDestroy {
        EventBus.onEnemyDied.Invoke()
    }
}
```

## `awake` 안에서의 룩업 순서

`component`가 `require`, `optional`, `child`, `parent` 필드 한정자를 사용하면, 컴파일러가 생성하는 `Awake()`는 사용자가 작성한 `awake` 바디보다 **먼저** 모든 룩업을 완료합니다. 따라서 `require` 필드는 코드 실행 시점에 이미 non-null이 보장됩니다.

```prsm
component WeaponHolder : MonoBehaviour {
    require weapon: Weapon
    optional shield: Shield?
    child muzzle: Transform

    awake {
        // weapon, shield, muzzle 은 이미 모두 해결된 상태
        weapon.initialize()
    }
}
```

## 충돌·트리거 콜백

충돌·트리거 블록은 상대방 오브젝트를 이름 있는 파라미터로 받습니다.

```prsm
onTriggerEnter(other) {
    if other.CompareTag("Pickup") {
        collect(other.gameObject)
    }
}

onCollisionEnter(col) {
    if col.relativeVelocity.magnitude > hardLandingThreshold {
        playLandFX()
    }
}
```

파라미터 타입은 생성 메서드 시그니처에서 추론됩니다. 트리거 이벤트는 `Collider`, 충돌 이벤트는 `Collision`입니다.
