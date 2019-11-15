{-
    1. let x = 5 in x
        -- 5
    2. let x = 5 in x * x
        -- 25
    3. let x = 5; y = 6 in x * y
        -- 30 
    4. let x = 3; y = 1000 in x + 3
        -- 6
-}

five = x
  where x = 5

twentyFive = x * x
  where
    x = 5

thirty = x * y
  where
    x = 5
    y = 6

six = x + 3
  where
    x = 3
    y = 1000