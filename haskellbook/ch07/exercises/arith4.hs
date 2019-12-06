module Arith4 where

roundTrip :: (Show a, Read a) => a -> a
roundTrip a = read (show a)

roundTripPf :: (Show a, Read a) => a -> a
roundTripPf = read . show

roundTrip' :: (Show a, Read b) => a -> b
roundTrip' a = read (show a)


main = do
    print (roundTrip 4)
    print (roundTripPf 4)
    print (roundTrip' 4 :: Int)
    print (id 4)