1. a `f x :: Char -> Char -> Char`
2. d `g 0 'c' "woot" :: Char`
3. d `h 1.0 2 :: Num b => b`
4. c `h 1 (5.5 :: Double) :: Double`
5. a `jackal "keyboard" "has the word jackal in it" :: [Char]`
6. e `jackal "keyboard" :: Eq b => b -> [Char]`
7. d `kessel 1 2 :: (Ord a, Num a) => a`
   - I think there's an error in the book where `(Ord a, Num a)` is the other way round.
8. a `kessel 1 2 :: (Ord a, Num a) => a`
9. c `kessel (1 :: Integer) 2 :: Integer`
