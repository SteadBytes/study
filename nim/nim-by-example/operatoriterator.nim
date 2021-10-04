# Iterators can also be operators

# Operators are defined with backtick-enclosed names
# This redefines the builtin `..` range operator with a different name
iterator `...`*[T](a: T, b: T): T =
  var res: T = T(a)
  while res <= b:
    yield res
    inc res

for i in 0...5:
  echo i

