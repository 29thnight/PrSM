---
title: Coroutines
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 10
---

# Coroutines

코루틴은 PrSM의 핵심 기능 중 하나입니다.

```prsm
coroutine hitInvincible() {
    invincible = true
    wait invincibleTime.s
    invincible = false
}
```

지원되는 wait 형식:

- `wait 1.0s`
- `wait nextFrame`
- `wait fixedFrame`
- `wait until condition`
- `wait while condition`

코루틴 제어 형식:

- `start coroutineName()`
- `stop coroutineName()`
- `stopAll()`

## 일반 `yield` (PrSM 5 부터)

`wait` 단축 외에도 코루틴은 일반 `yield`와 `yield break` 문장을 사용할 수 있습니다. 같은 형식을 반환 타입이 `Seq<T>`, `IEnumerator`, `IEnumerator<T>`, `IEnumerable`, `IEnumerable<T>`인 `func`에서도 사용할 수 있습니다.

```prsm
coroutine countdown(): Seq<Int> {
    for i in 5 downTo 1 {
        yield i
        wait 1s
    }
    yield 0
    yield break
}

coroutine fadeOut(): IEnumerator {
    var t = 1.0
    while t > 0.0 {
        t -= Time.deltaTime
        canvasGroup.alpha = t.toFloat()
        yield return null
    }
}
```

```csharp
public IEnumerator<int> countdown()
{
    for (int i = 5; i >= 1; i--)
    {
        yield return i;
        yield return new WaitForSeconds(1.0f);
    }
    yield return 0;
    yield break;
}

public IEnumerator fadeOut()
{
    var t = 1.0;
    while (t > 0.0)
    {
        t -= Time.deltaTime;
        canvasGroup.alpha = (float)t;
        yield return null;
    }
}
```

`Seq<T>`는 `IEnumerator<T>`로 변환됩니다. `wait` 문장은 같은 코루틴 본문 안에서 일반 `yield`와 공존합니다. 코루틴/이터레이터 반환 함수 외부의 `yield`는 E147, yield 값 타입이 선언된 요소 타입과 다르면 E148, `Seq<T>` 코루틴이 어떤 `T` 값도 yield하지 않으면 W033이 발생합니다.
