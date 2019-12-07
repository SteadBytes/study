# Recursion

Using an **identity value** for the _base case_ of a function means applying the function to that case doesn't change the result of previous applications.

- i.e. a base case of `0` for a function using `*`

Recursion can be thought of as _self-referential composition_

- Apply a function to an argument, pass that result on as an argument to a second application of the _same_ function e.t.c
- Relies on _inputs_ to determine the stopping point
  - Unlike normal composition which has a fixed number of applications

```haskell
applyTimes :: (Eq a, Num a) =>
               a -> (b -> b) -> b -> b
applyTimes 0 f b = b
applyTimes n f b = f . applyTimes (n - 1) $ b

-- Write out evaluation
applyTimes 5 (+1) 5
-- (+1) (applyTimes 4 (+1) 5)
-- (+1) ((+1) (applyTimes 3 (+1) 5))
-- (+1) ((+1) ((+1) (applyTimes 2 (+1) 5)))
-- (+1) ((+1) ((+1) ((+1) (applyTimes 1 (+1) 5))))
-- (+1) ((+1) ((+1) ((+1) ((+1) (applyTimes 0 (+1) 5)))))
-- (+1) ((+1) ((+1) ((+1) ((+1) 5)))))
```

## General outline for defining a recursive algorithm

1. Consider the _types_
   ```haskell
   -- we know that the Fibonacci sequence contains positive, whole numbers
   -- the function should generate the nth item in the sequence, so it's input is also positive, whole numbers
   fibonacci :: Integral a => a -> a
   ```
2. Consider the _base case(s)_
   ```haskell
   fibonacci :: Integral a => a -> a
   fibonacci 0 = 0 
   fibonacci 1 = 1
   ```
3. Consider the arguments_
   ```haskell
   fibonacci :: Integral a => a -> a
   fibonacci 0 = 0 
   fibonacci 1 = 1
   -- single argument identifying the member of the sequence to calculate
   -- each member of the sequence is the result of adding the *preceding* two members
   fibonacci x = (x - 1) (x - 2) -- won't work yet
   ```
4. Consider the _recursion_
   ```haskell
   fibonacci :: Integral a => a -> a
   fibonacci 0 = 0 
   fibonacci 1 = 1
   -- add in the recursive call and addition
   fibonacci x = fibonacci (x - 1) + fibonacci (x - 2)
   ```

## Bottom

`⊥` (bottom) refers to computations that **do not** successfully result in a value.

- Failed with an error
  - Deliberately returning `error`
  - _Partial functions_
- Failed to terminate
  - Infinite loop/recursion
- Corresponds to _false_ in logic
