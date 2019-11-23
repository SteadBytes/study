## Types

Compiler assigns the most polymorphic (broadest applicable) types when performing inference, for example using the `Num` typeclass instead of assigning to `Integer`:

```
Prelude> :type 7
7 :: Num a => a

-- declare a concrete type
Prelude> let x = 13 :: Integer
Prelude> :t x
x :: Integer
```

## Functions

`->` = Function type constructor.

- **No** data constructors
  - Functions are _values_

## Currying

**Curried by default**

- No language level support for functions w/multiple arguments.
  - Syntactic sugar to construct curried functions instead.

`->` type constructor for functions indicates successive function applications.

- Each takes on argument and returns one result
- Outermost layer returns another function that accepts the next argument
- Currying

GHCi can type check unimplemented expressions by binding the signature to `undefined`:

```
Prelude> let f :: a -> a -> a -> a; f = undefined
Prelude> let x :: Char; x = undefined
Prelude> :t f x
```

## Polymorphism

Polymorphic type variables allow for expressions to be implemented that accept and/or return multiple types **without the need for multiple variations of the expressions for each type**.

**Parametric** polymorphism = Fully polymorphic - the final concrete type of _parameters_ (type variables) could be anything.

- i.e. `id :: a -> a`

**Constrained** polymorphism = Possible values are _constrained_ via typeclasses - reducing the set of possible concrete types.

- i.e. `(+) :: Num a => a -> a -> a`
- Increases the possible behaviour of a function by bringing into scope a set of operations

**Principle type** = most generic type that typechecks.
