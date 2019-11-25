## Typeclasses

Type/Typeclass relationship:

- Type defines how a type in particular is _created_
- Typeclass defines how a set of types are _consumed_
  - Used in computations
  - Generalise over a set of types
    - Define & execute a set of standard features

`instance` keyword declares a typeclass instance:

```haskell
data Foo = Foo'

instance Eq Foo where
    Foo' == Foo' = True
    -- *Or* using prefix notation (can't have both at once)
    (==) Foo' Foo' = True
```

Cannot have multiple of the same typeclass instances for a given type.

- i.e. defining `Eq` twice will not compile

Tuple syntax in typeclasses denote _conjunction_ of typeclass constraints i.e. `Integral` requires a type to already have instances of both `Real` and `Enum`:

```haskell
class (Real a, Enum a) => Integral a where
-- ...
```

Typeclass inheritance is **additive only**.

- Cannot override methods provided by another typeclass
  - i.e. `Real` cannot override `(+)` from `Num`
- Avoid issues caused by multiple inheritance

## Partial functions

**Not** partial _application_

Functions which do not handle **all possible cases**

The following will fail at _runtime_ for any input values other than `0` or `1`.

```haskell
-- partial.hs
f :: Int -> Bool
f 0 = False
f 1 = True
```

```
Prelude> f 2
*** Exception: <interactive>:20:23-45: Non-exhaustive patterns in function f
```

Use `Wall` complier flag to generate warnings at compile time:

```
Prelude=> :set -Wall
Prelude=> :l partial.hs
<interactive>:23:23: warning: [-Wincomplete-patterns]
    Pattern match(es) are non-exhaustive
    In an equation for ‘f’:
        Patterns not matched: p where p is not one of {1, 0}
```

Must add an **unconditional case** to make the function **total**:

```haskell
f :: Int -> Bool
f 0 = False
f 1 = True
f _ = False
```

Use **sum types** to represent a constrained set of values.

- Reduces the number of cases that functions must handle
- i.e. don't use `Int` as an _implicit_ sum type (often used in C)

## Type-defaulting typeclasses

Typeclasses _default_ to a concrete type when a typeclass-constrained polymorphic value is evaluated without a concrete type being specified.

For example `Fractional` defaults to `Double`:

```
Prelude> :t 1 / 2
0.5
```

Numeric typeclass defaults:

```haskell
default Num Integer
default Real Integer
default Enum Integer
default Integral Integer
default Fractional Double
default RealFrac Double
default Floating Double
default RealFloat Double
```
