# Blocks can be introduced with indenting or parens

if true:
  echo "Nim is great!"

# Parens do not provide *end of statement inference* -> semicolons required to
# indicate end of statments
if true: (
  echo "Nim is cool!";
  echo "Nim is neat!"
)
# Won't compile `;` required after first `echo`
# if true: (
#   echo "Nim is cool!"
#   echo "Nim is neat!"
# )

while false:
  echo "This line is never output!"

# Indenting + `block` statement
block:
  echo "This line, on the other hand, is always output"

# `block` statement can be labelled e.g. for breaking out of nested loops
block outer:
  for i in 0..2000:
    for j in 0..2000:
      if i+j == 3145:
        echo i, ", ", j
        break outer

# Blocks introduce their own scope, but can access their surrounding scope
let b = 3
block:
  # `b` from surrounding scope
  echo b # --> 3
  # New `b` that *shadows* outer `b` (best avoided most of the time)
  let b = "bee"
  echo b # --> bee
# `b` from inside the block is not available in the outer scope
echo b # -->

# One _can_ use parens and semicolons instead of indentation (e.g. C style),
# however this is not considered idiomatic and most Nim code will not use this
# style
proc square(inSeq: seq[float]): seq[float] = (
  result = newSeq[float](len(inSeq));
  for i, v in inSeq: (
    result[i] = v*v;
  )
)
