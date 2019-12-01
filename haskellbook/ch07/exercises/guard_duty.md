1. Placing the `otherwise` case first will cause that branch to _always_ be executed. In this case, the result will always be `'F'`.

```haskell
avgGrade :: (Fractional a, Ord a) => a -> Char
avgGrade x
    | otherwise 'F'
    | y >= 0.9 = 'A'
    | y >= 0.8 = 'B'
    | y >= 0.7 = 'C'
    | y >= 0.59 = 'D'
    where y = x / 100
```

2. No, moving the `'C'` case before `'A'` will cause any score `>= 0.7` to return `'C'`, instead of scores between `0.7` and `0.9`

```haskell
avgGrade :: (Fractional a, Ord a) => a -> Char
avgGrade x
    | y >= 0.7 = 'C'
    | y >= 0.9 = 'A'
    | y >= 0.8 = 'B'
    | y >= 0.59 = 'D'
    | y < 0.59 = 'F'
    where y = x / 100
```

3. b
4. Any array of containing types with an `Eq` instance.
5. `pal :: Eq a=> [a] -> Bool`
6. c
7. `Num` types which have an `Ord` instance.
8. `numbers :: (Ord a, Num a, Num p)=> a -> p`