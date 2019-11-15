## Parenthesization

1. `2 + (2 * 3) - 1`
2. `(^) 10 (1 + 1)`
3. `((2 ^ 2) * (4 ^ 5)) + 1`

## Equivalent expressions

1. Equivalent
2. Equivalent
3. Not equivalent
4. Not equivalent
5. Not Equivalent

## More fun with functions

```haskell
let z = 7
let y = z + 8
let x = y ^ 2
waxOn = x * 5
```

1.

```haskell
10 + waxOn
-- 1135
(+10) waxOn
-- 1135
(-) 15 waxOn
-- 1110
(-) waxOn 15
-- 1110
```

3.

```haskell
triple waxOn
-- 3375
```

4 onwards:

```haskell
waxOn = x * 5
  where
    z = 7
    y = z + 8
    x = y ^ 2

triple x = x * 3

waxOff x = triple x
```
