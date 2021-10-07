# Nim has first class functions including closures.
import sequtils

let powersOfTwo = @[1, 2, 4, 8, 16, 32, 64, 128, 256]

# Two syntaxes for closures - `do` and `proc`
echo(powersOfTwo.filter do (x: int) -> bool: x > 32)
echo powersOfTwo.filter(proc (x: int): bool = x > 32)

# Pass a normal procedure as an argument
proc greaterThan32(x: int): bool = x > 32
echo powersOfTwo.filter(greaterThan32)

# As expected, closures capture values from their environment
let greeting = "Hello"
let greeter = proc (): void = echo(greeting)
greeter() # --> Hello
