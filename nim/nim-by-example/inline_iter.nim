# Inline iterators are _always_ inlined by the compiler - zero overhead
# abstraction at the cost of increased code size
# 
# Inspecting the generated C code shows the three inlined `while` loops.
# Compile to C:
#
#   nim -c c inline_iter.nim  
#
# The generated C is in the nimcache directory
# https://nim-lang.org/docs/nimc.html#compiler-usage-generated-c-code-directory.
# For example ~/.cache/nim/inline_iter_d/@minline_iter.nim.c

iterator countTo(n: int): int =
  var i = 0
  while i <= n:
    yield i
    inc i

for i in countTo(5):
  echo i

for i in countTo(5):
  echo i

for i in countTo(5):
  echo i
