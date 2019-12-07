module Recursion where

-- 2

sumToN :: (Eq a, Num a) => a -> a
sumToN 0 = 0
sumToN x = x + sumToN (x - 1)
 
-- or using go

sumToN' :: (Eq a, Num a) => a -> a
sumToN' x = go x 0
    where go x acc
           | x == 0 = acc
           | otherwise = go (x - 1) (acc + x)

-- 3
mul :: (Integral a) => a -> a -> a
mul x 0 = 0
mul x y = x + mul x (y - 1)