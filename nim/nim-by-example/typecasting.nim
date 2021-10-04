# Type conversion (checked by the compiler for safety)
var x = int(1.0 / 3)
# Annotation (empty `seq` needs type specification)
var y: seq[int] = @[]
# Casting (unsafe!)
var z = "Foobar"
proc ffi(foo: ptr array[6, char]) = echo repr(foo)
# Casts a string to a pointer to char array of length 6
ffi(cast[ptr array[6, char]](addr z[0]))
var
  tmp1: string
  tmp2: string
