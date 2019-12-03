## Multiple choice

1. d - A polymorphic function may resolve the values of different types, depending on inputs.
2. b - Two functions named `f` and `g` have types `Char -> String` and `String -> [String]` respectively. The composed function `g . f` has the type `Char -> [String]`
3. d - A function `f` has the type `Ord a => a -> a -> Bool` and we apply it to one numeric value. What is the type now? `(Ord a, Num a) => a -> Bool`
4. b - A function with the type `(a -> b) -> c` is a higher-order function.
5. a - Given the following definition of `f`, what is the type of `f True`? `f True :: Bool`

```haskell
f :: a -> a
f x = x
```
