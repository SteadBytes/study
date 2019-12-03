_Data constructor_ = Function to _construct_ a value of a given type

_Pattern matching_ = Match values against patterns and _bind_ variables to successful matches. Patterns can make use of unpacking to 'deconstruct' values.

```haskell
-- constructs a Colour value
data Colour = RGB (Int, Int, Int) -- constructs an RGB value
            | HSL (Int, Int, Int) -- constructs an HSL value
-- pattern match on each Colour type by unpacking
instance Show Colour where
    show (RGB a) = "rgb" ++ (show a)
    show (HSL a) = "hsl" ++ (show a)
    -- could further unpack the tuple if desired
    -- show (RGB (r, g, b)) = "rgb(" ++ show r ++ "," show g ++ "," ++ show b ++ ")"
```

## Case expressions

Control flow mechanism for choosing a different return value for a function based on inputs

- `if-then-else` is another such construct, though the two are _not_ equivalent

Following two functions are equivalent:

```haskell
-- using if-then-else construct
funcIf x = x + 1 == 1 then "yep" else "nope"

-- using case expression
funcCase x =
    case x + 1 == 1 of
        True -> "yep"
        False -> "wut"
```

## Function composition

Higher order function that allows combining of multiple functions such that the result of applying one function is passed to the next as an argument.

`.` operator

```haskell
(f . g) x = f (g x)
```

**Pointfree style** = Function composition without specifying arguments

```
Prelude> let f = negate . sum
Prelude> f [1..5]
-15
```

See [`arith2.hs`](./arith2.hs)
