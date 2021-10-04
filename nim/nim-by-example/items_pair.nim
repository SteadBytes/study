# First class iterators (as in Python)

# Iterates from `low` to `high`
type CustomRange = object
  low: int
  high: int

# `yield` returns execution to the body of the calling `for` loop. On the next
# iteration of the `for` loop, execution returns to the statement following the
# `yield`.

# `items` iterates over each value, similar to Python's `__next__`
iterator items(range: CustomRange): int =
  var i = range.low
  while i <= range.high:
    yield i # yield is *only* valid within an iterator
    inc i

# Won't compile - `return` is *not* valid within an iterator
# iterator items(range: CustomRange): int =
#   var i = range.low
#   while i <= range.high:
#     return i
#     inc i

# `pairs` iterates over pairs of `(index, value)`, similar to Python's `enumerate`
iterator pairs(range: CustomRange): tuple[a: int, b: char] =
  for i in range: # uses CustomRange.items
    yield (i, char(i + ord('a')))

# Nim will automatically call `pairs` when 2 iterator variables are present
for i, c in CustomRange(low: 1, high: 3):
  echo c


