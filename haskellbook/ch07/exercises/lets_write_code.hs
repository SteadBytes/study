module LetsWriteCode where

-- 1

tensDigit :: Integral a => a -> a
tensDigit x = d
    where xLast = x `div` 10
          d     = xLast `mod` 10

-- a)
tensDigit' x = d
    where (xLast, _) = divMod x 10
          (_, d) = divMod xLast 10

-- b) Yes, both have the same type

-- c)
hunsD x = d
    where (xLast, _) = divMod x 100
          (_, d) = divMod xLast 10

-- 2

foldBool :: a -> a -> Bool -> a
foldBool x y b
    | b = x
    | otherwise = y

foldBool3 :: a -> a -> Bool -> a
foldBool3 x _ True = x
foldBool3 _ y False = y

-- 3

g :: (a -> b) -> (a, c) -> (b, c)
g f (a, c) = (f a, c)


-- see arith4.hs for questions 4 onwards