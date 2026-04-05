---
title: Lifecycle
parent: Language Guide
grand_parent: English Docs
nav_order: 9
---

# Lifecycle

PrSM gives lifecycle methods first-class syntax inside `component` declarations. Instead of overriding a method from `MonoBehaviour`, you write a named block directly in the component body. The compiler generates the corresponding Unity method and wires it up correctly.

## Supported lifecycle blocks

| Block | Generated Unity method | When it runs |
|---|---|---|
| `awake` | `Awake()` | Before `Start`, at object instantiation |
| `start` | `Start()` | Before the first frame update |
| `update` | `Update()` | Every frame |
| `fixedUpdate` | `FixedUpdate()` | Every physics tick |
| `lateUpdate` | `LateUpdate()` | After all `Update` calls complete |
| `onEnable` | `OnEnable()` | When the component becomes enabled |
| `onDisable` | `OnDisable()` | When the component is disabled |
| `onDestroy` | `OnDestroy()` | Before the object is destroyed |
| `onTriggerEnter` | `OnTriggerEnter(Collider other)` | Trigger overlap begins |
| `onTriggerExit` | `OnTriggerExit(Collider other)` | Trigger overlap ends |
| `onTriggerStay` | `OnTriggerStay(Collider other)` | Trigger overlap continues |
| `onCollisionEnter` | `OnCollisionEnter(Collision other)` | Physics collision begins |
| `onCollisionExit` | `OnCollisionExit(Collision other)` | Physics collision ends |
| `onCollisionStay` | `OnCollisionStay(Collision other)` | Physics collision continues |

## Basic example

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

## Lookup ordering in `awake`

When a `component` uses `require`, `optional`, `child`, or `parent` field qualifiers, the compiler generates an `Awake()` that resolves all of those lookups **before** the user-written `awake` body runs. Any `require` field is guaranteed to be non-null by the time your code executes:

```prsm
component WeaponHolder : MonoBehaviour {
    require weapon: Weapon
    optional shield: Shield?
    child muzzle: Transform

    awake {
        // weapon, shield, muzzle are all already resolved here
        weapon.initialize()
    }
}
```

## Collision and trigger callbacks

Collision and trigger blocks receive the other party as a named parameter:

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

The parameter type is inferred from the generated method signature — `Collider` for trigger events and `Collision` for collision events.
