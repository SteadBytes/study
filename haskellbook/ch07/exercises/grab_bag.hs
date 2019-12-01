module GrabBag where

-- 1
-- All equivalent
mTh x y z = x * y * z
mTh' x y = \z -> x * y * z
mTh'' x = \z -> \y -> x * y * z
mTh''' x = \x -> \y -> \z -> x * y * z

-- 2
x = (mTh 3) :: Num a => a -> a -> a -- Option d

-- 3a
addOneIfOdd n = case odd n of
    True -> f n
    False -> n
    where f = \n -> n + 1
    
-- 3b
addFive = \x -> \y -> (if x > y then y else x) + 5

-- 3c
mflip f x y = f y x
