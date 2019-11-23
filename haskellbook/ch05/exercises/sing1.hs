-- module names must be capitalised
module Sing where

-- ++ operator cannot be in a type signature
fstString :: [Char] -> [Char]
fstString x = x ++ " in the rain"

-- function declaration returns String not a single Char
sndString :: [Char] -> [Char]
sndString x = x ++ " over the rainbow"

-- 'or' -> 'else"
sing = if (x > y) then fstString x else sndString y
  where x = "Singin"
        y = "Somewhere" -- x declared twice