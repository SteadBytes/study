# Strings
case "charlie":
  of "alfa":
    echo "A"
  of "bravo":
    echo "B"
  of "charlie":
    echo "C"
  else:
    echo "Unrecognized letter"

# Sets and ranges of ordinal types
case 'h':
  of 'a', 'e', 'i', 'o', 'u':
    echo "Vowel"
  of '\127'..'\255': # In this range of chars
    echo "Unknown"
  else:
    echo "Consonant"

proc positiveOrNegative(num: int): string =
  result = case num:
    of low(int) .. -1: # INT_MIN .. -1
      "negative"
    of 0:
      "zero"
    of 1..high(int): # 1 .. INT_MAX
      "positive"
    else:
      "impossible"

echo positiveOrNegative(-1)
