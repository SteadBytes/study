# Variadic functions
proc printThings(things: varargs[string]) =
  for thing in things:
    echo thing

printThings "words", "to", "print"

# Won't compile - by default `varargs` does not coerce args to strings
# printThings 1, "string", @[1, 2, 3]

# Coercing varargs applies the specified function (`$` in this case) to each
# argument
proc printThings2(things: varargs[string, `$`]) =
  for thing in things:
    echo thing

# `$` converts each argument into `string`
printThings2 1, "string", @[1, 2, 3]
