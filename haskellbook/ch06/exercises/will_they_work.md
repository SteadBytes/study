1. Yes. `length` returns an `Int` which has an `Ord` instance:

```
Prelude> max (length [1, 2, 3]) (length [8, 9, 10, 11, 12])
5
Prelude> :t length
length:: Foldable t => t a -> Int
Prelude> :info Int
data Int = GHC.Types.I# GHC.Prim.Int#   -- Defined in ‘GHC.Types’
instance Eq Int -- Defined in ‘GHC.Classes’
instance Ord Int -- Defined in ‘GHC.Classes’
-- some more instances
```

2. Yes. `(*)` has a constraint of `Num` this _defaults_ to `Integer` which has an `Ord` instance:

```
Prelude> compare (3 * 4) (3 * 5)
LT
Prelude> :t (*)
(*) :: Num a => a -> a -> a
Prelude> :info Integer
data Integer
-- some other instances
instance Eq Integer
  -- Defined in ‘integer-gmp-1.0.2.0:GHC.Integer.Type’
instance Ord Integer
  -- Defined in ‘integer-gmp-1.0.2.0:GHC.Integer.Type’
-- some more instances
```

3. No. `"Julie"` and `True` are not of the same type as `compare` requires:

```
Prelude> :t compare
compare :: Ord a => a -> a -> Ordering
```

4. Yes. Similar to 2, `(+)` has a constraint of `Num`, this defaults to `Integer` which has an instance of `Ord`.

```
Prelude> (5 + 3) > (3 + 6)
False
```
