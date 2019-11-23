module CorrectingSyntax where

-- 1
-- Problems: Upper case function name, quotes instead of backticks around x

x = (+)

f xs = w `x` 1
    where w = length xs

-- 4
first (a, b) = a

main :: IO ()
main = do
    -- 1
    print $ f [1, 2, 3, 4]

    -- 2
    -- Problems: Upper case argument treated as data constructor
    print $ (\ x -> x) 1

    -- 3
    -- Problems: Missing (x:xs) pattern matching
    print $ (\ (x:xs) -> x) [1, 2, 3]

    -- 4
    print $ first (1, 2)