# Strings in Nim are *null terminated*
# - Support zero copy C interop e.g. `cstring("foo")`
#  - Must ensure the lifetime of the `cstring` does not exceed the lifetime of
#    the string it is based upon! 

# "Normal" string (including unicode)
echo "words words words ⚑"
# Quoted string - never interpret escape codes
echo """
<html>
  <head>
  </head>\n\n

  <body>
  </body>
</html> """

proc re(s: string): string = s

# Raw strings - do not interpret escape sequences except for `""` which is
# interpreted as `"`
echo r".""." # --> .".
# Proc strings - same as raw strings but the method name prefixing the string
# is called. The following are thus equivalent:
echo re"\b[a-z]++\b" # --> \b[a-z]++\b
echo re(r"\b[a-z]++\b") # --> \b[a-z]++\b

# w.r.t assignment semantics, strings can be thought of as `seq[char]`
echo "abc"[0] # --> a
echo "abcdefg"[0 .. 4] # --> abcde
echo "abcdefg"[0 .. ^2] # --> abcdef

# `strutils` module provides procs for operating on strings
import strutils

var a = "hello welcome,friend"

# The split proc takes a sequence of characters and splits a string based on them
echo a.split({' ', ','}) # --> @["hello", "welcome", "friend"]

# The contains proc determines whether a string contains a substring or character
echo a.contains("hello") # --> true

# Strings *can* contain Unicode characters but are *not treated specially* from
# the perspective of the `string` type. The `unicode` module provides support
# for operating on Unicode strings e.g. counting glyphs.
import unicode
import strformat

let x = "Mmmm, coffee"
let y = x & " ☕"
echo &"'{x}' has len {len(x)}, runLen {runeLen(x)}" # --> 'Mmmm, coffee' has len 12, runeLen 12
echo &"'{y}' has len {len(y)}, runLen {runeLen(y)}" # --> 'Mmmm, coffee ☕' has len 16, runeLen 14
