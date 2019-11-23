1. It's impossible, not much to show here! `a -> a` provides no concrete information about the value of it's argument. All we know from the signature is that the return type is the same as the input type. This means that we cannot know which functions are supported by the input value in order to produce a new value. The only thing that can be done is to simply return the input value.

For example, in the following definition, the type signature contains insufficient information to determine that _all_ possible values of `a` support the `+` operator. Passing an `Integer` would work, whereas passing a `String` would not although both of these cases are supported by the type signature. As such, it won't compile.

```haskell
f :: a -> a
f x = x + x
```

2.

```haskell
f :: a -> a -> a
f x y = x
-- OR
f x y = y
```

3. There is **one** possible implementation of `a -> b -> b` and it's behaviour doesn't change when the types of `a` and `b` change. It's essentially `id` that ignores the first argument.

```haskell
f :: a -> b -> b
f x y = y
```
