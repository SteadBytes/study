## Reading Syntax

1.

a) Correct.
b) Incorrect: `[1, 2, 3] ++ [4, 5, 6]`
c) Correct.
d) Incorrect: `["hello" ++ " world"]`
e) Incorrect: `"hello" !! 4`
f) Correct.
g) Incorrect: `take 4 "lovely"`
h) Correct.

2.

```haskell
-- a -> d
concat [[1 * 6], [2 * 6], [3 * 6]]
-- [6, 12, 18]

-- b -> c
"rain" ++ drop 2 "elbow"
-- "rainbow"

-- c -> e
10 * head [1, 2, 3]
-- 10

-- d -> a
(take 3 "Julie") ++ (tail "yes")
-- "Jules"

-- e -> b
concat [tail [1, 2, 3]
        tail [3, 4, 5]
        tail [7, 8, 9]]
--- [2, 3, 5, 6, 8, 9]
```

## Building functions

1.


    a)

    ```haskell
    "Curry is awesome" ++ "!"
    -- "Curry is awesome!"
    ```

    b)

    ```haskell
    take 1 (drop 4 "Curry is awesome!")
    -- "y"
    ```

    c)

    ```haskell
    drop 9 "Curry is awesome!"
    -- "awesome!"
    ```

2. 3. 4. 5. See [`building_functions.hs`](./building_functions.hs)

6.  See [`reverse.hs`](./reverse.hs)