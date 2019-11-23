module Arith3Broken where

-- function names start with lower case
main :: IO ()
main = do
    -- attempting to add 2 to IO ()
    -- evaluate 1 + 2 before print
    print $ 1 + 2
    -- Num must be cast to String
    putStrLn $ show 10
    -- attempting to apply 1 to negation of - operator
    -- negate -1 :: (Num a, Num (a -> a)) => a -> a
    print (negate $ -1)
    print ((+) 0 blah)
      where blah = negate 1