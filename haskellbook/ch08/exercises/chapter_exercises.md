## Review of types

1. d - `[[True, False], [True, True], [False, True]] :: [[Bool]]`
2. b - `[[3 == 3], [6 > 5], [3 < 4]] :: [[Bool]]`
3. d
4. b

## Reviewing currying

See [`reviewing_currying.hs`](./reviewing_currying.hs)

## Recursion

1.

```haskell
-- definition given in text
dividedBy :: Integral a => a -> a -> (a, a)
dividedBy num denom = go num denom 0
    where go n   d count
           | n < d
           | otherwise = go (n - d) d (count + 1)

-- steps for reducing dividedBy 15 2
dividedBy 15 2 =
go 15 2 0
  | 15 < 2 = ...
  -- false, skip this branch
  | otherwise = go (15 - 2) 2 (0 + 1)
      go 13 2 1
          -- 13 >= 2 -> otherwise branch
          go (13 - 2) 2 (1 + 1)
              go 11 2 2
                      -- 11 >= 2 -> otherwise branch
                      go (11 - 2) 2 (2 + 1)
                          go 9 2 3
                              -- 9 >= 2 -> otherwise branch
                              go (9 - 2) 2 (3 + 1)
                                  go 7 2 4
                                      -- 7 >= 2 -> otherwise branch
                                      go (7 - 2) 2 (4 + 1)
                                          go 5 2 5
                                              -- 5 >= 2 -> otherwise branch
                                              go (5 - 2) 2 (5 + 1)
                                                  go 3 2 6
                                                  -- 3 >= 2 -> otherwise branch
                                                  go (3 - 2) 2 (6 + 1)
                                                      go 1 2 7
                                                          -- 1 < 2 -> evaluate n < d branch
                                                          | 1 < 2 = (7, 1)
```

2. See [`recursion.hs`](./recursion.hs)