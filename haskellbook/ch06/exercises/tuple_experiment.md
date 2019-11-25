`quotRem` returns a tuple of `(quotient, remainder)` from integral division **truncated towards 0**. For `quotRem x y`

```haskell
quotRem 20 4
-- (5, 0)

quotRem 4 20
-- (0, 4)

quotRem (-10) 6
-- (-2, -4)

quotRem 10 (-6)
-- (-1, 4)
```

`divMod` returns a tuple of `(quotient, modulo)` from integral division **truncated towards -ve infinity**. For `divMod x y`

```haskell
divMod 20 4
-- (5, 0)

divMod 4 20
-- (0, 4)

divMod (-10) 6
-- (-2, 2)

divMod 10 (-6)
-- (-2, -2)
```


## Fractional

The type didn't requite both typeclasses to be explicitly stated because `Fractional` already satisfies the `Num` constraint. `Fractional` is a subset of the `Num` typeclass and so all instances of `Fractional` must also be instances of `Num`, it is therefore redundant to specify both.