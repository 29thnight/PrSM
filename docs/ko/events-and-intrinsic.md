---
title: Events & Intrinsic
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 11
---

# Events & Intrinsic

## `listen` — 이벤트 연결

`listen` 키워드는 코드 블록을 Unity `UnityEvent` 또는 `UnityAction` 필드에 연결합니다. 생성된 `Awake()` 안에 `AddListener(...)` 호출로 lowering되며, 컴포넌트 룩업이 모두 완료된 이후에 추가됩니다.

### 기본 사용법

```prsm
listen startButton.onClick {
    SceneManager.LoadScene("Game")
}

listen slider.onValueChanged {
    updateVolume(slider.value)
}
```

### 파라미터 수신

`UnityEvent<T>` 콜백은 이벤트 페이로드를 자동으로 받습니다.

```prsm
listen healthBar.onValueChanged { val newValue ->
    if newValue <= 0.0 {
        triggerDeath()
    }
}
```

### 정리 주의사항

PrSM은 아직 `OnDestroy`에서 자동 `RemoveListener` 호출을 생성하지 않습니다. 수명이 긴 이벤트 소스를 대상으로 할 경우 `onDestroy` 블록 안에서 `intrinsic`으로 직접 정리 코드를 작성하세요.

---

## `intrinsic` — raw C# escape hatch

PrSM 문법이 커버하지 못하는 Unity API나 패턴이 있을 때, `intrinsic`은 컴포넌트 구조를 벗어나지 않고 raw C#을 삽입할 수 있는 탈출구입니다. `intrinsic` 안의 코드는 PrSM 시맨틱 검사의 대상이 아니며, C# 컴파일러에 의해서만 검증됩니다.

### 문장 블록 intrinsic

메서드 바디 안에 raw C# 문장을 인라인으로 삽입합니다.

```prsm
update {
    intrinsic {
        var ray = Camera.main.ScreenPointToRay(Input.mousePosition);
        if (Physics.Raycast(ray, out var hit, 100f)) {
            Debug.DrawLine(ray.origin, hit.point, Color.red);
        }
    }
}
```

### 타입 있는 intrinsic 표현식

raw C# 표현식을 타입이 있는 PrSM 값으로 취급합니다.

```prsm
val hitPoint: Vector3 = intrinsic<Vector3> { hit.point }
```

### Intrinsic 함수

함수 전체 바디를 raw C#으로 선언합니다.

```prsm
intrinsic func getMouseWorldPos(): Vector3 {
    """
    var ray = Camera.main.ScreenPointToRay(Input.mousePosition);
    Physics.Raycast(ray, out RaycastHit hit, 100f);
    return hit.point;
    """
}
```

### Intrinsic 코루틴

raw C# `yield` 문장을 포함하는 코루틴을 선언합니다.

```prsm
intrinsic coroutine fadeOut(): IEnumerator {
    """
    float t = 1f;
    while (t > 0f) {
        t -= Time.deltaTime;
        canvasGroup.alpha = t;
        yield return null;
    }
    """
}
```

`intrinsic`은 드물게 사용하는 탈출구입니다. PrSM 문법을 우회하는 기본 수단으로 사용하지 마세요.
