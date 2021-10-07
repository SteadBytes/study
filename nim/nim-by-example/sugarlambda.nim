import sugar
import strutils

# `->` macro (from sugar) simpilifies writing function type declarations e.g.
# `(char) -> char)
proc map(str: string, fun: (char) -> char): string =
  for c in str:
    result &= fun(c)

echo("foo".map(toUpperAscii)) # --> FOO


# `=>` macro (from sugar) is a shorthand for lambda functions. The following
# are exactly equivalent:
echo "foo".map((c) => char(ord(c) + 1)) # --> gpp
echo "foo".map(proc (c: char): char = char(ord(c) + 1)) # ->> gpp
