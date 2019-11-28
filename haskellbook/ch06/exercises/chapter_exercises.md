## Multiple choice

1. c - The `Eq` class makes equality tests possible.
2. b - The typeclass `Ord` is a subclass of `Eq`
3. a - `(>) :: Ord a => a -> a -> Bool`
4. c

```
Prelude> divMod 16 12
(1, 4)
```

5. a - The typeclass `Integral` includes `Int` and `Integer` numbers

## Does it typecheck?

1. No - `Person` has no `Show` instance.

```haskell
data Person = Person Bool deriving Show

printPerson :: Person -> IO ()
printPerson person = putStrLn (show person)
```

2. No, `Mood` does not have an `Eq` instance to support the `if` statement in `settleDown`.

```haskell
data Mood = Blah
          | Woot deriving (Show, Eq)

settleDown x = if x == Woot
                then Blah
                else x
```

3.  a. `Blah` or `Woot`
    b. Error as `Mood` does not have an instance of `Num`
    c. Error as `Mood` has no `Ord` typeclass (required by `(>)`)

4.  Yes. Though I'll note that `s1` is only partially applied, requiring a value for `Object` - attempting to print `s1`, for example, would result in an error.

## Given a datatype declaration, what can we do?

See [`what_can_we_do.hs`](./what_can_we_do.hs)

## Match the types

1. Cannot substitute, the assignment `i = 1` has type `i :: Num p => p`.
2. Can substitute, `Float` is a subclass of `Num` so the value `f = 1.0` matches both typeclasses.
3. Can substitute, `Float` has a `Fractional` instance.
4. Can substitute, `RealFrac` has a `Float` instance.
5. Can substitute, specifying `Ord a => a ->a` is increasing the _specificity_ of the original parametrically polymorphic definition (which was equivalent to `id`).
6. Can substitute, same as 5.
7. Cannot substitute, `sigmund` expects to return any type, but `myX` is _always_ an `Int`.
8. Cannot substitute, `sigmund'` expects to return _any_ value of `Num`, but (again) `myX` is _always_ an `Int`.
9. Can substitute, `Int` has an `Ord` instance to allow the `sort` call.
10. Can substitute, `Ord a => [a] -> a` is a _less_ constrained version.
11. Cannot substitute, `mySort` specifies a `[Char]` argument type, whereas `Ord [a] -> a` will allow a list of _any_ type.

## Type-Kwon-Do Two: Electric Typealoo

See [`type_kwon_do.hs`](./type_kwon_do.hs)