1. c; A value of type `[a]` is a list whose elements are all of some type _a_.
2. a; A function of type `[[a]] -> [a]` could transform a character into a string.
3. b; A function of type `[a] -> Int -> a` returns one element of type _a_ from a list.
4. A function of type `(a, b) -> a` takes a tuple argument and returns the first value.

## Determine the type

See [`determineTheType.hs`](./determineTheType.hs)

1.
a. value = `54`, type = `Num a => a`
b. value = `(0, "doge")`, type = `Num a => (a, [Char])`
c. value = `(0, "doge")`, type = `(Integer, [Char])`
d. value = `False`, type = `Bool`
e. value = `5`, type = `Integer`
f. value = `False`, type = `Bool`

2. `w :: Num a => a`
3. `z:: Num a => p -> a`
4. `f:: Fractional a => a`
5. `f :: [Char]`

## Does it compile?

1. Compiles.
2. Doesn't compile: `($)` is an _infix_ operator where it's first argument is a function and the second argument is a value to apply to that function. Here, it's first argument is a `Num` value and no second argument is provided. Fix by making the first argument to `$` a function:

```haskell
-- note that the $ operators here are both redundant
bigNum = (^) 5 $ 10
wahoo = (bigNum*) $ 10
```

3. Doesn't compile: `c` and `d` try to apply arguments to `Num` values.

```haskell
a = (+)
b = 5
c = a b 10
d = a c 200
```

4. Doesn't compile. Variables `b` is used before it's defined (only matters in GHCi) and `c` doesn't exist at all.

```haskell
c = 1
b = 10000 * c
a = 12 + b
```

## Type variable or specific type constructor?

Notation:

- `0` = constrained polymorphic
- `1` = fully polymorphic
- `2` = concrete

1.

```haskell
f :: zed -> Zed -> Blah
--    1      2       2
```

2.

```haskell
f :: Enum b => a -> b -> c
--             1    0    1
```

3.

```haskell
f :: f -> g -> c
--   1    1    1
```

## Write a type signature

1.

```haskell
functionH :: [a] -> a
functionH (x:_) = x
```

2.

```haskell
functionC :: Ord a => a -> a -> Bool
functionC x y = if (x > y) then True else False
```

3.

```haskell
functionS :: (a, b) -> b
functionS (x, y) = y
```

## Given a type, write the function

1. This is the _identity_ function

```haskell
i :: a -> a
i x = x
```

2. This is essentially _identity_ with two arguments where the second is ignored.

```haskell
c :: a -> b -> a
c x y = x
```

3. Yes, `c` and `c''` are the same thing.

```haskell
c'' :: b -> a -> b
c'' x y = x
```

4.

```haskell
c' :: a -> b -> b
c' x y = y
```

5.

```haskell
r :: [a] -> [a]
r xs = xs -- id
-- or
r xs = f xs
  where f :: [a] -> [a]
        f = take 2 -- could be take, drop, tail e.t.c
```

6.

```haskell
co :: (b -> c) -> (a -> b) -> a -> c
co bToC aToB a = bToC $ aToB a
```

7.

```haskell
a :: (a -> c) -> a -> a
a _ a = a
```

8.

```haskell
a' :: (a -> b) -> a -> b
a' aToB a = aToB a
```

## Fix it

1. [`sing1.hs`](./sing1.hs)
2. [`sing2.hs`](./sing2.hs)
3. [`arith3broken.hs`](./arith3broken.hs)

## Type-Kwon-Do

[`typeKwonDo.hs`](./typeKwonDo.hs)
