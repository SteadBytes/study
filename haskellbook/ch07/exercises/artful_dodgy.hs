module ArtfulDodgy where

dodgy x y = x + y * 10

oneIsOne = dodgy 1

oneIsTwo = (flip dodgy) 2

main :: IO ()
main = do
  mapM_
    print
    [ dodgy 1 0,
      dodgy 1 1,
      dodgy 2 2,
      dodgy 1 2,
      dodgy 2 1,
      oneIsOne 1,
      oneIsOne 2,
      oneIsTwo 1,
      oneIsTwo 2,
      oneIsOne 3,
      oneIsTwo 3
    ]