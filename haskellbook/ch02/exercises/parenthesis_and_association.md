1. Not equivalent. `*` has a higher precedence than `+`, so `a` is evaluated as `8 + (7 * 9)`

```haskell
8 + 7 * 9
-- 71
(8 + 7) * 9
-- 135
```

2. Equivalent. Since `*` has higher precedence than `+`, the multiplications on either side of the `+` are evaluated before applying the `+`.

```haskell
perimeter x y = (x * 2) + (y * 2)

perimeter 2 3
-- 10

perimeter x y = x * 2 + y * 2

perimeter 2 3
-- 10
```

3. Not equivalent. `/` has higher precedence than `+`, so `a` is equivalent to `(x / 2) + 9`.

```haskell
f x = x / 2 + 9

f 5
-- 11.5

f x = x / (2 + 9)
-- 0.45454545454545453
```
