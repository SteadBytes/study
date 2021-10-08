# Types are declared within `type` sections
# Type aliases are **the same as the type being aliased**
# - *Not* incompatible in any way

# `type` allows multiple types to be declared within a single section
type
  MyInteger* = int
  MyFloat* = float

let a: int = 2
echo a + MyInteger(4) # --> 6

let b: float = 2.5
echo b + MyFloat(1.5) # --> 4.0

# Won't compile - incompatible types
# let c = a + MyFloat(1.5)
