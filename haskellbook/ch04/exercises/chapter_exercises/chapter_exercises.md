1. `length :: [a] -> Int`
2.

```haskell
length [1, 2, 3, 4, 5]
-- 5
length [(1, 2), (2, 3), (3, 4)]
-- 3
length allAwesome
-- 2
length (concat allAwesome)
-- 5
```

3. `6 / 3` works, `6 / length [1, 2, 3]` returns an error.

- `/` requires both arguments to be `Fractional`
- `length [1, 2, 3]` returns an `Int`

4. `div 6 $ length [1, 2, 3]`
5. `Bool`, `True`
6. `Bool`, `False`
7. 
```haskell
length allAwesome == 2
-- True

-- Won't compile due to heterogeneous List types
length [1, 'a', 3, 'b']

length allAwesome + length awesome
-- 5
(8 == 8) && ('b' < 'a')
-- False

-- Won't compile due to 9 not being Bool
(8 == 8) && 9
```

8. 9. 10. See [`chapter_exercises.hs`](./chapter_exercises.hs) 

## Match the function names to their types

1. c
2. b
3. a
4. d