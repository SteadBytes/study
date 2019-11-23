**Redexes** = Reducible expressions i.e. `1 + 1`.

## Functions

Functions are **automatically curried**.

- _All_ functions actually take **one argument** and return **one result**.
- Haskell is applying a series of _nested_ functions when is seems that a function is taking multiple arguments.

Function names **must** start with lower case.

## Evaluation

**Lazy evaluation** - defers evaluation of terms until they're forced by other terms referring to them.

Values are irreducible expressions.

Applications of function to arguments are redexes.

Application _is_ evaluation

- Applying a function to an argument -> evaluation/reduction.
- Same as lambda calculus.

## Infix operators

Functions default to _prefix syntax_

Operators are _infix_

- `+`, `*`, `/` e.t.c

Alphanumeric function name = _prefix_ by default

Symbolic function = _infix_ by default

Some prefix functions (not all) can be used infix by wrapping the function name in backticks:

```haskell
10 `div` 4
-- 2
div 10 4
-- 2
```

Infix functions can be made prefix by wrapping in parenthesis:

```haskell
(+) 100 100
-- 200
100 + 100
-- 200
```

## Laws for quotients and remainders

```haskell
(quot x y)*y + (rem x y) == x
(div x y)*y + (mod x y) == x
```

## Using `mod`

In Haskell, if one or both arguments are negative, the results of `mod` will have the **same sign as the divisor**

- Whereas `rem` will have the same sign as the dividend

```haskell
(-5) `mod` 2
-- 1
5 `mod` (-2)
-- -1
(-5) `mod` (-2)
-- -1

(-5) `rem` 2
-- -1
5 `rem` (-2)
-- 1
(-5) `rem` (-2)
-- -1
```

## Parenthesization

`$` allows everything to the _right_ of it to be evaluated _first&_ and can be used to delay function application.

```haskell
(2^) $ (+2) $ (3*2)
-- 256
```

**Sectioning** = Partial application of an _infix_ function, when the returned function is called with an argument it is applied to whichever side was left blank:

```haskell
(+1) 2
-- 3
(1/) 2
-- 0.5
(/1) 2
-- 2.0
```

## Let and where

Introduce components of expressions.

`let` introduces an _expression_

`where` is a _declaration_ bound to a surrounding syntactic construct

```haskell
-- FunctionWithWhere.hs
module FunctionWithWhere where

printInc n = print plusTwo
    where plusTwo = n + 2

-- FunctionWithLet.hs
module FunctionWithLet where

printInc2 n = let plusTwo = n + 2
             in print plusTwo
```

## Definitions

**Parameter** (formal parameter) = Value that _will_ be passed to a function when the function is called.

**Argument** = _Input_ value a function is applied to; a function's parameter is bound to the value of an argument when the function is applied to that argument.

**Expression** = combination of symbols that conforms to syntactic rules and can be evaluated to some result.

**Function** = Mathematical object whose capabilities are limited to being applied to an argument and returning a result.

- Can be described as a list of ordered pairs of inputs and resulting outputs (mapping)
  - `f x = x + 2` applied to `2` would give ordered pair `(2, 4)`
