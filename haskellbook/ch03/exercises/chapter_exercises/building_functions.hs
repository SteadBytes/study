module BuildingFunctions where

strConcat :: String -> String -> String
strConcat x y = x ++ y

takeNth :: Int -> String -> String
takeNth n str = take 1 (drop n str)

dropN :: Int -> String -> String
dropN n str = drop n str

thirdLetter :: String -> Char
thirdLetter x = x !! 2

letterIndex :: Int -> Char
letterIndex x = "Curry is awesome!" !! x

-- Only works for "Curry is awesome" as per exercise specification
rvrs :: String -> String
rvrs x = drop 9 x ++ take 4 (drop 5 x) ++ take 5 x

main :: IO ()
main = do
    print s'
    print $ takeNth 4 s'
    print $ dropN 9 s'
    print $ thirdLetter s
    print $ letterIndex 2
    print $ rvrs s
    where s = "Curry is awesome"
          s' = strConcat s "!"