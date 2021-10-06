import math

# `=` denotes start of function body
proc fibonacci(n: int): int =
  if n < 2:
    result = n
  else:
    # Uniform function call syntax: foo(a, b) == a.foo(b)
    result = fibonacci(n - 1) + (n - 2).fibonacci

# `*` annotation exports a symbol (e.g. `pub` in Rust) to be available in other
# modules
proc foo*(): int = 2

# Static side-effect analyses to prevent side effects in pure functions
proc sum(x, y: int): int {.noSideEffect.} =
  x + y

# Won't compile - `echo` is not pure
# proc echoSum(x, y: int): int {. noSideEffect .} =
#   result = x + y
#   echo result


# Define operators by enclosing the operator symbol(s) in backticks See
# https://nim-lang.org/docs/manual.html#lexical-analysis-operators for valid
# operator symbols
proc `%` (x, y: int): int =
  ## `math.floorMod` e.g. Python's `%` operator
  floorMod(x, y)

assert(3 % 10 == 3)

proc `^&*^@%`(a, b: string): string =
  ## A confusingly named useless operator
  result = a[0] & b[high(b)]

assert("foo" ^&*^@% "bar" == "fr")

# Generic functions use `[]` for type parameters
proc myPlus[T](a: T, b: T): T =
  a + b

assert(myPlus(1, 2) == 3) # `int`
assert(myPlus(1.5, 2.5) == 4) # `float`
