1. Doesn't compile; `++` is an _infix_ operator.

   - `[1, 2, 3] ++ [4, 5, 6]`
   - `(++) [1, 2, 3] [4, 5, 6]`

2. Doesn't compile; single quotes denote `Char`, double quotes are needed for `String`.
   - `"<3" ++ " Haskell"`
3. Compiles.
