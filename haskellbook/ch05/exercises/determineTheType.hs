{-# LANGUAGE NoMonomorphismRestriction #-}

module DetermineTheType where

oneA = (* 9) 6
oneB = head [(0, "doge"), (1, "kitteh")]
oneC = head [(0 :: Integer, "doge"), (1, "kitteh")]
oneD = if False then True else False
oneE = length [1, 2, 3, 4, 5]
oneF = (length [1, 2, 3, 4]) > (length "TACOCAT")

w = y * 10
  where x = 5
        y = x + 5

z y = y * 10
  where x = 5
        y = x + 5

f = 4 / y
  where x = 5
        y = x + 5

f' = x ++ y ++ z
  where x = "Julie"
        y = " < 3"
        z = "Haskell"
