1.  a. `k :: (a, b) -> a`
    b. `k2 :: [Char]`, not the same as `k1` or `k3`
    c. `k1` and `k2`

2. 

```haskell
f :: (a, b, c) -> (d, e, f) -> ((a, d), (c, f))
f (a, _, c) (d, _, f) = ((a, d), (c, f))
```