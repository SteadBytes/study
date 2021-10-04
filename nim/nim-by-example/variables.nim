proc getAlphabet(): string =
  # `result` is an implicitly declared variable that represents the return
  # value. If there is no `return` statement, `result` is returned.
  result = ""
  for letter in 'a'..'z':
    result.add(letter)

# Computed at compile time
const alphabet = getAlphabet()
  # Won't compile - `const` symbols are immutable
  # alphabet = "abc"

  # Mutable variables
var
  a = "foo"
  b = 0
  # Initialised to 0
  c: int

a.add("bar")
b += 1
c = 3

# Immutable variables
let
  d = "foo"
  e = 5
  # Won't compile - `let` variables *must* be initialised
  # f: float

# Won't compile
# d.add("bar")
# e + 1
