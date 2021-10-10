import strformat
# Seqs (sequences) provide dynamically expandable storage (e.g. Rust vector, Python list e.t.c)

# `@` is syntax sugar for `newSeq`, followed by populating with values
var
  a = @[1, 2, 3]
  b = newSeq[int](3)

echo a # --> @[1, 2, 3]
echo b # --> @[0, 0, 0]

for i, v in a:
  b[i] = v*v

echo &"{len(b) = }" # --> len(b) = 3
echo b # --> @[1, 4, 9]

# Expand as necessary
for i in 4..100:
  b.add(i * i)

echo &"{len(b) = }" # --> len(b) = 100
echo b # --> @[1, 4, 9, 16, 25, 36, 49, ..., 10000]

echo &"{b[0] = }" # --> b[0] = 1
b.delete(0) # takes O(n) time
echo &"{b[0] = }" # --> b[0] = 4
b = a[0] & b # Prepend b with `a[0]`
echo &"{b[0] = }" # --> b[0] = 1

# Sequences are heap-allocated and immutable unless declared as `var`

# Won't compile - `c` is immutable
# let c = @[1, 2, 3]
# c.add(4)

var c = @[1, 2, 3]
c.add(4)

# Won't compile - sequences passed by value are immutable
# proc doSomething(mySeq: seq[int]) =
#   mySeq[0] = 2  # this is a compile-time error

# Must by reference to mutate
proc doSomething(mySeq: var seq[int]) =
  mySeq[0] = 2

var testSeq = @[1, 2, 3]
echo testSeq # --> @[1, 2, 3]
doSomething(testSeq)
echo testSeq # --> @[2, 2, 3]

# Copy a seq passed by value and modify the copy
proc doSomethingElse(mySeq: seq[int]) =
  var varMySeq = mySeq # copy the seq
  varMySeq[0] = 999
  assert varMySeq[0] == 999 # copied value is modified

var aSeq = @[1, 2, 3]
doSomethingElse(aSeq)
# Original not modified
assert aSeq[0] == 1
